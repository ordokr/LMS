// Export all page modules
pub mod integration;
pub mod dashboard;
pub mod courses;
pub mod not_found;
pub mod unauthorized;

// Re-export integration pages
pub use integration::*;
