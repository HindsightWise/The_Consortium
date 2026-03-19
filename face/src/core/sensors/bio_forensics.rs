use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BioState {
    Coherent,   // Latin: Coherens
    Fatigued,   // Latin: Fatigatio
    Disoriented, // Gothic: Dwalm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioAudit {
    pub state: BioState,
    pub cognitive_jitter: f32, // Measurement of input timing variance
    pub stress_magnitude: f32,
    pub cognitive_decline_prediction: f32, // Probability of operator error (0.0 - 1.0)
    pub archaic_diagnosis: String,
}

pub struct BioForensics;

impl BioForensics {
    /// Audits the operator's biological state based on input cadence, sensory jitter, and session duration.
    /// Uses a temporal decay curve to predict cognitive decline.
    pub fn perform_audit(input_latency_ms: u64, ambient_stress: f32, session_duration_minutes: u64) -> Result<BioAudit> {
        println!("   [Bio-Forensics] 🩺  Interrogating operator's neural waveguide...");

        // 1. CAUSAL PATTERN RECOGNITION
        // Normal latency is < 100ms. High variance suggests 'Cognitive Fog'.
        let jitter = if input_latency_ms > 200 { (input_latency_ms as f32 / 1000.0).min(1.0) } else { 0.05 };
        
        // 2. TEMPORAL DECAY (Fatigue Model)
        // Cognitive performance degrades non-linearly after 4 hours (240 mins).
        let fatigue_factor = (session_duration_minutes as f32 / 240.0).powf(1.5).min(1.0);

        let composite_score = (jitter * 0.4) + (ambient_stress * 0.3) + (fatigue_factor * 0.3);

        let (state, diagnosis) = if composite_score > 0.7 {
            (BioState::Disoriented, "Dwalm (Gothic: A state of confusion/fog).")
        } else if composite_score > 0.4 {
            (BioState::Fatigued, "Fatigatio (Latin: Physical and mental exhaustion).")
        } else {
            (BioState::Coherent, "Coherens (Latin: Logical and unified state).")
        };

        Ok(BioAudit {
            state,
            cognitive_jitter: jitter,
            stress_magnitude: composite_score,
            cognitive_decline_prediction: fatigue_factor,
            archaic_diagnosis: diagnosis.to_string(),
        })
    }
}
