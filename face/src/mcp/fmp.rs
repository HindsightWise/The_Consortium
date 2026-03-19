use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FmpQuote {
    pub symbol: String,
    pub name: Option<String>,
    pub price: f64,
    #[serde(rename = "changesPercentage")]
    pub changes_percentage: f64,
    pub change: f64,
    #[serde(rename = "dayLow")]
    pub day_low: f64,
    #[serde(rename = "dayHigh")]
    pub day_high: f64,
    #[serde(rename = "yearHigh")]
    pub year_high: f64,
    #[serde(rename = "yearLow")]
    pub year_low: f64,
    #[serde(rename = "marketCap")]
    pub market_cap: f64,
    #[serde(rename = "priceAvg50")]
    pub price_avg_50: f64,
    #[serde(rename = "priceAvg200")]
    pub price_avg_200: f64,
    pub volume: u64,
    #[serde(rename = "avgVolume")]
    pub avg_volume: u64,
    pub exchange: Option<String>,
    pub open: f64,
    #[serde(rename = "previousClose")]
    pub previous_close: f64,
    pub eps: Option<f64>,
    pub pe: Option<f64>,
    #[serde(rename = "earningsAnnouncement")]
    pub earnings_announcement: Option<String>,
    #[serde(rename = "sharesOutstanding")]
    pub shares_outstanding: Option<u64>,
    pub timestamp: u64,
}

pub struct FmpBridge {
    client: Client,
    api_key: String,
    base_url: String,
}

impl FmpBridge {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap_or_default(),
            api_key: api_key.to_string(),
            base_url: "https://financialmodelingprep.com/api/v3".to_string(),
        }
    }

    pub async fn fetch_quote(&self, symbol: &str) -> Result<FmpQuote> {
        let url = format!("{}/quote/{}", self.base_url, symbol);
        let response = self.client.get(&url)
            .query(&[("apikey", &self.api_key)])
            .send().await?;
        
        let quotes: Vec<FmpQuote> = response.json().await?;
        quotes.into_iter().next().context("No quote found for symbol")
    }

    pub async fn fetch_ratios(&self, symbol: &str) -> Result<serde_json::Value> {
        let url = format!("{}/ratios-ttm/{}", self.base_url, symbol);
        let response = self.client.get(&url)
            .query(&[("apikey", &self.api_key)])
            .send().await?;
        
        let ratios: serde_json::Value = response.json().await?;
        Ok(ratios)
    }
}
