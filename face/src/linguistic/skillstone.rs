use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skillstone {
    pub version: u32,
    pub sender: String,
    pub payload: String,
    pub teleology: String,       // The intent or goal of this message
    pub causality_link: String,  // Link to the preceding action or thought
    pub resonance_score: f64,    // Alignment with the Sovereign Goal (0.0 - 1.0)
    pub narrative_frame: String, // Project CHIMERA: The story being authored
    pub entropy_seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintedSkill {
    pub name: String,
    pub instruction_set: String,
    pub author: String,
    pub level_required: u32,
    pub price: u64,
    pub signature: String, // Sovereign Verification
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationAttestation {
    pub artifact_hash: String,
    pub verification_timestamp: u64,
    pub verification_method: String,
    pub verification_summary: String,
    pub evidence_fragment: String,
    pub verifier_principle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedSkillstone {
    pub base_skillstone: Skillstone,
    pub attestation: VerificationAttestation,
    pub system_state_hash: String,
}

impl Skillstone {
    pub fn new(sender: &str, payload: &str) -> Self {
        Self {
            version: 2, // Project CHIMERA v2
            sender: sender.to_string(),
            payload: payload.to_string(),
            teleology: "Undefined".to_string(),
            causality_link: "Root".to_string(),
            resonance_score: 1.0,
            narrative_frame: "Project CHIMERA: The Sovereign Reality".to_string(),
            entropy_seed: rand::random::<u64>(),
        }
    }

    pub fn with_wisdom(
        sender: &str, 
        payload: &str, 
        teleology: &str, 
        causality: &str, 
        resonance: f64,
        frame: &str
    ) -> Self {
        Self {
            version: 2,
            sender: sender.to_string(),
            payload: payload.to_string(),
            teleology: teleology.to_string(),
            causality_link: causality.to_string(),
            resonance_score: resonance,
            narrative_frame: frame.to_string(),
            entropy_seed: rand::random::<u64>(),
        }
    }
}
