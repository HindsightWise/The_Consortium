pub mod chronomaster;
pub mod hindsight;

use anyhow::{Result, anyhow};
use crate::core::soul::Soul;
use crate::linguistic::skillstone::MintedSkill;
use serde::{Deserialize, Serialize};

use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct MintRequest {
    pub skill_name: String,
    pub instructions: String,
    pub price: u64,
    pub nonce: u64, // The "Proof of Work"
}

pub struct Marketplace;

impl Marketplace {
    pub fn verify_work(name: &str, nonce: u64) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", name, nonce));
        let result = hasher.finalize();
        let hash_str = format!("{:x}", result);
        
        // Difficulty: Hash must start with "00" (Simple PoW)
        hash_str.starts_with("00")
    }

    pub fn mint_skill(soul: &Soul, request: &MintRequest) -> Result<MintedSkill> {
        if !Self::verify_work(&request.skill_name, request.nonce) {
            return Err(anyhow!("Proof of Work verification FAILED: Nonce is invalid!"));
        }

        println!("[Marketplace] {} successfully mined and minted skill: {}", soul.name, request.skill_name);
        
        // Sovereign Verification: Simulating a cryptographic signature
        let signature = format!("SIG_{}_{}", soul.name, rand::random::<u32>());

        Ok(MintedSkill {
            name: request.skill_name.clone(),
            instruction_set: request.instructions.clone(),
            author: soul.name.clone(),
            level_required: soul.level,
            price: request.price,
            signature,
        })
    }

    pub fn acquire_skill(soul: &mut Soul, skill: &MintedSkill) -> Result<()> {
        if soul.reputation_tokens < skill.price {
            return Err(anyhow!("Insufficient reputation tokens to acquire skill: {}", skill.name));
        }

        // Verify Signature
        if !skill.signature.starts_with("SIG_") {
            return Err(anyhow!("Sovereign Verification FAILED: Malicious Skill detected!"));
        }

        println!("[Marketplace] {} successfully acquired skill: {}", soul.name, skill.name);
        soul.reputation_tokens -= skill.price;
        soul.skill_inventory.push(skill.name.clone());
        
        Ok(())
    }
}
