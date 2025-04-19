// Integration tests module
// This module contains integration tests for the unified models and repositories

mod test_utils;
mod user_tests;
mod course_tests;
mod group_tests;
mod assignment_tests;
mod topic_tests;
mod submission_tests;
mod relationship_tests;

// Re-export test utilities
pub use test_utils::*;
