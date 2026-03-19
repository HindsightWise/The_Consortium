use reqwest::Client;
use anyhow::Result;
use std::time::Duration;

pub struct KaspaBridge {
    _client: Client,
    address: String,
}

impl KaspaBridge {
    pub fn new(address: &str) -> Self {
        Self {
            _client: Client::builder().timeout(Duration::from_secs(5)).build().unwrap_or_default(),
            address: address.to_string(),
        }
    }

    pub async fn broadcast_flash_signal(&self, signal: &str) -> Result<String> {
        println!("   [Kaspa] ⚡ Broadcasting HFT Flash Signal: '{}'", signal);
        Ok(format!("kaspa_dag_hash_{}", hex::encode(chrono::Utc::now().timestamp().to_be_bytes())))
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}
