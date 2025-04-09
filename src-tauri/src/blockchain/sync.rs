use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::blockchain::batching::{AdaptiveBatcher, SyncPriority, PendingChange};
use crate::blockchain::error::BlockchainError;

pub struct AdaptiveSyncManager {
    batcher: Arc<AdaptiveBatcher>,
}

impl AdaptiveSyncManager {
    pub fn new(batcher: Arc<AdaptiveBatcher>) -> Self {
        Self { batcher }
    }
    
    pub fn determine_sync_priority(&self, event: &UserEvent) -> SyncPriority {
        match event {
            UserEvent::GradeSubmission(_) => SyncPriority::Critical,
            UserEvent::CertificateIssuance(_) => SyncPriority::Critical,
            UserEvent::CourseCompletion(_) => SyncPriority::High,
            UserEvent::ForumPost(_) => SyncPriority::Background,
            UserEvent::ProfileUpdate(_) => SyncPriority::Background,
            UserEvent::BadgeAwarded(_) => SyncPriority::High,
            UserEvent::Custom(custom) => {
                // Determine priority based on custom event attributes
                // For example, check if it contains keywords like "exam", "grade", etc.
                if custom.metadata.contains("exam") || custom.metadata.contains("certificate") {
                    SyncPriority::Critical
                } else if custom.metadata.contains("assignment") || custom.metadata.contains("badge") {
                    SyncPriority::High
                } else {
                    SyncPriority::Background
                }
            }
        }
    }
    
    pub async fn sync_event(&self, event: &UserEvent) -> Result<(), BlockchainError> {
        // Determine priority
        let priority = self.determine_sync_priority(event);
        
        // Serialize event
        let data = bincode::serialize(event)
            .map_err(|e| BlockchainError::Serialization(e.to_string()))?;
        
        // Create pending change
        let change = PendingChange {
            id: 0, // Will be assigned by database
            change_type: event.event_type().to_string(),
            data,
            priority: priority as i32,
            created_at: chrono::Utc::now().timestamp(),
        };
        
        // Add to batcher
        self.batcher.add_change(&change, priority).await
            .map_err(|e| BlockchainError::Database(e))?;
        
        Ok(())
    }
    
    // Check if a sync is already pending for this event type
    pub async fn is_sync_pending(&self, event_type: &str) -> Result<bool, BlockchainError> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM pending_changes WHERE type = ?",
            event_type
        )
        .fetch_one(&self.batcher.db_pool)
        .await
        .map_err(|e| BlockchainError::Database(e))?
        .count as usize;
        
        Ok(count > 0)
    }
    
    // Force immediate synchronization of an event
    pub async fn force_sync(&self, event: &UserEvent) -> Result<(), BlockchainError> {
        // Create the change
        let change = PendingChange {
            id: 0,
            change_type: event.event_type().to_string(),
            data: bincode::serialize(event)
                .map_err(|e| BlockchainError::Serialization(e.to_string()))?,
            priority: SyncPriority::Critical as i32,
            created_at: chrono::Utc::now().timestamp(),
        };
        
        // Add as critical priority
        self.batcher.add_change(&change, SyncPriority::Critical).await
            .map_err(|e| BlockchainError::Database(e))?;
        
        // Trigger immediate processing
        self.batcher.check_critical_batch().await;
        
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UserEvent {
    GradeSubmission(GradeSubmission),
    CertificateIssuance(CertificateIssuance),
    CourseCompletion(CourseCompletion),
    ForumPost(ForumPost),
    ProfileUpdate(ProfileUpdate),
    BadgeAwarded(BadgeAwarded),
    Custom(CustomEvent),
}

impl UserEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            UserEvent::GradeSubmission(_) => "grade_submission",
            UserEvent::CertificateIssuance(_) => "certificate_issuance",
            UserEvent::CourseCompletion(_) => "course_completion",
            UserEvent::ForumPost(_) => "forum_post",
            UserEvent::ProfileUpdate(_) => "profile_update",
            UserEvent::BadgeAwarded(_) => "badge_awarded",
            UserEvent::Custom(_) => "custom_event",
        }
    }
}

// Event definitions
#[derive(Clone, Serialize, Deserialize)]
pub struct GradeSubmission {
    pub student_id: String,
    pub course_id: String,
    pub assignment_id: String,
    pub grade: f64,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CertificateIssuance {
    pub student_id: String,
    pub course_id: String,
    pub certificate_id: String,
    pub issue_date: i64,
    pub metadata: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CourseCompletion {
    pub student_id: String,
    pub course_id: String,
    pub completion_date: i64,
    pub grade: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ForumPost {
    pub user_id: String,
    pub thread_id: String,
    pub post_id: String,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProfileUpdate {
    pub user_id: String,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BadgeAwarded {
    pub student_id: String,
    pub course_id: String,
    pub badge_id: String,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomEvent {
    pub event_id: String,
    pub user_id: String,
    pub related_id: String,
    pub event_name: String,
    pub metadata: String,
    pub timestamp: i64,
}