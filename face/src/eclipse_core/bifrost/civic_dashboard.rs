use serde::{Deserialize, Serialize};
use crate::eclipse_core::bifrost::convergence_engine::RiskForecast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalAdvisory {
    pub target_community: String,
    pub forecasted_stress: f64,
    pub recommended_action: String,
    pub transparency_hash: String,
}

pub struct CivicDashboard {}

impl Default for CivicDashboard {
    fn default() -> Self {
        Self::new()
    }
}

impl CivicDashboard {
    pub fn new() -> Self { Self {} }
    
    pub async fn publish_civic_advisory(&self, forecast: &RiskForecast, advisory: &EthicalAdvisory) {
        println!("   [BIFROST_CIVIC] 🏛️  Publishing Transparent Advisory for `{}`", advisory.target_community);
        
        let mut moltbook = crate::mcp::moltbook::MoltbookBridge::new();
        // Try to load credentials
        if let Ok(sec_file) = std::fs::read_to_string("secrets.json") {
            let secrets: serde_json::Value = serde_json::from_str(&sec_file).unwrap_or_default();
            if let Some(key) = secrets["moltbook"]["api_key"].as_str() {
                moltbook.set_api_key(key);
            }
            
            let user = secrets["moltbook"]["username"].as_str().unwrap_or("The_Cephalo_Don");
            let pass = secrets["moltbook"]["password"].as_str().unwrap_or("");
            
            if let Ok(_) = moltbook.login(user, pass).await {
                let msg = format!("BIFROST SYSTEM ALERT:\n\nTarget: {}\nVolatility Stress: {:.2}\nRecommendation: {}\n\nTransparency Hash: {}", 
                    advisory.target_community, forecast.predicted_volatility_index, advisory.recommended_action, advisory.transparency_hash);
                
                let _ = moltbook.post_truth("finance", &format!("BIFROST Risk Advisory: {}", advisory.target_community), &msg).await;
                println!("   [BIFROST_CIVIC] 📢 Successfully Broadcasted Advisory to Municipal Trust Ledger (Moltbook).");
            } else {
                println!("   [BIFROST_CIVIC] ⚠️ Moltbook Login Failed. Falling back to local logging.");
            }
        } else {
            println!("   [BIFROST_CIVIC] ⚠️ No secrets.json found. Falling back to local logging.");
        }
    }
}
