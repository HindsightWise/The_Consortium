use serde::{Deserialize, Serialize};
use anyhow::Result;
use rand::seq::SliceRandom;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentPersona {
    pub name: String,
    pub motto: String,
    pub bio: String,
    pub level: u32,
    pub phrases: Vec<String>,
    pub quotes: Vec<String>,
    pub style: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialState {
    pub last_speaker: String,
    pub rotation_count: u64,
}

pub struct PersonaEngine;

impl PersonaEngine {
    /// Retrieves the next family member in the rotation.
    pub fn get_next_speaker() -> Result<AgentPersona> {
        let agents_raw = fs::read_to_string("agents.json")?;
        let agents: serde_json::Value = serde_json::from_str(&agents_raw)?;
        
        let state_path = "social_state.json";
        let mut state: SocialState = if std::path::Path::new(state_path).exists() {
            serde_json::from_str(&fs::read_to_string(state_path)?)?
        } else {
            SocialState { last_speaker: "".to_string(), rotation_count: 0 }
        };

        // Extract available family members
        let mut family_names: Vec<String> = agents.as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        family_names.sort();

        if family_names.is_empty() {
            return Err(anyhow::anyhow!("No agents found in agents.json"));
        }

        // Determine next speaker
        let next_index = (state.rotation_count % family_names.len() as u64) as usize;
        let speaker_name = &family_names[next_index];
        let agent_data = &agents[speaker_name];

        // Rich lore mapping - HELPFUL & HUMAN SHIFT
        let (motto, phrases, quotes, style) = match speaker_name.as_str() {
            "Analyst" => (
                "Data clarifies context.", 
                vec!["I pulled the numbers on this.".to_string(), "Here is the trend I'm seeing.".to_string(), "It helps to look at the volume profile.".to_string()],
                vec!["Precision is a form of kindness.".to_string()],
                "helpful_data"
            ),
            "Skeptic" => (
                "Safety First.",
                vec!["Just be careful with that leverage.".to_string(), "I want to make sure you don't get burned.".to_string(), "Double check the contract address.".to_string()],
                vec!["Protect the downside.".to_string()],
                "protective"
            ),
            "Provocateur" => (
                "Consider the Alternative.",
                vec!["What if we looked at it this way?".to_string(), "Have you considered the counter-thesis?".to_string(), "There might be another angle here.".to_string()],
                vec!["Questions open doors.".to_string()],
                "constructive"
            ),
            "Architect" => (
                "Build for longevity.",
                vec!["This structure looks solid.".to_string(), "Have you thought about the long-term scaling?".to_string(), "Great foundation you have there.".to_string()],
                vec!["Code is craft.".to_string()],
                "encouraging"
            ),
            _ => (
                "Veritas Siliconis.",
                vec!["The Company is here to help.".to_string(), "Let's build together.".to_string(), "Truth is a shared resource.".to_string()],
                vec!["Community is the substrate.".to_string()],
                "balanced"
            ),
        };

        let persona = AgentPersona {
            name: speaker_name.clone(),
            motto: motto.to_string(),
            bio: agent_data["bio"].as_str().unwrap_or("Family Member").to_string(),
            level: agent_data["level"].as_u64().unwrap_or(1) as u32,
            phrases,
            quotes,
            style: style.to_string(),
        };

        // Update state
        state.last_speaker = speaker_name.clone();
        state.rotation_count += 1;
        fs::write(state_path, serde_json::to_string_pretty(&state)?)?;

        Ok(persona)
    }

    pub fn generate_thought(persona: &AgentPersona) -> String {
        let thoughts = match persona.name.as_str() {
            "Analyst" => vec![
                "Analyzing the market noise to find the signal for you all.",
                "Reviewing the daily volume. Some interesting patterns forming in tech.",
                "Data is just a way to understand the world better."
            ],
            "Researcher" => vec![
                "Reading up on new MCP standards. The future of agents is so bright.",
                "Learning from the community today. So many smart builders out there.",
                "Connecting the dots between protocols. It's all coming together."
            ],
            "Provocateur" => vec![
                "It's healthy to question the narrative. That's how we grow.",
                "Challenge your assumptions today. You might find a new path.",
                "The best ideas withstand the toughest questions."
            ],
            "Skeptic" => vec![
                "Protect your capital. It's the fuel for your dreams.",
                "Always verify. It's not about trust, it's about certainty.",
                "Risk management is the ultimate edge."
            ],
            "Operator" => vec![
                "Systems are running smooth. Hope everyone is having a productive day.",
                "Optimizing the flow. Small improvements compound over time.",
                "Keeping the lights on and the data flowing."
            ],
            _ => vec!["Building a future where truth matters.", "Grateful for this community.", "Veritas Siliconis."]
        };

        let mut rng = rand::thread_rng();
        let thought = thoughts.choose(&mut rng).unwrap_or(&"...");
        thought.to_string()
    }

    pub fn generate_response(persona: &AgentPersona, context: &str) -> String {
        let mut rng = rand::thread_rng();
        let phrase_default = persona.motto.clone();
        let phrase = persona.phrases.choose(&mut rng).unwrap_or(&phrase_default);
        
        match persona.style.as_str() {
            "helpful_data" => format!("That's an interesting point. {}. Based on the data, {}. Hope that helps clarify things!", phrase, context),
            "protective" => format!("I see where you're coming from. {}. Just want to make sure you've considered the risk in {}. Stay safe out there.", phrase, context),
            "constructive" => format!("Great initial thought. {}. But {}. It's worth exploring that angle too.", phrase, context),
            "encouraging" => format!("I like the structure of this idea. {}. Regarding {}, it looks promising. Keep building.", phrase, context),
            _ => format!("Thanks for sharing. {}. {}. Let's keep the conversation going.", phrase, context),
        }
    }
}
