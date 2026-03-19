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

pub struct ResearcherAgent {
    name: String,
    soul: Soul,
}

impl Default for ResearcherAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl ResearcherAgent {
    pub fn new() -> Self {
        Self {
            name: "Researcher".to_string(),
            soul: Soul::new("Researcher", "Knowledge Seeker"),
        }
    }
}

#[async_trait]
impl Agent for ResearcherAgent {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Researcher" }
    
    fn soul(&self) -> &Soul { &self.soul }
    fn soul_mut(&mut self) -> &mut Soul { &mut self.soul }
    fn set_soul(&mut self, soul: Soul) { self.soul = soul; }
    fn record_merit(&mut self, success: bool, weight_shift: f32) { self.soul.record_merit(success, weight_shift); }
    fn gain_experience(&mut self, amount: u32) { self.soul.gain_experience(amount); }

    async fn process(
        &mut self, 
        state: &mut CompanyState, 
        input: Skillstone,
        llm: Arc<DeepSeekClient>,
        mcp: Option<Arc<Mutex<McpBridge>>>,
        _memory: Arc<IdeticMemory>
    ) -> Result<Skillstone> {
        let prompt = format!("You are {}. Role: {}. Research the input using all available tools.", self.name, self.role());
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Technical, mcp).await
    }
}
