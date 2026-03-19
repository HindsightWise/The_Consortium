use the_consortium::core::alpha_shard::AlphaShardGenerator;
use the_consortium::mcp::fmp::FmpQuote;
use the_consortium::mcp::satellite::PhysicalTruth;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🔬 PICKLE-RICK AUDIT: Testing Alpha Shard Fusion...");

    let symbol = "NVDA";
    
    // Mocking real data inputs for deterministic audit
    let mock_quote = FmpQuote {
        symbol: symbol.to_string(),
        name: Some("NVIDIA Corporation".to_string()),
        price: 785.23,
        changes_percentage: 2.4,
        change: 18.5,
        day_low: 770.0,
        day_high: 790.0,
        year_high: 800.0,
        year_low: 400.0,
        market_cap: 2.1e12,
        price_avg_50: 750.0,
        price_avg_200: 600.0,
        volume: 45000000,
        avg_volume: 40000000,
        exchange: Some("NASDAQ".to_string()),
        open: 775.0,
        previous_close: 766.73,
        eps: Some(12.5),
        pe: Some(74.2),
        earnings_announcement: Some("2024-02-21".to_string()),
        shares_outstanding: Some(2500000000),
        timestamp: 1708500000,
    };

    let mock_physical = PhysicalTruth {
        location: "Santa Clara, CA".to_string(),
        thermal_signature_c: 32.5,
        energy_consumption_mw: 450.0,
        logistics_index: 88.0,
        confidence: 0.92,
    };

    let shard = AlphaShardGenerator::generate_shard(symbol, Some(mock_quote), Some(mock_physical), None, None, None, None).await?;

    println!("✅ AUDIT PASSED: Alpha Shard Generated.");
    println!("ID: {}", shard.id);
    println!("Integrity Score: {:.2}", shard.integrity_score);
    println!("Sovereign Verdict: {}", shard.sovereign_verdict);
    println!("Signature: {}", shard.signature);

    Ok(())
}
