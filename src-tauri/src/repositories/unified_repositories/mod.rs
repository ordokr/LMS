// Unified repositories module
// This module contains consolidated repository implementations that replace redundant repositories

mod repository;
mod user_repository;
mod sqlite_user_repository;
mod course_repository;
mod sqlite_course_repository;
mod group_repository;
mod sqlite_group_repository;
mod assignment_repository;
mod sqlite_assignment_repository;
mod topic_repository;
mod sqlite_topic_repository;
mod submission_repository;
mod sqlite_submission_repository;

// Re-export repository interfaces
pub use repository::{Repository, PaginatedRepository, FilteredRepository, SortedRepository, FullRepository};
pub use user_repository::UserRepository;
pub use course_repository::CourseRepository;
pub use group_repository::GroupRepository;
pub use assignment_repository::AssignmentRepository;
pub use topic_repository::TopicRepository;
pub use submission_repository::SubmissionRepository;

// Re-export repository implementations
pub use sqlite_user_repository::SqliteUserRepository;
pub use sqlite_course_repository::SqliteCourseRepository;
pub use sqlite_group_repository::SqliteGroupRepository;
pub use sqlite_assignment_repository::SqliteAssignmentRepository;
pub use sqlite_topic_repository::SqliteTopicRepository;
pub use sqlite_submission_repository::SqliteSubmissionRepository;
