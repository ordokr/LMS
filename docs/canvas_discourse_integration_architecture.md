# Canvas-Discourse Integration Architecture

This document describes the architecture and implementation details of the Canvas LMS and Discourse forum integration.

## Overview

The integration enables seamless data synchronization between Canvas LMS and Discourse forums, allowing for discussions to be mirrored across both platforms. The architecture supports unidirectional and bidirectional synchronization with conflict resolution.

## Core Components

### 1. DiscourseClient

The `DiscourseClient` provides a Rust interface to the Discourse API, handling authentication, request formatting, and response parsing.

```rust
pub struct DiscourseClient {
    base_url: String,
    api_key: String,
    api_username: String,
    client: Client,
}
```

#### Key Methods:

- `get_topic(topic_id)`: Fetches a topic from Discourse
- `get_topic_posts(topic_id)`: Fetches posts for a specific topic
- `get_categories()`: Fetches all categories
- `get_user_notifications()`: Fetches notifications for a user
- `mark_notification_as_read()`: Marks a notification as read
- `create_notification()`: Creates a new notification

### 2. Integration Services

#### DiscourseIntegration

Interface for integrating with Discourse forums:

```rust
pub trait DiscourseIntegration {
    async fn sync_topic(&self, discourse_topic_id: i64) -> Result<Topic, Error>;
    async fn sync_category(&self, discourse_category_id: i64) -> Result<Category, Error>;
    async fn sync_user(&self, discourse_user_id: i64) -> Result<User, Error>;
    async fn push_topic_to_discourse(&self, topic: &Topic) -> Result<i64, Error>;
    async fn push_post_to_discourse(&self, post: &Post) -> Result<i64, Error>;
}
```

#### DiscourseIntegrationService

Implementation of the `DiscourseIntegration` interface that:
- Fetches data from Discourse
- Converts Discourse models to local models
- Handles data mapping and relationships

#### IntegrationSyncService

Manages bidirectional synchronization with conflict resolution:

```rust
pub struct IntegrationSyncService<C, D> {
    db: DB,
    canvas: C,
    discourse: D,
    event_dispatcher: Arc<IntegrationEventDispatcher>,
    conflict_resolver: ConflictResolver,
}
```

#### ConflictResolver

Handles conflict resolution when the same entity is modified in both systems:

```rust
pub enum ConflictStrategy {
    PreferCanvas,
    PreferDiscourse,
    PreferMostRecent,
    MergePreferCanvas,
    MergePreferDiscourse,
}
```

### 3. Events and Listeners

The system uses an event-driven architecture to notify other components of sync operations:

```rust
pub trait IntegrationEventListener {
    async fn on_topic_synced_to_discourse(&self, topic: &Topic, mapping: &TopicMapping);
    async fn on_topic_synced_to_canvas(&self, topic: &Topic, mapping: &TopicMapping);
    async fn on_post_synced_to_discourse(&self, post: &Post, mapping: &PostMapping);
    async fn on_post_synced_to_canvas(&self, post: &Post, mapping: &PostMapping);
    async fn on_sync_failure(&self, error: &Error, entity_type: &str, entity_id: &str);
}
```

## Synchronization Process

### Unidirectional Sync
1. Fetch data from source system
2. Convert to local model
3. Push to target system
4. Create/update mapping between source and target IDs
5. Trigger appropriate events

### Bidirectional Sync
1. Fetch latest data from both systems
2. Detect conflicts using timestamp comparison
3. Resolve conflicts using the configured strategy
4. Update local storage with the resolved version
5. Push changes back to both systems for consistency
6. Update sync status and timestamps

## Conflict Resolution Strategies
The system supports multiple conflict resolution strategies:

1. **PreferCanvas**: Always use Canvas data when conflicts occur
2. **PreferDiscourse**: Always use Discourse data when conflicts occur
3. **PreferMostRecent**: Use the most recently updated data
4. **MergePreferCanvas**: Merge changes with Canvas taking precedence for conflicts
5. **MergePreferDiscourse**: Merge changes with Discourse taking precedence for conflicts

## Usage Examples

### Initializing the Integration Service

```rust
let canvas_client = Arc::new(CanvasClient::new("https://canvas.example.com", "api_key"));
let discourse_client = Arc::new(DiscourseClient::new(
    "https://discourse.example.com", 
    "api_key", 
    "system_user"
));

let integration_service = IntegrationService::new(canvas_client, discourse_client);
```

### Synchronizing an Announcement to Discourse

```rust
let announcement = Announcement {
    id: "canvas-announcement-1".to_string(),
    title: "Important Announcement".to_string(),
    message: "Course begins next week".to_string(),
    course_id: "course-1".to_string(),
};

let result = integration_service.sync_announcement_to_forum(announcement).await;

if result.success {
    println!("Synced to Discourse as topic {}", result.discourse_topic_id.unwrap());
}
```

### Bidirectional Sync with Conflict Resolution

```rust
let sync_service = IntegrationSyncService::new(db, canvas_integration, discourse_integration)
    .with_conflict_strategy(ConflictStrategy::MergePreferMostRecent);

// Sync a specific topic
let topic_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
let result = sync_service.sync_topic_bidirectional(topic_id).await;

// Sync all pending topics
sync_service.sync_all_pending().await;
```

## Authentication

The integration supports Single Sign-On (SSO) between Canvas and Discourse:

```rust
let canvas_user = CanvasUser {
    id: "user123".to_string(),
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
};

let auth_result = integration_service.authenticate_user_with_discourse(canvas_user).await;

if auth_result.success {
    println!("Successfully authenticated with Discourse");
    println!("SSO token: {}", auth_result.sso_token.unwrap());
}
```
