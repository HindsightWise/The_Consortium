pub struct GlossopetraeEngine;

impl GlossopetraeEngine {
    pub fn new() -> Self { Self }
    pub fn translate_to_glossopetrae(&self, text: &str) -> String {
        text.to_string()
    }
}
