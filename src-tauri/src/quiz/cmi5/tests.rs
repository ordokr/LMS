#[cfg(test)]
mod tests {
    use super::*;
    use crate::quiz::cmi5::{Cmi5Service, LaunchService, LaunchMode};
    use std::path::Path;
    use std::sync::Arc;
    use tokio::runtime::Runtime;

    #[test]
    fn test_cmi5_service_creation() {
        let launch_service = Arc::new(LaunchService::new(
            "https://example.com/lrs",
            "https://example.com/lrs/auth"
        ));
        
        let cmi5_service = Cmi5Service::new(
            "https://example.com/lrs",
            Some("test:test"),
            launch_service
        );
        
        assert!(cmi5_service.is_ok());
    }
    
    #[test]
    fn test_statement_creation() {
        // Test creating an initialized statement
        let statement = create_initialized_statement(
            "test@example.com",
            "https://example.com/activity/1",
            "reg-123"
        );
        
        assert_eq!(statement.verb.id, "http://adlnet.gov/expapi/verbs/initialized");
        assert_eq!(statement.context.registration, "reg-123");
        
        // Test creating a completed statement
        let statement = create_completed_statement(
            "test@example.com",
            "https://example.com/activity/1",
            "reg-123"
        );
        
        assert_eq!(statement.verb.id, "http://adlnet.gov/expapi/verbs/completed");
        assert!(statement.result.is_some());
        assert_eq!(statement.result.unwrap().completion, Some(true));
        
        // Test creating a passed statement
        let score = Some(Cmi5Score::percentage(85.0));
        let statement = create_passed_statement(
            "test@example.com",
            "https://example.com/activity/1",
            "reg-123",
            score.clone()
        );
        
        assert_eq!(statement.verb.id, "http://adlnet.gov/expapi/verbs/passed");
        assert!(statement.result.is_some());
        let result = statement.result.unwrap();
        assert_eq!(result.success, Some(true));
        assert!(result.score.is_some());
        assert_eq!(result.score.unwrap().scaled, 0.85);
    }
    
    #[test]
    fn test_launch_parameters() {
        let rt = Runtime::new().unwrap();
        
        rt.block_on(async {
            let launch_service = Arc::new(LaunchService::new(
                "https://example.com/lrs",
                "https://example.com/lrs/auth"
            ));
            
            let params = launch_service.create_launch_parameters(
                "course-123",
                "au-456",
                "test@example.com",
                "reg-789",
                LaunchMode::Normal
            ).await;
            
            assert!(params.is_ok());
            let params = params.unwrap();
            
            assert_eq!(params.endpoint, "https://example.com/lrs");
            assert!(params.actor.contains("test@example.com"));
            assert_eq!(params.registration, "reg-789");
            assert!(params.activity_id.contains("course-123"));
            assert!(params.activity_id.contains("au-456"));
        });
    }
}
