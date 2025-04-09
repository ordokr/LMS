#[cfg(test)]
mod tests {
    use crate::models::forum::mapping::TopicMapping;
    use crate::repository::topic_mapping_repository::TopicMappingRepository;
    use crate::services::sync_service::SyncService;
    use crate::repository::course_category_repository::CourseCategoryRepository;
    use uuid::Uuid;
    use chrono::Utc;
    use sqlx::PgPool;
    
    // Helper function to set up test database connection
    async fn setup_test_db() -> PgPool {
        let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .expect("Failed to connect to database");
            
        pool
    }
    
    #[tokio::test]
    async fn test_create_topic_mapping() {
        let pool = setup_test_db().await;
        let repo = TopicMappingRepository::new(pool.clone());
        
        let canvas_topic_id = format!("test-canvas-{}", Uuid::new_v4());
        let discourse_topic_id = format!("test-discourse-{}", Uuid::new_v4());
        let mapping_id = Uuid::new_v4();
        
        let result = repo.create_topic_mapping(&canvas_topic_id, &discourse_topic_id, mapping_id).await;
        
        assert!(result.is_ok());
        let mapping = result.unwrap();
        
        assert_eq!(mapping.canvas_topic_id, canvas_topic_id);
        assert_eq!(mapping.discourse_topic_id, discourse_topic_id);
        assert_eq!(mapping.mapping_id, mapping_id);
        assert!(mapping.sync_enabled);
        
        // Clean up
        let _ = repo.delete_topic_mapping(mapping.id).await;
    }
    
    #[tokio::test]
    async fn test_sync_status_detection() {
        let pool = setup_test_db().await;
        let repo = TopicMappingRepository::new(pool.clone());
        let course_repo = CourseCategoryRepository::new(pool.clone());
        let service = SyncService::new(repo.clone(), course_repo);
        
        // Create a test mapping
        let canvas_topic_id = format!("test-canvas-{}", Uuid::new_v4());
        let discourse_topic_id = format!("test-discourse-{}", Uuid::new_v4());
        let mapping_id = Uuid::new_v4();
        
        let mapping = repo.create_topic_mapping(&canvas_topic_id, &discourse_topic_id, mapping_id).await.unwrap();
        
        // No updates yet, should not need sync
        let status = service.needs_sync(mapping.id).await.unwrap();
        assert!(status.is_none());
        
        // Update Canvas timestamp
        let now = Utc::now();
        let _ = repo.update_topic_canvas_time(mapping.id, now).await.unwrap();
        
        // Should now need sync from Canvas to Discourse
        let status = service.needs_sync(mapping.id).await.unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap(), "canvas_to_discourse");
        
        // Clean up
        let _ = repo.delete_topic_mapping(mapping.id).await;
    }
}