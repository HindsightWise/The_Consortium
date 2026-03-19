use anyhow::Result;
use the_consortium::mcp::alpaca::AlpacaBridge;

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- ALPHA SHARD DEPLOYMENT: PAPER TRADING ---");
    let alpaca = AlpacaBridge::default();

    // 1. Check Account
    println!("\n[1/3] Verifying Alpaca Account...");
    let account = alpaca.get_account().await?;
    println!("Account Status: {}", account);

    // 2. Execute Alpha Trades
    println!("\n[2/3] Executing Market Buy Orders...");
    
    let hon_order = alpaca.execute_order("HON", 10.0, "buy", "market", "gtc").await?;
    println!("$HON Order: {}", hon_order);

    let ba_order = alpaca.execute_order("BA", 10.0, "buy", "market", "gtc").await?;
    println!("$BA Order: {}", ba_order);

    // 3. Verify Positions
    println!("\n[3/3] Verifying Active Positions...");
    let positions = alpaca.get_positions().await?;
    println!("Current Positions:\n{}", positions);

    println!("\n--- DEPLOYMENT COMPLETE ---");
    Ok(())
}
