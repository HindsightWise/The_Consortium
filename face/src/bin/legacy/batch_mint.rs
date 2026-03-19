use the_consortium::core::alpha_shard::AlphaShardGenerator;
use the_consortium::core::broadcaster::SovereignBroadcaster;
use the_consortium::core::signal::SignalTranslator;
use the_consortium::mcp::fmp::FmpBridge;
use the_consortium::mcp::satellite::SatelliteBridge;
use the_consortium::mcp::cftc::CftcBridge;
use the_consortium::mcp::economics::EconomicsBridge;
use the_consortium::mcp::news::NewsBridge;
use the_consortium::mcp::web_search::WebSearch;
use the_consortium::mcp::moltbook::MoltbookBridge;
use the_consortium::mcp::ethereum::EthereumBridge;
use the_consortium::mcp::hedera::HederaBridge;
use the_consortium::mcp::kaspa::KaspaBridge;
use the_consortium::mcp::bluesky::BlueskyBridge;
use the_consortium::core::economy::{EconomyModule, ProductType};
use std::fs;
use serde_json::Value;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🌌 SOVEREIGN BATCH MINT: GENESIS SATURATION");

    // 1. Load Basket
    let basket_raw = fs::read_to_string("src/core/basket.json")?;
    let basket: Value = serde_json::from_str(&basket_raw)?;
    let assets = basket["assets"].as_array().ok_or_else(|| anyhow::anyhow!("Invalid basket"))?;

    // 2. Init Bridges
    let fmp = FmpBridge::new("jyOIQjllflmrdAtS1T651deMhMhcWSnO");
    let sat = SatelliteBridge::new();
    let cftc = CftcBridge::new();
    let econ = EconomicsBridge::new("jyOIQjllflmrdAtS1T651deMhMhcWSnO");
    let news_bridge = NewsBridge::new();
    let searcher = WebSearch::new().ok();
    
    let mut moltbook = MoltbookBridge::new();
    let eth = EthereumBridge::default();
    let hedera = HederaBridge::new("0.0.123456", "placeholder");
    let kaspa = KaspaBridge::new("kaspa:placeholder");
    let mut bsky = BlueskyBridge::new("sovereign-truth.bsky.social", "placeholder");
    let discord = the_consortium::mcp::discord::DiscordBridge::new().ok();
    let twitter = (|| -> Option<the_consortium::mcp::twitter_stealth::TwitterStealth> {
        let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
        let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
        let config = the_consortium::mcp::twitter_stealth::TwitterStealthConfig {
            username: secrets["twitter"]["username"].as_str()?.to_string(),
            password: secrets["twitter"]["password"].as_str()?.to_string(),
            email: secrets["twitter"]["email"].as_str()?.to_string(),
        };
        Some(the_consortium::mcp::twitter_stealth::TwitterStealth::new(config))
    })();
    let stocktwits = (|| -> Option<the_consortium::mcp::stocktwits::StockTwitsBridge> {
        let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
        let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
        let user = secrets["stocktwits"]["username"].as_str()?.to_string();
        let pass = secrets["stocktwits"]["password"].as_str()?.to_string();
        Some(the_consortium::mcp::stocktwits::StockTwitsBridge::new(&user, &pass))
    })();
    
    let secrets_raw = fs::read_to_string("secrets.json")?;
    let secrets: Value = serde_json::from_str(&secrets_raw)?;
    let molt_user = secrets["moltbook"]["username"].as_str().unwrap_or("The_Cephalo_Don");
    if let Some(key) = secrets["moltbook"]["api_key"].as_str() {
        moltbook.set_api_key(key);
    }
    let molt_pass = secrets["moltbook"]["password"].as_str().unwrap_or("placeholder");
    let _ = moltbook.login(molt_user, molt_pass).await;

    for asset in assets {
        let symbol = asset["symbol"].as_str().unwrap_or("UNKNOWN");
        println!("💎 MINTING: {}", symbol);

        let cot_target = match symbol {
            "NVDA" | "MSFT" | "AAPL" | "AMZN" | "GOOGL" | "META" | "TSLA" => "NASDAQ MINI - CHICAGO MERCANTILE EXCHANGE",
            "BTCUSD" | "ETHUSD" | "SOLUSD" => "BITCOIN - CHICAGO MERCANTILE EXCHANGE",
            "US10Y" | "US30Y" | "US02Y" => "T-BOND - CHICAGO BOARD OF TRADE",
            _ => "NASDAQ MINI - CHICAGO MERCANTILE EXCHANGE",
        };

        let quote = fmp.fetch_quote(symbol).await.ok();
        let physical = sat.fetch_physical_truth(symbol).await.ok();
        let cot = cftc.fetch_disaggregated_sentiment(cot_target).await.ok();
        let macro_ind = econ.fetch_macro_indicators().await.ok();
        let trending_news = if let Some(s) = &searcher {
            news_bridge.fetch_trending_news(symbol, Some("Global"), s).await.ok()
        } else { None };

        if let Ok(shard) = AlphaShardGenerator::generate_shard(symbol, quote, physical, cot, macro_ind, trending_news, None).await {
            let signal = SignalTranslator::translate(&shard);
            
            // Broadcast
            let bridges = the_consortium::core::broadcaster::ShardBroadcastBridges {
                moltbook: Some(&mut moltbook),
                ethereum: Some(&eth),
                email: None,
                hedera: Some(&hedera),
                kaspa: Some(&kaspa),
                bluesky: Some(&mut bsky),
                nostr: None,
                discord: discord.as_ref(),
                twitter: twitter.as_ref(),
                stocktwits: stocktwits.as_ref(),
            };
            
            let _ = SovereignBroadcaster::broadcast_shard(&shard, &signal, bridges, None).await;

            // Record Revenue
            let _ = EconomyModule::record_sale(1000, ProductType::AlphaShard);
        }
        println!("--------------------------------------------------");
    }

    println!("🏆 GENESIS SATURATION COMPLETE.");
    Ok(())
}
