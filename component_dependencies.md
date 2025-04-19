# Component Dependencies

This document maps the dependencies between different components in the LMS codebase. Understanding these relationships is crucial for safe refactoring.

## API Client Dependencies

### Canvas API Clients
- `src/api/canvas_client.rs` 
  - Used by: `src/services/canvas_service.rs`, `src/services/bidirectional_sync_service.rs`
  - Dependencies: `src/api/base_client.rs`, `src/error.rs`

- `src/clients/canvas_client.rs`
  - Used by: `src/services/mock_canvas_service.rs`, `src/tests/canvas_tests.rs`
  - Dependencies: `src/utils/api_client.rs`, `src/error.rs`

- `src-tauri/src/services/canvas_client.rs`
  - Used by: `src-tauri/src/services/sync_service.rs`, `src-tauri/src/handlers/canvas_handler.rs`
  - Dependencies: `src-tauri/src/api/mod.rs`, `src-tauri/src/error.rs`

- `src-tauri/src/api/canvas_client.rs`
  - Used by: `src-tauri/src/services/canvas_service.rs`
  - Dependencies: `src-tauri/src/api/mod.rs`, `src-tauri/src/error.rs`

- `src/clients/canvas.rs` (Mock)
  - Used by: `src/tests/canvas_mock_tests.rs`
  - Dependencies: None

### Discourse API Clients
- `src/api/discourse_client.rs`
  - Used by: `src/services/discourse_service.rs`, `src/services/bidirectional_sync_service.rs`
  - Dependencies: `src/api/base_client.rs`, `src/error.rs`

- `src/clients/discourse_client.rs`
  - Used by: `src/services/mock_discourse_service.rs`, `src/tests/discourse_tests.rs`
  - Dependencies: `src/utils/api_client.rs`, `src/error.rs`

- `src-tauri/src/services/discourse_client.rs`
  - Used by: `src-tauri/src/services/sync_service.rs`, `src-tauri/src/handlers/discourse_handler.rs`
  - Dependencies: `src-tauri/src/api/mod.rs`, `src-tauri/src/error.rs`

- `src/clients/discourse.rs` (Mock)
  - Used by: `src/tests/discourse_mock_tests.rs`
  - Dependencies: None

## Repository Dependencies

### User Repositories
- `src-tauri/src/repositories/unified/user_repository.rs`
  - Used by: `src-tauri/src/services/user_service.rs`, `src-tauri/src/handlers/user_handler.rs`
  - Dependencies: `src-tauri/src/models/unified/user.rs`, `src-tauri/src/database/connection.rs`

- `src-tauri/src/database/repositories/user_repository.rs`
  - Used by: `src-tauri/src/services/auth_service.rs`
  - Dependencies: `src-tauri/src/models/user.rs`, `src-tauri/src/database/connection.rs`

- `src-tauri/src/database/repositories/user.rs`
  - Used by: `src-tauri/src/services/profile_service.rs`
  - Dependencies: `src-tauri/src/models/user/user.rs`, `src-tauri/src/database/connection.rs`

### Course Repositories
- `src-tauri/src/database/repositories/course.rs`
  - Used by: `src-tauri/src/services/course_service.rs`
  - Dependencies: `src-tauri/src/models/course.rs`, `src-tauri/src/database/connection.rs`

- `src-tauri/src/db/course_repository.rs`
  - Used by: `src-tauri/src/services/enrollment_service.rs`
  - Dependencies: `src-tauri/src/models/course.rs`, `src-tauri/src/db/connection.rs`

- `src/db/sqlite.rs` (SqliteCourseRepository)
  - Used by: `src/services/course_service.rs`
  - Dependencies: `src/models/course.rs`, `src/db/connection.rs`

### Forum Repositories
- `src-tauri/src/repositories/forum_category_repository.rs`
  - Used by: `src-tauri/src/services/forum_service.rs`
  - Dependencies: `src-tauri/src/models/forum/category.rs`, `src-tauri/src/database/connection.rs`

- `src-tauri/src/repositories/forum.rs`
  - Used by: `src-tauri/src/services/discussion_service.rs`
  - Dependencies: `src-tauri/src/models/forum.rs`, `src-tauri/src/database/connection.rs`

### Module Repositories
- `src-tauri/src/repositories/module_repository.rs`
  - Used by: `src-tauri/src/services/module_service.rs`
  - Dependencies: `src-tauri/src/models/module.rs`, `src-tauri/src/database/connection.rs`

- `src-tauri/src/db/module_repository.rs`
  - Used by: `src-tauri/src/services/content_service.rs`
  - Dependencies: `src-tauri/src/models/module.rs`, `src-tauri/src/db/connection.rs`

- `src-tauri/src/db/module_repository_impl.rs`
  - Used by: `src-tauri/src/services/module_service.rs`
  - Dependencies: `src-tauri/src/models/module.rs`, `src-tauri/src/db/connection.rs`

## Error Handling Dependencies

### Error Type Definitions
- `src-tauri/src/error.rs`
  - Used by: Most files in `src-tauri/src/`
  - Dependencies: None

- `src/error.rs`
  - Used by: Most files in `src/`
  - Dependencies: None

- `src-tauri/src/core/errors.rs`
  - Used by: Files in `src-tauri/src/core/`
  - Dependencies: None

- `src/services/errors.rs`
  - Used by: Files in `src/services/`
  - Dependencies: None

- `src-tauri/src/utils/errors.rs`
  - Used by: Files in `src-tauri/src/utils/`
  - Dependencies: None

### Error Handling Services
- `src-tauri/src/utils/error_handler.rs`
  - Used by: Various services and handlers
  - Dependencies: `src-tauri/src/error.rs`

- `src/services/error_handling_service.rs`
  - Used by: Various services
  - Dependencies: `src/error.rs`

## Synchronization Services

- `src/services/bidirectional_sync_service.rs`
  - Used by: `src/services/sync_manager.rs`
  - Dependencies: `src/api/canvas_client.rs`, `src/api/discourse_client.rs`, `src/models/sync_state.rs`

- `src/services/incremental_sync_service.rs`
  - Used by: `src/services/sync_manager.rs`
  - Dependencies: `src/api/canvas_client.rs`, `src/api/discourse_client.rs`, `src/models/sync_state.rs`

- `src/services/sync_manager.rs`
  - Used by: `src/handlers/sync_handler.rs`
  - Dependencies: `src/services/bidirectional_sync_service.rs`, `src/services/incremental_sync_service.rs`

## Utility Functions

### HTTP Utilities
- Multiple implementations across:
  - `src/utils/http.rs`
  - `src-tauri/src/utils/http.rs`
  - `src/api/base_client.rs`
  - `src/utils/api_client.rs`

### Error Utilities
- Multiple implementations across:
  - `src/utils/error.rs`
  - `src-tauri/src/utils/error.rs`
  - `src/services/error_handling_service.rs`
  - `src-tauri/src/utils/error_handler.rs`
