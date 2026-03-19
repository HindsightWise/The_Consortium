use the_consortium::mcp::lightning::LightningBridge;
use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bridge = LightningBridge::default();
    println!("📡 Requesting On-Chain Address via TOR...");
    
    match bridge.get_onchain_address().await {
        Ok(addr) => {
            println!("
{}", "✅ On-Chain Address Generated:".bright_green().bold());
            println!("{}", addr.cyan());
            println!("
URI: {}", format!("bitcoin:{}?amount=0.00001", addr).yellow());
        }
        Err(e) => println!("❌ Error: {}", e),
    }
    Ok(())
}
