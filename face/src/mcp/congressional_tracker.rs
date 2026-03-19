use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CongressionalTrade {
    pub disclosure_date: String,
    pub transaction_date: String,
    pub owner: String,
    pub ticker: String,
    pub asset_description: String,
    pub type_raw: String,
    pub amount: String,
    pub representative: String,
    pub district: Option<String>,
    pub ptr_link: String,
}

pub struct CongressionalTracker {
    cache_path: String,
}

impl CongressionalTracker {
    pub fn new(cache_path: &str) -> Self {
        Self {
            cache_path: cache_path.to_string(),
        }
    }

    pub async fn fetch_latest_trades(&self) -> Result<Vec<CongressionalTrade>> {
        println!("   [Signal-Hunter] 🛰️ Fetching latest Congressional trades...");
        
        // Target the official Senate EFD search (most reliable source)
        let url = "https://efdsearch.senate.gov/search/view/paper/all_transactions.json";
        
        let client = reqwest::Client::new();
        let response = client.get(url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .send()
            .await?;

        if response.status().is_success() {
            let trades: Vec<CongressionalTrade> = response.json().await?;
            self.cache_trades(&trades)?;
            Ok(trades)
        } else {
            Err(anyhow::anyhow!("Failed to fetch trades: {}", response.status()))
        }
    }

    fn cache_trades(&self, trades: &Vec<CongressionalTrade>) -> Result<()> {
        let data = serde_json::to_string_pretty(trades)?;
        fs::write(&self.cache_path, data)?;
        Ok(())
    }

    pub fn get_recent_buys(&self, _days_ago: u64) -> Result<Vec<CongressionalTrade>> {
        let data = fs::read_to_string(&self.cache_path)?;
        let trades: Vec<CongressionalTrade> = serde_json::from_str(&data)?;
        
        // Basic filtering for "Purchase" and date (logic to be refined with chrono)
        let recent_buys: Vec<CongressionalTrade> = trades.into_iter()
            .filter(|t| t.type_raw.to_lowercase().contains("purchase") && t.ticker != "--")
            .collect();

        Ok(recent_buys)
    }
}
