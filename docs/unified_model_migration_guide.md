# Unified Model Migration Guide

This guide provides instructions for migrating from the old model implementations to the new unified models implemented in Phase 2 of the LMS Codebase Cleanup Plan.

## Overview

The migration process involves:

1. Replacing imports from old model files with imports from the unified models
2. Using the appropriate repository implementation for data access
3. Using the conversion methods for external system integration

## Import Changes

### Old Imports (Before)

```rust
// Old User model imports
use crate::models::user::user::User;
use crate::models::user::UserRepository;

// Old Course model imports
use crate::models::course::Course;
use crate::models::course::CourseRepository;

// Old Group model imports
use crate::models::group::Group;
use crate::models::group::GroupRepository;

// Old Assignment model imports
use crate::models::assignment::Assignment;
use crate::models::assignment::AssignmentRepository;

// Old Topic/Discussion model imports
use crate::models::discussion::Discussion;
use crate::models::discussion::DiscussionRepository;

// Old Submission model imports
use crate::models::submission::Submission;
use crate::models::submission::SubmissionRepository;
```

### New Imports (After)

```rust
// New unified model imports
use crate::models::unified_models::{
    User, Course, Group, GroupMembership, Assignment, Topic, Submission,
    CourseStatus, CourseVisibility, GroupJoinLevel, GroupMembershipStatus,
    AssignmentStatus, GradingType, SubmissionType as AssignmentSubmissionType,
    TopicStatus, TopicVisibility, TopicType,
    SubmissionStatus, SubmissionType, SubmissionComment
};

// New unified repository imports
use crate::repositories::unified_repositories::{
    Repository, UserRepository, CourseRepository, GroupRepository,
    AssignmentRepository, TopicRepository, SubmissionRepository,
    SqliteUserRepository, SqliteCourseRepository, SqliteGroupRepository,
    SqliteAssignmentRepository, SqliteTopicRepository, SqliteSubmissionRepository
};
```

## Repository Usage

### Old Repository Usage (Before)

```rust
// Create repository
let user_repo = SqliteUserRepository::new(pool.clone());

// Find by ID
let user = user_repo.find_by_id("user123").await?;

// Find by username
let user = user_repo.find_by_username("johndoe").await?;

// Create user
let new_user = User {
    id: "user456".to_string(),
    username: "janedoe".to_string(),
    name: "Jane Doe".to_string(),
    email: "jane@example.com".to_string(),
    // Other fields...
};
let created_user = user_repo.create(new_user).await?;

// Update user
let mut user = user_repo.find_by_id("user456").await?;
user.name = "Jane Smith".to_string();
let updated_user = user_repo.update(user).await?;

// Delete user
user_repo.delete("user456").await?;
```

### New Repository Usage (After)

```rust
// Create repository
let user_repo = SqliteUserRepository::new(pool.clone());

// Find by ID
let user = user_repo.find_by_id(&"user123".to_string()).await?;

// Find by username
let user = user_repo.find_by_username("johndoe").await?;

// Create user
let new_user = User::new(
    Some("user456".to_string()),
    "janedoe".to_string(),
    "Jane Doe".to_string(),
    "jane@example.com".to_string(),
);
let created_user = user_repo.create(&new_user).await?;

// Update user
let mut user = user_repo.find_by_id(&"user456".to_string()).await?.unwrap();
user.name = "Jane Smith".to_string();
let updated_user = user_repo.update(&user).await?;

// Delete user
user_repo.delete(&"user456".to_string()).await?;
```

## Model Creation

### Old Model Creation (Before)

```rust
// Create User
let user = User {
    id: "user123".to_string(),
    username: "johndoe".to_string(),
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    // Other fields...
};

// Create Course
let course = Course {
    id: "course123".to_string(),
    title: "Introduction to Rust".to_string(),
    description: "Learn Rust programming".to_string(),
    // Other fields...
};

// Create Assignment
let assignment = Assignment {
    id: "assignment123".to_string(),
    title: "Rust Project".to_string(),
    description: "Build a Rust application".to_string(),
    course_id: "course123".to_string(),
    // Other fields...
};
```

### New Model Creation (After)

```rust
// Create User
let user = User::new(
    Some("user123".to_string()),
    "johndoe".to_string(),
    "John Doe".to_string(),
    "john@example.com".to_string(),
);

// Create Course
let course = Course::new(
    Some("course123".to_string()),
    "Introduction to Rust".to_string(),
    Some("Learn Rust programming".to_string()),
    Some("instructor123".to_string()),
);

// Create Assignment
let assignment = Assignment::new(
    Some("assignment123".to_string()),
    "Rust Project".to_string(),
    Some("Build a Rust application".to_string()),
    "course123".to_string(),
);
```

## External System Integration

### Old External System Integration (Before)

```rust
// Convert from Canvas user
let canvas_user = canvas_client.get_user("12345").await?;
let user = User {
    id: uuid::Uuid::new_v4().to_string(),
    username: canvas_user.login_id,
    name: canvas_user.name,
    email: canvas_user.email,
    // Other fields...
};

// Convert to Canvas user
let canvas_user = CanvasUser {
    id: user.canvas_id.unwrap(),
    login_id: user.username,
    name: user.name,
    email: user.email,
    // Other fields...
};
```

### New External System Integration (After)

```rust
// Convert from Canvas user
let canvas_user = canvas_client.get_user("12345").await?;
let user = User::from_canvas_user(&canvas_user);

// Convert to Canvas user
let canvas_user = user.to_canvas_user();
```

## Enum Usage

### Old Enum Usage (Before)

```rust
// Course status
let status = "active";
let course_status = match status {
    "active" => CourseStatus::Active,
    "archived" => CourseStatus::Archived,
    "deleted" => CourseStatus::Deleted,
    _ => CourseStatus::Draft,
};

// Assignment status
let status = "published";
let assignment_status = match status {
    "published" => AssignmentStatus::Published,
    "unpublished" => AssignmentStatus::Draft,
    _ => AssignmentStatus::Draft,
};
```

### New Enum Usage (After)

```rust
// Course status
let status = "active";
let course_status = CourseStatus::from(status);

// Assignment status
let status = "published";
let assignment_status = AssignmentStatus::from(status);
```

## Common Migration Patterns

### 1. Replace Direct Field Access with Methods

**Before:**
```rust
if submission.grade.is_some() {
    // Submission is graded
}
```

**After:**
```rust
if submission.is_graded() {
    // Submission is graded
}
```

### 2. Use Builder Methods for Complex Objects

**Before:**
```rust
let mut topic = Topic {
    id: uuid::Uuid::new_v4().to_string(),
    title: "Discussion Topic".to_string(),
    content: Some("Let's discuss this topic".to_string()),
    // Many other fields...
};
topic.is_pinned = true;
```

**After:**
```rust
let mut topic = Topic::new(
    None,
    "Discussion Topic".to_string(),
    Some("Let's discuss this topic".to_string()),
);
topic.pin();
```

### 3. Use Enum Methods for String Conversion

**Before:**
```rust
let status_str = match topic.status {
    TopicStatus::Open => "open",
    TopicStatus::Closed => "closed",
    TopicStatus::Archived => "archived",
    TopicStatus::Deleted => "deleted",
};
```

**After:**
```rust
let status_str = topic.status.to_string();
```

## Testing Changes

### Old Testing (Before)

```rust
#[tokio::test]
async fn test_user_repository() {
    let pool = create_test_db_pool().await;
    let user_repo = SqliteUserRepository::new(pool.clone());
    
    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: "testuser".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        // Other fields...
    };
    
    let created_user = user_repo.create(user).await.unwrap();
    assert_eq!(created_user.username, "testuser");
    
    // Other test assertions...
}
```

### New Testing (After)

```rust
#[tokio::test]
async fn test_user_repository() {
    let pool = init_test_db().await;
    let user_repo = SqliteUserRepository::new(pool.clone());
    
    let user = User::new(
        None,
        "testuser".to_string(),
        "Test User".to_string(),
        "test@example.com".to_string(),
    );
    
    let created_user = user_repo.create(&user).await.unwrap();
    assert_eq!(created_user.username, "testuser");
    
    // Other test assertions...
    
    cleanup_test_db().await;
}
```

## Common Migration Issues

### 1. Option vs. Non-Option Fields

Some fields that were required in old models might be optional in new models, or vice versa. Check the field definitions carefully.

### 2. Method Names

Method names may have changed. For example, `find_by_id` now returns `Result<Option<T>, Error>` instead of `Result<T, Error>`.

### 3. Repository Method Parameters

Repository methods now take references (`&T`) instead of values (`T`).

### 4. Error Types

The unified models use a consistent error type (`Error`) throughout the repository layer.

## Conclusion

By following this migration guide, you can smoothly transition from the old model implementations to the new unified models. The unified models provide a more consistent, maintainable, and extensible foundation for the LMS application.
