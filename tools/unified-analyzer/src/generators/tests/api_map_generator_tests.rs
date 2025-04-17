#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::collections::HashMap;
    use std::fs;
    
    use crate::generators::ApiMapGenerator;
    use crate::output_schema::{UnifiedAnalysisOutput, ApiInfo, ApiEndpointInfo};

    #[test]
    fn test_api_map_generator_initialization() {
        let generator = ApiMapGenerator::new();
        assert!(generator.generate(&create_test_output(), &PathBuf::from("./test_output")).is_err());
    }

    #[test]
    fn test_to_anchor_method() {
        let generator = ApiMapGenerator::new();
        
        // Test basic conversion
        assert_eq!(generator.to_anchor("Test String"), "test-string");
        
        // Test with special characters
        assert_eq!(generator.to_anchor("Test & String"), "test-and-string");
        
        // Test with non-alphanumeric characters
        assert_eq!(generator.to_anchor("Test@#$%^&*()"), "testandstring");
    }

    #[test]
    fn test_path_to_id_method() {
        let generator = ApiMapGenerator::new();
        
        // Test basic conversion
        assert_eq!(generator.path_to_id("/api/v1/users"), "_api_v1_users");
        
        // Test with parameters
        assert_eq!(generator.path_to_id("/api/v1/users/{id}"), "_api_v1_users_id_");
        
        // Test with special characters
        assert_eq!(generator.path_to_id("/api/v1/users.json"), "_api_v1_users_json");
    }

    #[test]
    fn test_sanitize_id_method() {
        let generator = ApiMapGenerator::new();
        
        // Test basic conversion
        assert_eq!(generator.sanitize_id("Test String"), "Test_String");
        
        // Test with special characters
        assert_eq!(generator.sanitize_id("Test@#$%^&*()"), "Test");
    }

    #[test]
    fn test_generate_markdown() {
        let generator = ApiMapGenerator::new();
        let output = create_test_output();
        
        let result = generator.generate_markdown(&output);
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        assert!(markdown.contains("# API Map"));
        assert!(markdown.contains("## Table of Contents"));
        assert!(markdown.contains("## API Flow Diagram"));
    }

    #[test]
    fn test_template_loading() {
        // Create a temporary template file
        let temp_dir = tempfile::tempdir().unwrap();
        let template_path = temp_dir.path().join("api_map_template.html");
        
        fs::write(&template_path, r#"<!DOCTYPE html>
<html>
<head>
    <title>API Map</title>
</head>
<body>
    <!-- METHOD_FILTERS_PLACEHOLDER -->
    <!-- CATEGORIES_PLACEHOLDER -->
</body>
</html>"#).unwrap();
        
        // Test loading the template
        let template = fs::read_to_string(&template_path).unwrap();
        assert!(template.contains("<!-- METHOD_FILTERS_PLACEHOLDER -->"));
        assert!(template.contains("<!-- CATEGORIES_PLACEHOLDER -->"));
    }

    // Helper function to create a test output
    fn create_test_output() -> UnifiedAnalysisOutput {
        let mut output = UnifiedAnalysisOutput::default();
        
        // Add some API endpoints
        let mut endpoints = Vec::new();
        
        endpoints.push(ApiEndpointInfo {
            path: "/api/v1/users".to_string(),
            http_method: "GET".to_string(),
            controller: Some("UsersController".to_string()),
            action: Some("index".to_string()),
            description: Some("Get all users".to_string()),
            request_params: vec![],
            response_format: Some("JSON".to_string()),
            auth_required: true,
            rate_limited: false,
            category: Some("Users".to_string()),
        });
        
        endpoints.push(ApiEndpointInfo {
            path: "/api/v1/users/{id}".to_string(),
            http_method: "GET".to_string(),
            controller: Some("UsersController".to_string()),
            action: Some("show".to_string()),
            description: Some("Get a specific user".to_string()),
            request_params: vec!["id".to_string()],
            response_format: Some("JSON".to_string()),
            auth_required: true,
            rate_limited: false,
            category: Some("Users".to_string()),
        });
        
        output.api = ApiInfo {
            endpoints,
            base_url: Some("https://api.example.com".to_string()),
            version: Some("1.0".to_string()),
        };
        
        output
    }
}
