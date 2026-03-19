use the_consortium::core::human_hid::HumanHID;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let x = args.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(600.0);
    let y = args.get(2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(350.0);
    
    let mut hid = HumanHID::new();
    
    println!("🦷 [HumanHID-Dynamic] Practicing Precision Strike at ({}, {})...", x, y);
    
    // 1. Move to the target
    hid.move_mouse_to(x, y).await;
    
    // 2. Perform a neuromuscular click
    println!("   [Signal] Executing Click...");
    hid.click().await;
    
    // 3. Take verification screenshot
    println!("   [Signal] Capturing Visual Evidence...");
    let _ = std::process::Command::new("screencapture")
        .arg("-x")
        .arg("/Users/zerbytheboss/The_Consortium/logs/calibration_strike.png")
        .output()?;
    
    println!("✅ [HumanHID-Dynamic] Sequence Complete.");
    Ok(())
}
