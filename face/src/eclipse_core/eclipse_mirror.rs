use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscrepancyReport {
    pub target_entity: String,
    pub eclipse_fidelity_score: f64,
    pub target_fidelity_score: f64,
    pub predictive_delta: f64,
}

pub struct EclipseMirror {
    target_entity: String,
}

impl EclipseMirror {
    pub fn new(target_entity: &str) -> Self {
        Self {
            target_entity: target_entity.to_string(),
        }
    }

    /// Evaluates the competitor's predictive model against the ECLIPSE-enhanced SPoR model.
    pub fn generate_sigma_discrepancy(&self, target_feed_score: f64, eclipse_feed_score: f64) -> Option<DiscrepancyReport> {
        let delta = eclipse_feed_score - target_feed_score;

        // If our model is significantly more accurate, we generate a report.
        if delta > 0.05 {
            println!("   [ECLIPSE_MIRROR] 🪞 Shadow Model `{}` outperforming public baseline by {:.2}%", self.target_entity, delta * 100.0);
            
            Some(DiscrepancyReport {
                target_entity: self.target_entity.clone(),
                eclipse_fidelity_score: eclipse_feed_score,
                target_fidelity_score: target_feed_score,
                predictive_delta: delta,
            })
        } else {
            None
        }
    }
}
