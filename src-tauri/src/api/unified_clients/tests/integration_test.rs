#[cfg(test)]
mod integration_tests {
    use crate::api::unified_clients::{
        ApiClient, ApiClientConfig, PaginationParams,
        CanvasApiClient, DiscourseApiClient,
        create_canvas_client, create_discourse_client,
        CanvasClientAdapter, DiscourseClientAdapter,
    };
    use mockito::{mock, server_url};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_canvas_client() {
        // Setup mock server
        let _m = mock("GET", "/api/v1/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"123","name":"Test User","email":"test@example.com","login_id":"testuser"}"#)
            .create();
            
        // Create client
        let client = create_canvas_client(&server_url(), "test_key").unwrap();
        
        // Test API call
        let user = client.get_user("123").await.unwrap();
        
        // Verify response
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
    }
    
    #[tokio::test]
    async fn test_discourse_client() {
        // Setup mock server
        let _m = mock("GET", "/users/testuser.json")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("api_key".into(), "test_key".into()),
                mockito::Matcher::UrlEncoded("api_username".into(), "test_admin".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"user":{"id":123,"username":"testuser","name":"Test User","email":"test@example.com"}}"#)
            .create();
            
        // Create client
        let client = create_discourse_client(&server_url(), "test_key", "test_admin").unwrap();
        
        // Test API call
        let user = client.get_user_by_username("testuser").await.unwrap();
        
        // Verify response
        assert_eq!(user.username, "testuser");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_canvas_client_adapter() {
        // Setup mock server
        let _m = mock("GET", "/api/v1/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"123","name":"Test User","email":"test@example.com","login_id":"testuser"}"#)
            .create();
            
        // Create client
        let client = create_canvas_client(&server_url(), "test_key").unwrap();
        let adapter = CanvasClientAdapter::new(client);
        
        // Get the underlying client
        let unified_client = adapter.get_client();
        
        // Test API call
        let user = unified_client.get_user("123").await.unwrap();
        
        // Verify response
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
    }
    
    #[tokio::test]
    async fn test_discourse_client_adapter() {
        // Setup mock server
        let _m = mock("GET", "/users/testuser.json")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("api_key".into(), "test_key".into()),
                mockito::Matcher::UrlEncoded("api_username".into(), "test_admin".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"user":{"id":123,"username":"testuser","name":"Test User","email":"test@example.com"}}"#)
            .create();
            
        // Create client
        let client = create_discourse_client(&server_url(), "test_key", "test_admin").unwrap();
        let adapter = DiscourseClientAdapter::new(client);
        
        // Get the underlying client
        let unified_client = adapter.get_client();
        
        // Test API call
        let user = unified_client.get_user_by_username("testuser").await.unwrap();
        
        // Verify response
        assert_eq!(user.username, "testuser");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_pagination() {
        // Setup mock server
        let _m = mock("GET", "/api/v1/courses")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("page".into(), "1".into()),
                mockito::Matcher::UrlEncoded("per_page".into(), "10".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"items":[{"id":"1","name":"Course 1"},{"id":"2","name":"Course 2"}],"total":2,"page":1,"per_page":10,"total_pages":1,"has_next":false,"has_prev":false}"#)
            .create();
            
        // Create client
        let config = ApiClientConfig {
            base_url: server_url(),
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        
        let client = crate::api::unified_clients::create_api_client(config).unwrap();
        
        // Create pagination params
        let pagination = PaginationParams {
            page: Some(1),
            per_page: Some(10),
            cursor: None,
        };
        
        // Test API call
        let response = client.get_paginated::<serde_json::Value>("/api/v1/courses", &pagination, None).await.unwrap();
        
        // Verify response
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.total, Some(2));
        assert_eq!(response.page, Some(1));
        assert_eq!(response.per_page, Some(10));
        assert_eq!(response.total_pages, Some(1));
        assert_eq!(response.has_next, false);
        assert_eq!(response.has_prev, false);
    }
    
    #[tokio::test]
    async fn test_error_handling() {
        // Setup mock server
        let _m = mock("GET", "/api/v1/users/not_found")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"User not found"}"#)
            .create();
            
        // Create client
        let client = create_canvas_client(&server_url(), "test_key").unwrap();
        
        // Test API call
        let result = client.get_user("not_found").await;
        
        // Verify error
        assert!(result.is_err());
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => {
                match e {
                    crate::api::unified_clients::ApiError::ApiError { status_code, message } => {
                        assert_eq!(status_code, reqwest::StatusCode::NOT_FOUND);
                        assert!(message.contains("User not found"));
                    },
                    _ => panic!("Expected ApiError::ApiError"),
                }
            }
        }
    }
    
    #[tokio::test]
    async fn test_retry_mechanism() {
        // Setup mock server to return 429 (rate limit) first, then 200
        let _m = mock("GET", "/api/v1/users/123")
            .with_status(429)
            .with_header("content-type", "application/json")
            .with_header("retry-after", "1")
            .with_body(r#"{"error":"Rate limit exceeded"}"#)
            .create();
            
        let _m2 = mock("GET", "/api/v1/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"123","name":"Test User","email":"test@example.com","login_id":"testuser"}"#)
            .create();
            
        // Create client with retry
        let config = ApiClientConfig {
            base_url: server_url(),
            api_key: "test_key".to_string(),
            max_retries: 1, // Only retry once
            ..Default::default()
        };
        
        let client = CanvasApiClient::new(&config.base_url, &config.api_key).unwrap();
        
        // Test API call
        let user = client.get_user("123").await.unwrap();
        
        // Verify response
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
    }
}
