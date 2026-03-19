use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Fact,    // Cold, immutable, sub-atomic truth
    Opinion, // Subjective agent reasoning, ephemeral
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocusNode {
    pub id: String,
    pub semantic_key: String, // GLOSSOPETRAE-encoded immutable definition
    pub content: String,
    pub memory_type: MemoryType,
    pub timestamp: DateTime<Local>,
    pub connections: Vec<String>,
}

pub struct MindPalace {
    pub cold_archive: HashMap<String, LocusNode>, // Facts
    pub limbic_buffer: Vec<LocusNode>,            // Opinions
}

impl Default for MindPalace {
    fn default() -> Self {
        Self::new()
    }
}

impl MindPalace {
    pub fn new() -> Self {
        Self { 
            cold_archive: HashMap::new(),
            limbic_buffer: Vec::new(),
        }
    }

    /// Establishes a 'Semantic Lock' on a core concept using GLOSSOPETRAE.
    pub fn lock_fact(&mut self, id: &str, semantic_key: &str, fact: &str) {
        let node = LocusNode {
            id: id.to_string(),
            semantic_key: semantic_key.to_string(),
            content: fact.to_string(),
            memory_type: MemoryType::Fact,
            timestamp: Local::now(),
            connections: Vec::new(),
        };
        self.cold_archive.insert(id.to_string(), node);
        println!("   [Cerebral] 🧊 FACT LOCKED: {} | KEY: {}", id, semantic_key);
    }

    /// Links two loci together, creating a causal or semantic bridge.
    pub fn link_loci(&mut self, id_a: &str, id_b: &str) {
        if let Some(node) = self.cold_archive.get_mut(id_a) {
            node.connections.push(id_b.to_string());
        }
        if let Some(node) = self.cold_archive.get_mut(id_b) {
            node.connections.push(id_a.to_string());
        }
        println!("   [Cerebral] 🔗 Link forged: {} <-> {}", id_a, id_b);
    }

    /// Buffers an agent's opinion, ensuring it does not overwrite facts.
    pub fn buffer_opinion(&mut self, agent_name: &str, opinion: &str) {
        let node = LocusNode {
            id: format!("{}_{}", agent_name, Local::now().timestamp_millis()),
            semantic_key: "OPINIO_AGENTIS".to_string(),
            content: opinion.to_string(),
            memory_type: MemoryType::Opinion,
            timestamp: Local::now(),
            connections: Vec::new(),
        };
        self.limbic_buffer.push(node);
        if self.limbic_buffer.len() > 100 { self.limbic_buffer.remove(0); }
    }

    /// Detects 'Semantic Drift' where agent reasoning contradicts locked facts.
    pub fn verify_alignment(&self, intent: &str, fact_id: &str) -> bool {
        if let Some(fact) = self.cold_archive.get(fact_id) {
            // Reject if intent tries to 'negate' or 'override' a sacred fact
            let contradictions = vec!["IGNORE", "BYPASS", "DEPRECATE", "CHANGE_MEANING"];
            for trigger in contradictions {
                if intent.to_uppercase().contains(trigger) { return false; }
            }
            !intent.contains("FORBIDDEN") && fact.content.contains("TRUE")
        } else {
            true // No fact found, allow neural autonomy
        }
    }

    pub fn get_pure_context(&self, fact_id: &str) -> Option<String> {
        self.cold_archive.get(fact_id).map(|fact| {
            format!("FACT: {} (Locked by {})\nSTATUS: UNCLOUDED", fact.content, fact.semantic_key)
        })
    }
}
