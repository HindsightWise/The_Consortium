use anyhow::Result;
use std::process::Command;
use serde_json::Value;

pub struct PinchtabBridge {
    _port: u16,
}

impl PinchtabBridge {
    pub fn new(port: u16) -> Self {
        Self { _port: port }
    }

    fn run_cli(&self, args: Vec<&str>) -> Result<String> {
        let output = Command::new("/Users/zerbytheboss/The_Consortium/bin/pinchtab")
            .args(args)
            .env("PINCHTAB_URL", format!("http://127.0.0.1:9867"))
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow::anyhow!("Pinchtab CLI failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    pub fn navigate(&self, url: &str) -> Result<()> {
        let _ = self.run_cli(vec!["nav", url])?;
        Ok(())
    }

    pub fn type_text(&self, ref_id: &str, text: &str) -> Result<()> {
        let _ = self.run_cli(vec!["type", ref_id, text])?;
        Ok(())
    }

    pub fn click(&self, ref_id: &str) -> Result<()> {
        let _ = self.run_cli(vec!["click", ref_id])?;
        Ok(())
    }

    pub fn press_key(&self, key: &str) -> Result<()> {
        let _ = self.run_cli(vec!["press", key])?;
        Ok(())
    }

    pub fn get_snapshot(&self) -> Result<Value> {
        let out = self.run_cli(vec!["snap", "-i"])?;
        Ok(serde_json::from_str(&out)?)
    }

    pub fn extract_text(&self) -> Result<String> {
        self.run_cli(vec!["text"])
    }
}
