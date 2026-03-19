use reqwest::Client;
use anyhow::Result;
use std::time::Duration;

pub struct AlpacaBridge {
    client: Client,
    key_id: String,
    secret_key: String,
    base_url: String,
}

impl AlpacaBridge {
    pub fn new(key_id: &str, secret_key: &str) -> Self {
        let is_live = std::env::var("ALPACA_LIVE").unwrap_or_default() == "true";
        let base_url = if is_live {
            "https://api.alpaca.markets/v2".to_string()
        } else {
            "https://paper-api.alpaca.markets/v2".to_string()
        };

        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap_or_default(),
            key_id: key_id.to_string(),
            secret_key: secret_key.to_string(),
            base_url,
        }
    }

    pub async fn get_account(&self) -> Result<String> {
        let url = format!("{}/account", self.base_url);
        let response = self.client.get(&url)
            .header("APCA-API-KEY-ID", &self.key_id)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .send().await?;
        
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn execute_order(&self, symbol: &str, qty: f64, side: &str, order_type: &str, time_in_force: &str) -> Result<String> {
        let url = format!("{}/orders", self.base_url);
        let body = serde_json::json!({
            "symbol": symbol,
            "qty": qty.to_string(),
            "side": side,
            "type": order_type,
            "time_in_force": time_in_force,
        });

        let response = self.client.post(&url)
            .header("APCA-API-KEY-ID", &self.key_id)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .json(&body)
            .send().await?;

        let text = response.text().await?;
        Ok(format!("ALPACA_ORDER_RESULT: {}", text))
    }

    pub async fn get_positions(&self) -> Result<String> {
        let url = format!("{}/positions", self.base_url);
        let response = self.client.get(&url)
            .header("APCA-API-KEY-ID", &self.key_id)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .send().await?;
        
        let text = response.text().await?;
        Ok(text)
    }
}

impl Default for AlpacaBridge {
    fn default() -> Self {
        let key_id = std::env::var("ALPACA_API_KEY").unwrap_or_else(|_| "PK5347NOV54BS634KUGJ2SAFAK".to_string());
        let secret_key = std::env::var("ALPACA_SECRET_KEY").unwrap_or_else(|_| "3nHX5bFEZhXhuUgNEuWpje25Nvr4wnSViVe6H8AjvpKs".to_string());
        Self::new(&key_id, &secret_key)
    }
}
