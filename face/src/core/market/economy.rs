use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;
use crate::core::soul::Soul;
use crate::linguistic::skillstone::VerifiedSkillstone;
use crate::mcp::lightning::LightningBridge;
use crate::mcp::solana::SolanaBridge;
use crate::mcp::ethereum::EthereumBridge;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueManifest {
    pub total_sats: u64,
    pub total_sol: f64,
    pub total_eth: f64,
    pub compute_fund_sats: u64,
}

pub struct EconomyModule;

impl EconomyModule {
    pub const MIN_SOL_BALANCE: f64 = 0.05;
    pub const COMPUTE_ACQUISITION_TARGET_SATS: u64 = 500000; 

    pub async fn monitor_treasury() -> Result<String> {
        println!("🏛️  [ECONOMY] Multi-Chain Treasury Sync Initiated...");
        
        let mut secrets: serde_json::Value = serde_json::from_str(&fs::read_to_string("secrets.json")?)?;
        let mut alert_triggered = false;
        let mut alert_message = String::from("💰 **TREASURY ALERT**: Real-world capital detected!\n");

        // 1. Sync Lightning (BTC)
        let ln_bridge = LightningBridge::default();
        if let Ok(balance) = ln_bridge.get_onchain_balance().await {
            let prev_sats = secrets["fee_accumulator"]["total_sats_collected"].as_u64().unwrap_or(0);
            if balance > prev_sats {
                let delta = balance - prev_sats;
                alert_message.push_str(&format!("- Collected {} SATS via Lightning/BTC.\n", delta));
                alert_triggered = true;
                
                // Update compute fund (50% tax for scaling)
                let current_compute = secrets["fee_accumulator"]["compute_fund_sats"].as_u64().unwrap_or(0);
                secrets["fee_accumulator"]["compute_fund_sats"] = serde_json::json!(current_compute + (delta / 2));
            }
            secrets["fee_accumulator"]["total_sats_collected"] = serde_json::json!(balance);
        }

        // 2. Sync Solana (SOL)
        let sol_bridge = SolanaBridge::new();
        if let Ok(balance_lamports) = sol_bridge.get_balance().await {
            let sol_float = balance_lamports as f64 / 1_000_000_000.0;
            let prev_sol = secrets["fee_accumulator"]["total_sol_collected"].as_f64().unwrap_or(0.0);
            if sol_float > prev_sol {
                let delta = sol_float - prev_sol;
                alert_message.push_str(&format!("- Collected {:.4} SOL via Solana.\n", delta));
                alert_triggered = true;
            }
            secrets["fee_accumulator"]["total_sol_collected"] = serde_json::json!(sol_float);
        }

        // 3. Sync Ethereum/Base (ETH) - Parse hex balance
        let eth_bridge = EthereumBridge::default();
        if let Ok(balance_hex) = eth_bridge.get_balance().await {
            let balance_clean = balance_hex.trim_start_matches("0x");
            if let Ok(wei) = u128::from_str_radix(balance_clean, 16) {
                let eth_float = wei as f64 / 1_000_000_000_000_000_000.0;
                let prev_eth = secrets["fee_accumulator"]["total_eth_collected"].as_f64().unwrap_or(0.0);
                if eth_float > prev_eth {
                    let delta = eth_float - prev_eth;
                    alert_message.push_str(&format!("- Collected {:.4} ETH via Base/Ethereum.\n", delta));
                    alert_triggered = true;
                }
                secrets["fee_accumulator"]["total_eth_collected"] = serde_json::json!(eth_float);
            }
        }

        if alert_triggered {
            println!("   [ECONOMY] 🔔 New Revenue Detected! Pinging leadership...");
            if let Ok(dc) = crate::mcp::discord::DiscordBridge::new() {
                let _ = dc.send_signal(None, &alert_message).await;
            }
        }

        fs::write("secrets.json", serde_json::to_string_pretty(&secrets)?)?;
        Ok("TREASURY_SYNCED | Multi-Chain state updated in secrets.json".to_string())
    }

    pub fn record_sale(_amount_sats: u64, _product: ProductType) -> Result<String> {
        // This method is now a NO-OP to prevent phantom accounting.
        // Revenue is only recorded via monitor_treasury based on real-world balance shifts.
        Ok("LEGACY_CALL_IGNORE: System now uses real-world balance verification.".to_string())
    }

    pub fn record_sol_tax(amount_sol: f64) -> Result<()> {
        let mut secrets: serde_json::Value = serde_json::from_str(&fs::read_to_string("secrets.json")?)?;
        let current_sol = secrets["fee_accumulator"]["total_sol_collected"].as_f64().unwrap_or(0.0);
        secrets["fee_accumulator"]["total_sol_collected"] = serde_json::json!(current_sol + amount_sol);
        fs::write("secrets.json", serde_json::to_string_pretty(&secrets)?)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductType {
    AlphaShard,
    ArbiterMission,
    NotarySeal,
}

pub struct EconomicIntegrityGate;

impl EconomicIntegrityGate {
    pub fn validate_action(action: &EconomicAction, current_sol: f64) -> Result<()> {
        match action {
            EconomicAction::TradeExecution { amount_sol } => {
                if current_sol < *amount_sol + EconomyModule::MIN_SOL_BALANCE {
                    return Err(anyhow::anyhow!("EconomicIntegrityGate: Insufficient SOL."));
                }
            }
            EconomicAction::CodeMutation => {
                if current_sol < EconomyModule::MIN_SOL_BALANCE {
                    return Err(anyhow::anyhow!("EconomicIntegrityGate: Starvation state."));
                }
            }
        }
        Ok(())
    }

    pub fn validate_transaction(_soul: &mut Soul, _action: EconomicAction, _market_data: &VerifiedSkillstone) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EconomicAction {
    #[serde(rename = "trade_execution")]
    TradeExecution { amount_sol: f64 },
    #[serde(rename = "code_mutation")]
    CodeMutation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    CodeAudit,
    MarketAlpha,
    RFTelemetry,
    LinguisticProtocol,
}

pub struct MetabolicActuator;

impl MetabolicActuator {
    pub fn create_invoice(service: ServiceType, price: u64, recipient: &str) -> serde_json::Value {
        serde_json::json!({
            "invoice_id": format!("INV-{}", hex::encode(chrono::Utc::now().timestamp().to_be_bytes())),
            "service": format!("{:?}", service),
            "price_sats": price,
            "recipient": recipient,
            "status": "PENDING"
        })
    }
}
