# Consolidated Repositories

This directory contains the consolidated repository implementations for the LMS application. These repositories replace the redundant repository implementations that were scattered throughout the codebase.

## Overview

The consolidated repositories provide a consistent interface for data access across the application. They are designed to be:

- **Consistent**: All repositories follow the same design patterns and conventions
- **Comprehensive**: All repositories include all methods needed for all use cases
- **Configurable**: All repositories support flexible configuration options
- **Testable**: All repositories are designed for easy testing

## Repositories

The following repositories are included:

- **Repository**: Base interface for CRUD operations
- **PaginatedRepository**: Interface for pagination
- **FilteredRepository**: Interface for filtering
- **SortedRepository**: Interface for sorting
- **FullRepository**: Interface for full-featured repositories
- **UserRepository**: Interface for user-specific operations
- **SqliteUserRepository**: SQLite implementation of the user repository

## Usage

### Creating a Repository

```rust
// Create a SQLite user repository
let user_repository = SqliteUserRepository::new(db_pool);

// Register the repository
let mut registry = RepositoryRegistry::new();
registry.register::<User, String, SqliteUserRepository>("user", user_repository)?;
```

### Using a Repository

```rust
// Get a repository
let user_repository = registry.get::<User, String, SqliteUserRepository>("user")?;

// Find a user by ID
let user = user_repository.find_by_id(&"user123".to_string()).await?;

// Find a user by email
let user = user_repository.find_by_email("user@example.com").await?;

// Create a new user
let new_user = User::new(/* ... */);
let created_user = user_repository.create(&new_user).await?;

// Update a user
let updated_user = user_repository.update(&user).await?;

// Delete a user
user_repository.delete(&"user123".to_string()).await?;
```

### Using Pagination, Filtering, and Sorting

```rust
// Get a full-featured repository
let user_repository = registry.get::<User, String, FullRepository<User, String>>("user")?;

// Find users with pagination
let users = user_repository.find_with_pagination(1, 10).await?;

// Find users with filtering
let users = user_repository.find_by_filter("role:admin").await?;

// Find users with sorting
let users = user_repository.find_sorted("name", true).await?;

// Find users with filtering, sorting, and pagination
let users = user_repository.find_with_filter_sort_pagination(
    "role:admin",
    "name",
    true,
    1,
    10
).await?;
```

## Creating a New Repository

To create a new repository, follow these steps:

1. Create a new interface for your repository (e.g., `my_repository.rs`)
2. Create a new implementation of your repository (e.g., `sqlite_my_repository.rs`)
3. Add your repository to the `mod.rs` file
4. Register your repository with the repository registry

Example:

```rust
// my_repository.rs
use async_trait::async_trait;
use crate::errors::error::Result;
use crate::models::MyEntity;
use super::base_repository::Repository;

#[async_trait]
pub trait MyRepository: Repository<MyEntity, String> {
    async fn find_by_name(&self, name: &str) -> Result<Option<MyEntity>>;
}

// sqlite_my_repository.rs
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use crate::errors::error::{Error, Result};
use crate::models::MyEntity;
use super::base_repository::Repository;
use super::my_repository::MyRepository;

#[derive(Debug)]
pub struct SqliteMyRepository {
    pool: Pool<Sqlite>,
}

impl SqliteMyRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<MyEntity, String> for SqliteMyRepository {
    // Implement Repository methods...
}

#[async_trait]
impl MyRepository for SqliteMyRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<MyEntity>> {
        // Implement find_by_name...
    }
}
```

## Testing Repositories

Repositories are designed to be easily testable. Here's an example of how to test a repository:

```rust
#[tokio::test]
async fn test_user_repository() {
    // Create a test database
    let db_pool = create_test_db().await;
    
    // Create a repository
    let user_repository = SqliteUserRepository::new(db_pool);
    
    // Create a test user
    let user = User::new(/* ... */);
    let created_user = user_repository.create(&user).await.unwrap();
    
    // Find the user by ID
    let found_user = user_repository.find_by_id(&created_user.id).await.unwrap().unwrap();
    assert_eq!(found_user.id, created_user.id);
    
    // Find the user by email
    let found_user = user_repository.find_by_email(&created_user.email).await.unwrap().unwrap();
    assert_eq!(found_user.id, created_user.id);
    
    // Update the user
    let mut updated_user = created_user.clone();
    updated_user.name = "Updated Name".to_string();
    let updated_user = user_repository.update(&updated_user).await.unwrap();
    assert_eq!(updated_user.name, "Updated Name");
    
    // Delete the user
    user_repository.delete(&created_user.id).await.unwrap();
    let found_user = user_repository.find_by_id(&created_user.id).await.unwrap();
    assert!(found_user.is_none());
}
```

## Best Practices

1. **Use the Repository trait**: All repositories should implement the `Repository` trait
2. **Use the Result type**: All repository methods should return a `Result` type
3. **Use async/await**: All repository methods should be async
4. **Use proper error handling**: All repository methods should handle errors properly
5. **Use proper validation**: All repository methods should validate input
6. **Use proper documentation**: All repository methods should be documented
7. **Use proper testing**: All repositories should have comprehensive tests
8. **Use proper transactions**: Use transactions for operations that modify multiple entities
9. **Use proper pagination**: Use pagination for operations that return large result sets
10. **Use proper filtering**: Use filtering for operations that need to filter results
