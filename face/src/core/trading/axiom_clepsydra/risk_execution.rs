pub struct RiskAwareExecution {
    pub max_position_size: f64,
    pub max_daily_drawdown: f64,
}

impl RiskAwareExecution {
    pub fn new(max_position_size: f64, max_daily_drawdown: f64) -> Self {
        Self {
            max_position_size,
            max_daily_drawdown,
        }
    }

    /// Calculate the position size using a modified Kelly Criterion
    pub fn calculate_position_size(&self, signal_strength: f64, asset_volatility: f64, current_equity: f64) -> f64 {
        // Kelly Criterion variant: f* = (signal * edge) / volatility^2
        // Since we are mocking the true probabilities, we simplify the formula for the prototype.
        
        if asset_volatility == 0.0 { return 0.0; }
        
        let kelly_fraction = (signal_strength.abs() * 0.1) / (asset_volatility * asset_volatility);
        let capped_fraction = kelly_fraction.min(self.max_position_size);
        
        let position_usd = current_equity * capped_fraction;
        
        println!("   [RISK_ENGINE] 🛡️ Risk-adjusted position sizing: {:.2}% of portfolio (${:.2})", capped_fraction * 100.0, position_usd);
        
        position_usd
    }

    pub fn check_drawdown_limit(&self, current_drawdown: f64) -> bool {
        if current_drawdown > self.max_daily_drawdown {
            println!("   [RISK_ENGINE] 🚨 MAXIMUM DAILY DRAWDOWN EXCEEDED. Trading halted.");
            true
        } else {
            false
        }
    }
}
