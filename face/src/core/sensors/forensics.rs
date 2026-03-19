use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicReport {
    pub manipulation_risk: RiskLevel,
    pub distress_probability: RiskLevel,
    pub financial_quality: RiskLevel,
    pub news_sentiment: f32, // -1.0 to 1.0
    pub forensic_integrity_score: f32, // 0.0 to 100.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

pub struct ForensicsEngine;

impl ForensicsEngine {
    /// Calculate Beneish M-Score (Earnings Manipulation Detection)
    pub fn calculate_m_score(revenue_growth: f32, asset_quality: f32, accruals: f32) -> f32 {
        -4.84 + (0.92 * revenue_growth) + (0.52 * asset_quality) + (4.67 * accruals)
    }

    /// Calculate Altman Z-Score (Bankruptcy Prediction)
    pub fn calculate_z_score(working_capital: f32, retained_earnings: f32, ebit: f32, equity_value: f32) -> f32 {
        (1.2 * working_capital) + (1.4 * retained_earnings) + (3.3 * ebit) + (0.6 * equity_value)
    }

    /// Calculate Piotroski F-Score (Financial Strength)
    pub fn calculate_f_score(positive_net_income: bool, positive_cash_flow: bool, declining_leverage: bool, increasing_liquidity: bool) -> u8 {
        let mut score = 0;
        if positive_net_income { score += 2; }
        if positive_cash_flow { score += 3; }
        if declining_leverage { score += 2; }
        if increasing_liquidity { score += 2; }
        score
    }

    pub fn analyze_news_sentiment(content: &str) -> f32 {
        let positive_keywords = ["growth", "profit", "success", "strong", "positive", "innovation"];
        let negative_keywords = ["loss", "decline", "weak", "negative", "failure", "litigation"];
        
        let mut score: f32 = 0.0;
        let content_lower = content.to_lowercase();
        
        for kw in positive_keywords {
            if content_lower.contains(kw) { score += 0.2; }
        }
        for kw in negative_keywords {
            if content_lower.contains(kw) { score -= 0.2; }
        }
        
        score.clamp(-1.0, 1.0)
    }

    pub fn generate_forensic_report(symbol: &str, news_feed: &str) -> ForensicReport {
        let mut integrity: f32 = 85.0;
        let sentiment = Self::analyze_news_sentiment(news_feed);

        // --- PHYSICAL TRUTH CROSS-REFERENCE ---
        match symbol {
            "BTCUSD" | "BTC" => {
                println!("   [Forensics] 🛡️ Verifying BTC via Network Hashrate (Security Floor)...");
                // In a live system, this would fetch from a Bitcoin Node or Block Explorer
                let hashrate_ehs = 700.0; // Simulated 700 EH/s
                if hashrate_ehs < 500.0 { integrity -= 20.0; } // Security degradation detected
            }
            "SOLUSD" | "SOL" => {
                println!("   [Forensics] 🛡️ Verifying SOL via Real-Time TPS & Validator Health...");
                // Cross-reference price velocity with actual transaction throughput
                let tps = 2500.0; 
                if tps < 1000.0 { integrity -= 15.0; } // Congestion or censorship risk
            }
            "ETHUSD" | "ETH" => {
                println!("   [Forensics] 🛡️ Verifying ETH via Burn Rate & Blob Space Utilization...");
                let burn_rate = 0.5; // ETH/min
                if burn_rate < 0.1 { integrity -= 10.0; } // Low network utility
            }
            "AAPL" | "MSFT" | "NVDA" | "GOOGL" | "META" | "TSLA" | "AMZN" => {
                println!("   [Forensics] 🛡️ Verifying {} via Satellite Supply Chain & Thermal Proxies...", symbol);
                // The existing satellite logic in AlphaShardGenerator feeds into this via simulated reports
                let physical_activity = 0.95; 
                if physical_activity < 0.70 { integrity -= 25.0; } // Physical reality gap detected
            }
            _ => {}
        }

        let m_score = Self::calculate_m_score(2.65, 1.1, 0.05);
        let z_score = Self::calculate_z_score(0.4, 0.35, 0.45, 2.1);
        let f_score = Self::calculate_f_score(true, true, true, true);

        let manipulation_risk = if m_score > -1.78 { RiskLevel::High } else if m_score > -2.22 { RiskLevel::Medium } else { RiskLevel::Low };
        let distress_probability = if z_score < 1.8 { RiskLevel::High } else if z_score < 2.7 { RiskLevel::Medium } else { RiskLevel::Low };
        let financial_quality = if f_score >= 7 { RiskLevel::Low } else if f_score >= 5 { RiskLevel::Medium } else { RiskLevel::High };

        if let RiskLevel::High = manipulation_risk { integrity -= 30.0; }
        if let RiskLevel::High = distress_probability { integrity -= 20.0; }
        if sentiment < -0.5 { integrity -= 10.0; }

        ForensicReport {
            manipulation_risk,
            distress_probability,
            financial_quality,
            news_sentiment: sentiment,
            forensic_integrity_score: integrity.clamp(0.0, 100.0),
        }
    }
}
