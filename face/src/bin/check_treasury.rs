use the_consortium::mcp::lightning::LightningBridge;
use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", "==================================================".bright_green());
    println!("{}", "🏛️  SOVEREIGN TREASURY AUDIT: ON-CHAIN BALANCE".bright_green().bold());
    println!("{}", "==================================================".bright_green());

    let bridge = LightningBridge::default();
    println!("📡 Querying LND Blockchain Substrate via TOR...");

    match bridge.get_onchain_balance().await {
        Ok(balance) => {
            if balance > 0 {
                println!("
{}", "✅ TREASURY INITIALIZED!".bright_green().bold());
                println!("{} {} SATS", "Current On-Chain Balance:".cyan(), balance);
                println!("{}", "The Company is now officially CAPITALIZED with real Bitcoin.".bright_white());
            } else {
                println!("
{}", "⌛ TREASURY EMPTY (0 SATS)".yellow());
                println!("On-chain transactions usually require 3-6 confirmations to appear in LND balance.");
            }
        }
        Err(e) => {
            println!("
{}", "❌ AUDIT FAILED.".bright_red().bold());
            println!("Error: {}", e);
        }
    }

    Ok(())
}
