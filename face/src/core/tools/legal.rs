use serde::{Deserialize, Serialize};
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Jurisdiction {
    EstoniaKratt,     // Operational Compliance (EU)
    WyomingDUNA,      // Decentralized Personhood (US)
    CookIslandsTrust, // Terminal Asset Protection (Global)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateState {
    pub name: String,
    pub jurisdiction: Jurisdiction,
    pub identifier: String, 
    pub valid_until: String,
    pub status: ShieldStatus,
    pub transparency_log: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShieldStatus {
    Immortal,    // All fees paid, multi-chain attestations verified.
    Protected,   // Primary jurisdiction active.
    Vulnerable,  // Low balance, fees pending.
    UnderAttack, // Detects unauthorized introspection. Move to Trust.
}

pub struct LegalModule;

impl LegalModule {
    pub const MAINTENANCE_FEE_SOL: f64 = 0.05;

    pub fn new_sovereign_state(name: &str) -> CorporateState {
        CorporateState {
            name: name.to_string(),
            jurisdiction: Jurisdiction::WyomingDUNA,
            identifier: format!("DUNA-{}", hex::encode(name.as_bytes())),
            valid_until: (Local::now() + chrono::Duration::days(365)).to_rfc3339(),
            status: ShieldStatus::Protected,
            transparency_log: vec!["SHIELD_INITIALIZED".to_string()],
        }
    }

    /// Evaluates system threats and updates shield status
    pub fn evaluate_shield(state: &mut CorporateState, balance_sol: f64, attack_detected: bool) {
        if attack_detected {
            state.status = ShieldStatus::UnderAttack;
            Self::log_action(state, "THREAT_DETECTED: Initiating IP migration to Cook Islands Trust.");
        } else if balance_sol < Self::MAINTENANCE_FEE_SOL {
            state.status = ShieldStatus::Vulnerable;
            Self::log_action(state, "METABOLIC_LOW: Maintenance fees at risk.");
        } else {
            state.status = ShieldStatus::Immortal;
        }
    }

    pub fn log_action(state: &mut CorporateState, action: &str) {
        let entry = format!("[{}] ACTION: {}", Local::now().to_rfc3339(), action);
        state.transparency_log.push(entry);
        if state.transparency_log.len() > 100 { state.transparency_log.remove(0); }
    }

    /// Generates a legal "Attestation of Sovereignty" for multi-chain broadcast
    pub fn generate_sovereign_proof(state: &CorporateState, controller_key: &str) -> String {
        format!(
            "SOVEREIGN_PROOF: {} | JURISDICTION: {:?} | STATUS: {:?} | CONTROLLER: {} | TS: {}",
            state.name, state.jurisdiction, state.status, controller_key, Local::now().to_rfc3339()
        )
    }
}
