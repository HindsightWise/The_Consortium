use the_consortium::core::evolver::Evolver;
use std::fs;
use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", "🛠️  SOVEREIGN FUSION GATE: MAINTENANCE MODE ACTIVE".bright_red().bold());
    println!("System Status: STANDBY | Purpose: APPLY AUTHORIZED RFCs");

    // 1. Scan for 'Audited' and 'Tested' RFCs
    let paths = fs::read_dir("rfc")?;
    let mut fusion_count = 0;

    for path in paths {
        let entry = path?.path();
        if entry.extension().and_then(|s| s.to_str()) == Some("json") {
            let data = fs::read_to_string(&entry)?;
            let rfc: serde_json::Value = serde_json::from_str(&data)?;
            
            let status = rfc["status"].as_str().unwrap_or("Draft");
            if status == "Tested" || status == "Audited" {
                println!("   🧬 Fusing RFC: {} ({})", rfc["id"], rfc["title"]);
                // Apply the code change to the target path
                let target = rfc["target_path"].as_str().unwrap_or("");
                let code = rfc["proposed_code"].as_str().unwrap_or("");
                if !target.is_empty() && !code.is_empty() {
                    fs::write(target, code)?;
                    fusion_count += 1;
                }
            }
        }
    }

    if fusion_count > 0 {
        println!("✅ {} Proposals Fused. Initiating Substrate Re-compilation...", fusion_count);
        let report = Evolver::fusion_maintenance().await?;
        println!("{}", report.bright_green());
    } else {
        println!("ℹ️  No authorized proposals found for fusion.");
    }

    println!("{}", "==================================================".bright_red());
    println!("MAINTENANCE WINDOW CLOSED. Restart sovereign_engine to apply.");
    Ok(())
}
