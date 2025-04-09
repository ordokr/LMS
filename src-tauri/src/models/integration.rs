use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncDirection {
    CanvasToDiscourse,
    DiscourseToCanvas,
    Bidirectional,
}

impl Default for SyncDirection {
    fn default() -> Self {
        Self::Bidirectional
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategory {
    pub id: Uuid,
    pub canvas_course_id: String,
    pub discourse_category_id: i64,
    pub sync_enabled: bool,
    pub sync_direction: SyncDirection,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategoryMapping {
    pub id: String,
    pub course_id: String,
    pub category_id: String,
    pub sync_topics: bool,
    pub sync_assignments: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategoryCreate {
    pub course_id: String,
    pub category_id: String,
    pub sync_topics: Option<bool>,
    pub sync_assignments: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategoryUpdate {
    pub sync_enabled: Option<bool>,
    pub sync_direction: Option<SyncDirection>,
    pub last_synced_at: Option<DateTime<Utc>>,
}