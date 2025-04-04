use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::models::user::UserRole;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForumSettings {
    pub forum_name: String,
    pub forum_description: String,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: String,
    pub allow_guest_view: bool,
    pub require_email_verification: bool,
    pub allow_registration: bool, 
    pub allow_file_uploads: bool,
    pub max_upload_size_mb: i32,
    pub allowed_file_types: Vec<String>,
    pub topics_per_page: i32,
    pub posts_per_page: i32,
    pub min_chars_per_post: i32,
    pub max_chars_per_post: i32,
    pub enable_user_signatures: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForumSettingsUpdate {
    pub forum_name: String,
    pub forum_description: String,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: String,
    pub allow_guest_viewing: bool,
    pub allow_registration: bool,
    pub require_email_verification: bool,
    pub posts_per_page: i32,
    pub topics_per_page: i32,
    pub max_topic_title_length: i32,
    pub min_post_length: i32,
    pub max_post_length: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ReportStatus {
    Pending,
    Resolved,
    Dismissed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ReportDecision {
    Remove,
    Warn,
    Approve,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportedContent {
    pub id: i64,
    pub reporter_id: i64,
    pub reporter_name: String,
    pub content_id: i64,
    pub content_type: String,
    pub parent_id: Option<i64>,
    pub content_title: Option<String>,
    pub content_excerpt: Option<String>,
    pub reason: String,
    pub details: Option<String>,
    pub status: ReportStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_by: Option<i64>,
    pub resolver_name: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub decision: Option<ReportDecision>,
    pub resolution_note: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    UserLogin,
    UserLogout,
    UserRegistration,
    UserProfileUpdate,
    TopicCreated,
    TopicUpdated,
    TopicDeleted,
    PostCreated,
    PostUpdated,
    PostDeleted,
    CategoryCreated,
    CategoryUpdated,
    CategoryDeleted,
    ModeratorAction,
    AdminAction,
    SystemEvent,
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: i64,
    pub user_id: i64,
    pub username: Option<String>,
    pub ip_address: Option<String>,
    pub activity_type: ActivityType,
    pub target_id: Option<i64>,
    pub target_name: Option<String>,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityLogPage {
    pub logs: Vec<ActivityLog>,
    pub total: usize,
    pub page: usize,
    pub total_pages: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub active_users: i64,
    pub new_users_today: i64,
    pub total_topics: i64,
    pub new_topics_today: i64,
    pub total_posts: i64,
    pub new_posts_today: i64,
    pub total_page_views: i64,
    pub page_views_today: i64,
    pub pending_reports_count: Option<i64>,
    pub popular_topics: Vec<PopularTopic>,
    pub top_contributors: Vec<TopContributor>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopularTopic {
    pub id: i64,
    pub title: String,
    pub view_count: i32,
    pub author_name: String,
    pub category_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopContributor {
    pub id: i64,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub post_count: i32,
    pub topic_count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityData {
    pub time_series: Vec<TimeSeriesData>,
    pub distribution: Vec<DistributionData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeSeriesData {
    pub date: String,
    pub posts: i32,
    pub topics: i32,
    pub users: i32,
    pub views: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DistributionData {
    pub label: String,
    pub value: i32,
    pub color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserManagementPage {
    pub users: Vec<crate::models::user::User>,
    pub total: usize,
    pub page: usize,
    pub total_pages: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub enable_email: bool,
    pub smtp_host: String,
    pub smtp_port: i32,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
    pub smtp_from_name: String,
    pub smtp_use_tls: bool,
    
    pub email_welcome_enabled: bool,
    pub email_welcome_subject: String,
    
    pub email_post_reply_enabled: bool,
    pub email_post_reply_subject: String,
    
    pub email_topic_reply_enabled: bool,
    pub email_topic_reply_subject: String,
    
    pub email_mention_enabled: bool,
    pub email_mention_subject: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserGroup {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_visible: bool,
    pub is_public: bool,
    pub can_self_assign: bool,
    pub member_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserGroupCreate {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_visible: bool,
    pub is_public: bool,
    pub can_self_assign: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserGroupUpdate {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_visible: bool,
    pub is_public: bool,
    pub can_self_assign: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupMember {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

/// Represents a group membership in Canvas
/// Based on Canvas's GroupMembership model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub id: i64,
    pub group_id: Option<i64>,
    pub user_id: Option<i64>,
    pub workflow_state: Option<String>,
    pub moderator: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub just_created: Option<bool>,
    pub sis_batch_id: Option<i64>,
    pub sis_import_id: Option<i64>,
}

impl GroupMember {
    pub fn new() -> Self {
        Self {
            id: 0,
            group_id: None,
            user_id: None,
            workflow_state: None,
            moderator: None,
            created_at: None,
            updated_at: None,
            just_created: None,
            sis_batch_id: None,
            sis_import_id: None,
        }
    }
    
    /// Create a new membership
    pub fn create(group_id: i64, user_id: i64) -> Result<Self, String> {
        // Implementation would connect to backend service
        let new_member = Self {
            id: 0, // Would be set by backend
            group_id: Some(group_id),
            user_id: Some(user_id),
            workflow_state: Some("accepted".to_string()),
            moderator: Some(false),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            just_created: Some(true),
            sis_batch_id: None,
            sis_import_id: None,
        };
        
        Ok(new_member)
    }
    
    /// Get the group for this membership
    pub fn group(&self) -> Option<crate::models::forum::Group> {
        // Implementation would connect to backend service
        None
    }
    
    /// Get the user for this membership
    pub fn user(&self) -> Option<User> {
        // Implementation would connect to backend service
        None
    }
    
    /// Accept an invitation to a group
    pub fn accept(&mut self) -> Result<bool, String> {
        self.workflow_state = Some("accepted".to_string());
        self.updated_at = Some(Utc::now());
        // Implementation would connect to backend service to persist
        Ok(true)
    }
    
    /// Reject an invitation to a group
    pub fn reject(&mut self) -> Result<bool, String> {
        self.workflow_state = Some("rejected".to_string());
        self.updated_at = Some(Utc::now());
        // Implementation would connect to backend service to persist
        Ok(true)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteCustomization {
    pub site_name: String,
    pub site_tagline: Option<String>,
    pub site_description: Option<String>,
    pub site_logo_url: Option<String>,
    pub site_favicon_url: Option<String>,
    
    pub primary_color: String,
    pub secondary_color: String,
    pub success_color: String,
    pub info_color: String,
    pub warning_color: String,
    pub danger_color: String,
    pub background_color: String,
    pub text_color: String,
    
    pub heading_font: String,
    pub body_font: String,
    pub code_font: String,
    pub base_font_size: i32,
    
    pub border_radius: f64,
    pub button_style: String,
    pub card_style: String,
    
    pub custom_css: Option<String>,
    pub custom_header_html: Option<String>,
    pub custom_footer_html: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    pub include_users: bool,
    pub include_categories: bool,
    pub include_tags: bool,
    pub include_topics: bool,
    pub include_posts: bool,
    pub include_uploads: bool,
    pub include_settings: bool,
    pub format: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportOptions {
    pub merge_users: bool,
    pub replace_categories: bool,
    pub replace_tags: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportStats {
    pub users_imported: usize,
    pub categories_imported: usize,
    pub tags_imported: usize,
    pub topics_imported: usize,
    pub posts_imported: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub filename: String,
    pub size: usize,
    pub format: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Represents a system setting in the admin panel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub id: i64,
    pub name: Option<String>,
    pub value: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            value: None,
            description: None,
            created_at: None,
            updated_at: None,
        }
    }
    
    /// Get a setting by name
    pub fn get(name: &str) -> Option<Self> {
        // Implementation would connect to backend service
        None
    }
    
    /// Set a setting value
    pub fn set(name: &str, value: &str) -> Result<Self, String> {
        // Implementation would connect to backend service
        Err("Not implemented".to_string())
    }
}