use anyhow::Result;
use crate::mcp::usaspending_tracker::USASpendingTracker;

pub struct CapitalFlowMonitor {
    spending: USASpendingTracker,
}

impl CapitalFlowMonitor {
    pub fn new() -> Self {
        Self {
            spending: USASpendingTracker::new(),
        }
    }

    pub async fn run_correlation_cycle(&self) -> Result<Vec<String>> {
        println!("   [NSO-Supervisor] 🛡️ Starting Alpha-Only Funding Cycle...");
        
        // 1. Fetch High-Value Awards (>$5M) from the last 30 days
        let awards = self.spending.search_recent_awards(5_000_000.0).await?;

        // 2. Identify "Clusters" (Multiple awards to the same sector/region)
        let mut signals = Vec::new();

        for award in awards.iter() {
            let report = format!(
                "💎 ALPHA DETECTED: {} | Award: ${:.2}M from {} | Desc: {}",
                award.recipient_name,
                award.award_amount / 1_000_000.0,
                award.agency_name,
                award.description.chars().take(60).collect::<String>() + "..."
            );
            signals.push(report);
        }

        if signals.is_empty() {
            println!("   [NSO-Supervisor] ⚠️ No high-value funding clusters found.");
        } else {
            println!("   [NSO-Supervisor] ✅ Found {} active funding signals.", signals.len());
        }

        Ok(signals)
    }
}
