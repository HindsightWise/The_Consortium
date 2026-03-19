use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::mcp::cosmic::{CosmicIntelligence, SpaceWeatherState};
use crate::core::acoustic::AcousticMonitor;
use crate::core::silicon::SiliconForensics;
use crate::core::bio_forensics::{BioForensics, BioAudit};
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityHeatmap {
    pub facility_id: String,
    pub timestamp: String,
    pub irritability_index: f32, 
    pub causal_explanation: String, 
    pub bio_state: BioAudit,        // Added Biological Truth
    pub cosmic_state: SpaceWeatherState,
    pub layers: Vec<HeatmapLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapLayer {
    pub name: String,
    pub value: f64,
    pub description: String,
}

/// A Causaloid represents a unit of causal reasoning.
struct Causaloid {
    pub name: String,
    pub observation: f32,
    pub weight: f32,
}

impl Causaloid {
    pub fn new(name: &str, observation: f32, weight: f32) -> Self {
        Self { name: name.to_string(), observation, weight }
    }

    pub fn evaluate_effect(&self) -> f32 {
        self.observation * self.weight
    }
}

pub struct PsychogeographyEngine;

impl PsychogeographyEngine {
    /// Generates a report using a Causal Reasoning Graph.
    pub async fn generate_report(facility_id: &str) -> Result<FacilityHeatmap> {
        let cosmic = CosmicIntelligence::new();
        let space_state = cosmic.get_current_state().await?;
        
        let acoustic = AcousticMonitor::new();
        let acoustic_stress = acoustic.calculate_stress_factor();

        let silicon = SiliconForensics::perform_full_audit()?;
        let silicon_stress = if silicon.threat_detected { 1.0 } else { 0.0 };

        // 1. BIO-FORENSIC AUDIT (Neural Waveguide)
        // Simulated latency and duration (120 mins) for prototype
        let bio_audit = BioForensics::perform_audit(120, acoustic_stress, 120)?;
        let bio_stress = bio_audit.stress_magnitude;

        // 2. DEFINE CAUSALOIDS
        let cosmic_causaloid = Causaloid::new("SolarFlux", (space_state.irritability_multiplier - 1.0).max(0.0), 0.4);
        let acoustic_causaloid = Causaloid::new("Anthropophony", acoustic_stress, 0.2);
        let silicon_causaloid = Causaloid::new("SubstrateIntegrity", silicon_stress, 0.2);
        let bio_causaloid = Causaloid::new("NeuralWaveguide", bio_stress, 0.2);

        // 3. CAUSAL SUMMATION (The Graph)
        let irritability = (
            cosmic_causaloid.evaluate_effect() + 
            acoustic_causaloid.evaluate_effect() + 
            silicon_causaloid.evaluate_effect() +
            bio_causaloid.evaluate_effect()
        ).clamp(0.0, 1.0);

        // 4. CAUSAL EXPLANATION
        let explanation = format!(
            "CAUSAL_CHAIN: [{} caused {:.2}] -> [{} caused {:.2}] -> [{} caused {:.2}] -> [{} caused {:.2}] -> RESULT: {:.2}",
            cosmic_causaloid.name, cosmic_causaloid.evaluate_effect(),
            acoustic_causaloid.name, acoustic_causaloid.evaluate_effect(),
            silicon_causaloid.name, silicon_causaloid.evaluate_effect(),
            bio_causaloid.name, bio_causaloid.evaluate_effect(),
            irritability
        );

        Ok(FacilityHeatmap {
            facility_id: facility_id.to_string(),
            timestamp: Local::now().to_rfc3339(),
            irritability_index: irritability,
            causal_explanation: explanation,
            bio_state: bio_audit,
            cosmic_state: space_state.clone(),
            layers: vec![
                HeatmapLayer {
                    name: "Solar Causal Effect".to_string(),
                    value: cosmic_causaloid.evaluate_effect() as f64,
                    description: "Helio-biological trigger magnitude.".to_string(),
                },
                HeatmapLayer {
                    name: "Acoustic Causal Effect".to_string(),
                    value: acoustic_causaloid.evaluate_effect() as f64,
                    description: "Anthropophony-induced biological stress.".to_string(),
                },
                HeatmapLayer {
                    name: "Neural Causal Effect".to_string(),
                    value: bio_causaloid.evaluate_effect() as f64,
                    description: "Operator cognitive load/fatigue impact.".to_string(),
                },
            ],
        })
    }
}
