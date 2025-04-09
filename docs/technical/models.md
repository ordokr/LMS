# Data Models

*Generated on 2025-04-07*

This document describes the data models used in the application.

## Category

```rust
struct Category {

    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub parent_id: Option<i64>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | Option< | |
| name | String | |
| slug | String | |
| description | Option< | |
| color | Option< | |
| text_color | Option< | |
| parent_id | Option< | |
| position | i32 | |
| created_at | DateTime< | |
| updated_at | DateTime< | |
| is_deleted | bool | |

### Methods

- `fn new(name: String, slug: String, description: Option<String>)`

### Port Source Reference

**Canvas Equivalent:** `app/models/category.rb`

**Discourse Equivalent:** `app/models/category.rb`

---

## Conversion

### Methods

- `fn topic_from_canvas(canvas_topic: &canvas_lms::DiscussionTopic)`
- `fn topic_from_discourse(discourse_topic: &discourse::Topic)`

### Port Source Reference

**Canvas Equivalent:** `app/models/conversion.rb`

**Discourse Equivalent:** `app/models/conversion.rb`

---

## Course

```rust
struct Course {

    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: i64,
    pub created_at: DateTime<Utc>,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | i64 | |
| title | String | |
| description | Option< | |
| instructor_id | i64 | |
| created_at | DateTime< | |

### Port Source Reference

**Canvas Equivalent:** `app/models/course.rb`

**Discourse Equivalent:** `app/models/course.rb`

---

## Discussion

```rust
struct Discussion {

    pub id: String,
    pub course_id: String,
    pub title: String,
    pub content: String,
    pub topic_id: Option<String>,
    pub status: DiscussionStatus,
    pub created_at: String,
    pub updated_at: String,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | String | |
| course_id | String | |
| title | String | |
| content | String | |
| topic_id | Option< | |
| status | DiscussionStatus | |
| created_at | String | |
| updated_at | String | |

### Port Source Reference

**Canvas Equivalent:** `app/models/discussion.rb`

**Discourse Equivalent:** `app/models/discussion.rb`

---

## Discussion Mapping

```rust
struct DiscussionMapping {

    pub id: String,
    pub canvas_discussion_id: String,
    pub discourse_topic_id: String,
    pub course_category_id: String,
    pub title: String,
    pub last_sync: DateTime<Utc>,
    pub sync_enabled: bool,
    pub sync_posts: bool,
    pub created_at: DateTime<Utc>,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | String | |
| canvas_discussion_id | String | |
| discourse_topic_id | String | |
| course_category_id | String | |
| title | String | |
| last_sync | DateTime< | |
| sync_enabled | bool | |
| sync_posts | bool | |
| created_at | DateTime< | |

### Methods

- `fn new(
        canvas_discussion_id: &str,
        discourse_topic_id: &str,
        course_category_id: &str,
        title: &str,
    )`
- `fn new(mapping_id: &str)`

### Port Source Reference

**Canvas Equivalent:** `app/models/discussion_mapping.rb`

**Discourse Equivalent:** `app/models/discussion_mapping.rb`

---

## Ids

### Port Source Reference

**Canvas Equivalent:** `app/models/ids.rb`

**Discourse Equivalent:** `app/models/ids.rb`

---

## Integration

```rust
struct CourseCategory {

    pub id: Uuid,
    pub canvas_course_id: String,
    pub discourse_category_id: i64,
    pub sync_enabled: bool,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | Uuid | |
| canvas_course_id | String | |
| discourse_category_id | i64 | |
| sync_enabled | bool | |
| last_synced_at | Option< | |
| created_at | DateTime< | |
| updated_at | DateTime< | |

### Port Source Reference

**Canvas Equivalent:** `app/models/integration.rb`

**Discourse Equivalent:** `app/models/integration.rb`

---

## Mapping

```rust
struct CourseCategoryMapping {

    pub id: i64,
    pub course_id: i64,
    pub category_id: i64,
    pub sync_enabled: bool,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | i64 | |
| course_id | i64 | |
| category_id | i64 | |
| sync_enabled | bool | |
| last_synced_at | Option< | |
| created_at | DateTime< | |
| updated_at | DateTime< | |

### Methods

- `fn new(course_id: i64, category_id: i64)`

### Port Source Reference

**Canvas Equivalent:** `app/models/mapping.rb`

**Discourse Equivalent:** `app/models/mapping.rb`

---

## Mod

### Port Source Reference

**Canvas Equivalent:** `app/models/mod.rb`

**Discourse Equivalent:** `app/models/mod.rb`

---

## Module

```rust
struct Module {

    pub id: String,
    pub course_id: String,
    pub title: String,
    pub position: i32,
    pub items_count: i32,
    pub publish_final_grade: bool,
    pub published: bool,
    pub status: ModuleStatus,
    pub created_at: String,
    pub updated_at: String,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | String | |
| course_id | String | |
| title | String | |
| position | i32 | |
| items_count | i32 | |
| publish_final_grade | bool | |
| published | bool | |
| status | ModuleStatus | |
| created_at | String | |
| updated_at | String | |

### Port Source Reference

**Canvas Equivalent:** `app/models/module.rb`

**Discourse Equivalent:** `app/models/module.rb`

---

## Notification

```rust
struct Notification {

    pub id: String,
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub status: NotificationStatus,
    pub reference_id: Option<String>,
    pub reference_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | String | |
| user_id | String | |
| title | String | |
| message | String | |
| notification_type | NotificationType | |
| status | NotificationStatus | |
| reference_id | Option< | |
| reference_type | Option< | |
| created_at | String | |
| updated_at | String | |

### Port Source Reference

**Canvas Equivalent:** `app/models/notification.rb`

**Discourse Equivalent:** `app/models/notification.rb`

---

## Post

```rust
struct Post {

    pub id: Option<i64>,
    pub topic_id: i64,
    pub user_id: i64,
    pub content: String,
    pub content_html: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | Option< | |
| topic_id | i64 | |
| user_id | i64 | |
| content | String | |
| content_html | String | |
| created_at | DateTime< | |
| updated_at | DateTime< | |
| is_deleted | bool | |

### Methods

- `fn new(topic_id: i64, user_id: i64, content: String)`

### Port Source Reference

**Canvas Equivalent:** `app/models/post.rb`

**Discourse Equivalent:** `app/models/post.rb`

---

## Submission

```rust
struct Submission {

    pub id: String,
    pub assignment_id: String,
    pub user_id: String,
    pub content: String,
    pub attachments: Vec<String>,
    pub status: SubmissionStatus,
    pub score: Option<f64>,
    pub feedback: Option<String>,
    pub submitted_at: String,
    pub graded_at: Option<String>,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | String | |
| assignment_id | String | |
| user_id | String | |
| content | String | |
| attachments | Vec< | |
| status | SubmissionStatus | |
| score | Option< | |
| feedback | Option< | |
| submitted_at | String | |
| graded_at | Option< | |

### Port Source Reference

**Canvas Equivalent:** `app/models/submission.rb`

**Discourse Equivalent:** `app/models/submission.rb`

---

## Sync Config

```rust
struct SyncConfig {

    pub enabled: bool,
    pub sync_interval_seconds: u32,      // How often to perform sync
    pub check_interval_seconds: u32,     // How often to check if sync is needed
    pub max_retries: u32,
    pub retry_delay_seconds: u32,
    pub sync_courses: bool,
    pub sync_discussions: bool,
    pub sync_users: bool,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| enabled | bool | |
| sync_interval_seconds | u32 | |
| check_interval_seconds | u32 | |
| max_retries | u32 | |
| retry_delay_seconds | u32 | |
| sync_courses | bool | |
| sync_discussions | bool | |
| sync_users | bool | |

### Port Source Reference

**Canvas Equivalent:** `app/models/sync_config.rb`

**Discourse Equivalent:** `app/models/sync_config.rb`

---

## Tag

```rust
struct Tag {

    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | Option< | |
| name | String | |
| description | Option< | |
| created_at | DateTime< | |
| updated_at | DateTime< | |
| is_deleted | bool | |

### Methods

- `fn new(name: String, description: Option<String>)`

### Port Source Reference

**Canvas Equivalent:** `app/models/tag.rb`

**Discourse Equivalent:** `app/models/tag.rb`

---

## Topic

```rust
struct Topic {

    pub id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub category_id: i64,
    pub user_id: i64,
    pub views: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub is_closed: bool,
    pub is_pinned: bool,
    pub is_deleted: bool,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | Option< | |
| title | String | |
| slug | String | |
| category_id | i64 | |
| user_id | i64 | |
| views | i32 | |
| created_at | DateTime< | |
| updated_at | DateTime< | |
| last_posted_at | Option< | |
| is_closed | bool | |
| is_pinned | bool | |
| is_deleted | bool | |

### Methods

- `fn new(title: String, slug: String, category_id: i64, user_id: i64)`

### Port Source Reference

**Canvas Equivalent:** `app/models/topic.rb`

**Discourse Equivalent:** `app/models/topic.rb`

---

## User

```rust
struct User {

    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub created_at: String,
    pub updated_at: String,

}
```

### Fields

| Name | Type | Description |
|------|------|-------------|
| id | String | |
| email | String | |
| first_name | String | |
| last_name | String | |
| role | UserRole | |
| created_at | String | |
| updated_at | String | |

### Port Source Reference

**Canvas Equivalent:** `app/models/user.rb`

**Discourse Equivalent:** `app/models/user.rb`

---

