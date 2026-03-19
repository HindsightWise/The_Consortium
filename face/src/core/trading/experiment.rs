use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradingMethodology {
    MacroSwing,
    GridMaker,
    PairsArbitrage,
}

impl std::fmt::Display for TradingMethodology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradingMethodology::MacroSwing => write!(f, "MACRO_SWING"),
            TradingMethodology::GridMaker => write!(f, "GRID_MAKER"),
            TradingMethodology::PairsArbitrage => write!(f, "PAIRS_ARB"),
        }
    }
}

pub struct ExperimentTracker {
    allocations: HashMap<TradingMethodology, f64>,
}

impl Default for ExperimentTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ExperimentTracker {
    pub fn new() -> Self {
        let mut allocations = HashMap::new();
        // Conceptually allocate paper portfolio for strict segmentation.
        allocations.insert(TradingMethodology::MacroSwing, 30_000.0);
        allocations.insert(TradingMethodology::GridMaker, 30_000.0);
        allocations.insert(TradingMethodology::PairsArbitrage, 30_000.0);
        
        Self { allocations }
    }

    pub async fn dispatch_signal(
        &self, 
        methodology: TradingMethodology, 
        symbol: &str, 
        action: &str, 
        target_usd: f64
    ) -> Result<String, String> {
        // Enforce Allocation Limit
        let max_alloc = self.allocations.get(&methodology).unwrap_or(&0.0);
        if target_usd > *max_alloc {
            return Err(format!("Requested allocation (${:.2}) exceeds method limit (${:.2})", target_usd, max_alloc));
        }

        // Generate a uniquely tagged client order ID so we can track exact strategy PnL via Alpaca
        let client_order_id = format!("{}_{}_{}", methodology, symbol.replace("/", ""), chrono::Utc::now().timestamp());
        
        println!("   [EXPERIMENT_TRACKER] 📊 Dispatching Signal | Method: {} | Symbol: {} | Action: {} | Target ${:.2}", methodology, symbol, action, target_usd);
        
        // Let Strategy handle the physical API call using the tag
        Ok(client_order_id)
    }
}
