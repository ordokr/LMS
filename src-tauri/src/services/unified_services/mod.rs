// Unified services module
// This module contains consolidated service implementations that replace redundant services

mod base_service;
mod auth_service;
mod notification_service;

// Re-export base service
pub use base_service::{
    Service, ServiceConfig, BaseService, ServiceRegistry,
    get_service_registry, init_services, shutdown_services, health_check_services, get_service,
};

// Re-export auth service
pub use auth_service::AuthService;

// Re-export notification service
pub use notification_service::{
    NotificationService, Notification, NotificationType,
};
