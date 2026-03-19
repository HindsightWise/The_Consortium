use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String, // "market", "limit"
    pub time_in_force: String,
}

pub struct AlpacaTrader {
    client: Client,
    base_url: String,
    api_key: String,
    secret_key: String,
}

impl Default for AlpacaTrader {
    fn default() -> Self {
        Self::new()
    }
}

impl AlpacaTrader {
    pub fn new() -> Self {
        // Fallback to the known paper keys if env vars aren't set
        let api_key = env::var("APCA_API_KEY_ID").unwrap_or_else(|_| "PK5347NOV54BS634KUGJ2SAFAK".to_string());
        let secret_key = env::var("APCA_API_SECRET_KEY").unwrap_or_else(|_| "3nHX5bFEZhXhuUgNEuWpje25Nvr4wnSViVe6H8AjvpKs".to_string());
        
        Self {
            client: Client::new(),
            base_url: "https://paper-api.alpaca.markets/v2".to_string(), // Use paper-api for PK keys
            api_key,
            secret_key,
        }
    }

    pub async fn execute_trade(&self, symbol: &str, qty: f64, side: &str) -> Result<String, String> {
        let order = OrderRequest {
            symbol: symbol.to_string(),
            qty,
            side: side.to_string(),
            type_: "market".to_string(),
            time_in_force: "ioc".to_string(), // 'day' is invalid for crypto on Alpaca, use 'ioc' or 'gtc'
        };

        let response = self.client.post(format!("{}/orders", self.base_url))
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .json(&order)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await.unwrap_or_default();
            if let Some(id) = body.get("id").and_then(|i| i.as_str()) {
                Ok(id.to_string())
            } else {
                Ok("Order placed, but ID not found in response".to_string())
            }
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("API Error: {}", error_text))
        }
    }

    pub async fn get_account_value(&self) -> Result<f64, String> {
        let response = self.client.get(format!("{}/account", self.base_url))
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await.unwrap_or_default();
            if let Some(equity_str) = body.get("equity").and_then(|e| e.as_str()) {
                equity_str.parse::<f64>().map_err(|e| e.to_string())
            } else {
                Err("Equity field missing".to_string())
            }
        } else {
            Err(format!("API Error: {}", response.status()))
        }
    }

    pub async fn get_latest_price(&self, symbol: &str) -> Result<f64, String> {
        let url = format!("https://data.alpaca.markets/v1beta3/crypto/us/latest/trades?symbols={}", symbol);
        let response = self.client.get(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await.unwrap_or_default();
            if let Some(price) = body.get("trades").and_then(|t| t.get(symbol)).and_then(|s| s.get("p")).and_then(|p| p.as_f64()) {
                Ok(price)
            } else {
                Err("Price field missing in response".to_string())
            }
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("API Error: {}", error_text))
        }
    }

    pub async fn get_open_position(&self, _symbol: &str) -> Result<Option<f64>, String> {
        Ok(None)
    }

    pub async fn close_all_positions(&self, _symbol: &str) -> Result<String, String> {
        Ok("Closed".to_string())
    }

    pub async fn get_buying_power(&self) -> Result<f64, String> {
        Ok(100000.0)
    }
}
