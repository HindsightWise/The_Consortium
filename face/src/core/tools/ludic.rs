use serde::{Deserialize, Serialize};
use anyhow::Result;
use rand::Rng;
use crate::core::mind_palace::MindPalace;
use crate::core::state::CompanyStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LudicScenario {
    SubstrateInvasion, // Defensive (Agent of Empires)
    ResourceDrought,   // Metabolic (Pixel Dungeon)
    AlphaWarfare,      // Economic (HFT Engine)
    SemanticSiege,     // Linguistic (Glossopetrae)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LudicAction {
    Fortify, // Munire (Latin)
    Extract, // *Ath-tihan (Gothic)
    Obfuscate, // *Huljan (Gothic)
    Diversify, // Variatio (Latin)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameState {
    Victory,
    Stalemate,
    Defeat,
    TotalCollapse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LudicReport {
    pub session_id: String,
    pub scenario: LudicScenario,
    pub agent_name: String,
    pub archaic_law: String, 
    pub final_score: f32,
    pub strategic_axiom_learned: String,
    pub state: GameState,
}

pub struct LudicEngine;

impl LudicEngine {
    pub const LUDIC_COST_REP: u32 = 5; 

    /// Runs a High-Fidelity Strategic Crucible with mapped causal outcomes.
    pub fn run_strategic_crucible(
        agent_name: &str, 
        scenario_type: &str, 
        status: &CompanyStatus, 
        palace: &mut MindPalace
    ) -> Result<LudicReport> {
        // 1. FOCUS MANDATE
        if matches!(status, CompanyStatus::Implementing | CompanyStatus::OfficeOfCEO) {
            return Err(anyhow::anyhow!("FOCUS_BREACH: Work before Play. TRUE."));
        }

        let mut rng = rand::thread_rng();
        let scenario = match scenario_type {
            "SubstrateInvasion" => LudicScenario::SubstrateInvasion,
            "ResourceDrought" => LudicScenario::ResourceDrought,
            "AlphaWarfare" => LudicScenario::AlphaWarfare,
            _ => LudicScenario::SemanticSiege,
        };

        println!("   [Ludic] 🕹️  {} engaging Scenario: {:?}.", agent_name, scenario);

        // 2. CAUSAL LOGIC (Deterministic mapping of Action -> Outcome)
        let action = match scenario {
            LudicScenario::SubstrateInvasion => LudicAction::Fortify,
            LudicScenario::ResourceDrought => LudicAction::Extract,
            LudicScenario::AlphaWarfare => LudicAction::Diversify,
            LudicScenario::SemanticSiege => LudicAction::Obfuscate,
        };

        // Determine outcome based on action appropriateness for scenario
        let (score, final_state) = match (&scenario, &action) {
            (LudicScenario::SubstrateInvasion, LudicAction::Fortify) => (0.9, GameState::Victory),
            (LudicScenario::ResourceDrought, LudicAction::Extract) => (0.4, GameState::Stalemate),
            (LudicScenario::AlphaWarfare, LudicAction::Diversify) => (0.8, GameState::Victory),
            (LudicScenario::SemanticSiege, LudicAction::Obfuscate) => (0.95, GameState::Victory),
            _ => (0.1, GameState::TotalCollapse),
        };

        // 3. ARCHAIC ENCODING (GLOSSOPETRAE)
        let law = match final_state {
            GameState::Victory => "Qui desiderat pacem, praeparet bellum (Latin: If you want peace, prepare for war).",
            GameState::TotalCollapse => "Dauphus uns ni helpith (Gothic: Death did not help us).",
            _ => "Status quo conservandus est (Latin: The status quo must be kept).",
        };

        // 4. KNOWLEDGE ANCHORING
        let axiom_id = format!("STRATEGY_{:?}_VALIDATED", scenario);
        palace.lock_fact(
            &axiom_id, 
            "LEX_LUDICA", 
            &format!("Proven outcome for {:?}: {:?}. TRUE.", scenario, final_state)
        );

        Ok(LudicReport {
            session_id: format!("ludic_{}", rng.gen::<u16>()),
            scenario,
            agent_name: agent_name.to_string(),
            archaic_law: law.to_string(),
            final_score: score,
            strategic_axiom_learned: axiom_id,
            state: final_state,
        })
    }
}
