use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPermissionMapping {
    pub id: Uuid,
    pub canvas_user_id: String,
    pub discourse_user_id: String,
    pub canvas_role: String,
    pub discourse_group: String,
    pub sync_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

impl UserPermissionMapping {
    pub fn new(
        canvas_user_id: String, 
        discourse_user_id: String,
        canvas_role: String,
        discourse_group: String
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            canvas_user_id,
            discourse_user_id,
            canvas_role,
            discourse_group,
            sync_enabled: true,
            created_at: now,
            updated_at: now,
            last_synced_at: None,
        }
    }
    
    pub fn update_sync_time(&mut self) {
        self.last_synced_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}