//! LMS Integration library
//! 
//! This library provides integration between Canvas and Discourse learning management systems
//! through a Rust implementation.

pub mod api;
pub mod models;
pub mod services;

/// Re-exports of common components for easier access
pub use api::{canvas_client::CanvasClient, discourse_client::DiscourseClient};
pub use models::{sync_state::SyncState, sync_transaction::SyncTransaction};
pub use services::integration::sync_service::SyncService;
