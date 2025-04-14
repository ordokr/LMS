use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: String, // "success", "error", "warning", "info"
    pub created_at: String,
    pub read: bool,
    pub action_url: Option<String>,
    pub action_text: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
}
