use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationStatus {
    pub connected: bool,
    pub last_sync: Option<String>,
    pub pending_syncs: u32,
    pub sync_in_progress: bool,
    pub sync_errors: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseTopic {
    pub id: String,
    pub title: String,
    pub category: Option<String>,
    pub post_count: usize,
    pub sync_status: String,
    pub last_synced_at: Option<String>,
    pub discourse_topic_id: Option<i64>,
    pub discourse_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub topic_count: Option<usize>,
    pub parent_id: Option<String>,
    pub permissions: Option<String>,
    pub sync_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHistoryEntry {
    pub id: String,
    pub sync_type: String,
    pub content_type: String,
    pub content_id: String,
    pub sync_time: String,
    pub success: bool,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub title: String,
    pub canvas_updated_at: String,
    pub discourse_updated_at: String,
    pub canvas_content: String,
    pub discourse_content: String,
    pub detected_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStrategy {
    PreferCanvas,
    PreferDiscourse,
    PreferMostRecent,
    MergePreferCanvas,
    MergePreferDiscourse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub sync_status: SyncStatus,
    pub canvas_topic_id: Option<String>,
    pub discourse_topic_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub sync_status: SyncStatus,
    pub canvas_course_id: Option<String>,
    pub discourse_category_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Synced,
    PendingToCanvas,
    PendingToDiscourse,
    Conflict,
    Error,
    LocalOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasCourse {
    pub id: String,
    pub name: String,
    pub course_code: String,
    pub term: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub sync_status: String,
    pub last_synced_at: Option<String>,
    pub canvas_course_id: Option<i64>,
    pub discourse_category_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasAssignment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub course_id: String,
    pub course_name: String,
    pub due_at: Option<String>,
    pub points_possible: f64,
    pub submission_types: Vec<String>,
    pub sync_status: String,
    pub canvas_assignment_id: Option<i64>,
    pub discourse_topic_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSettings {
    pub canvas_api_url: Option<String>,
    pub canvas_api_token: Option<String>,
    pub discourse_api_url: Option<String>,
    pub discourse_api_key: Option<String>,
    pub discourse_username: Option<String>,
    pub sync_interval: Option<i32>,
    pub auto_sync_enabled: Option<bool>,
    pub notification_enabled: Option<bool>,
}
