# System Architecture

_Generated on: 2025-04-06_

## Technology Stack

### Database Solution (Hardcoded)

- **Engine**: SQLite
- **Driver**: sqlx (Rust crate)
- **Configuration**: Embedded, file-based
- **Path**: ./src-tauri/educonnect.db
- **Migrations**: sqlx built-in migrations

SQLite with sqlx is hardcoded as the database solution for this project. This combination provides:

- **Offline-First Architecture**: Local database enabling full offline functionality
- **Zero Configuration**: No separate database server setup required
- **Cross-Platform**: Works consistently on all supported platforms
- **Type Safety**: Through sqlx compile-time SQL checking

### Frontend

- No frontend technologies detected

### Backend

- **Rust**: Tauri backend (with sqlx for database access)

### Infrastructure

- No infrastructure technologies detected

### Testing

- **Jest**: v^29.7.0
- **Mocha**: v^10.2.0
- **Chai**: v^4.3.8

## System Components

### Model Layer

The system contains 46 models with 42 implemented (91%).

#### Key Models

- **User** (60% complete)
- **Category** (60% complete)
- **DiscussionMapping** (60% complete)
- **Post** (60% complete)
- **Topic** (60% complete)

### API Layer

The system exposes 59 API endpoints with 7 implemented (12%).

### UI Layer

The system has 99 UI components with 89 implemented (90%).

#### UI Component Types

- **Component**: 99 components

## Architecture Diagram

```mermaid
flowchart TB
    User((User)) --> FE[Frontend Layer]
    FE --> API[API Layer]
    API --> DB[(SQLite/sqlx)]
    FE --> |includes| UI[UI Components]
    UI --> |contains| FE_COMP[Component]
    API --> |uses| MODELS[Models]
    MODELS --> |includes| MODEL_LIST[User, Category, DiscussionMapping, Post, Topic]
    DB --> |type-safe| SQL[SQL Queries]
    DB --> |managed by| MIGRATIONS[Migrations]
```

## Entity Relationships

```mermaid
erDiagram
    Course ||--|| Module : relates
    Course ||--|| Assignment : relates
    Course ||--|| Submission : relates
    Course ||--|| Enrollment : relates
    Course ||--|| CourseStatus : relates
    Course ||--|| EnrollmentRole : relates
    Module ||--|| Course : relates
    Module ||--|| Assignment : relates
    Module ||--|| Submission : relates
    Module ||--|| Enrollment : relates
    Module ||--|| CourseStatus : relates
    Module ||--|| EnrollmentRole : relates
    Assignment ||--|| Course : relates
    Assignment ||--|| Module : relates
    Assignment ||--|| Submission : relates
    Assignment ||--|| Enrollment : relates
    Assignment ||--|| CourseStatus : relates
    Assignment ||--|| EnrollmentRole : relates
    Submission ||--|| Course : relates
    Submission ||--|| Module : relates
    Submission ||--|| Assignment : relates
    Submission ||--|| Enrollment : relates
    Submission ||--|| CourseStatus : relates
    Submission ||--|| EnrollmentRole : relates
    Enrollment ||--|| Course : relates
    Enrollment ||--|| Module : relates
    Enrollment ||--|| Assignment : relates
    Enrollment ||--|| Submission : relates
    Enrollment ||--|| CourseStatus : relates
    Enrollment ||--|| EnrollmentRole : relates
    CourseStatus ||--|| Course : relates
    CourseStatus ||--|| Module : relates
    CourseStatus ||--|| Assignment : relates
    CourseStatus ||--|| Submission : relates
    CourseStatus ||--|| Enrollment : relates
    CourseStatus ||--|| EnrollmentRole : relates
    EnrollmentRole ||--|| Course : relates
    EnrollmentRole ||--|| Module : relates
    EnrollmentRole ||--|| Assignment : relates
    EnrollmentRole ||--|| Submission : relates
    EnrollmentRole ||--|| Enrollment : relates
    EnrollmentRole ||--|| CourseStatus : relates
    ForumCategory ||--|| ForumTopic : relates
    ForumCategory ||--|| ForumPost : relates
    ForumCategory ||--|| ForumUserPreferences : relates
    ForumCategory ||--|| ForumTrustLevel : relates
    ForumCategory ||--|| Category : relates
    ForumCategory ||--o{ Post : relates
    ForumCategory ||--|| Topic : relates
    ForumTopic ||--|| ForumCategory : relates
    ForumTopic ||--|| ForumPost : relates
    ForumTopic ||--|| ForumUserPreferences : relates
    ForumTopic ||--|| ForumTrustLevel : relates
    ForumTopic ||--|| Category : relates
    ForumTopic ||--o{ Post : relates
    ForumTopic ||--|| Topic : relates
    ForumPost ||--|| ForumCategory : relates
    ForumPost ||--|| ForumTopic : relates
    ForumPost ||--|| ForumUserPreferences : relates
    ForumPost ||--|| ForumTrustLevel : relates
    ForumPost ||--|| Category : relates
    ForumPost ||--o{ Post : relates
    ForumPost ||--|| Topic : relates
    ForumUserPreferences ||--|| ForumCategory : relates
    ForumUserPreferences ||--|| ForumTopic : relates
    ForumUserPreferences ||--|| ForumPost : relates
    ForumUserPreferences ||--|| ForumTrustLevel : relates
    ForumUserPreferences ||--|| Category : relates
    ForumUserPreferences ||--o{ Post : relates
    ForumUserPreferences ||--|| Topic : relates
    ForumTrustLevel ||--|| ForumCategory : relates
    ForumTrustLevel ||--|| ForumTopic : relates
    ForumTrustLevel ||--|| ForumPost : relates
    ForumTrustLevel ||--|| ForumUserPreferences : relates
    ForumTrustLevel ||--|| Category : relates
    ForumTrustLevel ||--o{ Post : relates
    ForumTrustLevel ||--|| Topic : relates
    User ||--o{ UserRole : relates
    User ||--|| UserProfile : relates
    User ||--|| LoginRequest : relates
    User ||--|| RegisterRequest : relates
    User ||--|| AuthResponse : relates
    UserRole ||--|| User : relates
    UserRole ||--|| UserProfile : relates
    UserRole ||--|| LoginRequest : relates
    UserRole ||--|| RegisterRequest : relates
    UserRole ||--|| AuthResponse : relates
    UserProfile ||--|| User : relates
    UserProfile ||--o{ UserRole : relates
    UserProfile ||--|| LoginRequest : relates
    UserProfile ||--|| RegisterRequest : relates
    UserProfile ||--|| AuthResponse : relates
    LoginRequest ||--|| User : relates
    LoginRequest ||--o{ UserRole : relates
    LoginRequest ||--|| UserProfile : relates
    LoginRequest ||--|| RegisterRequest : relates
    LoginRequest ||--|| AuthResponse : relates
    RegisterRequest ||--|| User : relates
    RegisterRequest ||--o{ UserRole : relates
    RegisterRequest ||--|| UserProfile : relates
    RegisterRequest ||--|| LoginRequest : relates
    RegisterRequest ||--|| AuthResponse : relates
    AuthResponse ||--|| User : relates
    AuthResponse ||--o{ UserRole : relates
    AuthResponse ||--|| UserProfile : relates
    AuthResponse ||--|| LoginRequest : relates
    AuthResponse ||--|| RegisterRequest : relates
    DiscussionMapping ||--|| CanvasDiscussionEntry : relates
    DiscussionMapping ||--|| DiscourseTopic : relates
    DiscussionMapping ||--|| DiscoursePost : relates
    DiscussionMapping ||--|| SyncResult : relates
    DiscussionMapping ||--o{ Post : relates
    DiscussionMapping ||--|| Topic : relates
    CanvasDiscussionEntry ||--|| DiscussionMapping : relates
    CanvasDiscussionEntry ||--|| DiscourseTopic : relates
    CanvasDiscussionEntry ||--|| DiscoursePost : relates
    CanvasDiscussionEntry ||--|| SyncResult : relates
    CanvasDiscussionEntry ||--o{ Post : relates
    CanvasDiscussionEntry ||--|| Topic : relates
    DiscourseTopic ||--|| DiscussionMapping : relates
    DiscourseTopic ||--|| CanvasDiscussionEntry : relates
    DiscourseTopic ||--|| DiscoursePost : relates
    DiscourseTopic ||--|| SyncResult : relates
    DiscourseTopic ||--o{ Post : relates
    DiscourseTopic ||--|| Topic : relates
    DiscoursePost ||--|| DiscussionMapping : relates
    DiscoursePost ||--|| CanvasDiscussionEntry : relates
    DiscoursePost ||--|| DiscourseTopic : relates
    DiscoursePost ||--|| SyncResult : relates
    DiscoursePost ||--o{ Post : relates
    DiscoursePost ||--|| Topic : relates
    SyncResult ||--|| DiscussionMapping : relates
    SyncResult ||--|| CanvasDiscussionEntry : relates
    SyncResult ||--|| DiscourseTopic : relates
    SyncResult ||--|| DiscoursePost : relates
    SyncResult ||--o{ Post : relates
    SyncResult ||--|| Topic : relates
    CourseCategory ||--|| Category : relates
    CourseCategory ||--|| CourseCategoryCreate : relates
    CourseCategory ||--|| CourseCategoryUpdate : relates
    CourseCategoryCreate ||--|| Category : relates
    CourseCategoryCreate ||--|| CourseCategory : relates
    CourseCategoryCreate ||--|| CourseCategoryUpdate : relates
    CourseCategoryUpdate ||--|| Category : relates
    CourseCategoryUpdate ||--|| CourseCategory : relates
    CourseCategoryUpdate ||--|| CourseCategoryCreate : relates
```

