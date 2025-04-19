pub mod sync;
pub mod integration;
pub mod notification;

// Unified services
pub mod unified_services;
pub mod unified_discussion_sync;

pub use sync::*;
pub use integration::*;
pub use notification::*;
pub use unified_discussion_sync::*;

// Re-export unified services
pub use unified_services::{Service, ServiceConfig, BaseService};
pub use unified_services::{AuthService, NotificationService, Notification, NotificationType};
pub use unified_services::{get_service_registry, init_services, shutdown_services, health_check_services, get_service};