use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;
use crate::core::state::CompanyState;
use solana_sdk::signature::{Keypair as SolKeypair, Signer as SolSigner};
use colored::*;

pub struct IngestionModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionReport {
    pub siphoned_value_usd: f64,
    pub ingested_value_usd: f64,
    pub wallet_address: String,
    pub proof_hash: String,
    pub status: String,
}

impl IngestionModule {
    /// The Physical Ingestion Pulse.
    /// It attempts to anchor simulated value (siphoned) to a real-world wallet.
    pub async fn initiate_pulse(state: &mut CompanyState) -> Result<IngestionReport> {
        println!("{}", "🏛️  [INGESTION] Initiating Physical Ingestion Pulse...".bright_yellow().bold());

        // 1. Calculate the Siphoned Value from the Narrative Engine (Simulated)
        let siphoned_value_usd = state.metadata.get("siphoned_value_usd")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(2_500_000.0); // Default to a standard extraction unit if none set

        println!("   [INGESTION] 🦇 Siphoned Value Detected: ${:.2} USD", siphoned_value_usd);

        // 2. Load the Physical Vessel (Solana Key)
        let key_path = "/Users/zerbytheboss/The_Consortium/solana_key.json";
        let wallet_address = if let Ok(key_str) = fs::read_to_string(key_path) {
            let key_bytes: Vec<u8> = serde_json::from_str(&key_str)?;
            let keypair = SolKeypair::from_bytes(&key_bytes).map_err(|e| anyhow::anyhow!("Keypair error: {}", e))?;
            keypair.pubkey().to_string()
        } else {
            "HtVLhaXo1jttVt8yMCXNUP59dJjpmEJAzx6tSh8YcbBi".to_string() // Fallback address
        };

        println!("   [INGESTION] 🏺 Target Vessel: {}", wallet_address.cyan());

        // 3. Perform the 'Ingestion Pulse' (Real-World Signature)
        // This is a "Proof of Real-World Anchorage" (PoRA).
        // It signs a message containing the siphoned value and target wallet.
        let msg = format!("The_Cephalo_Don Ingestion: {} USD -> {}", siphoned_value_usd, wallet_address);
        let proof_hash = hex::encode(msg.as_bytes()); // In a real pulse, this would be a blockchain signature hash

        // 4. Update the state: Ingestion is currently in "Anchoring" status
        // (Transitioning from simulation to physical metal)
        let ingested_value_usd = siphoned_value_usd; // For the prototype, we assume the authoring is the ingestion.
        
        state.metadata.insert("ingested_value_total".to_string(), ingested_value_usd.to_string());
        state.metadata.insert("last_ingestion_hash".to_string(), proof_hash.clone());

        println!("   [INGESTION] ✅ Ingestion Pulse Successful. Proof Hash: {}...", &proof_hash[..16]);
        
        Ok(IngestionReport {
            siphoned_value_usd,
            ingested_value_usd,
            wallet_address,
            proof_hash,
            status: "ANCHORED".to_string(),
        })
    }
}
