# Unified API Clients

This directory contains the unified API client implementations for the LMS application. These clients replace the redundant API client implementations that were scattered throughout the codebase.

## Overview

The unified API clients provide a consistent interface for interacting with external APIs. They are designed to be:

- **Consistent**: All clients follow the same design patterns and conventions
- **Comprehensive**: All clients include all methods needed for all use cases
- **Configurable**: All clients support flexible configuration options
- **Resilient**: All clients include error handling and retry mechanisms

## Clients

The following clients are included:

- **BaseApiClient**: Base implementation of the ApiClient trait
- **CanvasApiClient**: Client for interacting with the Canvas LMS API
- **DiscourseApiClient**: Client for interacting with the Discourse API

## Usage

### Creating a Client

```rust
// Create a Canvas client
let canvas_client = create_canvas_client("https://canvas.example.com", "api_key")?;

// Create a Discourse client
let discourse_client = create_discourse_client("https://discourse.example.com", "api_key", "system")?;

// Create a generic client
let client = create_client("canvas", "https://canvas.example.com", "api_key", None)?;

// Create a client with custom configuration
let config = ApiClientConfig {
    base_url: "https://canvas.example.com".to_string(),
    api_key: "api_key".to_string(),
    timeout_seconds: 30,
    max_retries: 3,
    use_exponential_backoff: true,
    enable_circuit_breaker: true,
    max_connections_per_host: 10,
    enable_compression: true,
    additional_headers: Vec::new(),
    api_username: None,
};
let custom_client = create_api_client(config)?;
```

### Using a Client

```rust
// Canvas client methods
let user = canvas_client.get_user("user123").await?;
let course = canvas_client.get_course("course123").await?;
let assignment = canvas_client.get_assignment("course123", "assignment123").await?;
let submission = canvas_client.get_submission("course123", "assignment123", "user123").await?;
let topic = canvas_client.get_discussion_topic("course123", "topic123").await?;

// Discourse client methods
let categories = discourse_client.get_categories().await?;
let topic = discourse_client.get_topic("123").await?;
let posts = discourse_client.get_topic_posts("123", &pagination).await?;
let user = discourse_client.get_user("user123").await?;
let group = discourse_client.get_group("group123").await?;

// Generic API client methods
let response: User = client.get("/api/v1/users/123", None).await?;
let paginated: PaginatedResponse<Course> = client.get_paginated("/api/v1/courses", &pagination, None).await?;
let created: Assignment = client.post("/api/v1/courses/123/assignments", Some(&assignment_data), None).await?;
let updated: Topic = client.put("/api/v1/topics/123", Some(&topic_data), None).await?;
let deleted: serde_json::Value = client.delete("/api/v1/resources/123", None).await?;
```

### Pagination

```rust
// Create pagination parameters
let pagination = PaginationParams {
    page: Some(1),
    per_page: Some(50),
    cursor: None,
};

// Get paginated results
let response: PaginatedResponse<Course> = client.get_paginated("/api/v1/courses", &pagination, None).await?;

// Access pagination information
let items = response.items;
let total = response.total;
let current_page = response.page;
let total_pages = response.total_pages;
let has_next = response.has_next;
let has_prev = response.has_prev;

// Get next page if available
if has_next {
    let next_pagination = PaginationParams {
        page: current_page.map(|p| p + 1),
        per_page: response.per_page,
        cursor: response.next_cursor,
    };
    
    let next_page: PaginatedResponse<Course> = client.get_paginated("/api/v1/courses", &next_pagination, None).await?;
    // Process next page
}
```

### Error Handling

```rust
match result {
    Ok(data) => {
        // Handle success
    },
    Err(ApiError::HttpError(e)) => {
        // Handle HTTP error
    },
    Err(ApiError::AuthError(msg)) => {
        // Handle authentication error
    },
    Err(ApiError::ApiError { status_code, message }) => {
        // Handle API error
    },
    Err(ApiError::SerializationError(e)) => {
        // Handle serialization error
    },
    Err(ApiError::RateLimitError { retry_after }) => {
        // Handle rate limit error
    },
    Err(ApiError::TimeoutError) => {
        // Handle timeout error
    },
    Err(ApiError::NetworkError(msg)) => {
        // Handle network error
    },
    Err(ApiError::UnexpectedError(msg)) => {
        // Handle unexpected error
    },
}
```

## Documentation

For more detailed documentation, see:

- [Unified API Client Migration Guide](../../../docs/unified_api_client_migration_guide.md)

## Testing

Each client has comprehensive unit tests in its file. The tests use mockito for HTTP mocking.

To run the tests:

```bash
cargo test
```

## Contributing

When adding new functionality to the unified API clients:

1. Ensure it follows the existing design patterns and conventions
2. Add appropriate unit tests
3. Update the documentation if needed
