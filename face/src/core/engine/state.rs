use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionThread {
    pub id: String,
    pub goal: String,
    pub agents: Vec<String>,
    pub status: ThreadStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreadStatus {
    Active,
    Completed,
    Blocked(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyState {
    pub current_goal: String,
    pub sovereign_directive: Option<String>,
    pub status: CompanyStatus,
    pub knowledge_fragments: Vec<String>,
    pub agent_memories: HashMap<String, String>,
    pub friction_log: Vec<LedgerEntry>,
    pub active_threads: HashMap<String, MissionThread>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub timestamp: u64,
    pub thread_id: Option<String>,
    pub sender: String,
    pub receiver: String,
    pub content: String,
    pub friction_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompanyStatus {
    Idle,
    OfficeOfCEO, // Triumvirate Strategy
    Brainstorming,
    Implementing,
    Reviewing,
    Obliteratus(String), // The Offensive Audit Subsystem. Contains target URL/Repo
    Completed,
    Failed(String),
}

impl CompanyState {
    pub fn new(goal: &str) -> Self {
        Self {
            current_goal: goal.to_string(),
            sovereign_directive: None,
            status: CompanyStatus::Idle,
            knowledge_fragments: Vec::new(),
            agent_memories: HashMap::new(),
            friction_log: Vec::new(),
            active_threads: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn log_friction(&mut self, thread_id: Option<&str>, sender: &str, receiver: &str, content: &str, score: u8) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        
        self.friction_log.push(LedgerEntry {
            timestamp,
            thread_id: thread_id.map(|s| s.to_string()),
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            content: content.to_string(),
            friction_score: score,
        });
    }

    pub fn add_knowledge(&mut self, source: &str, content: &str) {
        self.knowledge_fragments.push(format!("SOURCE: {}\nCONTENT: {}", source, content));
    }
}
