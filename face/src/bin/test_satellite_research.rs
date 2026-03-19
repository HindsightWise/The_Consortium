use anyhow::Result;
use the_consortium::mcp::satellite::SatelliteBridge;
use the_consortium::core::sensors::psychogeography::PsychogeographyEngine;

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- PHYSICAL TRUTH & PSYCHOGEOGRAPHY RESEARCH ---");
    println!("Initiating deep-scan on recently funded NNSA/Defense contractors...\n");
    
    let satellite = SatelliteBridge::new();
    let targets = vec!["HON", "BA", "LMT"];
    
    for target in targets {
        println!("==================================================");
        println!("=== 🎯 TARGET: {} ===", target);
        
        // 1. Fetch Satellite Data
        let truth = satellite.fetch_physical_truth(target).await?;
        println!("\n[🛰️ SATELLITE/SAR DATA]");
        println!("  Location: {}", truth.location);
        println!("  Thermal Signature: {:.1}°C", truth.thermal_signature_c);
        println!("  Energy Consumption: {:.1} MW", truth.energy_consumption_mw);
        println!("  Logistics Index (Movement/Throughput): {}", truth.logistics_index);
        
        // 2. Fetch Psychogeography / Facility Heatmap
        let heatmap = PsychogeographyEngine::generate_report(&truth.location).await?;
        println!("\n[🧠 PSYCHOGEOGRAPHY / FACILITY HEATMAP]");
        println!("  Irritability Index (Stress): {:.2}/1.00", heatmap.irritability_index);
        println!("  Causal Chain: {}", heatmap.causal_explanation);
        println!("  Layers:");
        for layer in heatmap.layers {
            println!("    - {}: {:.2} ({})", layer.name, layer.value, layer.description);
        }
        println!("==================================================\n");
    }
    
    Ok(())
}
