#[cfg(test)]
mod tests {
    use crate::{
        api::forum_mapping::{create_mapping, get_mapping_by_course, CreateMappingRequest},
        models::mapping::CourseCategoryMapping,
        repository::course_category_repository::CourseCategoryRepository,
        AppState,
    };
    use axum::{
        extract::{Path, State},
        Json,
    };
    use sqlx::{Pool, Sqlite};

    #[sqlx::test]
    async fn test_create_mapping(pool: Pool<Sqlite>) {
        // Setup
        let repo = CourseCategoryRepository::new(pool.clone());
        let state = AppState {
            course_category_repo: repo,
            // Add other required fields for AppState...
        };
        
        let request = CreateMappingRequest {
            course_id: 123,
            category_id: 456,
        };
        
        // Execute
        let result = create_mapping(
            State(state),
            Json(request),
        ).await;
        
        // Verify
        assert!(result.is_ok());
        let response = result.unwrap();
        let mapping = response.0.mapping;
        
        assert_eq!(mapping.course_id, 123);
        assert_eq!(mapping.category_id, 456);
        assert!(mapping.sync_enabled);
        assert!(mapping.last_synced_at.is_none());
    }

    #[sqlx::test]
    async fn test_get_mapping_by_course(pool: Pool<Sqlite>) {
        // Setup
        let repo = CourseCategoryRepository::new(pool.clone());
        let state = AppState {
            course_category_repo: repo.clone(),
            // Add other required fields for AppState...
        };
        
        // Insert test data
        let mapping = repo.create(123, 456).await.unwrap();
        
        // Execute
        let result = get_mapping_by_course(
            State(state),
            Path(123),
        ).await;
        
        // Verify
        assert!(result.is_ok());
        let response = result.unwrap();
        let retrieved_mapping = response.0.mapping;
        
        assert_eq!(retrieved_mapping.id, mapping.id);
        assert_eq!(retrieved_mapping.course_id, 123);
        assert_eq!(retrieved_mapping.category_id, 456);
    }
}