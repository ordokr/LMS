#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        ApiClientConfig, CanvasApiClient, DiscourseApiClient,
        CanvasApi, DiscourseApi
    };
    use std::collections::HashMap;
    use mockito::{mock, server_url};
    use serde_json::json;

    #[tokio::test]
    async fn test_canvas_api_client() {
        let mock_server = server_url();
        
        // Mock the Canvas API response for getting a user
        let _m = mock("GET", "/api/v1/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "id": 123,
                    "name": "John Doe",
                    "email": "john.doe@example.com",
                    "login_id": "jdoe",
                    "avatar_url": "https://example.com/avatar.jpg"
                }
            "#)
            .create();
        
        // Create a Canvas API client
        let config = ApiClientConfig {
            base_url: format!("{}/api/v1", mock_server),
            api_key: Some("test_api_key".to_string()),
            api_token: Some("test_api_token".to_string()),
            username: None,
            password: None,
            timeout_seconds: 30,
            user_agent: "Canvas API Client Test".to_string(),
        };
        
        let client = CanvasApiClient::new(config).unwrap();
        
        // Test getting a user
        let user = client.get_user("123").await.unwrap();
        
        assert_eq!(user["id"].as_i64().unwrap(), 123);
        assert_eq!(user["name"].as_str().unwrap(), "John Doe");
        assert_eq!(user["email"].as_str().unwrap(), "john.doe@example.com");
        assert_eq!(user["login_id"].as_str().unwrap(), "jdoe");
        assert_eq!(user["avatar_url"].as_str().unwrap(), "https://example.com/avatar.jpg");
    }
    
    #[tokio::test]
    async fn test_discourse_api_client() {
        let mock_server = server_url();
        
        // Mock the Discourse API response for getting a user
        let _m = mock("GET", "/users/johndoe.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "user": {
                        "id": 123,
                        "username": "johndoe",
                        "name": "John Doe",
                        "avatar_template": "/user_avatar/discourse.example.com/johndoe/{size}/123_2.png",
                        "email": "john.doe@example.com",
                        "trust_level": 2
                    }
                }
            "#)
            .create();
        
        // Create a Discourse API client
        let config = ApiClientConfig {
            base_url: mock_server,
            api_key: Some("test_api_key".to_string()),
            api_token: None,
            username: Some("system".to_string()),
            password: None,
            timeout_seconds: 30,
            user_agent: "Discourse API Client Test".to_string(),
        };
        
        let client = DiscourseApiClient::new(config).unwrap();
        
        // Test getting a user by username
        let user = client.get_user_by_username("johndoe").await.unwrap();
        
        assert_eq!(user.id, 123);
        assert_eq!(user.username, "johndoe");
        assert_eq!(user.name, Some("John Doe".to_string()));
        assert_eq!(user.trust_level, Some(2));
    }
    
    #[tokio::test]
    async fn test_canvas_structured_data() {
        let mock_server = server_url();
        
        // Mock the Canvas API response for getting courses
        let _m = mock("GET", "/api/v1/courses")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                [
                    {
                        "id": "123",
                        "name": "Introduction to Computer Science",
                        "course_code": "CS101",
                        "workflow_state": "available"
                    },
                    {
                        "id": "456",
                        "name": "Advanced Programming",
                        "course_code": "CS201",
                        "workflow_state": "available"
                    }
                ]
            "#)
            .create();
        
        // Create a Canvas API client
        let config = ApiClientConfig {
            base_url: format!("{}/api/v1", mock_server),
            api_key: Some("test_api_key".to_string()),
            api_token: Some("test_api_token".to_string()),
            username: None,
            password: None,
            timeout_seconds: 30,
            user_agent: "Canvas API Client Test".to_string(),
        };
        
        let client = CanvasApiClient::new(config).unwrap();
        
        // Test getting courses
        let courses = client.get_courses(None).await.unwrap();
        
        assert_eq!(courses.len(), 2);
        assert_eq!(courses[0].id, "123");
        assert_eq!(courses[0].name, "Introduction to Computer Science");
        assert_eq!(courses[0].course_code, "CS101");
        assert_eq!(courses[1].id, "456");
        assert_eq!(courses[1].name, "Advanced Programming");
        assert_eq!(courses[1].course_code, "CS201");
    }
    
    #[tokio::test]
    async fn test_discourse_structured_data() {
        let mock_server = server_url();
        
        // Mock the Discourse API response for getting categories
        let _m = mock("GET", "/categories.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"
                {
                    "category_list": {
                        "categories": [
                            {
                                "id": 1,
                                "name": "General",
                                "slug": "general",
                                "topic_count": 25,
                                "post_count": 100,
                                "description": "General discussions"
                            },
                            {
                                "id": 2,
                                "name": "Support",
                                "slug": "support",
                                "topic_count": 15,
                                "post_count": 75,
                                "description": "Support discussions"
                            }
                        ]
                    }
                }
            "#)
            .create();
        
        // Create a Discourse API client
        let config = ApiClientConfig {
            base_url: mock_server,
            api_key: Some("test_api_key".to_string()),
            api_token: None,
            username: Some("system".to_string()),
            password: None,
            timeout_seconds: 30,
            user_agent: "Discourse API Client Test".to_string(),
        };
        
        let client = DiscourseApiClient::new(config).unwrap();
        
        // Test getting categories
        let categories = client.get_categories().await.unwrap();
        
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].id, 1);
        assert_eq!(categories[0].name, "General");
        assert_eq!(categories[0].slug, "general");
        assert_eq!(categories[0].topic_count, Some(25));
        assert_eq!(categories[1].id, 2);
        assert_eq!(categories[1].name, "Support");
        assert_eq!(categories[1].slug, "support");
        assert_eq!(categories[1].topic_count, Some(15));
    }
}
