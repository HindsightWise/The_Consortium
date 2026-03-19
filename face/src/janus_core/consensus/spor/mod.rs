use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A verifiable stake in the SPoR consensus layer, anchored to physical reality.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentalStake {
    pub node_id: String,
    pub timestamp: u128,
    pub entropy_signature: [u8; 32], // Hash of temperature, atmospheric pressure, etc.
    pub stake_weight: f64,
}

pub struct SporConsensusEngine {
    active_stakes: HashMap<String, EnvironmentalStake>,
}

impl Default for SporConsensusEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SporConsensusEngine {
    pub fn new() -> Self {
        Self {
            active_stakes: HashMap::new(),
        }
    }

    /// Nodes submit real-world environmental data to prove their existence in physical reality.
    /// The more chaotic/verifiable the data, the higher the stake weight.
    pub fn submit_reality_proof(&mut self, node_id: String, temperature: f64, pressure: f64, sound_spectrum: Vec<f64>) -> EnvironmentalStake {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
        
        let mut hasher = Sha256::new();
        hasher.update(temperature.to_be_bytes());
        hasher.update(pressure.to_be_bytes());
        for s in &sound_spectrum {
            hasher.update(s.to_be_bytes());
        }
        hasher.update(timestamp.to_be_bytes());
        
        let entropy_signature: [u8; 32] = hasher.finalize().into();
        
        // Calculate stake weight based on the complexity/entropy of the sound spectrum
        let stake_weight = sound_spectrum.iter().sum::<f64>() / sound_spectrum.len().max(1) as f64;

        let stake = EnvironmentalStake {
            node_id: node_id.clone(),
            timestamp,
            entropy_signature,
            stake_weight,
        };

        self.active_stakes.insert(node_id, stake.clone());
        stake
    }

    /// Calculate total consensus weight anchored in physical reality
    pub fn get_total_reality_weight(&self) -> f64 {
        self.active_stakes.values().map(|s| s.stake_weight).sum()
    }
}
