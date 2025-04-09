#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc, TimeZone};
    use uuid::Uuid;
    use std::collections::HashMap;
    use crate::models::forum::post::{Post, SyncStatus};
    use serde_json::json;

    #[test]
    fn test_post_creation() {
        let topic_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let content = "Test post content".to_string();
        
        let post = Post::new(topic_id, author_id, content.clone());
        
        // Verify basic fields
        assert_eq!(post.topic_id, topic_id);
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.content, content);
        assert_eq!(post.sync_status, SyncStatus::LocalOnly);
        assert_eq!(post.likes, 0);
        assert_eq!(post.is_solution, false);
        
        // Verify created_at and updated_at are set
        let now = Utc::now();
        let diff_created = (post.created_at - now).num_seconds().abs();
        assert!(diff_created < 5); // Within 5 seconds
        
        let diff_updated = (post.updated_at - now).num_seconds().abs();
        assert!(diff_updated < 5); // Within 5 seconds
    }

    #[test]
    fn test_post_validation() {
        let topic_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        // Valid post
        let post = Post::new(topic_id, author_id, "Test content".to_string());
        assert!(post.validate().is_ok());
        
        // Invalid: Empty content
        let mut invalid_post = Post::new(topic_id, author_id, "".to_string());
        assert!(invalid_post.validate().is_err());
        
        // Invalid: Empty topic_id
        let mut invalid_post = Post::new(Uuid::nil(), author_id, "Test content".to_string());
        assert!(invalid_post.validate().is_err());
        
        // Invalid: Empty author_id
        let mut invalid_post = Post::new(topic_id, Uuid::nil(), "Test content".to_string());
        assert!(invalid_post.validate().is_err());
    }

    #[test]
    fn test_post_from_discourse_api() {
        let topic_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        // Create a map of discourse user IDs to our UUIDs
        let mut author_map = HashMap::new();
        author_map.insert(123, author_id);
        
        let discourse_json = json!({
            "id": 456,
            "user_id": 123,
            "raw": "This is a test post",
            "cooked": "<p>This is a test post</p>",
            "created_at": "2025-04-01T10:30:00Z",
            "updated_at": "2025-04-02T15:45:30Z",
            "like_count": 5,
            "accepted_answer": true
        });
        
        let post = Post::from_discourse_api(&discourse_json, topic_id, &author_map).unwrap();
        
        // Verify basic fields
        assert_eq!(post.topic_id, topic_id);
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.content, "This is a test post");
        assert_eq!(post.html_content, Some("<p>This is a test post</p>".to_string()));
        assert_eq!(post.discourse_post_id, Some(456));
        assert_eq!(post.sync_status, SyncStatus::SyncedWithDiscourse);
        assert_eq!(post.likes, 5);
        assert_eq!(post.is_solution, true);
        
        // Verify dates were parsed correctly
        assert_eq!(post.created_at.year(), 2025);
        assert_eq!(post.created_at.month(), 4);
        assert_eq!(post.created_at.day(), 1);
        
        assert_eq!(post.updated_at.year(), 2025);
        assert_eq!(post.updated_at.month(), 4);
        assert_eq!(post.updated_at.day(), 2);
    }

    #[test]
    fn test_post_from_canvas_api() {
        let topic_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        // Create a map of canvas user IDs to our UUIDs
        let mut author_map = HashMap::new();
        author_map.insert("user_123".to_string(), author_id);
        
        let canvas_json = json!({
            "id": "entry_456",
            "user_id": "user_123",
            "message": "This is a canvas discussion entry",
            "created_at": "2025-05-01T14:30:45Z",
            "updated_at": "2025-05-02T10:15:30Z",
            "score": 92.5
        });
        
        let post = Post::from_canvas_api(&canvas_json, topic_id, &author_map).unwrap();
        
        // Verify basic fields
        assert_eq!(post.topic_id, topic_id);
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.content, "This is a canvas discussion entry");
        assert_eq!(post.canvas_entry_id, Some("entry_456".to_string()));
        assert_eq!(post.sync_status, SyncStatus::SyncedWithCanvas);
        assert_eq!(post.score, Some(92.5));
        
        // Verify dates were parsed correctly
        assert_eq!(post.created_at.year(), 2025);
        assert_eq!(post.created_at.month(), 5);
        assert_eq!(post.created_at.day(), 1);
        
        assert_eq!(post.updated_at.year(), 2025);
        assert_eq!(post.updated_at.month(), 5);
        assert_eq!(post.updated_at.day(), 2);
    }

    #[test]
    fn test_post_serialization() {
        let topic_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let content = "Test post content".to_string();
        
        let post = Post::new(topic_id, author_id, content);
        
        // Serialize to JSON
        let json = serde_json::to_string(&post).unwrap();
        
        // Deserialize from JSON
        let deserialized: Post = serde_json::from_str(&json).unwrap();
        
        // Verify fields
        assert_eq!(post.id, deserialized.id);
        assert_eq!(post.topic_id, deserialized.topic_id);
        assert_eq!(post.author_id, deserialized.author_id);
        assert_eq!(post.content, deserialized.content);
        
        // Verify dates are properly serialized/deserialized
        assert_eq!(
            post.created_at.timestamp(),
            deserialized.created_at.timestamp()
        );
        assert_eq!(
            post.updated_at.timestamp(),
            deserialized.updated_at.timestamp()
        );
    }
}