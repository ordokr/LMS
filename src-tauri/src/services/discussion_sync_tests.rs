#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::discussion_mapping::{DiscussionMapping, CanvasDiscussionEntry, DiscoursePost};
    use crate::models::course_category::{CourseCategory, SyncDirection};
    use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
    use crate::db::DbPool;
    use chrono::{Utc, Duration};
    use mockall::predicate::*;
    use mockall::mock;
    use std::sync::Arc;

    // Mock Canvas client
    mock! {
        CanvasClient {}
        
        async fn get_discussion(&self, id: &str) -> Result<CanvasDiscussion, Error>;
        async fn update_discussion(&self, id: &str, title: &str, message: &str) -> Result<(), Error>;
        async fn get_discussion_entries(&self, discussion_id: &str) -> Result<Vec<CanvasDiscussionEntry>, Error>;
        async fn create_discussion_entry(&self, discussion_id: &str, message: &str, user_id: Option<&str>) -> Result<String, Error>;
        async fn update_discussion_entry(&self, discussion_id: &str, entry_id: &str, message: &str) -> Result<(), Error>;
    }

    // Mock Discourse client
    mock! {
        DiscourseClient {}
        
        async fn get_topic(&self, id: &str) -> Result<DiscourseTopic, Error>;
        async fn get_topic_with_posts(&self, id: &str) -> Result<DiscourseTopicWithPosts, Error>;
        async fn update_topic(&self, id: &str, title: &str, content: &str) -> Result<(), Error>;
        async fn get_topic_posts(&self, topic_id: &str) -> Result<Vec<DiscoursePost>, Error>;
        async fn create_post(&self, topic_id: &str, content: &str, external_id: Option<&str>, user_id: Option<&str>) -> Result<String, Error>;
        async fn update_post(&self, id: &str, content: &str) -> Result<(), Error>;
        async fn update_post_external_id(&self, id: &str, external_id: &str) -> Result<(), Error>;
    }

    // Mock database for testing
    mock! {
        DbPool {}
        
        // Add necessary mocked methods
    }

    #[tokio::test]
    async fn test_sync_canvas_to_discourse() {
        // Create test data
        let now = Utc::now();
        let mapping = DiscussionMapping {
            id: "test-mapping".to_string(),
            canvas_discussion_id: "canvas-disc-123".to_string(),
            discourse_topic_id: "discourse-topic-456".to_string(),
            course_category_id: "course-cat-789".to_string(),
            title: "Test Discussion".to_string(),
            last_sync: now - Duration::hours(1),
            sync_enabled: true,
            sync_posts: true,
            created_at: now - Duration::days(1),
        };
        
        let course_category = CourseCategory {
            id: "course-cat-789".to_string(),
            canvas_course_id: "canvas-course-123".to_string(),
            discourse_category_id: "discourse-cat-456".to_string(),
            name: "Test Course".to_string(),
            sync_direction: SyncDirection::CanvasToDiscourse,
            // Add other required fields
            last_sync: now - Duration::hours(2),
            sync_enabled: true,
        };
        
        // Setup mocks
        let mut mock_canvas = MockCanvasClient::new();
        let mut mock_discourse = MockDiscourseClient::new();
        let mut mock_db = MockDbPool::new();
        
        // Mock Canvas discussion retrieval
        mock_canvas
            .expect_get_discussion()
            .with(eq("canvas-disc-123"))
            .returning(|_| {
                Ok(CanvasDiscussion {
                    id: "canvas-disc-123".to_string(),
                    title: "Updated Canvas Title".to_string(),
                    message: "Updated Canvas content".to_string(),
                    updated_at: Utc::now(),
                })
            });
        
        // Mock Canvas discussion entries retrieval
        mock_canvas
            .expect_get_discussion_entries()
            .with(eq("canvas-disc-123"))
            .returning(|_| {
                Ok(vec![
                    CanvasDiscussionEntry {
                        id: "entry-1".to_string(),
                        user_id: Some("user-1".to_string()),
                        message: "First reply from Canvas".to_string(),
                        created_at: Utc::now() - Duration::hours(2),
                        updated_at: Utc::now() - Duration::hours(1),
                        parent_id: None,
                    },
                    CanvasDiscussionEntry {
                        id: "entry-2".to_string(),
                        user_id: Some("user-2".to_string()),
                        message: "Second reply from Canvas".to_string(),
                        created_at: Utc::now() - Duration::hours(1),
                        updated_at: Utc::now() - Duration::minutes(30),
                        parent_id: None,
                    },
                ])
            });
        
        // Mock Discourse topic update
        mock_discourse
            .expect_update_topic()
            .with(eq("discourse-topic-456"), eq("Updated Canvas Title"), eq("Updated Canvas content"))
            .returning(|_, _, _| Ok(()));
        
        // Mock Discourse posts retrieval
        mock_discourse
            .expect_get_topic_posts()
            .with(eq("discourse-topic-456"))
            .returning(|_| {
                Ok(vec![
                    // First post is the topic itself
                    DiscoursePost {
                        id: "post-1".to_string(),
                        topic_id: "discourse-topic-456".to_string(),
                        user_id: Some("user-1".to_string()),
                        content: "Topic content".to_string(),
                        external_id: None,
                        created_at: Utc::now() - Duration::days(1),
                        updated_at: Utc::now() - Duration::days(1),
                    },
                ])
            });
        
        // Mock Discourse post creation (for new entries)
        mock_discourse
            .expect_create_post()
            .with(eq("discourse-topic-456"), any(), any(), any())
            .times(2)  // Expect 2 posts to be created
            .returning(|_, _, _, _| Ok("new-post-id".to_string()));
        
        // Create the sync service
        let service = DiscussionSyncService::new(
            Arc::new(mock_db),
            Arc::new(mock_canvas),
            Arc::new(mock_discourse),
        );
        
        // Perform the test
        let mut result = SyncResult::new("test-mapping");
        let sync_result = service.sync_discussion_topics(&mapping, &course_category, &mut result).await;
        
        // Verify the result
        assert!(sync_result.is_ok());
        assert_eq!(result.discourse_updates, 1);  // One topic update
        
        // Test post syncing
        let mut post_result = SyncResult::new("test-mapping");
        let post_sync_result = service.sync_canvas_to_discourse_posts(&mapping, &mut post_result).await;
        
        // Verify the post sync result
        assert!(post_sync_result.is_ok());
        assert_eq!(post_result.discourse_updates, 2);  // Two posts created
    }
}