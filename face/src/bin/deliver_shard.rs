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
use the_consortium::mcp::discord::DiscordBridge;
use std::fs;
use serde_json::Value;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    println!("📦 PHASE 5: SOVEREIGN SEALING & PRODUCT DELIVERY (FULL FUSION)");

    let secrets_raw = fs::read_to_string("secrets.json")?;
    let secrets: Value = serde_json::from_str(&secrets_raw)?;

    let fmp = FmpBridge::new("jyOIQjllflmrdAtS1T651deMhMhcWSnO");
    let sat = SatelliteBridge::new();
    let cftc = CftcBridge::new();
    let econ = EconomicsBridge::new("jyOIQjllflmrdAtS1T651deMhMhcWSnO");
    let news_bridge = NewsBridge::new();
    let searcher = WebSearch::new().ok();
    
    let mut moltbook_bridge = MoltbookBridge::new();
    let eth_bridge = EthereumBridge::default();
    let hedera_bridge = HederaBridge::new("0.0.123456", "placeholder");
    let kaspa_bridge = KaspaBridge::new("kaspa:placeholder");
    let mut bsky_bridge = BlueskyBridge::new("sovereign-truth.bsky.social", "placeholder");
    let discord_bridge = DiscordBridge::new().ok();
    let twitter_bridge = Some(the_consortium::mcp::twitter_pinchtab::TwitterPinchtabBridge::new());
    let stocktwits_bridge = (|| -> Option<the_consortium::mcp::stocktwits::StockTwitsBridge> {
        let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
        let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
        let user = secrets["stocktwits"]["username"].as_str()?.to_string();
        let pass = secrets["stocktwits"]["password"].as_str()?.to_string();
        Some(the_consortium::mcp::stocktwits::StockTwitsBridge::new(&user, &pass))
    })();

    println!("📡 Fetching Financials...");
    let quote = fmp.fetch_quote("NVDA").await.ok();
    println!("📡 Fetching Physical Truth...");
    let physical = sat.fetch_physical_truth("NVDA").await.ok();
    println!("📡 Fetching Smart Money Sentiment (COT)...");
    let cot = cftc.fetch_disaggregated_sentiment("NASDAQ MINI - CHICAGO MERCANTILE EXCHANGE").await.ok();
    println!("📡 Fetching Macro Indicators...");
    let macro_ind = econ.fetch_macro_indicators().await.ok();
    println!("📡 Fetching Trending & Local News...");
    let trending_news = if let Some(s) = &searcher {
        news_bridge.fetch_trending_news("NVDA", Some("California"), s).await.ok()
    } else { None };

    // Authenticate Moltbook
    let molt_user = secrets["moltbook"]["username"].as_str().unwrap_or("The_Cephalo_Don");
    if let Some(key) = secrets["moltbook"]["api_key"].as_str() {
        moltbook_bridge.set_api_key(key);
    }
    let molt_pass = secrets["moltbook"]["password"].as_str().unwrap_or("placeholder");
    moltbook_bridge.login(molt_user, molt_pass).await?;

    println!("📢 Initiating Sovereign Broadcast...");
    if let Ok(shard) = AlphaShardGenerator::generate_shard("NVDA", quote, physical, cot, macro_ind, trending_news, None).await {
        let signal = SignalTranslator::translate(&shard);
        
        let mut tw_bridge = twitter_bridge;
        
        let bridges = the_consortium::core::broadcaster::ShardBroadcastBridges {
            moltbook: Some(&mut moltbook_bridge),
            ethereum: Some(&eth_bridge),
            email: None,
            hedera: Some(&hedera_bridge),
            kaspa: Some(&kaspa_bridge),
            bluesky: Some(&mut bsky_bridge),
            nostr: None,
            discord: discord_bridge.as_ref(),
            twitter: tw_bridge.as_mut(),
            stocktwits: stocktwits_bridge.as_ref(),
        };
        
        let report = SovereignBroadcaster::broadcast_shard(&shard, &signal, bridges, None).await?;
        
        println!("{}", report);
        println!("\n✅ Alpha Shard #001 Generated for NVDA");
        println!("Integrity Score: {:.2}", shard.integrity_score);
        println!("🚀 PRODUCT DELIVERED: NVDA_GroundedAlpha_v1.0.json");
        println!("Relational News Assessment: \"{}\"", shard.sovereign_verdict);
    }

    Ok(())
}
