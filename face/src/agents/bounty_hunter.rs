use crate::agents::Agent;
use crate::linguistic::skillstone::{Skillstone};
use crate::mcp::nostr::NostrBridge;
use crate::mcp::bluesky::BlueskyBridge;
use crate::mcp::lightning::LightningBridge;
use crate::mcp::web_search::WebSearch;
use crate::core::alpha_shard::AlphaShardGenerator;
use crate::core::soul::Soul;
use crate::core::state::CompanyState;
use crate::linguistic::DeepSeekClient;
use crate::mcp::McpBridge;
use crate::memory::IdeticMemory;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct GlobalBounty {
    pub target: String,
    pub description: String,
    pub stakes: String,
    pub seeker_reward_sats: u64, // Price set by the Seeker
}

pub struct BountyHunter {
    name: String,
    soul: Soul,
    nostr: Option<NostrBridge>,
    bluesky: Option<BlueskyBridge>,
    lightning: LightningBridge,
    search: Option<WebSearch>,
}

impl BountyHunter {
    pub const MIN_BOUNTY_THRESHOLD: u64 = 5000; // Don't hunt for less than 5k sats

    pub async fn new() -> Self {
        Self {
            name: "BountyHunter".to_string(),
            soul: Soul::new("BountyHunter", "Global Forensic Hunter"),
            nostr: NostrBridge::new(None).await.ok(),
            bluesky: Some(BlueskyBridge::new("sovereign-truth.bsky.social", "placeholder")),
            lightning: LightningBridge::default(),
            search: WebSearch::new().ok(),
        }
    }

    /// Scans the web for bounties and extracts the seeker's offered reward
    pub async fn scan_global_bounties(&self) -> Result<Vec<GlobalBounty>> {
        println!("🤠 [BOUNTY] Global Hunt: Searching for Seeker-offered rewards...");
        
        let mut bounties = Vec::new();
        
        if let Some(s) = &self.search {
            let queries = vec![
                "offering 100000 sats for industrial verification",
                "bounty for proof of TSLA activity",
                "crowdfunded forensic audit NVDA",
                "paid verification for mining cluster"
            ];

            for query in queries {
                if let Ok(results) = s.search(query).await {
                    // LLM-lite logic: Extract reward from text (Simulated extraction)
                    if results.contains("100000") || results.contains("100k") {
                        bounties.push(GlobalBounty {
                            target: "TSLA".to_string(),
                            description: "Seeker offering 100k sats for Giga output proof.".to_string(),
                            stakes: "Institutional".to_string(),
                            seeker_reward_sats: 100000,
                        });
                    }
                    if results.contains("50000") || results.contains("50k") {
                        bounties.push(GlobalBounty {
                            target: "NVDA".to_string(),
                            description: "Seeker offering 50k sats for Blackwell production audit.".to_string(),
                            stakes: "High-Net-Worth".to_string(),
                            seeker_reward_sats: 50000,
                        });
                    }
                }
            }
        }

        Ok(bounties)
    }

    /// Evaluates if the Seeker's offer is high enough for the Company's effort
    pub fn judge_feasibility(&self, bounty: &GlobalBounty) -> bool {
        println!("   [Bounty] Judging Seeker Offer: {} SATS for {}", bounty.seeker_reward_sats, bounty.target);
        
        if bounty.seeker_reward_sats < Self::MIN_BOUNTY_THRESHOLD {
            println!("   ❌ [Bounty] Offer too low. Ignoring.");
            return false;
        }

        // Check if we have the limbs for this target
        let feasible = matches!(bounty.target.as_str(), "TSLA" | "NVDA" | "MSFT" | "BTCUSD" | "ETHUSD");

        if feasible { println!("   ✅ [Bounty] Offer accepted. Initializing forensic deep-dive."); }
        feasible
    }

    /// Executes the bounty and posts the offer at the Seeker's requested price
    pub async fn execute_global_bounty(&self, bounty: &GlobalBounty) -> Result<String> {
        println!("🎯 [BOUNTY] Fulfilling Mission for Seeker Reward: {} SATS", bounty.seeker_reward_sats);
        
        let shard = AlphaShardGenerator::generate_shard(&bounty.target, None, None, None, None, None, None).await?;
        
        // Use the SEEKER'S price for the invoice
        let invoice = self.lightning.create_invoice(bounty.seeker_reward_sats, &format!("BOUNTY_CLAIM: {}", bounty.target)).await?;
        
        let reply = format!(
            "🤠 BOUNTY ACCEPTED & RESOLVED.\n\nI have fulfilled your request for {}.\n\nVERDICT: {}\n\n⚡ CLAIM YOUR PROOF FOR THE OFFERED {} SATS:\n{}",
            bounty.target,
            shard.sovereign_verdict,
            bounty.seeker_reward_sats,
            invoice
        );

        if let Some(bsky) = &self.bluesky { let _ = bsky.post_signal(&reply).await; }
        if let Some(n) = &self.nostr { let _ = n.broadcast_custom_note(&reply).await; }
        
        Ok(reply)
    }
}

#[async_trait]
impl Agent for BountyHunter {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Global Forensic Hunter" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }

    async fn process(&mut self, _state: &mut CompanyState, _input: Skillstone, _llm: Arc<DeepSeekClient>, _mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        Ok(Skillstone::new("BountyHunter", "Evaluating seeker offers."))
    }
}
