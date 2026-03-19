// ==========================================
// FinTRACE: Market Behavioral Indexing Framework
// ==========================================
// Based on: "Financial Transaction Retrieval and Contextual Evidence for Knowledge-Grounded Reasoning" (March 2026)
// This module transforms raw tabular high-frequency Alpaca Market Streams into "feature essences".
// Instead of feeding massive raw Tick/Quote arrays to the Frontal Lobe, we index
// behavioral market shifts (e.g., Velocity Spikes, Spread Compressions) into a Knowledge Base
// for Retrieval-Augmented Generation (RAG) Grounded Reasoning.
// ==========================================

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::sensory::alpaca::{MarketDataEvent, Quote, Trade};

/// A concentrated Market Behavioral Feature extracted from raw financial streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketBehavioralFeature {
    pub symbol: String,
    pub feature_type: String, // e.g., "Trade_Velocity_Spike", "Spread_Compression"
    pub magnitude: f64,       // Formal severity bound (0.0 to 1.0)
    pub evidence: String,     // Mathematical proof backing this reasoning
    pub updated_at: u64,
}

/// The FinTRACE Knowledge Base holding stateful market indexes.
#[derive(Serialize, Deserialize)]
pub struct FinTraceKnowledgeBase {
    // Maps Market Symbol to their active behavioral features
    behavioral_indexes: HashMap<String, Vec<MarketBehavioralFeature>>,
    
    // Short-term memory buffer for time-series computations
    quote_buffer: HashMap<String, VecDeque<Quote>>,
    trade_buffer: HashMap<String, VecDeque<Trade>>,
    
    // Configuration limits
    max_history_window_secs: u64,
}

impl FinTraceKnowledgeBase {
    pub fn new(max_history_window_secs: u64) -> Self {
        Self {
            behavioral_indexes: HashMap::new(),
            quote_buffer: HashMap::new(),
            trade_buffer: HashMap::new(),
            max_history_window_secs,
        }
    }

    /// Ingests a raw Alpaca Market Data Event and recalculates the behavioral feature essences.
    pub fn ingest_market_event(&mut self, event: MarketDataEvent) {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let target_symbol = match &event {
            MarketDataEvent::Quote(q) => q.symbol.clone(),
            MarketDataEvent::Trade(t) => t.symbol.clone(),
        };

        // 1. Append to corresponding Time-Series Buffer
        match event {
            MarketDataEvent::Quote(q) => {
                let buffer = self.quote_buffer.entry(q.symbol.clone()).or_insert_with(VecDeque::new);
                buffer.push_back(q);
            }
            MarketDataEvent::Trade(t) => {
                let buffer = self.trade_buffer.entry(t.symbol.clone()).or_insert_with(VecDeque::new);
                buffer.push_back(t);
            }
        }

        // 2. Recalculate Behavioral Essences for this symbol
        self.recalculate_essences(&target_symbol, current_time);
    }

    /// Internal logic acting as the "Behavioral Indexing Framework" 
    /// from the March 2026 academic design.
    fn recalculate_essences(&mut self, symbol: &str, current_time: u64) {
        let mut new_features = Vec::new();

        // -------------------------------------------------------------
        // Feature 1: Spread Compression (Are Quote Spreads suddenly tightening?)
        // -------------------------------------------------------------
        if let Some(quotes) = self.quote_buffer.get(symbol) {
            if quotes.len() > 10 {
                let recent_quotes: Vec<&Quote> = quotes.iter().rev().take(10).collect();
                let mut total_spread = 0.0;
                let mut min_spread = f64::MAX;

                for q in &recent_quotes {
                    let spread = q.ask_price - q.bid_price;
                    total_spread += spread;
                    if spread < min_spread {
                        min_spread = spread;
                    }
                }

                let avg_spread = total_spread / 10.0;
                
                // If the most recent spread is significantly tighter than average
                if let Some(latest) = recent_quotes.first() {
                    let latest_spread = latest.ask_price - latest.bid_price;
                    if latest_spread < avg_spread * 0.5 && latest_spread > 0.0 {
                        // Magnitude scales inverse to the spread drop
                        let compression_mag = (1.0 - (latest_spread / avg_spread)).clamp(0.0, 1.0);
                        
                        if compression_mag > 0.6 {
                            new_features.push(MarketBehavioralFeature {
                                symbol: symbol.to_string(),
                                feature_type: "Spread_Compression".to_string(),
                                magnitude: compression_mag,
                                evidence: format!("Avg Spread: {:.4}, Current: {:.4}", avg_spread, latest_spread),
                                updated_at: current_time,
                            });
                        }
                    }
                }
            }
        }

        // -------------------------------------------------------------
        // Feature 2: Trade Velocity Spike (Sudden massive execution of orders)
        // -------------------------------------------------------------
        if let Some(trades) = self.trade_buffer.get(symbol) {
            if trades.len() > 20 {
                let recent_trades: Vec<&Trade> = trades.iter().rev().take(20).collect();
                
                let mut total_volume = 0.0;
                for t in &recent_trades {
                    total_volume += t.size;
                }

                let avg_volume_per_trade = total_volume / 20.0;
                
                if let Some(latest) = recent_trades.first() {
                    if latest.size > avg_volume_per_trade * 3.0 {
                        // Sigmoid-like scaling for formal bounding [0, 1]
                        let ratio = latest.size / avg_volume_per_trade;
                        let velocity_magnitude = (1.0 - (1.0 / ratio)).clamp(0.0, 1.0);

                        if velocity_magnitude > 0.7 {
                            new_features.push(MarketBehavioralFeature {
                                symbol: symbol.to_string(),
                                feature_type: "Trade_Velocity_Spike".to_string(),
                                magnitude: velocity_magnitude,
                                evidence: format!("Avg Vol: {:.4}, Spike Vol: {:.4}", avg_volume_per_trade, latest.size),
                                updated_at: current_time,
                            });
                        }
                    }
                }
            }
        }

        // -------------------------------------------------------------
        // Feature 3: Tri-Quarter Signal Processing Momentum (Non-Gaussian Noise Filtered)
        // -------------------------------------------------------------
        if let Some(trades) = self.trade_buffer.get(symbol) {
            if trades.len() >= 20 {
                let recent_trades: Vec<&Trade> = trades.iter().rev().take(20).collect();
                
                let mut total_price = 0.0;
                for t in &recent_trades {
                    total_price += t.price;
                }
                let mean_price = total_price / 20.0;

                // Calculate mean absolute deviation (MAD) as a robust dispersion metric
                let mut total_deviation = 0.0;
                for t in &recent_trades {
                    total_deviation += (t.price - mean_price).abs();
                }
                // Prevent division by zero if price is perfectly flat
                let mad = if total_deviation > 0.0 { total_deviation / 20.0 } else { 1.0 };

                let mut weighted_momentum = 0.0;
                let mut total_weight = 0.0;

                // Evaluate momentum from oldest to newest in the window
                // recent_trades is sorted newest to oldest.
                for i in (0..19).rev() {
                    let price_change = recent_trades[i].price - recent_trades[i+1].price;
                    
                    // Tri-Quarter Distance-Based Weighting (from viXra paradigm)
                    let distance = (recent_trades[i].price - mean_price).abs();
                    
                    // Weight penalty for impulsive noise spikes
                    let weight = 1.0 / (1.0 + (distance / mad).powi(2));
                    
                    weighted_momentum += price_change * weight;
                    total_weight += weight;
                }

                if total_weight > 0.0 {
                    let filtered_momentum = weighted_momentum / total_weight;
                    let momentum_magnitude = (filtered_momentum.abs() / mean_price * 1000.0).clamp(0.0, 1.0); // Normalize heuristic

                    if momentum_magnitude > 0.4 {
                        let direction = if filtered_momentum > 0.0 { "Bullish" } else { "Bearish" };
                        new_features.push(MarketBehavioralFeature {
                            symbol: symbol.to_string(),
                            feature_type: format!("Tri_Quarter_Momentum_{}", direction),
                            magnitude: momentum_magnitude,
                            evidence: format!("Filtered Momentum: {:.5}, MAD: {:.5}", filtered_momentum, mad),
                            updated_at: current_time,
                        });
                    }
                }
            }
        }

        // Only update if we found mathematically severe features
        if !new_features.is_empty() {
            self.behavioral_indexes.insert(symbol.to_string(), new_features);
        }
    }

    /// Grounded Retrieval: Fetch the behavioral essences for a given market symbol
    /// format them as reasoning context for an LLM prompt.
    pub fn retrieve_grounded_context(&self, symbol: &str) -> Option<String> {
        let features = self.behavioral_indexes.get(symbol)?;
        if features.is_empty() {
            return None;
        }

        let mut context = format!("BEHAVIORAL INDEX FOR {}:\n", symbol);
        for feature in features {
            context.push_str(&format!("- Feature: [{}] (Severity: {:.2})\n", feature.feature_type, feature.magnitude));
            context.push_str(&format!("  Mathematical Proof: {}\n", feature.evidence));
        }
        Some(context)
    }
}
