use anyhow::Result;
use candle_core::Device;
use candle_transformers::models::quantized_qwen2::ModelWeights;
use candle_core::quantized::gguf_file;
use hf_hub::{api::sync::Api, Repo, RepoType};
use std::fs::File;

pub struct Brainstem {
    _model: ModelWeights,
    _device: Device,
}

impl Brainstem {
    pub fn wake_up() -> Result<Self> {
        // println!("...

        // The user's M1 Mac benefits natively from the Metal backend
        let device = Device::new_metal(0).unwrap_or(Device::Cpu);

        // Fetch the perfectly stable 1.5B Qwen 2.5 model from HuggingFace
        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(
            "Qwen/Qwen2.5-1.5B-Instruct-GGUF".to_string(),
            RepoType::Model,
            "main".to_string(),
        ));
        
        // Use the exact official filename located inside that repository
        let model_path = repo.get("qwen2.5-1.5b-instruct-q4_k_m.gguf")?;
        
        // Deserialize the GGUF weights into the candle tensor space
        let mut file = File::open(model_path)?;
        let gguf_content = gguf_file::Content::read(&mut file)?;
        let model = ModelWeights::from_gguf(gguf_content, &mut file, &device)?;

        // println!("...
        
        Ok(Self { _model: model, _device: device })
    }

    pub fn check_salience(&self, impulse: &str) -> bool {
        // A minimal text analysis to determine if an impulse is worth waking
        // the 9-Billion parameter cloud/MLX frontal lobe for execution.
        let lowercase_impulse = impulse.to_lowercase();
        
        // A hardcoded mathematical bypass filter for the primary anomalies:
        let critical_tags = ["vip", "error", "down", "crash", "anomaly", "urgent", "system"];
        for tag in critical_tags {
            if lowercase_impulse.contains(tag) {
                // println!("...
                return true;
            }
        }

        // If it's a small generic ping, we ignore it to conserve energy
        if impulse.len() < 15 {
            // println!("...
            return false;
        }

        // For more complex impulses, the Brainstem will natively calculate structural relevance.
        // For now, if it survives the drop filter, we pass it up the spine.
        // println!("...
        true
    }
}
