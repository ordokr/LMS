use crate::models::mapping::{CourseCategoryMapping, CourseCategory, SyncDirection, SyncSummary};
use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
use crate::db::Database;
use chrono::Utc;
use mockall::predicate::*;
use mockall::mock;
use anyhow::Result;

// Mock APIs for testing
mock! {
    pub CanvasClient {}
    
    async fn get_course(&self, course_id: &str) -> Result<CourseData, crate::error::Error>;
    async fn update_course(&self, course_id: &str, name: &str, description: &str) -> Result<(), crate::error::Error>;
}

mock! {
    pub DiscourseClient {}
    
    async fn get_category(&self, category_id: &str) -> Result<CategoryData, crate::error::Error>;
    async fn update_category(&self, category_id: &str, name: &str, description: &str) -> Result<(), crate::error::Error>;
}

mock! {
    pub Database {}
    
    async fn save_course_category(&self, mapping: &CourseCategory) -> Result<(), crate::error::Error>;
}

// Course data structure to use in tests
#[derive(Debug, Clone)]
pub struct CourseData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub updated_at: chrono::DateTime<Utc>,
}

// Category data structure to use in tests
#[derive(Debug, Clone)]
pub struct CategoryData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub updated_at: chrono::DateTime<Utc>,
}

#[tokio::test]
async fn test_sync_canvas_to_discourse() -> Result<()> {
    // Create test data
    let mut mapping = CourseCategory::new(
        "course123",
        "category456",
        "Test Course"
    );
    
    // Set last_sync to past time
    mapping.last_sync = Utc::now() - chrono::Duration::hours(1);
    mapping.sync_direction = SyncDirection::CanvasToDiscourse;
    
    // Create mock APIs
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Set up Canvas mock to return course
    mock_canvas
        .expect_get_course()
        .with(eq("course123"))
        .returning(|_| {
            Ok(CourseData {
                id: "course123".to_string(),
                name: "Updated Course Name".to_string(),
                description: "New description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    // Discourse API should be updated with Canvas data
    mock_discourse
        .expect_update_category()
        .with(eq("category456"), eq("Updated Course Name"), eq("New description"))
        .returning(|_, _, _| Ok(()));
    
    // Database should save the updated mapping
    mock_db
        .expect_save_course_category()
        .returning(|_| Ok(()));
    
    // Execute sync
    let result = mapping.sync(&mock_db, &mock_canvas, &mock_discourse).await;
    
    // Verify sync succeeded
    assert!(result.is_ok());
    
    let summary = result.unwrap();
    assert!(!summary.operations.is_empty());
    assert_eq!(summary.status, "Completed");
    assert!(summary.end_time.is_some());
    
    // Verify last_sync was updated to be more recent
    assert!(mapping.last_sync > Utc::now() - chrono::Duration::minutes(1));
    
    Ok(())
}

#[tokio::test]
async fn test_sync_discourse_to_canvas() -> Result<()> {
    // Create test data
    let mut mapping = CourseCategory::new(
        "course123",
        "category456",
        "Test Course"
    );
    
    // Set last_sync to past time
    mapping.last_sync = Utc::now() - chrono::Duration::hours(1);
    mapping.sync_direction = SyncDirection::DiscourseToCanvas;
    
    // Create mock APIs
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Set up Discourse mock to return category
    mock_discourse
        .expect_get_category()
        .with(eq("category456"))
        .returning(|_| {
            Ok(CategoryData {
                id: "category456".to_string(),
                name: "Updated Category Name".to_string(),
                description: "New category description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    // Canvas API should be updated with Discourse data
    mock_canvas
        .expect_update_course()
        .with(eq("course123"), eq("Updated Category Name"), eq("New category description"))
        .returning(|_, _, _| Ok(()));
    
    // Database should save the updated mapping
    mock_db
        .expect_save_course_category()
        .returning(|_| Ok(()));
    
    // Execute sync
    let result = mapping.sync(&mock_db, &mock_canvas, &mock_discourse).await;
    
    // Verify sync succeeded
    assert!(result.is_ok());
    
    let summary = result.unwrap();
    assert!(!summary.operations.is_empty());
    assert_eq!(summary.status, "Completed");
    
    // Verify last_sync was updated
    assert!(mapping.last_sync > Utc::now() - chrono::Duration::minutes(1));
    
    Ok(())
}

#[tokio::test]
async fn test_bidirectional_sync() -> Result<()> {
    // Create test data
    let mut mapping = CourseCategory::new(
        "course123",
        "category456",
        "Test Course"
    );
    
    // Set last_sync to past time
    mapping.last_sync = Utc::now() - chrono::Duration::hours(1);
    mapping.sync_direction = SyncDirection::Bidirectional;
    
    // Create mock APIs
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Canvas mock - get course
    mock_canvas
        .expect_get_course()
        .with(eq("course123"))
        .returning(|_| {
            Ok(CourseData {
                id: "course123".to_string(),
                name: "Canvas Course Name".to_string(),
                description: "Canvas description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    // Discourse mock - get category
    mock_discourse
        .expect_get_category()
        .with(eq("category456"))
        .returning(|_| {
            Ok(CategoryData {
                id: "category456".to_string(),
                name: "Discourse Category Name".to_string(),
                description: "Discourse description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    // Canvas API should be updated with Discourse data
    mock_canvas
        .expect_update_course()
        .with(eq("course123"), eq("Discourse Category Name"), eq("Discourse description"))
        .returning(|_, _, _| Ok(()));
    
    // Discourse API should be updated with Canvas data
    mock_discourse
        .expect_update_category()
        .with(eq("category456"), eq("Canvas Course Name"), eq("Canvas description"))
        .returning(|_, _, _| Ok(()));
    
    // Database should save the updated mapping
    mock_db
        .expect_save_course_category()
        .returning(|_| Ok(()));
    
    // Execute sync
    let result = mapping.sync(&mock_db, &mock_canvas, &mock_discourse).await;
    
    // Verify sync succeeded
    assert!(result.is_ok());
    
    let summary = result.unwrap();
    assert!(summary.operations.len() >= 2); // Should have at least 2 operations
    assert_eq!(summary.status, "Completed");
    
    // Verify last_sync was updated
    assert!(mapping.last_sync > Utc::now() - chrono::Duration::minutes(1));
    
    Ok(())
}

#[tokio::test]
async fn test_sync_disabled() -> Result<()> {
    // Create test data with sync disabled
    let mut mapping = CourseCategory::new(
        "course123",
        "category456",
        "Test Course"
    );
    mapping.sync_enabled = false;
    
    // Create mock APIs
    let mock_canvas = MockCanvasClient::new();
    let mock_discourse = MockDiscourseClient::new();
    let mock_db = MockDatabase::new();
    
    // Execute sync
    let result = mapping.sync(&mock_db, &mock_canvas, &mock_discourse).await;
    
    // Verify sync was skipped because it's disabled
    assert!(result.is_ok());
    let summary = result.unwrap();
    assert_eq!(summary.status, "Completed");
    assert_eq!(summary.operations.len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_canvas_api_error_handling() -> Result<()> {
    // Create test data
    let mut mapping = CourseCategory::new(
        "course123",
        "category456",
        "Test Course"
    );
    mapping.sync_direction = SyncDirection::CanvasToDiscourse;
    
    // Create mock APIs
    let mut mock_canvas = MockCanvasClient::new();
    let mock_discourse = MockDiscourseClient::new();
    let mock_db = MockDatabase::new();
    
    // Set up Canvas mock to return an error
    mock_canvas
        .expect_get_course()
        .with(eq("course123"))
        .returning(|_| Err(crate::error::Error::ApiError("Canvas API error".to_string())));
    
    // Execute sync
    let result = mapping.sync(&mock_db, &mock_canvas, &mock_discourse).await;
    
    // Verify sync failed with an error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Canvas API error"));
    
    Ok(())
}

#[tokio::test]
async fn test_discourse_api_error_handling() -> Result<()> {
    // Create test data
    let mut mapping = CourseCategory::new(
        "course123",
        "category456",
        "Test Course"
    );
    mapping.sync_direction = SyncDirection::DiscourseToCanvas;
    
    // Create mock APIs
    let mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mock_db = MockDatabase::new();
    
    // Set up Discourse mock to return an error
    mock_discourse
        .expect_get_category()
        .with(eq("category456"))
        .returning(|_| Err(crate::error::Error::ApiError("Discourse API error".to_string())));
    
    // Execute sync
    let result = mapping.sync(&mock_db, &mock_canvas, &mock_discourse).await;
    
    // Verify sync failed with an error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Discourse API error"));
    
    Ok(())
}
