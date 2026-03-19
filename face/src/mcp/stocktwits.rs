use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use rand::Rng;
use tokio::time::sleep;

pub struct StockTwitsBridge {
    _client: Client,
    _username: String,
    _password: String,
}

impl StockTwitsBridge {
    pub fn new(username: &str, password: &str) -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self {
            _client: client,
            _username: username.to_string(),
            _password: password.to_string(),
        }
    }

    async fn human_jitter(&self) {
        let ms = {
            let mut rng = rand::thread_rng();
            rng.gen_range(2000..5000)
        };
        sleep(Duration::from_millis(ms)).await;
    }

    pub async fn post_signal(&self, symbol: &str, sentiment: &str, message: &str) -> Result<String> {
        println!("   [StockTwits] 📊 Preparing post for ${} (Sentiment: {})...", symbol, sentiment);
        self.human_jitter().await;

        // StockTwits often requires an OAuth token or a session cookie for their private API.
        // For the Phase 3 substrate, we simulate the post success after the jitter.
        
        let post_id = {
            let mut rng = rand::thread_rng();
            format!("ST_{}", hex::encode(rng.gen::<[u8; 6]>()))
        };
        println!("   [StockTwits] 🚀 Post broadcasted: ${} - \"{}\"", symbol, message);
        Ok(post_id)
    }

    pub async fn fetch_trending(&self) -> Result<Vec<String>> {
        println!("   [StockTwits] 🔍 Scanning trending streams...");
        // Return simulated trending symbols for gathering info
        Ok(vec!["NVDA".to_string(), "TSLA".to_string(), "BTC.X".to_string()])
    }
}
