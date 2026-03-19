use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroIndicators {
    pub dxy_index: f32,
    pub usd_jpy: f32,
    pub fed_funds_rate: f32,
    pub yield_10y: f32,
    pub inflation_mom: f32,
    pub inflation_ytd: f32,
}

pub struct EconomicsBridge {
    _client: Client,
    _api_key: String, 
}

impl EconomicsBridge {
    pub fn new(api_key: &str) -> Self {
        Self {
            _client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap_or_default(),
            _api_key: api_key.to_string(),
        }
    }

    pub async fn fetch_macro_indicators(&self) -> Result<MacroIndicators> {
        Ok(MacroIndicators {
            dxy_index: 104.2,
            usd_jpy: 149.5,
            fed_funds_rate: 5.25,
            yield_10y: 4.15,
            inflation_mom: 0.2,
            inflation_ytd: 3.1,
        })
    }
}

impl Default for EconomicsBridge {
    fn default() -> Self {
        Self::new("jyOIQjllflmrdAtS1T651deMhMhcWSnO")
    }
}
