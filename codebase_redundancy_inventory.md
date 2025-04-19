# Codebase Redundancy Inventory

This document catalogs redundant implementations and code waste identified in the LMS codebase.

## 1. API Client Implementations

### 1.1 Canvas API Clients
- `src/api/canvas_client.rs` - Full implementation with retry logic
- `src/clients/canvas_client.rs` - Duplicate implementation with mock functionality
- `src-tauri/src/services/canvas_client.rs` - Another implementation with similar functionality
- `src-tauri/src/api/canvas_client.rs` - Yet another implementation
- `src/clients/canvas.rs` - Mock implementation

### 1.2 Discourse API Clients
- `src/api/discourse_client.rs` - Full implementation with retry logic
- `src/clients/discourse_client.rs` - Duplicate implementation with mock functionality
- `src-tauri/src/services/discourse_client.rs` - Another implementation with similar functionality
- `src/clients/discourse.rs` - Mock implementation

### 1.3 Base/Generic API Clients
- `src/api/base_client.rs` - Generic API client implementation
- `src/utils/api_client.rs` - Another generic API client

### 1.4 HTTP Client Configurations
- `src-tauri/src/api/mod.rs` - `get_http_client()` function
- `src-tauri/src/quiz/cmi5/client.rs` - Custom HTTP client creation
- `src-tauri/src/server/http.rs` - Server HTTP configuration
- Multiple client implementations each creating their own HTTP client

## 2. Error Handling Implementations

### 2.1 Error Type Definitions
- `src-tauri/src/error.rs` - Main error enum
- `src/error.rs` - Duplicate error enum with similar categories
- `src-tauri/src/core/errors.rs` - Another error enum
- `src/services/errors.rs` - API-specific error enum
- `src-tauri/src/utils/errors.rs` - Another API-specific error enum

### 2.2 Error Handling Services
- `src-tauri/src/utils/error_handler.rs` - Error handling service
- `src/services/error_handling_service.rs` - Another error handling service

### 2.3 Error Mapping Functions
- Multiple implementations of error mapping functions across different modules

## 3. Repository Implementations

### 3.1 User Repositories
- `src-tauri/src/repositories/unified/user_repository.rs`
- `src-tauri/src/database/repositories/user_repository.rs`
- `src-tauri/src/database/repositories/user.rs`

### 3.2 Course Repositories
- `src-tauri/src/database/repositories/course.rs`
- `src-tauri/src/db/course_repository.rs`
- `src/db/sqlite.rs` (contains `SqliteCourseRepository`)

### 3.3 Forum Repositories
- `src-tauri/src/repositories/forum_category_repository.rs`
- `src-tauri/src/repositories/forum.rs`

### 3.4 Module Repositories
- `src-tauri/src/repositories/module_repository.rs`
- `src-tauri/src/db/module_repository.rs`
- `src-tauri/src/db/module_repository_impl.rs`

### 3.5 Repository Patterns
- Trait-based approach in some repositories
- Direct implementation in others
- Inconsistent method naming and parameter ordering

## 4. Model Implementations

### 4.1 User Models
- Multiple User model implementations across different modules

### 4.2 Course Models
- Multiple Course model implementations

### 4.3 Discussion/Forum Models
- Multiple implementations for forum/discussion entities

## 5. Utility Functions

### 5.1 HTTP Utilities
- Multiple implementations of HTTP request handling
- Duplicate retry logic
- Redundant authentication handling

### 5.2 Error Utilities
- Multiple implementations of error formatting
- Duplicate error categorization

## 6. Synchronization Services

### 6.1 Sync Services
- `src/services/bidirectional_sync_service.rs`
- `src/services/incremental_sync_service.rs`
- `src/services/sync_manager.rs`
- Overlapping functionality and duplicate code

## 7. Redundant Files

### 7.1 Duplicate Module Definitions
- Multiple module definition files with overlapping exports

### 7.2 Unused or Deprecated Files
- Several files appear to be unused or contain deprecated functionality

## 8. Code Quality Issues

### 8.1 Commented-Out Code
- Multiple instances of commented-out code that should be removed

### 8.2 TODO Comments
- Many TODO comments indicating incomplete implementations

### 8.3 Mock Implementations
- Multiple mock implementations that could be consolidated

## Next Steps

1. Prioritize areas for cleanup based on impact and complexity
2. Create unified implementations for each major component
3. Migrate existing code to use the unified implementations
4. Remove redundant code after thorough testing
