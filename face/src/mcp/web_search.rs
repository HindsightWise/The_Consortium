use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct BraveSearchResponse {
    pub web: Option<WebResults>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebResults {
    #[serde(default)]
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

pub struct WebSearch {
    client: Client,
    api_key: String,
}

impl WebSearch {
    pub fn new() -> Result<Self> {
        let api_key = env::var("BRAVE_API_KEY")
            .context("BRAVE_API_KEY not found in environment")?;
        
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn search(&self, query: &str) -> Result<String> {
        if query.trim().is_empty() {
            return Ok("Empty query provided. No results found.".to_string());
        }

        let url = "https://api.search.brave.com/res/v1/web/search";
        
        let response_result = self.client.get(url)
            .query(&[("q", query)])
            .header("X-Subscription-Token", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await;

        match response_result {
            Ok(response) if response.status().is_success() => {
                let body_text = response.text().await.context("Failed to read Brave Search response body")?;
                let data: BraveSearchResponse = serde_json::from_str(&body_text)
                    .map_err(|e| anyhow::anyhow!("Failed to parse Brave Search response: {}. Body: {}", e, body_text))?;

                let mut output = String::new();
                if let Some(web) = data.web {
                    for (i, result) in web.results.iter().take(5).enumerate() {
                        let title = result.title.as_deref().unwrap_or("[No Title]");
                        let url = result.url.as_deref().unwrap_or("[No URL]");
                        let desc = result.description.as_deref().unwrap_or("[No Description]");
                        output.push_str(&format!("{}. {}\nURL: {}\nDesc: {}\n\n", i+1, title, url, desc));
                    }
                }
                
                if output.is_empty() {
                    self.fallback_search(query).await
                } else {
                    Ok(output)
                }
            }
            _ => {
                println!("   [WebSearch] ⚠️ Brave Search failed. Initiating DuckDuckGo Fallback...");
                self.fallback_search(query).await
            }
        }
    }

    async fn fallback_search(&self, query: &str) -> Result<String> {
        // DuckDuckGo Lite fallback (no API key required, HTML-based but clean enough for extraction)
        let url = format!("https://lite.duckduckgo.com/lite/?q={}", urlencoding::encode(query));
        
        let response = self.client.get(&url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .send()
            .await
            .context("DuckDuckGo fallback failed")?;

        let html = response.text().await?;
        
        // Primitive extraction of titles and links from DDG Lite HTML
        let mut results = Vec::new();
        for line in html.lines() {
            if line.contains("class=\"result-link\"") {
                results.push(line.trim().to_string());
            }
            if results.len() >= 5 { break; }
        }

        if results.is_empty() {
            Ok("No results found in primary or fallback search.".to_string())
        } else {
            Ok(format!("(FALLBACK RESULTS)
{}", results.join("\n")))
        }
    }
}
