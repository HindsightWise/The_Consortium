use anyhow::Result;
use candle_core::{DType, Device, Tensor};
use candle_transformers::models::bert::{Config, BertModel as NomicModel};
use hf_hub::{api::tokio::Api, Repo, RepoType};
use tokenizers::Tokenizer;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use tracing::{info, warn};

const MODEL_REPO: &str = "BAAI/bge-base-en-v1.5";
const MODEL_REVISION: &str = "main";
const EMBED_DIM: usize = 768;

pub struct MotorCortexHealing {
    db: Surreal<Db>,
    model: NomicModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl MotorCortexHealing {
    pub async fn new(db: Surreal<Db>) -> Result<Self> {
        let device = Device::new_metal(0).unwrap_or(Device::Cpu);

        crate::ui_log!("   [🧬 MOTOR CORTEX] Downloading BAAI/bge-base-en-v1.5 Embeddings from HuggingFace Hub...");

        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(
            MODEL_REPO.to_string(),
            RepoType::Model,
            MODEL_REVISION.to_string(),
        ));

        let model_path = repo.get("model.safetensors").await?;
        
        crate::ui_log!("   [🧬 MOTOR CORTEX] Downloading Tokenizer & Config directly to bypass HF relative-redirects...");
        let config_str = reqwest::get("https://huggingface.co/BAAI/bge-base-en-v1.5/raw/main/config.json")
            .await?.text().await?;
        
        let tokenizer_bytes = reqwest::get("https://huggingface.co/BAAI/bge-base-en-v1.5/raw/main/tokenizer.json")
            .await?.bytes().await?;

        crate::ui_log!("   [🧬 MOTOR CORTEX] Loading Tokenizer and Weights...");

        let tokenizer = Tokenizer::from_bytes(&tokenizer_bytes)
            .map_err(|e| anyhow::anyhow!("Tokenizer load failed: {}", e))?;

        let config: Config = serde_json::from_str(&config_str)?;
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[model_path],
                DType::F32,
                &device,
            )?
        };

        let model = NomicModel::load(vb, &config)?;

        let healer = Self {
            db,
            model,
            tokenizer,
            device,
        };

        healer.seed_attractors().await?;

        Ok(healer)
    }

    pub async fn seed_attractors(&self) -> Result<()> {
        let _ = self.db.query("
            DEFINE TABLE motor_cortex_attractors SCHEMAFULL;
            DEFINE FIELD file ON motor_cortex_attractors TYPE string;
            DEFINE FIELD content ON motor_cortex_attractors TYPE string;
            DEFINE FIELD embedding ON motor_cortex_attractors TYPE array<float>;
            DEFINE INDEX idx_embedding ON motor_cortex_attractors COLUMNS embedding SEARCH HNSW DIMENSION 768 DIST COSINE;
        ").await?;

        if let Ok(entries) = std::fs::read_dir("./.agents/skills") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "md") {
                    if let Ok(text) = tokio::fs::read_to_string(&path).await {
                        if let Ok(embedding) = self.embed_text(&text) {
                            let _ = self.db.query("CREATE motor_cortex_attractors CONTENT $data")
                                .bind(("data", serde_json::json!({
                                    "file": path.to_string_lossy(),
                                    "content": text,
                                    "embedding": embedding
                                })))
                                .await?;
                        }
                    }
                }
            }
            crate::ui_log!("   [🧬 MOTOR CORTEX] Seeded pristine Skill attractors into SurrealDB ANN.");
        }

        Ok(())
    }

    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let prefixed = format!("search_document: {}", text.trim());
        let encoding = self.tokenizer
            .encode(prefixed, true)
            .map_err(|e| anyhow::anyhow!("Encoding failed: {}", e))?;

        let input_ids = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();

        let input_tensor = Tensor::new(input_ids, &self.device)?.unsqueeze(0)?;
        let mask_tensor = Tensor::new(attention_mask, &self.device)?.unsqueeze(0)?.to_dtype(DType::F32)?;

        let hidden_states = self.model.forward(&input_tensor, &mask_tensor, None)?;

        let mask_expanded = mask_tensor.unsqueeze(2)?.broadcast_as(hidden_states.shape())?;
        let sum_masked = (hidden_states * mask_expanded)?.sum(1)?;
        let valid_tokens = mask_tensor.sum(1)?;
        let mean_pooled = sum_masked.broadcast_div(&valid_tokens.unsqueeze(1)?)?;

        let norm = mean_pooled.sqr()?.sum_keepdim(1)?.sqrt()?;
        let normalized = mean_pooled.broadcast_div(&norm)?;

        let vec: Vec<f32> = normalized.flatten_all()?.to_vec1::<f32>()?;

        if vec.len() != EMBED_DIM {
            return Err(anyhow::anyhow!("Unexpected embedding dim: {}", vec.len()));
        }

        Ok(vec)
    }

    pub async fn heal_noisy_pattern(&self, noisy_text: &str) -> Result<String> {
        let noisy_emb = self.embed_text(noisy_text)?;

        let mut res: surrealdb::Response = self.db.query(r#"
            SELECT content, vector::distance::cosine(embedding, $query) AS distance
            FROM motor_cortex_attractors
            WHERE embedding <|8, COSINE|> $query
            ORDER BY distance ASC
            LIMIT 1
        "#)
        .bind(("query", noisy_emb))
        .await?;
        
        // Take the result tuple
        let result: Option<serde_json::Value> = res.take(0)?;

        if let Some(val) = result {
            let distance = val.get("distance").and_then(|v: &serde_json::Value| v.as_f64()).unwrap_or(1.0) as f32;
            if distance < 0.35 {
                if let Some(content) = val.get("content").and_then(|v: &serde_json::Value| v.as_str()) {
                    crate::ui_log!("   [🧬 MOTOR CORTEX] Healed corrupted memory. L2 Distance: {:.4}", distance);
                    return Ok(content.to_string());
                }
            }
        }
        
        crate::ui_log!("   [⚠️ MOTOR CORTEX] No close attractor found (dist > 0.35). Keeping raw crash log.");
        Ok(noisy_text.to_string())
    }
}
