use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a Discourse Badge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub badge_type_id: i64, // e.g., Bronze, Silver, Gold
    pub badge_grouping_id: Option<i64>,
    pub image_upload_id: Option<i64>,
    pub system: bool,
    pub enabled: bool,
    pub multiple_grant: bool,
    pub allow_title: bool,
    pub target_posts: bool,
    pub show_posts: bool,
    pub auto_revoke: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Badge {
    pub fn new(id: i64, name: String, badge_type_id: i64) -> Self {
        Self {
            id,
            name,
            description: None,
            badge_type_id,
            badge_grouping_id: None,
            image_upload_id: None,
            system: false,
            enabled: true,
            multiple_grant: false,
            allow_title: false,
            target_posts: false,
            show_posts: false,
            auto_revoke: false,
            created_at: None,
            updated_at: None,
        }
    }
}
