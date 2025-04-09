use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use crate::db::DB;
use crate::error::Error;

/// User preferences for customizing the application
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub theme: String,  // light, dark, system
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub forum_digest: bool,
    pub language: String,
    pub time_zone: String,
    pub date_format: String,
    pub compact_view: bool,
}

impl UserPreferences {
    pub fn new(user_id: Uuid) -> Self {
        UserPreferences {
            id: Uuid::new_v4(),
            user_id,
            theme: "system".to_string(),
            email_notifications: true,
            push_notifications: true,
            forum_digest: true,
            language: "en".to_string(),
            time_zone: "UTC".to_string(),
            date_format: "YYYY-MM-DD".to_string(),
            compact_view: false,
        }
    }

    // Database operations
    
    pub async fn find_by_user_id(db: &DB, user_id: Uuid) -> Result<Self, Error> {
        let prefs = sqlx::query_as::<_, UserPreferences>(
            "SELECT * FROM user_preferences WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(prefs)
    }
    
    pub async fn create(db: &DB, prefs: &UserPreferences) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO user_preferences 
            (id, user_id, theme, email_notifications, push_notifications, 
            forum_digest, language, time_zone, date_format, compact_view)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(prefs.id)
        .bind(prefs.user_id)
        .bind(&prefs.theme)
        .bind(prefs.email_notifications)
        .bind(prefs.push_notifications)
        .bind(prefs.forum_digest)
        .bind(&prefs.language)
        .bind(&prefs.time_zone)
        .bind(&prefs.date_format)
        .bind(prefs.compact_view)
        .execute(&db.pool)
        .await?;
        
        Ok(prefs.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE user_preferences SET
            theme = ?, email_notifications = ?, push_notifications = ?,
            forum_digest = ?, language = ?, time_zone = ?, 
            date_format = ?, compact_view = ?
            WHERE id = ?"
        )
        .bind(&self.theme)
        .bind(self.email_notifications)
        .bind(self.push_notifications)
        .bind(self.forum_digest)
        .bind(&self.language)
        .bind(&self.time_zone)
        .bind(&self.date_format)
        .bind(self.compact_view)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
}