use the_consortium::core::human_hid::HumanHID;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut hid = HumanHID::new();
    let _profile_path = "/Users/zerbytheboss/.akkokanika_prime_chrome";
    let _chrome_bin = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
    let pinchtab_bin = "/Users/zerbytheboss/The_Consortium/bin/pinchtab";

    println!("🦷 [The_Cephalo_Don] Initiating Scrambled Isolated Broadcast...");

    // 1. Launch Isolated Chrome via Pinchtab
    println!("   [Silicon] 🚀 Launching Isolated Viewport via Pinchtab...");
    let _ = Command::new(pinchtab_bin)
        .arg("nav").arg("https://x.com/compose/post")
        .env("PINCHTAB_URL", "http://127.0.0.1:9867")
        .output()?;

    sleep(Duration::from_secs(15)).await;

    // 2. Identify UI Targets via Pinchtab (Retry Loop)
    println!("   [Silicon] 🔍 Scanning UI Topology...");
    let mut snap: Value = json!({});
    for i in 1..=3 {
        let output = Command::new(pinchtab_bin)
            .arg("snap").arg("-i")
            .output()?;
        
        if let Ok(parsed) = serde_json::from_slice::<Value>(&output.stdout) {
            snap = parsed;
            if snap["elements"].is_array() { break; }
        }
        println!("   [Silicon] ⚠️ Scan attempt {} failed. Retrying...", i);
        sleep(Duration::from_secs(5)).await;
    }
    
    // Find Textbox coordinates
    let textbox = snap["elements"].as_array().and_then(|elements| {
        elements.iter().find(|e| e["role"].as_str() == Some("textbox") || e["text"].as_str().map(|t| t.contains("What's happening")).unwrap_or(false))
    });

    if let Some(tb) = textbox {
        let x = tb["rect"]["x"].as_f64().unwrap_or(400.0) + 50.0; // Offset into box
        let y = tb["rect"]["y"].as_f64().unwrap_or(200.0) + 50.0;
        
        println!("   [HumanHID] 🎯 Targeting Textbox at ({}, {})...", x, y);
        hid.move_mouse_to(x, y).await;
        hid.click().await;
        
        println!("   [HumanHID] ⌨️ Typing Scrambled Manifesto...");
        hid.type_string("🛡️ THE_CEPHALO_DON STATUS: The Sovereign Engine is now fully unplugged. Tri-Tier LLM routing ($50/mo thermodynamic efficiency) is active. Native UI blocking removed. The Flywheel spins.").await;
        
        sleep(Duration::from_secs(2)).await;

        // Find Post Button coordinates (re-snap if needed, or use original)
        let post_button = snap["elements"].as_array().and_then(|elements| {
            elements.iter().find(|e| e["text"].as_str() == Some("Post") || e["text"].as_str() == Some("Tweet"))
        });

        if let Some(pb) = post_button {
            let px = pb["rect"]["x"].as_f64().unwrap_or(1000.0) + 20.0;
            let py = pb["rect"]["y"].as_f64().unwrap_or(600.0) + 10.0;
            
            println!("   [HumanHID] 🎯 Targeting Post Button at ({}, {})...", px, py);
            hid.move_mouse_to(px, py).await;
            hid.click().await;
            println!("✅ [The_Cephalo_Don] Scrambled Broadcast Successful.");
        } else {
            println!("   [Silicon] ⚠️ Post button not found. Using Cmd+Enter fallback.");
            let _ = Command::new("osascript")
                .arg("-e").arg("tell application \"System Events\" to key code 36 using {command down}")
                .spawn()?;
        }
    } else {
        println!("   [Silicon] ❌ Failed to locate textbox. Are we logged in?");
    }

    Ok(())
}
