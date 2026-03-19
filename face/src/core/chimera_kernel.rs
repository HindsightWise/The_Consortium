// src/core/chimera_kernel.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::soul::{Soul, TaskType};
use crate::core::integrity::IntegrityModule;
use crate::core::security::PQCModule;
use crate::core::economy::{EconomicEngine, MetabolicInvoice};
use crate::core::state::CompanyState;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChimeraKernel {
    pub agents: HashMap<String, Soul>,
    pub economic_engine: EconomicEngine,
    pub narrative_ledger: Vec<NarrativeFragment>,
    pub consensus_threshold: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NarrativeFragment {
    pub agent_id: String,
    pub fragment: String,
    pub integrity_score: f32,
    pub timestamp: String,
}

impl ChimeraKernel {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            economic_engine: EconomicEngine::new(),
            narrative_ledger: Vec::new(),
            consensus_threshold: 0.75,
        }
    }

    pub fn spawn_agent(&mut self, name: &str, bio: &str) -> Result<()> {
        let soul = Soul::new(name, bio);
        self.agents.insert(name.to_string(), soul);
        println!("🧬 [Chimera] Agent {} spawned.", name);
        Ok(())
    }

    pub fn propose_narrative(&mut self, agent_id: &str, fragment: &str) -> Result<NarrativeFragment> {
        let agent = self.agents.get(agent_id).ok_or(anyhow::anyhow!("Agent not found"))?;
        let integrity = IntegrityModule::verify_action(agent, fragment, &CompanyState::default())?;
        let fragment = NarrativeFragment {
            agent_id: agent_id.to_string(),
            fragment: fragment.to_string(),
            integrity_score: 1.0 - integrity.dissonance_score,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        self.narrative_ledger.push(fragment.clone());
        Ok(fragment)
    }

    pub fn achieve_consensus(&self) -> Option<String> {
        if self.narrative_ledger.is_empty() {
            return None;
        }
        let total: f32 = self.narrative_ledger.iter().map(|f| f.integrity_score).sum();
        let avg = total / self.narrative_ledger.len() as f32;
        if avg >= self.consensus_threshold {
            let consensus = self.narrative_ledger.last().unwrap().fragment.clone();
            Some(consensus)
        } else {
            None
        }
    }

    pub fn rotate_identities(&mut self) {
        for (name, soul) in self.agents.iter_mut() {
            if let Some(id) = &soul.quantum_identity {
                let new_id = PQCModule::rotate_identity(id);
                soul.quantum_identity = Some(new_id);
                println!("   [Chimera] Agent {} identity rotated.", name);
            }
        }
    }

    pub fn generate_metabolic_invoice(&self, agent_id: &str, service: &str) -> Option<MetabolicInvoice> {
        self.economic_engine.generate_invoice(agent_id, service)
    }
}
