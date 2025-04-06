#[cfg(test)]
mod tests {
    use super::super::jwt;
    use std::env;

    #[test]
    fn test_token_generation_and_validation() {
        // Set up test environment
        env::set_var("JWT_SECRET", "test_secret_key_for_jwt_validation");

        // Generate token
        let user_id = "user123";
        let role = "student";
        let canvas_id = "canvas456";
        let token = jwt::generate_token(user_id, role, canvas_id).expect("Token generation should succeed");

        // Validate token
        let claims = jwt::validate_token(&token).expect("Token validation should succeed");
        
        // Check claims
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, role);
        assert_eq!(claims.canvas_id, canvas_id);
        
        // Test helper functions
        assert!(jwt::is_token_valid(&token));
        assert_eq!(jwt::get_user_id_from_token(&token), Some(user_id.to_string()));
        assert_eq!(jwt::get_user_role_from_token(&token), Some(role.to_string()));
    }

    #[test]
    fn test_invalid_token() {
        env::set_var("JWT_SECRET", "test_secret_key_for_jwt_validation");
        
        // Invalid token
        let invalid_token = "invalid.token.string";
        assert!(!jwt::is_token_valid(invalid_token));
        assert_eq!(jwt::get_user_id_from_token(invalid_token), None);
        assert_eq!(jwt::get_user_role_from_token(invalid_token), None);
    }
}