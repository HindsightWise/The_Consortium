use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TelemetryUpdate {
    pub lattice_integrity: Option<f32>,
    pub error_rate: Option<f32>,
    pub coherence: Option<f32>,
    pub uptime_secs: Option<u64>,
    pub active_skills: Option<usize>,
    pub token_usage: Option<u64>,
    pub context_fullness: Option<f32>,
    pub learning_subject: Option<String>,
    pub treasury_balances: Option<String>,
    pub alpaca_status: Option<String>,
    pub socialization_status: Option<String>,
    pub verified_action: Option<String>,
    pub follow_up_task: Option<String>,
    pub log_message: Option<String>,
}
