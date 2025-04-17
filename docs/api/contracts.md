# Ordo LMS & Forum: API Contracts

_Last updated: 2025-04-17_

<img alt="Status: Planning" src="https://img.shields.io/badge/status-planning-blue">

## Overview

This document defines the core API contracts for the Ordo LMS & Forum system. It serves as the definitive reference for all API interactions within the application, including both internal service-to-service communication and external client-facing endpoints.

Ordo's API architecture follows RESTful principles with the following key characteristics:
- JSON as the primary data interchange format
- JWT-based authentication
- Standardized error responses
- Versioned endpoints
- Offline-first capabilities with synchronization

## API Response Format

All API responses follow a standardized format to ensure consistency:

```json
{
  "success": true,
  "data": {
    // Response data here
  },
  "meta": {
    "version": "1.0",
    "timestamp": "2025-04-17T12:34:56Z"
  }
}
```

For error responses:

```json
{
  "success": false,
  "error": {
    "code": "RESOURCE_NOT_FOUND",
    "message": "The requested resource was not found",
    "details": {
      // Additional error details if available
    }
  },
  "meta": {
    "version": "1.0",
    "timestamp": "2025-04-17T12:34:56Z"
  }
}
```

## Authentication

Ordo uses JWT (JSON Web Tokens) for authentication. All authenticated requests require the token to be included in the Authorization header:

```
Authorization: Bearer <token>
```

### Authentication Endpoints

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/auth/login` | POST | User login with username/email and password | Planned |
| `/api/auth/refresh` | POST | Refresh an expired JWT token | Planned |
| `/api/auth/logout` | POST | Invalidate the current JWT token | Planned |
| `/api/auth/sso` | GET | Initiate SSO authentication flow | Planned |
| `/api/auth/register` | POST | Register a new user | Planned |

### Login Request

```json
{
  "email": "user@example.com",
  "password": "secure_password"
}
```

### Login Response

```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": "user_123",
      "name": "John Doe",
      "email": "user@example.com",
      "role": "student"
    }
  },
  "meta": {
    "version": "1.0",
    "timestamp": "2025-04-17T12:34:56Z"
  }
}
```

## Core Resource Endpoints

### Users

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/users` | GET | List all users with pagination | Planned |
| `/api/users/{id}` | GET | Get user details | Planned |
| `/api/users/{id}` | PUT | Update user details | Planned |
| `/api/users/{id}/enrollments` | GET | Get user's course enrollments | Planned |
| `/api/users/{id}/notifications` | GET | Get user's notifications | Planned |

### Courses

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/courses` | GET | List all courses with pagination | Planned |
| `/api/courses` | POST | Create a new course | Planned |
| `/api/courses/{id}` | GET | Get course details | Planned |
| `/api/courses/{id}` | PUT | Update course details | Planned |
| `/api/courses/{id}` | DELETE | Archive a course | Planned |
| `/api/courses/{id}/enrollments` | GET | List course enrollments | Planned |
| `/api/courses/{id}/modules` | GET | List course modules | Planned |
| `/api/courses/{id}/discussions` | GET | List course discussions | Planned |

### Assignments

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/courses/{course_id}/assignments` | GET | List assignments for a course | Planned |
| `/api/courses/{course_id}/assignments` | POST | Create a new assignment | Planned |
| `/api/assignments/{id}` | GET | Get assignment details | Planned |
| `/api/assignments/{id}` | PUT | Update assignment | Planned |
| `/api/assignments/{id}/submissions` | GET | List submissions for an assignment | Planned |
| `/api/assignments/{id}/submissions/{user_id}` | GET | Get a specific user's submission | Planned |
| `/api/assignments/{id}/submissions/{user_id}` | POST | Submit an assignment | Planned |

### Discussions

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/courses/{course_id}/discussions` | GET | List discussions for a course | Planned |
| `/api/courses/{course_id}/discussions` | POST | Create a new discussion | Planned |
| `/api/discussions/{id}` | GET | Get discussion details | Planned |
| `/api/discussions/{id}` | PUT | Update discussion | Planned |
| `/api/discussions/{id}/posts` | GET | List posts in a discussion | Planned |
| `/api/discussions/{id}/posts` | POST | Create a new post in a discussion | Planned |

### Categories (Forum)

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/categories` | GET | List all categories | Planned |
| `/api/categories` | POST | Create a new category | Planned |
| `/api/categories/{id}` | GET | Get category details | Planned |
| `/api/categories/{id}` | PUT | Update category | Planned |
| `/api/categories/{id}/topics` | GET | List topics in a category | Planned |

### Topics (Forum)

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/topics` | GET | List all topics with pagination | Planned |
| `/api/topics` | POST | Create a new topic | Planned |
| `/api/topics/{id}` | GET | Get topic details | Planned |
| `/api/topics/{id}` | PUT | Update topic | Planned |
| `/api/topics/{id}/posts` | GET | List posts in a topic | Planned |
| `/api/topics/{id}/posts` | POST | Create a new post in a topic | Planned |

### Posts (Forum)

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/posts` | GET | List all posts with pagination | Planned |
| `/api/posts/{id}` | GET | Get post details | Planned |
| `/api/posts/{id}` | PUT | Update post | Planned |
| `/api/posts/{id}` | DELETE | Delete post | Planned |
| `/api/posts/{id}/replies` | GET | List replies to a post | Planned |
| `/api/posts/{id}/reactions` | GET | List reactions to a post | Planned |
| `/api/posts/{id}/reactions` | POST | Add a reaction to a post | Planned |

### Notifications

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/notifications` | GET | List current user's notifications | Planned |
| `/api/notifications/{id}` | PUT | Mark notification as read | Planned |
| `/api/notifications/read-all` | POST | Mark all notifications as read | Planned |

## Integration Endpoints

These endpoints handle the integration between the LMS and forum components:

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/integration/course-category/map` | POST | Map a course to a forum category | Planned |
| `/api/integration/discussion-topic/map` | POST | Map a discussion to a forum topic | Planned |
| `/api/integration/sync/status` | GET | Get synchronization status | Planned |
| `/api/integration/sync/trigger` | POST | Trigger manual synchronization | Planned |

## Sync Engine Endpoints

These endpoints manage the offline-first capabilities:

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/sync/queue` | GET | Get pending sync operations | Planned |
| `/api/sync/queue` | POST | Add operations to sync queue | Planned |
| `/api/sync/execute` | POST | Execute synchronization | Planned |
| `/api/sync/conflicts` | GET | List sync conflicts | Planned |
| `/api/sync/conflicts/{id}/resolve` | POST | Resolve a sync conflict | Planned |

## Data Models

### User

```json
{
  "id": "string",
  "name": "string",
  "email": "string",
  "avatar_url": "string",
  "role": "admin|instructor|student",
  "created_at": "datetime",
  "updated_at": "datetime",
  "last_login": "datetime",
  "preferences": {
    "timezone": "string",
    "locale": "string",
    "notification_settings": {}
  }
}
```

### Course

```json
{
  "id": "string",
  "name": "string",
  "code": "string",
  "description": "string",
  "start_date": "datetime",
  "end_date": "datetime",
  "is_published": "boolean",
  "instructor_ids": ["string"],
  "created_at": "datetime",
  "updated_at": "datetime",
  "modules": ["Module"],
  "enrollment_count": "number"
}
```

### Assignment

```json
{
  "id": "string",
  "course_id": "string",
  "title": "string",
  "description": "string",
  "points_possible": "number",
  "due_date": "datetime",
  "unlock_date": "datetime",
  "lock_date": "datetime",
  "submission_types": ["string"],
  "is_published": "boolean",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Submission

```json
{
  "id": "string",
  "assignment_id": "string",
  "user_id": "string",
  "submitted_at": "datetime",
  "grade": "number",
  "score": "number",
  "feedback": "string",
  "attempt": "number",
  "attachments": ["Attachment"],
  "status": "submitted|graded|returned",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Discussion

```json
{
  "id": "string",
  "course_id": "string",
  "title": "string",
  "message": "string",
  "author_id": "string",
  "is_pinned": "boolean",
  "is_locked": "boolean",
  "reply_count": "number",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Category (Forum)

```json
{
  "id": "string",
  "name": "string",
  "description": "string",
  "color": "string",
  "parent_id": "string",
  "topic_count": "number",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Topic (Forum)

```json
{
  "id": "string",
  "category_id": "string",
  "title": "string",
  "author_id": "string",
  "is_closed": "boolean",
  "is_pinned": "boolean",
  "view_count": "number",
  "post_count": "number",
  "created_at": "datetime",
  "updated_at": "datetime",
  "last_posted_at": "datetime"
}
```

### Post (Forum)

```json
{
  "id": "string",
  "topic_id": "string",
  "author_id": "string",
  "content": "string",
  "reply_to_id": "string",
  "is_solution": "boolean",
  "reaction_count": "number",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Notification

```json
{
  "id": "string",
  "user_id": "string",
  "type": "string",
  "title": "string",
  "message": "string",
  "reference_type": "string",
  "reference_id": "string",
  "is_read": "boolean",
  "created_at": "datetime"
}
```

## Error Codes

| Code | Description |
|------|-------------|
| `INVALID_CREDENTIALS` | The provided username/password is incorrect |
| `UNAUTHORIZED` | The request requires authentication |
| `FORBIDDEN` | The user does not have permission to access the resource |
| `RESOURCE_NOT_FOUND` | The requested resource was not found |
| `VALIDATION_ERROR` | The request payload failed validation |
| `CONFLICT` | The request conflicts with the current state of the resource |
| `INTERNAL_ERROR` | An unexpected error occurred on the server |
| `SYNC_CONFLICT` | A synchronization conflict occurred |
| `RATE_LIMITED` | Too many requests, please try again later |

## Pagination

API endpoints that return collections support pagination using the following query parameters:

- `page`: The page number (1-based)
- `per_page`: Number of items per page (default: 20, max: 100)

Example:
```
GET /api/courses?page=2&per_page=25
```

Response includes pagination metadata:

```json
{
  "success": true,
  "data": [...],
  "meta": {
    "pagination": {
      "total_items": 243,
      "total_pages": 10,
      "current_page": 2,
      "per_page": 25,
      "next_page": 3,
      "prev_page": 1
    },
    "version": "1.0",
    "timestamp": "2025-04-17T12:34:56Z"
  }
}
```

## Filtering and Sorting

API endpoints that return collections support filtering and sorting using the following query parameters:

- `sort`: Field to sort by (prefix with `-` for descending order)
- `filter[field]`: Filter by field value

Example:
```
GET /api/courses?sort=-created_at&filter[is_published]=true
```

## Offline Support

The API is designed with offline-first capabilities in mind:

1. All endpoints that modify data return a unique operation ID
2. Clients can store these operations in their offline queue
3. When connectivity is restored, the sync API can be used to synchronize changes
4. Operations are implemented using CRDT principles to ensure conflict-free resolution

## API Versioning

The API is versioned through the URL path:

```
/api/v1/courses
```

## Rate Limiting

The API implements rate limiting to prevent abuse:

- 100 requests per minute per authenticated user
- 20 requests per minute per IP for unauthenticated requests

Response headers include rate limit information:

- `X-Rate-Limit-Limit`: The maximum number of requests allowed per time window
- `X-Rate-Limit-Remaining`: The number of requests remaining in the current time window
- `X-Rate-Limit-Reset`: The time when the current rate limit window resets in UTC epoch seconds

## Webhook Support

Ordo provides webhook notifications for key events:

| Endpoint | Method | Description | Status |
|----------|--------|-------------|--------|
| `/api/webhooks` | GET | List registered webhooks | Planned |
| `/api/webhooks` | POST | Register a new webhook | Planned |
| `/api/webhooks/{id}` | GET | Get webhook details | Planned |
| `/api/webhooks/{id}` | DELETE | Delete a webhook | Planned |
| `/api/webhooks/{id}/logs` | GET | View webhook delivery logs | Planned |

### Webhook Registration

```json
{
  "url": "https://example.com/webhook",
  "events": ["course.created", "assignment.submitted"],
  "secret": "webhook_secret_key"
}
```

## Implementation Status

The current implementation status of the API is:

- **Planned endpoints**: 100%
- **Implemented endpoints**: 0%
- **Documented endpoints**: 100%
- **Tested endpoints**: 0%

## Next Steps

1. Implement authentication endpoints
2. Implement core LMS endpoints (users, courses, assignments)
3. Implement forum endpoints
4. Implement integration endpoints
5. Add comprehensive test coverage
6. Deploy API gateway with proper monitoring

## References

- [REST API Best Practices](https://swagger.io/resources/articles/best-practices-in-api-design/)
- [JSON API Specification](https://jsonapi.org/)
- [OAuth 2.0 Specification](https://oauth.net/2/)