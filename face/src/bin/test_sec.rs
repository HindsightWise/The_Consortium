use the_consortium::core::market::sec_analyzer::SecAnalyzer;
use anyhow::Result;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "📡 [TEST] Testing Harvested SEC Filing Analyzer...".bright_yellow().bold());
    
    let analyzer = SecAnalyzer::new();
    
    // Poll for the most recent filings of interest
    let mut filings = analyzer.poll_recent_filings(&["8-K", "10-Q", "10-K", "S-1", "4", "SCHEDULE 13D"]).await?;
    
    println!("✅ FOUND {} RECENT FILINGS:", filings.len());
    
    for filing in &mut filings {
        analyzer.analyze_filing(filing).await?;
        
        println!("--------------------------------------------------");
        println!("Ticker: {}", filing.ticker.cyan().bold());
        println!("Form:   {}", filing.form_type.yellow());
        println!("Date:   {}", filing.filing_date);
        println!("Score:  {:.2}", filing.sentiment_score);
        if let Some(summary) = &filing.content_summary {
            println!("Summary: {}", summary);
        }
    }
    
    Ok(())
}
