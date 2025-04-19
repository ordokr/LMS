use sqlx::{Pool, Sqlite, SqlitePool};
use std::sync::Arc;
use std::sync::Once;
use std::path::Path;
use std::fs;

use crate::repositories::unified_repositories::{
    SqliteUserRepository, UserRepository,
    SqliteCourseRepository, CourseRepository,
    SqliteGroupRepository, GroupRepository,
    SqliteAssignmentRepository, AssignmentRepository,
    SqliteTopicRepository, TopicRepository,
    SqliteSubmissionRepository, SubmissionRepository
};

static INIT: Once = Once::new();

/// Initialize the test database
pub async fn init_test_db() -> Pool<Sqlite> {
    // Create a temporary database file
    let db_path = "test_db.sqlite";
    
    // Only initialize once
    INIT.call_once(|| {
        // Remove the database file if it exists
        if Path::new(db_path).exists() {
            fs::remove_file(db_path).expect("Failed to remove test database file");
        }
    });
    
    // Create a new database connection
    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path))
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    pool
}

/// Create repository instances for testing
pub struct TestRepositories {
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
    pub course_repo: Arc<dyn CourseRepository + Send + Sync>,
    pub group_repo: Arc<dyn GroupRepository + Send + Sync>,
    pub assignment_repo: Arc<dyn AssignmentRepository + Send + Sync>,
    pub topic_repo: Arc<dyn TopicRepository + Send + Sync>,
    pub submission_repo: Arc<dyn SubmissionRepository + Send + Sync>,
}

impl TestRepositories {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            user_repo: Arc::new(SqliteUserRepository::new(pool.clone())),
            course_repo: Arc::new(SqliteCourseRepository::new(pool.clone())),
            group_repo: Arc::new(SqliteGroupRepository::new(pool.clone())),
            assignment_repo: Arc::new(SqliteAssignmentRepository::new(pool.clone())),
            topic_repo: Arc::new(SqliteTopicRepository::new(pool.clone())),
            submission_repo: Arc::new(SqliteSubmissionRepository::new(pool.clone())),
        }
    }
}

/// Clean up test resources
pub async fn cleanup_test_db() {
    // Remove the database file
    let db_path = "test_db.sqlite";
    if Path::new(db_path).exists() {
        fs::remove_file(db_path).expect("Failed to remove test database file");
    }
}
