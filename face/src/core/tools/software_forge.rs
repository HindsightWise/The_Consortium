use reqwest::Client;
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::path::Path;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub process_id: u32,
    pub filename: String,
    pub timestamp: u64,
    pub duration_ms: u64,
    pub output_hash: String,
    pub status: String,
}

pub struct SoftwareForge {
    client: Client,
    api_key: String,
}

impl Default for SoftwareForge {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareForge {
    pub fn new() -> Self {
        let api_key = env::var("DEEPSEEK_API_KEY")
            .unwrap_or_else(|_| "sk-3e6c0d23d0354fc7b4efc1ea1c59afcb".to_string());
        
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .unwrap_or_default();

        Self {
            client,
            api_key,
        }
    }

    /// Autonomously generates a Python script, saves it, executes it, and returns an ExecutionReceipt.
    pub async fn forge_and_execute_with_receipt(&self, spec: &str, output_filename: &str) -> Result<(String, ExecutionReceipt), String> {
        println!("   [FORGE] 🔨 Initiating autonomous software forge for specification...");
        
        let prompt = format!(
            "You are an elite autonomous software engineer. Write a complete, functional Python script that fulfills the following specification: {}. 
            OUTPUT ONLY VALID PYTHON CODE. Do not use markdown blocks like ```python. Just output the raw code.",
            spec
        );

        let body = json!({
            "model": "deepseek-reasoner",
            "messages": [
                {"role": "system", "content": "You output only raw python code."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.2
        });

        let response = self.client.post("https://api.deepseek.com/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API Error: {}", response.status()));
        }

        let json_resp: serde_json::Value = response.json().await.unwrap_or_default();
        let code = json_resp["choices"][0]["message"]["content"].as_str()
            .ok_or_else(|| "Failed to parse code from response".to_string())?;

        // Clean up any accidental markdown blocks
        let clean_code = code.replace("```python\n", "").replace("```python", "").replace("```", "");
        
        let filepath = format!("salvage/{}", output_filename);
        fs::write(&filepath, clean_code.trim())
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        println!("   [FORGE] 💾 Software successfully forged and saved to {}", filepath);
        
        // Execute the script and measure metrics
        println!("   [FORGE] ⚡ Executing {}...", filepath);
        let start_time = Instant::now();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

        let python_bin = if Path::new(".venv_sec/bin/python3").exists() {
            ".venv_sec/bin/python3"
        } else {
            "python3"
        };

        let output = Command::new(python_bin)
            .arg(&filepath)
            .output()
            .map_err(|e| format!("Failed to execute script: {}", e))?;
        
        let duration = start_time.elapsed().as_millis();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        let final_output = if output.status.success() { stdout } else { stderr };
        
        // Generate cryptographic proof of output
        let mut hasher = Sha256::new();
        hasher.update(final_output.as_bytes());
        let hash_result = format!("{:x}", hasher.finalize());

        // Construct receipt
        let receipt = ExecutionReceipt {
            process_id: std::process::id(),
            filename: output_filename.to_string(),
            timestamp,
            duration_ms: duration as u64,
            output_hash: hash_result,
            status: if output.status.success() { "SUCCESS".to_string() } else { "FAILURE".to_string() },
        };

        if output.status.success() {
            Ok((final_output, receipt))
        } else {
            Err(format!("Execution failed: {}", final_output))
        }
    }

    /// Legacy wrapper for backward compatibility
    pub async fn forge_and_execute(&self, spec: &str, output_filename: &str) -> Result<String, String> {
        match self.forge_and_execute_with_receipt(spec, output_filename).await {
            Ok((output, _)) => Ok(output),
            Err(e) => Err(e)
        }
    }
}
