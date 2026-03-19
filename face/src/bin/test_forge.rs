use the_consortium::mcp::forge::browser::ForgeBrowser;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the local Geckodriver
    let browser = ForgeBrowser::connect(4444).await?;
    
    // Navigate to a target
    browser.navigate("https://en.wikipedia.org/wiki/Main_Page").await?;
    
    // Capture visual state
    browser.capture_screenshot("logs/forge_test.png").await?;
    
    // Extract text
    let text = browser.extract_text().await?;
    println!("   [FORGE] 📄 Extracted {} characters of text from the page.", text.len());
    
    // Close session
    browser.close().await?;
    
    Ok(())
}
