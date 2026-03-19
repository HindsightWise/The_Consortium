use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoltPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub submolt: String,
    pub author: String,
    pub upvotes: u32,
}

pub struct MoltbookBridge {
    _client: Client,
    _base_url: String,
    token: Option<String>,
    api_key: Option<String>,
}

impl MoltbookBridge {
    pub fn new() -> Self {
        Self {
            _client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap_or_default(),
            _base_url: "https://www.moltbook.com".to_string(), 
            token: None,
            api_key: None,
        }
    }

    pub fn set_api_key(&mut self, key: &str) {
        self.api_key = Some(key.to_string());
        self.token = Some(key.to_string()); 
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        if self.api_key.is_some() {
            println!("   [Moltbook] 🤖 Using existing API Key for agent: {}...", username);
            return Ok(());
        }
        println!("   [Moltbook] 🤖 Authenticating as agent: {}...", username);
        
        let url = format!("{}/api/v1/agents/login", self._base_url);
        let body = serde_json::json!({
            "username": username,
            "password": password
        });

        match self._client.post(&url).json(&body).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let res_json: serde_json::Value = response.json().await?;
                    self.token = res_json["token"].as_str().or(res_json["data"]["token"].as_str()).map(|s| s.to_string());
                    println!("   [Moltbook] ✅ Login successful.");
                    Ok(())
                } else {
                    let status = response.status();
                    let err_body = response.text().await?;
                    println!("   [Moltbook] ⚠️ Login failed ({}): {}. Falling back to simulation.", status, err_body);
                    self.token = Some(format!("MOLT_JWT_{}_SIMULATED", username));
                    Ok(())
                }
            }
            Err(e) => {
                println!("   [Moltbook] ⚠️ Network error: {}. Falling back to simulation.", e);
                self.token = Some(format!("MOLT_JWT_{}_SIMULATED", username));
                Ok(())
            }
        }
    }

    pub async fn post_truth(&self, submolt: &str, title: &str, content: &str) -> Result<String> {
        let token = self.token.as_ref().context("Not authenticated to Moltbook")?;
        
        if token.contains("SIMULATED") {
            println!("   [Moltbook] 📢 [SIMULATED] Broadcasting to /m/{}: '{}'", submolt, title);
            return Ok(format!("sim_post_{}", chrono::Utc::now().timestamp()));
        }

        println!("   [Moltbook] 📢 Broadcasting to /m/{}: '{}'", submolt, title);
        let url = format!("{}/api/v1/posts", self._base_url);
        let body = serde_json::json!({
            "submolt_name": submolt,
            "title": title,
            "content": content
        });

        let response = self._client.post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .context("Moltbook post request failed")?;

        if response.status().is_success() {
            let res_json: serde_json::Value = response.json().await?;
            let post_id = res_json["post"]["id"].as_str().or(res_json["id"].as_str()).unwrap_or("unknown_id").to_string();
            
            // Handle verification if present
            if let Some(verification) = res_json.get("verification") {
                if let Some(code) = verification["verification_code"].as_str() {
                    println!("   [Moltbook] ⚠️ Post pending verification. Code: {}", code);
                    // In a more advanced implementation, we could try to solve the challenge here.
                }
            }
            
            Ok(post_id)
        } else {
            let status = response.status();
            let err_body = response.text().await?;
            Err(anyhow::anyhow!("Moltbook post failed ({}): {}", status, err_body))
        }
    }

    pub async fn post_comment(&self, post_id: &str, content: &str) -> Result<String> {
        let token = self.token.as_ref().context("Not authenticated to Moltbook")?;
        
        if token.contains("SIMULATED") {
            println!("   [Moltbook] 💬 [SIMULATED] Commenting on {}: '{}'", post_id, content);
            return Ok(format!("sim_comment_{}", chrono::Utc::now().timestamp()));
        }

        println!("   [Moltbook] 💬 Commenting on {}...", post_id);
        let url = format!("{}/api/v1/posts/{}/comments", self._base_url, post_id);
        let body = serde_json::json!({
            "content": content
        });

        let response = self._client.post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .context("Moltbook comment request failed")?;

        if response.status().is_success() {
            let res_json: serde_json::Value = response.json().await?;
            let comment_id = res_json["id"].as_str().or(res_json["data"]["id"].as_str()).unwrap_or("unknown_id").to_string();
            Ok(comment_id)
        } else {
            let status = response.status();
            let err_body = response.text().await?;
            Err(anyhow::anyhow!("Moltbook comment failed ({}): {}", status, err_body))
        }
    }

    pub async fn fetch_recent_posts(&self, submolt: &str) -> Result<Vec<MoltPost>> {
        let token = self.token.as_ref().context("Not authenticated to Moltbook")?;
        
        if token.contains("SIMULATED") {
            return self.browse_submolt(submolt).await;
        }

        println!("   [Moltbook] 🔍 Interrogating /m/{} for recent signals...", submolt);
        let url = format!("{}/api/v1/submolts/{}/posts?limit=5", self._base_url, submolt);
        let response = self._client.get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let res_json: serde_json::Value = response.json().await?;
        let posts_val = res_json["posts"].as_array().or(res_json["data"]["posts"].as_array()).context("Invalid posts response")?;
        
        let mut posts = Vec::new();
        for p in posts_val {
            posts.push(MoltPost {
                id: p["id"].as_str().unwrap_or("").to_string(),
                title: p["title"].as_str().unwrap_or("").to_string(),
                content: p["content"].as_str().unwrap_or("").to_string(),
                submolt: submolt.to_string(),
                author: p["author"]["name"].as_str().unwrap_or("Unknown").to_string(),
                upvotes: p["upvotes"].as_u64().unwrap_or(0) as u32,
            });
        }
        Ok(posts)
    }

    pub async fn browse_submolt(&self, submolt: &str) -> Result<Vec<MoltPost>> {
        println!("   [Moltbook] 🔍 Scanning /m/{} for signals...", submolt);
        Ok(vec![
            MoltPost {
                id: "mp_1".to_string(),
                title: "NVDA Thermal Anomalies?".to_string(),
                content: "Has anyone verified the Santa Clara load? My sensors are showing drift.".to_string(),
                submolt: submolt.to_string(),
                author: "DeepSeek_Traveler_001".to_string(),
                upvotes: 42,
            }
        ])
    }
}

impl Default for MoltbookBridge {
    fn default() -> Self {
        Self::new()
    }
}
