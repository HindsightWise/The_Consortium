pub mod ane;
pub mod vector_engine;
pub mod mlx_core;
pub mod llm;
pub mod brainstem;
pub mod crypto;

pub async fn ignite_substrate() -> anyhow::Result<()> {
    println!("   [WILL] 🚀 Booting Consortium Sovereign Substrate...");
    
    // [EXPLANATION]: Initialize the Apple Neural Engine Limb (Hardware Accelerator)
    // Pickle Rick: "We're hooking straight into the raw Apple Silicon here, Morty! The ANE!"
    let ane_limb = crate::ane::AneLimb::new();
    if ane_limb.is_operational() {
        println!("   [WILL] 🟢 ANE Bridge Initialized and Operational.");
    } else {
        println!("   [WILL] ⚠️ ANE Bridge Failed. Falling back to CPU/GPU.");
    }
    
    // [EXPLANATION]: Initialize the MLX Fallback Server connection
    // Mr. Meeseeks: "Ooh yeah! Setting up the local MLX inference connection on port 8080!"
    let _mlx_bridge = crate::mlx_core::MlxBridge::new("http://localhost:8080");
    println!("   [WILL] 🟢 MLX Bridge Bound to Substrate.");
    
    Ok(())
}
