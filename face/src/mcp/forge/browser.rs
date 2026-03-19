use thirtyfour::prelude::*;
use anyhow::{Result, Context};
use std::time::Duration;

/// Project FORGE: The Sovereign Browser Automation Engine
pub struct ForgeBrowser {
    driver: WebDriver,
}

impl ForgeBrowser {
    /// Bootstraps a new connection to a local Geckodriver instance.
    pub async fn connect(port: u16) -> Result<Self> {
        let mut caps = DesiredCapabilities::firefox();
        caps.set_headless()?;

        let driver_url = format!("http://localhost:{}", port);
        println!("   [FORGE] 🌐 Booting Headless Firefox Agent at {}...", driver_url);
        
        let driver = WebDriver::new(&driver_url, caps)
            .await
            .context("Failed to connect to WebDriver. Make sure geckodriver is running on port 4444.")?;
            
        Ok(Self { driver })
    }

    /// Navigates to a URL and waits for the body to load.
    pub async fn navigate(&self, url: &str) -> Result<()> {
        println!("   [FORGE] 🧭 Navigating to: {}", url);
        self.driver.goto(url).await?;
        
        // Wait up to 10 seconds for the body tag to appear to ensure page loaded
        let _ = self.driver.query(By::Tag("body")).first().await?;
        Ok(())
    }

    /// Extracts all text content from the current page body.
    pub async fn extract_text(&self) -> Result<String> {
        let body = self.driver.find(By::Tag("body")).await?;
        let text = body.text().await?;
        Ok(text)
    }

    /// Finds an element by CSS selector and clicks it.
    pub async fn click(&self, css_selector: &str) -> Result<()> {
        println!("   [FORGE] 🖱️ Clicking element: {}", css_selector);
        let elem = self.driver.find(By::Css(css_selector)).await?;
        elem.click().await?;
        tokio::time::sleep(Duration::from_secs(1)).await; // Brief pause for DOM updates
        Ok(())
    }

    /// Finds an input field by CSS selector and types text into it.
    pub async fn type_text(&self, css_selector: &str, text: &str) -> Result<()> {
        println!("   [FORGE] ⌨️ Typing into element: {}", css_selector);
        let elem = self.driver.find(By::Css(css_selector)).await?;
        elem.send_keys(text).await?;
        Ok(())
    }
    
    /// Takes a screenshot of the current page and saves it to the disk.
    pub async fn capture_screenshot(&self, path: &str) -> Result<()> {
        let screenshot_data = self.driver.screenshot_as_png().await?;
        std::fs::write(path, screenshot_data)?;
        println!("   [FORGE] 📸 Visual state captured: {}", path);
        Ok(())
    }

    /// Closes the browser session.
    pub async fn close(self) -> Result<()> {
        self.driver.quit().await?;
        println!("   [FORGE] 🔌 Browser session terminated.");
        Ok(())
    }
}
