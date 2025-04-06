# Database Architecture

_Generated on: 2025-04-06_

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
// src-tauri/src/db/mod.rs
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool, Error};
use std::path::PathBuf;

pub async fn establish_connection() -> Result<SqlitePool, Error> {
    let db_path = app_local_data_dir()
        .map(|dir| dir.join("educonnect.db"))
        .ok_or_else(|| Error::Database("Failed to get app data directory".into()))?;
    
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
```

### Dependency Injection

The SQLite connection pool is injected into services, ensuring consistent database access throughout the application.

```rust
// Example service initialization
pub fn initialize_services(pool: SqlitePool) -> AppServices {
    let user_repository = UserRepository::new(pool.clone());
    let course_repository = CourseRepository::new(pool.clone());
    // ...other repositories
    
    let auth_service = AuthService::new(user_repository.clone());
    let course_service = CourseService::new(course_repository);
    // ...other services
    
    AppServices {
        auth_service,
        course_service,
        // ...other services
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

## Schema Documentation

### Users Schema

#### EnrollmentRole

Schema details not available

#### ForumUserPreferences

Schema details not available

#### User

Schema details not available

#### UserRole

Schema details not available

#### UserProfile

Schema details not available

#### User

Schema details not available

#### User

Schema details not available

#### UserRole

Schema details not available

### Courses Schema

#### Course

Schema details not available

#### Module

Schema details not available

#### Assignment

Schema details not available

#### CourseStatus

Schema details not available

#### Course

Schema details not available

#### Course

Schema details not available

#### Module

Schema details not available

#### Assignment

Schema details not available

#### CourseStatus

Schema details not available

#### DiscourseTopic

Schema details not available

#### DiscoursePost

Schema details not available

#### CourseCategory

Schema details not available

#### CourseCategoryCreate

Schema details not available

#### CourseCategoryUpdate

Schema details not available

#### Course

Schema details not available

### Discussions Schema

#### ForumCategory

Schema details not available

#### ForumTopic

Schema details not available

#### ForumPost

Schema details not available

#### ForumTrustLevel

Schema details not available

#### ForumCategory

Schema details not available

#### ForumTopic

Schema details not available

#### ForumPost

Schema details not available

#### DiscussionMapping

Schema details not available

#### CanvasDiscussionEntry

Schema details not available

#### Post

Schema details not available

### Other Schema

#### Submission

Schema details not available

#### Enrollment

Schema details not available

#### LoginRequest

Schema details not available

#### RegisterRequest

Schema details not available

#### AuthResponse

Schema details not available

#### LoginRequest

Schema details not available

#### RegisterRequest

Schema details not available

#### AuthResponse

Schema details not available

#### Category

Schema details not available

#### Submission

Schema details not available

#### SyncResult

Schema details not available

#### Tag

Schema details not available

#### Topic

Schema details not available

