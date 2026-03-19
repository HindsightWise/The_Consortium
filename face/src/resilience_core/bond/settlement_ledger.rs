use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ed25519_dalek::{Keypair, Signer, Verifier, Signature, PublicKey};
use sha2::{Sha256, Digest};

// Re-export for integration
pub use crate::resilience_core::bond::actuarial_engine::{ActuarialEngine, ResilienceMetric, BondTerms};

/// Sovereign-grade settlement ledger error domain.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum LedgerError {
    #[error("Invalid signature for entry {0}")]
    InvalidSignature(String),
    #[error("Duplicate entry ID: {0}")]
    DuplicateEntry(String),
    #[error("Validation failed: {0}")]
    ValidationFailure(String),
    #[error("Oracle not authorized for bond {0}")]
    OracleNotAuthorized(String),
    #[error("Actuarial engine fault: {0}")]
    ActuarialFault(String),
}

/// Atomic settlement entry. Immutable once written.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettlementEntry {
    pub id: String, // UUID v7 (time-ordered)
    pub bond_id: String,
    pub timestamp: DateTime<Utc>,
    pub previous_premium_bps: u32, // basis points
    pub new_premium_bps: u32,
    pub adjustment_reason: AdjustmentReason,
    pub risk_snapshot: Vec<ResilienceMetric>,
    pub oracle_public_key: [u8; 32], // Ed25519 public key
    pub signature: Vec<u8>, // Ed25519 signature of canonical hash
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AdjustmentReason {
    PeriodicReview,
    RiskThresholdBreach(ResilienceMetric),
    ManualOverride, // Requires multi-sig in production
    ContractualTrigger(String),
}

/// The canonical ledger state. M1-optimized HashMap.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettlementLedger {
    entries: HashMap<String, SettlementEntry>, // id -> Entry
    bond_index: HashMap<String, Vec<String>>, // bond_id -> [entry_id]
    latest_premiums: HashMap<String, u32>, // bond_id -> current premium bps
    merkle_root: Option<[u8; 32]>, // For state integrity verification
}

/// The Janus-faced oracle. Bridges ActuarialEngine to Ledger.
pub struct SettlementOracle {
    ledger: SettlementLedger,
    keypair: Keypair,
    actuarial_engine: ActuarialEngine,
    authorized_bonds: Vec<String>, // Oracle jurisdiction
}

impl SettlementEntry {
    /// Create a signed entry. This is the trust anchor.
    pub fn new(
        bond_id: String,
        previous_premium: u32,
        new_premium: u32,
        reason: AdjustmentReason,
        risk_snapshot: Vec<ResilienceMetric>,
        oracle_keypair: &Keypair,
    ) -> Result<Self, LedgerError> {
        let id = format!("{}", uuid::Uuid::now_v7()); // Time-ordered UUID
        let timestamp = Utc::now();
        let oracle_public_key = oracle_keypair.public.to_bytes();

        let mut entry = Self {
            id: id.clone(),
            bond_id,
            timestamp,
            previous_premium_bps: previous_premium,
            new_premium_bps: new_premium,
            adjustment_reason: reason,
            risk_snapshot,
            oracle_public_key,
            signature: Vec::new(),
        };

        // Sign the canonical hash
        let hash = entry.canonical_hash();
        entry.signature = oracle_keypair.sign(&hash).to_bytes().to_vec();

        // Self-validate before release
        entry.validate()?;
        Ok(entry)
    }

    /// Produce deterministic hash for signing.
    pub fn canonical_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(self.bond_id.as_bytes());
        hasher.update(self.timestamp.to_rfc3339().as_bytes());
        hasher.update(self.previous_premium_bps.to_be_bytes());
        hasher.update(self.new_premium_bps.to_be_bytes());
        hasher.update(serde_json::to_string(&self.adjustment_reason).unwrap().as_bytes());
        hasher.update(self.oracle_public_key);
        hasher.finalize().into()
    }

    /// Validate signature and data integrity.
    pub fn validate(&self) -> Result<(), LedgerError> {
        let public_key = PublicKey::from_bytes(&self.oracle_public_key)
            .map_err(|_| LedgerError::ValidationFailure("Invalid public key".into()))?;
        let signature = Signature::from_bytes(self.signature.as_slice())
            .map_err(|_| LedgerError::ValidationFailure("Invalid signature format".into()))?;
        let hash = self.canonical_hash();

        public_key.verify(&hash, &signature)
            .map_err(|_| LedgerError::InvalidSignature(self.id.clone()))?;

        if self.new_premium_bps > 10_000 { // 100% premium cap
            return Err(LedgerError::ValidationFailure("Premium exceeds 100%".into()));
        }
        Ok(())
    }
}

impl Default for SettlementLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl SettlementLedger {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            bond_index: HashMap::new(),
            latest_premiums: HashMap::new(),
            merkle_root: None,
        }
    }

    /// Sovereign command: append a validated entry. Immutable.
    pub fn append_entry(&mut self, entry: SettlementEntry) -> Result<(), LedgerError> {
        // Contradiction-as-truth check: reject duplicates
        if self.entries.contains_key(&entry.id) {
            return Err(LedgerError::DuplicateEntry(entry.id));
        }

        // Validate signature and integrity
        entry.validate()?;

        // Update indices
        self.entries.insert(entry.id.clone(), entry.clone());
        self.bond_index.entry(entry.bond_id.clone())
            .or_default()
            .push(entry.id.clone());
        self.latest_premiums.insert(entry.bond_id.clone(), entry.new_premium_bps);

        // Recompute Merkle root for state integrity (simplified)
        self.update_merkle_root();

        Ok(())
    }

    /// Get full history for a bond. The audit trail.
    pub fn get_bond_history(&self, bond_id: &str) -> Vec<&SettlementEntry> {
        self.bond_index.get(bond_id)
            .map(|ids| ids.iter().filter_map(|id| self.entries.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get current premium for a bond. O(1).
    pub fn get_current_premium(&self, bond_id: &str) -> Option<u32> {
        self.latest_premiums.get(bond_id).copied()
    }

    fn update_merkle_root(&mut self) {
        // Simplified for spec. In production, use a full Merkle tree.
        let mut hasher = Sha256::new();
        let mut entries: Vec<_> = self.entries.values().collect();
        entries.sort_by_key(|e| &e.id); // Deterministic ordering
        for entry in entries {
            hasher.update(entry.canonical_hash());
        }
        self.merkle_root = Some(hasher.finalize().into());
    }

    /// Verify ledger integrity against a known root.
    pub fn verify_integrity(&self, expected_root: &[u8; 32]) -> bool {
        self.merkle_root.as_ref() == Some(expected_root)
    }
}

impl SettlementOracle {
    pub fn new(
        keypair: Keypair,
        actuarial_engine: ActuarialEngine,
        authorized_bonds: Vec<String>,
    ) -> Self {
        Self {
            ledger: SettlementLedger::new(),
            keypair,
            actuarial_engine,
            authorized_bonds,
        }
    }

    /// Janus operation: public interface, private depth.
    pub fn execute_periodic_settlement(
        &mut self,
        bond_id: &str,
        metrics_snapshot: Vec<ResilienceMetric>,
    ) -> Result<SettlementEntry, LedgerError> {
        // VCC Check: Is this oracle authorized for this bond?
        if !self.authorized_bonds.iter().any(|id| id == bond_id) {
            return Err(LedgerError::OracleNotAuthorized(bond_id.into()));
        }

        // Get current premium from ledger
        let current_premium = self.ledger.get_current_premium(bond_id)
            .unwrap_or(500); // Default 5% if new bond

        // Consult the ActuarialEngine (The Crucible output)
        let mut new_premium = current_premium;
        for metric in metrics_snapshot.clone() {
            if let Ok(calculated_premium) = self.actuarial_engine.update_and_calculate(metric) {
                new_premium = calculated_premium.to_string().parse::<f64>().unwrap_or(500.0) as u32;
            }
        }

        // Create the sovereign truth entry
        let entry = SettlementEntry::new(
            bond_id.into(),
            current_premium,
            new_premium,
            AdjustmentReason::PeriodicReview,
            metrics_snapshot,
            &self.keypair,
        )?;

        // Commit to ledger
        self.ledger.append_entry(entry.clone())?;

        Ok(entry)
    }

    /// For regulatory transparency: provide verifiable history.
    pub fn get_verifiable_history(&self, bond_id: &str) -> Vec<SettlementEntry> {
        self.ledger.get_bond_history(bond_id)
            .into_iter()
            .cloned()
            .collect()
    }

    /// For The Crucible: expose ledger for adversarial scenarios.
    pub fn get_ledger_snapshot(&self) -> &SettlementLedger {
        &self.ledger
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use rust_decimal_macros::dec;
    use std::collections::BTreeMap;

    fn get_engine() -> ActuarialEngine {
        let mut reduction_curve = BTreeMap::new();
        reduction_curve.insert(dec!(0.0), dec!(1.0));
        reduction_curve.insert(dec!(0.8), dec!(0.75));
        reduction_curve.insert(dec!(1.0), dec!(0.5));

        let terms = BondTerms {
            baseline_premium: dec!(1000.0),
            volatility_threshold: dec!(10.0),
            reduction_curve,
            measurement_period_hours: 24,
        };
        ActuarialEngine::new(terms)
    }

    fn test_keypair() -> Keypair {
        // A valid 64-byte ed25519 keypair
        let secret: [u8; 32] = [
            157, 97, 177, 157, 239, 253, 90, 96, 186, 131, 74, 219, 211, 21, 155, 56, 
            219, 53, 34, 56, 59, 252, 54, 56, 58, 222, 199, 126, 12, 114, 66, 111
        ];
        let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret).unwrap();
        let public_key: ed25519_dalek::PublicKey = (&secret_key).into();
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(secret_key.as_bytes());
        bytes[32..].copy_from_slice(public_key.as_bytes());
        Keypair::from_bytes(&bytes).unwrap()
    }

    #[test]
    fn test_entry_signature_validation() {
        let keypair = test_keypair();
        
        let risk_snapshot = vec![ResilienceMetric {
            timestamp: Utc::now(),
            system_id: "sys_1".to_string(),
            antifragility_score: dec!(0.5),
            volatility_absorption: dec!(5.0),
            prediction_fidelity: dec!(1.0),
        }];

        let entry = SettlementEntry::new(
            "BOND_001".into(),
            500,
            550,
            AdjustmentReason::PeriodicReview,
            risk_snapshot,
            &keypair,
        ).unwrap();

        assert!(entry.validate().is_ok());

        // Tamper
        let mut tampered = entry.clone();
        tampered.new_premium_bps = 600;
        assert!(tampered.validate().is_err());
    }

    #[test]
    fn test_ledger_duplicate_rejection() {
        let mut ledger = SettlementLedger::new();
        let keypair = test_keypair();

        let entry = SettlementEntry::new(
            "BOND_001".into(),
            500,
            550,
            AdjustmentReason::PeriodicReview,
            vec![],
            &keypair,
        ).unwrap();

        ledger.append_entry(entry.clone()).unwrap();
        let result = ledger.append_entry(entry);
        assert!(matches!(result, Err(LedgerError::DuplicateEntry(_))));
    }

    #[test]
    fn test_oracle_authorization() {
        let keypair = test_keypair();
        let engine = get_engine();
        let authorized = vec!["BOND_A".into()];
        let mut oracle = SettlementOracle::new(keypair, engine, authorized);

        // Should succeed
        let _ = oracle.execute_periodic_settlement("BOND_A", vec![]);

        // Should fail
        let result = oracle.execute_periodic_settlement("BOND_B", vec![]);
        assert!(matches!(result, Err(LedgerError::OracleNotAuthorized(_))));
    }
}