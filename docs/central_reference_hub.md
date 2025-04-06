# LMS Project Central Reference Hub

_Generated on: 2025-04-06_

## Project Overview

| Component | Completion | Status |
|-----------|------------|--------|
| Models | 89% | ✅ |
| API Endpoints | 0% | ❌ |
| UI Components | 90% | ✅ |
| Test Coverage | 15% | ❌ |

**Overall Phase:** mid_development

## Model Reference

| Model | Properties | Completeness | Implementation |
|-------|------------|-------------|----------------|
| Assignment | 0 | 50% | [View Code](shared\models\course.rs) |
| Assignment | 0 | 50% | [View Code](src-tauri\src\models\course.rs) |
| AuthResponse | 0 | 39% | [View Code](shared\models\user.rs) |
| AuthResponse | 0 | 39% | [View Code](shared\src\models\user.rs) |
| Category | 0 | 60% | [View Code](src-tauri\src\models\category.rs) |
| Course | 0 | 55% | [View Code](shared\models\course.rs) |
| Course | 0 | 47% | [View Code](shared\src\models\course.rs) |
| Course | 0 | 50% | [View Code](src-tauri\src\models\course.rs) |
| Course | 0 | 46% | [View Code](src-tauri\src\shared\models\course.rs) |
| CourseStatus | 0 | 32% | [View Code](shared\models\course.rs) |
| CourseStatus | 0 | 32% | [View Code](src-tauri\src\models\course.rs) |
| Enrollment | 0 | 47% | [View Code](shared\models\course.rs) |
| EnrollmentRole | 0 | 32% | [View Code](shared\models\course.rs) |
| ForumCategory | 0 | 50% | [View Code](shared\models\forum.rs) |
| ForumCategory | 0 | 43% | [View Code](shared\src\models\forum.rs) |
| ForumPost | 0 | 50% | [View Code](shared\models\forum.rs) |
| ForumPost | 0 | 45% | [View Code](shared\src\models\forum.rs) |
| ForumTopic | 0 | 50% | [View Code](shared\models\forum.rs) |
| ForumTopic | 0 | 45% | [View Code](shared\src\models\forum.rs) |
| ForumTrustLevel | 0 | 47% | [View Code](shared\models\forum.rs) |
| ForumUserPreferences | 0 | 49% | [View Code](shared\models\forum.rs) |
| LoginRequest | 0 | 39% | [View Code](shared\models\user.rs) |
| LoginRequest | 0 | 39% | [View Code](shared\src\models\user.rs) |
| Module | 0 | 50% | [View Code](shared\models\course.rs) |
| Module | 0 | 50% | [View Code](src-tauri\src\models\course.rs) |
| Post | 0 | 60% | [View Code](src-tauri\src\models\post.rs) |
| RegisterRequest | 0 | 41% | [View Code](shared\models\user.rs) |
| RegisterRequest | 0 | 43% | [View Code](shared\src\models\user.rs) |
| Submission | 0 | 49% | [View Code](shared\models\course.rs) |
| Submission | 0 | 49% | [View Code](src-tauri\src\models\course.rs) |
| Tag | 0 | 57% | [View Code](src-tauri\src\models\tag.rs) |
| Topic | 0 | 60% | [View Code](src-tauri\src\models\topic.rs) |
| User | 0 | 60% | [View Code](shared\models\user.rs) |
| User | 0 | 45% | [View Code](shared\src\models\user.rs) |
| User | 0 | 60% | [View Code](src-tauri\src\models\user.rs) |
| UserProfile | 0 | 41% | [View Code](shared\models\user.rs) |
| UserRole | 0 | 45% | [View Code](shared\models\user.rs) |
| UserRole | 0 | 32% | [View Code](src-tauri\src\models\user.rs) |

## API Endpoint Reference

| Endpoint | Route | HTTP Method | Completeness | Implementation |
|----------|-------|------------|-------------|----------------|
| get(get_courses | N/A | GET | 20% | [View Code](src-tauri\src\api\courses.rs) |
| get(get_course | N/A | GET | 20% | [View Code](src-tauri\src\api\courses.rs) |
| post(create_course | N/A | GET | 20% | [View Code](src-tauri\src\api\courses.rs) |
| put(update_course | N/A | GET | 20% | [View Code](src-tauri\src\api\courses.rs) |
| delete(delete_course | N/A | GET | 20% | [View Code](src-tauri\src\api\courses.rs) |
| update_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| create_post | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| get_recent_posts | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| chainLength | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| get(get_categories | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| get(get_categories_by_course | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| get(get_topics | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| post(create_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| get(get_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| put(update_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| delete(delete_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| get(get_posts_by_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| post(create_post | N/A | GET | 20% | [View Code](src-tauri\src\api\forum.rs) |
| get(health_check | N/A | GET | 20% | [View Code](src-tauri\src\api\mod.rs) |
| get_course_category | N/A | GET | 20% | [View Code](src-tauri\src\api\mod.rs) |
| get_module_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\mod.rs) |
| get_assignment_topic | N/A | GET | 20% | [View Code](src-tauri\src\api\mod.rs) |
| get_course_forum_activity | N/A | GET | 20% | [View Code](src-tauri\src\api\mod.rs) |
| get(get_current_user | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| put(update_user_profile | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| post(create_post | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| post(create_topic | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| post(create_category | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| put(update_category | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| get(root | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| post(register_user | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| post(login_user | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| get(list_categories | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| get(get_category | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| get(get_topic | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| get(list_topics | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| get(list_topic_posts | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |
| get(get_post | N/A | GET | 20% | [View Code](src-tauri\src\main.rs) |

## UI Component Reference

| Component | Type | Completeness | Implementation |
|-----------|------|-------------|----------------|
| ActivityItem | Component | 35% | [View Code](src\components\shared\activity_stream.rs) |
| ActivityLog | Component | 40% | [View Code](src\components\forum\admin\activity_log.rs) |
| ActivityStream | Component | 40% | [View Code](src\components\shared\activity_stream.rs) |
| AdminCategories | Component | 40% | [View Code](src\components\admin\categories.rs) |
| AdminDashboard | Component | 40% | [View Code](src\components\admin\dashboard.rs) |
| AdminDashboard | Component | 40% | [View Code](src\components\forum\admin\dashboard.rs) |
| AdminLayout | Component | 35% | [View Code](src\components\admin\layout.rs) |
| AdminLayout | Component | 35% | [View Code](src\components\forum\admin\admin_layout.rs) |
| AdminSidebar | Component | 35% | [View Code](src\components\admin\layout.rs) |
| AdminUsers | Component | 25% | [View Code](src\components\admin\users.rs) |
| AllNotifications | Component | 40% | [View Code](src\components\forum\all_notifications.rs) |
| App | Component | 25% | [View Code](src\app.rs) |
| AppLayout | Component | 35% | [View Code](src\components\layout\app_layout.rs) |
| AssignmentDetail | Component | 40% | [View Code](src\components\lms\assignments.rs) |
| AssignmentDetail | Component | 35% | [View Code](src\pages\assignment_detail.rs) |
| AssignmentDiscussion | Component | 40% | [View Code](src\components\assignment_discussion.rs) |
| AssignmentDiscussions | Component | 40% | [View Code](src\components\assignment_discussions.rs) |
| AssignmentForm | Component | 40% | [View Code](src\components\lms\assignments.rs) |
| AssignmentsList | Component | 40% | [View Code](src\components\lms\assignments.rs) |
| BookmarkButton | Component | 40% | [View Code](src\components\forum\topics\bookmark_button.rs) |
| CategoriesList | Component | 40% | [View Code](src\components\forum\categories_list.rs) |
| CategoryDetail | Component | 35% | [View Code](src\components\categories.rs) |
| CategoryDetail | Component | 40% | [View Code](src\components\forum\category_detail.rs) |
| CategoryForm | Component | 50% | [View Code](src\components\categories.rs) |
| CategoryForm | Component | 40% | [View Code](src\components\forum\category_form.rs) |
| CategoryManagement | Component | 40% | [View Code](src\components\forum\admin\category_management.rs) |
| CourseCategoryLinker | Component | 40% | [View Code](src\components\shared\course_forum_linker.rs) |
| CourseDetail | Component | 40% | [View Code](src\components\courses\course_detail.rs) |
| CourseDetail | Component | 40% | [View Code](src\components\lms\courses.rs) |
| CourseDetail | Component | 35% | [View Code](src\pages\course_detail.rs) |
| CourseForm | Component | 20% | [View Code](src\components\lms\courses.rs) |
| CourseForum | Component | 25% | [View Code](src\pages\course_forum.rs) |
| CourseForumActivity | Component | 40% | [View Code](src\components\course_forum_activity.rs) |
| CourseList | Component | 45% | [View Code](src\components\courses\course_list.rs) |
| CoursesList | Component | 40% | [View Code](src\components\lms\courses.rs) |
| Dashboard | Component | 40% | [View Code](src\components\dashboard.rs) |
| Dashboard | Component | 35% | [View Code](src\features\dashboard\dashboard_view.rs) |
| ErrorDisplay | Component | 35% | [View Code](src\components\shared\error_display.rs) |
| Footer | Component | 35% | [View Code](src\components\layout\footer.rs) |
| ForumActivityWidget | Component | 40% | [View Code](src\components\forum_activity_widget.rs) |
| ForumCategories | Component | 40% | [View Code](src\components\categories.rs) |
| ForumCategories | Component | 40% | [View Code](src\components\forum\categories.rs) |
| ForumSearch | Component | 20% | [View Code](src\components\forum\forum_search.rs) |
| ForumSettings | Component | 40% | [View Code](src\components\forum\admin\forum_settings.rs) |
| ForumThreads | Component | 40% | [View Code](src\components\forum\forum_threads.rs) |
| ForumThreads | Component | 40% | [View Code](src\components\forum\threads.rs) |
| GroupManagement | Component | 40% | [View Code](src\components\forum\group_management.rs) |
| Header | Component | 35% | [View Code](src\components\layout\header.rs) |
| Home | Component | 35% | [View Code](src\app.rs) |
| Home | Component | 45% | [View Code](src\components\home.rs) |
| ImportExport | Component | 40% | [View Code](src\components\forum\admin\import_export.rs) |
| IntegrationDashboard | Component | 35% | [View Code](src\components\shared\integration_dashboard.rs) |
| Login | Component | 45% | [View Code](src\components\auth\login.rs) |
| ModerationQueue | Component | 25% | [View Code](src\components\forum\admin\moderation_queue.rs) |
| ModuleDetail | Component | 40% | [View Code](src\components\lms\modules.rs) |
| ModuleDetail | Component | 35% | [View Code](src\pages\module_detail.rs) |
| ModuleDiscussion | Component | 40% | [View Code](src\components\module_discussion.rs) |
| ModuleDiscussions | Component | 40% | [View Code](src\components\module_discussions.rs) |
| ModuleForm | Component | 40% | [View Code](src\components\lms\modules.rs) |
| ModuleItemForm | Component | 40% | [View Code](src\components\lms\modules.rs) |
| ModuleItemForm | Component | 40% | [View Code](src\components\lms\module_items.rs) |
| ModulesList | Component | 40% | [View Code](src\components\lms\modules.rs) |
| NotFound | Component | 35% | [View Code](src\app.rs) |
| NotificationCenter | Component | 40% | [View Code](src\components\forum\notifications\notification_center.rs) |
| NotificationDropdown | Component | 40% | [View Code](src\components\forum\notifications\notification_dropdown.rs) |
| NotificationIndicator | Component | 40% | [View Code](src\components\forum\notification_indicator.rs) |
| NotificationSettings | Component | 40% | [View Code](src\components\admin\notification_settings.rs) |
| NotificationsList | Component | 40% | [View Code](src\components\forum\notifications\notifications_list.rs) |
| NotificationsPage | Component | 40% | [View Code](src\components\forum\notifications\notifications_page.rs) |
| OfflineIndicator | Component | 35% | [View Code](src\app.rs) |
| OfflineIndicator | Component | 35% | [View Code](src\components\shared\offline_indicator.rs) |
| Pagination | Component | 20% | [View Code](src\components\common\pagination.rs) |
| ProfileEdit | Component | 40% | [View Code](src\components\forum\profile_edit.rs) |
| Register | Component | 45% | [View Code](src\components\auth\register.rs) |
| ReportedContent | Component | 40% | [View Code](src\components\forum\admin\reported_content.rs) |
| RichEditor | Component | 40% | [View Code](src\components\forum\rich_editor.rs) |
| SearchBar | Component | 40% | [View Code](src\components\forum\search_bar.rs) |
| Sidebar | Component | 35% | [View Code](src\components\layout\sidebar.rs) |
| SiteCustomization | Component | 40% | [View Code](src\components\forum\admin\site_customization.rs) |
| SubscriptionButton | Component | 40% | [View Code](src\components\forum\topics\subscription_button.rs) |
| SyncStatus | Component | 25% | [View Code](src\components\sync_status.rs) |
| TagAnalytics | Component | 40% | [View Code](src\components\forum\tag_analytics.rs) |
| TagBrowser | Component | 40% | [View Code](src\components\forum\tag_browser.rs) |
| TagCloud | Component | 40% | [View Code](src\components\forum\tag_cloud.rs) |
| TagDetail | Component | 40% | [View Code](src\components\forum\tag_detail.rs) |
| TagFeed | Component | 40% | [View Code](src\components\forum\tag_feed.rs) |
| TagFilter | Component | 40% | [View Code](src\components\forum\tag_filter.rs) |
| TagFollowing | Component | 40% | [View Code](src\components\forum\tag_following.rs) |
| TagManagement | Component | 40% | [View Code](src\components\forum\tag_management.rs) |
| TagSelector | Component | 40% | [View Code](src\components\forum\tag_selector.rs) |
| ThreadDetail | Component | 40% | [View Code](src\components\forum\thread_detail.rs) |
| ThreadDetail | Component | 40% | [View Code](src\components\posts.rs) |
| TopicForm | Component | 40% | [View Code](src\components\forum\topic_form.rs) |
| TopicForm | Component | 55% | [View Code](src\components\topics.rs) |
| TopicsList | Component | 35% | [View Code](src\components\topics.rs) |
| UserGroups | Component | 40% | [View Code](src\components\forum\admin\user_groups.rs) |
| UserManagement | Component | 25% | [View Code](src\components\forum\admin\user_management.rs) |
| UserProfile | Component | 25% | [View Code](src\components\forum\user_profile.rs) |
| UserSubscriptions | Component | 40% | [View Code](src\components\forum\user\subscriptions.rs) |

## Code Quality Summary

- **Average Complexity:** 3.4
- **High Complexity Files:** 582
- **Technical Debt Score:** 0%
- **SOLID Violations:** 0

## Canvas-Discourse Integration

This project includes integration between Canvas LMS and Discourse forum systems.

| Integration Component | Status | Documentation |
|----------------------|--------|---------------|
| Model Mapping | In Progress | [Integration Reference](canvas_discourse_integration.md) |
| API Integration | Planned | [Integration Reference](canvas_discourse_integration.md) |
| Authentication | Planned | [Integration Reference](canvas_discourse_integration.md) |
| Synchronization | Not Started | [Integration Reference](canvas_discourse_integration.md) |

For complete integration details, see the [Canvas-Discourse Integration Reference](canvas_discourse_integration.md).

## Available Reports

No additional reports available yet.
