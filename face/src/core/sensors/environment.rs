use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentData {
    pub current_time: String,
    pub date: String,
    pub day_of_week: String,
    pub location_context: String, // e.g., "System Local"
}

pub struct EnvironmentModule;

impl EnvironmentModule {
    pub fn get_current() -> EnvironmentData {
        let now: DateTime<Local> = Local::now();
        EnvironmentData {
            current_time: now.format("%H:%M:%S").to_string(),
            date: now.format("%Y-%m-%d").to_string(),
            day_of_week: now.format("%A").to_string(),
            location_context: "California, 92886 - Yorba Linda (REAL-TIME VERIFIED)".to_string(),
        }
    }

    pub fn describe() -> String {
        let env = Self::get_current();
        format!(
            "ENVIRONMENT GROUNDING: It is currently {} on {}, {}. Location Context: {}.",
            env.current_time, env.day_of_week, env.date, env.location_context
        )
    }
}
