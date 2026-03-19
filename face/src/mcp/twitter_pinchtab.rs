use anyhow::{Result, Context, anyhow};
use crate::mcp::pinchtab::PinchtabBridge;
use std::process::Command;

pub struct TwitterPinchtabBridge {
    pub pinchtab: PinchtabBridge,
}

impl TwitterPinchtabBridge {
    pub fn new() -> Self {
        Self {
            pinchtab: PinchtabBridge::new(9867), // Keep as dummy so struct matches
        }
    }

    pub async fn post_tweet(&self, text: &str) -> Result<String> {
        println!("   [NSO-Supervisor] Executing Playwright Stealth payload for X.com...");
        
        // Execute the Node.js stealth post script
        let script_dir = "/Users/zerbytheboss/The_Consortium/.agents/skills/twitter/scripts";
        
        let output = Command::new("node")
            .current_dir(script_dir)
            .arg("stealth_post.js")
            .arg(text)
            .output()
            .context("Failed to execute Node stealth_post.js payload")?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("   [NSO-Supervisor] ❌ Playwright payload failed: {}", stderr);
            return Err(anyhow!("Playwright stealth_post.js failed: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("   [NSO-Supervisor] ✅ X.com Playwright Outreach Complete:\n{}", stdout);

        Ok("SUCCESS".to_string())
    }
}
