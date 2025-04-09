use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseCategoryMapping {
    pub id: i64,
    pub course_id: i64,
    pub category_id: i64,
    pub sync_enabled: bool,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CourseCategoryMapping {
    pub fn new(course_id: i64, category_id: i64) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Will be set by the database
            course_id,
            category_id,
            sync_enabled: true,
            last_synced_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}