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

pub mod penetrator;
pub use penetrator::PenetratorAgent;

pub struct QaAgent {
    name: String,
    soul: Soul,
}

impl Default for QaAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl QaAgent {
    pub fn new() -> Self {
        Self {
            name: "QaAgent".to_string(),
            soul: Soul::new("QaAgent", "Quality Assurance Engineer"),
        }
    }
}

#[async_trait]
impl Agent for QaAgent {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "QA Engineer" }
    
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
        let prompt = format!("You are {}. Role: {}. Perform a strict QA audit of the input. Use tools to verify if needed.", self.name, self.role());
        llm.query(self.soul_mut(), &prompt, &input, state, crate::core::soul::TaskType::Analytical, mcp).await
    }
}
