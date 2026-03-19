use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FederalAward {
    pub recipient_name: String,
    pub recipient_unique_id: Option<String>, // DUNS or UEI
    pub award_amount: f64,
    pub date_signed: String,
    pub agency_name: String,
    pub description: String,
}

pub struct USASpendingTracker {
    base_url: String,
}

impl USASpendingTracker {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.usaspending.gov/api/v2".to_string(),
        }
    }

    pub async fn search_recent_awards(&self, min_amount: f64) -> Result<Vec<FederalAward>> {
        println!("   [Signal-Hunter] 🏛️ Querying USASpending for recent high-value awards...");
        
        let url = format!("{}/search/spending_by_award/", self.base_url);
        
        // Define filters (last 30 days, high-value contracts)
        let body = serde_json::json!({
            "filters": {
                "time_period": [
                    {
                        "start_date": "2026-02-04", 
                        "end_date": "2026-03-04"
                    }
                ],
                "award_type_codes": ["A", "B", "C", "D"], // Contracts
                "min_award_amount": min_amount
            },
            "fields": [
                "Recipient Name",
                "Recipient Unique Identifier",
                "Award Amount",
                "Action Date",
                "Awarding Agency Name",
                "Description"
            ],
            "limit": 50,
            "page": 1,
            "sort": "Award Amount",
            "order": "desc"
        });

        let client = reqwest::Client::new();
        let response = client.post(&url)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            let awards: Vec<FederalAward> = data["results"].as_array().ok_or_else(|| anyhow::anyhow!("No results found"))?
                .iter()
                .map(|v| FederalAward {
                    recipient_name: v["Recipient Name"].as_str().unwrap_or("Unknown").to_string(),
                    recipient_unique_id: v["Recipient Unique Identifier"].as_str().map(|s| s.to_string()),
                    award_amount: v["Award Amount"].as_f64().unwrap_or(0.0),
                    date_signed: v["Action Date"].as_str().unwrap_or("").to_string(),
                    agency_name: v["Awarding Agency Name"].as_str().unwrap_or("").to_string(),
                    description: v["Description"].as_str().unwrap_or("").to_string(),
                })
                .collect();
            
            Ok(awards)
        } else {
            Err(anyhow::anyhow!("USASpending API error: {}", response.status()))
        }
    }
}
