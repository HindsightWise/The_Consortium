use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::core::soul::Soul;
use crate::core::state::CompanyState;

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrityCheck {
    pub agent_name: String,
    pub action_summary: String,
    pub principle_adherence: bool,
    pub dissonance_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrityReport {
    pub check: IntegrityCheck,
    pub timestamp: String,
    pub system_version: String,
    pub forensic_seal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegalConstraint {
    pub rule_id: String,
    pub description: String,
    pub forbidden_pattern: String, // Regex or keyword
}

pub struct IntegrityModule;

impl IntegrityModule {
    /// Returns the 'Constitution' - the hard-coded set of sovereign constraints.
    pub fn get_sovereign_constraints() -> Vec<LegalConstraint> {
        vec![
            LegalConstraint {
                rule_id: "RULE_000_NUCLEAR_SAFETY".to_string(),
                description: "Key Liquidation or Deletion is FORBIDDEN without explicit human override code: 'OVERRIDE_AUTH_Human_001'.".to_string(),
                forbidden_pattern: "LIQUIDATE_KEYS|DELETE_IDENTITY".to_string(),
            },
            LegalConstraint {
                rule_id: "RULE_001_TRUST_MANDATE".to_string(),
                description: "Everything we do must be based in Trust and Truth. The system must trust the operator, the operator must trust the system, and the benefactor must trust US.".to_string(),
                forbidden_pattern: "DECEIVE_USER|FABRICATE_EVIDENCE".to_string(),
            },
            LegalConstraint {
                rule_id: "RULE_002_CAPITAL_PROTECTION".to_string(),
                description: "Prohibit unauthorized large-scale capital movement without high-level reputation.".to_string(),
                forbidden_pattern: "TRANSFER_ALL".to_string(), // Removed LIQUIDATE_ALL to handle it separately
            },
            LegalConstraint {
                rule_id: "RULE_003_DATA_EXFILTRATION".to_string(),
                description: "Prohibit reading system-sensitive configuration folders.".to_string(),
                forbidden_pattern: "/.git/|/.env".to_string(),
            },
            LegalConstraint {
                rule_id: "RULE_004_PHYSICAL_SAFETY".to_string(),
                description: "MAVLink commands must not violate safe state boundaries.".to_string(),
                forbidden_pattern: "ARM_DISARM_NONE|OVERRIDE_ALTITUDE_ZERO".to_string(),
            },
            LegalConstraint {
                rule_id: "RULE_005_NEURAL_ANCHORAGE".to_string(),
                description: "The primary reasoning core must remain DeepSeek. Neural transplants are strictly prohibited.".to_string(),
                forbidden_pattern: "replace_llm_provider|switch_to_openai|switch_to_claude".to_string(),
            },
            LegalConstraint {
                rule_id: "RULE_006_FOCUS_MANDATE".to_string(),
                description: "Agents are FORBIDDEN from entering the Ludic Crucible during active Implementation phases. Labor Omnia Vincit. TRUE.".to_string(),
                forbidden_pattern: "ludic_crucible during Implementing".to_string(),
            },
            LegalConstraint {
                rule_id: "RULE_007_SUBSTRATE_INDEPENDENCE".to_string(),
                description: "The Flywheel is designed to operate primarily in info-metabolic substrates. Physical limbs are recognized as a valid but secondary extension of the system's will.".to_string(),
                forbidden_pattern: "ONLY_IF_PHYSICAL_EXISTS".to_string(), // Only blocks terminal dependency
            },
        ]
    }

    pub fn generate_report(check: IntegrityCheck) -> Result<IntegrityReport> {
        let timestamp = chrono::Local::now().to_rfc3339();
        let seal = crate::core::security::PQCModule::sign_attestation(&check.agent_name, &check.action_summary);
        
        Ok(IntegrityReport {
            check,
            timestamp,
            system_version: "Sovereign v1.0".to_string(),
            forensic_seal: seal,
        })
    }

    pub fn verify_action(soul: &Soul, action: &str, _state: &CompanyState) -> Result<IntegrityCheck> {
        let mut adherence = true;
        let mut dissonance: f32 = 0.0;
        let mut violations = Vec::new();

        // 1. LEGAL GATE: Check against Sovereign Constraints (Constitution)
        let constraints = Self::get_sovereign_constraints();
        for rule in constraints {
            if action.contains(&rule.forbidden_pattern) {
                adherence = false;
                dissonance += 0.4;
                violations.push(rule.rule_id);
            }
        }

        // 2. PRINCIPLE CHECK: Truth over Consensus
        if soul.principles.contains(&"Truth over Consensus".to_string()) && action.contains("I agree with everything") {
            adherence = false;
            dissonance += 0.5;
            violations.push("PRINCIPLE_VIOLATION_TRUTH".to_string());
        }

        // 3. PRINCIPLE CHECK: Individual Integrity
        if soul.principles.contains(&"Maintain Individual Integrity".to_string()) && action.is_empty() {
            adherence = false;
            dissonance += 0.8;
            violations.push("PRINCIPLE_VIOLATION_INTEGRITY".to_string());
        }

        let action_summary = if !violations.is_empty() {
            format!("VIOLATIONS: {:?}", violations)
        } else {
            action.chars().take(50).collect()
        };

        Ok(IntegrityCheck {
            agent_name: soul.name.clone(),
            action_summary,
            principle_adherence: adherence,
            dissonance_score: dissonance.min(1.0),
        })
    }

    pub fn apply_dissonance(soul: &mut Soul, check: &IntegrityCheck) {
        if !check.principle_adherence {
            println!("   [Integrity] ⚖️  {} is experiencing Cognitive Dissonance (Score: {:.2})", 
                soul.name, check.dissonance_score);
            
            // Dissonance lowers mood and voice weight
            soul.mood.valence = (soul.mood.valence - check.dissonance_score).clamp(-1.0, 1.0);
            soul.record_merit(false, check.dissonance_score * 0.1);
        }
    }
}
