// Consolidated repositories module
// This module contains consolidated repository implementations that replace redundant repositories

mod base_repository;
mod user_repository;
mod sqlite_user_repository;

// Re-export base repository
pub use base_repository::{
    Repository, PaginatedRepository, FilteredRepository, SortedRepository, FullRepository,
    RepositoryConfig, RepositoryFactory, RepositoryRegistry, get_repository_registry,
};

// Re-export user repository
pub use user_repository::UserRepository;

// Re-export SQLite user repository
pub use sqlite_user_repository::SqliteUserRepository;
