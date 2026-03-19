use the_consortium::core::market::blockchain_intel::BlockchainIntelAnalyzer;
use anyhow::Result;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "⛓️  [TEST] Testing Harvested Blockchain Intel Analyzer...".bright_yellow().bold());
    
    let analyzer = BlockchainIntelAnalyzer::new();
    
    let addresses = vec![
        "bc1pegl43ma86g0rugurpe332daacqvam0kn8n7mphnuz7fhyx70tjxssujrxy", // Taproot
        "0xE228DaC8646AF4EEeA318DbEa84F9906649B56A5",                   // Unknown
        "sanct_node_01_xyz",                                            // Sanctioned
        "mixer_tornado_limb_02",                                        // Mixer
    ];
    
    for addr in addresses {
        let report = analyzer.get_address_risk(addr).await?;
        
        println!("--------------------------------------------------");
        println!("Address: {}", report.address.cyan());
        println!("Entity:  {} ({})", report.entity_name.bold(), report.entity_type);
        println!("Risk:    {} ({:.2})", report.risk_level.red(), report.risk_score);
        
        if !report.significant_factors.is_empty() {
            println!("Factors:");
            for factor in report.significant_factors {
                println!("  - {}: {:.2} ({})", factor.factor_type.yellow(), factor.score, factor.description);
            }
        }
    }
    
    Ok(())
}
