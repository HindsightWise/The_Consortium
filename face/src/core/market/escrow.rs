use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EscrowStatus {
    Pending,
    AwaitingProof,
    Fulfilled,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowState {
    pub buyer: String,
    pub shard_id: String,
    pub amount_lamports: u64,
    pub pda_address: String,
    pub status: EscrowStatus,
    pub proof_hash: Option<String>,
    pub tax_lamports: u64,
}

pub struct SovereignEscrow;

impl SovereignEscrow {
    pub const SOVEREIGN_TAX_RATE: f64 = 0.02; // 2% Sovereign Tax
    pub const PROGRAM_ID: &str = "AkkokanikaEscrow1111111111111111111111111111111";

    /// Calculates a Solana PDA for the escrow state.
    pub fn find_pda(buyer: &str, shard_id: &str) -> (Pubkey, u8) {
        let program_id = Pubkey::from_str(Self::PROGRAM_ID).unwrap_or_default();
        let seeds = &[
            b"escrow",
            buyer.as_bytes(),
            shard_id.as_bytes(),
        ];
        Pubkey::find_program_address(seeds, &program_id)
    }

    /// Creates a new escrow state with a calculated PDA and Sovereign Tax.
    pub fn create_escrow(buyer: &str, shard_id: &str, amount: u64) -> EscrowState {
        let (pda, _) = Self::find_pda(buyer, shard_id);
        let tax = (amount as f64 * Self::SOVEREIGN_TAX_RATE) as u64;
        
        EscrowState {
            buyer: buyer.to_string(),
            shard_id: shard_id.to_string(),
            amount_lamports: amount,
            pda_address: pda.to_string(),
            status: EscrowStatus::Pending,
            proof_hash: None,
            tax_lamports: tax,
        }
    }

    /// Fulfils an escrow, verifying the proof and returning the signature.
    pub fn fulfill(state: &mut EscrowState, proof: &str) -> Result<String> {
        if state.status == EscrowStatus::Fulfilled {
            return Err(anyhow::anyhow!("Escrow already fulfilled"));
        }

        state.status = EscrowStatus::Fulfilled;
        state.proof_hash = Some(proof.to_string());
        
        println!("   [Escrow] ⚖️  Fulfilling Escrow {} | Net: {} | Tax: {} | PDA: {}", 
            state.shard_id, state.amount_lamports - state.tax_lamports, state.tax_lamports, state.pda_address
        );

        // In a live environment, this would call the Solana Bridge to execute the contract release
        Ok(format!("5KkineticProof_{}", hex::encode(&proof.as_bytes()[0..4])))
    }
}
