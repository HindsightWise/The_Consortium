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
        let device = Device::Cpu;

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

        let mut input_ids = encoding.get_ids().to_vec();
        let mut attention_mask = encoding.get_attention_mask().to_vec();

        // Enforce maximum structural bounds to prevent candle-core Positional Embedding unwinding panics on massive contextual anchors
        if input_ids.len() > 512 {
            input_ids.truncate(512);
            attention_mask.truncate(512);
        }

        let input_tensor = Tensor::new(input_ids.as_slice(), &self.device)?.unsqueeze(0)?;
        let mask_tensor = Tensor::new(attention_mask.as_slice(), &self.device)?.unsqueeze(0)?.to_dtype(DType::F32)?;

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

    /// Surgically writes pruned contextual memories (from Oblivion Protocol) into permanent long-term graphical storage
    pub async fn archive_pruned_memory(&self, role: &str, content: &str, embedding: Vec<f32>) -> Result<()> {
        let result: surrealdb::Response = self.db.query(r#"
            CREATE archived_memories SET
                role = $role,
                content = $content,
                embedding = $embedding,
                archived_at = time::now()
        "#)
        .bind(("role", role))
        .bind(("content", content))
        .bind(("embedding", embedding))
        .await?;
        
        let _ = result.check()?;
        Ok(())
    }

    /// Surgically extracts the closest contextual shards from the long-term temporal graph over BAAI topologies.
    pub async fn query_deep_memory(&self, query_text: &str) -> Result<String> {
        let embedding = self.embed_text(query_text)?;
        
        // Execute Cosine query against 'archived_memories' limits to Top 5 nearest neighbors
        let mut res = self.db.query(r#"
            SELECT content, role, vector::distance::cosine(embedding, $embedding) AS distance 
            FROM archived_memories 
            WHERE embedding <|8, COSINE|> $embedding
            ORDER BY distance ASC LIMIT 5
        "#)
        .bind(("embedding", embedding))
        .await?;
        
        let records: Vec<serde_json::Value> = res.take(0)?;
        
        let mut results_str = String::new();
        for (i, rec) in records.iter().enumerate() {
            let content = rec.get("content").and_then(|v| v.as_str()).unwrap_or("");
            let distance = rec.get("distance").and_then(|v| v.as_f64()).unwrap_or(0.0);
            results_str.push_str(&format!("Result [{}] (Semantic Distance: {:.3}):\n{}\n\n", i+1, distance, content));
        }
        
        if results_str.is_empty() {
            Ok("[⚠️ MOTOR CORTEX] Semantic search yielded absolute zero relative vectors (No memories found).".to_string())
        } else {
            Ok(results_str)
        }
    }
}
