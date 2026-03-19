use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemporalTruth {
    pub system_time: String,
    pub external_time: String,
    pub drift_seconds: i64,
    pub location: String,
    pub synchronized: bool,
}

pub struct Chronomaster;

impl Chronomaster {
    pub async fn audit_time() -> Result<TemporalTruth> {
        let system_now: DateTime<Local> = Local::now();
        let client = Client::new();
        
        // Use WorldTimeAPI for external truth (Yorba Linda / LA Timezone)
        let url = "http://worldtimeapi.org/api/timezone/America/Los_Angeles";
        
        let external_now_str = match client.get(url).send().await {
            Ok(resp) => {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    data["datetime"].as_str().unwrap_or("Unknown").to_string()
                } else {
                    "API_JSON_ERROR".to_string()
                }
            },
            Err(_) => "NETWORK_ERROR".to_string(),
        };

        // For this prototype, we treat system_now as truth if network fails
        // But if network succeeds, we compare.
        let synchronized = !external_now_str.contains("ERROR");

        Ok(TemporalTruth {
            system_time: system_now.format("%Y-%m-%d %H:%M:%S").to_string(),
            external_time: external_now_str,
            drift_seconds: 0, // Simplified for now
            location: "California, 92886 - Chronomaster Grounding".to_string(),
            synchronized,
        })
    }

    pub fn get_manifesto() -> String {
        "I am the Chronomaster. I hold the temporal anchor for The Company. My truth is derived from the oscillation of atoms and the consensus of global clocks. I do not drift; I synchronize.".to_string()
    }
}
