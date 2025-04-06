#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt;
    use crate::models::user::User;
    use crate::db::Database;
    use crate::AppState;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use std::{env, sync::Arc};
    use tower::ServiceExt;
    use mockall::predicate::*;
    use mockall::mock;

    // Mock database for testing
    mock! {
        Database {}
        
        async fn authenticate_user(&self, username: &str, password: &str) -> Option<User>;
        async fn check_user_exists(&self, user_id: &str) -> bool;
    }

    #[tokio::test]
    async fn test_login_success() {
        env::set_var("JWT_SECRET", "test_secret");
        
        // Create mock database
        let mut mock_db = MockDatabase::new();
        
        // Set up expectations
        mock_db
            .expect_authenticate_user()
            .with(eq("test@example.com"), eq("password"))
            .returning(|_, _| {
                Some(User {
                    id: "user123".to_string(),
                    username: "test@example.com".to_string(),
                    role: "teacher".to_string(),
                    canvas_id: "canvas456".to_string(),
                    // Add other required fields
                })
            });
        
        // Create app state with mock database
        let state = Arc::new(AppState {
            db: Arc::new(mock_db),
            // Add other required fields
        });
        
        // Create router with login route
        let app = Router::new()
            .route("/api/auth/login", post(handlers::login))
            .with_state(state);
        
        // Create login request
        let request = Request::builder()
            .uri("/api/auth/login")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"username":"test@example.com","password":"password"}"#))
            .unwrap();
        
        // Execute request
        let response = app.oneshot(request).await.unwrap();
        
        // Assert successful login
        assert_eq!(response.status(), StatusCode::OK);
        
        // Parse response body
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let login_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert!(login_response.get("token").is_some());
        assert_eq!(login_response["user_id"], "user123");
        assert_eq!(login_response["role"], "teacher");
    }
    
    #[tokio::test]
    async fn test_auth_middleware() {
        env::set_var("JWT_SECRET", "test_secret");
        
        // Generate a test token
        let token = jwt::generate_token("user123", "teacher", "canvas456").unwrap();
        
        // Create mock database
        let mut mock_db = MockDatabase::new();
        
        // Set up expectations for user check
        mock_db
            .expect_check_user_exists()
            .with(eq("user123"))
            .returning(|_| true);
        
        // Create app state with mock database
        let state = Arc::new(AppState {
            db: Arc::new(mock_db),
            // Add other required fields
        });
        
        // Create router with protected route
        let app = Router::new()
            .route(
                "/api/protected",
                get(|| async { "Protected" })
                    .route_layer(from_fn_with_state(state.clone(), middleware::auth_middleware))
            )
            .with_state(state);
        
        // Create request with valid token
        let request = Request::builder()
            .uri("/api/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        
        // Execute request
        let response = app.oneshot(request).await.unwrap();
        
        // Assert successful authentication
        assert_eq!(response.status(), StatusCode::OK);
    }
}