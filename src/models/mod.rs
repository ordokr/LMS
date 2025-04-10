// src/models/mod.rs
//! Unified models for the LMS integration

mod user;
mod notification;
mod discussion;
mod course;
mod assignment;

pub use user::User;
pub use notification::Notification;
pub use discussion::Discussion;
pub use course::Course;
pub use assignment::Assignment;

// Re-export for convenience
pub mod unified {
    pub use super::*;
}
