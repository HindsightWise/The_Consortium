use crate::core::escrow::{SovereignEscrow, EscrowState};
use crate::core::alpha_shard::AlphaShardGenerator;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArbiterMission {
    pub mission_id: String,
    pub escrow: EscrowState,
    pub verification_target: String,
    pub required_confidence: f32,
}

pub struct ArbiterService;

impl ArbiterService {
    /// Registers a new mission where the Company acts as the trustless judge
    pub fn register_mission(buyer: &str, target: &str, amount: u64) -> ArbiterMission {
        let mission_id = format!("MISSION_{}", hex::encode(chrono::Utc::now().timestamp().to_be_bytes()));
        let escrow = SovereignEscrow::create_escrow(buyer, &mission_id, amount);
        
        ArbiterMission {
            mission_id,
            escrow,
            verification_target: target.to_string(),
            required_confidence: 0.85,
        }
    }

    /// Automatically audits the mission's physical reality and settles the escrow
    pub async fn auto_settle_mission(&mut self, mission: &mut ArbiterMission) -> Result<String> {
        println!("⚖️  ARBITER: Auditing Mission {} for target {}...", mission.mission_id, mission.verification_target);
        
        // 1. Verify Ground Truth (Thermal/Physical)
        // We use the AlphaShardGenerator to see if the target is "Active"
        let physical = AlphaShardGenerator::generate_shard(&mission.verification_target, None, None, None, None, None, None).await?;
        
        if physical.physical_proof.confidence >= mission.required_confidence {
            println!("   ✅ Verification Successful: Confidence {:.2} meets requirement.", physical.physical_proof.confidence);
            
            // 2. Fulfill Escrow
            let proof = format!("PHYSICAL_VERIFIED_{}", physical.signature);
            let tx_sig = SovereignEscrow::fulfill(&mut mission.escrow, &proof)?;
            
            // 3. Record Tax in EconomyModule
            let tax_lamports = mission.escrow.tax_lamports;
            let tax_sol = tax_lamports as f64 / 1_000_000_000.0;
            let _ = crate::core::economy::EconomyModule::record_sale(0, crate::core::economy::ProductType::ArbiterMission);
            let _ = crate::core::economy::EconomyModule::record_sol_tax(tax_sol);
            
            Ok(format!("SETTLED | TX: {} | TAX: {:.6} SOL", tx_sig, tax_sol))
        } else {
            Err(anyhow::anyhow!("Verification Failed: Insufficient physical confidence."))
        }
    }
}
