// This file exposes the Tauri backend as a library
// It's referenced by the Cargo.toml workspace configuration

// Export the AppState
pub mod app_state;

// Export the protocol handler
pub mod protocol_handler;

// Export all the modules needed by other workspace members
pub mod api {
    pub mod auth;
    // Add other API modules as needed
}

pub mod core {
    pub mod auth;
    pub mod errors;
    // Add other core modules as needed
}

pub mod database {
    pub mod repositories {
        pub mod user;
        pub mod forum;
        pub mod course;
        pub mod quiz_repository;
        // Add other repositories as needed
    }
    pub mod init_quiz_db;
    // Add other database modules as needed
}

pub mod shared {
    pub mod models {
        pub mod user;
        // Add other model modules as needed
    }
    // Add other shared modules as needed
}

pub mod models {
    pub mod quiz;
}

pub mod utils {
    pub mod file_system;
    // Any utility modules
}

pub mod analyzers {
    pub mod project_structure;
    pub mod ast_analyzer;
    // Add other analyzer modules as needed
}

pub mod ai {
    pub mod gemini_analyzer;
}

// Export the unified error handling system
pub mod errors;

// Re-export important types for easier access
pub use crate::api::auth::*;
pub use crate::core::auth::*;
pub use crate::core::errors::*;
pub use crate::database::repositories::user::*;
pub use crate::database::repositories::quiz_repository::*;
pub use crate::shared::models::user::*;
pub use crate::models::quiz::*;
pub use crate::app_state::AppState;

// Re-export unified error handling for easier access
pub use crate::errors::error::{Error, ErrorKind, ErrorCode, Result};
pub use crate::errors::api_error::{ApiError, ApiErrorKind, ApiResult};
pub use crate::errors::error_handler::{ErrorHandler, handle_error, handle_api_error, get_global_error_handler};
pub use crate::errors::error_context::{ErrorContext, with_context, with_context_fn};
pub use crate::errors::error_mapper::{ErrorMapper, map_error, map_api_error};