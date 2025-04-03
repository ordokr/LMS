// This file exposes the Tauri backend as a library
// It's referenced by the Cargo.toml workspace configuration

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
        // Add other repositories as needed
    }
    // Add other database modules as needed
}

pub mod shared {
    pub mod models {
        pub mod user;
        // Add other model modules as needed
    }
    // Add other shared modules as needed
}

pub mod utils {
    // Any utility modules
}

// Re-export important types for easier access
pub use crate::api::auth::*;
pub use crate::core::auth::*;
pub use crate::core::errors::*;
pub use crate::database::repositories::user::*;
pub use crate::shared::models::user::*;