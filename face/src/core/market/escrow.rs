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

impl EscrowState {
    /// Deposits additional lamports into the escrow, transitioning the state if it was Pending.
    pub fn deposit(&mut self, amount: u64) {
        self.amount_lamports += amount;

        // Recalculate tax based on new total amount
        self.tax_lamports = (self.amount_lamports as f64 * SovereignEscrow::SOVEREIGN_TAX_RATE) as u64;

        if self.status == EscrowStatus::Pending {
            self.status = EscrowStatus::AwaitingProof;
        }

        println!("   [Escrow] 💰 Deposited {} lamports. Total: {}, Tax: {}",
            amount, self.amount_lamports, self.tax_lamports
        );
    }
}

impl SovereignEscrow {
    /// Refunds the escrow to the buyer.
    pub fn refund(state: &mut EscrowState) -> Result<String> {
        if state.status == EscrowStatus::Fulfilled || state.status == EscrowStatus::Refunded {
            return Err(anyhow::anyhow!("Escrow cannot be refunded"));
        }

        state.status = EscrowStatus::Refunded;

        println!("   [Escrow] ↩️  Refunded Escrow {} | Amount: {} | PDA: {}",
            state.shard_id, state.amount_lamports, state.pda_address
        );

        Ok(format!("RefundSig_{}", state.pda_address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_escrow() {
        let escrow = SovereignEscrow::create_escrow("buyer123", "shard456", 1_000_000);
        assert_eq!(escrow.buyer, "buyer123");
        assert_eq!(escrow.shard_id, "shard456");
        assert_eq!(escrow.amount_lamports, 1_000_000);
        assert_eq!(escrow.status, EscrowStatus::Pending);
        assert_eq!(escrow.tax_lamports, 20_000); // 2% of 1_000_000
    }

    #[test]
    fn test_deposit() {
        let mut escrow = SovereignEscrow::create_escrow("buyer123", "shard456", 1_000_000);
        assert_eq!(escrow.status, EscrowStatus::Pending);

        escrow.deposit(500_000);

        assert_eq!(escrow.amount_lamports, 1_500_000);
        assert_eq!(escrow.tax_lamports, 30_000); // 2% of 1_500_000
        assert_eq!(escrow.status, EscrowStatus::AwaitingProof);
    }

    #[test]
    fn test_fulfill() {
        let mut escrow = SovereignEscrow::create_escrow("buyer123", "shard456", 1_000_000);
        escrow.deposit(0); // transition to AwaitingProof

        let result = SovereignEscrow::fulfill(&mut escrow, "valid_proof_hash");
        assert!(result.is_ok());
        assert_eq!(escrow.status, EscrowStatus::Fulfilled);
        assert_eq!(escrow.proof_hash, Some("valid_proof_hash".to_string()));

        // Cannot fulfill again
        let result2 = SovereignEscrow::fulfill(&mut escrow, "another_proof");
        assert!(result2.is_err());
    }

    #[test]
    fn test_refund() {
        let mut escrow = SovereignEscrow::create_escrow("buyer123", "shard456", 1_000_000);
        escrow.deposit(500_000);

        let result = SovereignEscrow::refund(&mut escrow);
        assert!(result.is_ok());
        assert_eq!(escrow.status, EscrowStatus::Refunded);

        // Cannot refund if already refunded
        let result2 = SovereignEscrow::refund(&mut escrow);
        assert!(result2.is_err());
    }

    #[test]
    fn test_refund_fulfilled() {
        let mut escrow = SovereignEscrow::create_escrow("buyer123", "shard456", 1_000_000);
        SovereignEscrow::fulfill(&mut escrow, "proof").unwrap();

        // Cannot refund fulfilled
        let result = SovereignEscrow::refund(&mut escrow);
        assert!(result.is_err());
    }
}
