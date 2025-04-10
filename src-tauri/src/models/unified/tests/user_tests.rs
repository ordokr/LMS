use crate::models::unified::User;
use chrono::Utc;
use std::collections::HashMap;

#[test]
fn test_user_creation() {
    let user = User::new(
        Some("test-id".to_string()),
        Some("Test User".to_string()),
        Some("test@example.com".to_string()),
        Some("testuser".to_string()),
        Some("https://example.com/avatar.jpg".to_string()),
        Some("canvas-123".to_string()),
        Some("discourse-456".to_string()),
        Some(Utc::now()),
        Some("native".to_string()),
        Some(vec!["student".to_string(), "teacher".to_string()]),
        None,
    );
    
    assert_eq!(user.id, "test-id");
    assert_eq!(user.name, "Test User");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.username, "testuser");
    assert_eq!(user.avatar, "https://example.com/avatar.jpg");
    assert_eq!(user.canvas_id, Some("canvas-123".to_string()));
    assert_eq!(user.discourse_id, Some("discourse-456".to_string()));
    assert_eq!(user.source_system, Some("native".to_string()));
    assert_eq!(user.roles, vec!["student", "teacher"]);
}

#[test]
fn test_from_canvas_user() {
    let canvas_user_json = serde_json::json!({
        "id": "12345",
        "name": "Canvas User",
        "email": "canvas_user@example.com",
        "login_id": "canvas_user",
        "avatar_url": "https://canvas.example.com/avatar.jpg"
    });
    
    let user = User::from_canvas_user(&canvas_user_json);
    
    assert_eq!(user.name, "Canvas User");
    assert_eq!(user.email, "canvas_user@example.com");
    assert_eq!(user.username, "canvas_user");
    assert_eq!(user.avatar, "https://canvas.example.com/avatar.jpg");
    assert_eq!(user.canvas_id, Some("12345".to_string()));
    assert_eq!(user.source_system, Some("canvas".to_string()));
    assert!(user.roles.contains(&"student".to_string()));
}

#[test]
fn test_from_discourse_user() {
    let discourse_user_json = serde_json::json!({
        "id": "67890",
        "name": "Discourse User",
        "username": "discourse_user",
        "email": "discourse_user@example.com",
        "avatar_template": "https://discourse.example.com/avatar.jpg",
        "last_seen_at": "2023-01-01T12:00:00Z",
        "groups": [{"name": "Moderator"}, {"name": "TrustedUser"}]
    });
    
    let user = User::from_discourse_user(&discourse_user_json);
    
    assert_eq!(user.name, "Discourse User");
    assert_eq!(user.email, "discourse_user@example.com");
    assert_eq!(user.username, "discourse_user");
    assert_eq!(user.avatar, "https://discourse.example.com/avatar.jpg");
    assert_eq!(user.discourse_id, Some("67890".to_string()));
    assert_eq!(user.source_system, Some("discourse".to_string()));
    assert!(user.roles.contains(&"moderator".to_string()));
    assert!(user.roles.contains(&"trusteduser".to_string()));
}

// Add more tests as needed for other models