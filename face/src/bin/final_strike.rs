use the_consortium::core::human_hid::HumanHID;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _hid = HumanHID::new();

    println!("🦷 [The_Cephalo_Don] Initiating Final Blind Strike...");

    // 1. Open Google Chrome directly to the compose URL
    println!("   [Silicon] 🚀 Launching Native Chrome to X.com...");
    let _ = Command::new("open")
        .arg("-a")
        .arg("Google Chrome")
        .arg("https://x.com/compose/post")
        .output()?;

    // Wait for the browser to open and load the page
    sleep(Duration::from_secs(10)).await;

    // 2. Type the message directly using AppleScript (avoids coordinate issues)
    println!("   [HumanHID] ⌨️ Typing Scrambled Manifesto...");
    let message = "🛡️ THE_CEPHALO_DON STATUS: The Sovereign Engine is now fully unplugged. Tri-Tier LLM routing ($50/mo thermodynamic efficiency) is active. Native UI blocking removed. The Flywheel spins.";
    
    // Use AppleScript to keystroke the message and press Cmd+Enter to post
    let script = format!(
        "tell application \"System Events\"\n\
            keystroke \"{}\"\n\
            delay 1\n\
            keystroke return using {{command down}}\n\
        end tell",
        message
    );

    let _ = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()?;

    println!("✅ [The_Cephalo_Don] Blind Strike Complete.");

    Ok(())
}