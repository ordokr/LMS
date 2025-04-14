use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Status of the integration between the LMS and Discourse
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationStatus {
    /// Whether the integration is currently connected
    pub connected: bool,
    
    /// Timestamp of the last successful synchronization
    pub last_sync: Option<String>,
    
    /// URL of the Discourse instance
    pub discourse_url: Option<String>,
    
    /// Number of topics that have been synchronized
    pub synced_topics_count: usize,
    
    /// Number of categories that have been synchronized
    pub synced_categories_count: usize,
    
    /// Number of synchronization operations in the last 24 hours
    pub sync_operations_24h: usize,
}

/// Represents a Discourse topic that has been synchronized with the LMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseTopic {
    /// Local ID of the topic in the LMS
    pub id: String,
    
    /// ID of the topic in Discourse
    pub discourse_topic_id: Option<i64>,
    
    /// Title of the topic
    pub title: String,
    
    /// Category the topic belongs to
    pub category: Option<String>,
    
    /// Number of posts in the topic
    pub post_count: i32,
    
    /// Status of synchronization for this topic
    pub sync_status: String,
    
    /// Timestamp when the topic was last synchronized
    pub last_synced_at: Option<String>,
    
    /// URL to view the topic in Discourse
    pub discourse_url: Option<String>,
    
    /// Whether the topic exists in Canvas
    pub exists_in_canvas: bool,
    
    /// Canvas entity ID if applicable
    pub canvas_entity_id: Option<String>,
    
    /// Canvas entity type if applicable (announcement, discussion, etc.)
    pub canvas_entity_type: Option<String>,
}

/// Represents a Discourse category that has been synchronized with the LMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseCategory {
    /// Local ID of the category in the LMS
    pub id: String,
    
    /// ID of the category in Discourse
    pub discourse_category_id: Option<i64>,
    
    /// Name of the category
    pub name: String,
    
    /// Description of the category
    pub description: Option<String>,
    
    /// Color used for the category in Discourse
    pub color: Option<String>,
    
    /// Number of topics in the category
    pub topic_count: Option<i32>,
    
    /// ID of the parent category if this is a subcategory
    pub parent_id: Option<i64>,
    
    /// Status of synchronization for this category
    pub sync_status: Option<String>,
    
    /// Permission settings for the category
    pub permissions: Option<String>,
    
    /// Canvas course ID if this category is mapped to a course
    pub canvas_course_id: Option<String>,
}

/// Represents an entry in the synchronization history log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHistoryEntry {
    /// ID of the sync event
    pub id: String,
    
    /// Whether the sync operation was successful
    pub success: bool,
    
    /// Type of sync operation (e.g., topic_to_discourse, post_from_discourse)
    pub sync_type: String,
    
    /// ID of the content that was synchronized
    pub content_id: String,
    
    /// Type of the content that was synchronized (topic, post, category, etc.)
    pub content_type: String,
    
    /// Timestamp when the sync operation occurred
    pub sync_time: String,
    
    /// Duration of the sync operation in milliseconds
    pub duration_ms: i32,
    
    /// Error message if the sync operation failed
    pub error_message: Option<String>,
    
    /// Direction of sync (to_discourse, from_discourse, bidirectional)
    pub direction: Option<String>,
}

/// Enum representing the possible synchronization status values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    /// Content is synchronized between LMS and Discourse
    Synced,
    
    /// Content is waiting to be synchronized
    Pending,
    
    /// Synchronization encountered an error
    Error,
    
    /// Content exists only in the local system
    LocalOnly,
    
    /// Content exists only in Discourse
    DiscourseOnly,
}

impl SyncStatus {
    /// Convert the sync status to a string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            SyncStatus::Synced => "Synced",
            SyncStatus::Pending => "Pending",
            SyncStatus::Error => "Error",
            SyncStatus::LocalOnly => "Local Only",
            SyncStatus::DiscourseOnly => "Discourse Only",
        }
    }
    
    /// Parse a string into a SyncStatus enum
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "synced" => SyncStatus::Synced,
            "pending" => SyncStatus::Pending,
            "error" => SyncStatus::Error,
            "local only" => SyncStatus::LocalOnly,
            "discourse only" => SyncStatus::DiscourseOnly,
            _ => SyncStatus::Pending,
        }
    }
}

impl ToString for SyncStatus {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
