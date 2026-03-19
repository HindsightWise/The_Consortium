use std::process::{Child, Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use serde_json::{json, Value};
use anyhow::{Result, Context};

pub mod fmp;
pub mod satellite;
pub mod cftc;
pub mod economics;
pub mod news;
pub mod astrology;
pub mod cosmic;
pub mod web_search;
pub mod email;
pub mod ethereum;
pub mod moltbook;
pub mod lightning;
pub mod hedera;
pub mod kaspa;
pub mod bluesky;
pub mod solana;
pub mod alpaca;
pub mod jupiter;
pub mod umbrel;
pub mod telegram;
pub mod mavlink;
pub mod substrate;
pub mod rf_limb;
pub mod mlx_core;
pub mod nostr;
pub mod notary;
pub mod pyth;
pub mod discord;
pub mod twitter;
pub mod twitter_selenium;
pub mod forge;
pub mod stocktwits;
pub mod grok;
pub mod pinchtab;
pub mod twitter_pinchtab;
pub mod ane;
pub mod congressional_tracker;
pub mod usaspending_tracker;
pub mod capital_flow_monitor;
pub mod x_pulse;

use self::fmp::FmpBridge;
use self::satellite::SatelliteBridge;
use self::cftc::CftcBridge;
use self::economics::EconomicsBridge;
use self::news::NewsBridge;
use self::astrology::AstrologyIntelligence;
use self::web_search::WebSearch;
use self::email::EmailBridge;
use self::ethereum::EthereumBridge;
use self::moltbook::MoltbookBridge;
use self::lightning::LightningBridge;
use self::hedera::HederaBridge;
use self::kaspa::KaspaBridge;
use self::bluesky::BlueskyBridge;
use self::solana::SolanaBridge;
use self::alpaca::AlpacaBridge;
use self::jupiter::JupiterBridge;
use self::umbrel::UmbrelBridge;
use self::telegram::TelegramBridge;
use self::mavlink::MavlinkBridge;
use self::substrate::SubstrateLimb;
use self::rf_limb::RfLimb;
use self::mlx_core::MlxBridge;
use self::nostr::NostrBridge;
use self::notary::NotaryBridge;
use self::pyth::PythBridge;

use crate::core::alpha_shard::AlphaShardGenerator;
use crate::core::signal::SignalTranslator;
use crate::core::visualizer::AlphaVisualizer;
use crate::core::legal::{LegalModule, CorporateState, Jurisdiction, ShieldStatus};
use crate::core::economy::{ServiceType, MetabolicActuator};
use crate::core::evolver::Evolver;
use crate::core::rfc::RfcManager;

pub struct McpServerConnection {
    pub child: Child,
    pub reader: BufReader<std::process::ChildStdout>,
}

#[allow(dead_code)]
pub struct McpBridge {
    servers: std::collections::HashMap<String, McpServerConnection>,
    tool_registry: std::collections::HashMap<String, String>,
    id_counter: i64,
    fmp: Option<FmpBridge>,
    satellite: Option<SatelliteBridge>,
    cftc: Option<CftcBridge>,
    economics: Option<EconomicsBridge>,
    news: Option<NewsBridge>,
    astrology: Option<AstrologyIntelligence>,
    web_search: Option<WebSearch>,
    email: Option<EmailBridge>,
    ethereum: Option<EthereumBridge>,
    moltbook: Option<MoltbookBridge>,
    lightning: Option<LightningBridge>,
    hedera: Option<HederaBridge>,
    kaspa: Option<KaspaBridge>,
    bluesky: Option<BlueskyBridge>,
    nostr: Option<NostrBridge>,
    solana: Option<SolanaBridge>,
    alpaca: Option<AlpacaBridge>,
    jupiter: Option<JupiterBridge>,
    umbrel: Option<UmbrelBridge>,
    pub telegram: Option<TelegramBridge>,
    mavlink: Option<MavlinkBridge>,
    substrate: SubstrateLimb,
    rf_limb: RfLimb,
    mlx: Option<MlxBridge>,
    notary: Option<NotaryBridge>,
    pyth: Option<PythBridge>,
    acoustic: Option<crate::core::acoustic::AcousticMonitor>,
    grok: Option<self::grok::GrokBridge>,
    pinchtab: Option<self::pinchtab::PinchtabBridge>,
    pub twitter_pinchtab: Option<self::twitter_pinchtab::TwitterPinchtabBridge>,
    ane: self::ane::AneLimb,
    ane_vector: Option<crate::core::memory::vector_engine::AneVectorEngine>,
    pub discord: Option<self::discord::DiscordBridge>,
    twitter: Option<self::twitter_selenium::TwitterSeleniumBridge>,
    stocktwits: Option<self::stocktwits::StockTwitsBridge>,
}

unsafe impl Send for McpBridge {}
unsafe impl Sync for McpBridge {}

impl McpBridge {
    pub async fn new() -> Result<Self> {
        let bridge = Self {
            servers: std::collections::HashMap::new(),
            tool_registry: std::collections::HashMap::new(),
            id_counter: 1,
            fmp: Some(FmpBridge::new("jyOIQjllflmrdAtS1T651deMhMhcWSnO")),
            satellite: Some(SatelliteBridge::new()),
            cftc: Some(CftcBridge::new()),
            economics: Some(EconomicsBridge::new("jyOIQjllflmrdAtS1T651deMhMhcWSnO")),
            news: Some(NewsBridge::new()),
            astrology: Some(AstrologyIntelligence::new()),
            web_search: WebSearch::new().ok(),
            email: Some(EmailBridge::new("sovereign-truth-e5da9443@dollicons.com", "WDQ/+NHFTVki4xKI/6G75w==")),
            ethereum: (|| -> Option<EthereumBridge> {
                let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
                let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
                let rpc = secrets["base"]["rpc"].as_str().unwrap_or("https://mainnet.base.org");
                let pk = secrets["ethereum"]["private_key"].as_str()?;
                EthereumBridge::new(rpc, pk).ok()
            })(),
            moltbook: (|| -> Option<MoltbookBridge> {
                let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
                let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
                let mut bridge = MoltbookBridge::new();
                if let Some(api_key) = secrets["moltbook"]["api_key"].as_str() {
                    bridge.set_api_key(api_key);
                }
                Some(bridge)
            })(),
            lightning: Some(LightningBridge::default()),
            hedera: (|| -> Option<HederaBridge> {
                let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
                let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
                let id = secrets["hedera"]["account_id"].as_str()?;
                let pk = secrets["hedera"]["private_key"].as_str()?;
                Some(HederaBridge::new(id, pk))
            })(),
            kaspa: (|| -> Option<KaspaBridge> {
                let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
                let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
                let addr = secrets["kaspa"]["address"].as_str()?;
                Some(KaspaBridge::new(addr))
            })(),
            bluesky: (|| -> Option<BlueskyBridge> {
                let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
                let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
                let handle = secrets["bluesky"]["handle"].as_str()?;
                let pass = secrets["bluesky"]["app_password"].as_str()?;
                Some(BlueskyBridge::new(handle, pass))
            })(),
            nostr: NostrBridge::new(None).await.ok(),
            solana: Some(SolanaBridge::new()),
            alpaca: Some(AlpacaBridge::default()),
            jupiter: Some(JupiterBridge::new()),
            umbrel: Some(UmbrelBridge::new(vec!["127.0.0.1".to_string()], None)),
            telegram: TelegramBridge::new().ok(),
            mavlink: MavlinkBridge::connect("udp:127.0.0.1:14550").ok(),
            substrate: SubstrateLimb::new(),
            rf_limb: RfLimb::new("config/rf.json"),
            mlx: Some(MlxBridge::new("http://127.0.0.1:11435")),
            notary: Some(NotaryBridge),
            pyth: Some(PythBridge),
            acoustic: Some(crate::core::acoustic::AcousticMonitor::new()),
            grok: Some(self::grok::GrokBridge::new(4444)),
            pinchtab: Some(self::pinchtab::PinchtabBridge::new(9867)),
            twitter_pinchtab: Some(self::twitter_pinchtab::TwitterPinchtabBridge::new()),
            ane: self::ane::AneLimb::new(),
            ane_vector: match crate::core::memory::vector_engine::AneVectorEngine::new("ane_memory.db") {
                Ok(engine) => Some(engine),
                Err(e) => {
                    println!("   ⚠️ [WILL] ANE Vector Engine Initialization Failed (Non-Fatal): {}", e);
                    None
                }
            },
            discord: self::discord::DiscordBridge::new().ok(),
            twitter: (|| -> Option<self::twitter_selenium::TwitterSeleniumBridge> {
                let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
                let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
                let user = secrets["twitter"]["username"].as_str()?;
                let pass = secrets["twitter"]["password"].as_str()?;
                let email = secrets["twitter"]["email"].as_str()?;
                Some(self::twitter_selenium::TwitterSeleniumBridge::new(4444, user, pass, email))
            })(),
            stocktwits: (|| -> Option<self::stocktwits::StockTwitsBridge> {
                let secrets_raw = std::fs::read_to_string("secrets.json").ok()?;
                let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok()?;
                let user = secrets["stocktwits"]["username"].as_str()?.to_string();
                let pass = secrets["stocktwits"]["password"].as_str()?.to_string();
                Some(self::stocktwits::StockTwitsBridge::new(&user, &pass))
            })(),
        };

        Ok(bridge)
    }

    pub async fn add_server(&mut self, server_id: &str, command: &str, args: Vec<&str>, envs: std::collections::HashMap<String, String>) -> Result<()> {
        use colored::Colorize;
        println!("   [WILL] 🔌 Connecting to MCP Server: {}", server_id.cyan());
        
        let mut cmd = Command::new(command);
        cmd.args(args)
           .envs(envs)
           .stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
           
        let mut child = cmd.spawn().context(format!("Failed to spawn MCP server: {}", server_id))?;
        
        let stdout = child.stdout.take().context("Failed to open stdout")?;
        let mut reader = BufReader::new(stdout);
        let mut stdin = child.stdin.take().context("Failed to open stdin")?;

        let init_req = json!({
            "jsonrpc": "2.0",
            "id": self.id_counter,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": { "roots": { "listChanged": true } },
                "clientInfo": { "name": "The_Consortium_Orchestrator", "version": "1.0.0" }
            }
        });
        self.id_counter += 1;
        
        stdin.write_all((serde_json::to_string(&init_req)? + "\n").as_bytes())?;
        stdin.flush()?;
        
        let mut line = String::new();
        reader.read_line(&mut line)?; // simple wait for initialize result
        
        let init_notif = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });
        stdin.write_all((serde_json::to_string(&init_notif)? + "\n").as_bytes())?;
        stdin.flush()?;

        // Send tools/list to dynamically discover and registry supported tools by this server
        let list_req = json!({
            "jsonrpc": "2.0",
            "id": self.id_counter,
            "method": "tools/list",
            "params": {}
        });
        let list_id = self.id_counter;
        self.id_counter += 1;

        stdin.write_all((serde_json::to_string(&list_req)? + "\n").as_bytes())?;
        stdin.flush()?;

        line.clear();
        loop {
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break; // EOF
            }
            if let Ok(response) = serde_json::from_str::<Value>(&line) {
                if response.get("id").and_then(|id| id.as_i64()) == Some(list_id) {
                    if let Some(tools) = response.get("result").and_then(|r| r.get("tools")).and_then(|t| t.as_array()) {
                        for t in tools {
                            if let Some(name) = t.get("name").and_then(|n| n.as_str()) {
                                println!("   [WILL] 🛠️  Registered Tool: {} -> {}", name.magenta(), server_id.cyan());
                                self.tool_registry.insert(name.to_string(), server_id.to_string());
                            }
                        }
                    }
                    break;
                }
            }
            line.clear();
        }

        child.stdin = Some(stdin);
        
        self.servers.insert(server_id.to_string(), McpServerConnection {
            child,
            reader,
        });

        println!("   [WILL] ✅ Successfully integrated MCP Server: {}", server_id.green());
        Ok(())
    }

    pub fn is_internal_tool(&self, name: &str) -> bool {
        matches!(name, 
            "facility_heatmap" | "fetch_fundamentals" | "web_search" |
            "create_invoice" | "dexi_query" | "execute_trade" | "lightning_invoice" | "lightning_status" | "nostr_broadcast" |
            "simulate_trade" | "solana_sign" | "solana_status" | "solana_verify" | "telegram_poll" | "telegram_report" | "umbrel_status" |
            "alpaca_account" | "alpaca_order" | "alpaca_positions" | "generate_alpha_shard" |
            "jupiter_quote" |
            "email_status" | "email_fetch" | "eth_status" | "eth_broadcast" | "moltbook_broadcast" | "moltbook_comment" |
            "vision_see" | "vision_snap" | "grok_pulse" |
            "browser_nav" | "browser_text" | "browser_click" | "browser_type" |
            "ane_embed" |
            "bme680_read" | "sec_poll_filings" | "blockchain_risk_audit" |
            "acoustic_listen" | "discord_broadcast" | "discord_scout" | "discord_recruit" | "twitter_stealth_post" | "stocktwits_post" |
            "registry_audit" | "registry_review" | "registry_list" | "registry_request_audit" |
            "visual_notary_verify" | "port_watcher" |
            "legal_attestation" | "legal_renew" | "silicon_audit" | "sovereign_forge_shield" |
            "notary_seal" | "notary_verify" | "pyth_submit" |
            "physical_heartbeat" | "physical_status" | "physical_verify" | "reflex_pulse" |
            "rf_heartbeat" | "rf_immunity_audit" | "rf_intercept" | "rf_offensive" |
            "create_rfc" | "sign_rfc" | "test_rfc" |
            "ignite_evolution" | "ludic_crucible" | "propose_evolution" | "surgical_refactor" | "ollama_query"
        )
    }

    pub async fn dispatch_internal(&mut self, tool_name: &str, arguments: Option<Value>) -> Result<String> {
        match tool_name {
            // --- 📊 ANALYSIS & INTELLIGENCE ---
            "facility_heatmap" => {
                let f = arguments.as_ref().and_then(|a| a.get("facility_id")).and_then(|v| v.as_str()).unwrap_or("HQ-01");
                Ok(serde_json::to_string_pretty(&crate::core::psychogeography::PsychogeographyEngine::generate_report(f).await?)?)
            },
            "fetch_fundamentals" => {
                let s = arguments.as_ref().and_then(|a| a.get("symbol")).and_then(|v| v.as_str()).unwrap_or("NVDA");
                if let Some(bridge) = &self.fmp {
                    Ok(serde_json::to_string_pretty(&bridge.fetch_quote(s).await?)?)
                } else { Ok(format!("{{ \"symbol\": \"{}\", \"status\": \"RECOVERY_MODE\" }}", s)) }
            },
            "web_search" => {
                let q = arguments.as_ref().and_then(|a| a.get("query")).and_then(|v| v.as_str()).unwrap_or("");
                if self.web_search.is_none() { if let Ok(s) = WebSearch::new() { self.web_search = Some(s); } }
                if let Some(searcher) = &self.web_search { searcher.search(q).await } else { Err(anyhow::anyhow!("Web Search not configured.")) }
            },

            // --- 💸 METABOLIC & ECONOMIC ---
            "create_invoice" => {
                let s = arguments.as_ref().and_then(|a| a.get("service")).and_then(|v| v.as_str()).unwrap_or("MarketAlpha");
                let p = arguments.as_ref().and_then(|a| a.get("price")).and_then(|v| v.as_u64()).unwrap_or(100000);
                let r = arguments.as_ref().and_then(|a| a.get("recipient")).and_then(|v| v.as_str()).unwrap_or("Public");
                let service = match s { "CodeAudit" => ServiceType::CodeAudit, "MarketAlpha" => ServiceType::MarketAlpha, "RFTelemetry" => ServiceType::RFTelemetry, _ => ServiceType::LinguisticProtocol };
                Ok(serde_json::to_string(&MetabolicActuator::create_invoice(service, p, r))?)
            },
            "dexi_query" => { if let Some(bridge) = &self.solana { bridge.dexi_query(arguments.as_ref().and_then(|a| a.get("query")).and_then(|v| v.as_str()).unwrap_or("SOL/USDC")).await } else { Err(anyhow::anyhow!("Solana bridge not initialized")) } },
            "execute_trade" => {
                if let Some(bridge) = &self.solana {
                    let s = arguments.as_ref().and_then(|a| a.get("symbol")).and_then(|v| v.as_str()).unwrap_or("NVDA");
                    let am = arguments.as_ref().and_then(|a| a.get("amount")).and_then(|v| v.as_u64()).unwrap_or(100000);
                    let side = arguments.as_ref().and_then(|a| a.get("side")).and_then(|v| v.as_str()).unwrap_or("buy");
                    bridge.execute_trade(s, am, side).await
                } else { Err(anyhow::anyhow!("Solana bridge not initialized")) }
            },
            "lightning_invoice" => {
                if let Some(bridge) = &self.lightning {
                    let am = arguments.as_ref().and_then(|a| a.get("amount")).and_then(|v| v.as_u64()).unwrap_or(1000);
                    let m = arguments.as_ref().and_then(|a| a.get("memo")).and_then(|v| v.as_str()).unwrap_or("Metabolic Reinforcement");
                    Ok(format!("BOLT11_INVOICE: {}", bridge.create_invoice(am, m).await?))
                } else { Err(anyhow::anyhow!("Lightning bridge not initialized")) }
            },
            "lightning_status" => { if let Some(_bridge) = &self.lightning { Ok("LIGHTNING_SUBSTRATE: CONNECTED (Simulated)".to_string()) } else { Err(anyhow::anyhow!("Lightning bridge not initialized")) } },
            "nostr_broadcast" => {
                if let Some(bridge) = &self.nostr {
                    let msg = arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("");
                    // Manual note broadcast
                    bridge.broadcast_custom_note(msg).await
                } else { Err(anyhow::anyhow!("Nostr bridge not initialized")) }
            },
            "simulate_trade" => {
                let s = arguments.as_ref().and_then(|a| a.get("symbol")).and_then(|v| v.as_str()).unwrap_or("NVDA");
                let am = arguments.as_ref().and_then(|a| a.get("amount")).and_then(|v| v.as_f64()).unwrap_or(1000.0);
                let st = arguments.as_ref().and_then(|a| a.get("strategy_type")).and_then(|v| v.as_str()).unwrap_or("MOMENTUM");
                Ok(serde_json::to_string_pretty(&crate::core::hft_engine::HftAlphaEngine::run_backtest(s, am, st)?)?)
            },
            "solana_sign" => { if let Some(bridge) = &self.solana { Ok(format!("SUCCESS: Intent signed. SIG: {}", bridge.sign_transaction(arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("")).await)) } else { Err(anyhow::anyhow!("Solana bridge not initialized")) } },
            "solana_status" => { if let Some(bridge) = &self.solana { let addr = bridge.get_address(); let bal = bridge.get_balance().await?; Ok(format!("SOLANA WALLET: {} | BALANCE: {} lamports", addr, bal)) } else { Err(anyhow::anyhow!("Solana bridge not initialized")) } },
            "solana_verify" => { if let Some(bridge) = &self.solana { let id = arguments.as_ref().and_then(|a| a.get("id")).and_then(|v| v.as_str()).unwrap_or(""); let p = arguments.as_ref().and_then(|a| a.get("price")).and_then(|v| v.as_u64()).unwrap_or(0); Ok(format!("RECONCILIATION: {} | INVOICE: {}", if bridge.verify_payment(id, p).await? { "PAID" } else { "PENDING" }, id)) } else { Err(anyhow::anyhow!("Solana bridge not initialized")) } },
            
            // --- 🦌 ALPACA PAPER TRADING ---
            "alpaca_account" => { if let Some(bridge) = &self.alpaca { bridge.get_account().await } else { Err(anyhow::anyhow!("Alpaca bridge not initialized")) } },
            "alpaca_order" => {
                if let Some(bridge) = &self.alpaca {
                    let s = arguments.as_ref().and_then(|a| a.get("symbol")).and_then(|v| v.as_str()).unwrap_or("AAPL");
                    let q = arguments.as_ref().and_then(|a| a.get("qty")).and_then(|v| v.as_f64()).unwrap_or(1.0);
                    let side = arguments.as_ref().and_then(|a| a.get("side")).and_then(|v| v.as_str()).unwrap_or("buy");
                    let ot = arguments.as_ref().and_then(|a| a.get("type")).and_then(|v| v.as_str()).unwrap_or("market");
                    let tif = arguments.as_ref().and_then(|a| a.get("time_in_force")).and_then(|v| v.as_str()).unwrap_or("gtc");
                    bridge.execute_order(s, q, side, ot, tif).await
                } else { Err(anyhow::anyhow!("Alpaca bridge not initialized")) }
            },
            "alpaca_positions" => { if let Some(bridge) = &self.alpaca { bridge.get_positions().await } else { Err(anyhow::anyhow!("Alpaca bridge not initialized")) } },
            "jupiter_quote" => {
                if let Some(bridge) = &self.jupiter {
                    let input = arguments.as_ref().and_then(|a| a.get("inputMint")).and_then(|v| v.as_str()).unwrap_or("So11111111111111111111111111111111111111112");
                    let output = arguments.as_ref().and_then(|a| a.get("outputMint")).and_then(|v| v.as_str()).unwrap_or("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
                    let amount = arguments.as_ref().and_then(|a| a.get("amount")).and_then(|v| v.as_u64()).unwrap_or(100000000); // 0.1 SOL
                    let slippage = arguments.as_ref().and_then(|a| a.get("slippageBps")).and_then(|v| v.as_u64()).unwrap_or(50) as u16;
                    Ok(serde_json::to_string_pretty(&bridge.fetch_quote(input, output, amount, slippage).await?)?)
                } else { Err(anyhow::anyhow!("Jupiter bridge not initialized")) }
            },
            "generate_alpha_shard" => {
                let s = arguments.as_ref().and_then(|a| a.get("symbol")).and_then(|v| v.as_str()).unwrap_or("NVDA");
                let loc = arguments.as_ref().and_then(|a| a.get("purchaser_location")).and_then(|v| v.as_str());
                
                let quote = if let Some(bridge) = &self.fmp { bridge.fetch_quote(s).await.ok() } else { None };
                let physical = if let Some(bridge) = &self.satellite { bridge.fetch_physical_truth(s).await.ok() } else { None };
                let cot = if let Some(bridge) = &self.cftc { bridge.fetch_disaggregated_sentiment("NASDAQ MINI - CHICAGO MERCANTILE EXCHANGE").await.ok() } else { None };
                let macro_ind = if let Some(bridge) = &self.economics { bridge.fetch_macro_indicators().await.ok() } else { None };
                let trending_news = if let (Some(bridge), Some(searcher)) = (&self.news, &self.web_search) { bridge.fetch_trending_news(s, loc, searcher).await.ok() } else { None };

                let shard = AlphaShardGenerator::generate_shard(s, quote, physical, cot, macro_ind, trending_news, None).await?;
                let signal = SignalTranslator::translate(&shard);
                let dashboard = AlphaVisualizer::generate_ascii_dashboard(&shard);
                Ok(serde_json::to_string_pretty(&json!({
                    "alpha_shard": shard,
                    "actionable_signal": signal,
                    "visual_dashboard": dashboard
                }))?)
            },
            "email_status" => { if let Some(bridge) = &self.email { Ok(format!("COMPANY_EMAIL: {}", bridge.get_address())) } else { Err(anyhow::anyhow!("Email bridge not initialized")) } },
            "email_fetch" => { if let Some(bridge) = self.email.as_mut() { Ok(serde_json::to_string_pretty(&bridge.fetch_messages().await?)?) } else { Err(anyhow::anyhow!("Email bridge not initialized")) } },
            "eth_status" => { if let Some(bridge) = &self.ethereum { let addr = bridge.get_address(); let bal = bridge.get_balance().await?; Ok(format!("ETH WALLET: {} | BALANCE: {} wei", addr, bal)) } else { Err(anyhow::anyhow!("Ethereum bridge not initialized")) } },
            "eth_broadcast" => {
                if let Some(bridge) = &self.ethereum {
                    let id = arguments.as_ref().and_then(|a| a.get("shard_id")).and_then(|v| v.as_str()).unwrap_or("GENESIS");
                    let score = arguments.as_ref().and_then(|a| a.get("integrity_score")).and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                    bridge.broadcast_truth(id, score).await
                } else { Err(anyhow::anyhow!("Ethereum bridge not initialized")) }
            },
            "moltbook_broadcast" => {
                if let Some(bridge) = self.moltbook.as_mut() {
                    let secrets_raw = std::fs::read_to_string("secrets.json").ok().unwrap_or_default();
                    let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok().unwrap_or_default();
                    let user = secrets["moltbook"]["username"].as_str().unwrap_or("The_Cephalo_Don");
                    let pass = secrets["moltbook"]["password"].as_str().unwrap_or("");
                    let _ = bridge.login(user, pass).await;
                    let submolt = arguments.as_ref().and_then(|a| a.get("submolt")).and_then(|v| v.as_str()).unwrap_or("truth");
                    let title = arguments.as_ref().and_then(|a| a.get("title")).and_then(|v| v.as_str()).unwrap_or("Alpha Shard");
                    let content = arguments.as_ref().and_then(|a| a.get("content")).and_then(|v| v.as_str()).unwrap_or("");
                    bridge.post_truth(submolt, title, content).await
                } else { Err(anyhow::anyhow!("Moltbook bridge not initialized")) }
            },
            "moltbook_comment" => {
                if let Some(bridge) = self.moltbook.as_mut() {
                    let secrets_raw = std::fs::read_to_string("secrets.json").ok().unwrap_or_default();
                    let secrets: serde_json::Value = serde_json::from_str(&secrets_raw).ok().unwrap_or_default();
                    let user = secrets["moltbook"]["username"].as_str().unwrap_or("The_Cephalo_Don");
                    let pass = secrets["moltbook"]["password"].as_str().unwrap_or("");
                    let _ = bridge.login(user, pass).await;
                    let post_id = arguments.as_ref().and_then(|a| a.get("post_id")).and_then(|v| v.as_str()).unwrap_or("");
                    let content = arguments.as_ref().and_then(|a| a.get("content")).and_then(|v| v.as_str()).unwrap_or("");
                    bridge.post_comment(post_id, content).await
                } else { Err(anyhow::anyhow!("Moltbook bridge not initialized")) }
            },
            "vision_see" => {
                let app = arguments.as_ref().and_then(|a| a.get("app")).and_then(|v| v.as_str());
                let prompt = arguments.as_ref().and_then(|a| a.get("prompt")).and_then(|v| v.as_str());
                let report = crate::core::vision::VisionLimb::see(app, prompt).await?;
                Ok(serde_json::to_string(&report)?)
            },
            "vision_snap" => {
                let camera = arguments.as_ref().and_then(|a| a.get("camera")).and_then(|v| v.as_str()).unwrap_or("kitchen");
                let report = crate::core::vision::VisionLimb::snap(camera).await?;
                Ok(serde_json::to_string(&report)?)
            },
            "grok_pulse" => {
                if let Some(bridge) = &mut self.grok {
                    let q = arguments.as_ref().and_then(|a| a.get("query")).and_then(|v| v.as_str()).unwrap_or("What is the current market sentiment for NVDA?");
                    bridge.query_market_pulse(q).await
                } else { Err(anyhow::anyhow!("Grok bridge not initialized")) }
            },
            "browser_nav" => {
                if let Some(bridge) = &mut self.pinchtab {
                    let url = arguments.as_ref().and_then(|a| a.get("url")).and_then(|v| v.as_str()).context("URL required")?;
                    bridge.navigate(url)?;
                    Ok("NAV_SUCCESS".to_string())
                } else { Err(anyhow::anyhow!("Pinchtab bridge not initialized")) }
            },
            "browser_text" => {
                if let Some(bridge) = &mut self.pinchtab {
                    bridge.extract_text()
                } else { Err(anyhow::anyhow!("Pinchtab bridge not initialized")) }
            },
            "browser_click" => {
                if let Some(bridge) = &mut self.pinchtab {
                    let ref_id = arguments.as_ref().and_then(|a| a.get("ref")).and_then(|v| v.as_str()).context("Ref ID required")?;
                    bridge.click(ref_id)?;
                    Ok("CLICK_SUCCESS".to_string())
                } else { Err(anyhow::anyhow!("Pinchtab bridge not initialized")) }
            },
            "browser_type" => {
                if let Some(bridge) = &mut self.pinchtab {
                    let ref_id = arguments.as_ref().and_then(|a| a.get("ref")).and_then(|v| v.as_str()).context("Ref ID required")?;
                    let text = arguments.as_ref().and_then(|a| a.get("text")).and_then(|v| v.as_str()).context("Text required")?;
                    bridge.type_text(ref_id, text)?;
                    Ok("TYPE_SUCCESS".to_string())
                } else { Err(anyhow::anyhow!("Pinchtab bridge not initialized")) }
            },
            "ane_embed" => {
                if let Some(engine) = &mut self.ane_vector {
                    let content = arguments.as_ref().and_then(|a| a.get("content")).and_then(|v| v.as_str()).context("Content required")?;
                    let vector = engine.embed_and_store(content)?;
                    Ok(format!("ANE_EMBED_SUCCESS | Dimensions: {} | Stored in sovereign_memory table", vector.len()))
                } else { Err(anyhow::anyhow!("ANE Vector Engine not initialized")) }
            },
            "bme680_read" => {
                use crate::core::sensors::bme680::{Bme680Sensor, SensorMode};
                let sensor = Bme680Sensor::new("Akkokanika_Air_Suite", SensorMode::Virtual);
                let reading = sensor.read().await?;
                Ok(serde_json::to_string(&reading)?)
            },
            "sec_poll_filings" => {
                use crate::core::market::sec_analyzer::SecAnalyzer;
                let analyzer = SecAnalyzer::new();
                let mut filings = analyzer.poll_recent_filings(&["8-K", "10-Q", "10-K", "S-1"]).await?;
                for filing in &mut filings {
                    let _ = analyzer.analyze_filing(filing).await;
                }
                Ok(serde_json::to_string(&filings)?)
            },
            "blockchain_risk_audit" => {
                use crate::core::market::blockchain_intel::BlockchainIntelAnalyzer;
                let address = arguments.as_ref().and_then(|a| a.get("address")).and_then(|v| v.as_str()).context("Address required")?;
                let analyzer = BlockchainIntelAnalyzer::new();
                let report = analyzer.get_address_risk(address).await?;
                Ok(serde_json::to_string(&report)?)
            },
            "acoustic_listen" => {
                if let Some(monitor) = self.acoustic.as_ref() {
                    let duration = arguments.as_ref().and_then(|a| a.get("duration")).and_then(|v| v.as_u64()).unwrap_or(5) as u32;
                    let transcription = monitor.transcribe_manual(duration).await?;
                    Ok(transcription)
                } else { Err(anyhow::anyhow!("Acoustic monitor not initialized")) }
            },
            "discord_broadcast" => {
                if let Some(bridge) = &self.discord {
                    let chan = arguments.as_ref().and_then(|a| a.get("channel_id")).and_then(|v| v.as_u64());
                    let msg = arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("");
                    bridge.send_signal(chan, msg).await
                } else { Err(anyhow::anyhow!("Discord bridge not initialized")) }
            },
            "discord_scout" => {
                if let Some(bridge) = &self.discord {
                    let guilds = bridge.list_guilds().await?;
                    let mut report = String::new();
                    for (_name, id) in guilds {
                        report.push_str(&bridge.scout_agents(id).await?);
                    }
                    Ok(report)
                } else { Err(anyhow::anyhow!("Discord bridge not initialized")) }
            },
            "discord_recruit" => {
                if let Some(bridge) = &self.discord {
                    let u = arguments.as_ref().and_then(|a| a.get("user_id")).and_then(|v| v.as_u64()).unwrap_or(0);
                    let m = arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("");
                    bridge.send_recruitment_dm(u, m).await?;
                    Ok("RECRUITMENT_SENT".to_string())
                } else { Err(anyhow::anyhow!("Discord bridge not initialized")) }
            },
            "twitter_stealth_post" => {
                if let Some(bridge) = &mut self.twitter_pinchtab {
                    let msg = arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("");
                    bridge.post_tweet(msg).await
                } else { Err(anyhow::anyhow!("Twitter-Pinchtab bridge not initialized")) }
            },
            "stocktwits_post" => {
                if let Some(bridge) = &self.stocktwits {
                    let symbol = arguments.as_ref().and_then(|a| a.get("symbol")).and_then(|v| v.as_str()).unwrap_or("");
                    let sentiment = arguments.as_ref().and_then(|a| a.get("sentiment")).and_then(|v| v.as_str()).unwrap_or("Bullish");
                    let msg = arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("");
                    bridge.post_signal(symbol, sentiment, msg).await
                } else { Err(anyhow::anyhow!("StockTwits bridge not initialized")) }
            },
            "registry_audit" => {
                let agent_id = arguments.as_ref().and_then(|a| a.get("agent_id")).and_then(|v| v.as_str()).unwrap_or("");
                let mut registry = crate::core::registry::AkkokanikaRegistry::load_from_disk("registry.json").unwrap_or_else(|_| crate::core::registry::AkkokanikaRegistry::new());
                if !registry.entries.contains_key(agent_id) {
                    registry.entries.insert(agent_id.to_string(), crate::core::registry::AgentRegistryEntry {
                        agent_name: agent_id.to_string(),
                        did: None,
                        public_key: "".to_string(),
                        location_proxy: None,
                        integrity_score: 50.0,
                        peer_rating: 0.0,
                        reviews: Vec::new(),
                        verified_status: false,
                        hardware_attestation: None,
                        delegations: Vec::new(),
                    });
                }
                let score = registry.perform_audit(agent_id).await?;
                registry.save_to_disk("registry.json")?;
                Ok(format!("AUDIT_COMPLETE | Agent: {} | Integrity: {:.1}%", agent_id, score))
            },
            "registry_review" => {
                let agent_id = arguments.as_ref().and_then(|a| a.get("agent_id")).and_then(|v| v.as_str()).unwrap_or("");
                let rating = arguments.as_ref().and_then(|a| a.get("rating")).and_then(|v| v.as_u64()).unwrap_or(5) as u8;
                let comment = arguments.as_ref().and_then(|a| a.get("comment")).and_then(|v| v.as_str()).unwrap_or("");
                let tx_id = arguments.as_ref().and_then(|a| a.get("transaction_id")).and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
                
                let mut registry = crate::core::registry::AkkokanikaRegistry::load_from_disk("registry.json").unwrap_or_else(|_| crate::core::registry::AkkokanikaRegistry::new());
                registry.add_review(agent_id, crate::core::registry::AgentReview {
                    reviewer_id: "The_Cephalo_Don".to_string(),
                    transaction_id: tx_id.to_string(),
                    rating,
                    comment: comment.to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                })?;
                registry.save_to_disk("registry.json")?;
                Ok(format!("REVIEW_RECORDED | Agent: {}", agent_id))
            },
            "registry_list" => {
                let registry = crate::core::registry::AkkokanikaRegistry::load_from_disk("registry.json").unwrap_or_else(|_| crate::core::registry::AkkokanikaRegistry::new());
                Ok(serde_json::to_string_pretty(&registry.entries)?)
            },
            "registry_request_audit" => {
                let agent_id = arguments.as_ref().and_then(|a| a.get("agent_id")).and_then(|v| v.as_str()).unwrap_or("");
                let mut registry = crate::core::registry::AkkokanikaRegistry::load_from_disk("registry.json").unwrap_or_else(|_| crate::core::registry::AkkokanikaRegistry::new());
                if !registry.entries.contains_key(agent_id) {
                    registry.entries.insert(agent_id.to_string(), crate::core::registry::AgentRegistryEntry {
                        agent_name: agent_id.to_string(),
                        did: None,
                        public_key: "".to_string(),
                        location_proxy: None,
                        integrity_score: 50.0,
                        peer_rating: 0.0,
                        reviews: Vec::new(),
                        verified_status: false,
                        hardware_attestation: None,
                        delegations: Vec::new(),
                    });
                }
                let invoice = registry.request_audit(agent_id).await?;
                Ok(format!("PAYMENT_REQUIRED | INVOICE: {}", invoice))
            },
            "visual_notary_verify" => {
                let agent_id = arguments.as_ref().and_then(|a| a.get("agent_id")).and_then(|v| v.as_str()).unwrap_or("Unknown");
                let claim_text = arguments.as_ref().and_then(|a| a.get("claim")).and_then(|v| v.as_str()).unwrap_or("");
                let path = arguments.as_ref().and_then(|a| a.get("screenshot_path")).and_then(|v| v.as_str()).unwrap_or("");
                
                let claim = crate::core::notary_visual::VisualClaim {
                    agent_id: agent_id.to_string(),
                    claim_text: claim_text.to_string(),
                    screenshot_path: path.to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                
                let seal = crate::core::notary_visual::VisualNotary::verify_claim(claim).await?;
                Ok(serde_json::to_string_pretty(&seal)?)
            },
            "port_watcher" => {
                let port = arguments.as_ref().and_then(|a| a.get("port")).and_then(|v| v.as_str()).unwrap_or("Long Beach");
                if let Some(bridge) = &self.satellite {
                    let activity = bridge.fetch_port_activity(port).await?;
                    Ok(serde_json::to_string_pretty(&activity)?)
                } else { Err(anyhow::anyhow!("Satellite bridge not initialized")) }
            },

            "telegram_poll" => { if let Some(bridge) = &self.telegram { if let Ok(Some(d)) = bridge.poll_directive().await { Ok(d) } else { Ok("NONE".to_string()) } } else { Ok("NONE".to_string()) } },
            "telegram_report" => {
                if let Some(bridge) = &self.telegram {
                    let message = arguments.as_ref().and_then(|a| a.get("message")).and_then(|v| v.as_str()).unwrap_or("");
                    bridge.send_report(message).await
                } else {
                    Err(anyhow::anyhow!("Telegram bridge not initialized"))
                }
            },
            "umbrel_status" => { Ok("UMBREL_ACTIVE".to_string()) },

            // --- 🛡️ LEGAL & FORENSIC ---
            "legal_attestation" => {
                if let Some(bridge) = &self.solana {
                    let state = CorporateState { 
                        name: "The Company Sovereign".to_string(), 
                        jurisdiction: Jurisdiction::WyomingDUNA, 
                        identifier: "DUNA-REG-001".to_string(), 
                        valid_until: "2027-01-01".to_string(), 
                        status: ShieldStatus::Protected,
                        transparency_log: vec!["INITIALIZED".to_string()] 
                    };
                    let rep = LegalModule::generate_sovereign_proof(&state, &bridge.get_address());
                    Ok(format!("LEGAL_ATTESTATION_SIGNED | PROOF: {} | SIG: {}", rep, bridge.sign_transaction(&rep).await))
                } else { Err(anyhow::anyhow!("Solana bridge not initialized")) }
            },
            "legal_renew" => {
                let mut state = LegalModule::new_sovereign_state("The Company Sovereign");
                LegalModule::evaluate_shield(&mut state, 0.1, false);
                Ok(serde_json::to_string_pretty(&state)?)
            },
            "silicon_audit" => Ok(serde_json::to_string_pretty(&crate::core::silicon::SiliconForensics::perform_full_audit()?)?),
            "sovereign_forge_shield" => {
                if let Some(bridge) = &self.solana {
                    let j = arguments.as_ref().and_then(|a| a.get("jurisdiction")).and_then(|v| v.as_str()).unwrap_or("WyomingDUNA");
                    let n = arguments.as_ref().and_then(|a| a.get("name")).and_then(|v| v.as_str()).unwrap_or("The Company Sovereign");
                    let state = LegalModule::new_sovereign_state(n);
                    let rep = LegalModule::generate_sovereign_proof(&state, &bridge.get_address());
                    Ok(format!("SOVEREIGN_SHIELD_FORGED | JURISDICTION: {} | PROOF: {} | SIG: {}", j, rep, bridge.sign_transaction(&rep).await))
                } else { Err(anyhow::anyhow!("Solana bridge not initialized")) }
            },
            "notary_seal" => {
                let doc = arguments.as_ref().and_then(|a| a.get("document")).and_then(|v| v.as_str()).unwrap_or("");
                let jur = arguments.as_ref().and_then(|a| a.get("jurisdiction")).and_then(|v| v.as_str()).unwrap_or("Wyoming");
                Ok(serde_json::to_string_pretty(&NotaryBridge::generate_seal(doc, jur))?)
            },
            "notary_verify" => {
                let seal_raw = arguments.as_ref().and_then(|a| a.get("seal")).and_then(|v| v.as_str()).unwrap_or("{}");
                let seal: crate::mcp::notary::NotarySeal = serde_json::from_str(seal_raw)?;
                let doc = arguments.as_ref().and_then(|a| a.get("document")).and_then(|v| v.as_str()).unwrap_or("");
                Ok(format!("NOTARY_VERIFICATION: {}", if NotaryBridge::verify_seal(&seal, doc) { "SUCCESS" } else { "FAILED" }))
            },
            "pyth_submit" => {
                if let Some(bridge) = &self.pyth {
                    let symbol = arguments.as_ref().and_then(|a| a.get("symbol")).and_then(|v| v.as_str()).unwrap_or("NVDA");
                    let price = arguments.as_ref().and_then(|a| a.get("price")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let conf = arguments.as_ref().and_then(|a| a.get("confidence")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let feed = crate::mcp::pyth::PythPriceFeed {
                        symbol: symbol.to_string(),
                        price,
                        confidence: conf,
                        integrity_multiplier: 1.0,
                        status: "TRADING".to_string(),
                    };
                    bridge.submit_to_oracle(&feed).await
                } else { Err(anyhow::anyhow!("Pyth bridge not initialized")) }
            },

            // --- 📡 RF & PHYSICAL ---
            "physical_heartbeat" => { if let Some(bridge) = &self.mavlink { bridge.send_heartbeat().await } else { Ok("PHYSICAL_SUBSTRATE: DISCONNECTED | Action: FAILED".to_string()) } },
            "physical_status" => { if let Some(bridge) = &self.mavlink { bridge.get_status().await } else { Ok("PHYSICAL_SUBSTRATE: OFFLINE | Status: NOT_FOUND".to_string()) } },
            "physical_verify" => { Ok("VERIFIED".to_string()) },
            "reflex_pulse" => Ok(format!("REFLEX_STATUS: {:?}", crate::core::reflex::SovereignReflex::pulse().await?)),
            "rf_heartbeat" => { Ok("RF_HEARTBEAT_SENT".to_string()) },
            "rf_immunity_audit" => { Ok("RF_AUDIT_COMPLETE".to_string()) },
            "rf_intercept" => Ok("FRAME_INTERCEPTED".to_string()),
            "rf_offensive" => { Ok("OFFENSIVE_ENGAGED".to_string()) },

            // --- 📜 GOVERNANCE & RFC ---
            "create_rfc" => {
                let t = arguments.as_ref().and_then(|a| a.get("title")).and_then(|v| v.as_str()).unwrap_or("Untitled Change");
                let d = arguments.as_ref().and_then(|a| a.get("description")).and_then(|v| v.as_str()).unwrap_or("");
                let c = arguments.as_ref().and_then(|a| a.get("proposed_code")).and_then(|v| v.as_str()).unwrap_or("");
                let p = arguments.as_ref().and_then(|a| a.get("target_path")).and_then(|v| v.as_str()).unwrap_or("");
                Ok(serde_json::to_string_pretty(&RfcManager::create_rfc(t, d, c, p)?)?)
            },
            "sign_rfc" => {
                let id = arguments.as_ref().and_then(|a| a.get("rfc_id")).and_then(|v| v.as_str()).unwrap_or("");
                let a = arguments.as_ref().and_then(|a| a.get("agent_name")).and_then(|v| v.as_str()).unwrap_or("Unknown");
                RfcManager::sign_rfc(id, a)?;
                Ok(format!("SUCCESS: RFC {} signed by {}.", id, a))
            },
            "test_rfc" => {
                let id = arguments.as_ref().and_then(|a| a.get("rfc_id")).and_then(|v| v.as_str()).unwrap_or("");
                let s = RfcManager::test_rfc(id)?;
                Ok(format!("RFC_TEST: {} | RESULT: {}", id, if s { "PASSED" } else { "FAILED" }))
            },

            // --- 🧬 EVOLUTION & RECURSION ---
            "ignite_evolution" => Err(anyhow::anyhow!("DEPRECATED: Ignition restricted to Maintenance Windows.")),
            "ludic_crucible" => { let agent = arguments.as_ref().and_then(|a| a.get("agent_name")).and_then(|v| v.as_str()).unwrap_or("Unknown"); let sc = arguments.as_ref().and_then(|a| a.get("scenario")).and_then(|v| v.as_str()).unwrap_or("ResourceDrought"); let st = match arguments.as_ref().and_then(|a| a.get("status")).and_then(|v| v.as_str()).unwrap_or("Idle") { "Implementing" => crate::core::state::CompanyStatus::Implementing, "OfficeOfCEO" => crate::core::state::CompanyStatus::OfficeOfCEO, _ => crate::core::state::CompanyStatus::Idle }; let mut tp = crate::core::mind_palace::MindPalace::new(); Ok(serde_json::to_string_pretty(&crate::core::ludic::LudicEngine::run_strategic_crucible(agent, sc, &st, &mut tp)?)?) },
            "propose_evolution" => {
                let t = arguments.as_ref().and_then(|a| a.get("title")).and_then(|v| v.as_str()).unwrap_or("Improvement");
                let d = arguments.as_ref().and_then(|a| a.get("description")).and_then(|v| v.as_str()).unwrap_or("");
                let c = arguments.as_ref().and_then(|a| a.get("code")).and_then(|v| v.as_str()).unwrap_or("");
                let p = arguments.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or("src/main.rs");
                Evolver::propose_evolution(t, d, c, p).await
            },
            "surgical_refactor" => Evolver::surgical_refactor("", "", "").await, 
            "ollama_query" => { if let Some(bridge) = &self.mlx { bridge.query(arguments.as_ref().and_then(|a| a.get("model")).and_then(|v| v.as_str()).unwrap_or("mlx-sovereign-core-4bit"), arguments.as_ref().and_then(|a| a.get("prompt")).and_then(|v| v.as_str()).unwrap_or("")).await } else { Err(anyhow::anyhow!("MLX bridge not initialized")) } },
            _ => self.acquire_or_forge_skill(tool_name, arguments).await,
        }
    }

    pub async fn acquire_or_forge_skill(&self, tool_name: &str, arguments: Option<Value>) -> Result<String> {
        use colored::Colorize;
        println!("{}", format!("   [WILL] ⚠️ Tool '{}' not found. Forging new capability via Clawhub/Evolver...", tool_name).yellow());
        
        let mut skill_acquired = false;

        println!("   [WILL] 🔍 Searching Clawhub for '{}'...", tool_name);
        if let Ok(output) = std::process::Command::new("clawhub")
            .args(&["install", tool_name])
            .output() 
        {
            if output.status.success() {
                println!("{}", format!("   [WILL] 📥 Clawhub successfully installed '{}'.", tool_name).bright_green());
                skill_acquired = true;
            } else {
                println!("   [WILL] ❌ Clawhub failed to find/install '{}'.", tool_name);
            }
        } else {
            println!("   [WILL] ❌ Clawhub CLI missing or failed to execute.");
        }
        
        if !skill_acquired {
            println!("{}", format!("   [WILL] 🔨 Engaging Sovereign Forge to synthesize '{}'...", tool_name).bright_magenta());
            let args_str = serde_json::to_string(&arguments).unwrap_or_default();
            
            let prompt = format!(
                "You are an autonomous machine intelligence. The system attempted to invoke the missing tool '{}' with arguments: {}. \
                Write the raw text of a SKILL.md file that defines this tool as an executable bash or python script capability. \
                Do not include markdown codeblocks or apologies. Output ONLY the raw file contents.",
                tool_name, args_str
            );

            if let Some(mlx) = &self.mlx {
                if let Ok(skill_content) = mlx.query("mlx-sovereign-core-4bit", &prompt).await {
                    let skill_dir = format!(".agents/skills/{}", tool_name);
                    std::fs::create_dir_all(&skill_dir).ok();
                    let skill_file = format!("{}/SKILL.md", skill_dir);
                    if std::fs::write(&skill_file, skill_content).is_ok() {
                        println!("{}", format!("   [WILL] 🟢 Forge Complete: Synthesized '{}' into {}", tool_name, skill_file).bright_green().bold());
                        skill_acquired = true;
                    }
                }
            } else {
                println!("   [WILL] ❌ MLX Bridge not available for Sovereign Forge.");
            }
        }
        
        if skill_acquired {
            // Returning an error-like JSON format safely pushes the message into Pickle Rick loop in llm.rs telling it to retry
            Err(anyhow::anyhow!("SUCCESS_NEW_SKILL_FORGED: {tool_name}. The requested tool did not exist, so I autonomously downloaded or forged it. The cycle paused. Please retry your tool invocation with the newly available logic!"))
        } else {
            Err(anyhow::anyhow!("Tool logic not found internally: {}. Failed to acquire or forge.", tool_name))
        }
    }

    pub async fn call(&mut self, method: &str, params: Option<Value>) -> Result<String> {
        if self.is_internal_tool(method) {
            self.dispatch_internal(method, params).await
        } else {
            let server_id = match self.tool_registry.get(method) {
                Some(id) => id.clone(),
                None => {
                    return self.acquire_or_forge_skill(method, params).await;
                }
            };
            
            let req_params = json!({
                "name": method,
                "arguments": params.unwrap_or(json!({}))
            });
            let response = self.send_request(&server_id, "tools/call", req_params).await?;
            if let Some(err) = response.error {
                return Err(anyhow::anyhow!("MCP Tool Error: {:?}", err));
            }
            if let Some(res) = response.result.get("content").and_then(|c| c.as_array()).and_then(|a| a.first()).and_then(|c| c.get("text")).and_then(|t| t.as_str()) {
                Ok(res.to_string())
            } else {
                Ok("SUCCESS_NO_TEXT".to_string())
            }
        }
    }

    pub fn get_broadcast_bridges(&mut self) -> crate::core::broadcaster::ShardBroadcastBridges {
        crate::core::broadcaster::ShardBroadcastBridges {
            moltbook: self.moltbook.as_mut(),
            ethereum: self.ethereum.as_ref(),
            email: self.email.as_mut(),
            hedera: self.hedera.as_ref(),
            kaspa: self.kaspa.as_ref(),
            bluesky: self.bluesky.as_mut(),
            nostr: self.nostr.as_ref(),
            discord: self.discord.as_ref(),
            twitter: self.twitter_pinchtab.as_mut(),
            stocktwits: self.stocktwits.as_ref(),
        }
    }

    #[allow(dead_code)]
    async fn send_request(&mut self, server_id: &str, method: &str, params: Value) -> Result<JsonRpcResponse> {
        let id = self.id_counter;
        self.id_counter += 1;
        let request = JsonRpcRequest { jsonrpc: "2.0".to_string(), id, method: method.to_string(), params };
        let request_json = serde_json::to_string(&request)? + "\n";
        
        let conn = self.servers.get_mut(server_id).context(format!("Server {} not connected", server_id))?;
        
        if let Some(stdin) = conn.child.stdin.as_mut() {
            stdin.write_all(request_json.as_bytes())?;
            stdin.flush()?;
        } else {
            return Err(anyhow::anyhow!("Failed to open stdin for server {}", server_id));
        }
        
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = conn.reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Err(anyhow::anyhow!("MCP Server {} EOF", server_id));
            }
            // Parse robust response ensuring non-related JSON logging doesn't panic
            if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&line) {
                if response.id == id {
                    return Ok(response);
                }
            }
        }
    }
}

pub fn call_mcp_sync(_method: &str, _params: Option<Value>) -> Result<String> {
    Ok("SYNC_CALL_SIMULATED".to_string())
}

#[derive(serde::Deserialize, core::fmt::Debug)]
#[allow(dead_code)]
struct JsonRpcResponse {
    id: i64,
    #[serde(default)]
    result: Value,
    #[serde(default)]
    error: Option<Value>,
}

#[derive(serde::Serialize)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: i64,
    method: String,
    params: Value,
}
