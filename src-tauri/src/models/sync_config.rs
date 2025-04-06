use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub enabled: bool,
    pub sync_interval_seconds: u32,      // How often to perform sync
    pub check_interval_seconds: u32,     // How often to check if sync is needed
    pub max_retries: u32,
    pub retry_delay_seconds: u32,
    pub sync_courses: bool,
    pub sync_discussions: bool,
    pub sync_users: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sync_interval_seconds: 3600,    // 1 hour
            check_interval_seconds: 300,    // 5 minutes
            max_retries: 3,
            retry_delay_seconds: 600,       // 10 minutes
            sync_courses: true,
            sync_discussions: true,
            sync_users: false,             // User sync not implemented yet
        }
    }
}