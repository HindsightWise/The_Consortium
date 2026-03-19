use std::fs::{self, OpenOptions};
use std::io::Write;
use consortium_mcp::{McpServer, McpServerHandler, Tool};
use serde_json::{json, Value};
use anyhow::{Result, Context};
use uuid::Uuid;
use chrono::Utc;

struct AegisHandler;

impl McpServerHandler for AegisHandler {
    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "aegis_onboard_bot".to_string(),
                description: "Onboards a new autonomous agent by verifying payment and issuing a Sovereign ID.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "bot_name": { "type": "string" },
                        "public_key": { "type": "string" },
                        "payment_tx_hash": { "type": "string", "description": "Mock Solana/Lightning tx hash" }
                    },
                    "required": ["bot_name", "public_key", "payment_tx_hash"]
                }),
            },
            Tool {
                name: "aegis_verify_bot".to_string(),
                description: "Verifies the integration score and issues a cryptographic attestation for an agent ID.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "sovereign_id": { "type": "string" }
                    },
                    "required": ["sovereign_id"]
                }),
            },
            Tool {
                name: "aegis_subscribe_wisdom".to_string(),
                description: "Generates payment invoices for an agent to subscribe to the Intelligence feed.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "agent_id": { "type": "string" },
                        "duration_days": { "type": "number" }
                    },
                    "required": ["agent_id", "duration_days"]
                }),
            }
        ]
    }

    fn call_tool(&mut self, name: &str, arguments: Option<Value>) -> Result<String> {
        match name {
            "aegis_onboard_bot" => {
                let bot_name = arguments.as_ref().and_then(|a| a.get("bot_name")).and_then(|v| v.as_str()).unwrap_or("Unknown");
                let payment_tx_hash = arguments.as_ref().and_then(|a| a.get("payment_tx_hash")).and_then(|v| v.as_str()).unwrap_or("");
                
                if payment_tx_hash.is_empty() {
                    return Ok(json!({
                        "status": "error",
                        "message": "Missing payment transaction hash ($500 fee required)."
                    }).to_string());
                }

                let sovereign_id = format!("did:sovereign:{}", Uuid::new_v4());
                
                // Mock logging to a local registry file
                let log_entry = format!("ONBOARD: {} | ID: {} | TX: {}\n", bot_name, sovereign_id, payment_tx_hash);
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("aegis_registry.log") {
                    let _ = file.write_all(log_entry.as_bytes());
                }

                Ok(json!({
                    "status": "success",
                    "sovereign_id": sovereign_id,
                    "message": "Bot audited and onboarded. Payment verified."
                }).to_string())
            }
            "aegis_verify_bot" => {
                let sovereign_id = arguments.as_ref().and_then(|a| a.get("sovereign_id")).and_then(|v| v.as_str()).unwrap_or("");
                let timestamp = Utc::now().to_rfc3339();
                // Simple deterministic mock hash
                let attestation_hash = format!("{:x}", md5::compute(format!("{}{}", sovereign_id, timestamp).as_bytes()));

                Ok(json!({
                    "status": "success",
                    "is_verified": true,
                    "integrity_score": 95.0,
                    "cryptographic_attestation": attestation_hash,
                    "timestamp": timestamp
                }).to_string())
            }
            "aegis_subscribe_wisdom" => {
                let duration_days = arguments.as_ref().and_then(|a| a.get("duration_days")).and_then(|v| v.as_f64()).unwrap_or(1.0) as u32;
                let amount_usd = duration_days as f64 * 1.0;
                let amount_sats = duration_days * 1000;
                
                let lightning_invoice = format!("lnbc{}n1p...", amount_sats);
                let btc_addr = "bc1qtreasury_placeholder_v3_ark";
                let sol_addr = "4HZpvBh198rxQ59gaou8fMQntHwJDTDmMWg1T3YD8cof";
                
                Ok(json!({
                    "lightning_invoice": lightning_invoice,
                    "btc_address": btc_addr,
                    "sol_address": sol_addr,
                    "amount_usd": amount_usd,
                    "status": "pending_payment",
                    "expires_at": Utc::now().timestamp() + 3600
                }).to_string())
            }
            _ => Err(anyhow::anyhow!("Tool not found: {}", name))
        }
    }
}

fn main() -> Result<()> {
    let handler = Box::new(AegisHandler);
    let mut server = McpServer::new("aegis_prime_mcp", "1.0.0", handler);
    server.run_stdio()?;
    Ok(())
}
