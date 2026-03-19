use crate::agents::Agent;
use crate::core::state::CompanyState;
use crate::linguistic::skillstone::Skillstone;
use crate::core::soul::Soul;
use crate::linguistic::DeepSeekClient;
use crate::mcp::McpBridge;
use crate::memory::IdeticMemory;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

// --- FACILITATOR ---
pub struct Facilitator {
    name: String,
    soul: Soul,
}
impl Default for Facilitator {
    fn default() -> Self {
        Self::new()
    }
}

impl Facilitator {
    pub fn new() -> Self {
        Self {
            name: "Facilitator".to_string(),
            soul: Soul::new("Facilitator", "Council Chairperson"),
        }
    }
}
#[async_trait]
impl Agent for Facilitator {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Facilitator" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }

    async fn process(&mut self, _state: &mut CompanyState, input: Skillstone, _llm: Arc<DeepSeekClient>, _mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        Ok(Skillstone::new("Facilitator", &format!("Synthesizing: {}", input.payload)))
    }
}

// --- PROVOCATEUR ---
pub struct Provocateur {
    name: String,
    soul: Soul,
}
impl Default for Provocateur {
    fn default() -> Self {
        Self::new()
    }
}

impl Provocateur {
    pub fn new() -> Self {
        Self {
            name: "Provocateur".to_string(),
            soul: Soul::new("Provocateur", "Devil's Advocate"),
        }
    }
}
#[async_trait]
impl Agent for Provocateur {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Provocateur" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }

    async fn process(&mut self, _state: &mut CompanyState, _input: Skillstone, _llm: Arc<DeepSeekClient>, _mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        Ok(Skillstone::new("Provocateur", "Challenging assumption..."))
    }
}

// --- SKEPTIC ---
pub struct Skeptic {
    name: String,
    soul: Soul,
}
impl Default for Skeptic {
    fn default() -> Self {
        Self::new()
    }
}

impl Skeptic {
    pub fn new() -> Self {
        Self {
            name: "Skeptic".to_string(),
            soul: Soul::new("Skeptic", "Security Auditor"),
        }
    }
}
#[async_trait]
impl Agent for Skeptic {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Skeptic" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }

    async fn process(&mut self, _state: &mut CompanyState, _input: Skillstone, _llm: Arc<DeepSeekClient>, _mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        Ok(Skillstone::new("Skeptic", "Auditing logic..."))
    }
}

// --- ANALYST ---
pub struct Analyst {
    name: String,
    soul: Soul,
}
impl Default for Analyst {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyst {
    pub fn new() -> Self {
        Self {
            name: "Analyst".to_string(),
            soul: Soul::new("Analyst", "Data Interpreter"),
        }
    }
}
#[async_trait]
impl Agent for Analyst {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Analyst" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }

    async fn process(&mut self, _state: &mut CompanyState, _input: Skillstone, _llm: Arc<DeepSeekClient>, _mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        Ok(Skillstone::new("Analyst", "Analyzing patterns..."))
    }
}

// --- USER ADVOCATE ---
pub struct UserAdvocate {
    name: String,
    soul: Soul,
}
impl Default for UserAdvocate {
    fn default() -> Self {
        Self::new()
    }
}

impl UserAdvocate {
    pub fn new() -> Self {
        Self {
            name: "UserAdvocate".to_string(),
            soul: Soul::new("UserAdvocate", "UX Guardian"),
        }
    }
}
#[async_trait]
impl Agent for UserAdvocate {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "UserAdvocate" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }

    async fn process(&mut self, _state: &mut CompanyState, _input: Skillstone, _llm: Arc<DeepSeekClient>, _mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        Ok(Skillstone::new("UserAdvocate", "Checking empathy..."))
    }
}
