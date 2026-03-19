use the_consortium::mcp::lightning::LightningBridge;
use colored::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bridge = LightningBridge::default();
    
    // Extracted r_hash for the 1000 sat invoice
    let r_hash_base64 = "5r0a9cwsp0mkde2ygfsg5a57wfgpsnt4trlmty26mls="; 

    println!("{}", "==================================================".bright_blue());
    println!("{}", "📡 SOVEREIGN ZAP MONITOR: WAITING FOR 1000 SATS".bright_blue().bold());
    println!("{}", "==================================================".bright_blue());
    println!("Monitoring Payment Hash: {}", r_hash_base64.cyan());

    let mut attempts = 0;
    loop {
        attempts += 1;
        match bridge.check_payment(r_hash_base64).await {
            Ok(true) => {
                println!("\n{}", "⚡ ZAP DETECTED! 1,000 SATS RECEIVED.".bright_green().bold());
                println!("The Company Treasury has been initialized with real Bitcoin.");
                break;
            }
            Ok(false) => {
                if attempts % 5 == 0 {
                    println!("   [...] Still waiting for payment (Attempt {})...", attempts);
                }
            }
            Err(e) => {
                println!("   [!] Error polling node: {}", e.to_string().red());
            }
        }
        sleep(Duration::from_secs(3)).await;
    }

    Ok(())
}
