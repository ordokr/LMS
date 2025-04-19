# Unified Model Architecture

This document describes the unified model architecture implemented in Phase 2 of the LMS Codebase Cleanup Plan. The architecture consolidates multiple redundant model implementations into a single, consistent set of models and repositories.

## Architecture Overview

The unified model architecture follows these key principles:

1. **Single Source of Truth**: Each entity type has exactly one model implementation
2. **Clear Separation of Concerns**: Models, repositories, and services have distinct responsibilities
3. **Consistent Interfaces**: All repositories implement the same base interface
4. **Extensibility**: Models support extensible metadata for future requirements
5. **Compatibility**: Models can be converted to/from external system formats (Canvas, Discourse)

## Core Components

### Models

The unified model architecture includes the following core models:

1. **User**: Represents a user in the system (student, instructor, admin)
2. **Course**: Represents a course with its properties and relationships
3. **Group**: Represents a group of users with membership management
4. **Assignment**: Represents a course assignment with submission options
5. **Topic**: Represents a discussion topic or forum thread
6. **Submission**: Represents a student submission for an assignment

Each model is designed to accommodate all use cases from the previous redundant implementations while maintaining a clean, consistent interface.

### Repositories

The repository layer provides a consistent interface for data access:

1. **Base Repository Interface**: Defines common CRUD operations for all entity types
2. **Specialized Repository Interfaces**: Extend the base interface with entity-specific operations
3. **SQLite Implementations**: Concrete implementations for SQLite database

The repository pattern ensures consistent data access and error handling across the application.

## Model Relationships

The unified models have the following relationships:

```
User
 ├── Creates/Owns → Courses
 ├── Creates/Owns → Groups
 ├── Creates/Owns → Topics
 ├── Creates → Assignments
 └── Submits → Submissions

Course
 ├── Contains → Assignments
 ├── Contains → Topics
 └── Contains → Groups

Group
 └── Contains → Users (via GroupMembership)

Assignment
 ├── Has → Submissions
 └── May have → Topic (for discussion assignments)

Topic
 └── May be linked to → Assignment

Submission
 └── Belongs to → Assignment
```

## Model Details

### User Model

The `User` model represents a user in the system with the following key properties:

- **id**: Unique identifier (UUID)
- **username**: Unique username
- **name**: Display name
- **email**: Email address
- **avatar_url**: URL to user avatar
- **bio**: User biography
- **roles**: User roles (instructor, student, admin)
- **preferences**: User preferences
- **external_ids**: IDs in external systems (Canvas, Discourse)
- **metadata**: Extensible metadata for future requirements

### Course Model

The `Course` model represents a course with the following key properties:

- **id**: Unique identifier (UUID)
- **title**: Course title
- **description**: Course description
- **code**: Course code
- **status**: Course status (active, archived, deleted)
- **visibility**: Course visibility (public, private, institution)
- **start_date**: Course start date
- **end_date**: Course end date
- **instructor_id**: ID of the primary instructor
- **external_ids**: IDs in external systems (Canvas, Discourse)
- **metadata**: Extensible metadata for future requirements

### Group Model

The `Group` model represents a group of users with the following key properties:

- **id**: Unique identifier (UUID)
- **name**: Group name
- **description**: Group description
- **course_id**: ID of the course the group belongs to
- **leader_id**: ID of the group leader
- **join_level**: Group join level (open, invitation, closed)
- **external_ids**: IDs in external systems (Canvas, Discourse)
- **metadata**: Extensible metadata for future requirements

The `GroupMembership` model represents a user's membership in a group:

- **id**: Unique identifier (UUID)
- **group_id**: ID of the group
- **user_id**: ID of the user
- **status**: Membership status (active, invited, left, removed)
- **role**: Role in the group (leader, member)
- **created_at**: When the membership was created
- **updated_at**: When the membership was last updated

### Assignment Model

The `Assignment` model represents a course assignment with the following key properties:

- **id**: Unique identifier (UUID)
- **title**: Assignment title
- **description**: Assignment description
- **course_id**: ID of the course the assignment belongs to
- **due_date**: Assignment due date
- **points_possible**: Maximum points possible
- **submission_types**: Allowed submission types
- **grading_type**: Grading type (points, percentage, letter, pass_fail)
- **status**: Assignment status (draft, published, archived)
- **external_ids**: IDs in external systems (Canvas, Discourse)
- **metadata**: Extensible metadata for future requirements

### Topic Model

The `Topic` model represents a discussion topic with the following key properties:

- **id**: Unique identifier (UUID)
- **title**: Topic title
- **content**: Topic content/message
- **course_id**: ID of the course the topic belongs to
- **category_id**: ID of the category the topic belongs to
- **author_id**: ID of the topic author
- **status**: Topic status (open, closed, archived, deleted)
- **visibility**: Topic visibility (public, private, course, group)
- **topic_type**: Topic type (regular, question_answer, assignment, announcement)
- **tags**: Topic tags
- **external_ids**: IDs in external systems (Canvas, Discourse)
- **metadata**: Extensible metadata for future requirements

### Submission Model

The `Submission` model represents a student submission for an assignment with the following key properties:

- **id**: Unique identifier (UUID)
- **assignment_id**: ID of the assignment
- **user_id**: ID of the student
- **submission_type**: Type of submission (text, url, file, etc.)
- **content**: Submission content
- **status**: Submission status (not_submitted, draft, submitted, graded, etc.)
- **grade**: Assigned grade
- **score**: Numeric score
- **comments**: Submission comments
- **external_ids**: IDs in external systems (Canvas, Discourse)
- **metadata**: Extensible metadata for future requirements

## Repository Interfaces

### Base Repository Interface

The base repository interface (`Repository<T, ID>`) defines common CRUD operations for all entity types:

```rust
#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: &ID) -> Result<Option<T>, Error>;
    async fn find_all(&self) -> Result<Vec<T>, Error>;
    async fn create(&self, entity: &T) -> Result<T, Error>;
    async fn update(&self, entity: &T) -> Result<T, Error>;
    async fn delete(&self, id: &ID) -> Result<(), Error>;
    async fn count(&self) -> Result<i64, Error>;
}
```

### Specialized Repository Interfaces

Each entity type has a specialized repository interface that extends the base interface with entity-specific operations:

- **UserRepository**: User-specific operations (find by username, email, etc.)
- **CourseRepository**: Course-specific operations (find by title, instructor, etc.)
- **GroupRepository**: Group-specific operations (find by name, course, etc.)
- **AssignmentRepository**: Assignment-specific operations (find by course, due date, etc.)
- **TopicRepository**: Topic-specific operations (find by course, author, etc.)
- **SubmissionRepository**: Submission-specific operations (find by assignment, user, etc.)

## External System Integration

The unified models can be converted to and from external system formats (Canvas, Discourse) using the following methods:

- **from_canvas_X**: Convert from Canvas format to unified model
- **to_canvas_X**: Convert from unified model to Canvas format
- **from_discourse_X**: Convert from Discourse format to unified model
- **to_discourse_X**: Convert from unified model to Discourse format

This allows seamless integration with external systems while maintaining a consistent internal model.

## Error Handling

The unified model architecture uses a consistent error handling approach:

- **Error Type**: A single error type (`Error`) is used throughout the repository layer
- **Error Categories**: Errors are categorized (not found, validation, database, etc.)
- **Error Context**: Errors include context information for debugging
- **Error Propagation**: Errors are propagated using the `?` operator

## Testing

The unified model architecture includes comprehensive tests:

- **Unit Tests**: Each model and repository has unit tests
- **Integration Tests**: Tests for model relationships and repository interactions
- **Conversion Tests**: Tests for external system format conversions

## Migration Path

To migrate from the old models to the unified models:

1. Replace imports from old model files with imports from the unified models
2. Use the appropriate repository implementation for data access
3. Use the conversion methods for external system integration

## Conclusion

The unified model architecture provides a solid foundation for the LMS application by eliminating redundancy, ensuring consistency, and supporting all required functionality. It serves as a single source of truth for all entity types and provides a clear path for future development.
