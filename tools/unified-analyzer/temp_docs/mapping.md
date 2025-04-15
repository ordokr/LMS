# Canvas and Discourse Feature Mapping

This document maps features from Canvas LMS and Discourse to our Rust implementation, tracking progress and architectural decisions.

## Table of Contents
1. [Core Models](#core-models)
2. [User & Authentication](#user--authentication)
3. [Course Components](#course-components)
4. [Forum Components](#forum-components)
5. [Integration Points](#integration-points)
6. [API Endpoints](#api-endpoints)
7. [UI Components](#ui-components)

---

## Core Models

### User Model

| Canvas/Discourse Feature | Our Implementation | Status | Notes |
|--------------------------|-------------------|--------|-------|
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `native-modules` | `models/module.rs:Module` | 0% | Auto-generated with 1 fields, 0 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `native-modules` | `models/module.rs:Module` | 0% | Auto-generated with 1 fields, 0 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `native-modules` | `models/module.rs:Module` | 0% | Auto-generated with 1 fields, 0 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `for` | `models/module.rs:Module` | 0% | Auto-generated with 4 fields, 6 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `for` | `models/module.rs:Module` | 0% | Auto-generated with 4 fields, 6 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `for` | `models/module.rs:Module` | 0% | Auto-generated with 4 fields, 6 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `for` | `models/module.rs:Module` | 0% | Auto-generated with 4 fields, 6 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `for` | `models/module.rs:Module` | 0% | Auto-generated with 4 fields, 6 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `for` | `models/module.rs:Module` | 0% | Auto-generated with 4 fields, 6 methods |
| Canvas `util` | `models/group.rs:Group` | 0% | Auto-generated with 34 fields, 15 methods |
| Canvas `upgradeinsecurerequests` | `models/grade.rs:Grade` | 0% | Auto-generated with 20 fields, 0 methods |
| Canvas `for` | `models/module.rs:Module` | 0% | Auto-generated with 4 fields, 6 methods |
| Unified `UserModel` | `models/user_model.rs:UserModel` | 0% | Auto-generated with 27 fields, 5 methods |
| Canvas `user_defined_metadata` | `models/user.rs:User` | 0% | Auto-generated with 6 fields, 0 methods |
| Canvas `User` | `models/auth.rs:User` | 80% | Missing preferences |
| Canvas `UserProfile` | `models/auth.rs:UserProfile` | 60% | Simplified fields |
| Discourse `User` | `models/auth.rs:User` | 70% | Need trust levels |
| Discourse `UserCustomFields` | `models/auth.rs:UserMetadata` | 50% | Basic implementation |

### Course Model

| Canvas Feature | Our Implementation | Status | Notes |
|----------------|-------------------|--------|-------|
| Canvas `Course` | `models/lms.rs:Course` | 75% | Missing workflow states |
| Canvas `CourseSection` | `models/lms.rs:CourseSection` | 60% | Basic structure only |
| Canvas `Enrollment` | `models/lms.rs:Enrollment` | 70% | Missing custom roles |
| Canvas `EnrollmentTerm` | `models/lms.rs:Term` | 50% | Basic implementation |

### Forum Model

| Discourse Feature | Our Implementation | Status | Notes |
|-------------------|-------------------|--------|-------|
| Discourse `Category` | `models/forum.rs:Category` | 80% | Missing permissions |
| Discourse `Topic` | `models/forum.rs:Topic` | 75% | Missing bookmarks |
| Discourse `Post` | `models/forum.rs:Post` | 80% | Missing revisions |
| Discourse `Tag` | `models/forum.rs:Tag` | 60% | Basic implementation |

---

## User & Authentication

### Authentication

| Original Feature | Our Implementation | Status | Notes |
|------------------|-------------------|--------|-------|
| Canvas `AuthProvider` | `services/auth_service.rs:AuthProvider` | 70% | LDAP missing |
| Canvas `PseudonymsController` | `services/auth_service.rs:login_methods` | 60% | Basic auth only |
| Discourse `UserAuthenticator` | `services/auth_service.rs:authenticate` | 65% | Basic implementation |
| Discourse `UserAuthToken` | `models/auth.rs:AuthToken` | 80% | JWT-based implementation |

### User Roles & Permissions

| Original Feature | Our Implementation | Status | Notes |
|------------------|-------------------|--------|-------|
| Canvas `Role` | `models/auth.rs:Role` | 70% | Missing inheritance |
| Canvas `Permission` | `models/auth.rs:Permission` | 60% | Simplified model |
| Discourse `TrustLevel` | `models/forum.rs:TrustLevel` | 75% | Basic implementation |
| Discourse `GroupUser` | `models/auth.rs:UserGroup` | 50% | Minimal implementation |

---

## Course Components

### Assignments

| Canvas Feature | Our Implementation | Status | Notes |
|----------------|-------------------|--------|-------|
| Canvas `Assignment` | `models/lms.rs:Assignment` | 80% | Added core fields and methods from Canvas model |
| Canvas `Submission` | `models/lms.rs:Submission` | 80% | Added workflow_state and other key fields |
| Canvas `Rubric` | `models/lms.rs:Rubric` | 40% | Basic structure only |
| Canvas `AssignmentGroup` | `models/lms.rs:AssignmentGroup` | 75% | Added group rules from Canvas |

### Content

| Canvas Feature | Our Implementation | Status | Notes |
|----------------|-------------------|--------|-------|
| Canvas `ContentTag` | `models/lms.rs:ContentItem` | 60% | Simplified model |
| Canvas `ContextModule` | `models/lms.rs:Module` | 75% | Missing prerequisites |
| Canvas `WikiPage` | `models/lms.rs:Page` | 70% | Missing revisions |
| Canvas `Attachment` | `models/lms.rs:Attachment` | 60% | Basic implementation |

### Gradebook

| Canvas Feature | Our Implementation | Status | Notes |
|----------------|-------------------|--------|-------|
| Canvas `GradingPeriod` | `models/lms.rs:GradingPeriod` | 50% | Basic implementation |
| Canvas `GradingStandard` | `models/lms.rs:GradingStandard` | 40% | Basic structure |
| Canvas `SubmissionComment` | `models/lms.rs:SubmissionFeedback` | 60% | Missing attachments |
| Canvas `Score` | `models/lms.rs:Score` | 70% | Basic implementation |

---

## Forum Components

### Categories & Topics

| Discourse Feature | Our Implementation | Status | Notes |
|-------------------|-------------------|--------|-------|
| Discourse `Category` | `models/forum.rs:Category` | 80% | Missing custom fields |
| Discourse `Topic` | `models/forum.rs:Topic` | 75% | Missing pinned status |
| Discourse `TopicList` | `services/forum_service.rs:get_topics` | 70% | Basic filtering |
| Discourse `CategoryGroup` | Not implemented | 0% | Low priority |

### Posts & Discussions

| Discourse Feature | Our Implementation | Status | Notes |
|-------------------|-------------------|--------|-------|
| Discourse `Post` | `models/forum.rs:Post` | 80% | Missing history |
| Discourse `PostAction` | `models/forum.rs:Reaction` | 60% | Like/flag only |
| Discourse `PostReply` | `models/forum.rs:Post` | 70% | Parent-child relationship |
| Discourse `Draft` | `models/forum.rs:Draft` | 40% | Basic implementation |

### Trust & Moderation

| Discourse Feature | Our Implementation | Status | Notes |
|-------------------|-------------------|--------|-------|
| Discourse `TrustLevel` | `models/forum.rs:TrustLevel` | 70% | Basic levels |
| Discourse `UserHistory` | Not implemented | 0% | Planned |
| Discourse `Flag` | `models/forum.rs:Flag` | 50% | Basic implementation |
| Discourse `FlagType` | `models/forum.rs:FlagType` | 60% | Limited types |

---

## Integration Points

### LMS-Forum Connections

| Integration Feature | Our Implementation | Status | Notes |
|---------------------|-------------------|--------|-------|
| Course → Category Mapping | `services/integration_service.rs:ensure_course_category` | 80% | Working implementation |
| Assignment → Topic Mapping | `services/integration_service.rs:create_assignment_topic` | 70% | Basic functionality |
| Gradable Discussions | `services/integration_service.rs:grade_forum_participation` | 40% | Basic structure |
| Module Items → Forum Posts | `services/integration_service.rs:link_module_item_to_forum` | 50% | One-way linking |

### Shared Components

| Integration Feature | Our Implementation | Status | Notes |
|---------------------|-------------------|--------|-------|
| Unified Notifications | `models/notification.rs` | 60% | Basic implementation |
| Shared Activities | `models/activity.rs` | 50% | Simplified model |
| Content References | `models/content_ref.rs` | 65% | Basic implementation |
| Search Integration | `services/search_service.rs` | 40% | Limited implementation |

---

## API Endpoints

### LMS API Endpoints

| Canvas Endpoint | Our Implementation | Status | Notes |
|-----------------|-------------------|--------|-------|
| `GET /api/v1/courses` | `api/lms.rs:get_courses` | 80% | Missing filtering |
| `GET /api/v1/courses/:id` | `api/lms.rs:get_course` | 85% | Working endpoint |
| `GET /api/v1/courses/:id/assignments` | `api/lms.rs:get_assignments` | 75% | Basic endpoint |
| `POST /api/v1/courses/:id/assignments` | `api/lms.rs:create_assignment` | 70% | Missing validations |

### Forum API Endpoints

| Discourse Endpoint | Our Implementation | Status | Notes |
|-------------------|-------------------|--------|-------|
| `GET /categories.json` | `api/forum.rs:get_categories` | 80% | Working endpoint |
| `GET /c/:slug/:id.json` | `api/forum.rs:get_category` | 75% | Working endpoint |
| `GET /t/:slug/:id.json` | `api/forum.rs:get_topic` | 85% | Working endpoint |
| `POST /posts.json` | `api/forum.rs:create_post` | 80% | Working endpoint |

### Integration API Endpoints

| Integration Endpoint | Our Implementation | Status | Notes |
|---------------------|-------------------|--------|-------|
| `GET /api/v1/integration/course/:id/forum` | `api/integration.rs:get_course_forum` | 70% | Working endpoint |
| `GET /api/v1/integration/topic/:id/course_references` | `api/integration.rs:get_topic_refs` | 60% | Basic implementation |
| `POST /api/v1/sync` | `api/sync.rs:sync_data` | 65% | Basic sync endpoint |
| `GET /api/v1/integration/activity` | `api/integration.rs:get_activity` | 50% | Limited implementation |

---

## UI Components

### LMS UI Components

| UI Feature | Our Implementation | Status | Notes |
|------------|-------------------|--------|-------|
| Course Dashboard | `components/lms/course_dashboard.rs` | 75% | Missing activity feed |
| Assignment View | `components/lms/assignment_detail.rs` | 70% | Basic functionality |
| Gradebook | `components/lms/gradebook.rs` | 60% | Basic implementation |
| Module Navigator | `components/lms/module_navigator.rs` | 65% | Basic implementation |

### Forum UI Components

| UI Feature | Our Implementation | Status | Notes |
|------------|-------------------|--------|-------|
| Category Index | `components/forum/categories.rs` | 80% | Working component |
| Topic List | `components/forum/topic_list.rs` | 75% | Basic functionality |
| Topic View | `components/forum/thread_detail.rs` | 70% | Missing some features |
| Post Editor | `components/forum/post_editor.rs` | 65% | Basic editor |

### Shared UI Components

| UI Feature | Our Implementation | Status | Notes |
|------------|-------------------|--------|-------|
| User Profile | `components/auth/profile.rs` | 60% | Basic profile |
| Notification Center | `components/shared/notifications.rs` | 55% | Basic notifications |
| Activity Stream | `components/shared/activity_stream.rs` | 50% | Limited implementation |
| Offline Indicator | `components/shared/offline_indicator.rs` | 75% | Working component |

---

## Implementation Priorities

### High Priority
1. ⭐️ Complete core auth system integration
2. ⭐️ Finish course category mapping
3. ⭐️ Implement assignment discussion integration
4. ⭐️ Complete sync engine for critical components

### Medium Priority
1. Complete gradebook functionality
2. Enhance forum moderation tools
3. Add cross-referencing between content types
4. Implement unified activity stream

### Low Priority
1. Advanced forum features (badges, polls)
2. LMS outcomes and competencies
3. Advanced analytics and reporting
4. Import/export functionality

---

## File Location Map

### Canvas → Our Implementation

| Canvas File | Our Implementation |
|-------------|-------------------|
| `app/models/user.rb` | `src/models/auth.rs` |
| `app/models/course.rb` | `src/models/lms.rs` |
| `app/models/assignment.rb` | `src/models/lms.rs` |
| `app/controllers/courses_controller.rb` | `src/services/lms_service.rs` |

### Discourse → Our Implementation

| Discourse File | Our Implementation |
|----------------|-------------------|
| `app/models/category.rb` | `src/models/forum.rs` |
| `app/models/topic.rb` | `src/models/forum.rs` |
| `app/models/post.rb` | `src/models/forum.rs` |
| `app/controllers/categories_controller.rb` | `src/services/forum_service.rs` |

---

## Tracking Notes

This section documents key decisions and adaptations made during the port:

1. **Authentication:** We unified Canvas and Discourse auth systems into a single JWT-based system
2. **Data Models:** Simplified many models while preserving core functionality
3. **Integration:** Added new integration layer not present in either original system
4. **Offline Support:** Added CRDT-based sync not present in either original system

## Helpful Resources

- [Canvas LMS API Documentation](https://canvas.instructure.com/doc/api/)
- [Discourse API Documentation](https://docs.discourse.org/)
- [Canvas Data Dictionary](https://canvas.instructure.com/doc/api/file.data_dictionary.html)
- [Discourse Developer Guide](https://meta.discourse.org/c/development/41)