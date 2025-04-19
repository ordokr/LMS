# Unified Error Handling

This directory contains the unified error handling system for the LMS application. This system replaces the redundant error handling implementations that were scattered throughout the codebase.

## Overview

The unified error handling system provides a consistent approach to error handling across the application. It includes:

- **Error Types**: Consistent error types for different parts of the application
- **Error Context**: Ability to add context to errors for better debugging
- **Error Mapping**: Tools for mapping between different error types
- **Error Handling**: Centralized error handling with logging and UI notifications

## Components

### Error

The `Error` type is the main error type for the application. It includes:

- **Kind**: The kind of error (Database, Validation, Authentication, etc.)
- **Code**: Error code for categorization and client-side handling
- **Message**: Human-readable error message
- **Context**: Optional context information for debugging
- **Source**: Optional source error
- **Stack Trace**: Optional stack trace for debugging

### ApiError

The `ApiError` type is specific to API-related errors. It includes:

- **Kind**: The kind of API error (Offline, NetworkError, ServerError, etc.)
- **Message**: Human-readable error message
- **Status Code**: Optional status code for HTTP errors
- **Context**: Optional context information for debugging

### ErrorHandler

The `ErrorHandler` provides centralized error handling functionality:

- **Logging**: Log errors with appropriate log levels
- **UI Notifications**: Show errors in the UI
- **Panic Handling**: Capture and log unexpected panics
- **Result Handling**: Helper functions for handling results

### ErrorContext

The `ErrorContext` provides tools for adding context to errors:

- **with_context**: Add context to a Result
- **with_context_fn**: Add context to a Result using a function
- **ResultExt**: Extension trait for Result to add context

### ErrorMapper

The `ErrorMapper` provides tools for mapping between different error types:

- **map_error**: Map an error to an Error
- **map_api_error**: Map an error to an ApiError
- **map_result**: Map a result to a Result<T, Error>
- **map_api_result**: Map a result to an ApiResult<T>
- **MapErrorExt**: Extension trait for Result to map errors

## Usage

### Creating Errors

```rust
// Create a database error
let error = Error::database("Failed to connect to database");

// Create a not found error
let error = Error::not_found("User not found");

// Create an error with context
let error = Error::validation("Invalid input")
    .with_context("Username must be at least 3 characters");

// Create an error with source
let error = Error::parsing("Failed to parse JSON")
    .with_source(serde_json::Error::...);
```

### Adding Context to Results

```rust
// Add context to a Result
let result = db.get_user(user_id)
    .with_context("Failed to get user");

// Add context to a Result using a function
let result = db.get_user(user_id)
    .with_context_fn(|| format!("Failed to get user with ID {}", user_id));
```

### Mapping Errors

```rust
// Map an error to an Error
let error = map_error(sqlx_error, |e| Error::database(format!("Database error: {}", e)));

// Map a result to a Result<T, Error>
let result = map_result(db_result, |e| Error::database(format!("Database error: {}", e)));

// Using the extension trait
let result = db_result.map_error(|e| Error::database(format!("Database error: {}", e)));
```

### Handling Errors

```rust
// Handle an error
handle_error(&error);

// Handle a result
let value = get_global_error_handler().handle_result(result);

// Handle an API error
handle_api_error(&api_error);

// Handle an API result
let value = get_global_error_handler().handle_api_result(api_result);
```

## Error Conversion

The unified error handling system provides conversions between different error types:

- **sqlx::Error** -> **Error**
- **serde_json::Error** -> **Error**
- **reqwest::Error** -> **Error**
- **ApiError** -> **Error**
- **reqwest::Error** -> **ApiError**
- **serde_json::Error** -> **ApiError**

## Best Practices

1. **Use the appropriate error type**: Use `Error` for application errors and `ApiError` for API-related errors.
2. **Add context to errors**: Always add context to errors to make debugging easier.
3. **Use the error handler**: Use the error handler to log errors and show UI notifications.
4. **Map errors consistently**: Use the error mapper to map between different error types.
5. **Include source errors**: Include source errors when possible to preserve the error chain.
6. **Use appropriate error kinds**: Use the appropriate error kind for each error to ensure consistent handling.
