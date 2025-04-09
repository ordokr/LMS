#[cfg(test)]
mod user_profile_tests {
    use crate::models::user::User;
    use crate::models::user::profile::UserProfile;
    use crate::controllers::user_controller;
    use chrono::Utc;
    use sqlx::SqlitePool;
    use std::sync::Arc;

    #[sqlx::test]
    async fn test_get_user_profile() {
        // Set up test database
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        // Apply migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        // Insert test user
        let user_id = "test-user-id";
        let username = "testuser";
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, display_name, email, avatar_url, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            user_id,
            username,
            "Test User",
            "test@example.com",
            None::<String>,
            now.to_rfc3339()
        )
        .execute(&pool)
        .await
        .unwrap();
        
        // Call the function
        let db_state = tauri::State::new(pool);
        let result = user_controller::get_user_profile(username.to_string(), db_state)
            .await
            .unwrap();
        
        // Verify results
        let (user, profile) = result;
        
        // Check user fields
        assert_eq!(user.id, user_id);
        assert_eq!(user.username, username);
        assert_eq!(user.display_name, "Test User");
        
        // Check profile was created with default values
        assert_eq!(profile.user_id, user_id);
        assert_eq!(profile.profile_views, 0); // Should be incremented to 1 but this is before the increment
        assert_eq!(profile.trust_level, 0);
        assert_eq!(profile.is_moderator, false);
        assert_eq!(profile.is_admin, false);
        assert_eq!(profile.posts_count, 0);
    }
    
    #[sqlx::test]
    async fn test_update_user_profile() {
        // Set up test database
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        // Apply migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        // Insert test user
        let user_id = "test-user-id";
        let username = "testuser";
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, display_name, email, avatar_url, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            user_id,
            username,
            "Test User",
            "test@example.com",
            None::<String>,
            now.to_rfc3339()
        )
        .execute(&pool)
        .await
        .unwrap();
        
        // Update the profile
        let bio = Some("This is my bio".to_string());
        let location = Some("Test Location".to_string());
        
        let update = crate::models::user::profile::UserProfileUpdate {
            bio: bio.clone(),
            website: None,
            location: location.clone(),
            title: None,
            tag_line: None,
        };
        
        let db_state = tauri::State::new(pool);
        let updated_profile = user_controller::update_user_profile(user_id.to_string(), update, db_state)
            .await
            .unwrap();
        
        // Verify results
        assert_eq!(updated_profile.user_id, user_id);
        assert_eq!(updated_profile.bio, bio);
        assert_eq!(updated_profile.location, location);
    }
}