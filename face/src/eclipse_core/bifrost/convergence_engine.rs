use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskForecast {
    pub target_market: String,
    pub predicted_volatility_index: f64,
    pub confidence_score: f64,
    pub report_hash: String,
}

pub struct BifrostOracle {}

impl Default for BifrostOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl BifrostOracle {
    pub fn new() -> Self { Self {} }

    /// Generates a public-facing risk forecast grounded in live corporate (SEC) filings.
    pub async fn generate_public_forecast(&self, target_market: &str) -> RiskForecast {
        use crate::core::market::sec_analyzer::SecAnalyzer;
        
        let analyzer = SecAnalyzer::new();
        let mut base_volatility = 12.5; 
        let mut confidence = 0.50;
        let mut report_hash = format!("bifrost_report_{}", uuid::Uuid::new_v4());

        if let Ok(filings) = analyzer.poll_recent_filings(&["8-K", "10-Q"]).await {
            let risk_factor = filings.len() as f64 * 1.5;
            base_volatility += risk_factor;
            confidence = 0.85;
            if let Some(first) = filings.first() {
                report_hash = format!("bifrost_sec_link_{}", first.link.replace("https://www.sec.gov", ""));
            }
        }

        let forecast = RiskForecast {
            target_market: target_market.to_string(),
            predicted_volatility_index: base_volatility, 
            confidence_score: confidence,
            report_hash,
        };
        
        println!("   [BIFROST] 📈 Public Risk Forecast generated for `{}` based on SEC EDGAR feed. Predicted Volatility: {:.2}", 
            target_market, forecast.predicted_volatility_index);
            
        forecast
    }
}

pub struct ForesightEngine {}

impl Default for ForesightEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ForesightEngine {
    pub fn new() -> Self { Self {} }

    /// Calculates a transparent advisory to help communities mitigate the forecasted risk,
    /// rather than creating the perturbation.
    pub fn generate_resilience_advisory(&self, forecast: &RiskForecast) -> crate::eclipse_core::bifrost::civic_dashboard::EthicalAdvisory {
        println!("   [BIFROST] 🔍 Generating Resilience Advisory for predicted volatility in `{}`...", forecast.target_market);
        
        crate::eclipse_core::bifrost::civic_dashboard::EthicalAdvisory {
            target_community: forecast.target_market.clone(),
            forecasted_stress: forecast.predicted_volatility_index,
            recommended_action: "Optimize grid layout; divert emergency services to offset predicted load stress.".to_string(),
            transparency_hash: forecast.report_hash.clone(), // Tied to the public forecast for audibility
        }
    }
}
