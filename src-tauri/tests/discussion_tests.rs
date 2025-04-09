#[cfg(test)]
mod tests {
    use crate::api::courses::create_course;
    use crate::api::discussions::{create_discussion, get_discussions, sync_discussion};
    use crate::api::integration::create_course_category_mapping;
    use crate::models::course::{Course, CourseStatus};
    use crate::models::discussion::{DiscussionCreate, DiscussionStatus};
    use crate::models::integration::CourseCategoryCreate;
    use crate::services::sync::SyncService;
    use crate::db::test_utils::{create_test_db_pool, clean_test_db};
    use mockito::mock;
    use uuid::Uuid;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_discussion_creation_and_sync() {
        // Set up test DB
        let pool = create_test_db_pool().await;
        clean_test_db(&pool).await;
        
        // Create repositories
        let course_repo = Arc::new(crate::db::course_repository::SqliteCourseRepository::new(pool.clone()));
        let discussion_repo = Arc::new(crate::db::discussion_repository::SqliteDiscussionRepository::new(pool.clone()));
        let category_repo = Arc::new(crate::db::course_category_repository::SqliteCourseCategoryRepository::new(pool.clone()));
        
        // Create a mock for Discourse API
        let mut server = mockito::Server::new();
        
        // Mock for creating a topic
        let create_topic_mock = mock("POST", "/posts.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 123, "topic_id": 456}"#)
            .create_on(&server);
            
        // Create a sync service with our mock
        let sync_service = SyncService::new(
            server.url(),
            "mock-api-key".to_string(),
            "system".to_string(),
            category_repo.clone()
        );
        
        // 1. Create a course
        let course = Course {
            id: Uuid::new_v4().to_string(),
            title: "Test Course".to_string(),
            description: "Test Course Description".to_string(),
            status: CourseStatus::Active,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            modules: None,
        };
        
        let course = create_course(
            course.clone(), 
            tauri::State::new(course_repo.clone())
        ).await.expect("Failed to create course");
        
        // 2. Create a course-category mapping
        let mapping_create = CourseCategoryCreate {
            course_id: course.id.clone(),
            category_id: "123".to_string(),  // Simulated Discourse category ID
            sync_topics: Some(true),
            sync_assignments: Some(false),
        };
        
        let mapping = create_course_category_mapping(
            mapping_create.clone(), 
            tauri::State::new(category_repo.clone())
        ).await.expect("Failed to create mapping");
        
        // 3. Create a discussion
        let discussion_create = DiscussionCreate {
            course_id: course.id.clone(),
            title: "Test Discussion".to_string(),
            content: "This is a test discussion".to_string(),
            topic_id: None,
            status: Some(DiscussionStatus::Open),
        };
        
        let discussion = create_discussion(
            discussion_create.clone(), 
            tauri::State::new(discussion_repo.clone())
        ).await.expect("Failed to create discussion");
        
        // 4. Sync the discussion with Discourse
        let synced_discussion = sync_discussion(
            discussion.id.clone(),
            tauri::State::new(discussion_repo.clone()),
            tauri::State::new(sync_service)
        ).await.expect("Failed to sync discussion");
        
        // 5. Verify the discussion has a topic_id now
        assert!(synced_discussion.topic_id.is_some());
        assert_eq!(synced_discussion.topic_id.unwrap(), "456");
        
        // 6. Verify the Discourse API was called
        create_topic_mock.assert();
    }
}