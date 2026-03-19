use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersonalExperienceToken {
    pub pent_id: String,
    pub timestamp: i64,
    pub geographic_hash: String,
    pub emotional_valence_hash: String,
    pub participant_attestation: String,
    pub cryptographic_proof: String,
}

pub struct PentNotary {}

impl Default for PentNotary {
    fn default() -> Self {
        Self::new()
    }
}

impl PentNotary {
    pub fn new() -> Self { Self {} }
    
    pub fn mint_pent(
        &self, 
        geographic_hash: &str, 
        emotional_valence_score: f64, 
        attestation: &str
    ) -> PersonalExperienceToken {
        let timestamp = Utc::now().timestamp();
        let pent_id = format!("PENT_{}", Uuid::new_v4());
        
        // Simulate a biometric valence hash
        let mut hasher = Sha256::new();
        hasher.update(emotional_valence_score.to_le_bytes());
        let valence_hash = format!("{:x}", hasher.finalize());
        
        // Combine inputs for the final proof
        let mut proof_hasher = Sha256::new();
        proof_hasher.update(format!("{}{}{}{}{}", pent_id, timestamp, geographic_hash, valence_hash, attestation).as_bytes());
        let proof = format!("{:x}", proof_hasher.finalize());
        
        println!("   [AXIOM] 🧘 PENT Minted: {}", pent_id);
        println!("   [AXIOM] 📍 Location Hash: {}", geographic_hash);
        println!("   [AXIOM] 💖 Valence/Attestation: {} | '{}'", valence_hash, attestation);
        println!("   [AXIOM] 🔗 Cryptographic Proof: {}", proof);

        PersonalExperienceToken {
            pent_id,
            timestamp,
            geographic_hash: geographic_hash.to_string(),
            emotional_valence_hash: valence_hash,
            participant_attestation: attestation.to_string(),
            cryptographic_proof: proof,
        }
    }
}
