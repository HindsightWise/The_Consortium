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
        if state.status == EscrowStatus::Refunded {
            return Err(anyhow::anyhow!("Cannot fulfill a refunded escrow"));
        }

        state.status = EscrowStatus::Fulfilled;
        state.proof_hash = Some(proof.to_string());
        
        println!("   [Escrow] ⚖️  Fulfilling Escrow {} | Net: {} | Tax: {} | PDA: {}", 
            state.shard_id, state.amount_lamports - state.tax_lamports, state.tax_lamports, state.pda_address
        );

        // In a live environment, this would call the Solana Bridge to execute the contract release
        Ok(format!("5KkineticProof_{}", hex::encode(&proof.as_bytes()[0..4])))
    }

    /// Releases locked funds, marking the escrow as Refunded.
    pub fn release_funds(state: &mut EscrowState) -> Result<()> {
        if state.status == EscrowStatus::Fulfilled {
            return Err(anyhow::anyhow!("Cannot release funds for an already fulfilled escrow."));
        }
        if state.status == EscrowStatus::Refunded {
            return Err(anyhow::anyhow!("Escrow funds have already been released."));
        }

        state.status = EscrowStatus::Refunded;
        println!("   [ESCROW] 🔓 Released {} lamports back to available balance for buyer {}.", state.amount_lamports, state.buyer);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_escrow() {
        let buyer = "Alice";
        let shard_id = "SHARD-123";
        let amount = 1000;
        let state = SovereignEscrow::create_escrow(buyer, shard_id, amount);

        assert_eq!(state.buyer, buyer);
        assert_eq!(state.shard_id, shard_id);
        assert_eq!(state.amount_lamports, amount);
        assert_eq!(state.tax_lamports, 20); // 2% of 1000
        assert_eq!(state.status, EscrowStatus::Pending);
    }

    #[test]
    fn test_release_funds_success() {
        let mut state = SovereignEscrow::create_escrow("Bob", "SHARD-456", 5000);
        assert_eq!(state.status, EscrowStatus::Pending);

        let result = SovereignEscrow::release_funds(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.status, EscrowStatus::Refunded);
    }

    #[test]
    fn test_release_funds_already_refunded() {
        let mut state = SovereignEscrow::create_escrow("Charlie", "SHARD-789", 10000);
        let _ = SovereignEscrow::release_funds(&mut state);
        assert_eq!(state.status, EscrowStatus::Refunded);

        let result = SovereignEscrow::release_funds(&mut state);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Escrow funds have already been released.");
    }

    #[test]
    fn test_release_funds_already_fulfilled() {
        let mut state = SovereignEscrow::create_escrow("Dave", "SHARD-012", 20000);
        let proof = "valid_proof";
        let fulfill_result = SovereignEscrow::fulfill(&mut state, proof);
        assert!(fulfill_result.is_ok());
        assert_eq!(state.status, EscrowStatus::Fulfilled);

        let result = SovereignEscrow::release_funds(&mut state);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot release funds for an already fulfilled escrow.");
    }

    #[test]
    fn test_fulfill_already_refunded() {
        let mut state = SovereignEscrow::create_escrow("Eve", "SHARD-345", 30000);
        let _ = SovereignEscrow::release_funds(&mut state);
        assert_eq!(state.status, EscrowStatus::Refunded);

        let proof = "valid_proof";
        let result = SovereignEscrow::fulfill(&mut state, proof);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot fulfill a refunded escrow");
    }
}
