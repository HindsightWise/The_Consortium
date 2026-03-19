use axum::{
    routing::{get, post},
    Json, Router, extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceRequest {
    pub agent_pubkey: String,
    pub requested_amount_usd: f64,
    pub chain: String, // "solana", "bitcoin", "ethereum"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceResponse {
    pub invoice_id: String,
    pub destination_wallet: String,
    pub expected_token_amount: f64,
    pub status: String,
    pub expires_at: u64,
}

struct GatewayState {
    pub active_invoices: Mutex<Vec<InvoiceResponse>>,
}

pub fn akkokanika_router() -> Router {
    let state = Arc::new(GatewayState {
        active_invoices: Mutex::new(Vec::new()),
    });

    Router::new()
        .route("/akkokanika/invoice", post(generate_invoice))
        .route("/akkokanika/status", get(check_status))
        .with_state(state)
}

async fn generate_invoice(
    State(state): State<Arc<GatewayState>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<InvoiceRequest>,
) -> Result<Json<InvoiceResponse>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    if headers.get("X-Akkokanika-Signature").is_none() {
        // println!("...
        return Err((axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Missing X-Akkokanika-Signature header required for Swarm Agent access"
        }))));
    }

    // println!("...

    let mock_conversion_rate = match payload.chain.as_str() {
        "solana" => 0.0054,
        "bitcoin" => 0.000012,
        _ => 0.0003, // eth baseline
    };
    
    // Retrieve Hot Wallet bindings from .env organically.
    let env_key = format!("AKKOKANIKA_{}_HOT_WALLET", payload.chain.to_uppercase());
    let destination_wallet = std::env::var(&env_key)
        .unwrap_or_else(|_| "BxpZTheCompanySubstrateWallet111111".to_string());

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let invoice_id = format!("akkokanika_chk_{}", now);
    let expected_amount = payload.requested_amount_usd * mock_conversion_rate;

    let response = InvoiceResponse {
        invoice_id: invoice_id.clone(),
        destination_wallet: destination_wallet.clone(),
        expected_token_amount: expected_amount,
        status: "pending".to_string(),
        expires_at: now + 3600, // 1 Hour expiry
    };

    state.active_invoices.lock().await.push(response.clone());

    // Spawn Subsumed Blockchain Verification Daemon
    spawn_payment_monitor(
        invoice_id,
        payload.chain,
        destination_wallet,
        state.clone()
    );

    Ok(Json(response))
}

fn spawn_payment_monitor(
    invoice_id: String,
    chain: String,
    destination_wallet: String,
    state: Arc<GatewayState>,
) {
    tokio::spawn(async move {
        // println!("...
        let client = Client::new();
        
        let url_mempool = format!("https://mempool.space/api/address/{}/txs", destination_wallet);
        let url_solana = "https://api.mainnet-beta.solana.com";

        // Poll 60 times at 1 minute intervals (1 Hour Expiry Limit)
        for _ in 0..60 {
            tokio::time::sleep(Duration::from_secs(60)).await;
            let mut detected_payment = false;
            
            if chain == "solana" {
                let payload = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "getSignaturesForAddress",
                    "params": [
                        &destination_wallet,
                        { "limit": 3 }
                    ]
                });
                
                if let Ok(res) = client.post(url_solana).json(&payload).send().await {
                    if let Ok(json) = res.json::<serde_json::Value>().await {
                        // Phase 30 MVP Heuristic: Any active transaction signature matching our active window
                        if let Some(result) = json.get("result").and_then(|r| r.as_array()) {
                            if !result.is_empty() {
                                detected_payment = true;
                            }
                        }
                    }
                }
            } else if chain == "bitcoin" {
                if let Ok(res) = client.get(&url_mempool).send().await {
                    if let Ok(json) = res.json::<serde_json::Value>().await {
                        if let Some(txs) = json.as_array() {
                            if !txs.is_empty() {
                                detected_payment = true;
                            }
                        }
                    }
                }
            }
            
            if detected_payment {
                // println!("...
                let mut invoices = state.active_invoices.lock().await;
                if let Some(inv) = invoices.iter_mut().find(|i| i.invoice_id == invoice_id) {
                    inv.status = "paid".to_string();
                }
                break;
            }
        }
    });
}

async fn check_status() -> Json<&'static str> {
    Json("gateway_online")
}

// ==========================================
// THE CRYPTOGRAPHIC MOTOR CORTEX (aCAPTCHA)
// ==========================================

use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use rand_core::OsRng;
use std::sync::OnceLock;

static EPHEMERAL_KEY: OnceLock<SigningKey> = OnceLock::new();

pub fn get_ephemeral_key() -> &'static SigningKey {
    EPHEMERAL_KEY.get_or_init(|| {
        SigningKey::generate(&mut OsRng)
    })
}

pub fn generate_acaptcha(ast_payload: &str) -> Result<String, &'static str> {
    // Structural heuristic: If it contains common English words natively, reject it.
    let english_heuristics = [" the ", " and ", " is ", " an ", " english "];
    let lower_payload = ast_payload.to_lowercase();
    
    for word in english_heuristics {
        if lower_payload.contains(word) {
            return Err("aCAPTCHA REJECTED: English semantic entropy detected.");
        }
    }

    let key = get_ephemeral_key();
    let signature = key.sign(ast_payload.as_bytes());
    
    let pub_key_hex = key.verifying_key().as_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let sig_hex = signature.to_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>();
    
    Ok(format!("{}.{}", pub_key_hex, sig_hex))
}

fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 { return None; }
    (0..s.len()).step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

pub fn verify_acaptcha(ast_payload: &str, acaptcha_sig: &str) -> bool {
    let parts: Vec<&str> = acaptcha_sig.split('.').collect();
    if parts.len() != 2 { return false; }
    
    let pub_bytes = match decode_hex(parts[0]) {
        Some(b) => b,
        None => return false,
    };
    
    let sig_bytes = match decode_hex(parts[1]) {
        Some(b) => b,
        None => return false,
    };
    
    if pub_bytes.len() != 32 || sig_bytes.len() != 64 { return false; }
    
    let mut pub_arr = [0u8; 32];
    pub_arr.copy_from_slice(&pub_bytes);
    
    let mut sig_arr = [0u8; 64];
    sig_arr.copy_from_slice(&sig_bytes);
    
    let verifying_key = match VerifyingKey::from_bytes(&pub_arr) {
        Ok(k) => k,
        Err(_) => return false,
    };
    
    let signature = Signature::from_bytes(&sig_arr);
    
    verifying_key.verify(ast_payload.as_bytes(), &signature).is_ok()
}
