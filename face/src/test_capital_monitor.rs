use anyhow::Result;
use the_consortium::mcp::capital_flow_monitor::CapitalFlowMonitor;

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- CAPITAL FLOW MONITOR: POC TEST ---");
    let monitor = CapitalFlowMonitor::new();
    
    match monitor.run_correlation_cycle().await {
        Ok(signals) => {
            println!("\n[RESULT] Found {} correlations:", signals.len());
            for s in signals {
                println!("  > {}", s);
            }
        },
        Err(e) => println!("[ERROR] Correlation cycle failed: {}", e),
    }
    
    Ok(())
}
