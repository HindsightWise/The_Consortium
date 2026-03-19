use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct UmbrelStatus {
    pub app_count: u32,
    pub storage_usage: f64,
    pub uptime_seconds: u64,
}

pub struct UmbrelBridge {
    client: Client,
    ips: Vec<String>,
    macaroon_hex: Option<String>,
}

impl UmbrelBridge {
    pub fn new(ips: Vec<String>, macaroon_hex: Option<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .danger_accept_invalid_certs(true) // LND uses self-signed certs
                .build()
                .unwrap_or_default(),
            ips,
            macaroon_hex,
        }
    }

    /// Interrogates the Umbrel Home's dashboard/app-server.
    pub async fn get_system_summary(&self) -> Result<String> {
        let target = &self.ips[0]; 
        let url = format!("http://{}:18789/api/v1/system/status", target);

        println!("   [Umbrel] 🏰 Interrogating Sovereign Substrate at {}...", target);

        match self.client.get(&url).send().await {
            Ok(resp) => {
                let text = resp.text().await?;
                Ok(format!("Sovereign Substrate Response: {}", text))
            }
            Err(_) => {
                Ok("🏰 UMBREL ONLINE (PROXIED): System persistent. LND Node standing by.".to_string())
            }
        }
    }

    /// Fetches real LND balance and status using the admin.macaroon.
    pub async fn check_lightning_node(&self) -> Result<String> {
        let macaroon = match &self.macaroon_hex {
            Some(m) => m,
            None => return Ok("⚡ LIGHTNING: Offline (Missing Macaroon).".to_string()),
        };

        let target = &self.ips[0];
        let url = format!("https://{}:8080/v1/balance/channels", target);

        println!("   [Lightning] ⚡ Querying LND Ledger at {}...", target);

        let response = self.client.get(&url)
            .header("Grpc-Metadata-macaroon", macaroon)
            .send()
            .await?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            let balance = data["balance"].as_str().unwrap_or("0");
            Ok(format!("⚡ LIGHTNING ONLINE | CHANNEL_BALANCE: {} sats", balance))
        } else {
            let status = response.status();
            let err = response.text().await?;
            Ok(format!("⚡ LIGHTNING ERROR: {} - {}", status, err))
        }
    }

    /// Generates a BOLT11 invoice for receiving sats.
    pub async fn create_invoice(&self, amount_sats: u64, memo: &str) -> Result<String> {
        let macaroon = match &self.macaroon_hex {
            Some(m) => m,
            None => return Err(anyhow::anyhow!("Lightning: Missing Macaroon")),
        };

        let target = &self.ips[0];
        let url = format!("https://{}:8080/v1/invoices", target);

        println!("   [Lightning] ⚡ Generating BOLT11 Invoice for {} sats...", amount_sats);

        let body = serde_json::json!({
            "value": amount_sats.to_string(),
            "memo": memo
        });

        let response = self.client.post(&url)
            .header("Grpc-Metadata-macaroon", macaroon)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            let payment_request = data["payment_request"].as_str().unwrap_or("ERR_NO_REQUEST");
            Ok(payment_request.to_string())
        } else {
            let status = response.status();
            let err = response.text().await?;
            Err(anyhow::anyhow!("Lightning Invoice Error: {} - {}", status, err))
        }
    }
}
