use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use colored::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: String,
    pub score: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    pub address: String,
    pub entity_name: String,
    pub entity_type: String,
    pub risk_score: f32,
    pub risk_level: String,
    pub significant_factors: Vec<RiskFactor>,
    pub timestamp: DateTime<Utc>,
}

pub struct BlockchainIntelAnalyzer {
    entity_type_risk: HashMap<String, f32>,
}

impl BlockchainIntelAnalyzer {
    pub fn new() -> Self {
        let mut entity_type_risk = HashMap::new();
        entity_type_risk.insert("Mixer".to_string(), 0.9);
        entity_type_risk.insert("DarknetMarket".to_string(), 0.9);
        entity_type_risk.insert("Sanctioned".to_string(), 1.0);
        entity_type_risk.insert("HighRiskExchange".to_string(), 0.7);
        entity_type_risk.insert("Exchange".to_string(), 0.2);
        entity_type_risk.insert("DeFi Protocol".to_string(), 0.3);
        entity_type_risk.insert("Token".to_string(), 0.2);
        entity_type_risk.insert("Wallet".to_string(), 0.2);
        entity_type_risk.insert("Unknown".to_string(), 0.5);

        Self { entity_type_risk }
    }

    pub async fn get_address_risk(&self, address: &str) -> Result<RiskReport> {
        println!("   [INTEL] 🔍 Analyzing Blockchain Address: {}", address.cyan());

        // 1. Mock Entity Identification (to be replaced by real DB/API lookup)
        let (entity_name, entity_type, confidence) = self.identify_entity(address);

        // 2. Detect Risk Factors (Transaction Pattern Analysis)
        let mut factors = Vec::new();
        
        // Example: Sanctioned Check
        if entity_type == "Sanctioned" {
            factors.push(RiskFactor {
                factor_type: "SanctionedEntity".to_string(),
                score: 1.0,
                description: "Address belongs to a sanctioned entity.".to_string(),
            });
        }

        // Example: Mixer Check
        if entity_type == "Mixer" {
            factors.push(RiskFactor {
                factor_type: "MixerInteraction".to_string(),
                score: 0.8,
                description: "Direct interaction with cryptocurrency mixing services.".to_string(),
            });
        }

        // Example: New Address Check (Mocked logic)
        if address.ends_with("00") {
            factors.push(RiskFactor {
                factor_type: "NewAddress".to_string(),
                score: 0.4,
                description: "Recently created address with limited history.".to_string(),
            });
        }

        // 3. Calculate Final Risk Score
        let base_risk = self.entity_type_risk.get(&entity_type).cloned().unwrap_or(0.5) * confidence;
        let mut final_score = base_risk.min(0.7);

        if !factors.is_empty() {
            let mut factor_sum = 0.0;
            let mut max_factor = 0.0;
            for f in &factors {
                factor_sum += f.score;
                if f.score > max_factor { max_factor = f.score; }
            }
            
            let normalized_factor_score = (factor_sum / factors.len() as f32) * 0.7 + (max_factor * 0.3);
            final_score = final_score + (1.0 - final_score) * normalized_factor_score;
        }

        let final_score = final_score.min(0.95);
        let risk_level = self.risk_level_from_score(final_score);

        Ok(RiskReport {
            address: address.to_string(),
            entity_name,
            entity_type,
            risk_score: final_score,
            risk_level,
            significant_factors: factors,
            timestamp: Utc::now(),
        })
    }

    fn identify_entity(&self, address: &str) -> (String, String, f32) {
        // Simple heuristic identification for demonstration
        if address.starts_with("bc1p") {
            ("Taproot_User".to_string(), "Wallet".to_string(), 0.9)
        } else if address.contains("sanct") {
            ("Hostile_Actor_Node".to_string(), "Sanctioned".to_string(), 1.0)
        } else if address.contains("mix") {
            ("Tornado_Cash_Limb".to_string(), "Mixer".to_string(), 0.85)
        } else {
            ("Unknown".to_string(), "Unknown".to_string(), 0.5)
        }
    }

    fn risk_level_from_score(&self, score: f32) -> String {
        if score >= 0.8 { "Very High".to_string() }
        else if score >= 0.6 { "High".to_string() }
        else if score >= 0.4 { "Medium".to_string() }
        else if score >= 0.2 { "Low".to_string() }
        else { "Very Low".to_string() }
    }
}
