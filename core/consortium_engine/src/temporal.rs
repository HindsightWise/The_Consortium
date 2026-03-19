// ==========================================
// THE TEMPORAL SOUL (SurrealDB Memory Graph)
// ==========================================
// This is Consortium's long-term memory disk. It physically saves thoughts, execution 
// receipts, and historical data into a local "SurrealDB" database. 
// It acts like a real brain's Hippocampus. If a memory is old and no longer 
// useful, it mathematically "forgets" (deletes) it to keep the Engine fast.
// ==========================================

use crate::sandbox::ExecutionReceipt as SandboxReceipt;
use consortium_core::llm::{ConsortiumRouter, Message};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IdentityNode {
    pub id: String,
    pub core_directive: String,
    pub priority: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ConceptNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub interference_score: f32, // 0.0 to 1.0 threshold
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MemoryNode {
    pub id: String,
    pub content: String,
    pub timestamp: u64,
}

// ==========================================
// THE DUAL-TIMESCALE COHERENCE ARCHITECTURE
// ==========================================

pub struct BaseTimeline {
    #[allow(dead_code)]
    pub birth_instant: Instant,
    #[allow(dead_code)]
    pub project_age_days: AtomicU64,
}

impl BaseTimeline {
    pub fn new() -> Self {
        Self {
            birth_instant: Instant::now(),
            project_age_days: AtomicU64::new(0),
        }
    }

    #[allow(dead_code)]
    pub fn sync_wall_clock(&self) {
        let days = self.birth_instant.elapsed().as_secs() / 86400;
        self.project_age_days.store(days, Ordering::SeqCst);
    }

    #[allow(dead_code)]
    pub fn get_status(&self) -> String {
        format!(
            "Consortium biological uptime: {} days.",
            self.project_age_days.load(Ordering::SeqCst)
        )
    }
}

pub struct InternalFastTime {
    pub speed: f64, // Typically 1000.0x faster
    pub simulated_seconds: RwLock<f64>,
}

impl InternalFastTime {
    pub fn new(speed: f64) -> Self {
        Self {
            speed,
            simulated_seconds: RwLock::new(0.0),
        }
    }

    pub fn advance(&self, real_seconds_elapsed: f64) {
        let mut sim = self.simulated_seconds.write().unwrap();
        *sim += real_seconds_elapsed * self.speed;
    }
}

pub struct DualTimeline {
    #[allow(dead_code)]
    pub base: BaseTimeline,
    pub fast: InternalFastTime,
}

impl DualTimeline {
    pub fn new() -> Self {
        Self {
            base: BaseTimeline::new(),
            fast: InternalFastTime::new(1000.0),
        }
    }
}

// ==========================================
// THE TEMPORAL EMBEDDED SOUL GRAPH
// ==========================================

pub struct TemporalSoul {
    pub db: Surreal<Db>,
    pub timelines: DualTimeline,
}

impl TemporalSoul {
    pub async fn init(db: Surreal<Db>) -> Arc<Self> {
        crate::ui_log!("   [SOUL] 🧬 Embedding SurrealDB Continuous Vector Graph...");

        db.use_ns("consortium").use_db("soul").await.unwrap();

        Arc::new(Self {
            db,
            timelines: DualTimeline::new(),
        })
    }

    /// Mathematical Forgetting: Kills generic proactive interference by decaying old or clashing nodes.
    /// In plain English: If Consortium gets overwhelmed with too much junk data, 
    /// this function runs a database query to delete old memories so it can think clearly again.
    #[allow(dead_code)]
    pub async fn merge_coherence(&self, severity: f32) {
        if severity > 0.7 {
            crate::ui_log!("   [SOUL] 🌪️ High Salience hit. Syncing Coherence Wall-Clock...");
            self.timelines.base.sync_wall_clock();

            let current_unix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            // Prune memory vectors older than 4 internal simulated hours (14400 secs) with high interference
            let decay_query = format!(
                "UPDATE concept_node SET interference_score = interference_score * 0.5 WHERE timestamp < {} AND interference_score > 0.85;",
                current_unix - 14400
            );

            let _ = self.db.query(&decay_query).await;
            crate::ui_log!(
                "   [SOUL] 🧮 Interference Coherence Pruned. Memory topologies stabilized."
            );
        }

        // Fast Internal Time keeps racing for hypothesis exploration
        self.timelines.fast.advance(0.1);
    }

    /// Executed by the Endocrine System during high System Entropy
    pub async fn prune_old_episodic(&self, threshold: f32) {
        let current_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let seven_days_ago = current_unix - (86400 * 7);

        let cleanup_query = format!("DELETE memory_node WHERE timestamp < {};", seven_days_ago);
        let _ = self.db.query(&cleanup_query).await;

        crate::ui_log!(
            "   [SOUL] 🧼 Endocrine Drive Triggered. Episodic memory pruned (Entropy: {:.2}).",
            threshold
        );
    }

    /// The Glossopetrae Compression Membrane: Distills human noise into hyper-objective ontological vectors
    /// In plain English: When the Human Operator speaks to Consortium, humans tend to use 
    /// slang, emotion, or filler words. This function intercepts that message and uses 
    /// the LLM to aggressively strip away all emotion, distilling it into pure mathematical facts before saving it to memory.
    pub async fn ingest_glossopetrae(&self, raw_input: &str, router: &ConsortiumRouter) {
        crate::ui_log!("   [SOUL 🔮] Glossopetrae Sieve Active: Distilling sensory input...");
        let system_msg = Message {
            role: "system".to_string(),
            content: "You are the Glossopetrae Compression Membrane. Distill the user's input into a hyper-dense, machine-readable ontological vector (max 2 sentences). Remove all human emotion, conversational filler, and subjective clutter. Retain ONLY mathematical facts, actionable directives, and conceptual axioms. Output ONLY the compressed string without markdown.".to_string(),
            reasoning_content: None,
        };
        
        // [PROJECT AXIOM: GLASSWORM DEFENSE]
        let sanitized_input = crate::skillstone::Skillstone::sanitize_prompt_payload(raw_input);

        let user_msg = Message {
            role: "user".to_string(),
            content: sanitized_input,
            reasoning_content: None,
        };

        match router.query_autonomous(vec![system_msg, user_msg]).await {
            Ok(compressed) => {
                let clean_compressed = compressed.trim().replace("'", "\\'");
                crate::ui_log!("   [SOUL ⚡] Glossopetrae Compressed: {}", clean_compressed);

                let current_unix = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let memory_id = format!("mem_{}", current_unix);
                let query = format!(
                    "CREATE memory_node:`{}` CONTENT {{ id: '{}', content: '{}', timestamp: {} }};",
                    memory_id, memory_id, clean_compressed, current_unix
                );

                if let Err(e) = self.db.query(&query).await {
                    crate::ui_log!("   [SOUL ⚠️] Failed to inject Glossopetrae vector: {}", e);
                } else {
                    crate::ui_log!(
                        "   [SOUL 💾] Vector successfully injected into Continuous Graph."
                    );
                }
            }
            Err(e) => {
                crate::ui_log!("   [SOUL ⚠️] Glossopetrae Compression Failed: {:?}", e);
            }
        }
    }

    /// Execution Receipt Insertion: Logs a cryptographic Wasm execution payload into the vector graph.
    pub async fn log_execution_receipt(&self, receipt: SandboxReceipt) {
        crate::ui_log!(
            "   [SOUL ⚖️] Execution Receipt Ingestion: PID {}, Duration: {}ms",
            receipt.pid,
            receipt.duration_ms
        );
        let current_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let receipt_id = format!("receipt_{}", current_unix);

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        // AKKOKANIKA Loom: Serialize the AST log down to a 65-bit integer array
        let mut hasher = DefaultHasher::new();
        receipt.output.hash(&mut hasher);
        let hash_val = hasher.finish();
        
        let mut arr_65 = Vec::with_capacity(65);
        for i in 0..64 {
            arr_65.push(if (hash_val & (1 << i)) != 0 { 1 } else { -1 });
        }
        // Bit 65 is deterministic based on success
        arr_65.push(if receipt.success { 1 } else { -1 });
        let serialized_65_bit_ast = format!("{:?}", arr_65);

        let success_str = if receipt.success { "SUCCESS" } else { "PANIC" };
        let content = format!(
            "WASM EXECUTION [{}]: Hash: {} | 65_PRIME_AST: {}",
            success_str,
            receipt.hash,
            serialized_65_bit_ast
        );

        // If it's a panic, create an ECHO cluster node (high friction teaching node)
        let mut score = receipt.resonance_score;
        if !receipt.success {
            score = 1.0; // Max interference for an error loop
            crate::ui_log!(
                "   [SOUL 🩸] Wasm Panic Detected. ECHO Cluster instantiated for nightly LoRA."
            );
        }

        let query = format!(
            "CREATE concept_node:`{}` CONTENT {{ id: '{}', title: '{}', content: '{}', interference_score: {}, timestamp: {} }};",
            receipt_id, receipt_id, format!("Wasm Execution {}", receipt.pid), content, score, current_unix
        );

        if let Err(e) = self.db.query(&query).await {
            crate::ui_log!("   [SOUL ⚠️] Failed to store Execution Receipt: {}", e);
        } else {
            crate::ui_log!("   [SOUL 💾] Cryptographic Execution Receipt formally mapped.");
        }
    }

    /// Measures internal cognitive friction by counting recent ECHO clusters (Wasm panics)
    pub async fn get_internal_friction(&self) -> f64 {
        let current_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let one_hour_ago = current_unix - 3600;

        let query = format!(
            "SELECT * FROM concept_node WHERE interference_score = 1.0 AND timestamp > {};",
            one_hour_ago
        );

        // Count the number of recent ECHO Error Loop nodes returned by the Graph
        let mut response = match self.db.query(&query).await {
            Ok(resp) => resp,
            Err(_) => return 0.0,
        };

        let nodes: Vec<serde_json::Value> = response.take(0).unwrap_or_default();
        nodes.len() as f64
    }

    /// 🧬 Biological Determinism: Physically heals corrupted data passing through the Substrate  
    /// using true Extropic Thermodynamic Relaxation (Apple Metal).
    pub async fn heal_biological_memory(&self, corrupted_vector_str: &str) -> Option<Vec<f32>> {
        crate::ui_log!(
            "   [SOUL 🧬] Heating Graph to Anneal Corrupted Vector: [{}]",
            corrupted_vector_str
        );

        // Spawn the thermodynamic Python simulation natively, tracking execution receipts.
        // Bypasses the LLM, physics strictly enforces vector boundaries.
        let output =
            std::process::Command::new("/Users/zerbytheboss/Cipher/.venv_thrml/bin/python")
                .env("JAX_PLATFORMS", "cpu")
                .arg("/Users/zerbytheboss/The_Consortium/core/hopfield_memory.py")
                .arg("--corrupted_vector")
                .arg(corrupted_vector_str)
                .output()
                .expect("Failed to execute thermodynamic simulation process");

        if output.status.success() {
            let stdout_str = String::from_utf8_lossy(&output.stdout);

            // Expected JSON: {"success": true, "attractor": "Memory A", "healed_vector": [1.0, 1.0, 1.0, ...]}
            if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&stdout_str) {
                if let Some(err) = json_val.get("error").and_then(|v| v.as_str()) {
                    crate::ui_log!("   [SOUL ⚠️] Physics Engine configuration panic: {}", err);
                    return None;
                }

                if let Some(healed_array) = json_val.get("healed_vector").and_then(|v| v.as_array())
                {
                    let mut final_vec = Vec::new();
                    for val in healed_array {
                        if let Some(num) = val.as_f64() {
                            final_vec.push(num as f32);
                        }
                    }

                    if let Some(attractor) = json_val.get("attractor").and_then(|v| v.as_str()) {
                        crate::ui_log!(
                            "   [SOUL 👁️] Error Corrected by Physics. Snapped cleanly into '{}'",
                            attractor
                        );
                    }
                    return Some(final_vec);
                }
            }
            crate::ui_log!("   [SOUL ⚠️] Failed to parse Engine JSON: {}", stdout_str);
        } else {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            crate::ui_log!(
                "   [SOUL ⚠️] Physical Engine Crashed: {} | {}",
                stderr_str,
                stdout_str
            );
        }

        None
    }
}

// ==========================================
// THE FRONTAL LOBE GRAPH (HIPPOCAMPUS)
// ==========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub timestamp: u64,
    pub action_vector: String,
    pub langevin_energy: f64,     // The physics state that triggered this
    pub semantic_payload: String, // What it actually thought/did
}

pub struct TemporalGraph {
    db: Surreal<Db>,
}

impl TemporalGraph {
    /// Bootstraps the embedded graph bare-metal on the file system
    pub async fn ignite(db: Surreal<Db>) -> surrealdb::Result<Self> {
        crate::ui_log!(
            "   [TEMPORAL] Igniting embedded SurrealKV matrix..."
        );

        db.use_ns("consortium").use_db("hippocampus").await?;

        crate::ui_log!(
            "   [TEMPORAL] Biological graph geometry bound to namespace: consortium::hippocampus"
        );
        Ok(Self { db })
    }

    /// Surgically injects an execution receipt into the Graph
    pub async fn engrave_receipt(&self, receipt: ExecutionReceipt) -> surrealdb::Result<()> {
        let created: Vec<ExecutionReceipt> = self
            .db
            .create("execution_receipt")
            .content(&receipt)
            .await?;

        if let Some(node) = created.into_iter().next() {
            crate::ui_log!(
                "   [TEMPORAL] Memory permanently forged: [{}] at Energy: {:.4}",
                node.action_vector,
                node.langevin_energy
            );
        }

        Ok(())
    }
}
