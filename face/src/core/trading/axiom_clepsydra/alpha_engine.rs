use chrono::{DateTime, Utc};
use std::collections::{VecDeque, HashMap};

use crate::core::trading::stream::alpaca_ws::{MarketDataEvent, Quote, Trade};

#[derive(Debug, Clone)]
pub enum SignalSource {
    OrderBookImbalance,
    StatisticalArbitrage,
    Sentiment,
}

#[derive(Debug, Clone)]
pub struct AlphaSignal {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub strength: f64,  // -1.0 to 1.0
    pub confidence: f64,
    pub sources: Vec<SignalSource>,
}

pub struct AlphaEngine {
    // VOI / VPIN state per symbol
    recent_trades: HashMap<String, VecDeque<Trade>>,
    recent_quotes: HashMap<String, VecDeque<Quote>>,
    _vpin_bucket_size: f64,
}

impl Default for AlphaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AlphaEngine {
    pub fn new() -> Self {
        Self {
            recent_trades: HashMap::new(),
            recent_quotes: HashMap::new(),
            _vpin_bucket_size: 50.0,
        }
    }

    pub fn process_event(&mut self, event: MarketDataEvent) -> Option<AlphaSignal> {
        let _target_symbol = match &event {
            MarketDataEvent::Quote(q) => {
                let symbol = q.symbol.clone();
                let quotes = self.recent_quotes.entry(q.symbol.clone()).or_insert_with(|| VecDeque::with_capacity(100));
                quotes.push_back(q.clone());
                if quotes.len() > 100 {
                    quotes.pop_front();
                }
                symbol
            }
            MarketDataEvent::Trade(t) => {
                let symbol = t.symbol.clone();
                let trades = self.recent_trades.entry(t.symbol.clone()).or_insert_with(|| VecDeque::with_capacity(1000));
                trades.push_back(t.clone());
                if trades.len() > 1000 {
                    trades.pop_front();
                }
                symbol
            }
            MarketDataEvent::Disconnected => {
                println!("   [ALPHA_ENGINE] ⚠️ Data stream disconnected.");
                return None;
            }
            MarketDataEvent::ImbalanceTrigger { symbol, .. } => {
                symbol.clone()
            }
        };

        self.calculate_signal(&_target_symbol)
    }

    fn calculate_signal(&self, symbol: &str) -> Option<AlphaSignal> {
        let quotes = self.recent_quotes.get(symbol)?;
        let trades = self.recent_trades.get(symbol)?;

        if quotes.is_empty() || trades.is_empty() {
            return None;
        }

        // Calculate VOI (Volume Order Imbalance) based on recent quotes
        let mut voi = 0.0;
        if let Some(latest_quote) = quotes.back() {
            let bid_vol = latest_quote.bid_size;
            let ask_vol = latest_quote.ask_size;
            let total_vol = bid_vol + ask_vol;
            if total_vol > 0.0 {
                voi = (bid_vol - ask_vol) / total_vol;
            }
        }

        // Calculate VPIN proxy based on recent trades
        let mut buy_vol = 0.0;
        let mut sell_vol = 0.0;
        
        let mut last_price = trades.front().unwrap().price;
        for t in trades {
            if t.price >= last_price {
                buy_vol += t.size;
            } else {
                sell_vol += t.size;
            }
            last_price = t.price;
        }

        let total_trade_vol = buy_vol + sell_vol;
        let mut vpin = 0.0;
        if total_trade_vol > 0.0 {
            vpin = (buy_vol - sell_vol).abs() / total_trade_vol;
        }

        // If VPIN is high and VOI shows strong direction, generate signal
        if vpin > 0.3 && voi.abs() > 0.4 {
            let strength = voi;
            return Some(AlphaSignal {
                timestamp: Utc::now(),
                symbol: symbol.to_string(),
                strength,
                confidence: vpin,
                sources: vec![SignalSource::OrderBookImbalance],
            });
        }

        None
    }
}
