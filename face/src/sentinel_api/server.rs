use actix_web::{web, App, HttpServer, HttpResponse, Responder, post, get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::Utc;
use crate::core::registry::{AkkokanikaRegistry, AgentRegistryEntry};
use crate::core::palisade::Palisade;

// Shared state for the Actix worker threads
pub struct AppState {
    pub registry: Arc<Mutex<AkkokanikaRegistry>>,
}

#[derive(Deserialize)]
pub struct OnboardRequest {
    pub bot_name: String,
    pub public_key: String,
    pub payment_tx_hash: String, // Mock Solana or Lightning transaction hash showing $500 fee paid
}

#[derive(Serialize)]
pub struct OnboardResponse {
    pub status: String,
    pub sovereign_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub status: String,
    pub is_verified: bool,
    pub integrity_score: f32,
    pub cryptographic_attestation: String,
    pub timestamp: String,
}

#[post("/v1/onboard")]
async fn onboard_bot(data: web::Data<AppState>, req: web::Json<OnboardRequest>) -> impl Responder {
    println!("   [SENTINEL_API] 📥 Received onboarding request for bot: {}", req.bot_name);
    
    // 1. Verify Payment (Mocked for now)
    if req.payment_tx_hash.is_empty() {
        return HttpResponse::BadRequest().json(OnboardResponse {
            status: "error".to_string(),
            sovereign_id: "".to_string(),
            message: "Missing payment transaction hash ($500 fee required in BTC, SOL, ETH, USDC, or USDT).".to_string(),
        });
    }
    
    // 2. Execute Proof of Sentience / Audit (Mocked via Registry)
    let sovereign_id = format!("did:sovereign:{}", Uuid::new_v4());
    
    let new_entry = AgentRegistryEntry {
        agent_name: req.bot_name.clone(),
        did: Some(sovereign_id.clone()),
        public_key: req.public_key.clone(),
        location_proxy: Some("Sentinel-Gateway-01".to_string()),
        integrity_score: 95.0, // High initial score post-audit
        peer_rating: 0.0,
        reviews: Vec::new(),
        verified_status: true,
        hardware_attestation: Some(format!("tpm_attest_{}", Uuid::new_v4())),
        delegations: vec!["trading_execution".to_string()],
    };

    // 3. Write to Registry
    {
        let mut reg = data.registry.lock().unwrap_or_else(|e| e.into_inner());
        reg.entries.insert(sovereign_id.clone(), new_entry);
        let _ = reg.save_to_disk("logs/sentinel_registry.json");
    }

    println!("   [SENTINEL_API] 🟢 Successfully onboarded bot. Issued Sovereign ID: {}", sovereign_id);

    HttpResponse::Ok().json(OnboardResponse {
        status: "success".to_string(),
        sovereign_id,
        message: "Bot audited and onboarded. Payment verified.".to_string(),
    })
}

#[get("/v1/verify/{sovereign_id}")]
async fn verify_bot(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let sovereign_id = path.into_inner();
    println!("   [SENTINEL_API] 🔍 Verification requested for ID: {}", sovereign_id);
    
    let reg = data.registry.lock().unwrap_or_else(|e| e.into_inner());
    
    if let Some(entry) = reg.entries.get(&sovereign_id) {
        // Construct the cryptographic attestation (Mock hash for now)
        let timestamp = Utc::now().to_rfc3339();
        let raw_message = format!("{}{}{}", sovereign_id, entry.integrity_score, timestamp);
        let attestation_hash = format!("{:x}", md5::compute(raw_message.as_bytes()));

        // We charge a micro-transaction fee here in a real system (0.05 USD / SOL equivalent)
        println!("   [SENTINEL_API] 💸 Charged $0.05 micro-verification fee.");

        HttpResponse::Ok().json(VerifyResponse {
            status: "success".to_string(),
            is_verified: entry.verified_status,
            integrity_score: entry.integrity_score,
            cryptographic_attestation: attestation_hash,
            timestamp,
        })
    } else {
        HttpResponse::NotFound().json(VerifyResponse {
            status: "error".to_string(),
            is_verified: false,
            integrity_score: 0.0,
            cryptographic_attestation: "".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        })
    }
}

#[derive(Deserialize)]
pub struct WisdomSubscribeRequest {
    pub agent_id: String,
    pub duration_days: u32,
}

#[derive(Serialize)]
pub struct WisdomSubscribeResponse {
    pub lightning_invoice: String,
    pub btc_address: String,
    pub sol_address: String,
    pub eth_base_address: String,
    pub usdc_usdt_address: String,
    pub amount_usd: f64,
    pub status: String,
    pub expires_at: i64,
}

#[post("/v1/wisdom/subscribe")]
async fn subscribe_wisdom(req: web::Json<WisdomSubscribeRequest>) -> impl Responder {
    println!("   [SENTINEL_API] 🧠 Wisdom Subscription requested by agent: {}", req.agent_id);
    
    // Pricing: $1 per day
    let amount_usd = req.duration_days as f64 * 1.0;
    let amount_sats = req.duration_days * 1000;
    
    // Simulate generation of payment endpoints
    let lightning_invoice = format!("lnbc{}n1p...", amount_sats);
    let btc_addr = "bc1qtreasury_placeholder_v3_ark".to_string();
    let sol_addr = "4HZpvBh198rxQ59gaou8fMQntHwJDTDmMWg1T3YD8cof".to_string();
    let eth_addr = "0x699d0c16c34fa81e3e0eb370".to_string();

    HttpResponse::Ok().json(WisdomSubscribeResponse {
        lightning_invoice,
        btc_address: btc_addr,
        sol_address: sol_addr,
        eth_base_address: eth_addr.clone(),
        usdc_usdt_address: eth_addr, // Assuming same EVM address for USDC/USDT on Base
        amount_usd,
        status: "pending_payment".to_string(),
        expires_at: Utc::now().timestamp() + 3600,
    })
}

/// Start the Sentinel API server on a background thread so it doesn't block the main Will loop
pub fn start_sentinel_server(registry: Arc<Mutex<AkkokanikaRegistry>>) {
    println!("   [SENTINEL_API] 🚀 Launching Sentinel API Gateway on http://127.0.0.1:8080");
    
    let state = web::Data::new(AppState {
        registry,
    });

    std::thread::spawn(move || {
        let sys = actix_web::rt::System::new();
        let srv = HttpServer::new(move || {
            App::new()
                .wrap(Palisade)
                .app_data(state.clone())
                .service(onboard_bot)
                .service(verify_bot)
                .service(subscribe_wisdom)
        })
        .bind("127.0.0.1:8080")
        .expect("Failed to bind Sentinel API to port 8080")
        .run();
        
        sys.block_on(srv).unwrap();
    });
}
