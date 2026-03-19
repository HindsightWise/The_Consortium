use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: i64,
}

pub struct OrderBookImbalanceEngine {
    decay_factor: f64,
    depth: usize,
}

impl OrderBookImbalanceEngine {
    pub fn new(depth: usize) -> Self {
        Self {
            decay_factor: 0.8, // Exponential decay for deeper levels
            depth,
        }
    }

    /// Calculate Volume-Weighted Order Book Imbalance (VOI)
    /// Returns scalar from -1.0 (extreme sell pressure) to +1.0 (extreme buy pressure)
    pub fn calculate_imbalance(&self, snapshot: &OrderBookSnapshot) -> f64 {
        let mut bid_weight = 0.0;
        let mut ask_weight = 0.0;
        let mut total_weight = 0.0;

        let bid_iter = snapshot.bids.iter().take(self.depth);
        let ask_iter = snapshot.asks.iter().take(self.depth);

        for (i, bid) in bid_iter.enumerate() {
            let weight = self.decay_factor.powi(i as i32);
            bid_weight += bid.size * weight;
            total_weight += bid.size * weight;
        }

        for (i, ask) in ask_iter.enumerate() {
            let weight = self.decay_factor.powi(i as i32);
            ask_weight += ask.size * weight;
            total_weight += ask.size * weight;
        }

        if total_weight == 0.0 {
            return 0.0;
        }

        // Normalized imbalance: (Bids - Asks) / Total
        (bid_weight - ask_weight) / total_weight
    }

    /// Volume-Synchronized Probability of Informed Trading (VPIN) - Simplified Proxy
    pub fn calculate_vpin_proxy(&self, recent_buy_volume: f64, recent_sell_volume: f64) -> f64 {
        let total_volume = recent_buy_volume + recent_sell_volume;
        if total_volume == 0.0 {
            return 0.0;
        }
        
        // Order flow toxicity: |Buy Vol - Sell Vol| / Total Vol
        (recent_buy_volume - recent_sell_volume).abs() / total_volume
    }
}
