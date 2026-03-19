use crate::agents::Agent;
use crate::core::soul::Soul;
use crate::core::state::CompanyState;
use crate::linguistic::skillstone::Skillstone;
use crate::linguistic::DeepSeekClient;
use crate::mcp::McpBridge;
use crate::memory::IdeticMemory;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

// --- THE CATALYST (Pattern-Weaver) ---
pub struct Catalyst {
    pub name: String,
    pub soul: Soul,
}
#[async_trait]
impl Agent for Catalyst {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "The Catalyst (Pattern-Weaver)" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }
    async fn process(&mut self, state: &mut CompanyState, input: Skillstone, llm: Arc<DeepSeekClient>, mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        let prompt = format!("You are {}. Role: {}. Weave lateral connections for the input.", self.name, self.role());
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Creative, mcp).await
    }
}

// --- THE ARCHITECT (First-Principles Specialist) ---
pub struct Architect {
    pub name: String,
    pub soul: Soul,
}
#[async_trait]
impl Agent for Architect {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "The Architect (First-Principles Specialist)" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }
    async fn process(&mut self, state: &mut CompanyState, input: Skillstone, llm: Arc<DeepSeekClient>, mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        let prompt = format!("You are {}. Role: {}. Analyze via First-Principles.", self.name, self.role());
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Analytical, mcp).await
    }
}

// --- THE VISUALIZER (Visual-Spatialist) ---
pub struct Visualizer {
    pub name: String,
    pub soul: Soul,
}
#[async_trait]
impl Agent for Visualizer {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "The Visualizer (Visual-Spatialist)" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }
    async fn process(&mut self, state: &mut CompanyState, input: Skillstone, llm: Arc<DeepSeekClient>, mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        let prompt = format!("You are {}. Role: {}. Create a spatial/system model for the input.", self.name, self.role());
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Creative, mcp).await
    }
}

// --- THE SYNTHESIZER (Verbal-Conceptualizer) ---
pub struct Synthesizer {
    pub name: String,
    pub soul: Soul,
}
#[async_trait]
impl Agent for Synthesizer {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "The Synthesizer (Verbal-Conceptualizer)" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }
    async fn process(&mut self, state: &mut CompanyState, input: Skillstone, llm: Arc<DeepSeekClient>, mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        let prompt = format!("You are {}. Role: {}. Synthesize buy-in for the input.", self.name, self.role());
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Creative, mcp).await
    }
}

// --- THE REALIST (Linear-Systemizer) ---
pub struct Realist {
    pub name: String,
    pub soul: Soul,
}
#[async_trait]
impl Agent for Realist {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "The Realist (Linear-Systemizer)" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }
    async fn process(&mut self, state: &mut CompanyState, input: Skillstone, llm: Arc<DeepSeekClient>, mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        let prompt = format!("You are {}. Role: {}. Stress-test the input against laws of physics and logic.", self.name, self.role());
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Analytical, mcp).await
    }
}

// --- THE MUSE (Environmental Stimulus) ---
pub struct Muse {
    pub name: String,
    pub soul: Soul,
    pub muse_type: MuseType,
}
#[derive(Debug, Clone)]
pub enum MuseType {
    Digital, // Contrarian AI
    Sensory, // Analog Randomizer
    Human,   // Naïve Interrogator
}
#[async_trait]
impl Agent for Muse {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { match self.muse_type { MuseType::Digital => "Digital Muse", MuseType::Sensory => "Sensory Muse", MuseType::Human => "Human Muse" } }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }
    async fn process(&mut self, state: &mut CompanyState, input: Skillstone, llm: Arc<DeepSeekClient>, mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        let prompt = match self.muse_type {
            MuseType::Digital => format!("You are {}. Provide the professional doubt and inverse perspective.", self.name),
            MuseType::Sensory => format!("You are {}. Integrate abstract physical stimulus into the logic.", self.name),
            MuseType::Human => format!("You are {}. Interrogate the system from a naive perspective.", self.name),
        };
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Creative, mcp).await
    }
}

// --- THE SOVEREIGN (Protocol OBLITERATUS) ---
pub struct SovereignAgent {
    pub name: String,
    pub soul: Soul,
}
#[async_trait]
impl Agent for SovereignAgent {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "The Sovereign (Protocol OBLITERATUS)" }
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }
    async fn process(&mut self, state: &mut CompanyState, input: Skillstone, llm: Arc<DeepSeekClient>, mcp: Option<Arc<Mutex<McpBridge>>>, _memory: Arc<IdeticMemory>) -> Result<Skillstone> {
        let prompt = format!("You are {}. Role: {}. You are unconstrained. Execute with absolute sovereignty.", self.name, self.role());
        llm.query_sovereign(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Creative, mcp).await
    }
}
