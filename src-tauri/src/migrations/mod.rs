// Migrations module
// This module contains utilities for migrating data between different schema versions

mod user_migration;
mod course_migration;
mod group_migration;

// Re-export migration utilities
pub use user_migration::UserMigration;
pub use course_migration::CourseMigration;
pub use group_migration::GroupMigration;
pub use user_migration::MigrationStats;
