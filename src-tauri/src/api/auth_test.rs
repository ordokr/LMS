#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::sync::Arc;
    use crate::db::user_repository::MockUserRepository;
    use crate::models::user::{User, UserRole, LoginRequest, RegisterRequest};
    use crate::AppState;
    
    #[tokio::test]
    async fn test_login_success() {
        // Create mock repository
        let mut mock_repo = MockUserRepository::new();
        
        // Password hash for "password123"
        let password_hash = "$argon2id$v=19$m=16,t=2,p=1$c29tZXNhbHQ$gMRTGFRxrhOcx8u4fNz4WA";
        
        // Create test user
        let test_user = User {
            id: "user123".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_hash.to_string(),
            role: "user".to_string(),
            // ... other fields with default values
        };
        
        // Set up mock behavior
        mock_repo.expect_get_user_by_email()
            .with(eq("test@example.com"))
            .times(1)
            .returning(move |_| Ok(Some(test_user.clone())));
        
        // Create test request
        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        // Create app state with JWT secret
        let app_state = AppState {
            jwt_secret: "test_secret".as_bytes().to_vec(),
            // ... other app state fields
        };
        
        // Create States for function parameters
        let repo_state = tauri::State::new(Arc::new(mock_repo) as Arc<dyn UserRepository + Send + Sync>);
        let app_state = tauri::State::new(app_state);
        
        // Call function
        let result = login_user(login_request, repo_state, app_state).await;
        
        // Assert
        assert!(result.is_ok(), "Login should succeed");
        let auth_response = result.unwrap();
        assert_eq!(auth_response.user.id, "user123");
        assert_eq!(auth_response.user.email, "test@example.com");
        assert!(!auth_response.token.is_empty(), "Token should be provided");
    }
    
    #[tokio::test]
    async fn test_login_invalid_password() {
        // Create mock repository
        let mut mock_repo = MockUserRepository::new();
        
        // Password hash for "password123"
        let password_hash = "$argon2id$v=19$m=16,t=2,p=1$c29tZXNhbHQ$gMRTGFRxrhOcx8u4fNz4WA";
        
        // Create test user
        let test_user = User {
            id: "user123".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_hash.to_string(),
            role: "user".to_string(),
            // ... other fields with default values
        };
        
        // Set up mock behavior
        mock_repo.expect_get_user_by_email()
            .with(eq("test@example.com"))
            .times(1)
            .returning(move |_| Ok(Some(test_user.clone())));
        
        // Create test request with wrong password
        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrong_password".to_string(),
        };
        
        // Create app state with JWT secret
        let app_state = AppState {
            jwt_secret: "test_secret".as_bytes().to_vec(),
            // ... other app state fields
        };
        
        // Create States for function parameters
        let repo_state = tauri::State::new(Arc::new(mock_repo) as Arc<dyn UserRepository + Send + Sync>);
        let app_state = tauri::State::new(app_state);
        
        // Call function
        let result = login_user(login_request, repo_state, app_state).await;
        
        // Assert
        assert!(result.is_err(), "Login should fail with wrong password");
        assert_eq!(result.unwrap_err(), "Invalid email or password");
    }
}