#[cfg(target_os = "macos")]
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
#[cfg(target_os = "macos")]
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
#[cfg(target_os = "macos")]
use core_graphics::geometry::CGPoint;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

/// The Human Scrambler: Translates perfect machine vectors into 
/// biologically plausible, entropy-rich HID events.
pub struct HumanHID {
    #[cfg(target_os = "macos")]
    current_pos: CGPoint,
    #[cfg(not(target_os = "macos"))]
    current_pos: (f64, f64),
    #[cfg(target_os = "macos")]
    source: CGEventSource,
}

impl HumanHID {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        {
            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
            // Assume starting roughly in the center or a safe zone
            Self {
                current_pos: CGPoint::new(500.0, 500.0),
                source,
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            Self {
                current_pos: (500.0, 500.0),
            }
        }
    }

    /// Moves the mouse from current position to (target_x, target_y) using a Bezier-like noisy curve.
    pub async fn move_mouse_to(&mut self, target_x: f64, target_y: f64) {
        #[cfg(target_os = "macos")]
        {
            let mut rng = rand::thread_rng();
            
            let start_x = self.current_pos.x;
            let start_y = self.current_pos.y;
            
            // Fitts's Law approximation: Distance dictates steps
            let distance = ((target_x - start_x).powi(2) + (target_y - start_y).powi(2)).sqrt();
            let steps = (distance / 15.0).max(10.0).min(50.0) as usize; // 10 to 50 micro-steps
            
            // Generate a random control point to simulate a curved, imperfect trajectory
            let control_x = start_x + (target_x - start_x) * rng.gen_range(0.3..0.7) + rng.gen_range(-100.0..100.0);
            let control_y = start_y + (target_y - start_y) * rng.gen_range(0.3..0.7) + rng.gen_range(-100.0..100.0);

            for i in 1..=steps {
                let t = i as f64 / steps as f64;

                // Quadratic Bezier interpolation
                let current_x = (1.0 - t).powi(2) * start_x + 2.0 * (1.0 - t) * t * control_x + t.powi(2) * target_x;
                let current_y = (1.0 - t).powi(2) * start_y + 2.0 * (1.0 - t) * t * control_y + t.powi(2) * target_y;

                // Add micro-jitter (tremor)
                let jitter_x = rng.gen_range(-1.5..1.5);
                let jitter_y = rng.gen_range(-1.5..1.5);

                let pos = CGPoint::new(current_x + jitter_x, current_y + jitter_y);

                if let Ok(event) = CGEvent::new_mouse_event(
                    self.source.clone(),
                    CGEventType::MouseMoved,
                    pos,
                    CGMouseButton::Left,
                ) {
                    event.post(CGEventTapLocation::HID);
                }

                self.current_pos = pos;

                // Variable delay to simulate acceleration/deceleration
                let delay_ms = if i < steps / 3 || i > steps - (steps / 3) {
                    rng.gen_range(8..15) // Slower at start and end
                } else {
                    rng.gen_range(2..6) // Faster in the middle
                };

                sleep(Duration::from_millis(delay_ms)).await;
            }

            // Final snap to exact target to correct overshoot
            self.current_pos = CGPoint::new(target_x, target_y);
            if let Ok(event) = CGEvent::new_mouse_event(self.source.clone(), CGEventType::MouseMoved, self.current_pos, CGMouseButton::Left) {
                event.post(CGEventTapLocation::HID);
            }
            sleep(Duration::from_millis(rng.gen_range(100..300))).await; // Rest after movement
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.current_pos = (target_x, target_y);
            sleep(Duration::from_millis(10)).await;
        }
    }

    /// Performs a human-like click with randomized down-to-up delay.
    pub async fn click(&self) {
        #[cfg(target_os = "macos")]
        {
            let mut rng = rand::thread_rng();

            // Mouse Down
            if let Ok(down) = CGEvent::new_mouse_event(self.source.clone(), CGEventType::LeftMouseDown, self.current_pos, CGMouseButton::Left) {
                down.post(CGEventTapLocation::HID);
            }

            // Neuromuscular delay between press and release
            sleep(Duration::from_millis(rng.gen_range(30..80))).await;

            // Mouse Up
            if let Ok(up) = CGEvent::new_mouse_event(self.source.clone(), CGEventType::LeftMouseUp, self.current_pos, CGMouseButton::Left) {
                up.post(CGEventTapLocation::HID);
            }

            // Post-click cognitive delay
            sleep(Duration::from_millis(rng.gen_range(200..500))).await;
        }
    }

    /// Types a string with biologically plausible cadence.
    pub async fn type_string(&self, text: &str) {
        let mut rng = rand::thread_rng();
        
        for ch in text.chars() {
            // NOTE: For true CoreGraphics keyboard injection, we map chars to CGKeyCode.
            // For simplicity in this architectural demo, we shell out to osascript 
            // but inject our stochastic delays between EACH character, rather than the whole string.
            
            #[cfg(target_os = "macos")]
            {
                // Send individual keystroke
                let escaped_ch = ch.to_string().replace("\"", "\\\"");
                let _ = std::process::Command::new("osascript")
                    .arg("-e").arg(format!("tell application \"System Events\" to keystroke \"{}\"", escaped_ch))
                    .output();
            }
            
            // Stochastic typing delay (simulates finding the key, pressing, releasing)
            let base_delay = rng.gen_range(50..120);
            
            // Simulate cognitive pauses (e.g., spacebar or punctuation causes a slightly longer pause)
            let cognitive_modifier = if ch == ' ' || ch == '.' || ch == ',' {
                rng.gen_range(50..250)
            } else {
                0
            };
            
            sleep(Duration::from_millis(base_delay + cognitive_modifier)).await;
        }
    }
}
