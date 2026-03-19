use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationParams {
    pub symbol: String,
    pub amount: f64,
    pub strategy_type: String, // e.g., "MOMENTUM", "MEAN_REVERSION"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationResult {
    pub expected_yield: f64,
    pub slippage_estimate: f64,
    pub latency_impact_ms: u64,
    pub confidence_interval: f32,
    pub verdict: String,
}

pub struct MarketSimulator;

impl MarketSimulator {
    /// Simulates a trade based on HFTBacktest patterns.
    pub fn simulate(params: SimulationParams) -> SimulationResult {
        let mut rng = rand::thread_rng();
        
        // 1. Model Slippage: Larger orders in lower liquidity create more drag
        let slippage = (params.amount / 100000.0) * rng.gen_range(0.01..0.05);
        
        // 2. Model Latency: Simulate networking/processing delay
        let latency = rng.gen_range(5..150);
        
        // 3. Yield Calculation (Simulated Alpha)
        let base_yield = if params.strategy_type == "MOMENTUM" { 0.02 } else { 0.005 };
        let noise = rng.gen_range(-0.01..0.01);
        let expected_yield = base_yield + noise - slippage;

        let verdict = if expected_yield > 0.0 {
            "PROFITABLE_MODEL".to_string()
        } else {
            "NEGATIVE_EXPECTANCY".to_string()
        };

        SimulationResult {
            expected_yield,
            slippage_estimate: slippage,
            latency_impact_ms: latency,
            confidence_interval: 0.85,
            verdict,
        }
    }
}
