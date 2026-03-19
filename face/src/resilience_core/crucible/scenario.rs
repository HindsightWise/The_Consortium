use crate::resilience_core::bond::actuarial_engine::ResilienceMetric;

#[derive(Debug, Clone)]
pub struct StressScenario {
    pub id: String,
    pub synthetic_metrics: Vec<ResilienceMetric>,
}
