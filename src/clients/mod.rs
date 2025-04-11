// This module contains API clients for external services
pub mod canvas;
pub mod discourse;

// Re-export the client structs for easier imports
pub use canvas::{CanvasClient, CanvasError, Course, Announcement};
pub use discourse::{DiscourseClient, DiscourseError, Topic, User, SsoData};
