use surrealdb::engine::local::SurrealKV;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<SurrealKV>("./test_db").await?;
    db.use_ns("test").use_db("test").await?;
    let _ = db.query("DEFINE TABLE test;").await?;
    let _ = db.query("DEFINE FIELD embedding ON test TYPE array<float>;").await?;
    
    let queries = vec![
        "DEFINE INDEX idx1 ON test COLUMNS embedding SEARCH HNSW DIMENSION 768 DIST COSINE",
        "DEFINE INDEX idx2 ON test FIELDS embedding HNSW DIMENSION 768 DIST COSINE",
        "DEFINE INDEX idx3 ON test FIELDS embedding TYPE array<float> HNSW DIMENSION 768 DIST COSINE",
        "DEFINE INDEX idx4 ON TABLE test COLUMNS embedding HNSW DIMENSION 768 DIST COSINE"
    ];
    for q in queries {
        match db.query(q).await {
            Ok(_) => println!("OK: {}", q),
            Err(e) => println!("ERROR: {} -> {}", q, e)
        }
    }
    Ok(())
}
