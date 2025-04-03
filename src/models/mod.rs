pub mod auth;
pub mod forum;
pub mod lms;
pub mod sync;
pub mod user;
pub mod admin; // Add this line to export admin models

pub use auth::*;
pub use forum::*;
pub use lms::*;
pub use sync::*;