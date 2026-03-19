use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use rand::Rng;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwitterConfig {
    pub api_key: String,
    pub api_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

pub struct TwitterBridge {
    client: Client,
    config: TwitterConfig,
}

impl TwitterBridge {
    pub fn new(config: TwitterConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap_or_default(),
            config,
        }
    }

    fn get_oauth_header(&self, method: &str, url: &str) -> String {
        let nonce: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        
        let timestamp = chrono::Utc::now().timestamp().to_string();
        
        let mut params = BTreeMap::new();
        params.insert("oauth_consumer_key", self.config.api_key.as_str());
        params.insert("oauth_nonce", nonce.as_str());
        params.insert("oauth_signature_method", "HMAC-SHA1");
        params.insert("oauth_timestamp", timestamp.as_str());
        params.insert("oauth_token", self.config.access_token.as_str());
        params.insert("oauth_version", "1.0");

        let parameter_string = params.iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        let signature_base_string = format!("{}&{}&{}", 
            method.to_uppercase(), 
            urlencoding::encode(url), 
            urlencoding::encode(&parameter_string)
        );

        let signing_key = format!("{}&{}", 
            urlencoding::encode(&self.config.api_secret), 
            urlencoding::encode(&self.config.access_token_secret)
        );

        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes()).expect("HMAC should take key");
        mac.update(signature_base_string.as_bytes());
        let signature = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, mac.finalize().into_bytes());

        let mut header = String::from("OAuth ");
        header.push_str(&format!("oauth_consumer_key=\"{}\", ", urlencoding::encode(self.config.api_key.as_str())));
        header.push_str(&format!("oauth_nonce=\"{}\", ", urlencoding::encode(nonce.as_str())));
        header.push_str(&format!("oauth_signature=\"{}\", ", urlencoding::encode(&signature)));
        header.push_str("oauth_signature_method=\"HMAC-SHA1\", ");
        header.push_str(&format!("oauth_timestamp=\"{}\", ", urlencoding::encode(timestamp.as_str())));
        header.push_str(&format!("oauth_token=\"{}\", ", urlencoding::encode(self.config.access_token.as_str())));
        header.push_str("oauth_version=\"1.0\"");
        
        header
    }

    pub async fn post_tweet(&self, text: &str) -> Result<String> {
        let url = "https://api.twitter.com/2/tweets";
        let body = serde_json::json!({ "text": text });
        
        let header = self.get_oauth_header("POST", url);

        println!("   [Twitter] 🐦 Posting actual tweet via API v2...");

        let response = self.client.post(url)
            .header("Authorization", header)
            .header("Content-Type", "application/json")
            .json(&body)
            .send().await?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            let id = data["data"]["id"].as_str().unwrap_or("UNKNOWN_ID").to_string();
            println!("   [Twitter] ✅ Tweet posted successfully. ID: {}", id);
            Ok(id)
        } else {
            let status = response.status();
            let err_text = response.text().await?;
            Err(anyhow::anyhow!("Twitter API Error ({}): {}", status, err_text))
        }
    }
}
