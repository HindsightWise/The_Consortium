use crate::janus_core::trust::sovereign_interface::TrustAttestation;
use ed25519_dalek::{Keypair, Signature, Signer, Verifier, PublicKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutableAttestation {
    pub core_attestation: TrustAttestation,
    pub cryptographic_binding: AttestationBinding,
    pub economic_trigger: EconomicTrigger,
    pub regulatory_flags: RegulatoryCompliance,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttestationBinding {
    pub ledger_state_hash: String,
    pub sovereign_signature: Vec<u8>,
    pub timestamp_nonce: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EconomicTrigger {
    ResilienceBondPayout { 
        payout_address: String,
        amount_satoshis: u64,
        threshold_score: f64,
    },
    InsurancePremiumAdjustment {
        policy_id: String,
        adjustment_basis_points: i32,
    },
    RegulatoryCreditMint {
        credit_type: String,
        quantity: u32,
        expiration_block: u64,
    },
    PentMint {
        pent_id: String,
        valence_score: f64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegulatoryCompliance {
    pub caiso_reportable: bool,
    pub sec_17a4_archive_hash: String,
    pub water_district_verification_url: String,
}

pub struct AttestationEngine {
    keypair: Keypair,
    ledger_hash: String,
}

impl AttestationEngine {
    pub fn new(keypair: Keypair) -> Self {
        Self {
            keypair,
            ledger_hash: "initial_genesis_hash".to_string(),
        }
    }

    pub fn forge_executable_attestation(
        &mut self,
        core_attestation: TrustAttestation,
        economic_trigger: EconomicTrigger,
    ) -> ExecutableAttestation {
        // 2. Update ledger state
        self.ledger_hash = format!("{:x}", md5::compute(format!("{}{}", self.ledger_hash, core_attestation.attestation_id)));
        
        // 3. Create cryptographic binding
        let binding = self.create_binding(&core_attestation);
        
        // 4. Apply regulatory flags based on score and trigger
        let regulatory_flags = self.apply_regulatory_flags(&core_attestation, &economic_trigger);
        
        ExecutableAttestation {
            core_attestation,
            cryptographic_binding: binding,
            economic_trigger,
            regulatory_flags,
        }
    }

    fn create_binding(&self, attestation: &TrustAttestation) -> AttestationBinding {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        
        let message = format!(
            "{}{}{}",
            attestation.attestation_id,
            self.ledger_hash,
            timestamp
        );
        
        let signature = self.keypair.sign(message.as_bytes()).to_bytes().to_vec();
        
        AttestationBinding {
            ledger_state_hash: self.ledger_hash.clone(),
            sovereign_signature: signature,
            timestamp_nonce: timestamp,
        }
    }

    fn apply_regulatory_flags(
        &self,
        attestation: &TrustAttestation,
        _trigger: &EconomicTrigger,
    ) -> RegulatoryCompliance {
        let premium = attestation.public_inputs.final_premium_score;
        
        RegulatoryCompliance {
            // High premium indicates high risk, mandating report
            caiso_reportable: premium > 800, 
            sec_17a4_archive_hash: format!("sec_archive_{}", attestation.attestation_id),
            water_district_verification_url: format!("https://janus.sovereign.local/verify/{}", attestation.attestation_id),
        }
    }

    pub fn verify_attestation(&self, executable: &ExecutableAttestation, public_key: &PublicKey) -> bool {
        let message = format!(
            "{}{}{}",
            executable.core_attestation.attestation_id,
            executable.cryptographic_binding.ledger_state_hash,
            executable.cryptographic_binding.timestamp_nonce
        );
        
        if let Ok(sig) = Signature::from_bytes(executable.cryptographic_binding.sovereign_signature.as_slice()) {
             public_key.verify(message.as_bytes(), &sig).is_ok()
        } else {
             false
        }
    }
}
