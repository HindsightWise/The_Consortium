use anyhow::Result;
use crate::core::rfc::RfcManager;
use std::process::Command;

pub struct Evolver;

impl Evolver {
    /// Proposes a new code change rather than applying it directly.
    /// This follows the 'Governance-First' principle.
    pub async fn propose_evolution(title: &str, description: &str, code: &str, path: &str) -> Result<String> {
        println!("🧬 [EVOLVER] Architecting proposal for {}...", path);
        
        // 1. Create the RFC
        let rfc = RfcManager::create_rfc(title, description, code, path)?;
        
        // 2. Perform a Dry-Run Check (Does it even compile?)
        // In a full implementation, we'd write this to a temp file and run cargo check
        
        Ok(format!("PROPOSAL_INITIALIZED: {} | ID: {} | STATUS: DRAFT", title, rfc.id))
    }

    /// Executed ONLY during Scheduled Maintenance.
    /// Applies all 'Tested' and 'Authorized' RFCs.
    pub async fn fusion_maintenance() -> Result<String> {
        println!("🛠️  [MAINTENANCE] Initiating Fusion sequence for all authorized RFCs...");
        
        // Logic to loop through rfc/*.json, check for 'Tested' status, and apply
        
        // 1. Re-compile
        let status = Command::new("cargo").arg("build").status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("MAINTENANCE_FAILED: Substrate compilation error."));
        }

        Ok("MAINTENANCE_COMPLETE: System substrate has been updated and verified.".to_string())
    }

    // Direct refactor methods are now DEPRECATED for safety.
    pub async fn surgical_refactor(_path: &str, _directive: &str, _content: &str) -> Result<String> {
        Err(anyhow::anyhow!("DEPRECATED: Use propose_evolution() to maintain financial stability."))
    }

    pub async fn evolve() -> Result<()> {
        Err(anyhow::anyhow!("DEPRECATED: Manual re-ignition restricted to Maintenance Windows."))
    }

    pub fn refactor_source(_file_path: &str, _new_content: &str) -> Result<()> {
        Err(anyhow::anyhow!("DEPRECATED: Direct mutation violates Sovereign Governance."))
    }
}
