use crate::models::mapping::CourseCategoryMapping;
use crate::repository::course_category_repository::CourseCategoryRepository;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use anyhow::Result;

/// Test fixture for course-category mapping tests
struct CourseCategoryTestFixture {
    pub repo: CourseCategoryRepository,
    pub pool: Pool<Postgres>,
}

impl CourseCategoryTestFixture {
    async fn new() -> Self {
        // Create a test database connection
        // Using a unique database name for tests
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/lms_test".to_string());
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");
            
        // Run migrations to set up the schema
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
            
        // Create the repository instance
        let repo = CourseCategoryRepository::new(pool.clone());
        
        Self { repo, pool }
    }
    
    async fn cleanup(&self) -> Result<()> {
        // Clean up test data
        sqlx::query!("DELETE FROM course_category_mappings")
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }
}

#[tokio::test]
async fn test_create_and_get_by_id() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Test data
    let course_id = 12345;
    let category_id = 67890;
    
    // Create a new mapping
    let mapping = fixture.repo.create(course_id, category_id).await?;
    
    // Verify created mapping
    assert_eq!(mapping.course_id, course_id);
    assert_eq!(mapping.category_id, category_id);
    assert!(mapping.sync_enabled);
    assert!(mapping.sync_topics);
    assert!(mapping.sync_users);
    assert!(mapping.id > 0);
    
    // Retrieve mapping by ID
    let retrieved = fixture.repo.get_by_id(mapping.id).await?;
    
    // Verify retrieved mapping
    assert_eq!(retrieved.id, mapping.id);
    assert_eq!(retrieved.course_id, course_id);
    assert_eq!(retrieved.category_id, category_id);
    assert_eq!(retrieved.sync_enabled, mapping.sync_enabled);
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_get_by_course_id() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Test data
    let course_id = 12345;
    let category_id = 67890;
    
    // Create a new mapping
    let mapping = fixture.repo.create(course_id, category_id).await?;
    
    // Retrieve mapping by course ID
    let retrieved = fixture.repo.get_by_course_id(course_id).await?;
    
    // Verify retrieved mapping
    assert_eq!(retrieved.id, mapping.id);
    assert_eq!(retrieved.course_id, course_id);
    assert_eq!(retrieved.category_id, category_id);
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_get_by_category_id() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Test data
    let course_id = 12345;
    let category_id = 67890;
    
    // Create a new mapping
    let mapping = fixture.repo.create(course_id, category_id).await?;
    
    // Retrieve mapping by category ID
    let retrieved = fixture.repo.get_by_category_id(category_id).await?;
    
    // Verify retrieved mapping
    assert_eq!(retrieved.id, mapping.id);
    assert_eq!(retrieved.course_id, course_id);
    assert_eq!(retrieved.category_id, category_id);
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_update_sync_time() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Test data
    let course_id = 12345;
    let category_id = 67890;
    
    // Create a new mapping
    let mapping = fixture.repo.create(course_id, category_id).await?;
    
    // Record creation time
    let creation_time = mapping.updated_at;
    
    // Wait a short time to ensure timestamp changes
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    
    // Update sync time
    let updated = fixture.repo.update_sync_time(mapping.id).await?;
    
    // Verify last_synced_at field was updated
    assert!(updated.last_synced_at.is_some());
    assert!(updated.updated_at > creation_time);
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_list_all() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Clean up to ensure we start with empty table
    fixture.cleanup().await?;
    
    // Create multiple mappings
    let mapping1 = fixture.repo.create(1001, 2001).await?;
    let mapping2 = fixture.repo.create(1002, 2002).await?;
    let mapping3 = fixture.repo.create(1003, 2003).await?;
    
    // Retrieve all mappings
    let all_mappings = fixture.repo.list_all().await?;
    
    // Verify all mappings are returned
    assert_eq!(all_mappings.len(), 3);
    
    // Verify the mappings are sorted by created_at DESC
    assert!(all_mappings[0].created_at >= all_mappings[1].created_at);
    assert!(all_mappings[1].created_at >= all_mappings[2].created_at);
    
    // Verify IDs of created mappings are in the list
    let ids: Vec<i64> = all_mappings.iter().map(|m| m.id).collect();
    assert!(ids.contains(&mapping1.id));
    assert!(ids.contains(&mapping2.id));
    assert!(ids.contains(&mapping3.id));
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Create a mapping
    let mapping = fixture.repo.create(12345, 67890).await?;
    
    // Delete the mapping
    fixture.repo.delete(mapping.id).await?;
    
    // Try to retrieve the deleted mapping
    let result = fixture.repo.get_by_id(mapping.id).await;
    
    // Verify it returns an error
    assert!(result.is_err());
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_update() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Create a mapping with default settings (all sync options enabled)
    let mapping = fixture.repo.create(12345, 67890).await?;
    assert!(mapping.sync_enabled);
    assert!(mapping.sync_topics);
    assert!(mapping.sync_users);
    
    // Update the mapping to disable some sync options
    let updated = fixture.repo.update(
        mapping.id,
        false, // sync_enabled
        true,  // sync_topics
        false, // sync_users
    ).await?;
    
    // Verify the updated mapping
    assert!(!updated.sync_enabled);
    assert!(updated.sync_topics);
    assert!(!updated.sync_users);
    assert_eq!(updated.course_id, mapping.course_id);
    assert_eq!(updated.category_id, mapping.category_id);
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_create_duplicate_course_id() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Create a mapping
    fixture.repo.create(12345, 67890).await?;
    
    // Try to create another mapping with the same course_id
    // This assumes the database has a unique constraint on course_id
    let result = fixture.repo.create(12345, 99999).await;
    
    // Verify it returns an error
    assert!(result.is_err());
    
    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_create_duplicate_category_id() -> Result<()> {
    let fixture = CourseCategoryTestFixture::new().await;
    
    // Create a mapping
    fixture.repo.create(12345, 67890).await?;
    
    // Try to create another mapping with the same category_id
    // This assumes the database has a unique constraint on category_id
    let result = fixture.repo.create(99999, 67890).await;
    
    // Verify it returns an error
    assert!(result.is_err());
    
    fixture.cleanup().await?;
    Ok(())
}
