use the_consortium::mcp::McpBridge;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let mut bridge = McpBridge::new().await?;
    
    let args: Vec<String> = std::env::args().collect();
    let msg = args.get(1).map(|s| s.as_str()).unwrap_or("The silicon is breathing.");
    
    println!("🦷 [The_Cephalo_Don] Initiating Sovereign Broadcast via Silicon Bridge...");
    
    let result = bridge.dispatch_internal("twitter_stealth_post", Some(json!({ "message": msg }))).await?;
    
    println!("✅ [The_Cephalo_Don] Result: {}", result);
    
    Ok(())
}
