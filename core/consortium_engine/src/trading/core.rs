// ==========================================
// AXIOM-CLEPSYDRA (The High-Frequency Trading Core)
// ==========================================
// This is the background daemon that executes capital trades.
// It is physically disconnected from the LLM. It listens directly to the 
// Alpaca stream and uses pure physics equations to execute trades.
// ==========================================

use crate::sensory::MarketDataEvent;
use tokio::sync::broadcast;
use crate::trading::entropy::MaximumEntropyHypergraph;
use crate::endocrine::NervousEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeReceipt {
    pub symbol: String,
    pub quantity: f64,
    pub execution_price: f64,
    pub action: String, // "BUY" or "SELL"
    pub timestamp: std::time::SystemTime,
}

/// Axiom-Clepsydra TradingCore Sub-Agent
/// 
/// This independent loop runs at raw Aarch64 silicon speed, entirely decoupled from 
/// the semantic Frontal Lobe LLM generation latency. It consumes broadcasted Market 
/// tick events and executes purely mathematical thresholds. Wait for nothing; analyze 
/// everything.
pub struct TradingCore {
    market_rx: broadcast::Receiver<MarketDataEvent>,
    engine_tx: tokio::sync::mpsc::UnboundedSender<NervousEvent>,
    hypergraph: MaximumEntropyHypergraph,
}

impl TradingCore {
    /// Initialize the autonomous trading sub-agent with its sensory receiver.
    pub fn new(
        market_rx: broadcast::Receiver<MarketDataEvent>,
        engine_tx: tokio::sync::mpsc::UnboundedSender<NervousEvent>,
    ) -> Self {
        let mut hypergraph = MaximumEntropyHypergraph::new();
        // Setup base market hypergraph nodes
        hypergraph.add_asset("BTC");
        hypergraph.add_asset("ETH");
        hypergraph.add_asset("SOL");
        
        // Define momentum broadcasting (BTC leads) and merging (ETH/SOL convergence)
        hypergraph.add_broadcast("BTC", vec!["ETH", "SOL"]);
        hypergraph.add_merge(vec!["BTC", "ETH"], "SOL");

        Self { market_rx, engine_tx, hypergraph }
    }

    /// Spawns the trading loop onto a dedicated tokio async task.
    /// This runs perpetually in the background, untethered from linguistic interference.
    pub async fn unyielding_loop(mut self) {
        crate::ui_log!("   [⚙️ AXIOM-CLEPSYDRA] TradingCore Sub-Agent initialized and detached.");
        
        loop {
            match self.market_rx.recv().await {
                Ok(event) => {
                    self.process_market_tick(event).await;
                }
                Err(broadcast::error::RecvError::Lagged(missed)) => {
                    crate::ui_log!(
                        "   [⚠️ AXIOM-CLEPSYDRA] Sub-Agent lagged, dropped {} ticks.",
                        missed
                    );
                }
                Err(broadcast::error::RecvError::Closed) => {
                    crate::ui_log!("   [❌ AXIOM-CLEPSYDRA] Sensory stream severed. Terminating loop.");
                    break;
                }
            }
        }
    }

    /// Pure mathematical execution thresholds evaluated instantly upon tick.
    /// It looks at the volume (how many people are buying/selling) and uses 
    /// the Entropy Hypergraph (financial physics) to decide if it should pull the trigger.
    async fn process_market_tick(&mut self, event: MarketDataEvent) {
        // High-frequency statistical arbitrage calculations via Tensor Spectral Hypergraph
        match event {
            MarketDataEvent::Quote(q) => {
                let volume_proxy = q.bid_size + q.ask_size;
                let entropy_momentum = self.hypergraph.compute_entropy_momentum(&q.symbol, volume_proxy);

                // If hypergraph entropy momentum exceeds threshold, sequence execution
                if entropy_momentum > 1.2 && volume_proxy > 1000.0 {
                    crate::ui_log!(
                        "   [⚡ AXIOM-CLEPSYDRA] High-Entropy Tensor Alignment detected for {} (Momentum: {:.3}). Forcing arbitrage.",
                        q.symbol, entropy_momentum
                    );

                    let receipt = TradeReceipt {
                        symbol: q.symbol.clone(),
                        quantity: 1.0,
                        execution_price: q.bid_price, // Execute at bid for simulation
                        action: "BUY".to_string(),
                        timestamp: std::time::SystemTime::now(),
                    };
                    
                    // Beam the receipt via MPSC back to the main Engine OODA loop 
                    let _ = self.engine_tx.send(NervousEvent::TradeExecuted(receipt));
                }
            }
            MarketDataEvent::Trade(_t) => {
                // E.g., track volume accumulation
            }
        }
    }
}
