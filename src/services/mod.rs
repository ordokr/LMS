mod forum_service;
mod course_service;
mod auth_service;
mod integration_service;

pub use forum_service::ForumService;
pub use course_service::CourseService;
pub use auth_service::AuthService;
pub use integration_service::IntegrationService;

pub mod user;
pub mod auth;
pub mod forum;
pub mod admin;
pub mod notification; // Add this line
pub mod api;

pub use user::UserService;
pub use auth::AuthService;
pub use forum::ForumService;
pub use admin::AdminService;
pub use notification::NotificationService; // Add this line