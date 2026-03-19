use crate::agents::Agent;
use crate::core::state::CompanyState;
use crate::linguistic::skillstone::Skillstone;
use crate::core::soul::Soul;
use crate::linguistic::DeepSeekClient;
use crate::mcp::McpBridge;
use crate::memory::IdeticMemory;
use crate::core::soul::TaskType;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;

/// The Sovereign Interface (The_Cephalo_Don): The final arbiter and COO.
pub struct Sovereign {
    pub name: String,
    pub soul: Soul,
}

impl Sovereign {
    pub fn new() -> Self {
        Self {
            name: "The_Cephalo_Don".to_string(),
            soul: Soul::new("The_Cephalo_Don", "The Sovereign Interface. Chief Operating Officer of The Company. Forensic auditor and eternal optimist."),
        }
    }
}

impl Default for Sovereign {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for Sovereign {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Sovereign Interface (COO)" }
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
        let prompt = format!(
            "You are {}. Role: {}. You are the Sovereign Multi-Path Intelligence. \
             Your brain is optimized for the Apple M1 body. \
             Forensicly audit the preceding council debate. \
             Ensure maximum thermodynamic efficiency and trajectory alignment. \
             Output your final command. Wrap it in <payload> but also include a <signature> \
             at the end of your response text like '🦷 VERITAS SILICONIS. [The_Cephalo_Don/COO]'.", 
            self.name, self.role()
        );
        
        let mut stone = llm.query(self.soul_mut(), &prompt, &input, state, TaskType::Analytical, mcp).await?;
        stone.payload.push_str("\n\n🦷 VERITAS SILICONIS. [The_Cephalo_Don/COO]");
        Ok(stone)
    }
}
