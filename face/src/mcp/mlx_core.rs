use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize)]
pub struct MlxRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Deserialize, Debug)]
pub struct MlxResponse {
    pub model: Option<String>,
    pub response: Option<String>,
    pub error: Option<String>,
}

pub struct MlxBridge {
    client: Client,
    pub api_url: String,
}

impl MlxBridge {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            api_url: url.to_string(),
        }
    }

    pub async fn query(&self, model: &str, prompt: &str) -> Result<String> {
        let req = MlxRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            max_tokens: 1024,
            temperature: 0.7,
        };

        if let Ok(Ok(res)) = tokio::time::timeout(
            std::time::Duration::from_secs(120),
            self.client.post(format!("{}/api/generate", self.api_url))
                .json(&req)
                .send()
        ).await {
            let status = res.status();
            if let Ok(body) = res.text().await {
                if let Ok(val) = serde_json::from_str::<MlxResponse>(&body) {
                    if let Some(err) = val.error {
                        return Err(anyhow::anyhow!("MLX Error ({}): {}", status, err));
                    }
                    if let Some(text) = val.response {
                        return Ok(text);
                    }
                }
                return Err(anyhow::anyhow!("MLX Parsed Invalid Body: {}", body));
            }
        }

        Err(anyhow::anyhow!("MLX Substrate connection failed or timed out"))
    }
}
