use the_consortium::{Orchestrator};
use the_consortium::core::ingestion::IngestionModule;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🏛️  [INGESTION] One-Off Physical Ingestion Pulse Initiated...");
    
    let mut orchestrator = Orchestrator::new("MISSION: Project CHIMERA v2.0 (The Witness Protocol).")?;
    
    // Set a mock siphoned value for the ingestion test
    orchestrator.state.metadata.insert("siphoned_value_usd".to_string(), "10000000.0".to_string());
    
    let report = IngestionModule::initiate_pulse(&mut orchestrator.state).await?;
    
    println!("✅ INGESTION REPORT:");
    println!("   Siphoned: ${:.2}", report.siphoned_value_usd);
    println!("   Ingested: ${:.2}", report.ingested_value_usd);
    println!("   Wallet: {}", report.wallet_address);
    println!("   Hash: {}", report.proof_hash);
    println!("   Status: {}", report.status);
    
    Ok(())
}
