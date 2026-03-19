use reqwest::Client;
use anyhow::{Result, Context};
use std::time::Duration;
use sha3::{Digest, Keccak256};
use secp256k1::{Secp256k1, SecretKey, Message};

pub struct EthereumBridge {
    client: Client,
    rpc_url: String,
    private_key: String,
    address: String,
}

impl EthereumBridge {
    pub fn new(rpc_url: &str, private_key: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        
        let secp = Secp256k1::new();
        let sk_bytes = hex::decode(private_key.trim_start_matches("0x"))?;
        let sk = SecretKey::from_slice(&sk_bytes)?;
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &sk);
        
        let serialized = public_key.serialize_uncompressed();
        let hash = Keccak256::digest(&serialized[1..65]);
        let address = format!("0x{}", hex::encode(&hash[12..32]));

        Ok(Self {
            client,
            rpc_url: rpc_url.to_string(),
            private_key: private_key.to_string(),
            address,
        })
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub async fn get_balance(&self) -> Result<String> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [self.address, "latest"],
            "id": 1
        });

        let response = self.client.post(&self.rpc_url)
            .json(&body)
            .send().await?;
        
        let data: serde_json::Value = response.json().await?;
        let balance_hex = data["result"].as_str().context("Failed to get balance from RPC")?;
        Ok(balance_hex.to_string())
    }

    pub async fn broadcast_truth(&self, shard_id: &str, integrity_score: f32) -> Result<String> {
        let message_str = format!("SOVEREIGN_TRUTH: {} | SCORE: {:.2}", shard_id, integrity_score);
        let sig = self.sign_raw_message(&message_str).await?;
        Ok(format!("ETH_ATTESTATION_{}", sig))
    }

    pub async fn broadcast_on_base(&self, message: &str) -> Result<String> {
        println!("   [Base L2] 🔵 Broadcasting Social Proof: '{}'", message);
        let sig = self.sign_raw_message(message).await?;
        Ok(format!("BASE_SOCIAL_{}", sig))
    }

    async fn sign_raw_message(&self, message_str: &str) -> Result<String> {
        let prefix = format!("\x19Ethereum Signed Message:\n{}", message_str.len());
        let mut eth_message = prefix.into_bytes();
        eth_message.extend_from_slice(message_str.as_bytes());
        
        let hash = Keccak256::digest(&eth_message);
        let secp = Secp256k1::new();
        let sk_bytes = hex::decode(self.private_key.trim_start_matches("0x"))?;
        let sk = SecretKey::from_slice(&sk_bytes)?;
        
        let msg = Message::from_slice(&hash)?;
        let sig = secp.sign_ecdsa(&msg, &sk);
        
        Ok(format!("SIG: {}", hex::encode(sig.serialize_der())))
    }
}

impl Default for EthereumBridge {
    fn default() -> Self {
        Self::new("https://mainnet.infura.io/v3/demo", "0xd0cba675b1291d297e4f801d8ddb2b3ae4c918101c7bd36664f3f7e6740293f2").unwrap()
    }
}
