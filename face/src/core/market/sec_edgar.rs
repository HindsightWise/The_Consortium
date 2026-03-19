pub struct SecEdgarPipeline;

impl SecEdgarPipeline {
    pub fn new() -> Self { Self }
    pub async fn analyze_form4(&self, ticker: &str) -> String {
        format!("SEC Edgar Form 4 output for {}", ticker)
    }
}
