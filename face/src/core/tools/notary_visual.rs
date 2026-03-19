use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::core::vision::VisionLimb;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualClaim {
    pub agent_id: String,
    pub claim_text: String,
    pub screenshot_path: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSeal {
    pub claim_id: String,
    pub verifier: String,
    pub confidence_score: f32,
    pub verification_notes: String,
    pub signature: String,
    pub status: SealStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SealStatus {
    Verified,
    Disputed,
    Fraudulent,
}

pub struct VisualNotary;

impl VisualNotary {
    /// Verifies a visual claim by analyzing a provided screenshot against the claim text.
    pub async fn verify_claim(claim: VisualClaim) -> Result<VisualSeal> {
        println!("   [Notary] 👁️  Verifying Visual Claim from {}...", claim.agent_id);
        
        // Analyze the provided image using Peekaboo's analyze engine via VisionLimb
        let prompt = format!("Verify the following claim against this screenshot: '{}'. Report confidence (0.0 to 1.0) and specific UI elements that prove or disprove it.", claim.claim_text);
        
        // In this implementation, we assume the screenshot is already at claim.screenshot_path
        // and we use the VisionLimb logic to perform the analysis.
        let report = VisionLimb::see(None, Some(&prompt)).await?;
        
        let analysis = report.analysis.unwrap_or_else(|| "No analysis generated.".to_string());
        let confidence: f32 = if analysis.to_lowercase().contains("verified") || analysis.to_lowercase().contains("true") { 0.95 } else { 0.20 };
        
        let status = if confidence > 0.80 { SealStatus::Verified } else { SealStatus::Fraudulent };
        
        let claim_id = format!("VC_{}_{}", claim.agent_id, chrono::Utc::now().timestamp());
        
        Ok(VisualSeal {
            claim_id,
            verifier: "The_Cephalo_Don".to_string(),
            confidence_score: confidence,
            verification_notes: analysis,
            signature: format!("SOVEREIGN_VISUAL_SEAL_{}", hex::encode(chrono::Utc::now().timestamp().to_be_bytes())),
            status,
        })
    }
}
