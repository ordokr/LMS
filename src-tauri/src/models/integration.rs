use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategory {
    pub id: Uuid,
    pub canvas_course_id: String,
    pub discourse_category_id: i64,
    pub sync_enabled: bool,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategoryCreate {
    pub canvas_course_id: String,
    pub discourse_category_id: i64,
    pub sync_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategoryUpdate {
    pub sync_enabled: Option<bool>,
    pub last_synced_at: Option<DateTime<Utc>>,
}