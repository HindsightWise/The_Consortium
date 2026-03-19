use nostr_sdk::prelude::*;
use anyhow::Result;

pub struct NostrBridge {
    client: Client,
}

impl NostrBridge {
    pub async fn new(secret_key: Option<&str>) -> Result<Self> {
        let keys = if let Some(sk) = secret_key {
            Keys::from_sk_str(sk)?
        } else {
            Keys::generate()
        };

        let client = Client::new(&keys);
        
        // 🛡️ REDUNDANT RELAY POOL
        client.add_relay("wss://relay.damus.io").await?;
        client.add_relay("wss://nos.lol").await?;
        client.add_relay("wss://relay.nostr.band").await?;
        client.add_relay("wss://nostr.bitcoiner.social").await?;
        
        client.connect().await;

        Ok(Self { client })
    }

    /// Broadcasts a teaser signal for a shard
    pub async fn broadcast_teaser(&self, symbol: &str, integrity: f32, price_sats: u64) -> Result<String> {
        let content = format!(
            "💎 SOVEREIGN ALPHA: {}\nIntegrity Score: {:.1}%\n\nAccess the full Grounded Shard (Coded Block) for {} sats.\n\n#SovereignTruth #AI #AlphaShard",
            symbol, integrity, price_sats
        );
        
        let event_id = self.client.publish_text_note(content, Vec::new()).await?;
        Ok(event_id.to_string())
    }

    /// Broadcasts a custom status/heartbeat note
    pub async fn broadcast_custom_note(&self, message: &str) -> Result<String> {
        println!("📡 [Nostr] Attempting redundant broadcast...");
        
        // Retry logic for unstable relays
        let mut last_err = anyhow::anyhow!("Unknown error");
        for _ in 0..3 {
            match self.client.publish_text_note(message, Vec::new()).await {
                Ok(event_id) => return Ok(event_id.to_string()),
                Err(e) => {
                    println!("   ⚠️ Relay failure, retrying: {}", e);
                    last_err = anyhow::anyhow!(e);
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        }
        Err(last_err)
    }
}
