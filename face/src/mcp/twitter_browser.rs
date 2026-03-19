use anyhow::{Result, Context};
use crate::mcp::forge::browser::ForgeBrowser;
use std::time::Duration;
use tokio::time::sleep;

pub struct TwitterBrowserBridge {
    browser: Option<ForgeBrowser>,
    port: u16,
}

impl TwitterBrowserBridge {
    pub fn new(port: u16) -> Self {
        Self {
            browser: None,
            port,
        }
    }

    async fn ensure_connected(&mut self) -> Result<&ForgeBrowser> {
        if self.browser.is_none() {
            self.browser = Some(ForgeBrowser::connect(self.port).await?);
        }
        Ok(self.browser.as_ref().unwrap())
    }

    pub async fn post_tweet(&mut self, username: &str, password: &str, text: &str) -> Result<String> {
        let browser = self.ensure_connected().await?;
        println!("   [Twitter-Browser] 🕵️  Navigating to X.com...");
        
        browser.navigate("https://x.com/i/flow/login").await?;
        sleep(Duration::from_secs(5)).await;

        // Note: Real browser interaction for login would go here.
        // For the Phase 3 substrate, we are assuming the browser has a persistent session 
        // or we use the headless python script which is already tuned for Playwright.
        
        println!("   [Twitter-Browser] ⌨️  Posting: "{}"", text);
        
        // Falling back to the python script for the actual heavy lifting of browser automation
        // as it is more resilient for X's complex DOM.
        let output = std::process::Command::new("python3")
            .arg("src/mcp/twitter_headless.py")
            .arg("post")
            .arg(text)
            .output()?;

        let res_text = String::from_utf8_lossy(&output.stdout);
        if res_text.contains("successfully") {
            Ok("SUCCESS".to_string())
        } else {
            Err(anyhow::anyhow!("Headless Failure: {}", res_text))
        }
    }
}
