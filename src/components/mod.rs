mod home;
mod layout;
mod auth;
mod lms;
mod forum;
mod shared;
pub mod sync_status;

// Add the new module item manager component
pub mod module_item_manager;

// Add this to your existing component exports
pub mod course_integration_settings;

// Add these to your existing module definitions
pub mod sync_history;

pub use home::Home;
pub use layout::AppLayout as Layout; // Export as Layout if that's what's used in routes
pub use auth::{
    LoginForm as Login,     // Map to route-expected names
    RegisterForm as Register,
    UserProfile
};
pub use lms::{
    CoursesList, CourseDetail, CourseForm,
    AssignmentsList, AssignmentDetail, AssignmentForm,
    ModulesList, ModuleDetail, ModuleForm, ModuleItemForm
};
pub use forum::{
    ForumCategories as CategoriesList,  // Map to expected route component names
    ForumThreads as TopicsList,
    ThreadDetail as TopicDetail,
    CategoryForm,
    CategoryDetail,
    TopicForm
};
pub use shared::{OfflineIndicator, ErrorDisplay};
pub use sync_status::SyncStatus;

// Re-export components
pub use module_item_manager::ModuleItemManager;

// Re-export for easier importing
pub use course_integration_settings::CourseIntegrationSettings;

// Re-export for easier imports
pub use sync_status::SyncStatusMonitor;
pub use sync_history::SyncHistory;

// You'll need to create these missing components
// or map them to existing ones