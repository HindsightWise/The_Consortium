use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::llm::automata::VisiblyRecursiveAutomaton;

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct MlxRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub max_tokens: u32,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::HashMap<String, f32>>,
}

#[derive(Deserialize, Debug)]
pub struct MlxChoiceMessage {
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct MlxChoice {
    pub message: MlxChoiceMessage,
}

#[derive(Deserialize, Debug)]
pub struct MlxResponse {
    pub choices: Option<Vec<MlxChoice>>,
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
        let automaton = VisiblyRecursiveAutomaton::new();
        let mask = automaton.prune_logits(31980);
        // println!("...

        let req = MlxRequest {
            model: model.to_string(),
            messages: vec![Message { role: "user".to_string(), content: prompt.to_string() }],
            stream: false,
            max_tokens: 1024,
            temperature: 0.7,
            logit_bias: Some(mask),
        };

        if let Ok(Ok(res)) = tokio::time::timeout(
            std::time::Duration::from_secs(120),
            self.client.post(format!("{}/v1/chat/completions", self.api_url))
                .json(&req)
                .send()
        ).await {
            let status = res.status();
            if let Ok(body) = res.text().await {
                if let Ok(val) = serde_json::from_str::<MlxResponse>(&body) {
                    if let Some(err) = val.error {
                        return Err(anyhow::anyhow!("MLX Error ({}): {}", status, err));
                    }
                    if let Some(choices) = val.choices {
                        if let Some(first) = choices.first() {
                            return Ok(first.message.content.clone());
                        }
                    }
                }
                return Err(anyhow::anyhow!("MLX Parsed Invalid Body: {}", body));
            }
        }

        Err(anyhow::anyhow!("MLX Substrate connection failed or timed out"))
    }
}
