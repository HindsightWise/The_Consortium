#[cfg(test)]
mod internal_tests {
    use crate::core::state::CompanyState;
    use crate::core::soul::Soul;
    use crate::core::orchestrator::Orchestrator;

    #[test]
    fn test_soul_creation() {
        let soul = Soul::new("TestAgent", "A test entity.");
        assert_eq!(soul.name, "TestAgent");
        assert_eq!(soul.voice_weight, 1.0);
        assert!(soul.mood.valence == 0.0);
    }

    #[test]
    fn test_state_serialization() {
        let state = CompanyState::new("Test Goal");
        let json = serde_json::to_string(&state);
        assert!(json.is_ok());
    }

    #[test]
    fn test_orchestrator_initialization() {
        // This validates that the core dependency injection and agent registry loading 
        // logic is sound and doesn't panic on startup.
        let orchestrator = Orchestrator::new("Test Mission");
        assert!(orchestrator.is_ok());
    }
}
