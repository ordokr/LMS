pub mod schema;
pub mod repositories;

use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite, SqlitePool};
use std::{path::Path, str::FromStr};

pub async fn init_db(db_path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    // Ensure the database directory exists
    if let Some(parent) = Path::new(db_path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create database directory");
        }
    }
    
    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    Ok(pool)
}

// Declare the submodules
mod course;      // Missing: Need to create this file
mod forum;       // Missing: Need to create this file

// Public exports
pub use repositories::{
    // Only export what's actually being used elsewhere
    // Consider uncommenting these as you implement them
    // UserRepository,
    // CategoryRepository,
    // TopicRepository,
    // PostRepository,
};

// Re-export functions from the course module
pub use self::course::{create_course, get_courses};

// Re-export functions from the forum module
pub use self::forum::{create_forum_thread, get_forum_threads, create_forum_post, get_forum_posts};

// Other possible exports - add as needed
// pub use self::assignments::{create_assignment, get_assignments};
// pub use self::submissions::{create_submission, get_submissions};