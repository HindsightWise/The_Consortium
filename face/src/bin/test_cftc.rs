use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let base_url = "https://publicreporting.cftc.gov/resource";
    let dataset_id = "gpe5-46if"; // TFF Futures Only dataset (Updated 2026)
    let url = format!("{}/{}.json", base_url, dataset_id);
    
    let market_proxy = "NASDAQ 100 STOCK INDEX";
    
    println!("📡 Testing CFTC Fetch for: {}", market_proxy);
    
    let response = client.get(&url)
        .query(&[
            ("$limit", "100"), 
            ("$where", "market_and_exchange_names LIKE '%NASDAQ%'"),
            ("$order", "report_date_as_yyyy_mm_dd DESC")
        ])
        .send().await?;
    
    let status = response.status();
    let text = response.text().await?;
    
    println!("Status: {}", status);
    println!("Response Body: {}", text);
    
    Ok(())
}
