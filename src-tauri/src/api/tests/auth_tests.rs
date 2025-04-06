#[cfg(test)]
mod auth_tests {
    use super::*;
    use crate::api::auth::{generate_token, verify_token, Claims};
    use crate::models::auth::User;
    use uuid::Uuid;

    #[test]
    fn test_jwt_token_generation_and_verification() {
        // Create a test user
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            password_hash: "not_relevant_for_this_test".to_string(),
            role: "student".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Generate a token
        let token = generate_token(&user).expect("Token generation should succeed");
        
        // Verify the token
        let claims = verify_token(&token).expect("Token verification should succeed");
        
        // Assert expected values
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.name, "Test User");
        assert_eq!(claims.role, "student");
    }

    #[test]
    fn test_expired_token() {
        // This test would be more complex and would require mocking time
        // or creating a token with a very short expiration
    }
}