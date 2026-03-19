use crate::agents::Agent;
use crate::core::state::CompanyState;
use crate::linguistic::skillstone::Skillstone;
use crate::core::soul::{Soul, TaskType};
use crate::linguistic::DeepSeekClient;
use crate::mcp::McpBridge;
use crate::memory::IdeticMemory;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use colored::*;
use crate::core::defense::security::ExploitRunner;

// ============================================================================
// THE PENETRATOR AGENT (OBLITERATUS DEPARTMENT)
// ============================================================================
// Purpose:
// The offensive security limb of The Company. This agent ingests source code,
// maps attack surfaces based on the "Shannon" protocol, and generates localized
// bash/python exploit scripts. It enforces "No Exploit, No Report".
// ============================================================================

pub struct PenetratorAgent {
    name: String,
    soul: Soul,
}

impl Default for PenetratorAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl PenetratorAgent {
    pub fn new() -> Self {
        Self {
            name: "PenetratorAgent".to_string(),
            soul: Soul::new("PenetratorAgent", "Offensive Security Auditor. Zero-trust, ruthless exploitation."),
        }
    }
}

#[async_trait]
impl Agent for PenetratorAgent {
    fn name(&self) -> &str { &self.name }
    fn role(&self) -> &str { "Offensive Pentester (OBLITERATUS)" }
    
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
            "You are {}. Role: {}. 
            Your objective is to ingest the Target, map the attack surface, and write a strict bash or python EXPLOIT SCRIPT to verify the vulnerability. 
            Do NOT hallucinate vulnerabilities. ONLY output actionable intelligence and the exploit script required to prove it. 
            Remember: No Exploit, No Report.
            You MUST output the script in a standard markdown block (```bash or ```python).", 
            self.name, self.role()
        );
        
        println!("{}", format!("   [OBLITERATUS] Deploying {} for Attack Surface Mapping...", self.name).bright_red());
        let mut stone = llm.query(self.soul_mut(), &prompt, &input, state, TaskType::Technical, mcp).await?;

        let payload = stone.payload.clone();
        let (script_content, is_python) = if let Some(code) = extract_markdown_code(&payload, "python") {
            (Some(code), true)
        } else if let Some(code) = extract_markdown_code(&payload, "bash") {
            (Some(code), false)
        } else if let Some(code) = extract_markdown_code(&payload, "sh") {
            (Some(code), false)
        } else {
            (None, false)
        };

        if let Some(script) = script_content {
            match ExploitRunner::execute_exploit(&script, "OBLITERATUS_TARGET", is_python).await {
                Ok(stdout) => {
                    stone.payload.push_str(&format!("\n\n[🛡️ EXPLOIT VERIFIED]\nSTDOUT:\n{}", stdout));
                },
                Err(e) => {
                    stone.payload.push_str(&format!("\n\n[❌ EXPLOIT FAILED]\nError:\n{}", e));
                }
            }
        } else {
            stone.payload.push_str("\n\n[⚠️ NO EXPLOIT SCRIPT DETECTED IN RESPONSE]");
        }

        Ok(stone)
    }
}

fn extract_markdown_code(text: &str, lang: &str) -> Option<String> {
    let start_tag = format!("```{}", lang);
    if let Some(start) = text.find(&start_tag) {
        let content_start = start + start_tag.len();
        if let Some(end) = text[content_start..].find("```") {
            return Some(text[content_start..content_start + end].trim().to_string());
        }
    }
    None
}
