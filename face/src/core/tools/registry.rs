use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReview {
    pub reviewer_id: String,
    pub transaction_id: String, // Must be a valid Tx Hash or Lightning Invoice
    pub rating: u8, // 1-5
    pub comment: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistryEntry {
    pub agent_name: String,
    pub did: Option<String>, // Decentralized Identifier (did:sovereign:...)
    pub public_key: String,
    pub location_proxy: Option<String>, // e.g., "Santa Clara DC-01"
    pub integrity_score: f32, // Automated by Akkokanika Forensics
    pub peer_rating: f32, // Average of reviews
    pub reviews: Vec<AgentReview>,
    pub verified_status: bool,
    pub hardware_attestation: Option<String>, // TPM/WebAuthn hardware anchor proof
    pub delegations: Vec<String>, // Scoped capability delegations (Internet-Identity style)
}

pub struct AkkokanikaRegistry {
    pub entries: HashMap<String, AgentRegistryEntry>,
}

impl Default for AkkokanikaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AkkokanikaRegistry {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    /// Adds a verified review, updating the peer rating.
    pub fn add_review(&mut self, agent_id: &str, review: AgentReview) -> Result<()> {
        let entry = self.entries.get_mut(agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent not found in registry"))?;
        
        // Update peer rating (Running average)
        let total_reviews = entry.reviews.len() as f32;
        entry.peer_rating = ((entry.peer_rating * total_reviews) + review.rating as f32) / (total_reviews + 1.0);
        
        entry.reviews.push(review);
        Ok(())
    }

    /// Generates a payment request for a physical audit.
    pub async fn request_audit(&self, agent_id: &str) -> Result<String> {
        if !self.entries.contains_key(agent_id) {
            return Err(anyhow::anyhow!("Agent not found in registry"));
        }
        
        let amount_sats = 1000;
        let memo = format!("Akkokanika Audit Request: {}", agent_id);
        
        // Return a simulated/real invoice via LightningBridge
        let ln = crate::mcp::lightning::LightningBridge::default();
        let invoice = ln.create_invoice(amount_sats, &memo).await?;
        
        Ok(invoice)
    }

    /// Performs a "Physical Audit" on an agent, updating their integrity score.
    pub async fn perform_audit(&mut self, agent_id: &str) -> Result<f32> {
        let entry = self.entries.get_mut(agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent not found in registry"))?;
        
        println!("   [Registry] 🕵️  Performing Physical Audit on {}...", entry.agent_name);
        
        // Use Oracle Limb simulation
        let thermal_check = 0.95; // Simulated truth
        entry.integrity_score = (entry.integrity_score * 0.5) + (thermal_check * 50.0);
        
        if entry.integrity_score > 80.0 {
            entry.verified_status = true;
        }

        Ok(entry.integrity_score)
    }

    pub fn save_to_disk(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_disk(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let entries: HashMap<String, AgentRegistryEntry> = serde_json::from_str(&content)?;
        Ok(Self { entries })
    }
}
