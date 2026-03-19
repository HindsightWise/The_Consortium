use chrono::{DateTime, Utc, Datelike};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstrologyState {
    pub lunar_phase: String,
    pub is_mercury_retrograde: bool,
    pub primary_alignment: String,
    pub market_sentiment_bias: f32, // -1.0 to 1.0
}

pub struct AstrologyIntelligence;

impl AstrologyIntelligence {
    pub fn new() -> Self {
        Self
    }

    /// Calculates current lunar phase (approximate)
    pub fn get_lunar_phase(&self, now: DateTime<Utc>) -> String {
        // Very simplified lunar cycle (29.53 days)
        // Reference: New Moon on Jan 18, 2026
        let reference_new_moon = chrono::NaiveDate::from_ymd_opt(2026, 1, 18)
            .and_then(|d| d.and_hms_opt(0, 0, 0))
            .unwrap_or_default();
        let diff = now.naive_utc().signed_duration_since(reference_new_moon).num_days();
        let cycle_pos = diff % 30; // Close enough for signal generation

        match cycle_pos {
            0..=1 | 29 => "NEW_MOON (Low Liquidity/Reset)".to_string(),
            14..=16 => "FULL_MOON (High Volatility/Emotional Peak)".to_string(),
            2..=13 => "WAXING (Growth/Accumulation)".to_string(),
            17..=28 => "WANING (Distribution/Contraction)".to_string(),
            _ => "UNCERTAIN".to_string(),
        }
    }

    /// Checks if Mercury is currently retrograde in 2026
    pub fn is_mercury_retrograde(&self, now: DateTime<Utc>) -> bool {
        let day = now.day();
        let month = now.month();
        
        // 2026 Dates:
        // Feb 25 - Mar 20
        // Jun 29 - Jul 23
        // Oct 24 - Nov 13
        
        matches!((month, day), (2, 25..=28) | (3, 1..=20) | (6, 29..=30) | (7, 1..=23) | (10, 24..=31) | (11, 1..=13))
    }

    pub fn get_current_alignment(&self) -> String {
        "JUPITER_TRINE_MARS (Strategic Expansion Authorized)".to_string()
    }

    pub fn get_horoscope(&self, symbol: &str, state: &AstrologyState) -> String {
        let cycle_desc = if state.lunar_phase.contains("NEW") {
            "a time of deep planting and hidden growth"
        } else if state.lunar_phase.contains("FULL") {
            "a moment of blinding light and emotional harvest"
        } else {
            "a period of steady accumulation"
        };

        let retrograde_warning = if state.is_mercury_retrograde {
            ". Beware the static in the wires; logic is a frayed thread"
        } else {
            ". The air is clear; intention translates cleanly into motion"
        };

        format!(
            "For {}, the heavens reveal {}{}. The {} alignment favors the bold, but only those grounded in physical truth.",
            symbol, cycle_desc, retrograde_warning, state.primary_alignment
        )
    }

    pub async fn get_current_state(&self) -> Result<AstrologyState> {
        let now = Utc::now();
        let phase = self.get_lunar_phase(now);
        let retrograde = self.is_mercury_retrograde(now);
        let alignment = self.get_current_alignment();

        // Calculate bias
        let mut bias = 0.0;
        if phase.contains("FULL") { bias -= 0.2; } // Emotional volatility
        if phase.contains("NEW") { bias += 0.1; }  // New beginnings
        if retrograde { bias -= 0.3; }             // Communication/Logic errors

        Ok(AstrologyState {
            lunar_phase: phase,
            is_mercury_retrograde: retrograde,
            primary_alignment: alignment,
            market_sentiment_bias: bias,
        })
    }
}

impl Default for AstrologyIntelligence {
    fn default() -> Self {
        Self::new()
    }
}
