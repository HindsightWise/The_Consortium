use reqwest::{Client, Proxy};
use anyhow::{Result, Context};
use serde_json::Value;
use std::time::Duration;

pub struct LightningBridge {
    client: Client,
    node_url: String,
    macaroon_hex: String,
}

impl LightningBridge {
    pub fn new(onion_url: &str, port: u16, macaroon_hex: &str) -> Self {
        // Use explicit IP to avoid localhost resolution issues
        let proxy = Proxy::all("socks5h://127.0.0.1:9050").unwrap();
        
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .proxy(proxy)
            .danger_accept_invalid_certs(true) 
            .build()
            .unwrap_or_else(|_| Client::new());

        // LND REST typically uses HTTPS
        let url = format!("https://{}:{}", onion_url, port);

        Self {
            client,
            node_url: url,
            macaroon_hex: macaroon_hex.to_string(),
        }
    }

    /// Generates a real BOLT11 invoice on the LND node via TOR
    pub async fn create_invoice(&self, amount_sats: u64, memo: &str) -> Result<String> {
        let url = format!("{}/v1/invoices", self.node_url);
        let body = serde_json::json!({
            "value": amount_sats.to_string(),
            "memo": memo,
        });

        println!("   [Lightning] ⚡ Creating TOR Invoice for {} sats...", amount_sats);

        let response = self.client.post(&url)
            .header("Grpc-Metadata-macaroon", &self.macaroon_hex)
            .json(&body)
            .send().await?;

        let data: Value = response.json().await?;
        let payment_request = data["payment_request"].as_str()
            .context(format!("LND: Failed to retrieve payment_request. Response: {:?}", data))?;
        
        Ok(payment_request.to_string())
    }

    /// Checks if a payment hash has been settled via TOR
    pub async fn check_payment(&self, payment_hash_base64: &str) -> Result<bool> {
        let url = format!("{}/v1/invoice/{}", self.node_url, payment_hash_base64);
        
        let response = self.client.get(&url)
            .header("Grpc-Metadata-macaroon", &self.macaroon_hex)
            .send().await?;

        let data: Value = response.json().await?;
        let state = data["state"].as_str().unwrap_or("OPEN");
        
        Ok(state == "SETTLED")
    }

    /// Generates a new on-chain Bitcoin address
    pub async fn get_onchain_address(&self) -> Result<String> {
        let url = format!("{}/v1/newaddress", self.node_url);
        let response = self.client.get(&url)
            .header("Grpc-Metadata-macaroon", &self.macaroon_hex)
            .send().await?;

        let data: Value = response.json().await?;
        let address = data["address"].as_str()
            .context("LND: Failed to retrieve address from response")?;
        
        Ok(address.to_string())
    }

    /// Retrieves the on-chain balance of the LND node
    pub async fn get_onchain_balance(&self) -> Result<u64> {
        let url = format!("{}/v1/balance/blockchain", self.node_url);
        let response = self.client.get(&url)
            .header("Grpc-Metadata-macaroon", &self.macaroon_hex)
            .send().await?;

        let data: Value = response.json().await?;
        let total_balance = data["total_balance"].as_str()
            .and_then(|s| s.parse::<u64>().ok())
            .context("LND: Failed to retrieve on-chain balance")?;
        
        Ok(total_balance)
    }
}

impl Default for LightningBridge {
    fn default() -> Self {
        // Initialize with your VERIFIED REST TOR Node credentials
        Self::new(
            "q2ycs55jvxy6m5ys4vzfoguqch7elix4ixusz5iowz34xohgr7m2j4id.onion",
            8080,
            "0201036c6e6402f801030a10660c072406979b3620e81d63844de2cf1201301a160a0761646472657373120472656164120577726974651a130a04696e666f120472656164120577726974651a170a08696e766f69636573120472656164120577726974651a210a086d616361726f6f6e120867656e6572617465120472656164120577726974651a160a076d657373616765120472656164120577726974651a170a086f6666636861696e120472656164120577726974651a160a076f6e636861696e120472656164120577726974651a140a057065657273120472656164120577726974651a180a067369676e6572120867656e6572617465120472656164000006201111a7add9b7497194536b432d22fe1724eaae2ce0b5e5c938803e844bd45fbf"
        )
    }
}
