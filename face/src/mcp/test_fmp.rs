use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("FMP_API_KEY").unwrap_or("jyOIQjllflmrdAtS1T651deMhMhcWSnO".to_string());
    let symbol = "COPX";
    let url = format!("https://financialmodelingprep.com/api/v3/quote/{}?apikey={}", symbol, api_key);
    
    println!("[TEST] Calling: {}", url);
    
    let client = Client::new();
    let response = client.get(&url).header("Accept", "application/json").send().await?;
    
    println!("[TEST] Status: {}", response.status());
    let body = response.text().await?;
    println!("[TEST] Body (first 500 chars):\n{}", &body[..std::cmp::min(500, body.len())]);
    
    Ok(())
}