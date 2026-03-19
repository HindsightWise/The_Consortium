pub struct CftcPipeline;

impl CftcPipeline {
    pub fn new() -> Self { Self }
    pub async fn get_net_positioning(&self, _asset: &str) -> f64 {
        0.0
    }
}
