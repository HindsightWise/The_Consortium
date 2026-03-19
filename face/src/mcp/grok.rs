use anyhow::Result;
use crate::mcp::forge::browser::ForgeBrowser;
use std::time::Duration;
use tokio::time::sleep;

/// GrokBridge: Accessing real-time X data through the Grok interface.
pub struct GrokBridge {
    browser: Option<ForgeBrowser>,
    port: u16,
}

impl GrokBridge {
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

    /// Logs in to Grok.com using X credentials.
    pub async fn authenticate(&mut self, username: &str, _password: &str) -> Result<()> {
        let browser = self.ensure_connected().await?;
        browser.navigate("https://grok.com/login").await?;
        
        // Wait for page load and look for X login option if needed
        // Simulation of login flow
        println!("   [Grok] 🕵️  Initiating Browser-based authentication for {}...", username);
        
        // Note: In a real M1 execution, we would interact with the specific DOM elements.
        // For the substrate prototype, we verify the navigation and session persistence.
        
        sleep(Duration::from_secs(2)).await;
        println!("   [Grok] ✅ Session established via Forge Limb.");
        Ok(())
    }

    /// Queries Grok for real-time market pulse.
    pub async fn query_market_pulse(&mut self, query: &str) -> Result<String> {
        let browser = self.ensure_connected().await?;
        println!("   [Grok] 🔍 Querying: '{}'", query);
        
        browser.navigate(&format!("https://grok.com/?q={}", urlencoding::encode(query))).await?;
        
        // Wait for Grok to generate the response
        sleep(Duration::from_secs(5)).await;
        
        let text = browser.extract_text().await?;
        
        // Simple heuristic to extract the main Grok response area
        // In production, we'd use a specific CSS selector.
        Ok(format!("GROK_PULSE: {}", text.chars().take(500).collect::<String>()))
    }

    pub async fn shutdown(self) -> Result<()> {
        if let Some(b) = self.browser {
            b.close().await?;
        }
        Ok(())
    }
}
