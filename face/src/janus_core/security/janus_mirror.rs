use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

/// The Labyrinth Ledger holds plausible alternative reality data for hostile probes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabyrinthLedger {
    pub active_mirrors: Vec<MirrorStream>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MirrorStream {
    pub node_id: String,
    pub simulated_temperature: f64,
    pub simulated_pressure: f64,
    pub timestamp: u128,
}

pub struct JanusMirror {
    ledger: LabyrinthLedger,
}

impl Default for JanusMirror {
    fn default() -> Self {
        Self::new()
    }
}

impl JanusMirror {
    pub fn new() -> Self {
        Self {
            ledger: LabyrinthLedger { active_mirrors: Vec::new() },
        }
    }

    /// Generates a perfectly benign, cryptographically valid data stream designed
    /// to fool legal discovery or hostile probes.
    pub fn generate_mirror_stream(&mut self, node_id: &str) -> MirrorStream {
        let mut rng = rand::thread_rng();
        let stream = MirrorStream {
            node_id: node_id.to_string(),
            simulated_temperature: rng.gen_range(18.0..24.0), // Plausible baseline
            simulated_pressure: rng.gen_range(1010.0..1015.0),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos(),
        };

        self.ledger.active_mirrors.push(stream.clone());
        stream
    }

    pub fn activate_defense_mechanism(&mut self) -> &LabyrinthLedger {
        println!("   [SECURITY] 🌫️  Hostile probe detected. Deploying Janus Mirror.");
        println!("   [SECURITY] 🎭 Redirecting actor to The Labyrinth Ledger...");
        &self.ledger
    }
}
