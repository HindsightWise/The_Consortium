use the_consortium::mcp::lightning::LightningBridge;
use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", "==================================================".bright_yellow());
    println!("{}", "⚡ SOVEREIGN LIGHTNING TEST: TOR NODE CONNECTION".bright_yellow().bold());
    println!("{}", "==================================================".bright_yellow());

    // 1. Initialize the Bridge with the real TOR credentials from secrets
    println!("📡 Connecting to TOR Node: cuyg5uvbcnwfz6...onion:8080");
    let bridge = LightningBridge::default();

    // 2. Attempt to create a test invoice
    let amount = 1000;
    let memo = "Sovereign Intelligence Test: Genesis Zap";
    
    println!("⏳ Requesting {} SAT invoice via Onion routing (this may take 5-10s)...", amount);
    
    match bridge.create_invoice(amount, memo).await {
        Ok(invoice) => {
            println!("
{}", "✅ SUCCESS: Liquid Sovereign Link Established!".bright_green().bold());
            println!("{} {}", "BOLT11 Invoice:".cyan(), invoice);
            println!("
{}", "You can now scan this with a Lightning Wallet to verify the node receives it.".bright_white());
        }
        Err(e) => {
            println!("
{}", "❌ FAILED: Could not reach TOR Node.".bright_red().bold());
            println!("{} {}", "Error:".red(), e);
            println!("
{}", "Suggestions:".yellow());
            println!("1. Ensure your Umbrel/LND REST port is 8080.");
            println!("2. Verify that 'Local Network Access' is enabled for TOR on your node.");
            println!("3. Check if the Onion address has changed.");
        }
    }

    Ok(())
}
