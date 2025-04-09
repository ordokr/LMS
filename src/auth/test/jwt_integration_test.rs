use super::super::jwt;
use crate::models::user::User;
use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

// Setup function that will be called only once
fn setup() {
    INIT.call_once(|| {
        // Set up test environment variables
        env::set_var("JWT_SECRET", "test_secret_for_integration_tests");
    });
}

#[test]
fn test_token_generation_and_verification_with_full_data() {
    setup();

    // Create a test user with all fields
    let user = User {
        id: "user123".to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        display_name: Some("Test User".to_string()),
        role: "student".to_string(),
        canvas_id: "canvas456".to_string(),
        discourse_id: Some("discourse789".to_string()),
        // Add other fields as needed for your User struct
    };

    // Generate a complete token with all fields
    let token = jwt::generate_token(
        &user.id,
        &user.role,
        &user.canvas_id,
        user.discourse_id.as_deref(),
        Some(&user.email),
        user.display_name.as_deref()
    ).expect("Token generation should succeed");

    // Verify the token
    let claims = jwt::validate_token(&token).expect("Token validation should succeed");
    
    // Check all fields are properly set
    assert_eq!(claims.sub, user.id);
    assert_eq!(claims.role, user.role);
    assert_eq!(claims.canvas_id, user.canvas_id);
    assert_eq!(claims.discourse_id, user.discourse_id);
    assert_eq!(claims.email, Some(user.email.clone()));
    assert_eq!(claims.name, user.display_name);
}

#[test]
fn test_token_generation_and_verification_with_minimal_data() {
    setup();

    // Create a test user with minimal fields
    let user_id = "minimal_user";
    let role = "guest";
    let canvas_id = "minimal_canvas";

    // Generate a token with minimal data
    let token = jwt::generate_token(
        user_id,
        role,
        canvas_id,
        None,
        None,
        None
    ).expect("Token generation should succeed");

    // Verify the token
    let claims = jwt::validate_token(&token).expect("Token validation should succeed");
    
    // Check required fields are set
    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.role, role);
    assert_eq!(claims.canvas_id, canvas_id);
    
    // Check optional fields are None
    assert_eq!(claims.discourse_id, None);
    assert_eq!(claims.email, None);
    assert_eq!(claims.name, None);
}

#[test]
fn test_token_expiration() {
    setup();
    
    // Generate a token
    let token = jwt::generate_token(
        "expiring_user",
        "student",
        "canvas_id",
        None,
        None,
        None
    ).expect("Token generation should succeed");
    
    // Verify the token is valid
    let claims = jwt::validate_token(&token).expect("Token validation should succeed");
    
    // Check expiration time is set to future (at least 23 hours from now)
    let now = chrono::Utc::now().timestamp() as usize;
    assert!(claims.exp > now);
    assert!(claims.exp > now + 23 * 3600); // At least 23 hours in the future
}

#[test]
fn test_invalid_token() {
    setup();
    
    // Try to validate an invalid token
    let result = jwt::validate_token("invalid.token.format");
    assert!(result.is_err());
    
    // Create a malformed but properly formatted token
    let malformed_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    let result = jwt::validate_token(malformed_token);
    assert!(result.is_err());
}

#[test]
fn test_helper_functions() {
    setup();
    
    // Generate a token
    let user_id = "helper_test_user";
    let role = "admin";
    let canvas_id = "canvas123";
    
    let token = jwt::generate_token(
        user_id,
        role,
        canvas_id,
        None,
        None,
        None
    ).expect("Token generation should succeed");
    
    // Test helper functions
    assert!(jwt::is_token_valid(&token));
    assert_eq!(jwt::get_user_id_from_token(&token), Some(user_id.to_string()));
    assert_eq!(jwt::get_user_role_from_token(&token), Some(role.to_string()));
    
    // Test with invalid token
    assert!(!jwt::is_token_valid("invalid.token"));
    assert_eq!(jwt::get_user_id_from_token("invalid.token"), None);
    assert_eq!(jwt::get_user_role_from_token("invalid.token"), None);
}
