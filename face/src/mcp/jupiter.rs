// ARCHITECTURAL BLUEPRINT: JUPITER BRIDGE FOR ERMM v1.0
// PATH: src/mcp/jupiter.rs
// STATUS: RFC_1771891196 (APPROVED)

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context, anyhow};
use reqwest::Client;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterQuote {
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String, // lamports
    pub out_amount: String, // lamports
    pub other_amount_threshold: String,
    pub slippage_bps: u16,
    pub swap_mode: String,
    pub price_impact_pct: f64,
    pub route: Vec<JupiterRouteStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterRouteStep {
    pub percent: u8,
    pub dex: String,
    pub input_mint: String,
    pub output_mint: String,
}

pub struct JupiterBridge {
    client: Client,
    base_url: String,
}

impl Default for JupiterBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl JupiterBridge {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap_or_default(),
            base_url: "https://quote-api.jup.ag/v6".to_string(),
        }
    }

    /// Canonical Truth Layer: Fetch executable quote from Jupiter
    pub async fn fetch_quote(
        &self,
        input_mint: &str,   // e.g., "So11111111111111111111111111111111111111112" (SOL)
        output_mint: &str,  // e.g., "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" (USDC)
        amount_lamports: u64,
        slippage_bps: u16,
    ) -> Result<JupiterQuote> {
        let url = format!("{}/quote", self.base_url);
        let params = [
            ("inputMint", input_mint),
            ("outputMint", output_mint),
            ("amount", &amount_lamports.to_string()),
            ("slippageBps", &slippage_bps.to_string()),
            ("swapMode", "ExactIn"),
        ];
        
        let response = self.client.get(&url).query(&params).send().await
            .context("Jupiter quote request failed")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow!(
                "Jupiter quote error {}: {}", status, body
            ));
        }
        
        let quote: JupiterQuote = response.json().await?;
        Ok(quote)
    }

    /// Bridge Verification: Compare observational price vs. executable quote
    /// Returns (is_valid, deviation_pct). Valid if deviation ≤ 3%.
    pub fn verify_against_observation(
        observed_price: f64,     // from DexScreener
        quote: &JupiterQuote,
        input_decimal: u8,
        output_decimal: u8,
    ) -> (bool, f64) {
        let in_amount: f64 = quote.in_amount.parse().unwrap_or(0.0);
        let out_amount: f64 = quote.out_amount.parse().unwrap_or(0.0);
        
        if in_amount <= 0.0 || observed_price <= 0.0 {
            return (false, 100.0);
        }
        
        let executable_price = (out_amount / 10f64.powi(output_decimal as i32)) 
                             / (in_amount / 10f64.powi(input_decimal as i32));
        
        let deviation_pct = ((executable_price - observed_price).abs() / observed_price) * 100.0;
        
        (deviation_pct <= 3.0, deviation_pct)
    }
}

// INTEGRATION PATH: src/core/ermm.rs (ERMM v1.0)
// Add JupiterBridge as dependency, modify price verification logic:
// 1. Call DexScreener for observational awareness
// 2. Call JupiterBridge::fetch_quote for executable truth
// 3. Run verify_against_observation with 3% threshold
// 4. If PASS → proceed with Jupiter swap instruction
// 5. If FAIL → trigger systemic risk alert, abort trade
// 6. Log Metabolic Learning Artifact: deviation %, timestamp, success/fail