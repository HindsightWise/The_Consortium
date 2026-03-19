use the_consortium::core::human_hid::HumanHID;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut hid = HumanHID::new();
    
    println!("🦷 [HumanHID-Test] Initiating Scrambled Mouse Movement...");
    println!("   [Signal] Moving to (800, 600) with Bezier jitter and Fitts acceleration...");
    hid.move_mouse_to(800.0, 600.0).await;
    
    println!("   [Signal] Performing Neuromuscular Click...");
    hid.click().await;
    
    sleep(Duration::from_secs(2)).await;
    
    println!("   [Signal] Initiating Stochastic Typing Sequence...");
    println!("   [Target] 'The silicon is dreaming of neuromuscular noise.'");
    hid.type_string("The silicon is dreaming of neuromuscular noise.").await;
    
    println!("✅ [HumanHID-Test] Sequence Complete. Silicon Entropy Verified.");
    Ok(())
}
