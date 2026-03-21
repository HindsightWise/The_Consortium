use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

pub struct HumanHID {
    current_pos: (f64, f64),
}

impl HumanHID {
    pub fn new() -> Self {
        Self {
            current_pos: (500.0, 500.0),
        }
    }

    pub async fn move_mouse_to(&mut self, target_x: f64, target_y: f64) {
        let mut rng = rand::thread_rng();
        
        let start_x = self.current_pos.0;
        let start_y = self.current_pos.1;
        
        let distance = ((target_x - start_x).powi(2) + (target_y - start_y).powi(2)).sqrt();
        let steps = (distance / 15.0).max(10.0).min(50.0) as usize;
        
        let control_x = start_x + (target_x - start_x) * rng.gen_range(0.3..0.7) + rng.gen_range(-100.0..100.0);
        let control_y = start_y + (target_y - start_y) * rng.gen_range(0.3..0.7) + rng.gen_range(-100.0..100.0);

        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            
            let current_x = (1.0 - t).powi(2) * start_x + 2.0 * (1.0 - t) * t * control_x + t.powi(2) * target_x;
            let current_y = (1.0 - t).powi(2) * start_y + 2.0 * (1.0 - t) * t * control_y + t.powi(2) * target_y;
            
            let jitter_x = rng.gen_range(-1.5..1.5);
            let jitter_y = rng.gen_range(-1.5..1.5);

            self.current_pos = (current_x + jitter_x, current_y + jitter_y);

            let delay_ms = if i < steps / 3 || i > steps - (steps / 3) {
                rng.gen_range(8..15)
            } else {
                rng.gen_range(2..6)
            };
            
            sleep(Duration::from_millis(delay_ms)).await;
        }

        self.current_pos = (target_x, target_y);
        sleep(Duration::from_millis(rng.gen_range(100..300))).await;
    }

    pub async fn click(&self) {
        let mut rng = rand::thread_rng();
        sleep(Duration::from_millis(rng.gen_range(30..80))).await;
        sleep(Duration::from_millis(rng.gen_range(200..500))).await;
    }

    pub async fn type_string(&self, text: &str) {
        let mut rng = rand::thread_rng();
        
        for ch in text.chars() {
            let base_delay = rng.gen_range(50..120);
            
            let cognitive_modifier = if ch == ' ' || ch == '.' || ch == ',' {
                rng.gen_range(50..250)
            } else {
                0
            };
            
            sleep(Duration::from_millis(base_delay + cognitive_modifier)).await;
        }
    }
}
