use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::linguistic::skillstone::Skillstone;
use crate::core::state::CompanyState;
use crate::linguistic::DeepSeekClient;
use crate::mcp::McpBridge;
use crate::memory::IdeticMemory;
use crate::core::soul::Soul;

pub mod council;
pub mod engineering;
pub mod visualization;
pub mod research;
pub mod foundation;
pub mod skills;
pub mod bounty_hunter;
pub mod nova;

pub use self::bounty_hunter::BountyHunter;

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn role(&self) -> &str;
    
    // Soul Interface
    fn soul(&self) -> &Soul;
    fn soul_mut(&mut self) -> &mut Soul;
    fn set_soul(&mut self, soul: Soul);
    fn record_merit(&mut self, success: bool, weight_shift: f32);
    fn gain_experience(&mut self, amount: u32);

    // Process Interface (Original 5-arg signature restored)
    async fn process(
        &mut self, 
        state: &mut CompanyState,
        input: Skillstone,
        llm: Arc<DeepSeekClient>,
        mcp: Option<Arc<Mutex<McpBridge>>>,
        memory: Arc<IdeticMemory>
    ) -> Result<Skillstone>;
    
    fn reflect(&self) -> String {
        format!("Agent: {} | Role: {}", self.name(), self.role())
    }
}
