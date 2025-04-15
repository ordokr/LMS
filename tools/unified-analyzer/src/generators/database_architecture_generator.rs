use std::fs;
use std::path::Path;
use chrono::Local;

use crate::analyzers::unified_analyzer::AnalysisResult;

/// Generate database architecture documentation
pub fn generate_database_architecture(_result: &AnalysisResult, base_dir: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Generating database architecture documentation...");

    // Ensure docs directory exists
    let docs_dir = base_dir.join("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(&docs_dir)?;
    }

    // Create the database architecture path
    let db_path = docs_dir.join("database_architecture.md");

    // Generate the content
    let mut content = String::new();

    // Header
    content.push_str("# Database Architecture\n\n");
    content.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));

    // Database Solution
    content.push_str("## Database Solution\n\n");
    content.push_str("This project uses **SQLite** with the **sqlx** Rust crate as its database solution. This combination is hardcoded into the application architecture.\n\n");

    content.push_str("### Why SQLite + sqlx?\n\n");
    content.push_str("- **Offline-First Architecture**: SQLite provides local database capabilities essential for our offline-first approach\n");
    content.push_str("- **Zero-Configuration**: No separate database server installation required\n");
    content.push_str("- **Cross-Platform**: SQLite works consistently across all supported platforms\n");
    content.push_str("- **Performance**: Excellent performance for our expected workloads\n");
    content.push_str("- **Type Safety**: sqlx provides compile-time SQL query validation\n");
    content.push_str("- **Transactions**: Full ACID compliance with transaction support\n\n");

    // Implementation Details
    content.push_str("## Implementation Details\n\n");

    content.push_str("### Database Connection\n\n");
    content.push_str("```rust\n");
    content.push_str("use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};\n\n");
    content.push_str("async fn create_db_pool() -> Result<SqlitePool, sqlx::Error> {\n");
    content.push_str("    let db_url = std::env::var(\"DATABASE_URL\")\n");
    content.push_str("        .unwrap_or_else(|_| \"sqlite:lms.db\".to_string());\n\n");
    content.push_str("    SqlitePoolOptions::new()\n");
    content.push_str("        .max_connections(5)\n");
    content.push_str("        .connect(&db_url)\n");
    content.push_str("        .await\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    // Database Schema
    content.push_str("## Database Schema\n\n");

    content.push_str("### Core Tables\n\n");

    // Example tables - in a real implementation, these would be extracted from the codebase
    let tables = [
        ("users", "User accounts", ["id", "username", "email", "password_hash", "created_at", "updated_at"]),
        ("courses", "Course information", ["id", "title", "description", "instructor_id", "created_at", "updated_at"]),
        ("enrollments", "User enrollments in courses", ["id", "user_id", "course_id", "role", "created_at", "updated_at"]),
        ("assignments", "Course assignments", ["id", "course_id", "title", "description", "due_date", "points_possible"]),
        ("submissions", "Assignment submissions", ["id", "assignment_id", "user_id", "content", "submitted_at", "grade"]),
        ("discussions", "Discussion topics", ["id", "course_id", "title", "content", "user_id", "created_at"]),
        ("posts", "Discussion posts", ["id", "discussion_id", "user_id", "content", "created_at", "updated_at"]),
        ("sync_transactions", "Synchronization transactions", ["id", "entity_type", "entity_id", "operation", "status", "created_at"])
    ];

    for (table, description, columns) in tables {
        content.push_str(&format!("#### {}\n\n", table));
        content.push_str(&format!("{}\n\n", description));
        content.push_str("| Column | Type | Description |\n");
        content.push_str("|--------|------|-------------|\n");

        for column in columns {
            let column_type = match column {
                "id" => "INTEGER PRIMARY KEY",
                "user_id" | "course_id" | "assignment_id" | "discussion_id" => "INTEGER REFERENCES",
                "created_at" | "updated_at" | "submitted_at" | "due_date" => "TIMESTAMP",
                "points_possible" | "grade" => "REAL",
                _ => "TEXT"
            };

            let description = match column {
                "id" => "Unique identifier",
                "user_id" => "Reference to users table",
                "course_id" => "Reference to courses table",
                "assignment_id" => "Reference to assignments table",
                "discussion_id" => "Reference to discussions table",
                "created_at" => "Creation timestamp",
                "updated_at" => "Last update timestamp",
                "submitted_at" => "Submission timestamp",
                "due_date" => "Assignment due date",
                "points_possible" => "Maximum possible points",
                "grade" => "Assigned grade",
                _ => "Content field"
            };

            content.push_str(&format!("| {} | {} | {} |\n", column, column_type, description));
        }

        content.push_str("\n");
    }

    // Database Migration
    content.push_str("## Database Migration\n\n");
    content.push_str("The project uses sqlx's built-in migrations system for schema management. Migrations are SQL files stored in `src-tauri/migrations/` and run automatically when the application starts.\n\n");

    content.push_str("Example migration file structure:\n\n");
    content.push_str("```\n");
    content.push_str("src-tauri/migrations/\n");
    content.push_str("├── 20230101000000_initial_schema.sql\n");
    content.push_str("├── 20230201000000_add_user_preferences.sql\n");
    content.push_str("└── 20230301000000_add_course_features.sql\n");
    content.push_str("```\n\n");

    // Query Examples
    content.push_str("## Query Examples\n\n");

    content.push_str("### Select Query\n\n");
    content.push_str("```rust\n");
    content.push_str("async fn get_course(pool: &SqlitePool, course_id: i64) -> Result<Course, sqlx::Error> {\n");
    content.push_str("    sqlx::query_as!(Course,\n");
    content.push_str("        \"SELECT id, title, description, instructor_id, created_at, updated_at \n");
    content.push_str("         FROM courses \n");
    content.push_str("         WHERE id = ?\",\n");
    content.push_str("        course_id\n");
    content.push_str("    )\n");
    content.push_str("    .fetch_one(pool)\n");
    content.push_str("    .await\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("### Insert Query\n\n");
    content.push_str("```rust\n");
    content.push_str("async fn create_course(pool: &SqlitePool, course: &NewCourse) -> Result<i64, sqlx::Error> {\n");
    content.push_str("    let result = sqlx::query!(\n");
    content.push_str("        \"INSERT INTO courses (title, description, instructor_id) \n");
    content.push_str("         VALUES (?, ?, ?)\",\n");
    content.push_str("        course.title, course.description, course.instructor_id\n");
    content.push_str("    )\n");
    content.push_str("    .execute(pool)\n");
    content.push_str("    .await?;\n\n");
    content.push_str("    Ok(result.last_insert_rowid())\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("### Transaction Example\n\n");
    content.push_str("```rust\n");
    content.push_str("async fn enroll_user_in_course(pool: &SqlitePool, user_id: i64, course_id: i64) -> Result<(), sqlx::Error> {\n");
    content.push_str("    let mut tx = pool.begin().await?;\n\n");
    content.push_str("    // Check if user exists\n");
    content.push_str("    let user = sqlx::query!(\"SELECT id FROM users WHERE id = ?\", user_id)\n");
    content.push_str("        .fetch_optional(&mut tx)\n");
    content.push_str("        .await?;\n\n");
    content.push_str("    if user.is_none() {\n");
    content.push_str("        return Err(sqlx::Error::RowNotFound);\n");
    content.push_str("    }\n\n");
    content.push_str("    // Check if course exists\n");
    content.push_str("    let course = sqlx::query!(\"SELECT id FROM courses WHERE id = ?\", course_id)\n");
    content.push_str("        .fetch_optional(&mut tx)\n");
    content.push_str("        .await?;\n\n");
    content.push_str("    if course.is_none() {\n");
    content.push_str("        return Err(sqlx::Error::RowNotFound);\n");
    content.push_str("    }\n\n");
    content.push_str("    // Create enrollment\n");
    content.push_str("    sqlx::query!(\n");
    content.push_str("        \"INSERT INTO enrollments (user_id, course_id, role) \n");
    content.push_str("         VALUES (?, ?, 'student')\",\n");
    content.push_str("        user_id, course_id\n");
    content.push_str("    )\n");
    content.push_str("    .execute(&mut tx)\n");
    content.push_str("    .await?;\n\n");
    content.push_str("    tx.commit().await?;\n\n");
    content.push_str("    Ok(())\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    // Offline Sync
    content.push_str("## Offline Sync Strategy\n\n");
    content.push_str("The database architecture supports offline-first operation with eventual consistency:\n\n");

    content.push_str("1. **Local-First**: All operations are performed on the local SQLite database first\n");
    content.push_str("2. **Change Tracking**: Changes are tracked in the `sync_transactions` table\n");
    content.push_str("3. **Sync Process**: When online, changes are synchronized with the server\n");
    content.push_str("4. **Conflict Resolution**: Conflicts are resolved using predefined strategies\n\n");

    content.push_str("### Sync Transaction Table\n\n");
    content.push_str("The `sync_transactions` table tracks all changes that need to be synchronized:\n\n");

    content.push_str("```sql\n");
    content.push_str("CREATE TABLE sync_transactions (\n");
    content.push_str("    id INTEGER PRIMARY KEY,\n");
    content.push_str("    entity_type TEXT NOT NULL,\n");
    content.push_str("    entity_id INTEGER NOT NULL,\n");
    content.push_str("    operation TEXT NOT NULL,\n");
    content.push_str("    status TEXT NOT NULL,\n");
    content.push_str("    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,\n");
    content.push_str("    synced_at TIMESTAMP,\n");
    content.push_str("    error_message TEXT\n");
    content.push_str(");\n");
    content.push_str("```\n\n");

    // Performance Considerations
    content.push_str("## Performance Considerations\n\n");

    content.push_str("### Indexes\n\n");
    content.push_str("The following indexes are created to improve query performance:\n\n");

    content.push_str("```sql\n");
    content.push_str("-- User lookup by email\n");
    content.push_str("CREATE INDEX idx_users_email ON users(email);\n\n");
    content.push_str("-- Course lookup by instructor\n");
    content.push_str("CREATE INDEX idx_courses_instructor ON courses(instructor_id);\n\n");
    content.push_str("-- Enrollment lookup by user and course\n");
    content.push_str("CREATE INDEX idx_enrollments_user_course ON enrollments(user_id, course_id);\n\n");
    content.push_str("-- Assignment lookup by course\n");
    content.push_str("CREATE INDEX idx_assignments_course ON assignments(course_id);\n\n");
    content.push_str("-- Submission lookup by assignment and user\n");
    content.push_str("CREATE INDEX idx_submissions_assignment_user ON submissions(assignment_id, user_id);\n\n");
    content.push_str("-- Discussion lookup by course\n");
    content.push_str("CREATE INDEX idx_discussions_course ON discussions(course_id);\n\n");
    content.push_str("-- Post lookup by discussion\n");
    content.push_str("CREATE INDEX idx_posts_discussion ON posts(discussion_id);\n\n");
    content.push_str("-- Sync transaction lookup by status\n");
    content.push_str("CREATE INDEX idx_sync_transactions_status ON sync_transactions(status);\n");
    content.push_str("```\n\n");

    content.push_str("### Query Optimization\n\n");
    content.push_str("- Use prepared statements for all queries\n");
    content.push_str("- Limit result sets for pagination\n");
    content.push_str("- Use transactions for related operations\n");
    content.push_str("- Avoid N+1 query problems by using JOINs\n");

    // Write to file
    fs::write(&db_path, content)?;

    println!("Database architecture documentation generated at: {:?}", db_path);

    Ok(())
}
