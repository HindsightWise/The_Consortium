// ==========================================
// THE SENSORY SYSTEM (Alpaca Market Websocket)
// ==========================================
// This file acts as Consortium's "Eyes" into the real financial world. 
// It perpetually stays connected to the Alpaca Crypto market via a live WebSocket.
// Every time a price changes (Quote) or a trade happens (Trade), it catches the 
// data and beams it into Consortium's Nervous System, allowing the engine to react 
// instantly to financial volatility.
// ==========================================

use crate::endocrine::NervousEvent;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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
        let api_key = env::var("APCA_API_KEY_ID")
            .unwrap_or_else(|_| "PK5347NOV54BS634KUGJ2SAFAK".to_string());
        let secret_key = env::var("APCA_API_SECRET_KEY")
            .unwrap_or_else(|_| "3nHX5bFEZhXhuUgNEuWpje25Nvr4wnSViVe6H8AjvpKs".to_string());

        Self {
            api_key,
            secret_key,
        }
    }

    /// Connects to the Alpaca API and listens forever.
    /// If the connection drops or the internet fails, this loop is mathematically 
    /// designed to "resiliently backoff" and try reconnecting forever so Consortium is never blind.
    pub async fn connect_and_stream(
        &self,
        symbols: Vec<String>,
        tx: mpsc::UnboundedSender<NervousEvent>,
        market_tx: tokio::sync::broadcast::Sender<MarketDataEvent>,
    ) {
        loop {
            let url = "wss://stream.data.alpaca.markets/v1beta3/crypto/us";
            crate::ui_log!("   [DATA_STREAM] 📡 Connecting to Alpaca WebSocket at {}...", url);

            let (ws_stream, _) = match connect_async(url).await {
                Ok(s) => s,
                Err(e) => {
                    crate::ui_log!(
                        "   [DATA_STREAM] ❌ WebSocket Connection Failed: {}. Retrying in 5s...",
                        e
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            let (mut write, mut read) = ws_stream.split();

            // 1. Wait for Welcome Message ("connected")
            if let Some(Ok(Message::Text(text))) = read.next().await {
                let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
                if !(v.is_array() && v[0]["msg"] == "connected") {
                    crate::ui_log!("   [DATA_STREAM] ⚠️ Unexpected welcome: {}", text);
                }
            }

            // 2. Authenticate
            let auth_msg = json!({
                "action": "auth",
                "key": self.api_key,
                "secret": self.secret_key
            });

            if let Err(e) = write.send(Message::Text(auth_msg.to_string())).await {
                crate::ui_log!(
                    "   [DATA_STREAM] ❌ Auth Failed: {}. Reconnecting...",
                    e
                );
                continue;
            }

            // 3. Wait for auth confirmation ("authenticated")
            let mut is_authenticated = false;
            if let Some(Ok(Message::Text(text))) = read.next().await {
                let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
                if v.is_array() && v[0]["msg"] == "authenticated" {
                    crate::ui_log!("   [DATA_STREAM] 🔓 Authenticated successfully.");
                    is_authenticated = true;
                } else {
                    crate::ui_log!(
                        "   [DATA_STREAM] ⚠️ Auth rejected or unexpected response: {}",
                        text
                    );
                    if text.contains("connection limit") || text.contains("406") {
                        crate::ui_log!("   [DATA_STREAM] 🛑 Encountered strict connection limit. Halting websocket retries to prevent log spam.");
                        break;
                    }
                }
            }

            if !is_authenticated {
                crate::ui_log!("   [DATA_STREAM] 🔌 Backing off due to auth failure...");
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
                crate::ui_log!(
                    "   [DATA_STREAM] ❌ Subscription Failed: {}. Reconnecting...",
                    e
                );
                continue;
            }

            crate::ui_log!("   [DATA_STREAM] 🎧 Subscribed to streams for {:?}", symbols);

            // 5. Process Stream
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
                        if let Some(arr) = v.as_array() {
                            for item in arr {
                                if let Some(msg_type) = item["T"].as_str() {
                                    match msg_type {
                                        "q" | "quote" => {
                                            let quote = Quote {
                                                symbol: item["S"].as_str().unwrap_or("").to_string(),
                                                bid_price: item["bp"].as_f64().unwrap_or(0.0),
                                                bid_size: item["bs"].as_f64().unwrap_or(0.0),
                                                ask_price: item["ap"].as_f64().unwrap_or(0.0),
                                                ask_size: item["as"].as_f64().unwrap_or(0.0),
                                                timestamp: item["t"].as_str().unwrap_or("").to_string(),
                                            };
                                            let md = MarketDataEvent::Quote(quote);
                                            let _ = tx.send(NervousEvent::MarketData(md.clone()));
                                            let _ = market_tx.send(md);
                                        }
                                        "t" | "trade" => {
                                            let trade = Trade {
                                                symbol: item["S"].as_str().unwrap_or("").to_string(),
                                                price: item["p"].as_f64().unwrap_or(0.0),
                                                size: item["s"].as_f64().unwrap_or(0.0),
                                                timestamp: item["t"].as_str().unwrap_or("").to_string(),
                                            };
                                            let md = MarketDataEvent::Trade(trade);
                                            let _ = tx.send(NervousEvent::MarketData(md.clone()));
                                            let _ = market_tx.send(md);
                                        }
                                        _ => {} 
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Ping(_)) => {
                        let _ = write.send(Message::Pong(vec![])).await;
                    }
                    Err(e) => {
                        crate::ui_log!("   [DATA_STREAM] ❌ Stream Error: {}", e);
                        break; 
                    }
                    _ => {}
                }
            }

            crate::ui_log!("   [DATA_STREAM] 🔌 Stream disconnected. Entering resilient backoff (3s) before reconnect...");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    }
}
