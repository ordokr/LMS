#!/bin/bash
echo "Building minimal Ordo Quiz standalone version..."

# Check if src-tauri directory exists
if [ -d "src-tauri" ]; then
    cd src-tauri
    
    # Create data directory if it doesn't exist
    if [ ! -d "data" ]; then
        mkdir -p data
        echo "Created data directory"
    fi
    
    # Check if database file exists, if not create it
    if [ ! -f "data/ordo_quiz.db" ]; then
        echo "Creating new database file..."
        
        # Create a minimal SQLite database with required tables
        cat > create_db.sql << EOF
.open data/ordo_quiz.db
CREATE TABLE IF NOT EXISTS courses (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  code TEXT,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS enrollments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  course_id INTEGER NOT NULL,
  role INTEGER NOT NULL,
  status INTEGER NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  role INTEGER NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
EOF
        
        # Execute the SQL script
        sqlite3 < create_db.sql
        
        # Check if database was created successfully
        if [ -f "data/ordo_quiz.db" ]; then
            echo "Database created successfully"
            rm create_db.sql
        else
            echo "Failed to create database"
        fi
    else
        echo "Database file already exists"
    fi
    
    # Update database connection string in code
    echo "Updating database connection string..."
    
    # Create a minimal .env file with database connection
    cat > .env << EOF
DATABASE_URL=sqlite:data/ordo_quiz.db
JWT_SECRET=ordo_quiz_secret_key_for_development_only
EOF
    
    # Create missing directories and files
    mkdir -p "src/shared/models"
    
    # Create a minimal AppState.rs file
    echo "Creating minimal AppState.rs..."
    cat > src/app_state.rs << EOF
// Minimal AppState implementation
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub db_pool: SqlitePool,
    pub jwt_secret: Vec<u8>,
}
EOF
    
    # Create minimal models
    echo "Creating minimal models module..."
    cat > src/shared/models/mod.rs << EOF
// Minimal models module
pub mod user;
pub mod auth;
pub mod course;
pub mod forum;
EOF
    
    # Create minimal user model
    echo "Creating minimal user model..."
    cat > src/shared/models/user.rs << EOF
// Minimal user model
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Student = 0,
    Teacher = 1,
    Admin = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
EOF
    
    # Create minimal auth model
    echo "Creating minimal auth model..."
    cat > src/shared/models/auth.rs << EOF
// Minimal auth model
use serde::{Deserialize, Serialize};
use crate::shared::models::user::{User, UserRole};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: u64,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthProfile {
    pub id: i64,
    pub username: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}
EOF
    
    # Create minimal course model
    echo "Creating minimal course model..."
    cat > src/shared/models/course.rs << EOF
// Minimal course model
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: i64,
    pub user_id: i64,
    pub course_id: i64,
    pub role: EnrollmentRole,
    pub status: EnrollmentStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum EnrollmentRole {
    Student = 0,
    Teacher = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum EnrollmentStatus {
    Pending = 0,
    Active = 1,
    Inactive = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum CourseStatus {
    Draft = 0,
    Published = 1,
    Archived = 2,
}
EOF
    
    # Create minimal forum model
    echo "Creating minimal forum model..."
    cat > src/shared/models/forum.rs << EOF
// Minimal forum model
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumCategory {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumTopic {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub category_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: i64,
    pub content: String,
    pub author_id: i64,
    pub topic_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub category_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author_id: i64,
    pub topic_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
EOF
    
    # Create minimal quiz-standalone binary
    echo "Creating minimal quiz-standalone entry point..."
    mkdir -p "src/bin"
    
    cat > src/bin/quiz_standalone.rs << EOF
// Minimal Ordo Quiz standalone entry point
use std::sync::Arc;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Ordo Quiz...");

    // Initialize database connection
    let db_url = "sqlite:data/ordo_quiz.db?mode=rwc";
    let db_pool = SqlitePool::connect(db_url).await?;

    // Create app state
    let app_state = Arc::new(lms_lib::AppState {
        db_pool,
        jwt_secret: "ordo_quiz_secret_key".as_bytes().to_vec(),
    });

    // Start the application
    println!("Ordo Quiz is running!");
    println!("Press Ctrl+C to exit");

    // Keep the application running
    tokio::signal::ctrl_c().await?;
    println!("Shutting down Ordo Quiz...");

    Ok(())
}
EOF
    
    # Update lib.rs to export AppState
    echo "Updating lib.rs..."
    
    if [ -f "src/lib.rs" ]; then
        echo "mod app_state;" >> src/lib.rs
        echo "pub use app_state::AppState;" >> src/lib.rs
    else
        cat > src/lib.rs << EOF
// Minimal lib.rs
mod app_state;
pub use app_state::AppState;

// Re-export essential components
pub mod shared;
EOF
    fi
    
    # Update Cargo.toml to include missing dependencies
    echo "Updating Cargo.toml with missing dependencies..."
    
    # Check if argon2 is in Cargo.toml, if not add it
    if ! grep -q "argon2 =" Cargo.toml; then
        echo 'argon2 = "0.5.2"' >> Cargo.toml
    fi
    
    # Check if axum-extra is in Cargo.toml, if not add it
    if ! grep -q "axum-extra =" Cargo.toml; then
        echo 'axum-extra = { version = "0.7", features = ["typed-header"] }' >> Cargo.toml
    fi
    
    # Check if time is in Cargo.toml, if not add it
    if ! grep -q "time =" Cargo.toml; then
        echo 'time = "0.3"' >> Cargo.toml
    fi
    
    # Build the minimal version
    echo "Building minimal version..."
    cargo build --bin quiz-standalone
    
    if [ $? -eq 0 ]; then
        echo "Build successful!"
        echo "You can now run the Ordo Quiz standalone with:"
        echo "  ./target/debug/quiz-standalone"
    else
        echo "Build failed. Please check the error messages above."
    fi
    
    cd ..
else
    echo "src-tauri directory not found"
fi

echo "Build process complete"
read -p "Press Enter to continue..."
