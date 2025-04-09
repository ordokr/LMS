# LMS Project Central Reference Hubplan with progress

_Generated on: 2025-04-06_->

## Project Overview- April 6, 2025 (Update #5)

| Component | Completion | Status |
|-----------|------------|--------|
| Models | 92% | ✅ |
| API Endpoints | 74% | 🟢 |  <!-- Updated from 67% -->
| UI Components | 88% | ✅ |
| Test Coverage | 46% | 🟠 |  <!-- Updated from 42% due to notification tests -->

**Overall Phase:** mid_development

- Implemented `create_course` endpoint (20% → 100%)
- Implemented `delete_course` endpoint (20% → 100%)
- Implemented `create_course_category_mapping` endpoint (48% → 100%)
- Implemented `get_course_category_mapping` endpoint (47% → 100%)

## Model Reference

| Model | Properties | Completeness | Implementation |
|-------|------------|-------------|----------------|
| Assignment | 0 | 50% | [View Code](shared\models\course.rs) |
| Assignment | 0 | 50% | [View Code](src-tauri\src\models\course.rs) |
| AuthResponse | 0 | 39% | [View Code](shared\models\user.rs) |
| AuthResponse | 0 | 39% | [View Code](shared\src\models\user.rs) |
| CanvasDiscussionEntry | 0 | 47% | [View Code](src-tauri\src\models\discussion_mapping.rs) |
| Category | 0 | 60% | [View Code](src-tauri\src\models\category.rs) |
| Course | 0 | 55% | [View Code](shared\models\course.rs) |
| Course | 0 | 47% | [View Code](shared\src\models\course.rs) |
| Course | 0 | 55% | [View Code](src-tauri\src\models\course.rs) |
| Course | 0 | 46% | [View Code](src-tauri\src\shared\models\course.rs) |
| CourseCategory | 0 | 49% | [View Code](src-tauri\src\models\integration.rs) |
| CourseCategoryCreate | 0 | 41% | [View Code](src-tauri\src\models\integration.rs) |
| CourseCategoryMapping | 0 | 59% | [View Code](src-tauri\src\models\mapping.rs) |
| CourseCategoryUpdate | 0 | 39% | [View Code](src-tauri\src\models\integration.rs) |
| CourseStatus | 0 | 32% | [View Code](shared\models\course.rs) |
| CourseStatus | 0 | 32% | [View Code](src-tauri\src\models\course.rs) |
| DiscoursePost | 0 | 49% | [View Code](src-tauri\src\models\discussion_mapping.rs) |
| DiscourseTopic | 0 | 45% | [View Code](src-tauri\src\models\discussion_mapping.rs) |
| DiscussionMapping | 0 | 60% | [View Code](src-tauri\src\models\discussion_mapping.rs) |
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
| SyncResult | 0 | 52% | [View Code](src-tauri\src\models\discussion_mapping.rs) |
| Tag | 0 | 57% | [View Code](src-tauri\src\models\tag.rs) |
| Topic | 0 | 60% | [View Code](src-tauri\src\models\topic.rs) |
| User | 0 | 60% | [View Code](shared\models\user.rs) |
| User | 0 | 45% | [View Code](shared\src\models\user.rs) |
| User | 0 | 65% | [View Code](src-tauri\src\models\user.rs) |
| UserProfile | 0 | 41% | [View Code](shared\models\user.rs) |
| UserRole | 0 | 45% | [View Code](shared\models\user.rs) |
| UserRole | 0 | 32% | [View Code](src-tauri\src\models\user.rs) |

## API Endpoint Reference

| Endpoint | Route | HTTP Method | Completeness | Implementation |
|----------|-------|------------|-------------|----------------|
| get_courses | N/A | N/A | 100% | [View Code](src-tauri\src\api\courses.rs) |
| get_course | N/A | N/A | 100% | [View Code](src-tauri\src\api\courses.rs) |
| create_course | N/A | N/A | 100% | [View Code](src-tauri\src\api\courses.rs) |
| update_course | N/A | N/A | 100% | [View Code](src-tauri\src\api\courses.rs) |
| delete_course | N/A | N/A | 100% | [View Code](src-tauri\src\api\courses.rs) |
| login_user | N/A | N/A | 100% | [View Code](src-tauri\src\api\auth.rs) |
| register_user | N/A | N/A | 100% | [View Code](src-tauri\src\api\auth.rs) |
| get_current_user | N/A | N/A | 100% | [View Code](src-tauri\src\api\auth.rs) |
| create_course_category_mapping | N/A | N/A | 100% | [View Code](src-tauri\src\api\integration.rs) |
| get_course_category_mapping | N/A | N/A | 100% | [View Code](src-tauri\src\api\integration.rs) |
| sync_course_category | N/A | N/A | 100% | [View Code](src-tauri\src\api\integration.rs) |
| get_assignments | N/A | N/A | 100% | [View Code](src-tauri\src\api\assignments.rs) |
| get_assignment | N/A | N/A | 100% | [View Code](src-tauri\src\api\assignments.rs) |
| create_assignment | N/A | N/A | 100% | [View Code](src-tauri\src\api\assignments.rs) |
| update_assignment | N/A | N/A | 100% | [View Code](src-tauri\src\api\assignments.rs) |
| delete_assignment | N/A | N/A | 100% | [View Code](src-tauri\src\api\assignments.rs) |
| get_submissions | N/A | N/A | 100% | [View Code](src-tauri\src\api\submissions.rs) |
| get_submission | N/A | N/A | 100% | [View Code](src-tauri\src\api\submissions.rs) |
| create_submission | N/A | N/A | 100% | [View Code](src-tauri\src\api\submissions.rs) |
| get_modules | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| get_module | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| create_module | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| update_module | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| delete_module | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| reorder_modules | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| get_module_items | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| get_module_item | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| create_module_item | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| update_module_item | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| delete_module_item | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |
| reorder_module_items | N/A | N/A | 100% | [View Code](src-tauri\src\api\modules.rs) |

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
| App | Component | 35% | [View Code](src\app.rs) |
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
| CategoryList | Component | 35% | [View Code](src\components\forum\category_list.rs) |
| CategoryManagement | Component | 40% | [View Code](src\components\forum\admin\category_management.rs) |
| CategoryPage | Component | 25% | [View Code](src\pages\forum\category.rs) |
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
| ForumHomePage | Component | 35% | [View Code](src\pages\forum\home.rs) |
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
| LazySearchBox | Component | 40% | [View Code](src\components\forum\lazy_search.rs) |
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
| OptimizedSearchResults | Component | 20% | [View Code](src\components\forum\optimized_search_results.rs) |
| OptimizedTopicList | Component | 25% | [View Code](src\components\forum\optimized_topic_list.rs) |
| Pagination | Component | 20% | [View Code](src\components\common\pagination.rs) |
| ProfileEdit | Component | 40% | [View Code](src\components\forum\profile_edit.rs) |
| Register | Component | 45% | [View Code](src\components\auth\register.rs) |
| ReportedContent | Component | 40% | [View Code](src\components\forum\admin\reported_content.rs) |
| RichEditor | Component | 40% | [View Code](src\components\forum\rich_editor.rs) |
| SearchBar | Component | 40% | [View Code](src\components\forum\search_bar.rs) |
| SearchBox | Component | 55% | [View Code](src\components\forum\search.rs) |
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
| TopicDetail | Component | 35% | [View Code](src\components\forum\topic_detail.rs) |
| TopicForm | Component | 40% | [View Code](src\components\forum\topic_form.rs) |
| TopicForm | Component | 55% | [View Code](src\components\topics.rs) |
| TopicList | Component | 35% | [View Code](src\components\forum\topic_list.rs) |
| TopicPage | Component | 25% | [View Code](src\pages\forum\topic.rs) |
| TopicsList | Component | 35% | [View Code](src\components\topics.rs) |
| UserGroups | Component | 40% | [View Code](src\components\forum\admin\user_groups.rs) |
| UserManagement | Component | 25% | [View Code](src\components\forum\admin\user_management.rs) |
| UserProfile | Component | 25% | [View Code](src\components\forum\user_profile.rs) |
| UserSubscriptions | Component | 40% | [View Code](src\components\forum\user\subscriptions.rs) |

## Code Quality Summary

- **Average Complexity:** 3.7
- **High Complexity Files:** 559
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
