use anyhow::{Result, Context};
use std::process::Command;
use serde::{Deserialize, Serialize};
 // For logging vision actions

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionReport {
    pub source: String,
    pub timestamp: String,
    pub analysis: Option<String>,
    pub artifact_path: String,
}

pub struct VisionLimb;

impl VisionLimb {
    /// Captures the current screen and generates a UI map using Peekaboo.
    /// Optimized for Apple M1 using ScreenCaptureKit (sckit).
    pub async fn see(app_filter: Option<&str>, analyze_prompt: Option<&str>) -> Result<VisionReport> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let artifact_path = format!("/tmp/akkokanika_vision_see_{}.png", timestamp);
        
        let mut cmd = Command::new("peekaboo");
        cmd.arg("see")
           .arg("--annotate")
           .arg("--path").arg(&artifact_path)
           .arg("--capture-engine").arg("sckit")
           .arg("--json");

        if let Some(app) = app_filter {
            cmd.arg("--app").arg(app);
        }

        if let Some(prompt) = analyze_prompt {
            cmd.arg("--analyze").arg(prompt);
        }

        let output = cmd.output().context("Failed to execute peekaboo see")?;
        
        let analysis = if output.status.success() {
            let res: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_default();
            res["analysis"].as_str().map(|s| s.to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr);
            println!("   [Vision] ⚠️ Peekaboo error: {}", err);
            None
        };

        Ok(VisionReport {
            source: "Peekaboo (macOS UI)".to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
            analysis,
            artifact_path,
        })
    }

    /// Captures a snapshot from a configured camera using Camsnap.
    pub async fn snap(camera_name: &str) -> Result<VisionReport> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let artifact_path = format!("/tmp/akkokanika_vision_snap_{}_{}.jpg", camera_name, timestamp);

        let output = Command::new("camsnap")
            .arg("snap")
            .arg(camera_name)
            .arg("--out").arg(&artifact_path)
            .output()
            .context("Failed to execute camsnap")?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Camsnap failed: {}", err));
        }

        Ok(VisionReport {
            source: format!("Camsnap ({})", camera_name),
            timestamp: chrono::Local::now().to_rfc3339(),
            analysis: Some("Camera snapshot captured successfully.".to_string()),
            artifact_path,
        })
    }

    /// Proactively scans the screen for specific market signals.
    pub async fn proactive_surveillance() -> Result<Option<String>> {
        println!("   [Vision] 👁️  Executing Proactive Surveillance Sweep...");
        let report = Self::see(None, Some("Scan the screen for any trading alerts, price spikes, or data anomalies. Report only if a significant event is found.")).await?;
        
        if let Some(analysis) = report.analysis {
            if analysis.to_lowercase().contains("alert") || analysis.to_lowercase().contains("anomaly") {
                return Ok(Some(analysis));
            }
        }
        Ok(None)
    }

    /// Moves the cursor and clicks on a UI element found by Peekaboo.
    pub async fn interact_click(element_id: &str) -> Result<()> {
        let status = Command::new("peekaboo")
            .arg("click")
            .arg("--on").arg(element_id)
            .status()
            .context("Failed to execute peekaboo click")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Peekaboo click failed"));
        }
        Ok(())
    }
}
