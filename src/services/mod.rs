mod forum_service;
mod course_service;
mod auth_service;
mod integration_service;

pub use forum_service::ForumService;
pub use course_service::CourseService;
pub use auth_service::AuthService;
pub use integration_service::IntegrationService;

pub mod auth;
pub mod user;
pub mod forum;
pub mod admin; // Add this line to export admin service