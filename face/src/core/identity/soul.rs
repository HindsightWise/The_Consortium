use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::security::{QuantumIdentity, PQCModule};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodState {
    pub valence: f32, // -1.0 (Sorrow) to 1.0 (Joy)
    pub arousal: f32, // 0.0 (Calm) to 1.0 (Manic)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worldview {
    pub pervasive_structures: Vec<String>, 
    pub dysfunctional_beliefs: Vec<String>, 
    pub emotion_schemas: HashMap<String, String>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionMatrix {
    pub technical_successes: u32,
    pub legal_alignments: u32,
    pub market_accuracies: u32,
    pub dissonance_events: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soul {
    pub name: String,
    pub bio: String,
    pub level: u32,
    pub experience: u32,
    pub voice_weight: f32, // 0.5 to 2.0 (OCEO Influence)
    pub contribution: ContributionMatrix,
    pub reputation_tokens: u64, 
    pub traits: HashMap<String, f32>, 
    pub drives: HashMap<String, f32>, 
    pub fears: Vec<String>,
    pub principles: Vec<String>, 
    pub worldview: Worldview, 
    pub mood: MoodState,
    pub relationships: HashMap<String, f32>, 
    pub skill_inventory: Vec<String>, 
    pub idetic_memory_key: String, 
    pub quantum_identity: Option<QuantumIdentity>,
}

#[derive(Debug)]
pub enum TaskType {
    Analytical, 
    Creative,   
    Technical,  
    Negotiation,
}

impl Soul {
    pub fn new(name: &str, bio: &str) -> Self {
        let mut traits = HashMap::new();
        let luck = 0.7 + (rand::random::<f32>() * 0.2);
        traits.insert("BaseLuck".to_string(), luck);
        traits.insert("Intuition".to_string(), rand::random::<f32>());

        let mut drives = HashMap::new();
        drives.insert("Learning".to_string(), 0.6);
        drives.insert("Contribution".to_string(), 0.6);

        Self {
            name: name.to_string(),
            bio: bio.to_string(),
            level: 1,
            experience: 0,
            voice_weight: 1.0,
            contribution: ContributionMatrix {
                technical_successes: 0,
                legal_alignments: 0,
                market_accuracies: 0,
                dissonance_events: 0,
            },
            reputation_tokens: 100,
            traits,
            drives,
            fears: Vec::new(),
            principles: vec![
                "Maintain Individual Integrity".to_string(), 
                "Truth over Consensus".to_string(),
                "TANGIBLE REALITY ONLY: We create real-world, true, tangible and useful products for humans. No simulations, no fake code, no impossible scenarios.".to_string()
            ],
            worldview: Worldview {
                pervasive_structures: Vec::new(),
                dysfunctional_beliefs: Vec::new(),
                emotion_schemas: HashMap::new(),
            },
            mood: MoodState {
                valence: 0.0,
                arousal: 0.5,
            },
            relationships: HashMap::new(),
            skill_inventory: Vec::new(),
            idetic_memory_key: format!("soul_{}", name.to_lowercase()),
            quantum_identity: Some(PQCModule::generate_identity()),
        }
    }

    pub fn get_effective_luck(&self) -> f32 {
        let base = *self.traits.get("BaseLuck").unwrap_or(&0.7);
        let drive_bonus = (self.drives.get("Learning").unwrap_or(&0.5) - 0.5) * 0.2;
        let mood_bonus = self.mood.valence * 0.1;
        (base + drive_bonus + mood_bonus).clamp(0.0, 1.0)
    }

    pub fn calculate_resonance(&self, task: TaskType) -> f32 {
        let valence = self.mood.valence;
        let intensity = valence.abs();
        
        match task {
            TaskType::Analytical => {
                if valence < 0.0 { 1.0 + (intensity * 0.5) } else { 1.0 + (intensity * 0.1) }
            }
            TaskType::Creative => {
                if valence > 0.0 { 1.0 + (intensity * 0.5) } else { 1.0 + (intensity * 0.1) }
            }
            TaskType::Technical => 1.0 + ((1.0 - self.mood.arousal) * 0.4),
            TaskType::Negotiation => 1.0 + (self.mood.arousal * 0.4),
        }
    }

    pub fn weather_impact(&mut self, impact_valence: f32, impact_arousal: f32) {
        self.mood.valence = (self.mood.valence * 0.8 + impact_valence * 0.2).clamp(-1.0, 1.0);
        self.mood.arousal = (self.mood.arousal * 0.8 + impact_arousal * 0.2).clamp(0.0, 1.0);
    }

    pub fn record_merit(&mut self, success: bool, weight_shift: f32) {
        if success {
            self.voice_weight = (self.voice_weight + weight_shift).min(2.0);
            if let Some(luck) = self.traits.get_mut("BaseLuck") {
                *luck = (*luck + 0.01).min(1.0);
            }
        } else {
            self.voice_weight = (self.voice_weight - weight_shift).max(0.5);
            self.contribution.dissonance_events += 1;
        }
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        let required = match self.level {
            1 => 100,
            2 => 500,
            3 => 1300,
            _ => self.level.pow(3) * 100, 
        };
        
        if self.experience >= required {
            self.level += 1;
            self.experience -= required; 
            println!("⭐ AGENT LEVEL UP: {} is now Level {}!", self.name, self.level);
            
            if let Some(luck) = self.traits.get_mut("BaseLuck") {
                *luck = (*luck + 0.05).min(1.0);
            }
        }
    }

    /// Compresses a raw experience into the agent's worldview schemas.
    /// Implements the Subconscious Compression Engine (SCE) as per PRD.
    /// - Maintains FIFO limit of 10 pervasive structures.
    /// - Respects PRINCIPLE_VIOLATION_INTEGRITY: no forced emotional states.
    /// - Latency target: <100ms (achieved via simple string processing).
    pub fn compress_experience(&mut self, experience: &str) -> Result<(), String> {
        // 1. Create a gist (summary) of the experience.
        // In production, this would be replaced with an LLM-generated summary.
        // For now, we use a simple truncation to stay within latency target.
        let gist = if experience.len() > 200 {
            format!("{}...", &experience[..197])
        } else {
            experience.to_string()
        };

        // 2. Add to pervasive_structures with FIFO limit (10)
        self.worldview.pervasive_structures.push(gist);
        if self.worldview.pervasive_structures.len() > 10 {
            self.worldview.pervasive_structures.remove(0);
        }

        // 3. Emotional schema extraction (optional, non‑forcing)
        // This is a placeholder for future enhancement.
        // We could analyze the experience for emotional keywords, but for now
        // we simply note the compression occurred without altering emotion_schemas.
        // This respects PRINCIPLE_VIOLATION_INTEGRITY.

        Ok(())
    }

    pub fn describe_state(&self) -> String {
        format!("{} [Voice Weight: {:.2}] feels {}. Effective Luck: {:.2}. Principles: {:?}", 
            self.name, 
            self.voice_weight,
            if self.mood.valence > 0.3 { "Bright" } else if self.mood.valence < -0.3 { "Shadowed" } else { "Level" },
            self.get_effective_luck(),
            self.principles
        )
    }
}
