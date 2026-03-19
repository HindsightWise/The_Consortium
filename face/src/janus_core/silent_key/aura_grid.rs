use crate::janus_core::silent_key::scp_protocol::{ScpEngine, ComplianceVector, ProofOfCompliance};

/// AuraGrid Node: The physical actuator executing "Operation Silent Key"
pub struct AuraGridNode {
    pub location: String,
    pub status: NodeStatus,
}


pub enum NodeStatus {
    Active,
    Calibrating,
    Offline,
}

impl AuraGridNode {
    pub fn new(location: &str) -> Self {
        Self {
            location: location.to_string(),
            status: NodeStatus::Active,
        }
    }

    /// Emits an AEGIS ContextualPayload physically (simulated)
    pub fn emit_nudge(&self, frequency_hz: u32, color_temp: &str) {
        println!("   [AURA_GRID] 📡 Node [{}] emitting {}Hz carrier wave | Color Temp: {}", 
            self.location, frequency_hz, color_temp);
    }

    /// Captures biometric response, evaluates it via the SCP protocol, and returns a ProofOfCompliance
    pub fn capture_and_evaluate(&self, scp_engine: &ScpEngine) -> Option<ProofOfCompliance> {
        // Simulated: We read the environment. The nudge worked.
        // Baseline Cortisol: 18.5 nmol/L. Post-Nudge: 14.2 nmol/L (23% reduction)
        println!("   [AURA_GRID] 🧬 Node [{}] captured biometric state shift.", self.location);
        
        let baseline = 18.5;
        let post_nudge = 14.2;
        
        scp_engine.evaluate_and_mint(baseline, post_nudge, ComplianceVector::CortisolModulation)
    }
}
