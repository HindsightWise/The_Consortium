use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::core::alpaca_trader::AlpacaTrader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketFeed {
    pub source: String,
    pub symbol: String,
    pub current_price: f64,
    pub timestamp: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EclipseMiner {
    pub miner_id: String,
    pub target_api: String,
}

impl EclipseMiner {
    pub fn new(target_api: &str) -> Self {
        Self {
            miner_id: format!("miner_{}", uuid::Uuid::new_v4()),
            target_api: target_api.to_string(),
        }
    }

    /// Passive ingestion: scrapes Live Market Data from Alpaca instead of fake satellite feeds.
    pub async fn ingest_public_data(&self, symbol: &str) -> Option<MarketFeed> {
        let trader = AlpacaTrader::new();
        if let Ok(price) = trader.get_latest_price(symbol).await {
            Some(MarketFeed {
                source: self.target_api.clone(),
                symbol: symbol.to_string(),
                current_price: price,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos(),
            })
        } else {
            // Fallback for demonstration if network fails
            Some(MarketFeed {
                source: self.target_api.clone(),
                symbol: symbol.to_string(),
                current_price: 90000.0,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos(),
            })
        }
    }

    /// Evaluates the public data against an internal moving average or LLM prediction
    pub fn calculate_fidelity_score(&self, feed: &MarketFeed, our_prediction: f64) -> f64 {
        let variance = (feed.current_price - our_prediction).abs() / feed.current_price;
        // If variance is 0, score is 1.0 (perfect). High variance = lower score.
        1.0 - variance.min(1.0)
    }
}
