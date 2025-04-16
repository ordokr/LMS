use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use sqlx::query_as;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    id: Uuid,
    username: String,
    email: String,
    name: String,
    avatar_url: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    canvas_id: Option<i32>,
    discourse_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Course {
    id: Uuid,
    title: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    canvas_id: Option<i32>,
    discourse_id: Option<i32>,
    instructor_id: Uuid,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
    category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Discussion {
    id: Uuid,
    title: String,
    content: String,
    author_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    course_id: Uuid,
    topic_id: Uuid,
    canvas_id: Option<i32>,
    discourse_id: Option<i32>,
}

use sqlx::sqlite::{SqlitePoolOptions};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting migration...");

    // Source database connection string (replace with your actual path)
    let source_db_url = "sqlite:lms.db";

    // Destination database connection string (replace with your actual path)
    let dest_db_url = "sqlite:unified.db";

    // Create a connection pool to the source database
    let source_pool = SqlitePoolOptions::new()
      .max_connections(5)
      .connect(source_db_url)
      .await?;

    println!("Connected to source database.");

    // Create a connection pool to the destination database
    let dest_pool = SqlitePoolOptions::new()
      .max_connections(5)
      .connect(dest_db_url)
      .await?;

    println!("Connected to destination database.");

    // Create the unified_users table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS unified_users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            email TEXT,
            name TEXT,
            avatar_url TEXT,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL,
            canvas_id INTEGER,
            discourse_id INTEGER
        )"
    )
  .execute(&dest_pool)
  .await?;

    println!("Created unified_users table (if not exists).");

    // Fetch users from the source database
    let users = query_as::<_, User>(
        "SELECT id, username, email, name, avatar_url, created_at, updated_at, canvas_id, discourse_id FROM users"
    )
  .fetch_all(&source_pool)
  .await?;

    println!("Fetched {} users", users.len());

    // Insert users into the destination database
    for user in users {
        let mut tx = dest_pool.begin().await?;
        let query = sqlx::query!(
            "INSERT INTO unified_users (id, username, email, name, avatar_url, created_at, updated_at, canvas_id, discourse_id)
             VALUES (?,?,?,?,?,?,?,?,?)",
            user.id.to_string(),
            user.username,
            user.email,
            user.name,
            user.avatar_url,
            user.created_at,
            user.updated_at,
            user.canvas_id,
            user.discourse_id
        );
        query.execute(&mut tx).await?;
        tx.commit().await?;
    }

    println!("Inserted users into destination database.");

    // Create the unified_courses table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS unified_courses (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL,
            canvas_id INTEGER,
            discourse_id INTEGER,
            instructor_id TEXT NOT NULL,
            start_date DATETIME,
            end_date DATETIME,
            category TEXT
        )"
    )
  .execute(&dest_pool)
  .await?;

    println!("Created unified_courses table (if not exists).");

    // Fetch courses from the source database
    let courses = query_as::<_, Course>(
        "SELECT id, title, description, created_at, updated_at, canvas_id, discourse_id, instructor_id, start_date, end_date, category FROM courses"
    )
  .fetch_all(&source_pool)
  .await?;

    println!("Fetched {} courses", courses.len());

    // Insert courses into the destination database
    for course in courses {
        let mut tx = dest_pool.begin().await?;
        let query = sqlx::query!(
            "INSERT INTO unified_courses (id, title, description, created_at, updated_at, canvas_id, discourse_id, instructor_id, start_date, end_date, category)
             VALUES (?,?,?,?,?,?,?,?,?,?,?)",
            course.id.to_string(),
            course.title,
            course.description,
            course.created_at,
            course.updated_at,
            course.canvas_id,
            course.discourse_id,
            course.instructor_id.to_string(),
            course.start_date,
            course.end_date,
            course.category
        );
        query.execute(&mut tx).await?;
        tx.commit().await?;
    }

    println!("Inserted courses into destination database.");

    // Create the unified_discussions table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS unified_discussions (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT,
            author_id TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL,
            course_id TEXT NOT NULL,
            topic_id TEXT NOT NULL,
            canvas_id INTEGER,
            discourse_id INTEGER
        )"
    )
  .execute(&dest_pool)
  .await?;

    println!("Created unified_discussions table (if not exists).");

    // Fetch discussions from the source database
    let discussions = query_as::<_, Discussion>(
        "SELECT id, title, content, author_id, created_at, updated_at, course_id, topic_id, canvas_id, discourse_id FROM discussions"
    )
  .fetch_all(&source_pool)
  .await?;

    println!("Fetched {} discussions", discussions.len());

    // Insert discussions into the destination database
    for discussion in discussions {
        let mut tx = dest_pool.begin().await?;
        let query = sqlx::query!(
            "INSERT INTO unified_discussions (id, title, content, author_id, created_at, updated_at, course_id, topic_id, canvas_id, discourse_id)
             VALUES (?,?,?,?,?,?,?,?,?,?)",
            discussion.id.to_string(),
            discussion.title,
            discussion.content,
            discussion.author_id.to_string(),
            discussion.created_at,
            discussion.updated_at,
            discussion.course_id.to_string(),
            discussion.topic_id.to_string(),
            discussion.canvas_id,
            discussion.discourse_id
        );
        query.execute(&mut tx).await?;
        tx.commit().await?;
    }

    println!("Inserted discussions into destination database.");

    println!("Migration complete.");

    Ok(())
}
