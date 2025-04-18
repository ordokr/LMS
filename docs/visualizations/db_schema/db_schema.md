# Database Schema

```mermaid
erDiagram
    users {
        TEXT id PK
        TEXT name
        TEXT email
        TEXT username
        TEXT avatar
        TEXT canvas_id
        TEXT discourse_id
        TEXT last_login
        TEXT source_system
        TEXT roles
        TEXT metadata
        TEXT created_at
        TEXT updated_at
    }

    user_profiles {
        TEXT user_id PK,FK
        TEXT bio
        TEXT avatar_url
        TEXT created_at
        TEXT updated_at
    }

    user_preferences {
        TEXT user_id PK,FK
        TEXT preferences
        TEXT created_at
        TEXT updated_at
    }

    user_integration_settings {
        TEXT user_id PK,FK
        TEXT settings
        TEXT created_at
        TEXT updated_at
    }

    courses {
        TEXT id PK
        TEXT title
        TEXT description
        TEXT status
        TEXT created_at
        TEXT updated_at
        TEXT canvas_specific_fields
        TEXT discourse_specific_fields
    }

    assignments {
        TEXT id PK
        TEXT course_id FK
        TEXT title
        TEXT description
        TEXT due_date
        REAL points_possible
        TEXT status
        TEXT created_at
        TEXT updated_at
        TEXT submission_types
        TEXT canvas_specific_fields
        TEXT discourse_specific_fields
    }

    submissions {
        TEXT id PK
        TEXT assignment_id FK
        TEXT user_id FK
        TEXT content
        TEXT attachments
        TEXT status
        REAL score
        TEXT feedback
        TEXT submitted_at
        TEXT graded_at
    }

    discussions {
        TEXT id PK
        TEXT course_id FK
        TEXT title
        TEXT content
        TEXT topic_id
        TEXT status
        TEXT created_at
        TEXT updated_at
    }

    discussion_mappings {
        TEXT id PK
        TEXT canvas_discussion_id
        TEXT discourse_topic_id
        TEXT course_category_id
        TEXT title
        TIMESTAMP last_sync
        BOOLEAN sync_enabled
        BOOLEAN sync_posts
        TIMESTAMP created_at
    }

    course_category_mappings {
        TEXT id PK
        TEXT course_id FK
        TEXT category_id
        BOOLEAN sync_topics
        BOOLEAN sync_assignments
        TEXT created_at
        TEXT updated_at
    }

    modules {
        TEXT id PK
        TEXT course_id FK
        TEXT title
        INTEGER position
        INTEGER items_count
        BOOLEAN publish_final_grade
        BOOLEAN published
        TEXT status
        TEXT created_at
        TEXT updated_at
    }

    module_items {
        TEXT id PK
        TEXT module_id FK
        TEXT title
        INTEGER position
        TEXT item_type
        TEXT content_id
        TEXT content_type
        TEXT url
        TEXT page_url
        BOOLEAN published
        TEXT created_at
        TEXT updated_at
    }

    notifications {
        TEXT id PK
        TEXT user_id FK
        TEXT title
        TEXT message
        TEXT notification_type
        TEXT status
        TEXT reference_id
        TEXT reference_type
        TEXT created_at
        TEXT updated_at
    }

    sync_history {
        TEXT id PK
        TEXT mapping_id
        TEXT sync_type
        TEXT status
        TEXT message
        TEXT details
        TIMESTAMP started_at
        TIMESTAMP completed_at
        TIMESTAMP created_at
    }

    sync_status {
        TEXT id PK
        TEXT entity_type
        TEXT entity_id
        TEXT last_synced_at
        TEXT canvas_version
        TEXT discourse_version
        TEXT sync_status
        TEXT error_message
    }

    users ||--o{ submissions : "has"
    users ||--o{ notifications : "has"
    users ||--|| user_profiles : "has"
    users ||--|| user_preferences : "has"
    users ||--|| user_integration_settings : "has"

    courses ||--o{ assignments : "has"
    courses ||--o{ discussions : "has"
    courses ||--o{ modules : "has"
    courses ||--o{ course_category_mappings : "has"

    assignments ||--o{ submissions : "has"

    modules ||--o{ module_items : "has"
```

## Table Details

This diagram shows the database schema for the Ordo application, including all tables and their relationships. The schema is designed to support both offline-first functionality and integration with Canvas and Discourse.

### Key Tables

- **users**: Central user table that harmonizes user data from Canvas and Discourse
- **courses**: Course information with integration fields for Canvas and Discourse
- **assignments**: Assignment data that can be synchronized with Canvas
- **discussions**: Discussion topics that can be synchronized with Discourse
- **modules**: Course modules for organizing content
- **sync_status** and **sync_history**: Tables for tracking synchronization between systems

### Relationships

The schema includes several types of relationships:
- One-to-one relationships (e.g., users to user_profiles)
- One-to-many relationships (e.g., courses to assignments)
- Many-to-many relationships through mapping tables

### Integration Design

The schema is designed to support seamless integration between:
- Canvas LMS (for courses, assignments, etc.)
- Discourse forums (for discussions)
- Ordo's native offline-first functionality

This allows for a unified experience while maintaining compatibility with existing systems.