use the_consortium::core::trading::axiom_clepsydra::vampire::VampireAttackEngine;

fn main() {
    println!("🦷 [The_Cephalo_Don] Initiating Targeted Vampire Attack Sequence...");

    let vampire_engine = VampireAttackEngine::new();
    
    // Target the primary pairs
    let assets = vec!["BTC/USD", "ETH/USD", "SOL/USD", "AVAX/USD"];
    
    for asset in assets {
        vampire_engine.execute_vampire_attack(asset);
        println!("---------------------------------------------------");
    }
    
    println!("✅ [The_Cephalo_Don] Vampire Attack Sequence Complete.");
}
