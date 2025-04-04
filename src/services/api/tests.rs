#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use crate::config::{Config, CanvasConfig, DiscourseConfig};

    #[tokio::test]
    async fn test_get_courses() {
        // Start a mock server
        let mut server = Server::new();
        
        // Configure the mock response
        let mock = server
            .mock("GET", "/courses")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[
                {"id": 1, "name": "Test Course 1", "workflow_state": "available"},
                {"id": 2, "name": "Test Course 2", "workflow_state": "available"}
            ]"#)
            .create();
        
        // Create a config with our mock server URL
        let mut config = Config::default();
        config.canvas.api_url = server.url();
        
        // Create API client with our config
        let client = ApiClient::new(config);
        
        // Call the method under test
        let result = client.get_courses().await;
        
        // Verify the result
        assert!(result.is_ok());
        let courses = result.unwrap();
        assert_eq!(courses.len(), 2);
        assert_eq!(courses[0].id, 1);
        assert_eq!(courses[0].name, Some("Test Course 1".to_string()));
        
        // Verify that the mock was called
        mock.assert();
    }
}