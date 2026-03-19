use anchor_lang::prelude::*;

// In a real Solana program, this would be a proper macro invocation, 
// but to ensure it compiles within the `the_consortium` crate as an internal mock,
// we will structure it similarly but not compile it as a BPF program directly here.

#[derive(Clone, Debug)]
pub struct SigmaBondState {
    pub issuer: Pubkey,
    pub principal: u64,
    pub maturity: u64,
    pub leverage_factor: f64,
    pub bamm_index_oracle: Pubkey,
    pub base_volatility: f64,
    pub is_active: bool,
}

impl SigmaBondState {
    pub fn initialize(
        issuer: Pubkey,
        bamm_oracle: Pubkey,
        principal: u64,
        maturity_days: u64,
        leverage_factor: f64,
    ) -> Self {
        let current_time = chrono::Utc::now().timestamp() as u64; // mock clock
        Self {
            issuer,
            principal,
            maturity: current_time + (maturity_days * 86400),
            leverage_factor,
            bamm_index_oracle: bamm_oracle,
            base_volatility: 50.0,
            is_active: true,
        }
    }

    pub fn calculate_yield(&self, bamm_index: f64) -> u64 {
        let yield_ratio = (bamm_index * self.leverage_factor) / self.base_volatility;
        let yield_amount = (self.principal as f64 * yield_ratio) as u64;
        yield_amount.min(self.principal * 2) // Cap at 200%
    }

    pub fn rebalance_peg(&self, bamm_index: f64) -> f64 {
        let current_value = self.principal + self.calculate_yield(bamm_index);
        let target_value = self.principal as f64 * (1.0 + (bamm_index * self.leverage_factor) / self.base_volatility);
        
        
        (current_value as f64 - target_value).abs() / target_value
    }
}
