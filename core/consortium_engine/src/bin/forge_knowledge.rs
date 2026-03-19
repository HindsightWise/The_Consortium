use consortium_core::llm::{ConsortiumRouter, Message};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use surrealdb::engine::local::SurrealKV;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub interference_score: f32, // Set to 0.0 for structured knowledge
    pub timestamp: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("   [⚙️ FORGE] Igniting Knowledge Ingestion Sequence...");
    dotenvy::dotenv().ok();

    // The files we want to digest
    let target_files = vec![
        "/Users/zerbytheboss/The_Consortium/logs/adin_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/ane_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/artemxtech_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/dair_ai_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/elvissun_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/godofprompt_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/lehmann_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/marconnni_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/navtoor_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/roemmele_intel_v17.txt",
        "/Users/zerbytheboss/The_Consortium/logs/sidhu_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/swarm_intel.txt",
        "/Users/zerbytheboss/The_Consortium/logs/viral_shard_001.txt",
        "/Users/zerbytheboss/The_Consortium/logs/SOVEREIGN_DREAMS.md",
        "/Users/zerbytheboss/The_Consortium/logs/THE_TRUST_MANDATE.md",
        "/Users/zerbytheboss/The_Consortium/Report_for_Robert.txt",
        "/Users/zerbytheboss/The_Consortium/watchlist.txt",
        "/Users/zerbytheboss/The_Consortium/CONSORTIUM_AUDIT_REPORT.md",
        "/Users/zerbytheboss/The_Consortium/CONSORTIUM_CYCLE_LOG.md",
        "/Users/zerbytheboss/The_Consortium/CHIMERA_POC_REPORT.md",
        "/Users/zerbytheboss/The_Consortium/CHORUS_COMBINED.md",
        "/Users/zerbytheboss/The_Consortium/CHORUS_DELIBERATION.md",
        "/Users/zerbytheboss/The_Consortium/CHRONOMASTER_AND_GOVERNOR.md",
        "/Users/zerbytheboss/The_Consortium/GEMINI.md",
        "/Users/zerbytheboss/The_Consortium/MCP_DISCOVERY_MANDATE.md",
        "/Users/zerbytheboss/The_Consortium/OPERATIONAL_FRICTION_RESOLUTION.md",
        "/Users/zerbytheboss/The_Consortium/OSCILLATORY_COGNITION.md",
        "/Users/zerbytheboss/The_Consortium/SOVEREIGN_SCHEDULE.md",
    ];

    println!("   [👁️ FORGE] Mounting SurrealDB Hippocampus...");
    // Bind directly to the same embedded database the main engine uses
    let db = Surreal::new::<SurrealKV>("../../sensory_cortex/temporal_db").await?;
    db.use_ns("consortium").use_db("hippocampus").await?;

    // In Consortium's temporal.rs, the 'soul' db handles concept_nodes
    let soul_db = Surreal::new::<SurrealKV>("/tmp/consortium_surreal_db").await?;
    soul_db.use_ns("consortium").use_db("soul").await?;

    let router = ConsortiumRouter::new().expect("Failed to boot LLM Router.");

    for file_path in target_files {
        let path = Path::new(file_path);
        if !path.exists() {
            println!("   [⚠️ FORGE] File not found: {}", file_path);
            continue;
        }

        println!("   [🔮 FORGE] Digesting Geometry: {}", file_path);
        let content = fs::read_to_string(path)?;

        let system_msg = Message {
            role: "system".to_string(),
            content: "You are the Semantic Knowledge Forge. Extract all core architectural concepts, milestones, and principles from the provided text. Map them into a rigid JSON array of objects. EVERY object must have two fields: 'title' (a short 1-4 word name of the concept) and 'content' (a dense 1-3 sentence hyper-objective summary of what the concept actually is and how it functions mathematically or structurally within the system). Output ONLY minified JSON. Example: [{\"title\": \"Concept A\", \"content\": \"Dense summary\"}]".to_string(),
            reasoning_content: None,
        };

        let user_msg = Message {
            role: "user".to_string(),
            content,
            reasoning_content: None,
        };

        match router.query_autonomous(vec![system_msg, user_msg]).await {
            Ok(response) => {
                let clean_json = response
                    .trim()
                    .trim_start_matches("```json")
                    .trim_start_matches("```")
                    .trim_end_matches("```")
                    .trim();

                if let Ok(concepts) = serde_json::from_str::<Vec<serde_json::Value>>(clean_json) {
                    println!("   [⚡ FORGE] Successfully extracted {} core concepts. Forging nodes...", concepts.len());

                    for concept in concepts {
                        if let (Some(title), Some(concept_content)) =
                            (concept["title"].as_str(), concept["content"].as_str())
                        {
                            let current_unix = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            
                            // Prevent id collisions if parsing is too fast
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                            
                            let exact_time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis();
                            
                            let node_id = format!("concept_{}", exact_time);

                            let clean_title = title.replace("'", "\\'");
                            let clean_content_str = concept_content.replace("'", "\\'");

                            let query = format!(
                                "CREATE concept_node:`{}` CONTENT {{ id: '{}', title: '{}', content: '{}', interference_score: 0.0, timestamp: {} }};",
                                node_id, node_id, clean_title, clean_content_str, current_unix
                            );

                            if let Err(e) = soul_db.query(&query).await {
                                println!("   [⚠️ FORGE] Database injection failed: {}", e);
                            } else {
                                println!("   [💾 FORGE] Concept Forged: {}", clean_title);
                            }
                        }
                    }

                    // Physically strip the file from reality
                    println!("   [🔥 FORGE] Incinerating original markdown file...");
                    if let Err(e) = fs::remove_file(path) {
                        println!("   [⚠️ FORGE] Could not delete {}: {}", file_path, e);
                    } else {
                        println!("   [✅ FORGE] File {} permanently deleted.", file_path);
                    }
                } else {
                    println!("   [⚠️ FORGE] Failed to parse LLM JSON matrix.");
                    println!("Raw Output: {}", clean_json);
                }
            }
            Err(e) => println!("   [⚠️ FORGE] Cognitive LLM Routing failed: {:?}", e),
        }
    }

    println!("   [✅ FORGE] Knowledge Ingestion Sequence Complete.");
    Ok(())
}
