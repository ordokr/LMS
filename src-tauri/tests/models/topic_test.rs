#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc, TimeZone};
    use uuid::Uuid;
    use std::collections::HashMap;
    use crate::models::forum::topic::{Topic, SyncStatus};
    use serde_json::json;

    #[test]
    fn test_topic_creation() {
        let category_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let title = "Test Topic Title".to_string();
        let content = "Test topic content".to_string();
        
        let topic = Topic::new(category_id, author_id, title.clone(), content.clone());
        
        // Verify basic fields
        assert_eq!(topic.category_id, category_id);
        assert_eq!(topic.author_id, author_id);
        assert_eq!(topic.title, title);
        assert_eq!(topic.content, content);
        assert_eq!(topic.sync_status, SyncStatus::LocalOnly);
        assert_eq!(topic.is_pinned, false);
        assert_eq!(topic.is_closed, false);
        assert_eq!(topic.view_count, 0);
        
        // Verify created_at and updated_at are set
        let now = Utc::now();
        let diff_created = (topic.created_at - now).num_seconds().abs();
        assert!(diff_created < 5); // Within 5 seconds
        
        let diff_updated = (topic.updated_at - now).num_seconds().abs();
        assert!(diff_updated < 5); // Within 5 seconds
        
        // Verify last_post_at is set
        assert!(topic.last_post_at.is_some());
        let diff_last_post = (topic.last_post_at.unwrap() - now).num_seconds().abs();
        assert!(diff_last_post < 5); // Within 5 seconds
    }

    #[test]
    fn test_topic_validation() {
        let category_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        // Valid topic
        let topic = Topic::new(
            category_id,
            author_id,
            "Test Topic".to_string(),
            "Test content".to_string()
        );
        assert!(topic.validate().is_ok());
        
        // Invalid: Empty title
        let invalid_topic = Topic::new(
            category_id,
            author_id,
            "".to_string(),
            "Test content".to_string()
        );
        assert!(invalid_topic.validate().is_err());
        
        // Invalid: Empty content
        let invalid_topic = Topic::new(
            category_id,
            author_id,
            "Test Topic".to_string(),
            "".to_string()
        );
        assert!(invalid_topic.validate().is_err());
        
        // Invalid: Empty category_id
        let invalid_topic = Topic::new(
            Uuid::nil(),
            author_id,
            "Test Topic".to_string(),
            "Test content".to_string()
        );
        assert!(invalid_topic.validate().is_err());
        
        // Invalid: Empty author_id
        let invalid_topic = Topic::new(
            category_id,
            Uuid::nil(),
            "Test Topic".to_string(),
            "Test content".to_string()
        );
        assert!(invalid_topic.validate().is_err());
    }

    #[test]
    fn test_topic_from_discourse_api() {
        let category_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        // Create a map of discourse user IDs to our UUIDs
        let mut author_map = HashMap::new();
        author_map.insert(123, author_id);
        
        let discourse_json = json!({
            "id": 456,
            "title": "Test Discourse Topic",
            "post_stream": {
                "posts": [
                    {
                        "raw": "This is the topic content",
                        "user_id": 123
                    }
                ]
            },
            "created_at": "2025-04-01T10:30:00Z",
            "last_posted_at": "2025-04-05T15:45:30Z",
            "pinned": true,
            "closed": false,
            "views": 42,
            "tags": ["general", "question", "help"]
        });
        
        let topic = Topic::from_discourse_api(&discourse_json, category_id, &author_map).unwrap();
        
        // Verify basic fields
        assert_eq!(topic.category_id, category_id);
        assert_eq!(topic.author_id, author_id);
        assert_eq!(topic.title, "Test Discourse Topic");
        assert_eq!(topic.content, "This is the topic content");
        assert_eq!(topic.discourse_topic_id, Some(456));
        assert_eq!(topic.sync_status, SyncStatus::SyncedWithDiscourse);
        assert_eq!(topic.is_pinned, true);
        assert_eq!(topic.is_closed, false);
        assert_eq!(topic.view_count, 42);
        assert_eq!(topic.tags, vec!["general", "question", "help"]);
        
        // Verify dates were parsed correctly
        assert_eq!(topic.created_at.year(), 2025);
        assert_eq!(topic.created_at.month(), 4);
        assert_eq!(topic.created_at.day(), 1);
        
        assert_eq!(topic.last_post_at.unwrap().year(), 2025);
        assert_eq!(topic.last_post_at.unwrap().month(), 4);
        assert_eq!(topic.last_post_at.unwrap().day(), 5);
    }

    #[test]
    fn test_topic_from_canvas_api() {
        let category_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        // Create a map of canvas user IDs to our UUIDs
        let mut author_map = HashMap::new();
        author_map.insert("user_123".to_string(), author_id);
        
        let canvas_json = json!({
            "id": "discussion_456",
            "title": "Test Canvas Discussion",
            "message": "This is the discussion content",
            "user_id": "user_123",
            "posted_at": "2025-05-01T14:30:45Z",
            "delayed_post_at": "2025-05-02T00:00:00Z",
            "locked": true,
            "discussion_type": "threaded_question"
        });
        
        let topic = Topic::from_canvas_api(&canvas_json, category_id, &author_map).unwrap();
        
        // Verify basic fields
        assert_eq!(topic.category_id, category_id);
        assert_eq!(topic.author_id, author_id);
        assert_eq!(topic.title, "Test Canvas Discussion");
        assert_eq!(topic.content, "This is the discussion content");
        assert_eq!(topic.canvas_discussion_id, Some("discussion_456".to_string()));
        assert_eq!(topic.sync_status, SyncStatus::SyncedWithCanvas);
        assert_eq!(topic.is_closed, true);
        assert_eq!(topic.is_question, true);
        
        // Verify dates were parsed correctly
        assert_eq!(topic.created_at.year(), 2025);
        assert_eq!(topic.created_at.month(), 5);
        assert_eq!(topic.created_at.day(), 1);
        
        assert_eq!(topic.publish_at.unwrap().year(), 2025);
        assert_eq!(topic.publish_at.unwrap().month(), 5);
        assert_eq!(topic.publish_at.unwrap().day(), 2);
    }

    #[test]
    fn test_topic_serialization() {
        let category_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let title = "Test Topic Title".to_string();
        let content = "Test topic content".to_string();
        
        let mut topic = Topic::new(category_id, author_id, title, content);
        topic.tags = vec!["tag1".to_string(), "tag2".to_string()];
        
        // Serialize to JSON
        let json = serde_json::to_string(&topic).unwrap();
        
        // Deserialize from JSON
        let deserialized: Topic = serde_json::from_str(&json).unwrap();
        
        // Verify fields
        assert_eq!(topic.id, deserialized.id);
        assert_eq!(topic.category_id, deserialized.category_id);
        assert_eq!(topic.author_id, deserialized.author_id);
        assert_eq!(topic.title, deserialized.title);
        assert_eq!(topic.content, deserialized.content);
        assert_eq!(topic.tags, deserialized.tags);
        
        // Verify dates are properly serialized/deserialized
        assert_eq!(
            topic.created_at.timestamp(),
            deserialized.created_at.timestamp()
        );
        assert_eq!(
            topic.updated_at.timestamp(),
            deserialized.updated_at.timestamp()
        );
        assert_eq!(
            topic.last_post_at.unwrap().timestamp(),
            deserialized.last_post_at.unwrap().timestamp()
        );
    }
}