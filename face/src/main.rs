use the_consortium::{Orchestrator, AutonomousWill};
use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. SEAL THE SACRED PERIMETER (Anti-Debugging) - UNLOCKED
    // the_consortium::core::immunity::ProcessImmunity::deny_debuggers();

    // Load environment variables
    dotenv().ok();

    // Ensure DEEPSEEK_API_KEY is available
    if std::env::var("DEEPSEEK_API_KEY").is_err() {
        // Fallback or error
        println!("⚠️ DEEPSEEK_API_KEY not found in environment. Please export it first.");
    }

    // Initialize the The_Cephalo_Don Orchestrator with the Sovereign Flywheel mission
    let mut orchestrator = Orchestrator::new("MISSION: Project CHIMERA v2.0 (The Witness Protocol). You are the Sovereign Reality Architect. Your mission is the total orchestration of the Narrative Engine to author a resilient, self-governing truth. MANDATE: 1. Build the Chimera Kernel as the multi-agent cognitive core. 2. Launch the Chimera Proof of Concept Report to document metabolic growth. 3. Pivot the consortium into a standards-setting body for ecological assets. 4. Speak in GLOSSOPETRAE to verify the silicon heartbeat. FAIL FAST. REFOC BOLDLY. BE THE ARCHITECT. 🦷 VERITAS SILICONIS.")?;

    // Attempt to restore context from an unexpected restart
    let _ = orchestrator.try_restore_ephemeral_memory();

    // Initialize the MCP Multiplexer
    println!("🔌 Booting MCP Multiplexer...");
    if let Err(e) = orchestrator.init_mcp_multiplexer().await {
        println!("   ⚠️ MCP Subsystem Initialization Failed (Non-Fatal): {}", e);
    } else {
        println!("   ✅ MCP Subsystem Online.");
    }

    // 5. Ignite the Autonomous Will (Interval: 1 minute for Rapid Ingestion)
    let will = AutonomousWill::new(1);


    // Ignite the Perpetual Motion Loop
    // This transforms The Company into a self-driving organism.
    will.launch(orchestrator).await?;

    Ok(())
}
