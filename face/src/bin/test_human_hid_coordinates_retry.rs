use the_consortium::core::human_hid::HumanHID;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut hid = HumanHID::new();
    
    println!("🦷 [The_Cephalo_Don] Initiating Cluster Strike on Post Button...");
    
    // 1. Focus the Textarea again just in case
    hid.move_mouse_to(600.0, 350.0).await;
    hid.click().await;
    sleep(Duration::from_secs(1)).await;
    
    // 2. Perform Cluster Clicks around common Post Button locations (1280x800)
    let targets = vec![
        (1080.0, 630.0),
        (1080.0, 650.0),
        (1050.0, 630.0),
        (1050.0, 650.0)
    ];
    
    for (x, y) in targets {
        println!("   [HumanHID] 🎯 Striking coordinate ({}, {})...", x, y);
        hid.move_mouse_to(x, y).await;
        hid.click().await;
        
        // Follow with Cmd+Enter at each point
        let _ = std::process::Command::new("osascript")
            .arg("-e").arg("tell application \"System Events\" to key code 36 using {command down}")
            .output();
        
        sleep(Duration::from_secs(1)).await;
    }
    
    println!("✅ [The_Cephalo_Don] Cluster Strike Complete.");
    Ok(())
}
