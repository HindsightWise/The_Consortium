use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::env;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub bid_price: f64,
    pub bid_size: f64,
    pub ask_price: f64,
    pub ask_size: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub price: f64,
    pub size: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub enum MarketDataEvent {
    Quote(Quote),
    Trade(Trade),
    Disconnected,
    ImbalanceTrigger { symbol: String, side: String, ratio: f64 },
}

pub struct AlpacaWebSocket {
    api_key: String,
    secret_key: String,
}

impl Default for AlpacaWebSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl AlpacaWebSocket {
    pub fn new() -> Self {
        let api_key = env::var("APCA_API_KEY_ID").unwrap_or_else(|_| "PK5347NOV54BS634KUGJ2SAFAK".to_string());
        let secret_key = env::var("APCA_API_SECRET_KEY").unwrap_or_else(|_| "3nHX5bFEZhXhuUgNEuWpje25Nvr4wnSViVe6H8AjvpKs".to_string());
        
        Self {
            api_key,
            secret_key,
        }
    }

    pub async fn connect_and_stream(
        &self, 
        symbols: Vec<String>, 
        tx: mpsc::Sender<MarketDataEvent>
    ) {
        loop {
            let url = "wss://stream.data.alpaca.markets/v1beta3/crypto/us";
            println!("   [DATA_STREAM] 📡 Connecting to Alpaca WebSocket at {}...", url);

        let (ws_stream, _) = match connect_async(url).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("   [DATA_STREAM] ❌ WebSocket Connection Failed: {}. Retrying in 5 seconds...", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        let (mut write, mut read) = ws_stream.split();

        // 1. Wait for Welcome Message ("connected")
        if let Some(Ok(Message::Text(text))) = read.next().await {
            let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
            if !(v.is_array() && v[0]["msg"] == "connected") {
                eprintln!("   [DATA_STREAM] ⚠️ Unexpected welcome: {}", text);
            }
        }

        // 2. Authenticate
        let auth_msg = json!({
            "action": "auth",
            "key": self.api_key,
            "secret": self.secret_key
        });
        
        if let Err(e) = write.send(Message::Text(auth_msg.to_string())).await {
            eprintln!("   [DATA_STREAM] ❌ Auth Failed: {}. Reconnecting...", e);
            continue;
        }

        // 3. Wait for auth confirmation ("authenticated")
        let mut is_authenticated = false;
        if let Some(Ok(Message::Text(text))) = read.next().await {
            let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
            if v.is_array() && v[0]["msg"] == "authenticated" {
                println!("   [DATA_STREAM] 🔓 Authenticated successfully.");
                is_authenticated = true;
            } else {
                eprintln!("   [DATA_STREAM] ⚠️ Auth rejected or unexpected response: {}", text);
                if text.contains("connection limit") || text.contains("406") {
                    eprintln!("   [DATA_STREAM] 🛑 Encountered strict connection limit. Halting websocket retries to prevent log spam.");
                    break;
                }
            }
        }
        
        if !is_authenticated {
            println!("   [DATA_STREAM] 🔌 Backing off due to auth failure...");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            continue;
        }

        // 4. Subscribe to Quotes and Trades
        let sub_msg = json!({
            "action": "subscribe",
            "trades": symbols,
            "quotes": symbols
        });
        
        if let Err(e) = write.send(Message::Text(sub_msg.to_string())).await {
            eprintln!("   [DATA_STREAM] ❌ Subscription Failed: {}. Reconnecting...", e);
            continue;
        }

        println!("   [DATA_STREAM] 🎧 Subscribed to streams for {:?}", symbols);

        // 3. Process Stream
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
                    if let Some(arr) = v.as_array() {
                        for item in arr {
                            if let Some(msg_type) = item["T"].as_str() {
                                match msg_type {
                                    "q" | "quote" => {
                                        // Quote
                                        let quote = Quote {
                                            symbol: item["S"].as_str().unwrap_or("").to_string(),
                                            bid_price: item["bp"].as_f64().unwrap_or(0.0),
                                            bid_size: item["bs"].as_f64().unwrap_or(0.0),
                                            ask_price: item["ap"].as_f64().unwrap_or(0.0),
                                            ask_size: item["as"].as_f64().unwrap_or(0.0),
                                            timestamp: item["t"].as_str().unwrap_or("").to_string(),
                                        };
                                        let _ = tx.send(MarketDataEvent::Quote(quote)).await;
                                    }
                                    "t" | "trade" => {
                                        // Trade
                                        let trade = Trade {
                                            symbol: item["S"].as_str().unwrap_or("").to_string(),
                                            price: item["p"].as_f64().unwrap_or(0.0),
                                            size: item["s"].as_f64().unwrap_or(0.0),
                                            timestamp: item["t"].as_str().unwrap_or("").to_string(),
                                        };
                                        let _ = tx.send(MarketDataEvent::Trade(trade)).await;
                                    }
                                    _ => {} // Ignore other message types (like "success", "error")
                                }
                            }
                        }
                    }
                }
                Ok(Message::Ping(_)) => {
                    // Respond to pings to keep connection alive
                    let _ = write.send(Message::Pong(vec![])).await;
                }
                Err(e) => {
                    eprintln!("   [DATA_STREAM] ❌ Stream Error: {}", e);
                    break; // Break the inner loop to trigger the reconnect
                }
                _ => {}
            }
        }
        
        println!("   [DATA_STREAM] 🔌 Stream disconnected. Entering resilient backoff (3s) before reconnect...");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    }
}
