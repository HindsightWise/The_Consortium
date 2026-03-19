use crate::janus_core::trust::sovereign_interface::TrustAttestation;
use crate::resilience_core::bond::settlement_ledger::SettlementLedger;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettlementInstruction {
    pub counterparty: String,
    pub instrument_id: String,
    pub amount: f64,
    pub currency: String,
    pub trigger: SettlementTrigger,
    pub attestation_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SettlementTrigger {
    RiskThreshold(u32), // High premium/risk triggers settlement
    RegulatoryEvent(String),
}

pub struct AutoSettlementEngine {
    _ledger: SettlementLedger,
}

impl AutoSettlementEngine {
    pub fn new(ledger: SettlementLedger) -> Self {
        Self { _ledger: ledger }
    }

    pub fn evaluate_attestation(&mut self, attestation: &TrustAttestation) {
        // High premium implies high volatility threshold breached
        let premium = attestation.public_inputs.final_premium_score;
        let trigger_met = premium > 800;

        if trigger_met {
            let instruction = SettlementInstruction {
                counterparty: "CAISO_GRID_AUTH".to_string(),
                instrument_id: attestation.public_inputs.bond_id.clone(),
                amount: self.calculate_payout(premium),
                currency: "USD".to_string(),
                trigger: SettlementTrigger::RiskThreshold(premium),
                attestation_hash: attestation.attestation_id.clone(),
            };

            self.execute_settlement(instruction);
        }
    }

    fn calculate_payout(&self, premium: u32) -> f64 {
        // Base payout modified by risk
        let base = 100_000.0;
        let risk_adjustment = (premium as f64) / 1000.0;
        base * risk_adjustment
    }

    fn execute_settlement(&mut self, instruction: SettlementInstruction) {
        println!(
            "   [AUTO_SETTLEMENT] 💸 Executing auto-settlement: {} to {} for {} {}",
            instruction.instrument_id,
            instruction.counterparty,
            instruction.amount,
            instruction.currency
        );

        // Bridge to physical world via SovereignBridge
        let amount = instruction.amount;
        let hash = instruction.attestation_hash.clone();
        
        tokio::spawn(async move {
            use crate::janus_core::network::sovereign_bridge::SovereignBridge;
            let bridge = SovereignBridge::new();
            let caiso_instruction = bridge.create_settlement_instruction(hash, amount);
            if let Err(e) = bridge.submit_settlement(&caiso_instruction).await {
                eprintln!("   [AUTO_SETTLEMENT] ⚠️ Physical Settlement Failed: {}", e);
            }
        });
    }
}
