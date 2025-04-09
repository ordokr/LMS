#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;
    use crate::api::integration_settings::{
        get_course_integration_settings,
        update_course_integration_settings,
        disconnect_course_from_canvas
    };
    use crate::models::integration_settings::CourseIntegrationSettings;
    use crate::tests::common::{setup, create_test_course};

    #[tokio::test]
    async fn test_default_settings() {
        let pool = setup();
        let course_id = create_test_course(&pool).await;
        
        // Get default integration settings
        let settings = get_course_integration_settings(course_id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to get default integration settings");
            
        // Verify default values
        assert_eq!(settings.course_id, course_id);
        assert_eq!(settings.canvas_course_id, None);
        assert_eq!(settings.auto_sync_enabled, false);
        assert_eq!(settings.sync_frequency_hours, Some(24)); // Default to daily
        assert_eq!(settings.sync_modules, true);
        assert_eq!(settings.sync_assignments, true);
        assert_eq!(settings.sync_discussions, true);
        assert_eq!(settings.sync_files, true);
        assert_eq!(settings.sync_announcements, true);
        assert_eq!(settings.last_sync, None);
    }
    
    #[tokio::test]
    async fn test_update_settings() {
        let pool = setup();
        let course_id = create_test_course(&pool).await;
        
        // Get default settings first
        let default_settings = get_course_integration_settings(course_id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to get default integration settings");
            
        // Modify settings
        let mut updated_settings = default_settings.clone();
        updated_settings.canvas_course_id = Some("canvas-123".to_string());
        updated_settings.auto_sync_enabled = true;
        updated_settings.sync_frequency_hours = Some(12);
        updated_settings.sync_discussions = false;
        
        // Update the settings
        let result = update_course_integration_settings(
            updated_settings.clone(), 
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to update integration settings");
        
        // Verify updated values
        assert_eq!(result.canvas_course_id, Some("canvas-123".to_string()));
        assert_eq!(result.auto_sync_enabled, true);
        assert_eq!(result.sync_frequency_hours, Some(12));
        assert_eq!(result.sync_discussions, false);
        assert_eq!(result.sync_modules, true); // unchanged
        
        // Fetch again to double check persistence
        let fetched_settings = get_course_integration_settings(course_id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to fetch updated integration settings");
            
        assert_eq!(fetched_settings.canvas_course_id, Some("canvas-123".to_string()));
        assert_eq!(fetched_settings.auto_sync_enabled, true);
        assert_eq!(fetched_settings.sync_frequency_hours, Some(12));
    }
    
    #[tokio::test]
    async fn test_disconnect_from_canvas() {
        let pool = setup();
        let course_id = create_test_course(&pool).await;
        
        // First set up a connection
        let mut settings = get_course_integration_settings(course_id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to get default integration settings");
            
        settings.canvas_course_id = Some("canvas-123".to_string());
        settings.auto_sync_enabled = true;
        
        update_course_integration_settings(
            settings, 
            sqlx::pool::PoolConnection::from(pool.clone())
        )
        .await
        .expect("Failed to update integration settings");
        
        // Now disconnect
        disconnect_course_from_canvas(course_id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to disconnect from Canvas");
            
        // Verify disconnection
        let updated_settings = get_course_integration_settings(course_id.clone(), sqlx::pool::PoolConnection::from(pool.clone()))
            .await
            .expect("Failed to fetch updated settings");
            
        assert_eq!(updated_settings.canvas_course_id, None);
        assert_eq!(updated_settings.auto_sync_enabled, false);
    }
}