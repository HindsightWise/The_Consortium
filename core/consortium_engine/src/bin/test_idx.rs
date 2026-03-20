use surrealdb::engine::local::SurrealKV;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<SurrealKV>("./test_db_idx2").await?;
    db.use_ns("test").use_db("test").await?;
    let _ = db.query("DEFINE TABLE motor_cortex_attractors SCHEMAFULL;").await?;
    let _ = db.query("DEFINE FIELD embedding ON motor_cortex_attractors TYPE array<float>;").await?;
    
    let queries = vec![
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors COLUMNS embedding HNSW DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors FIELDS embedding HNSW DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors COLUMNS embedding SEARCH HNSW DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors FIELDS embedding SEARCH HNSW DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors COLUMNS embedding TYPE array<float> HNSW DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors FIELDS embedding TYPE array<float> HNSW DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON TABLE motor_cortex_attractors COLUMNS embedding SEARCH HNSW DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors FIELDS embedding SEARCH MTREE DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors COLUMNS embedding SEARCH MTREE DIMENSION 768 DIST COSINE;",
        "DEFINE INDEX idx_embedding ON motor_cortex_attractors FIELDS embedding MTREE DIMENSION 768 DIST COSINE;"
    ];

    for q in queries {
        let res = db.query(q).await;
        match res {
            Ok(mut r) => {
                let errs = r.take_errors();
                if errs.is_empty() {
                    println!("SUCCESS: {}", q);
                } else {
                    println!("ERROR MAP for {}: {:?}", q, errs);
                }
            },
            Err(e) => println!("ERROR for {}: {}", q, e),
        }
    }
    
    Ok(())
}
