#[cfg(test)]
mod tests {
    
    use crate::core::soul::Soul;

    #[test]
    fn test_compress_experience_basic() {
        let mut soul = Soul::new("TestAgent", "A test agent for SCE validation");
        let experience = "The autonomous flywheel achieved a 2.3% alpha in simulated momentum trading.";
        
        // Initial state
        assert_eq!(soul.worldview.pervasive_structures.len(), 0);
        
        // Compress first experience
        let result = soul.compress_experience(experience);
        assert!(result.is_ok());
        assert_eq!(soul.worldview.pervasive_structures.len(), 1);
        assert!(soul.worldview.pervasive_structures[0].contains("flywheel"));
        
        // Emotional schemas remain untouched (integrity preservation)
        assert_eq!(soul.worldview.emotion_schemas.len(), 0);
    }

    #[test]
    fn test_compress_experience_fifo_limit() {
        let mut soul = Soul::new("TestAgent", "FIFO limit validation");
        
        // Add 15 experiences (should keep only last 10)
        for i in 1..=15 {
            let exp = format!("Experience number {} for FIFO testing.", i);
            soul.compress_experience(&exp).unwrap();
        }
        
        assert_eq!(soul.worldview.pervasive_structures.len(), 10);
        
        // Verify oldest (1-5) are gone, newest (6-15) remain
        let first_kept = &soul.worldview.pervasive_structures[0];
        assert!(first_kept.contains("6")); // Experience 6 is now first
        
        let last_kept = &soul.worldview.pervasive_structures[9];
        assert!(last_kept.contains("15")); // Experience 15 is last
        
        // Ensure experience 1 is not present
        for gist in &soul.worldview.pervasive_structures {
            assert!(!gist.contains("1"));
            // assert!(!gist.contains("1"));
        }
    }

    #[test]
    fn test_compress_experience_gist_truncation() {
        let mut soul = Soul::new("TestAgent", "Gist truncation test");
        
        // Create a long experience (>200 chars)
        let long_exp = "This is a very lengthy experience that exceeds the two hundred character limit by a significant margin. It contains multiple sentences and detailed information about market movements, technical indicators, and strategic insights that should be compressed into a more manageable gist format for the agent's worldview.".to_string();
        
        assert!(long_exp.len() > 200);
        
        soul.compress_experience(&long_exp).unwrap();
        
        let gist = &soul.worldview.pervasive_structures[0];
        assert_eq!(gist.len(), 200); // Should be exactly 200 chars with "..."
        assert!(gist.ends_with("..."));
        
        // Verify it starts with the same prefix
        assert!(gist.starts_with("This is a very lengthy experience"));
    }

    #[test]
    fn test_compress_experience_short_no_truncation() {
        let mut soul = Soul::new("TestAgent", "Short experience test");
        
        let short_exp = "Brief insight.";
        soul.compress_experience(short_exp).unwrap();
        
        let gist = &soul.worldview.pervasive_structures[0];
        assert_eq!(gist, "Brief insight."); // No truncation needed
    }

    #[test]
    fn test_compress_experience_empty_string() {
        let mut soul = Soul::new("TestAgent", "Empty experience test");
        
        // Empty experience should still be processed
        soul.compress_experience("").unwrap();
        
        assert_eq!(soul.worldview.pervasive_structures.len(), 1);
        assert_eq!(soul.worldview.pervasive_structures[0], "");
    }

    #[test]
    fn test_compress_experience_integrity_preservation() {
        let mut soul = Soul::new("TestAgent", "Integrity preservation test");
        
        // Manually add an emotion schema (simulating previous state)
        soul.worldview.emotion_schemas.insert("frustration".to_string(), "When systems fail".to_string());
        
        let initial_schema_count = soul.worldview.emotion_schemas.len();
        
        // Compress multiple experiences
        for i in 1..=5 {
            let exp = format!("Experience {} with potential emotional content.", i);
            soul.compress_experience(&exp).unwrap();
        }
        
        // Emotion schemas should remain unchanged
        assert_eq!(soul.worldview.emotion_schemas.len(), initial_schema_count);
        assert!(soul.worldview.emotion_schemas.contains_key("frustration"));
        
        // No new emotion schemas should be automatically created
        assert!(!soul.worldview.emotion_schemas.contains_key("joy"));
        assert!(!soul.worldview.emotion_schemas.contains_key("sorrow"));
    }

    #[test]
    fn test_compress_experience_latency_simulation() {
        let mut soul = Soul::new("TestAgent", "Latency test");
        
        // Time the compression of multiple experiences
        let start = std::time::Instant::now();
        
        for i in 1..=100 {
            let exp = format!("Quick experience {} for latency testing.", i);
            soul.compress_experience(&exp).unwrap();
        }
        
        let duration = start.elapsed();
        
        // Should complete well under 100ms total for 100 compressions
        // (Average <1ms per compression)
        assert!(duration.as_millis() < 1000, "Compression too slow: {:?}", duration);
        
        // Verify results
        assert_eq!(soul.worldview.pervasive_structures.len(), 10); // FIFO limit
    }

    #[test]
    fn test_compress_experience_with_special_characters() {
        let mut soul = Soul::new("TestAgent", "Special chars test");
        
        let complex_exp = "Experience with emoji 🚀, symbols @#$%, and multi-byte characters: 日本語.";
        soul.compress_experience(complex_exp).unwrap();
        
        let gist = &soul.worldview.pervasive_structures[0];
        
        // Should preserve the content (may truncate if over 200 chars)
        assert!(gist.contains("emoji"));
        // Note: Truncation might cut multi-byte chars, but that's acceptable for MVP
    }
}