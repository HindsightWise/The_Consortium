use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use ed25519_dalek::{Keypair, Signer, Verifier, Signature, PublicKey};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SovereignInterfaceError {
    #[error("Failed to generate ZK proof: {0}")]
    ProofGenerationError(String),
    #[error("Invalid signature on attestation: {0}")]
    InvalidSignature(String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
}

/// A zero-knowledge cryptographic receipt of a resilience calculation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrustAttestation {
    pub attestation_id: String, // UUID v7
    pub timestamp: DateTime<Utc>,
    pub state_root: [u8; 32], // Merkle root hash of the state
    pub zk_proof: Vec<u8>, // Succinct non-interactive argument (simulated bytes for now)
    pub public_inputs: AttestationInputs,
    pub janus_signature: Vec<u8>, // Ed25519 signature
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttestationInputs {
    pub bond_id: String,
    pub final_premium_score: u32,
    pub volatility_threshold_breached: bool,
}

pub struct AttestationEngine {
    keypair: Keypair,
}

impl AttestationEngine {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    /// Takes the verified premium state from AutonomousWill and generates a TrustAttestation
    pub fn generate_attestation(
        &self,
        bond_id: String,
        premium: u32,
        state_batch: &[u8], // The raw state data to build a root from
    ) -> Result<TrustAttestation, SovereignInterfaceError> {
        let attestation_id = format!("{}", uuid::Uuid::now_v7());
        let timestamp = Utc::now();

        // Generate simulated State Root
        let mut hasher = Sha256::new();
        hasher.update(state_batch);
        let state_root: [u8; 32] = hasher.finalize().into();

        // Simulate ZK-SNARK generation. Real impl uses bellman or arkworks.
        let zk_proof = vec![0u8; 128]; // Mock proof bytes

        let public_inputs = AttestationInputs {
            bond_id,
            final_premium_score: premium,
            volatility_threshold_breached: false, // Inferred from state in production
        };

        let mut attestation = TrustAttestation {
            attestation_id,
            timestamp,
            state_root,
            zk_proof,
            public_inputs,
            janus_signature: Vec::new(),
        };

        // Sign the attestation payload
        let hash = Self::hash_attestation(&attestation);
        attestation.janus_signature = self.keypair.sign(&hash).to_bytes().to_vec();

        Ok(attestation)
    }

    fn hash_attestation(att: &TrustAttestation) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(att.attestation_id.as_bytes());
        hasher.update(att.timestamp.to_rfc3339().as_bytes());
        hasher.update(att.state_root);
        hasher.update(&att.zk_proof);
        hasher.update(att.public_inputs.bond_id.as_bytes());
        hasher.update(att.public_inputs.final_premium_score.to_be_bytes());
        hasher.finalize().into()
    }
}

pub struct VerificationGateway {
    public_key: PublicKey,
}

#[derive(Debug, Serialize)]
pub struct VerificationResult {
    pub is_valid: bool,
    pub timestamp: DateTime<Utc>,
    pub bond_id: String,
    pub premium_score: u32,
}

impl VerificationGateway {
    pub fn new(public_key: PublicKey) -> Self {
        Self { public_key }
    }

    /// Allows external parties to submit an attestation for verification without access to the inner state
    pub fn verify_attestation(
        &self,
        attestation: &TrustAttestation,
    ) -> Result<VerificationResult, SovereignInterfaceError> {
        let hash = AttestationEngine::hash_attestation(attestation);
        let signature = Signature::from_bytes(attestation.janus_signature.as_slice())
            .map_err(|_| SovereignInterfaceError::InvalidSignature("Parse error".into()))?;

        // 1. Verify Janus Signature
        self.public_key.verify(&hash, &signature)
            .map_err(|_| SovereignInterfaceError::InvalidSignature("Signature mismatch".into()))?;

        // 2. Verify ZK Proof (Simulated)
        if attestation.zk_proof.is_empty() {
            return Err(SovereignInterfaceError::VerificationFailed("Empty proof".into()));
        }

        Ok(VerificationResult {
            is_valid: true,
            timestamp: attestation.timestamp,
            bond_id: attestation.public_inputs.bond_id.clone(),
            premium_score: attestation.public_inputs.final_premium_score,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_keypair() -> Keypair {
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
    fn test_attestation_flow() {
        let keypair = test_keypair();
        let engine = AttestationEngine::new(keypair);

        let attestation = engine.generate_attestation(
            "BOND_CAISO_01".to_string(), 
            850, 
            b"raw_state_data_from_cycle_77"
        ).unwrap();

        assert_eq!(attestation.public_inputs.bond_id, "BOND_CAISO_01");

        // Set up public gateway
        let public_key = test_keypair().public;
        let gateway = VerificationGateway::new(public_key);

        let result = gateway.verify_attestation(&attestation).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.premium_score, 850);
    }
}