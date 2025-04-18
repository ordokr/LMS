# Model and Controller Migration Candidates

_Last updated: 2025-04-18_

This document provides a comprehensive list of models and controllers from Canvas LMS and Discourse that are candidates for migration to the Ordo platform. It serves as a reference for developers implementing the migration plan outlined in [MIGRATION_PLAN.md](MIGRATION_PLAN.md).

## Core Models

These models represent the fundamental data structures that exist in both Canvas and Discourse and should be prioritized for migration.

| Model | Canvas | Discourse | Priority | Notes |
|-------|--------|-----------|----------|-------|
| User | ✅ | ✅ | High | Core user model with authentication, profiles, and preferences |
| Group | ✅ | ✅ | High | User grouping functionality in both systems |
| Course/Category | ✅ | ✅ | High | Canvas has Courses, Discourse has Categories - similar organizational units |
| Assignment/Topic | ✅ | ✅ | High | Canvas Assignments map to Discourse Topics for content organization |
| Submission/Post | ✅ | ✅ | High | Canvas Submissions map to Discourse Posts for user-generated content |
| File/Upload | ✅ | ✅ | High | File attachment functionality in both systems |
| Notification | ✅ | ✅ | High | User notification system in both platforms |
| Tag | ✅ | ✅ | Medium | Content tagging functionality |
| Bookmark | ✅ | ✅ | Medium | User bookmarking functionality |
| ContentShare | ✅ | ✅ | Medium | Sharing content between users |
| CustomField | ✅ | ✅ | Medium | Extensible metadata for various models |
| Calendar/Event | ✅ | ✅ | Medium | Time-based scheduling functionality |
| Theme | ✅ | ✅ | Medium | UI customization capabilities |
| Badge | ✅ | ✅ | Medium | Achievement/gamification system |
| UserProfile | ✅ | ✅ | Medium | Extended user information |
| Enrollment/Membership | ✅ | ✅ | High | User participation in courses/categories |

## Core Controllers

These controllers handle the core functionality that exists in both Canvas and Discourse and should be prioritized for migration.

| Controller | Canvas | Discourse | Priority | Notes |
|------------|--------|-----------|----------|-------|
| UsersController | ✅ | ✅ | High | User management and profiles |
| GroupsController | ✅ | ✅ | High | Group management |
| CoursesController/CategoriesController | ✅ | ✅ | High | Course/Category management |
| AssignmentsController/TopicsController | ✅ | ✅ | High | Assignment/Topic management |
| SubmissionsController/PostsController | ✅ | ✅ | High | Submission/Post management |
| FilesController/UploadsController | ✅ | ✅ | High | File management |
| NotificationsController | ✅ | ✅ | High | Notification management |
| TagsController | ✅ | ✅ | Medium | Tag management |
| BookmarksController | ✅ | ✅ | Medium | Bookmark management |
| ContentSharesController | ✅ | ✅ | Medium | Content sharing functionality |
| CalendarEventsController | ✅ | ✅ | Medium | Calendar management |
| ThemesController | ✅ | ✅ | Medium | Theme management |
| BadgesController | ✅ | ✅ | Medium | Badge management |
| SearchController | ✅ | ✅ | Medium | Search functionality |
| EnrollmentsController/MembershipsController | ✅ | ✅ | High | Enrollment/Membership management |

## Canvas-Specific Models Worth Migrating

These models are unique to Canvas but provide valuable functionality that should be incorporated into the Ordo platform.

| Model | Priority | Notes |
|-------|----------|-------|
| Quiz | High | Assessment functionality |
| Rubric | High | Grading criteria |
| GradingStandard | High | Grading scales |
| OutcomeGroup | Medium | Learning outcomes |
| ContextModule | Medium | Content organization |
| Eportfolio | Medium | Student portfolios |
| GradebookColumn | Medium | Gradebook customization |
| ConversationMessage | Medium | Messaging system |
| Progress | Medium | Progress tracking |
| PlannerNote | Medium | Student planning |
| MediaObject | Medium | Media embedding |
| WebConference | Medium | Virtual meetings |
| WikiPage | Medium | Collaborative content |
| ExternalTool | Medium | LTI integrations |

## Discourse-Specific Models Worth Migrating

These models are unique to Discourse but provide valuable functionality that should be incorporated into the Ordo platform.

| Model | Priority | Notes |
|-------|----------|-------|
| TopicList | High | Topic organization and display |
| PostAction | High | Post moderation actions |
| UserAction | Medium | User activity tracking |
| FormTemplate | Medium | Structured content creation |
| SidebarSection | Medium | UI customization |
| WebHook | Medium | Integration capabilities |
| TopicTimer | Medium | Scheduled topic actions |
| DirectoryItem | Medium | User directory functionality |
| TopicPosters | Medium | Topic participation tracking |
| TopicTag | Medium | Topic categorization |
| UserOption | Medium | User preferences |
| UserProfile | Medium | Extended user information |
| WebHookEvent | Medium | Integration event handling |
| WatchedWord | Medium | Content moderation |

## Integration Considerations

When implementing these models and controllers, consider the following integration points:

1. **Authentication Systems**:
   - Both systems have complex authentication mechanisms
   - Prioritize migrating core authentication models and controllers
   - Implement a unified authentication system that supports multiple providers

2. **Content Organization**:
   - Map Canvas courses to Discourse categories
   - Map Canvas assignments to Discourse topics
   - Map Canvas submissions to Discourse posts
   - Create a unified content organization system

3. **User Relationships**:
   - Canvas has teacher/student relationships
   - Discourse has moderator/regular user relationships
   - Create a unified permission system that accommodates both hierarchies

4. **File Management**:
   - Both systems have robust file attachment systems
   - Unify the upload and storage mechanisms
   - Implement consistent file permissions across contexts

5. **Notification Systems**:
   - Both have comprehensive notification systems
   - Create a unified notification architecture
   - Support multiple notification channels (in-app, email, push)

## Implementation Strategy

For each model and controller to be migrated, follow these steps:

1. **Analysis**: Review the original implementation in Canvas/Discourse
2. **Mapping**: Create a mapping of fields and relationships to the new system
3. **Language Selection**: Determine whether to implement in Rust or Haskell based on the [Haskell integration guidelines](../architecture/haskell_integration.md)
4. **Implementation**: Develop the model and controller in the appropriate language
5. **Testing**: Create comprehensive tests for the new implementation
6. **Integration**: Integrate with other components of the system
7. **Documentation**: Update documentation to reflect the new implementation

## Migration Phases

Align the migration of these models and controllers with the phases outlined in [MIGRATION_PLAN.md](MIGRATION_PLAN.md):

1. **Phase 1**: Core User and Authentication System
   - User, UserProfile, Authentication models and controllers

2. **Phase 2**: Core Course and Category Management
   - Course/Category, Group, Enrollment/Membership models and controllers

3. **Phase 3**: Content and Discussion
   - Assignment/Topic, Submission/Post, File/Upload models and controllers

4. **Phase 4**: Canvas Module and Assignment Integration
   - Quiz, Rubric, GradingStandard, OutcomeGroup models and controllers

5. **Phase 5**: Advanced Features and Integrations
   - Remaining models and controllers based on priority

## Progress Tracking

Track the migration progress of each model and controller in the project management system, with the following statuses:

- **Not Started**: Migration has not begun
- **In Analysis**: Analyzing the original implementation
- **In Development**: Implementing the model/controller
- **In Testing**: Testing the implementation
- **Completed**: Migration completed and integrated

## References

- [Canvas Models Documentation](https://canvas.instructure.com/doc/api/file.object_ids.html)
- [Discourse Models Documentation](https://docs.discourse.org/)
- [MIGRATION_PLAN.md](MIGRATION_PLAN.md)
- [INTEGRATION_PLAN.md](INTEGRATION_PLAN.md)
