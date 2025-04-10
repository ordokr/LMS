use std::sync::Arc;
use tokio::sync::Mutex;
use crate::services::sync::{UserSyncService, CourseSyncService, AssignmentSyncService};
use crate::core::errors::AppError;
use crate::storage::connection_status::ConnectionStatus;
use log::{info, error};

pub struct SyncManager {
    user_sync: Arc<UserSyncService>,
    course_sync: Arc<CourseSyncService>,
    assignment_sync: Arc<AssignmentSyncService>,
    connection_status: Arc<ConnectionStatus>,
    sync_lock: Arc<Mutex<()>>,
}

impl SyncManager {
    pub fn new(
        user_sync: Arc<UserSyncService>,
        course_sync: Arc<CourseSyncService>,
        assignment_sync: Arc<AssignmentSyncService>,
        connection_status: Arc<ConnectionStatus>,
    ) -> Self {
        Self {
            user_sync,
            course_sync,
            assignment_sync,
            connection_status,
            sync_lock: Arc::new(Mutex<()>),
        }
    }
    
    pub async fn sync_all(&self) -> Result<SyncSummary, AppError> {
        // Ensure we don't have concurrent syncs
        let _lock = self.sync_lock.lock().await;
        
        info!("Starting full synchronization");
        
        if !self.connection_status.is_online() {
            info!("Device is offline. Skipping sync.");
            return Ok(SyncSummary {
                success: false,
                users_synced: 0,
                courses_synced: 0,
                assignments_synced: 0,
                errors: vec!["Device is offline".to_string()],
            });
        }
        
        let mut summary = SyncSummary {
            success: true,
            users_synced: 0,
            courses_synced: 0,
            assignments_synced: 0,
            errors: Vec::new(),
        };
        
        // Sync users
        match self.user_sync.sync_all_users().await {
            Ok(users) => {
                summary.users_synced = users.len();
                info!("Synced {} users", users.len());
            },
            Err(e) => {
                error!("Error syncing users: {}", e);
                summary.errors.push(format!("User sync error: {}", e));
                summary.success = false;
            }
        }
        
        // Sync courses
        match self.course_sync.sync_all_courses().await {
            Ok(courses) => {
                summary.courses_synced = courses.len();
                info!("Synced {} courses", courses.len());
            },
            Err(e) => {
                error!("Error syncing courses: {}", e);
                summary.errors.push(format!("Course sync error: {}", e));
                summary.success = false;
            }
        }
        
        // Sync assignments
        match self.assignment_sync.sync_all_assignments().await {
            Ok(assignments) => {
                summary.assignments_synced = assignments.len();
                info!("Synced {} assignments", assignments.len());
            },
            Err(e) => {
                error!("Error syncing assignments: {}", e);
                summary.errors.push(format!("Assignment sync error: {}", e));
                summary.success = false;
            }
        }
        
        info!("Completed full synchronization with status: {}", 
            if summary.success { "SUCCESS" } else { "PARTIAL FAILURE" });
        
        Ok(summary)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSummary {
    pub success: bool,
    pub users_synced: usize,
    pub courses_synced: usize,
    pub assignments_synced: usize,
    pub errors: Vec<String>,
}