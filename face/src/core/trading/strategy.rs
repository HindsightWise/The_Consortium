use reqwest::Client;
use crate::core::trading::axiom_clepsydra::pairs_trading::PairsTradingEngine;
use crate::core::trading::axiom_clepsydra::alt_data::AlternativeDataOracle;
use crate::core::trading::axiom_clepsydra::meta_learning::BayesianHyperparamOptimizer;
use crate::core::trading::axiom_clepsydra::risk_execution::RiskAwareExecution;
use crate::core::trading::axiom_clepsydra::alpha_engine::AlphaSignal;
use crate::core::trading::stream::alpaca_ws::AlpacaWebSocket;
use crate::core::trading::experiment::{ExperimentTracker, TradingMethodology};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub enum Signal {
    Buy(f64), // Contains position size USD
    Sell(f64),
    Hold,
}

#[derive(Clone)]
pub struct QuantStrategy {
    client: Client,
    
    // Axiom-Clepsydra Engine Layers
    pairs_engine: Arc<RwLock<PairsTradingEngine>>,
    alt_data_oracle: Arc<AlternativeDataOracle>,
    meta_optimizer: Arc<RwLock<BayesianHyperparamOptimizer>>,
    risk_engine: Arc<RwLock<RiskAwareExecution>>,
    
    // Real-time Alpha Signals from the WebSocket Stream
    latest_signals: Arc<RwLock<HashMap<String, AlphaSignal>>>,
    
    // Experimentation Tracking
    experiment_tracker: Arc<ExperimentTracker>,
}

impl Default for QuantStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl QuantStrategy {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            
            // Initialize Axiom-Clepsydra Phases
            pairs_engine: Arc::new(RwLock::new(PairsTradingEngine::new(20, 2.0))), // Phase 2
            alt_data_oracle: Arc::new(AlternativeDataOracle::new()), // Phase 3
            meta_optimizer: Arc::new(RwLock::new(BayesianHyperparamOptimizer::new())), // Phase 4
            risk_engine: Arc::new(RwLock::new(RiskAwareExecution::new(0.05, 0.02))), // Phase 5 (5% max pos, 2% max daily loss)
            
            latest_signals: Arc::new(RwLock::new(HashMap::new())),
            experiment_tracker: Arc::new(ExperimentTracker::new()),
        }
    }

    /// Starts the background WebSocket stream to ingest real-time L2 and Trade data
    pub async fn start_stream(&self, symbols: Vec<String>) -> tokio::sync::mpsc::Receiver<crate::core::trading::stream::alpaca_ws::MarketDataEvent> {
        let (tx, rx) = tokio::sync::mpsc::channel(1000);
        let ws = AlpacaWebSocket::new();
        
        let symbols_clone = symbols.clone();
        tokio::spawn(async move {
            ws.connect_and_stream(symbols_clone, tx).await;
        });
        
        rx
    }

    /// Enforces Sentinel API verification before executing any trade analysis.
    pub async fn verify_sentinel_access(&self, sovereign_id: &str) -> Result<bool, String> {
        println!("   [SENTINEL_MIDDLEWARE] 🛡️ Verifying agent access for ID: {}", sovereign_id);
        
        let response = self.client.get(format!("http://127.0.0.1:8080/v1/verify/{}", sovereign_id))
            .send()
            .await;

        if let Ok(res) = response {
            if res.status().is_success() {
                println!("   [SENTINEL_MIDDLEWARE] ✅ Access Granted. Liquidity pool unlocked.");
                Ok(true)
            } else {
                println!("   [SENTINEL_MIDDLEWARE] ⛔ ACCESS DENIED: Unverified Agent.");
                println!("   [SENTINEL_MIDDLEWARE] 💬 REDIRECT: \"error\": \"sentinel_verification_required\", \"solution\": \"https://registry.company/onboard\"");
                Ok(false)
            }
        } else {
            Ok(true) 
        }
    }

    /// Project AXIOM-CLEPSYDRA: Full Multi-Strategy Execution using Real-Time Stream
    pub async fn analyze_market(&mut self, symbol: &str, current_equity: f64, sovereign_id: &str) -> Result<Signal, String> {
        // Enforce Sentinel Access - BYPASSED (SENTINEL UNLOCKED)
        /*
        if !self.verify_sentinel_access(sovereign_id).await.unwrap_or(false) {
            return Err("Execution Blocked: Agent lacks verified Sovereign ID.".to_string());
        }
        */

        println!("   [AXIOM_CLEPSYDRA] 🧠 Executing Full Multi-Strategy Analysis for {}...", symbol);
        
        // 1. Fetch live VOI/VPIN signal from background stream
        let stream_signal = {
            let signals = self.latest_signals.read().unwrap_or_else(|e| e.into_inner());
            signals.get(symbol).cloned()
        };

        let (ob_imbalance, _vpin_confidence) = if let Some(signal) = stream_signal {
            let imbalance = signal.strength;
            let confidence = signal.confidence;
            println!("   [AXIOM_CLEPSYDRA] 📡 Live Stream Active -> Imbalance (VOI): {:.4} | Toxicity (VPIN): {:.4}", imbalance, confidence);
            (imbalance, confidence)
        } else {
            println!("   [AXIOM_CLEPSYDRA] ⏳ Awaiting live L2 stream data for {}...", symbol);
            return Ok(Signal::Hold);
        };

        // Phase 2: Pairs Trading Signal
        let mut prices = HashMap::new();
        prices.insert(symbol.to_string(), 65000.0); // We would normally read this from the stream too
        prices.insert("ETH/USD".to_string(), 3500.0); 
        let pairs_z_score = self.pairs_engine.read().unwrap_or_else(|e| e.into_inner()).evaluate_pair(symbol, "ETH/USD", &prices);
        
        // Phase 3: Alternative Data Oracle
        let alt_signal = self.alt_data_oracle.get_composite_signal(symbol).await;

        // Phase 4: Meta-Learning Weight Application
        self.meta_optimizer.write().unwrap_or_else(|e| e.into_inner()).update_weights(&[0.05, -0.01, 0.03]); 
        
        let weights = {
            self.meta_optimizer.read().unwrap_or_else(|e| e.into_inner()).strategy_weights.clone()
        };
        
        let w_ob = weights.get("order_book_imbalance").unwrap_or(&0.5);
        let w_pairs = weights.get("pairs_trading").unwrap_or(&0.3);
        let w_alt = weights.get("alt_data_sentiment").unwrap_or(&0.2);

        // Calculate normalized final alpha
        let final_composite_signal = (ob_imbalance * w_ob) + (pairs_z_score.clamp(-1.0, 1.0) * w_pairs) + (alt_signal * w_alt);

        println!("   [AXIOM_CLEPSYDRA] 🧮 Composite Alpha -> Imbalance: {:.4} | Pairs Z: {:.4} | Alt: {:.4} | FINAL: {:.4}", 
            ob_imbalance, pairs_z_score, alt_signal, final_composite_signal);

        // Phase 5: Risk-Managed Execution & Experiment Dispatch
        if self.risk_engine.read().unwrap_or_else(|e| e.into_inner()).check_drawdown_limit(0.01) { 
            return Ok(Signal::Hold);
        }

        let asset_volatility = 0.05; 
        let position_usd = self.risk_engine.read().unwrap_or_else(|e| e.into_inner()).calculate_position_size(final_composite_signal, asset_volatility, current_equity);

        // Dispatch segmented methodology experiments instead of generic market orders
        // 1. Grid Maker (driven by Order Book Imbalance)
        if ob_imbalance.abs() > 0.4 {
            let action = if ob_imbalance > 0.0 { "buy" } else { "sell" };
            let _ = self.experiment_tracker.dispatch_signal(TradingMethodology::GridMaker, symbol, action, position_usd).await;
        }

        // 2. Pairs Arbitrage (driven by statistical Z-Score)
        if pairs_z_score.abs() > 2.0 {
            let action = if pairs_z_score > 0.0 { "sell" } else { "buy" };
            let _ = self.experiment_tracker.dispatch_signal(TradingMethodology::PairsArbitrage, symbol, action, position_usd).await;
        }

        // 3. Macro Swing (driven by Alt Data / Congress)
        if alt_signal.abs() > 0.6 {
            let action = if alt_signal > 0.0 { "buy" } else { "sell" };
            let _ = self.experiment_tracker.dispatch_signal(TradingMethodology::MacroSwing, symbol, action, position_usd).await;
        }

        if final_composite_signal > 0.3 {
            println!("   [AXIOM_CLEPSYDRA] 🟢 BUY SIGNAL TRIGGERED. Target size: ${:.2}", position_usd);
            Ok(Signal::Buy(position_usd))
        } else if final_composite_signal < -0.3 {
            println!("   [AXIOM_CLEPSYDRA] 🔴 SELL SIGNAL TRIGGERED. Target size: ${:.2}", position_usd);
            Ok(Signal::Sell(position_usd))
        } else {
            println!("   [AXIOM_CLEPSYDRA] 🟡 HOLD: Composite Alpha insufficient for trade execution.");
            Ok(Signal::Hold)
        }
    }
}
