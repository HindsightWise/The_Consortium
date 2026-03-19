use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;
use regex::Regex;
use std::collections::HashMap;
use colored::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecFiling {
    pub id: String,
    pub ticker: String,
    pub cik: String,
    pub form_type: String,
    pub filing_date: String,
    pub link: String,
    pub content_summary: Option<String>,
    pub sentiment_score: f32,
}

pub struct SecAnalyzer {
    client: Client,
    user_agent: String,
}

impl SecAnalyzer {
    const RSS_URL: &'static str = "https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&output=atom";
    const TICKER_MAP_URL: &'static str = "https://www.sec.gov/files/company_tickers.json";

    pub fn new() -> Self {
        Self {
            client: Client::new(),
            user_agent: "TheCompany (Contact: admin@sovereign-truth.com) Rust-Core/0.1.0".to_string(),
        }
    }

    pub async fn get_ticker_map(&self) -> Result<HashMap<String, String>> {
        let resp = self.client.get(Self::TICKER_MAP_URL)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let mut map = HashMap::new();
        if let Some(obj) = resp.as_object() {
            for (_, company) in obj {
                if let (Some(cik), Some(ticker)) = (company.get("cik_str"), company.get("ticker")) {
                    let cik_str = format!("{:010}", cik.as_u64().unwrap_or(0));
                    map.insert(cik_str, ticker.as_str().unwrap_or("UNKNOWN").to_string());
                }
            }
        }
        Ok(map)
    }

    pub async fn poll_recent_filings(&self, target_forms: &[&str]) -> Result<Vec<SecFiling>> {
        println!("   [SEC] 📡 Polling SEC EDGAR Atom Feed...");
        let ticker_map = self.get_ticker_map().await.unwrap_or_default();
        
        let resp = self.client.get(Self::RSS_URL)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?
            .text()
            .await?;

        let mut filings = Vec::new();
        let entry_re = Regex::new(r"(?s)<entry>(.*?)</entry>")?;
        let title_re = Regex::new(r"<title>(.*?)</title>")?;
        let link_re = Regex::new(r#"<link.*?href="(.*?)""#)?;
        let _form_re = Regex::new(r#"<category.*?term="(.*?)".*?label="form type""#)?;
        let updated_re = Regex::new(r"<updated>(.*?)</updated>")?;
        let cik_re = Regex::new(r"data/(\d+)/")?;

        for cap in entry_re.captures_iter(&resp) {
            let entry_content = &cap[1];
            
            // Flexible extraction of the form type from the <category> tag
            let form_type = if let Some(c) = Regex::new(r#"<category[^>]*?term="(.*?)"[^>]*?label="form type""#)?.captures(entry_content) {
                c[1].to_string()
            } else if let Some(c) = Regex::new(r#"<category[^>]*?label="form type"[^>]*?term="(.*?)""#)?.captures(entry_content) {
                c[1].to_string()
            } else {
                continue;
            };

            if !target_forms.contains(&form_type.as_str()) {
                continue;
            }

            let _title = title_re.captures(entry_content).map(|c| c[1].to_string()).unwrap_or_default();
            let link = link_re.captures(entry_content).map(|c| c[1].to_string()).unwrap_or_default();
            let date = updated_re.captures(entry_content).map(|c| c[1].to_string()).unwrap_or_default();

            let cik = cik_re.captures(&link).map(|c| format!("{:010}", c[1].parse::<u64>().unwrap_or(0))).unwrap_or_default();
            let ticker = ticker_map.get(&cik).cloned().unwrap_or_else(|| "UNKNOWN".to_string());

            filings.push(SecFiling {
                id: uuid::Uuid::new_v4().to_string(),
                ticker,
                cik,
                form_type,
                filing_date: date,
                link,
                content_summary: None,
                sentiment_score: 0.0,
            });

            if filings.len() >= 10 { break; }
        }

        Ok(filings)
    }

    pub async fn analyze_filing(&self, filing: &mut SecFiling) -> Result<()> {
        println!("   [SEC] 🔍 Analyzing Filing: {} ({})", filing.ticker.cyan(), filing.form_type.yellow());
        
        // In a real harvest, we'd follow the link, download the primary document,
        // and extract sections. For the initial core integration, we provide a structured
        // summary and simulated sentiment based on the form type and ticker status.
        
        let sentiment = match filing.form_type.as_str() {
            "8-K" => 0.1, // Often neutral to slightly positive (material events)
            "10-Q" | "10-K" => -0.05, // Often contains cautious risk disclosures
            "S-1" => 0.4, // Initial public offerings are typically growth-oriented
            _ => 0.0,
        };

        filing.sentiment_score = sentiment;
        filing.content_summary = Some(format!(
            "Automated analysis of {} filing for {}. Detected {} sentiment profile. \
            Source: {}", 
            filing.form_type, filing.ticker, 
            if sentiment > 0.0 { "POSITIVE" } else if sentiment < 0.0 { "CAUTIOUS" } else { "NEUTRAL" },
            filing.link
        ));

        Ok(())
    }
}
