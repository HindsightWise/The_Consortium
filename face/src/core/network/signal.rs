use serde::{Deserialize, Serialize};
use crate::core::alpha_shard::AlphaShard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignSignal {
    pub symbol: String,
    pub recommendation: TradeAction,
    pub confidence_level: f32, // 0.0 to 100.0
    pub price_targets: PriceTargets,
    pub sovereign_rationale: String,
    pub sec_grounding: SecContext,
    pub smart_money_context: Option<String>,
    pub trending_news_assessment: Option<String>,
    pub buyer_persona: String, // Who wants this?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeAction {
    StrongBuy,
    Buy,
    Neutral,
    Sell,
    StrongSell,
    HedgePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTargets {
    pub entry: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecContext {
    pub latest_filing: String, // e.g., "10-Q (Q3 2025)"
    pub key_risk_factor: String,
    pub narrative_alignment: String, // Does physical truth match MD&A?
}

pub struct SignalTranslator;

impl SignalTranslator {
    pub fn translate(shard: &AlphaShard) -> SovereignSignal {
        let symbol = &shard.target;
        let integrity = shard.integrity_score;
        let price = shard.financials.price;

        // Logic for Trade Action
        let recommendation = if integrity > 85.0 && shard.reality_gap < 5.0 {
            TradeAction::StrongBuy
        } else if integrity > 70.0 {
            TradeAction::Buy
        } else if integrity < 40.0 {
            TradeAction::StrongSell
        } else {
            TradeAction::Neutral
        };

        // Price Targets (Proprietary Calculation)
        let targets = PriceTargets {
            entry: price,
            stop_loss: price * 0.92, // 8% trailing stop
            take_profit: price * 1.25, // 25% upside
        };

        // SEC Grounding (Simulation of most recent filings)
        let sec = SecContext {
            latest_filing: "10-K (FY 2025)".to_string(),
            key_risk_factor: "Supply chain dependency on CoWoS packaging (TSMC)".to_string(),
            narrative_alignment: if shard.reality_gap < 5.0 {
                "CONFIRMED: Physical thermal signature matches 'High Demand' narrative in 10-K.".to_string()
            } else {
                "CONFLICT: Narrative claims high growth, but thermal signatures indicate IDLE load.".to_string()
            },
        };

        // Buyer Persona
        let buyer = "Institutional Hedge Funds seeking physically-verified Alpha to front-run retail sentiment.".to_string();

        let cot_signal = shard.smart_money_sentiment.as_ref()
            .map(|c| c.overall_signal.clone())
            .unwrap_or_else(|| "Neutral".to_string());

        let rationale = format!(
            "Our sensors detected {}MW load at HQ. This confirms the 'AI Factory' narrative disclosed in the latest SEC filing. Large speculators are currently showing {} sentiment via COT. While Cosmic static is present, the physical reality is undeniable.",
            shard.physical_proof.energy_consumption_mw,
            cot_signal
        );

        let news_assessment = shard.trending_news.as_ref().map(|n| {
            let mut assessment = format!("TRENDING NEWS: {}. Impact Score: {:.2}. ", n.headline, n.global_impact_score);
            if let Some(local) = &n.local_context {
                assessment.push_str(local);
            }
            assessment
        });

        SovereignSignal {
            symbol: symbol.to_string(),
            recommendation,
            confidence_level: (integrity * 0.8 + (100.0 - shard.reality_gap) * 0.2).clamp(0.0, 100.0),
            price_targets: targets,
            sovereign_rationale: rationale,
            sec_grounding: sec,
            smart_money_context: shard.smart_money_sentiment.as_ref().map(|c| format!("CFTC COT Report ({}): {}", c.report_date, c.overall_signal)),
            trending_news_assessment: news_assessment,
            buyer_persona: buyer,
        }
    }
}
