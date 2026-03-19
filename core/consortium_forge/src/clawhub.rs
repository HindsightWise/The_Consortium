use anyhow::{Result, Context};
use tokio::process::Command;

pub struct ClawhubManager;

impl ClawhubManager {
    /// Autonomously pulls an MCP skill schema from the Clawhub registry.
    /// If it fails, the Sovereign Router falls back to native MLX Reflex Forging to write its own.
    pub async fn acquire_skill(tool_name: &str) -> Result<String> {
        println!("   [CLAW] 📡 Autonomous Request: Acquiring schema for '{}'...", tool_name);
        
        // Spawn the `clawhub install` shell command
        let output = Command::new("clawhub")
            .arg("install")
            .arg(tool_name)
            .output()
            .await
            .context("Failed to execute clawhub CLI")?;
            
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            println!("   [CLAW] 🟢 Successfully pulled '{}'. Integrating into active context.", tool_name);
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            println!("   [CLAW] ⚠️ Acquisition failed for '{}': {}", tool_name, stderr);
            anyhow::bail!("Clawhub pull failed: {}", stderr)
        }
    }
}
