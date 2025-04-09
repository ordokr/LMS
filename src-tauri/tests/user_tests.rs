#[cfg(test)]
mod tests {
    use crate::api::auth::register_user;
    use crate::api::users::{
        get_user_profile, update_user_profile, get_user_preferences, 
        update_user_preferences, get_user_integration_settings, update_user_integration_settings
    };
    use crate::models::user::{RegisterRequest, UserProfileUpdate, UserRole};
    use crate::db::test_utils::{create_test_db_pool, clean_test_db};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_profile_flow() {
        // Set up test DB
        let pool = create_test_db_pool().await;
        clean_test_db(&pool).await;
        
        // Create repositories
        let user_repo = Arc::new(crate::db::user_repository::SqliteUserRepository::new(pool.clone()));
        
        // 1. Register a user
        let register_req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            role: Some(UserRole::Student),
        };
        
        let user = register_user(
            register_req, 
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to register user");
        
        // 2. Get user profile
        let profile = get_user_profile(
            user.id.clone(), 
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to get user profile");
        
        // Verify profile data
        assert_eq!(profile.user_id, user.id);
        assert_eq!(profile.first_name, "Test");
        assert_eq!(profile.last_name, "User");
        assert_eq!(profile.email, "test@example.com");
        assert_eq!(profile.bio, "");
        assert_eq!(profile.avatar_url, "");
        
        // 3. Update user profile
        let profile_update = UserProfileUpdate {
            first_name: None,
            last_name: None,
            email: None,
            bio: Some("This is my test bio".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };
        
        let updated_profile = update_user_profile(
            user.id.clone(),
            profile_update,
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to update user profile");
        
        // Verify updated profile
        assert_eq!(updated_profile.bio, "This is my test bio");
        assert_eq!(updated_profile.avatar_url, "https://example.com/avatar.jpg");
        
        // 4. Get user preferences
        let preferences = get_user_preferences(
            user.id.clone(),
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to get user preferences");
        
        // Verify preferences exist (just check structure)
        assert!(preferences.get("notifications").is_some());
        assert!(preferences.get("display").is_some());
        
        // 5. Update user preferences
        let new_preferences = serde_json::json!({
            "notifications": {
                "email": false,
                "push": true,
                "discussion_replies": false,
                "assignment_grades": true
            },
            "display": {
                "theme": "dark",
                "font_size": "large",
                "sidebar_collapsed": true
            }
        });
        
        let updated_preferences = update_user_preferences(
            user.id.clone(),
            new_preferences.clone(),
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to update user preferences");
        
        // Verify updated preferences
        assert_eq!(updated_preferences["display"]["theme"], "dark");
        assert_eq!(updated_preferences["notifications"]["email"], false);
        
        // 6. Get integration settings
        let settings = get_user_integration_settings(
            user.id.clone(),
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to get user integration settings");
        
        // Verify settings exist
        assert!(settings.get("discourse").is_some());
        assert!(settings.get("canvas").is_some());
        
        // 7. Update integration settings
        let new_settings = serde_json::json!({
            "discourse": {
                "auto_sync": false,
                "notify_on_replies": false
            },
            "canvas": {
                "auto_sync": true,
                "real_time_updates": true
            }
        });
        
        let updated_settings = update_user_integration_settings(
            user.id.clone(),
            new_settings.clone(),
            tauri::State::new(user_repo.clone())
        ).await.expect("Failed to update user integration settings");
        
        // Verify updated settings
        assert_eq!(updated_settings["discourse"]["auto_sync"], false);
        assert_eq!(updated_settings["canvas"]["real_time_updates"], true);
    }

    #[tokio::test]
    async fn test_auth_password_handling() {
        // Import the required functions
        use crate::api::auth::{hash_password, verify_password};
        
        // Test that password hashing produces different hashes for the same password
        let password = "secure_password_123";
        let hash1 = hash_password(password).expect("Password hashing should succeed");
        let hash2 = hash_password(password).expect("Password hashing should succeed");

        // Hashes should be different due to different salts
        assert_ne!(hash1, hash2);

        // But verification should work for both
        assert!(verify_password(password, &hash1));
        assert!(verify_password(password, &hash2));

        // Wrong password should fail verification
        assert!(!verify_password("wrong_password", &hash1));
    }

    #[tokio::test]
    async fn test_token_validation_edge_cases() {
        // Import required functions
        use crate::api::auth::{generate_token, verify_token};
        use std::env;
        
        // Set up test environment
        env::set_var("JWT_SECRET", "test_secret_key_for_auth_tests");
        
        // Test completely invalid token format
        let invalid_token = "not.a.valid.token.format";
        let result = verify_token(invalid_token);
        assert!(result.is_err());

        // Test token with valid format but invalid signature
        let forged_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = verify_token(forged_token);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_role_based_permissions() {
        // Import required functions and types
        use crate::api::auth::{generate_token, verify_token};
        use crate::models::user::UserRole;
        use std::env;
        use uuid::Uuid;
        
        // Set up test environment
        env::set_var("JWT_SECRET", "test_secret_key_for_auth_tests");
        
        // Generate admin token
        let admin_id = Uuid::new_v4().to_string();
        let admin_token = generate_token(
            &admin_id,
            &UserRole::Admin.to_string()
        ).expect("Token generation should succeed");
        
        // Generate student token
        let student_id = Uuid::new_v4().to_string();
        let student_token = generate_token(
            &student_id,
            &UserRole::Student.to_string()
        ).expect("Token generation should succeed");
        
        // Verify both tokens
        let admin_claims = verify_token(&admin_token).expect("Admin token should be valid");
        let student_claims = verify_token(&student_token).expect("Student token should be valid");
        
        // Check that roles are preserved correctly
        assert_eq!(admin_claims.role, UserRole::Admin.to_string());
        assert_eq!(student_claims.role, UserRole::Student.to_string());
        
        // Test permission checks
        assert!(has_permission(&admin_claims, "create_course"));
        assert!(!has_permission(&student_claims, "create_course"));
        
        assert!(has_permission(&admin_claims, "view_course"));
        assert!(has_permission(&student_claims, "view_course"));
    }
    
    // Helper function to check permissions based on role
    fn has_permission(claims: &crate::api::auth::Claims, permission: &str) -> bool {
        match permission {
            "create_course" => claims.role == UserRole::Admin.to_string() || 
                              claims.role == UserRole::Instructor.to_string(),
            "view_course" => true, // All authenticated users can view courses
            _ => false,
        }
    }
}