use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceWeatherState {
    pub kp_index: f32, // Planetary K-index (0-9)
    pub solar_flux: f32,
    pub flare_status: String,
    pub schumann_resonance_hz: f32, // Fundamental frequency (normal: 7.83)
    pub schumann_amplitude: f32,    // Signal strength (normal: 1.0)
    pub irritability_multiplier: f32,
}

pub struct CosmicIntelligence;

impl CosmicIntelligence {
    pub fn new() -> Self {
        Self
    }

    /// Fetches the current Planetary K-index (Magnetic disturbance level)
    pub async fn get_kp_index(&self) -> Result<f32> {
        // In a live implementation, this hits NOAA SWPC JSON endpoints
        // https://services.swpc.noaa.gov/json/planetary-k-index.json
        println!("   [Cosmic] 🛰️  Querying NOAA for geomagnetic disturbances...");
        
        // Simulation of current Cycle 25 activity
        Ok(4.5) // Moderate activity
    }

    /// Fetches solar flux data (10.7cm radio burst)
    pub async fn get_solar_flux(&self) -> Result<f32> {
        println!("   [Cosmic] 🛰️  Measuring solar radio flux...");
        Ok(165.0) // Typical peak activity level
    }

    /// Measures the Earth-Ionosphere cavity resonance
    pub async fn get_schumann_resonance(&self) -> Result<(f32, f32)> {
        println!("   [Cosmic] 🛰️  Interrogating Earth-Ionosphere cavity for Schumann peaks...");
        // In live mode, this would fetch from a station like GCMS (Global Coherence Monitoring System)
        
        // Simulation: Solar storms (Kp > 4) typically cause frequency drift and amplitude spikes
        let freq = 7.83 + (rand::random::<f32>() - 0.5) * 0.2;
        let amp = 1.0 + (rand::random::<f32>() * 0.5);
        Ok((freq, amp))
    }

    pub async fn get_current_state(&self) -> Result<SpaceWeatherState> {
        let kp = self.get_kp_index().await?;
        let flux = self.get_solar_flux().await?;
        let (sch_freq, sch_amp) = self.get_schumann_resonance().await?;
        
        // Calculate Irritability Multiplier (The Chizhevsky Factor)
        // High Kp + High Flux + Schumann Drift = Maximum Excitability
        let irritability = 1.0 + (kp / 10.0) + (flux / 500.0) + (sch_amp / 10.0);

        Ok(SpaceWeatherState {
            kp_index: kp,
            solar_flux: flux,
            flare_status: if kp > 6.0 { "X-CLASS_POTENTIAL" } else { "STABLE" }.to_string(),
            schumann_resonance_hz: sch_freq,
            schumann_amplitude: sch_amp,
            irritability_multiplier: irritability,
        })
    }
}

impl Default for CosmicIntelligence {
    fn default() -> Self {
        Self::new()
    }
}
