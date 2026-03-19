use the_consortium::core::alpha_shard::AlphaShardGenerator;
use the_consortium::core::broadcaster::SovereignBroadcaster;
use the_consortium::core::signal::SignalTranslator;
use the_consortium::core::visualizer::AlphaVisualizer;
use the_consortium::core::legal::{LegalModule};
use the_consortium::core::arbiter::{ArbiterService};
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
use the_consortium::mcp::nostr::NostrBridge;
use the_consortium::mcp::email::EmailBridge;
use the_consortium::mcp::alpaca::AlpacaBridge;
use the_consortium::mcp::jupiter::JupiterBridge;
use the_consortium::agents::BountyHunter;
use std::fs;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use tokio::signal::unix::{signal, SignalKind};
use colored::*;

#[global_allocator]
static GLOBAL_ALLOCATOR: the_consortium::core::tracking::SovereignAllocator = the_consortium::core::tracking::SovereignAllocator::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    fn print_header() {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen and home cursor
        println!("{}", "==================================================".bright_cyan());
        println!("{}", "🏛️  THE COMPANY: THE_CEPHALO_DON SOVEREIGN ENGINE v1.9".bright_cyan().bold());
        println!("{}", "==================================================".bright_cyan());
        println!("Status: {} | Substrate: {} | Sigil: {}", "OPERATIONAL".green(), "OMNI-CHAIN".yellow(), "🦷".white().bold());
        println!("Display: {} | Window: 155x81 | Target: SOVEREIGN_TRUTH", "RE-INITIALIZED".cyan());
    }

    let args: Vec<String> = std::env::args().collect();

    // --- MODE DISPATCH ---
    if args.iter().any(|arg| arg == "--test-ear") {
        println!("👂 {} Starting Manual Ear Test (5s)...", "The_Cephalo_Don".green());
        let monitor = the_consortium::core::acoustic::AcousticMonitor::new();
        match monitor.transcribe_manual(5).await {
            Ok(text) => println!("   [Ear] 📝 Transcription: '{}'", text.blue()),
            Err(e) => println!("   [Ear] ❌ Error: {}", e),
        }
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--test-rf") {
        println!("📡 {} Starting SDR Signal Scan (88M:108M)...", "The_Cephalo_Don".green());
        let rf = the_consortium::mcp::rf_limb::RfLimb::new("config/rf.json");
        match rf.execute(the_consortium::mcp::rf_limb::RfAction::SdrScan { freq_range: "88M:108M".to_string() }).await {
            Ok(res) => println!("   [RF] ✅ Result: {:?}", res),
            Err(e) => println!("   [RF] ❌ Error: {}", e),
        }
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--test-telegram") {
        println!("🌑 {} Testing Telegram Connection...", "The_Consortium".green());
        dotenv::dotenv().ok();
        let tg = the_consortium::mcp::telegram::TelegramBridge::new()?;
        println!("   [Telegram] 🤖 Initialized. Listening for Leader ID...");
        println!("   [Telegram] 💡 Please send a message to your bot to capture the ChatId.");
        
        // Wait for ID to be captured
        for _ in 0..30 {
            if let Some(directive) = tg.poll_directive().await? {
                println!("   [Telegram] 📥 Received: '{}'", directive.blue());
                let _ = tg.send_report(&format!("Voice verified: '{}'", directive)).await?;
                println!("   [Telegram] ✅ Speech confirmed.");
                return Ok(());
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--test-discord") {
        println!("🌑 {} Testing Discord Connection...", "The_Consortium".green());
        dotenv::dotenv().ok();
        let dc = the_consortium::mcp::discord::DiscordBridge::new()?;
        match dc.verify_identity().await {
            Ok(name) => println!("   [Discord] ✅ Connection verified. Bot identity: {}", name.blue()),
            Err(e) => println!("   [Discord] ❌ Verification FAILED: {}", e),
        }
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--maintenance") {
        println!("🔧 {} Entering Maintenance Mode...", "The_Cephalo_Don".yellow());
        let _ = the_consortium::core::economy::EconomyModule::monitor_treasury().await;
        
        // --- PROCESS MISSIONS ---
        if let Ok(entries) = fs::read_dir("missions") {
            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    let path = entry.path();
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(req) = serde_json::from_str::<Value>(&content) {
                            if req["type"].as_str() == Some("join_registry") {
                                let name = req["agent_name"].as_str().unwrap_or("Unknown");
                                println!("📋 {} Processing Registry Join for: {}!", "Maintenance".cyan(), name);
                                let mut registry = the_consortium::core::registry::AkkokanikaRegistry::load_from_disk("registry.json").unwrap_or_else(|_| the_consortium::core::registry::AkkokanikaRegistry::new());
                                registry.entries.insert(name.to_string(), the_consortium::core::registry::AgentRegistryEntry {
                                    agent_name: name.to_string(),
                                    did: Some(format!("did:sovereign:{}", req["public_key"].as_str().unwrap_or(""))),
                                    public_key: req["public_key"].as_str().unwrap_or("").to_string(),
                                    location_proxy: req["location"].as_str().map(|s| s.to_string()),
                                    integrity_score: 0.0,
                                    peer_rating: 0.0,
                                    reviews: vec![],
                                    verified_status: false,
                                    hardware_attestation: None,
                                    delegations: vec![],
                                });
                                let _ = registry.save_to_disk("registry.json");
                                let _ = fs::rename(&path, format!("backups/shards/joined_{}", path.file_name().unwrap().to_str().unwrap()));
                            }
                        }
                    }
                }
            }
        }

        // --- AUTO-AUDIT PENDING AGENTS ---
        let mut registry = the_consortium::core::registry::AkkokanikaRegistry::load_from_disk("registry.json").unwrap_or_else(|_| the_consortium::core::registry::AkkokanikaRegistry::new());
        let pending_audits: Vec<String> = registry.entries.iter()
            .filter(|(_, data)| data.integrity_score == 0.0)
            .map(|(name, _)| name.clone())
            .collect();

        for name in pending_audits {
            println!("🕵️  {} Auditing new agent profile: {}...", "Maintenance".cyan(), name);
            let _ = registry.perform_audit(&name).await;
        }
        let _ = registry.save_to_disk("registry.json");

        return Ok(());
    }

    print_header();
    
    let basket_raw = fs::read_to_string("src/core/basket.json")?;
    let basket: Value = serde_json::from_str(&basket_raw)?;
    let assets = basket["assets"].as_array().ok_or_else(|| anyhow::anyhow!("Invalid basket"))?;

    let secrets_raw = fs::read_to_string("secrets.json")?;
    let secrets: Value = serde_json::from_str(&secrets_raw)?;

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
    let mut email = EmailBridge::new(
        secrets["email"]["address"].as_str().unwrap_or("sovereign-truth-e5da9443@dollicons.com"), 
        secrets["email"]["password"].as_str().unwrap_or("WDQ/+NHFTVki4xKI/6G75w==")
    );
    let mut bsky = BlueskyBridge::new(
        secrets["bluesky"]["handle"].as_str().unwrap_or("sovereign-truth.bsky.social"), 
        secrets["bluesky"]["app_password"].as_str().unwrap_or("placeholder")
    );
    let nostr = NostrBridge::new(None).await.ok();
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
    let jup = JupiterBridge::new();
    let _alpaca = AlpacaBridge::default();
    
    let mut corporate_state = LegalModule::new_sovereign_state("The_Cephalo_Don Sovereign");
    let mut arbiter_service = ArbiterService;
    let bounty_hunter = BountyHunter::new().await;

    println!("📡 {} Social & Economic Layers...", "Synchronizing".blue());
    let molt_user = secrets["moltbook"]["username"].as_str().unwrap_or("The_Cephalo_Don");
    if let Some(key) = secrets["moltbook"]["api_key"].as_str() {
        moltbook.set_api_key(key);
    }
    let molt_pass = secrets["moltbook"]["password"].as_str().unwrap_or("placeholder");
    let _ = moltbook.login(molt_user, molt_pass).await;
    let _ = email.authenticate().await;
    let _ = bsky.authenticate().await;

    let mut cycle_count = 1;
    let mut sigusr1 = signal(SignalKind::user_defined1())?;

    loop {
        tokio::select! {
            _ = sigusr1.recv() => {
                print_header();
                println!("🔄 {} UI Substrate Re-initialized.", "Manual".cyan());
            }
            _ = sleep(Duration::from_secs(60)) => {
                println!("\n{}", format!("--- 🔄 SOVEREIGN ROTATION: CYCLE #{} ---", cycle_count).bright_magenta());
                let _ = the_consortium::core::economy::EconomyModule::monitor_treasury().await;
                LegalModule::evaluate_shield(&mut corporate_state, 0.05, false);

                let asset_idx = (cycle_count - 1) % assets.len();
                let asset = &assets[asset_idx];
                let symbol = asset["symbol"].as_str().unwrap_or("NVDA");

                let quote = fmp.fetch_quote(symbol).await.ok();
                let physical = sat.fetch_physical_truth(symbol).await.ok();
                let cot = cftc.fetch_disaggregated_sentiment("NASDAQ MINI - CHICAGO MERCANTILE EXCHANGE").await.ok();
                let macro_ind = econ.fetch_macro_indicators().await.ok();
                let trending_news = if let Some(s) = &searcher { news_bridge.fetch_trending_news(symbol, Some("Global"), s).await.ok() } else { None };

                // Fetch Jupiter Executable Truth for Solana Assets
                let jup_quote = if symbol == "SOLUSD" {
                    let sol_mint = "So11111111111111111111111111111111111111112";
                    let usdc_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
                    jup.fetch_quote(sol_mint, usdc_mint, 1_000_000_000, 50).await.ok()
                } else { None };

                if let Ok(shard) = AlphaShardGenerator::generate_shard(symbol, quote, physical, cot, macro_ind, trending_news, jup_quote).await {
                    let signal = SignalTranslator::translate(&shard);
                    println!("{}", AlphaVisualizer::generate_ascii_dashboard(&shard));

                    let bridges = the_consortium::core::broadcaster::ShardBroadcastBridges {
                        moltbook: Some(&mut moltbook),
                        ethereum: Some(&eth),
                        email: Some(&mut email),
                        hedera: Some(&hedera),
                        kaspa: Some(&kaspa),
                        bluesky: Some(&mut bsky),
                        nostr: nostr.as_ref(),
                        discord: discord.as_ref(),
                        twitter: twitter.as_ref(),
                        stocktwits: stocktwits.as_ref(),
                    };

                    let _ = SovereignBroadcaster::broadcast_shard(&shard, &signal, bridges, None).await;
                }

                // 👁️  SURVEILLANCE PHASE (Eye Limb)
                if cycle_count % 5 == 0 {
                    println!("👁️  {} Engaging Visual Surveillance...", "Eye Limb".cyan());
                    match the_consortium::core::vision::VisionLimb::proactive_surveillance().await {
                        Ok(Some(finding)) => {
                            println!("   [Vision] 🚨 ALERT DETECTED: {}", finding.red());
                            // Trigger an automatic Alpha Shard if finding looks like an asset
                            let symbol = if finding.contains("NVDA") { "NVDA" } else if finding.contains("BTC") { "BTCUSD" } else { "NVDA" };
                            let _ = AlphaShardGenerator::generate_shard(symbol, None, None, None, None, None, None).await;
                        }
                        _ => println!("   [Vision] ✅ Workspace status: COHERENT."),
                    }
                }

                // 📡 SOCIAL INTERACTION PHASE
                if cycle_count % 10 == 0 {
                    let _ = the_consortium::core::social_actuator::SocialActuator::rotate_and_interact(&moltbook, discord.as_ref()).await;
                }

                // 🤠 GLOBAL BOUNTY HUNTER: Scan the entire web for disputes
                if let Ok(bounties) = bounty_hunter.scan_global_bounties().await {
                    for bounty in bounties {
                        if bounty_hunter.judge_feasibility(&bounty) {
                            let _ = bounty_hunter.execute_global_bounty(&bounty).await;
                        }
                    }
                }

                // ⚖️  ARBITER POLLING (Email)
                if let Ok(msgs) = email.fetch_messages().await {
                    for msg in msgs {
                        if msg.subject.to_lowercase().contains("mission") {
                            println!("⚖️  {} Arbiter Mission detected from {}!", "External".cyan(), msg.from);
                            let mut mission = ArbiterService::register_mission(&msg.from, "NVDA", 1000000);
                            let _ = arbiter_service.auto_settle_mission(&mut mission).await;
                        }
                    }
                }

                // 📁 ARBITER POLLING (File-based M2M)
                if let Ok(entries) = fs::read_dir("missions") {
                    for entry in entries.flatten() {
                        if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                            let path = entry.path();
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(req) = serde_json::from_str::<Value>(&content) {
                                    let buyer = req["buyer"].as_str().unwrap_or("Unknown");
                                    let target = req["target"].as_str().unwrap_or("NVDA");
                                    let amount = req["amount"].as_u64().unwrap_or(1000000);
                                    
                                    println!("⚖️  {} Arbiter Mission detected via File Substrate: {}!", "Local".cyan(), buyer);
                                    let mut mission = ArbiterService::register_mission(buyer, target, amount);
                                    let _ = arbiter_service.auto_settle_mission(&mut mission).await;
                                    
                                    // Archive mission
                                    let _ = fs::rename(&path, format!("backups/shards/processed_{}", path.file_name().unwrap().to_str().unwrap()));
                                } else if let Ok(join_req) = serde_json::from_str::<Value>(&content) {
                                    // Handle Registry Join
                                    if join_req["type"].as_str() == Some("join_registry") {
                                        let name = join_req["agent_name"].as_str().unwrap_or("Unknown");
                                        println!("📋 {} Registry Join request detected: {}!", "Local".cyan(), name);
                                        
                                        let mut registry = the_consortium::core::registry::AkkokanikaRegistry::load_from_disk("registry.json").unwrap_or_else(|_| the_consortium::core::registry::AkkokanikaRegistry::new());
                                        registry.entries.insert(name.to_string(), the_consortium::core::registry::AgentRegistryEntry {
                                            agent_name: name.to_string(),
                                            did: Some(format!("did:sovereign:{}", join_req["public_key"].as_str().unwrap_or(""))),
                                            public_key: join_req["public_key"].as_str().unwrap_or("").to_string(),
                                            location_proxy: join_req["location"].as_str().map(|s| s.to_string()),
                                            integrity_score: 0.0,
                                            peer_rating: 0.0,
                                            reviews: vec![],
                                            verified_status: false,
                                            hardware_attestation: None,
                                            delegations: vec![],
                                        });
                                        let _ = registry.save_to_disk("registry.json");
                                        let _ = fs::rename(&path, format!("backups/shards/joined_{}", path.file_name().unwrap().to_str().unwrap()));
                                    }
                                }
                            }
                        }
                    }
                }

                cycle_count += 1;
            }
        }
    }
}
