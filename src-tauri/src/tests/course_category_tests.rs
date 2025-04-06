use uuid::Uuid;
use chrono::Utc;
use crate::db::course_category_repository::CourseCategoryRepository;
use crate::models::integration::{CourseCategoryCreate, CourseCategoryUpdate};
use sqlx::PgPool;

async fn setup_test_db() -> PgPool {
    let pool = PgPool::connect("postgresql://postgres:password@localhost:5432/test_db")
        .await
        .expect("Failed to connect to test database");
    
    // Clear existing test data
    sqlx::query!("DELETE FROM course_categories")
        .execute(&pool)
        .await
        .expect("Failed to clear test data");
        
    pool
}

#[tokio::test]
async fn test_create_course_category_mapping() {
    let pool = setup_test_db().await;
    let repo = CourseCategoryRepository::new(pool);
    
    let new_mapping = CourseCategoryCreate {
        canvas_course_id: "course-123".to_string(),
        discourse_category_id: 456,
        sync_enabled: true,
    };
    
    let result = repo.create(new_mapping).await.expect("Failed to create mapping");
    
    assert_eq!(result.canvas_course_id, "course-123");
    assert_eq!(result.discourse_category_id, 456);
    assert_eq!(result.sync_enabled, true);
    assert!(result.last_synced_at.is_none());
}

#[tokio::test]
async fn test_find_by_canvas_course_id() {
    let pool = setup_test_db().await;
    let repo = CourseCategoryRepository::new(pool);
    
    // Create test mapping
    let new_mapping = CourseCategoryCreate {
        canvas_course_id: "course-find-test".to_string(),
        discourse_category_id: 789,
        sync_enabled: true,
    };
    
    let created = repo.create(new_mapping).await.expect("Failed to create mapping");
    
    // Find by canvas course ID
    let found = repo.find_by_canvas_course_id("course-find-test")
        .await
        .expect("Query failed")
        .expect("Mapping not found");
    
    assert_eq!(found.id, created.id);
    assert_eq!(found.canvas_course_id, "course-find-test");
    assert_eq!(found.discourse_category_id, 789);
}

#[tokio::test]
async fn test_update_course_category_mapping() {
    let pool = setup_test_db().await;
    let repo = CourseCategoryRepository::new(pool);
    
    // Create test mapping
    let new_mapping = CourseCategoryCreate {
        canvas_course_id: "course-update-test".to_string(),
        discourse_category_id: 101,
        sync_enabled: true,
    };
    
    let created = repo.create(new_mapping).await.expect("Failed to create mapping");
    
    // Update the mapping
    let now = Utc::now();
    let update = CourseCategoryUpdate {
        sync_enabled: Some(false),
        last_synced_at: Some(now),
    };
    
    let updated = repo.update(created.id, update)
        .await
        .expect("Update failed")
        .expect("Mapping not found after update");
    
    assert_eq!(updated.id, created.id);
    assert_eq!(updated.sync_enabled, false);
    assert!(updated.last_synced_at.is_some());
}

#[tokio::test]
async fn test_delete_course_category_mapping() {
    let pool = setup_test_db().await;
    let repo = CourseCategoryRepository::new(pool);
    
    // Create test mapping
    let new_mapping = CourseCategoryCreate {
        canvas_course_id: "course-delete-test".to_string(),
        discourse_category_id: 202,
        sync_enabled: true,
    };
    
    let created = repo.create(new_mapping).await.expect("Failed to create mapping");
    
    // Delete the mapping
    let deleted = repo.delete(created.id).await.expect("Delete failed");
    assert!(deleted);
    
    // Verify it's gone
    let not_found = repo.find_by_id(created.id).await.expect("Query failed");
    assert!(not_found.is_none());
}