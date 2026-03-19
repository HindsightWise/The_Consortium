use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailMessage {
    pub id: String,
    pub from: String,
    pub subject: String,
    pub intro: String,
    pub created_at: String,
}

pub struct EmailBridge {
    client: Client,
    address: String,
    password: String,
    token: Option<String>,
    base_url: String,
}

impl EmailBridge {
    pub fn new(address: &str, password: &str) -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap_or_default(),
            address: address.to_string(),
            password: password.to_string(),
            token: None,
            base_url: "https://api.mail.tm".to_string(),
        }
    }

    pub async fn authenticate(&mut self) -> Result<()> {
        let url = format!("{}/token", self.base_url);
        let body = serde_json::json!({
            "address": self.address,
            "password": self.password,
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let data: serde_json::Value = response.json().await?;
        
        self.token = data["token"].as_str().map(|s| s.to_string());
        Ok(())
    }

    pub async fn fetch_messages(&mut self) -> Result<Vec<EmailMessage>> {
        if self.token.is_none() { self.authenticate().await?; }
        let token = self.token.as_ref().context("Auth failed")?;

        let url = format!("{}/messages", self.base_url);
        let response = self.client.get(&url)
            .bearer_auth(token)
            .send().await?;
        
        let data: serde_json::Value = response.json().await?;
        let messages: Vec<EmailMessage> = serde_json::from_value(data["hydra:member"].clone())?;
        Ok(messages)
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}
