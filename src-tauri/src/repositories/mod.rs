// Export unified repositories
pub mod unified_repositories;
pub mod consolidated;

// Re-export repository interfaces
pub use unified_repositories::{Repository, PaginatedRepository, FilteredRepository, SortedRepository, FullRepository};
pub use unified_repositories::{UserRepository, CourseRepository, GroupRepository, AssignmentRepository, TopicRepository, SubmissionRepository};

// Re-export consolidated repositories
pub use consolidated::{Repository as ConsolidatedRepository, PaginatedRepository as ConsolidatedPaginatedRepository, FilteredRepository as ConsolidatedFilteredRepository, SortedRepository as ConsolidatedSortedRepository, FullRepository as ConsolidatedFullRepository};
pub use consolidated::{UserRepository as ConsolidatedUserRepository, SqliteUserRepository};
pub use consolidated::{RepositoryConfig, RepositoryFactory, RepositoryRegistry, get_repository_registry};

// Re-export repository implementations
pub use unified_repositories::{SqliteUserRepository, SqliteCourseRepository, SqliteGroupRepository, SqliteAssignmentRepository, SqliteTopicRepository, SqliteSubmissionRepository};