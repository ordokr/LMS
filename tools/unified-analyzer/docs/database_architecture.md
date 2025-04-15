# Database Architecture

_Generated on: 2025-04-14_

## Database Solution

This project uses **SQLite** with the **sqlx** Rust crate as its database solution. This combination is hardcoded into the application architecture.

### Why SQLite + sqlx?

- **Offline-First Architecture**: SQLite provides local database capabilities essential for our offline-first approach
- **Zero-Configuration**: No separate database server installation required
- **Cross-Platform**: SQLite works consistently across all supported platforms
- **Performance**: Excellent performance for our expected workloads
- **Type Safety**: sqlx provides compile-time SQL query validation
- **Transactions**: Full ACID compliance with transaction support

## Implementation Details

### Database Connection

```rust
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

async fn create_db_pool() -> Result<SqlitePool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:lms.db".to_string());

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
```

## Database Schema

### Core Tables

#### users

User accounts

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| username | TEXT | Content field |
| email | TEXT | Content field |
| password_hash | TEXT | Content field |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

#### courses

Course information

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| title | TEXT | Content field |
| description | TEXT | Content field |
| instructor_id | TEXT | Content field |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

#### enrollments

User enrollments in courses

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| user_id | INTEGER REFERENCES | Reference to users table |
| course_id | INTEGER REFERENCES | Reference to courses table |
| role | TEXT | Content field |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

#### assignments

Course assignments

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| course_id | INTEGER REFERENCES | Reference to courses table |
| title | TEXT | Content field |
| description | TEXT | Content field |
| due_date | TIMESTAMP | Assignment due date |
| points_possible | REAL | Maximum possible points |

#### submissions

Assignment submissions

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| assignment_id | INTEGER REFERENCES | Reference to assignments table |
| user_id | INTEGER REFERENCES | Reference to users table |
| content | TEXT | Content field |
| submitted_at | TIMESTAMP | Submission timestamp |
| grade | REAL | Assigned grade |

#### discussions

Discussion topics

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| course_id | INTEGER REFERENCES | Reference to courses table |
| title | TEXT | Content field |
| content | TEXT | Content field |
| user_id | INTEGER REFERENCES | Reference to users table |
| created_at | TIMESTAMP | Creation timestamp |

#### posts

Discussion posts

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| discussion_id | INTEGER REFERENCES | Reference to discussions table |
| user_id | INTEGER REFERENCES | Reference to users table |
| content | TEXT | Content field |
| created_at | TIMESTAMP | Creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

#### sync_transactions

Synchronization transactions

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| entity_type | TEXT | Content field |
| entity_id | TEXT | Content field |
| operation | TEXT | Content field |
| status | TEXT | Content field |
| created_at | TIMESTAMP | Creation timestamp |

## Database Migration

The project uses sqlx's built-in migrations system for schema management. Migrations are SQL files stored in `src-tauri/migrations/` and run automatically when the application starts.

Example migration file structure:

```
src-tauri/migrations/
├── 20230101000000_initial_schema.sql
├── 20230201000000_add_user_preferences.sql
└── 20230301000000_add_course_features.sql
```

## Query Examples

### Select Query

```rust
async fn get_course(pool: &SqlitePool, course_id: i64) -> Result<Course, sqlx::Error> {
    sqlx::query_as!(Course,
        "SELECT id, title, description, instructor_id, created_at, updated_at 
         FROM courses 
         WHERE id = ?",
        course_id
    )
    .fetch_one(pool)
    .await
}
```

### Insert Query

```rust
async fn create_course(pool: &SqlitePool, course: &NewCourse) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO courses (title, description, instructor_id) 
         VALUES (?, ?, ?)",
        course.title, course.description, course.instructor_id
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
```

### Transaction Example

```rust
async fn enroll_user_in_course(pool: &SqlitePool, user_id: i64, course_id: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Check if user exists
    let user = sqlx::query!("SELECT id FROM users WHERE id = ?", user_id)
        .fetch_optional(&mut tx)
        .await?;

    if user.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    // Check if course exists
    let course = sqlx::query!("SELECT id FROM courses WHERE id = ?", course_id)
        .fetch_optional(&mut tx)
        .await?;

    if course.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    // Create enrollment
    sqlx::query!(
        "INSERT INTO enrollments (user_id, course_id, role) 
         VALUES (?, ?, 'student')",
        user_id, course_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
```

## Offline Sync Strategy

The database architecture supports offline-first operation with eventual consistency:

1. **Local-First**: All operations are performed on the local SQLite database first
2. **Change Tracking**: Changes are tracked in the `sync_transactions` table
3. **Sync Process**: When online, changes are synchronized with the server
4. **Conflict Resolution**: Conflicts are resolved using predefined strategies

### Sync Transaction Table

The `sync_transactions` table tracks all changes that need to be synchronized:

```sql
CREATE TABLE sync_transactions (
    id INTEGER PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id INTEGER NOT NULL,
    operation TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    synced_at TIMESTAMP,
    error_message TEXT
);
```

## Performance Considerations

### Indexes

The following indexes are created to improve query performance:

```sql
-- User lookup by email
CREATE INDEX idx_users_email ON users(email);

-- Course lookup by instructor
CREATE INDEX idx_courses_instructor ON courses(instructor_id);

-- Enrollment lookup by user and course
CREATE INDEX idx_enrollments_user_course ON enrollments(user_id, course_id);

-- Assignment lookup by course
CREATE INDEX idx_assignments_course ON assignments(course_id);

-- Submission lookup by assignment and user
CREATE INDEX idx_submissions_assignment_user ON submissions(assignment_id, user_id);

-- Discussion lookup by course
CREATE INDEX idx_discussions_course ON discussions(course_id);

-- Post lookup by discussion
CREATE INDEX idx_posts_discussion ON posts(discussion_id);

-- Sync transaction lookup by status
CREATE INDEX idx_sync_transactions_status ON sync_transactions(status);
```

### Query Optimization

- Use prepared statements for all queries
- Limit result sets for pagination
- Use transactions for related operations
- Avoid N+1 query problems by using JOINs
