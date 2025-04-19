# Unified Models

This directory contains the unified model implementations for the LMS application. These models replace the redundant model implementations that were scattered throughout the codebase.

## Overview

The unified models provide a single source of truth for all entity types in the LMS application. They are designed to be:

- **Consistent**: All models follow the same design patterns and conventions
- **Comprehensive**: All models include all fields and methods needed for all use cases
- **Compatible**: All models can be converted to/from external system formats (Canvas, Discourse)
- **Extensible**: All models include extensible metadata for future requirements

## Models

The following models are included:

- **User**: Represents a user in the system (student, instructor, admin)
- **Course**: Represents a course with its properties and relationships
- **Group**: Represents a group of users with membership management
- **Assignment**: Represents a course assignment with submission options
- **Topic**: Represents a discussion topic or forum thread
- **Submission**: Represents a student submission for an assignment

## Usage

### Creating a Model

```rust
// Create a User
let user = User::new(
    None,                     // ID (None for auto-generated)
    "johndoe".to_string(),    // Username
    "John Doe".to_string(),   // Name
    "john@example.com".to_string(), // Email
);

// Create a Course
let course = Course::new(
    None,                     // ID (None for auto-generated)
    "Introduction to Rust".to_string(), // Title
    Some("Learn Rust programming".to_string()), // Description
    Some(instructor_id),      // Instructor ID
);

// Create an Assignment
let assignment = Assignment::new(
    None,                     // ID (None for auto-generated)
    "Rust Project".to_string(), // Title
    Some("Build a Rust application".to_string()), // Description
    course_id,                // Course ID
);
```

### Using Repositories

```rust
// Create a repository
let user_repo = SqliteUserRepository::new(pool.clone());

// Find a user by ID
let user = user_repo.find_by_id(&user_id).await?;

// Find a user by username
let user = user_repo.find_by_username("johndoe").await?;

// Create a user
let created_user = user_repo.create(&user).await?;

// Update a user
let updated_user = user_repo.update(&user).await?;

// Delete a user
user_repo.delete(&user_id).await?;
```

### External System Integration

```rust
// Convert from Canvas user
let canvas_user = canvas_client.get_user("12345").await?;
let user = User::from_canvas_user(&canvas_user);

// Convert to Canvas user
let canvas_user = user.to_canvas_user();

// Convert from Discourse user
let discourse_user = discourse_client.get_user("12345").await?;
let user = User::from_discourse_user(&discourse_user);

// Convert to Discourse user
let discourse_user = user.to_discourse_user();
```

## Documentation

For more detailed documentation, see:

- [Unified Model Architecture](../../../docs/unified_model_architecture.md)
- [Unified Model Migration Guide](../../../docs/unified_model_migration_guide.md)

## Testing

Each model has comprehensive unit tests in its file. Integration tests for all models are available in the `tests/integration` directory.

To run the tests:

```bash
cargo test
```

## Contributing

When adding new functionality to the unified models:

1. Ensure it follows the existing design patterns and conventions
2. Add appropriate unit tests
3. Update the integration tests if necessary
4. Document the changes in the model file and update the architecture documentation if needed
