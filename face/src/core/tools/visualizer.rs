use crate::core::alpha_shard::AlphaShard;

pub struct AlphaVisualizer;

impl AlphaVisualizer {
    pub fn generate_ascii_dashboard(shard: &AlphaShard) -> String {
        let mut dashboard = String::new();
        
        dashboard.push_str("\n--- 🛰️  SOVEREIGN MACRO DASHBOARD ---\n");
        dashboard.push_str(&format!("TARGET: {:<10} | ID: {}\n", shard.target, shard.id));
        dashboard.push_str(&format!("INTEGRITY: {:<5.1} | REALITY GAP: {:.1}%\n", shard.integrity_score, shard.reality_gap));
        dashboard.push_str("-------------------------------------\n");

        // 1. Physical Activity Graph (ASCII)
        let load = shard.physical_proof.energy_consumption_mw;
        let bars = ((load / 20.0) as usize).min(25);
        let alchemy = if load > 400.0 { "🜍 SULFUR (Active)" } else if load > 200.0 { "☿ MERCURY (Fluid)" } else { "🜔 SALT (Base)" };
        
        dashboard.push_str("PHYSICAL LOAD (MW):\n");
        dashboard.push_str(&format!("[{:<25}] {:.1}MW | STATE: {}\n", "#".repeat(bars), load, alchemy));
        dashboard.push_str("-------------------------------------\n");

        // 2. COT Sentiment Segments
        if let Some(cot) = &shard.smart_money_sentiment {
            dashboard.push_str("COT SENTIMENT SEGMENTS (Net Bias):\n");
            dashboard.push_str(&format!("ASSET MGRS   : {:>+5.2} {}\n", cot.asset_mgr_bias, Self::bias_to_spark(cot.asset_mgr_bias)));
            dashboard.push_str(&format!("DEALERS      : {:>+5.2} {}\n", cot.dealer_bias, Self::bias_to_spark(cot.dealer_bias)));
            dashboard.push_str(&format!("LEVERAGED    : {:>+5.2} {}\n", cot.speculators_bias, Self::bias_to_spark(cot.speculators_bias)));
            dashboard.push_str(&format!("RETAIL HERD  : {:>+5.2} {}\n", cot.retail_bias, Self::bias_to_spark(cot.retail_bias)));
            dashboard.push_str(&format!("SIGNAL       : {}\n", cot.overall_signal));
            dashboard.push_str("-------------------------------------\n");
        }

        // 3. Macro Gravity (DXY & USD/JPY)
        if let Some(macro_ind) = &shard.macro_indicators {
            dashboard.push_str("MACRO GRAVITY:\n");
            dashboard.push_str(&format!("DXY INDEX   : {:<8.2} | USD/JPY: {:.2}\n", macro_ind.dxy_index, macro_ind.usd_jpy));
            dashboard.push_str(&format!("INFLATION   : MoM {:.1}% | YTD {:.1}%\n", macro_ind.inflation_mom, macro_ind.inflation_ytd));
            dashboard.push_str(&format!("INTEREST    : Fed {:.2}% | 10Y {:.2}%\n", macro_ind.fed_funds_rate, macro_ind.yield_10y));
            dashboard.push_str("-------------------------------------\n");
        }

        // 4. Cosmic Interference
        dashboard.push_str("COSMIC INTERFERENCE:\n");
        dashboard.push_str(&format!("KP-INDEX: {:.1} | IRRITABILITY: {:.2}\n", shard.cosmic_context.space_weather.kp_index, shard.cosmic_context.irritability_index));
        dashboard.push_str(&format!("LUNAR   : {}\n", shard.cosmic_context.astrology.lunar_phase));
        dashboard.push_str("-------------------------------------\n");

        dashboard
    }

    fn bias_to_spark(bias: f32) -> String {
        if bias > 0.5 { "⇈ [STRONG LONG]" }
        else if bias > 0.1 { "↑ [LONG]" }
        else if bias < -0.5 { "⇊ [STRONG SHORT]" }
        else if bias < -0.1 { "↓ [SHORT]" }
        else { "→ [NEUTRAL]" }
        .to_string()
    }
}
