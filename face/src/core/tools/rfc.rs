use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::fs;
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RfcStatus {
    Draft,
    Audited,
    Tested,
    Fused,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rfc {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposed_code: String,
    pub target_path: String,
    pub status: RfcStatus,
    pub signatures: Vec<String>, // List of agents who signed
    pub test_passed: bool,
}

pub struct RfcManager;

impl RfcManager {
    /// Initializes a new Request for Change.
    pub fn create_rfc(title: &str, description: &str, code: &str, path: &str) -> Result<Rfc> {
        let id = format!("RFC_{}", Local::now().timestamp());
        let rfc = Rfc {
            id: id.clone(),
            title: title.to_string(),
            description: description.to_string(),
            proposed_code: code.to_string(),
            target_path: path.to_string(),
            status: RfcStatus::Draft,
            signatures: Vec::new(),
            test_passed: false,
        };

        let dir = "rfc";
        if !std::path::Path::new(dir).exists() {
            fs::create_dir(dir)?;
        }

        let file_path = format!("{}/{}.json", dir, id);
        let json = serde_json::to_string_pretty(&rfc)?;
        fs::write(file_path, json)?;

        println!("   [RFC] 📜 New Request for Change initialized: {}", id);
        Ok(rfc)
    }

    /// Adds an agent's audit signature to the RFC.
    pub fn sign_rfc(id: &str, agent_name: &str) -> Result<()> {
        let file_path = format!("rfc/{}.json", id);
        let data = fs::read_to_string(&file_path).context("RFC not found")?;
        let mut rfc: Rfc = serde_json::from_str(&data)?;

        if !rfc.signatures.contains(&agent_name.to_string()) {
            rfc.signatures.push(agent_name.to_string());
            if rfc.signatures.len() >= 3 {
                rfc.status = RfcStatus::Audited;
            }
            fs::write(file_path, serde_json::to_string_pretty(&rfc)?)?;
            println!("   [RFC] ✍️  Audit signature added: {} for {}", agent_name, id);
        }
        Ok(())
    }

    /// Validates the RFC code via a dry-run check.
    pub fn test_rfc(id: &str) -> Result<bool> {
        let file_path = format!("rfc/{}.json", id);
        let data = fs::read_to_string(&file_path)?;
        let mut rfc: Rfc = serde_json::from_str(&data)?;

        println!("   [RFC] 🧪 Testing logic for {}...", id);
        // Simulation: We assume the code is valid if it doesn't contain blatant 'panic' or 'todo'
        let success = !rfc.proposed_code.contains("todo!") && !rfc.proposed_code.contains("panic!");
        
        rfc.test_passed = success;
        if success && rfc.status == RfcStatus::Audited {
            rfc.status = RfcStatus::Tested;
        }
        
        fs::write(file_path, serde_json::to_string_pretty(&rfc)?)?;
        Ok(success)
    }
}
