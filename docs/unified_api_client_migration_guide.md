# Unified API Client Migration Guide

This guide provides instructions for migrating from the old API client implementations to the new unified API clients implemented in Phase 3 of the LMS Codebase Cleanup Plan.

## Overview

The migration process involves:

1. Replacing imports from old API client files with imports from the unified API clients
2. Using the appropriate API client implementation for API interactions
3. Updating method calls to use the new unified API client interfaces

## Import Changes

### Old Imports (Before)

```rust
// Old Canvas client imports
use crate::api::canvas_client::CanvasClient;
use crate::api::canvas_client::CanvasApiError;

// Old Discourse client imports
use crate::api::discourse_client::DiscourseClient;
use crate::api::discourse_client::DiscourseApiError;

// Old service client imports
use crate::services::canvas_client::CanvasClient as ServiceCanvasClient;
use crate::services::canvas_client::CanvasError;
use crate::services::discourse_client::DiscourseClient as ServiceDiscourseClient;
```

### New Imports (After)

```rust
// New unified API client imports
use crate::api::unified_clients::{
    ApiClient, ApiClientConfig, ApiError, Result,
    CanvasApiClient, DiscourseApiClient,
    create_canvas_client, create_discourse_client, create_client
};
```

## Client Creation

### Old Client Creation (Before)

```rust
// Create Canvas client
let canvas_client = CanvasClient::new("https://canvas.example.com", "api_key");

// Create Discourse client
let discourse_client = DiscourseClient::new("https://discourse.example.com", "api_key", "system");

// Create service Canvas client
let canvas_config = CanvasConfig {
    url: "https://canvas.example.com".to_string(),
    api_token: "api_key".to_string(),
    timeout_seconds: 30,
};
let service_canvas_client = ServiceCanvasClient::new(canvas_config);
```

### New Client Creation (After)

```rust
// Create Canvas client
let canvas_client = create_canvas_client("https://canvas.example.com", "api_key")?;

// Create Discourse client
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

## Method Usage

### Old Method Usage (Before)

```rust
// Canvas client methods
let notifications = canvas_client.get_user_notifications("user123").await?;
let notification = canvas_client.mark_notification_as_read("notification123").await?;
let created = canvas_client.create_notification(&notification_data).await?;

// Discourse client methods
let categories = discourse_client.get_categories().await?;
let topic = discourse_client.get_topic(123).await?;
let posts = discourse_client.get_topic_posts(123).await?;

// Service Canvas client methods
let is_connected = service_canvas_client.test_connection().await?;
let announcement = service_canvas_client.get_announcement("course123", "announcement123").await?;
let announcement_id = service_canvas_client.create_announcement("course123", "Title", "Message").await?;
```

### New Method Usage (After)

```rust
// Canvas client methods
let notifications = canvas_client.get_user_notifications("user123").await?;
let notification = canvas_client.mark_notification_as_read("notification123").await?;
let created = canvas_client.create_notification(&notification_data).await?;

// Additional Canvas client methods
let user = canvas_client.get_user("user123").await?;
let course = canvas_client.get_course("course123").await?;
let assignment = canvas_client.get_assignment("course123", "assignment123").await?;
let submission = canvas_client.get_submission("course123", "assignment123", "user123").await?;
let topic = canvas_client.get_discussion_topic("course123", "topic123").await?;

// Discourse client methods
let categories = discourse_client.get_categories().await?;
let topic = discourse_client.get_topic("123").await?;
let posts = discourse_client.get_topic_posts("123", &pagination).await?;

// Additional Discourse client methods
let user = discourse_client.get_user("user123").await?;
let user_by_username = discourse_client.get_user_by_username("username").await?;
let category = discourse_client.get_category("category123").await?;
let group = discourse_client.get_group("group123").await?;
let group_members = discourse_client.get_group_members("group123", &pagination).await?;

// Generic API client methods
let response: User = client.get("/api/v1/users/123", None).await?;
let paginated: PaginatedResponse<Course> = client.get_paginated("/api/v1/courses", &pagination, None).await?;
let created: Assignment = client.post("/api/v1/courses/123/assignments", Some(&assignment_data), None).await?;
let updated: Topic = client.put("/api/v1/topics/123", Some(&topic_data), None).await?;
let deleted: serde_json::Value = client.delete("/api/v1/resources/123", None).await?;
```

## Error Handling

### Old Error Handling (Before)

```rust
// Canvas client error handling
match result {
    Ok(data) => {
        // Handle success
    },
    Err(CanvasApiError::HttpError(e)) => {
        // Handle HTTP error
    },
    Err(CanvasApiError::AuthError(msg)) => {
        // Handle authentication error
    },
    Err(CanvasApiError::ApiError { status_code, message }) => {
        // Handle API error
    },
    Err(CanvasApiError::SerializationError(e)) => {
        // Handle serialization error
    },
}

// Service Canvas client error handling
match result {
    Ok(data) => {
        // Handle success
    },
    Err(CanvasError::Network(e)) => {
        // Handle network error
    },
    Err(CanvasError::Authentication(msg)) => {
        // Handle authentication error
    },
    Err(CanvasError::NotFound(msg)) => {
        // Handle not found error
    },
    Err(CanvasError::Api(msg)) => {
        // Handle API error
    },
    Err(CanvasError::RateLimit) => {
        // Handle rate limit error
    },
    Err(CanvasError::InvalidResponse(msg)) => {
        // Handle invalid response error
    },
    Err(CanvasError::Configuration(msg)) => {
        // Handle configuration error
    },
}
```

### New Error Handling (After)

```rust
// Unified API client error handling
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

## Pagination

### Old Pagination (Before)

```rust
// Manual pagination
let mut page = 1;
let per_page = 50;
let mut all_items = Vec::new();

loop {
    let url = format!("/api/v1/courses?page={}&per_page={}", page, per_page);
    let items: Vec<Course> = client.get(&url, None).await?;
    
    if items.is_empty() {
        break;
    }
    
    all_items.extend(items);
    page += 1;
}
```

### New Pagination (After)

```rust
// Using PaginationParams
let pagination = PaginationParams {
    page: Some(1),
    per_page: Some(50),
    cursor: None,
};

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

## Retry and Error Recovery

### Old Retry Logic (Before)

```rust
// Manual retry logic
let mut retries = 0;
let max_retries = 3;

loop {
    match client.get_user("user123").await {
        Ok(user) => {
            return Ok(user);
        },
        Err(e) => {
            if retries >= max_retries {
                return Err(e);
            }
            
            if let CanvasApiError::HttpError(http_err) = &e {
                if http_err.is_timeout() || http_err.is_connect() {
                    retries += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(retries))).await;
                    continue;
                }
            }
            
            return Err(e);
        }
    }
}
```

### New Retry Logic (After)

```rust
// Automatic retry with exponential backoff
let config = ApiClientConfig {
    base_url: "https://canvas.example.com".to_string(),
    api_key: "api_key".to_string(),
    max_retries: 3,
    use_exponential_backoff: true,
    // Other configuration options...
    ..Default::default()
};

let client = create_api_client(config)?;

// The client will automatically retry on network errors, timeouts, and server errors
let user = client.get("/api/v1/users/123", None).await?;
```

## Testing

### Old Testing (Before)

```rust
#[tokio::test]
async fn test_canvas_client() {
    let client = CanvasClient::new("https://example.com", "test_key");
    let notifications = client.get_user_notifications("user1").await.unwrap();
    assert!(!notifications.is_empty());
    assert_eq!(notifications[0].id, "canvas1");
}
```

### New Testing (After)

```rust
#[tokio::test]
async fn test_canvas_client() {
    // Using mockito for HTTP mocking
    let _m = mock("GET", "/api/v1/users/123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"123","name":"Test User","email":"test@example.com","login_id":"testuser"}"#)
        .create();
        
    let client = CanvasApiClient::new(&server_url(), "test_key").unwrap();
    let user = client.get_user("123").await.unwrap();
    
    assert_eq!(user.name, "Test User");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.username, "testuser");
}
```

## Common Migration Issues

### 1. Error Type Changes

The unified API clients use a single `ApiError` type instead of separate error types for each client. Update your error handling code to use the new error type.

### 2. Method Signature Changes

Some method signatures have changed to be more consistent. For example, many methods now take string slices (`&str`) instead of owned strings (`String`).

### 3. Return Type Changes

The unified API clients return `Result<T, ApiError>` for all methods, where `T` is the expected return type. This is different from some old clients that returned custom result types.

### 4. Configuration Changes

The unified API clients use a single `ApiClientConfig` struct for configuration instead of separate configuration types for each client.

## Conclusion

By following this migration guide, you can smoothly transition from the old API client implementations to the new unified API clients. The unified API clients provide a more consistent, maintainable, and extensible foundation for API interactions in the LMS application.
