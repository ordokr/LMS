#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    use crate::api::canvas::CourseData;
    use crate::api::discourse::CategoryData;

    // Create mocks for testing
    mock! {
        Database {}
        
        async fn save_course_category(&self, mapping: &CourseCategory) -> Result<(), Error>;
        async fn get_course_category(&self, id: &str) -> Result<CourseCategory, Error>;
        async fn mapping_exists(&self, canvas_course_id: &str, discourse_category_id: &str) -> Result<bool, Error>;
    }

    mock! {
        CanvasClient {}
        
        async fn get_course(&self, course_id: &str) -> Result<CourseData, Error>;
        async fn update_course(&self, course_id: &str, name: &str, description: &str) -> Result<(), Error>;
    }

    mock! {
        DiscourseClient {}
        
        async fn get_category(&self, category_id: &str) -> Result<CategoryData, Error>;
        async fn update_category(&self, category_id: &str, name: &str, description: &str) -> Result<(), Error>;
    }

    #[tokio::test]
    async fn test_bidirectional_sync() {
        // Create test mapping
        let mut mapping = CourseCategory::new(
            "canvas123",
            "discourse456",
            "Test Course"
        );
        mapping.sync_direction = SyncDirection::Bidirectional;
        
        // Create mocks
        let mut mock_db = MockDatabase::new();
        let mut mock_canvas = MockCanvasClient::new();
        let mut mock_discourse = MockDiscourseClient::new();
        
        // Setup Canvas mock
        mock_canvas
            .expect_get_course()
            .with(eq("canvas123"))
            .returning(|_| {
                Ok(CourseData {
                    id: "canvas123".to_string(),
                    name: "Canvas Course".to_string(),
                    description: "Canvas description".to_string(),
                    updated_at: Utc::now(),
                })
            });
        
        // Setup Discourse mock
        mock_discourse
            .expect_get_category()
            .with(eq("discourse456"))
            .returning(|_| {
                Ok(CategoryData {
                    id: "discourse456".to_string(),
                    name: "Discourse Category".to_string(),
                    description: "Discourse description".to_string(),
                    updated_at: Utc::now(),
                })
            });
        
        // Expect updates in both directions
        mock_discourse
            .expect_update_category()
            .with(eq("discourse456"), eq("Canvas Course"), eq("Canvas description"))
            .returning(|_, _, _| Ok(()));
        
        mock_canvas
            .expect_update_course()
            .with(eq("canvas123"), eq("Discourse Category"), eq("Discourse description"))
            .returning(|_, _, _| Ok(()));
        
        // Expect database save
        mock_db
            .expect_save_course_category()
            .returning(|_| Ok(()));
        
        // Execute sync
        let result = mapping.sync(
            &mock_db,
            &mock_canvas,
            &mock_discourse,
        ).await;
        
        // Verify result
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert_eq!(summary.operations.len(), 2);
        assert_eq!(summary.status, "Completed");
    }
    
    #[tokio::test]
    async fn test_canvas_to_discourse_sync() {
        // Create test mapping
        let mut mapping = CourseCategory::new(
            "canvas123",
            "discourse456",
            "Test Course"
        );
        mapping.sync_direction = SyncDirection::CanvasToDiscourse;
        
        // Create mocks
        let mut mock_db = MockDatabase::new();
        let mut mock_canvas = MockCanvasClient::new();
        let mut mock_discourse = MockDiscourseClient::new();
        
        // Setup Canvas mock
        mock_canvas
            .expect_get_course()
            .with(eq("canvas123"))
            .returning(|_| {
                Ok(CourseData {
                    id: "canvas123".to_string(),
                    name: "Canvas Course".to_string(),
                    description: "Canvas description".to_string(),
                    updated_at: Utc::now(),
                })
            });
        
        // Expect only Discourse update
        mock_discourse
            .expect_update_category()
            .with(eq("discourse456"), eq("Canvas Course"), eq("Canvas description"))
            .returning(|_, _, _| Ok(()));
        
        // Expect database save
        mock_db
            .expect_save_course_category()
            .returning(|_| Ok(()));
        
        // Execute sync
        let result = mapping.sync(
            &mock_db,
            &mock_canvas,
            &mock_discourse,
        ).await;
        
        // Verify result
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert_eq!(summary.operations.len(), 1);
        assert_eq!(summary.status, "Completed");
    }
}