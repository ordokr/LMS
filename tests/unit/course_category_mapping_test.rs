use chrono::Utc;
use mockall::predicate::*;
use mockall::mock;
use uuid::Uuid;

use crate::models::{
    course::Course,
    category::Category,
    mapping::{CourseCategory, CourseCategoryMapping, SyncDirection}
};
use crate::services::mapping_service::MappingService;
use crate::error::Error;
use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
use crate::db::Database;

// Mock Canvas client
mock! {
    CanvasClient {}
    
    trait CanvasClientTrait {
        async fn get_course(&self, course_id: &str) -> Result<crate::api::canvas::CourseData, Error>;
        async fn update_course(&self, course_id: &str, name: &str, description: &str) -> Result<(), Error>;
    }
}

// Mock Discourse client
mock! {
    DiscourseClient {}
    
    trait DiscourseClientTrait {
        async fn get_category(&self, category_id: &str) -> Result<crate::api::discourse::CategoryData, Error>;
        async fn update_category(&self, category_id: &str, name: &str, description: &str) -> Result<(), Error>;
        async fn create_category(&self, name: &str, description: &str) -> Result<crate::api::discourse::CategoryData, Error>;
    }
}

// Mock Database
mock! {
    Database {}
    
    trait DatabaseTrait {
        async fn save_course_category(&self, mapping: &CourseCategory) -> Result<(), Error>;
        async fn get_course_category(&self, id: &str) -> Result<CourseCategory, Error>;
        async fn get_course_categories(&self) -> Result<Vec<CourseCategory>, Error>;
        async fn delete_course_category(&self, id: &str) -> Result<(), Error>;
        async fn mapping_exists(&self, canvas_course_id: &str, discourse_category_id: &str) -> Result<bool, Error>;
    }
}

#[tokio::test]
async fn test_create_mapping() {
    // Setup mocks
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Define test data
    let canvas_id = "canvas-123";
    let discourse_id = "discourse-456";
    let name = "Test Course";
    
    // Set up mock behavior
    mock_canvas
        .expect_get_course()
        .with(eq(canvas_id))
        .times(1)
        .returning(|_| {
            Ok(crate::api::canvas::CourseData {
                id: "canvas-123".to_string(),
                name: "Test Course".to_string(),
                description: "Test Description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    mock_discourse
        .expect_get_category()
        .with(eq(discourse_id))
        .times(1)
        .returning(|_| {
            Ok(crate::api::discourse::CategoryData {
                id: "discourse-456".to_string(),
                name: "Test Category".to_string(),
                description: "Test Description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    mock_db
        .expect_mapping_exists()
        .with(eq(canvas_id), eq(discourse_id))
        .times(1)
        .returning(|_, _| Ok(false));
    
    mock_db
        .expect_save_course_category()
        .times(1)
        .returning(|_| Ok(()));
    
    // Create service with mocks
    let mapping_service = MappingService::new(
        std::sync::Arc::new(mock_db),
        std::sync::Arc::new(mock_canvas),
        std::sync::Arc::new(mock_discourse),
    );
    
    // Execute test
    let result = mapping_service.create_mapping(
        canvas_id,
        discourse_id,
        name,
        Some(SyncDirection::Bidirectional),
    ).await;
    
    // Verify result
    assert!(result.is_ok());
    let mapping = result.unwrap();
    assert_eq!(mapping.canvas_course_id, canvas_id);
    assert_eq!(mapping.discourse_category_id, discourse_id);
    assert_eq!(mapping.name, name);
    assert_eq!(mapping.sync_direction, SyncDirection::Bidirectional);
    assert!(mapping.sync_enabled);
}

#[tokio::test]
async fn test_create_mapping_already_exists() {
    // Setup mocks
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Define test data
    let canvas_id = "canvas-123";
    let discourse_id = "discourse-456";
    let name = "Test Course";
    
    // Set up mock behavior - mapping already exists
    mock_db
        .expect_mapping_exists()
        .with(eq(canvas_id), eq(discourse_id))
        .times(1)
        .returning(|_, _| Ok(true));
    
    // Create service with mocks
    let mapping_service = MappingService::new(
        std::sync::Arc::new(mock_db),
        std::sync::Arc::new(mock_canvas),
        std::sync::Arc::new(mock_discourse),
    );
    
    // Execute test
    let result = mapping_service.create_mapping(
        canvas_id,
        discourse_id,
        name,
        Some(SyncDirection::Bidirectional),
    ).await;
    
    // Verify result
    assert!(result.is_err());
    assert!(matches!(result, Err(Error::DuplicateMapping)));
}

#[tokio::test]
async fn test_get_mapping() {
    // Setup mocks
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Define test data
    let mapping_id = Uuid::new_v4().to_string();
    
    // Set up mock behavior
    mock_db
        .expect_get_course_category()
        .with(eq(mapping_id.clone()))
        .times(1)
        .returning(move |_| {
            Ok(CourseCategory {
                id: mapping_id.clone(),
                canvas_course_id: "canvas-123".to_string(),
                discourse_category_id: "discourse-456".to_string(),
                name: "Test Mapping".to_string(),
                last_sync: Utc::now(),
                sync_enabled: true,
                sync_direction: SyncDirection::Bidirectional,
            })
        });
    
    // Create service with mocks
    let mapping_service = MappingService::new(
        std::sync::Arc::new(mock_db),
        std::sync::Arc::new(mock_canvas),
        std::sync::Arc::new(mock_discourse),
    );
    
    // Execute test
    let result = mapping_service.get_mapping(&mapping_id).await;
    
    // Verify result
    assert!(result.is_ok());
    let mapping = result.unwrap();
    assert_eq!(mapping.id, mapping_id);
    assert_eq!(mapping.canvas_course_id, "canvas-123");
    assert_eq!(mapping.discourse_category_id, "discourse-456");
}

#[tokio::test]
async fn test_update_mapping() {
    // Setup mocks
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Define test data
    let mapping_id = Uuid::new_v4().to_string();
    let new_name = "Updated Course Name";
    
    // Set up mock behavior
    mock_db
        .expect_get_course_category()
        .with(eq(mapping_id.clone()))
        .times(1)
        .returning(move |_| {
            Ok(CourseCategory {
                id: mapping_id.clone(),
                canvas_course_id: "canvas-123".to_string(),
                discourse_category_id: "discourse-456".to_string(),
                name: "Test Mapping".to_string(),
                last_sync: Utc::now(),
                sync_enabled: true,
                sync_direction: SyncDirection::CanvasToDiscourse,
            })
        });
    
    mock_db
        .expect_save_course_category()
        .times(1)
        .returning(|_| Ok(()));
    
    // Create service with mocks
    let mapping_service = MappingService::new(
        std::sync::Arc::new(mock_db),
        std::sync::Arc::new(mock_canvas),
        std::sync::Arc::new(mock_discourse),
    );
    
    // Execute test - update name and sync direction
    let result = mapping_service.update_mapping(
        &mapping_id,
        Some(new_name),
        Some(false),
        Some(SyncDirection::Bidirectional),
    ).await;
    
    // Verify result
    assert!(result.is_ok());
    let mapping = result.unwrap();
    assert_eq!(mapping.id, mapping_id);
    assert_eq!(mapping.name, new_name);
    assert_eq!(mapping.sync_enabled, false);
    assert_eq!(mapping.sync_direction, SyncDirection::Bidirectional);
}

#[tokio::test]
async fn test_sync_mapping_canvas_to_discourse() {
    // Setup mocks
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Define test data
    let mapping_id = Uuid::new_v4().to_string();
    let canvas_id = "canvas-123";
    let discourse_id = "discourse-456";
    let course_name = "Canvas Course";
    let course_desc = "Canvas Description";
    
    // Set up mock behavior for initial mapping fetch
    mock_db
        .expect_get_course_category()
        .with(eq(mapping_id.clone()))
        .times(1)
        .returning(move |_| {
            Ok(CourseCategory {
                id: mapping_id.clone(),
                canvas_course_id: canvas_id.to_string(),
                discourse_category_id: discourse_id.to_string(),
                name: "Test Mapping".to_string(),
                last_sync: Utc::now() - chrono::Duration::hours(1), // 1 hour ago
                sync_enabled: true,
                sync_direction: SyncDirection::CanvasToDiscourse,
            })
        });
    
    // Expect Canvas API call to get course data
    mock_canvas
        .expect_get_course()
        .with(eq(canvas_id))
        .times(1)
        .returning(move |_| {
            Ok(crate::api::canvas::CourseData {
                id: canvas_id.to_string(),
                name: course_name.to_string(),
                description: course_desc.to_string(),
                updated_at: Utc::now(),
            })
        });
    
    // Expect Discourse API call to update category with course data
    mock_discourse
        .expect_update_category()
        .with(eq(discourse_id), eq(course_name), eq(course_desc))
        .times(1)
        .returning(|_, _, _| Ok(()));
    
    // Expect database update to save sync time
    mock_db
        .expect_save_course_category()
        .times(1)
        .returning(|_| Ok(()));
    
    // Create service with mocks
    let mapping_service = MappingService::new(
        std::sync::Arc::new(mock_db),
        std::sync::Arc::new(mock_canvas),
        std::sync::Arc::new(mock_discourse),
    );
    
    // Execute test
    let result = mapping_service.sync_mapping(&mapping_id).await;
    
    // Verify result
    assert!(result.is_ok());
    let summary = result.unwrap();
    assert!(summary.operations.len() > 0);
}

#[tokio::test]
async fn test_sync_mapping_bidirectional() {
    // Setup mocks
    let mut mock_canvas = MockCanvasClient::new();
    let mut mock_discourse = MockDiscourseClient::new();
    let mut mock_db = MockDatabase::new();
    
    // Define test data
    let mapping_id = Uuid::new_v4().to_string();
    let canvas_id = "canvas-123";
    let discourse_id = "discourse-456";
    
    // Set up mock behavior for initial mapping fetch
    mock_db
        .expect_get_course_category()
        .with(eq(mapping_id.clone()))
        .times(1)
        .returning(move |_| {
            Ok(CourseCategory {
                id: mapping_id.clone(),
                canvas_course_id: canvas_id.to_string(),
                discourse_category_id: discourse_id.to_string(),
                name: "Test Mapping".to_string(),
                last_sync: Utc::now() - chrono::Duration::hours(1), // 1 hour ago
                sync_enabled: true,
                sync_direction: SyncDirection::Bidirectional,
            })
        });
    
    // Expect Canvas API call to get course data
    mock_canvas
        .expect_get_course()
        .with(eq(canvas_id))
        .times(1)
        .returning(move |_| {
            Ok(crate::api::canvas::CourseData {
                id: canvas_id.to_string(),
                name: "Canvas Course".to_string(),
                description: "Canvas Description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    // Expect Discourse API call to get category data
    mock_discourse
        .expect_get_category()
        .with(eq(discourse_id))
        .times(1)
        .returning(move |_| {
            Ok(crate::api::discourse::CategoryData {
                id: discourse_id.to_string(),
                name: "Discourse Category".to_string(),
                description: "Discourse Description".to_string(),
                updated_at: Utc::now(),
            })
        });
    
    // Expect API calls for bidirectional sync
    mock_discourse
        .expect_update_category()
        .times(1)
        .returning(|_, _, _| Ok(()));
    
    mock_canvas
        .expect_update_course()
        .times(1)
        .returning(|_, _, _| Ok(()));
    
    // Expect database update to save sync time
    mock_db
        .expect_save_course_category()
        .times(1)
        .returning(|_| Ok(()));
    
    // Create service with mocks
    let mapping_service = MappingService::new(
        std::sync::Arc::new(mock_db),
        std::sync::Arc::new(mock_canvas),
        std::sync::Arc::new(mock_discourse),
    );
    
    // Execute test
    let result = mapping_service.sync_mapping(&mapping_id).await;
    
    // Verify result
    assert!(result.is_ok());
    let summary = result.unwrap();
    assert!(summary.operations.len() >= 2); // Should have at least 2 operations for bidirectional sync
}
