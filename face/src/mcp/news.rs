use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::mcp::web_search::WebSearch;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub url: String,
    pub relation_to_asset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingContext {
    pub headline: String,
    pub global_impact_score: f32, // -1.0 to 1.0
    pub local_context: Option<String>,
    pub relevant_articles: Vec<NewsArticle>,
}

pub struct NewsBridge;

impl NewsBridge {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_trending_news(&self, symbol: &str, location: Option<&str>, searcher: &WebSearch) -> Result<TrendingContext> {
        let query = format!("current trending news {} stocks market impact {}", symbol, location.unwrap_or("World"));
        let _ = searcher.search(&query).await?;

        // In a live system, we would pass search_results to an LLM (Ollama/DeepSeek) 
        // to generate the relation_to_asset assessment.
        // For the prototype, we use high-fidelity simulation of current Feb 2026 trends.
        
        let local = location.map(|l| format!("Local news for {}: Tech hub expansion and energy grid strain noted.", l));
        
        let articles = vec![
            NewsArticle {
                title: "Fed Signals Potential Rate Cut as Inflation Cools".to_string(),
                url: "https://finance.example.com/fed-updates".to_string(),
                relation_to_asset: format!("Bullish for {}: Lower rates increase DCF valuation for growth tech.", symbol),
            },
            NewsArticle {
                title: "Global Supply Chain: Port Congestion Easing in Asia".to_string(),
                url: "https://logistics.example.com/supply-chain".to_string(),
                relation_to_asset: "Confirmed: Supports NVDA/AMD physical turnover and logistics activity.".to_string(),
            }
        ];

        Ok(TrendingContext {
            headline: "Tech Resurgence Amidst Macro Stabilization".to_string(),
            global_impact_score: 0.65,
            local_context: local,
            relevant_articles: articles,
        })
    }
}

impl Default for NewsBridge {
    fn default() -> Self {
        Self::new()
    }
}
