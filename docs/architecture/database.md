# Database Architecture

_Last updated: 2025-04-17_

## Overview

This document provides a comprehensive overview of the database architecture for the Ordo project. The database architecture is designed to support offline-first functionality, efficient synchronization, and seamless integration with Canvas and Discourse.

## Database Solution

Ordo uses a hybrid database approach:

1. **SQLite** with the **sqlx** Rust crate for structured data storage
2. **Redb** for ephemeral state and sync metadata

### Why SQLite + sqlx?

- **Offline-First Architecture**: SQLite provides local database capabilities essential for our offline-first approach
- **Zero-Configuration**: No separate database server installation required
- **Cross-Platform**: SQLite works consistently across all supported platforms
- **Performance**: Excellent performance for our expected workloads
- **Type Safety**: sqlx provides compile-time SQL query validation
- **Transactions**: Full ACID compliance with transaction support

### Why Redb?

- **Zero-Copy**: Redb is an embedded, zero-copy database that provides excellent performance for read-heavy workloads
- **MVCC**: Multi-Version Concurrency Control allows background sync operations without blocking the UI thread
- **Rust Native**: Built specifically for Rust, with excellent integration capabilities
- **Lightweight**: Minimal overhead for ephemeral state storage

## Database Schema

For a detailed view of the database schema, please refer to:

- [Database Schema Documentation](../models/database_schema.md)
- [Database Schema Visualization](../visualizations/db_schema/db_schema.md)

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

### Redb Integration

```rust
use redb::{Database, TableDefinition, Error, ReadableTable, Table, Value, WriteTransaction};

const SYNC_METADATA: TableDefinition<&str, &[u8]> = TableDefinition::new("sync_metadata");

fn update_sync_metadata(db: &Database, key: &str, value: &[u8]) -> Result<(), Error> {
    let write_txn = db.begin_write()?;
    {
        let mut table: Table<&str, &[u8]> = write_txn.open_table(SYNC_METADATA)?;
        table.insert(key, value)?;
    }
    write_txn.commit()?;
    Ok(())
}

fn get_sync_metadata(db: &Database, key: &str) -> Result<Option<Vec<u8>>, Error> {
    let read_txn = db.begin_read()?;
    let table: ReadableTable<&str, &[u8]> = read_txn.open_table(SYNC_METADATA)?;
    if let Some(value) = table.get(key)? {
        Ok(Some(value.value().to_vec()))
    } else {
        Ok(None)
    }
}
```

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

## Integration with Canvas and Discourse

The database schema includes mapping tables to facilitate integration with Canvas and Discourse:

- `discussion_mappings`: Maps Canvas discussions to Discourse topics
- `course_category_mappings`: Maps courses to Discourse categories
- `sync_status`: Tracks synchronization status for entities

This allows for seamless integration between the systems while maintaining offline-first capabilities.

## References

- [Database Schema Documentation](../models/database_schema.md)
- [Database Schema Visualization](../visualizations/db_schema/db_schema.md)
- [Synchronization Architecture](synchronization.md)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Redb Documentation](https://github.com/kixiron/redb)
