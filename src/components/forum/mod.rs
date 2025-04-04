mod forum_categories;
mod forum_threads;
mod thread_detail;
mod topic_form;
mod category_form;
mod category_detail;
mod rich_editor;
mod forum_search;
mod user_profile;
mod profile_edit;
mod notification_indicator;
mod all_notifications;
mod tag_selector;
mod tag_browser;
mod tag_detail;
mod tag_management;
mod tag_following;
mod tag_feed;

// Add admin module
mod admin {
    mod admin_dashboard;
    mod user_management;
    mod content_moderation;
    mod forum_settings;
    mod reported_content;
    mod activity_log;
    mod admin_layout;
    
    pub use admin_dashboard::AdminDashboard;
    pub use user_management::UserManagement;
    pub use content_moderation::ContentModeration;
    pub use forum_settings::ForumSettings;
    pub use reported_content::ReportedContent;
    pub use activity_log::ActivityLog;
    pub use admin_layout::AdminLayout;
}

pub use forum_categories::ForumCategories;
pub use forum_threads::ForumThreads;
pub use thread_detail::ThreadDetail;
pub use topic_form::TopicForm;
pub use category_form::CategoryForm;
pub use category_detail::CategoryDetail;
pub use rich_editor::RichEditor;
pub use forum_search::ForumSearch;
pub use user_profile::UserProfile;
pub use profile_edit::ProfileEdit;
pub use notification_indicator::NotificationIndicator;
pub use all_notifications::AllNotifications;
pub use tag_selector::TagSelector;
pub use tag_browser::TagBrowser;
pub use tag_detail::TagDetail;
pub use tag_management::TagManagement;
pub use tag_following::TagFollowing;
pub use tag_feed::TagFeed;

// Export admin components
pub use admin::*;

mod group_management;
pub use group_management::GroupManagement;

mod categories;
mod threads;
mod posts;

pub use categories::*;
pub use threads::*;
pub use posts::*;