# Comprehensive Implementation Report
_Generated on 2025-04-04_

## Project Overview

- **Total Files**: 368
- **Total Directories**: 85
- **Total Size**: 6.08 MB

### File Types

| Extension | Count |
|-----------|-------|
| .yml | 2 |
| (no extension) | 3 |
| .json | 12 |
| .js | 11 |
| .pdb | 1 |
| .css | 10 |
| .bat | 1 |
| .lock | 2 |
| .toml | 6 |
| .md | 50 |
| .html | 2 |
| .code-workspace | 1 |
| .backup | 1 |
| .bak | 1 |
| .bak2 | 1 |
| .fixed | 1 |
| .svg | 2 |
| .rs | 228 |
| .css or appropriate css file | 1 |
| .png | 14 |
| .icns | 1 |
| .ico | 1 |
| .db | 1 |
| .sql | 4 |
| .py | 9 |
| .ps1 | 1 |
| .pyc | 1 |

## Models (60% Complete)

| Model | File | Type | Completeness | Fields/Methods |
|-------|------|------|-------------|----------------|
| Course | src-tauri\src\models\course.rs | struct | 100% | 10 fields, 0 methods |
| Module | src-tauri\src\models\course.rs | struct | 80% | 8 fields, 0 methods |
| Assignment | src-tauri\src\models\course.rs | struct | 80% | 8 fields, 0 methods |
| Submission | src-tauri\src\models\course.rs | struct | 70% | 7 fields, 0 methods |
| Enrollment | src-tauri\src\lms\models.rs | struct | 80% | 8 fields, 0 methods |
| CourseStatus | src-tauri\src\models\course.rs | enum | 60% | 3 fields, 0 methods |
| EnrollmentRole | src-tauri\src\lms\models.rs | enum | 100% | 5 fields, 0 methods |
| ForumCategory | shared\src\models\forum.rs | struct | 80% | 4 fields, 0 methods |
| ForumTopic | shared\src\models\forum.rs | struct | 100% | 5 fields, 0 methods |
| ForumPost | shared\src\models\forum.rs | struct | 100% | 5 fields, 0 methods |
| ForumUserPreferences | shared\models\forum.rs | struct | 70% | 7 fields, 0 methods |
| ForumTrustLevel | shared\models\forum.rs | struct | 60% | 6 fields, 0 methods |
| User | src-tauri\src\models\user.rs | struct | 100% | 10 fields, 1 methods |
| UserRole | src-tauri\src\models\user.rs | enum | 60% | 3 fields, 0 methods |
| UserProfile | src\components\auth.rs | struct | 60% | 3 fields, 0 methods |
| LoginRequest | src\components\auth\login.rs | struct | 40% | 2 fields, 0 methods |
| RegisterRequest | src\components\auth\register.rs | struct | 100% | 5 fields, 0 methods |
| AuthResponse | src-tauri\src\routes\users.rs | struct | 40% | 2 fields, 0 methods |
| GreetArgs | src\app.rs | struct | 20% | 1 fields, 0 methods |
| CreateCourseArgs | src\app.rs | struct | 40% | 2 fields, 0 methods |
| ForumThread | src\app.rs | struct | 80% | 4 fields, 0 methods |
| AuditEntry | tools\update_audit.rs | struct | 60% | 6 fields, 0 methods |
| ModelImplementation | src\bin\update_audit.rs | struct | 60% | 3 fields, 0 methods |
| DashboardStats | src\models\admin.rs | struct | 100% | 12 fields, 0 methods |
| ActivityItem | src\components\admin\dashboard.rs | struct | 80% | 8 fields, 0 methods |
| SystemHealth | src\components\admin\dashboard.rs | struct | 100% | 5 fields, 0 methods |
| UserListResponse | src\components\admin\users.rs | struct | 100% | 5 fields, 0 methods |
| AdminUserView | src\components\admin\users.rs | struct | 100% | 13 fields, 0 methods |
| SuspendUserPayload | src\components\admin\users.rs | struct | 60% | 3 fields, 0 methods |
| UpdateRolePayload | src\components\admin\users.rs | struct | 40% | 2 fields, 0 methods |
| MockIntegrationService | src\components\module_discussion_test.rs | struct | 80% | 2 fields, 1 methods |
| LoginResponse | src\components\auth\login.rs | struct | 40% | 2 fields, 0 methods |
| RegisterResponse | src\components\auth\register.rs | struct | 40% | 2 fields, 0 methods |
| UserData | src\components\auth.rs | struct | 70% | 7 fields, 0 methods |
| AuthData | src\components\auth.rs | struct | 40% | 2 fields, 0 methods |
| CourseDetail | src\components\courses\course_detail.rs | struct | 60% | 6 fields, 0 methods |
| Config | src\config.rs | struct | 80% | 2 fields, 1 methods |
| CanvasConfig | src\config.rs | struct | 40% | 2 fields, 0 methods |
| DiscourseConfig | src\config.rs | struct | 60% | 3 fields, 0 methods |
| ForumConfig | src\core\forum.rs | struct | 60% | 3 fields, 0 methods |
| Category | src-tauri\src\models\category.rs | struct | 100% | 11 fields, 1 methods |
| TrustSystem | src\core\forum.rs | struct | 0% | 0 fields, 1 methods |
| PluginConfig | src\core\forum.rs | struct | 0% | 0 fields, 0 methods |
| Hierarchy | src\core\forum.rs | struct | 60% | 1 fields, 1 methods |
| Args | src\main.rs | struct | 100% | 5 fields, 0 methods |
| ForumSettings | src\models\admin.rs | struct | 100% | 16 fields, 0 methods |
| ForumSettingsUpdate | src\models\admin.rs | struct | 100% | 13 fields, 0 methods |
| ReportedContent | src\models\admin.rs | struct | 100% | 17 fields, 0 methods |
| ActivityLog | src\models\admin.rs | struct | 90% | 9 fields, 0 methods |
| ActivityLogPage | src\models\admin.rs | struct | 80% | 4 fields, 0 methods |
| PopularTopic | src\models\admin.rs | struct | 100% | 5 fields, 0 methods |
| TopContributor | src\models\admin.rs | struct | 60% | 6 fields, 0 methods |
| ActivityData | src\models\admin.rs | struct | 40% | 2 fields, 0 methods |
| TimeSeriesData | src\models\admin.rs | struct | 100% | 5 fields, 0 methods |
| DistributionData | src\models\admin.rs | struct | 60% | 3 fields, 0 methods |
| UserManagementPage | src\models\admin.rs | struct | 80% | 4 fields, 0 methods |
| NotificationSettings | src\models\notification.rs | struct | 100% | 11 fields, 0 methods |
| UserGroup | src\models\admin.rs | struct | 100% | 11 fields, 0 methods |
| UserGroupCreate | src\models\admin.rs | struct | 70% | 7 fields, 0 methods |
| UserGroupUpdate | src\models\admin.rs | struct | 70% | 7 fields, 0 methods |
| GroupMember | src\models\admin.rs | struct | 100% | 10 fields, 1 methods |
| SiteCustomization | src\models\admin.rs | struct | 100% | 23 fields, 0 methods |
| ExportOptions | src\models\admin.rs | struct | 80% | 8 fields, 0 methods |
| ImportOptions | src\models\admin.rs | struct | 60% | 3 fields, 0 methods |
| ImportStats | src\models\admin.rs | struct | 100% | 5 fields, 0 methods |
| BackupInfo | src\models\admin.rs | struct | 100% | 5 fields, 0 methods |
| Setting | src\models\admin.rs | struct | 80% | 6 fields, 1 methods |
| ReportStatus | src\models\admin.rs | enum | 60% | 3 fields, 0 methods |
| ReportDecision | src\models\admin.rs | enum | 60% | 3 fields, 0 methods |
| ActivityType | src\models\admin.rs | enum | 100% | 17 fields, 0 methods |
| Tag | src-tauri\src\models\tag.rs | struct | 80% | 6 fields, 1 methods |
| TagWithTopics | src\models\forum\tag.rs | struct | 40% | 2 fields, 0 methods |
| CreateTagRequest | src\models\forum\tag.rs | struct | 100% | 5 fields, 0 methods |
| UpdateTagRequest | src\models\forum\tag.rs | struct | 100% | 5 fields, 0 methods |
| FollowedTag | src\models\forum\tag.rs | struct | 100% | 5 fields, 0 methods |
| Topic | src-tauri\src\models\topic.rs | struct | 100% | 12 fields, 1 methods |
| Post | src-tauri\src\models\post.rs | struct | 100% | 8 fields, 1 methods |
| ForumStats | src\models\forum.rs | struct | 100% | 5 fields, 0 methods |
| CreateTopicRequest | src-tauri\src\forum\topics.rs | struct | 60% | 3 fields, 0 methods |
| CreatePostRequest | src-tauri\src\api\forum.rs | struct | 40% | 2 fields, 0 methods |
| UpdatePostRequest | src-tauri\src\api\forum.rs | struct | 20% | 1 fields, 0 methods |
| TopicSearchResult | src\models\forum.rs | struct | 90% | 9 fields, 0 methods |
| PostSearchResult | src\models\forum.rs | struct | 80% | 8 fields, 0 methods |
| UserSearchResult | src\models\forum.rs | struct | 70% | 7 fields, 0 methods |
| TopicCreationRequest | src\models\forum.rs | struct | 70% | 7 fields, 0 methods |
| TopicUpdateRequest | src\models\forum.rs | struct | 70% | 7 fields, 0 methods |
| Group | src\models\forum.rs | struct | 100% | 24 fields, 1 methods |
| Site | src\models\forum.rs | struct | 100% | 19 fields, 1 methods |
| SiteFeatures | src\models\forum.rs | struct | 70% | 7 fields, 0 methods |
| PostActionType | src\models\forum.rs | struct | 100% | 8 fields, 1 methods |
| UserFieldType | src\models\forum.rs | struct | 90% | 9 fields, 0 methods |
| SearchResult | src\models\search.rs | struct | 100% | 22 fields, 0 methods |
| GroupMembershipLevel | src\models\forum.rs | enum | 60% | 3 fields, 0 methods |
| ModuleItem | src-tauri\src\lms\models.rs | struct | 100% | 14 fields, 0 methods |
| CompletionRequirement | src-tauri\src\lms\models.rs | struct | 60% | 3 fields, 0 methods |
| AssignmentGroup | src\models\lms.rs | struct | 100% | 8 fields, 1 methods |
| AssignmentGroupRules | src\models\lms.rs | struct | 80% | 4 fields, 0 methods |
| CourseCreationRequest | src\models\lms.rs | struct | 60% | 6 fields, 0 methods |
| ModuleWithItems | src\models\lms.rs | struct | 40% | 2 fields, 0 methods |
| Page | src\models\lms.rs | struct | 100% | 12 fields, 1 methods |
| SubmissionComment | src\models\lms.rs | struct | 60% | 6 fields, 0 methods |
| EnrollmentStatus | src\models\lms.rs | enum | 80% | 4 fields, 0 methods |
| Notification | src\models\notification.rs | struct | 100% | 11 fields, 1 methods |
| NotificationPreference | src\models\notification.rs | struct | 100% | 8 fields, 1 methods |
| NotificationPreferences | src\models\notification.rs | struct | 100% | 15 fields, 0 methods |
| NotificationSummary | src\models\notification.rs | struct | 80% | 4 fields, 0 methods |
| NotificationData | src\models\notification.rs | struct | 100% | 22 fields, 0 methods |
| DigestFrequency | src\models\notification.rs | enum | 60% | 3 fields, 0 methods |
| NotificationType | src\models\notification.rs | enum | 100% | 14 fields, 0 methods |
| SearchRequest | src\models\search.rs | struct | 100% | 16 fields, 0 methods |
| SearchResponse | src\models\search.rs | struct | 70% | 7 fields, 0 methods |
| SearchFilters | src\models\search.rs | struct | 70% | 7 fields, 0 methods |
| SearchSuggestion | src\models\search.rs | struct | 70% | 7 fields, 0 methods |
| SearchStats | src\models\search.rs | struct | 80% | 4 fields, 0 methods |
| Badge | src\models\user.rs | struct | 100% | 15 fields, 1 methods |
| UserUpdateRequest | src\models\user.rs | struct | 60% | 6 fields, 0 methods |
| UserPreferences | src\models\user.rs | struct | 100% | 34 fields, 0 methods |
| UserPreferencesUpdate | src\models\user.rs | struct | 100% | 22 fields, 0 methods |
| TopicSubscription | src\models\user.rs | struct | 100% | 10 fields, 0 methods |
| BookmarkedTopic | src\models\user.rs | struct | 100% | 10 fields, 0 methods |
| ApiClient | src\utils\api_client.rs | struct | 80% | 2 fields, 1 methods |
| TopicsResponse | src\services\api.rs | struct | 20% | 1 fields, 0 methods |
| TopicList | src\services\api.rs | struct | 20% | 1 fields, 0 methods |
| CategoriesResponse | src\services\api.rs | struct | 20% | 1 fields, 0 methods |
| CategoryList | src\services\api.rs | struct | 20% | 1 fields, 0 methods |
| ApiError | src\utils\errors.rs | enum | 70% | 7 fields, 0 methods |
| TopicSearchResultDto | src\services\forum.rs | struct | 90% | 9 fields, 0 methods |
| PostSearchResultDto | src\services\forum.rs | struct | 80% | 8 fields, 0 methods |
| UserSearchResultDto | src\services\forum.rs | struct | 70% | 7 fields, 0 methods |
| SearchResultDto | src\services\forum.rs | enum | 60% | 3 fields, 0 methods |
| ForumService | src\services\forum_service.rs | struct | 100% | 4 fields, 1 methods |
| CrossReference | src\services\integration_service.rs | struct | 80% | 8 fields, 0 methods |
| ActivityEntry | src\services\integration_service.rs | struct | 70% | 7 fields, 0 methods |
| IntegrationService | src\services\integration_service.rs | struct | 100% | 5 fields, 1 methods |
| EntityType | src\services\integration_service.rs | enum | 60% | 6 fields, 0 methods |
| ActionType | src\services\integration_service.rs | enum | 70% | 7 fields, 0 methods |
| WebSocketService | src\services\websocket.rs | struct | 100% | 3 fields, 1 methods |
| WebSocketMessage | src\services\websocket.rs | enum | 60% | 3 fields, 0 methods |
| LocalStorage | src\storage\local_storage.rs | struct | 60% | 1 fields, 1 methods |
| LocalStorageError | src\storage\local_storage.rs | enum | 60% | 6 fields, 0 methods |
| SyncQueue | src\sync\sync_queue.rs | struct | 60% | 1 fields, 1 methods |
| SyncOperation | src-tauri\src\sync\operations.rs | struct | 100% | 12 fields, 1 methods |
| SyncState | src\sync\sync_state.rs | struct | 100% | 4 fields, 1 methods |
| EntityStatus | src\sync\sync_state.rs | enum | 100% | 5 fields, 0 methods |
| JwtClaims | src\utils\auth.rs | struct | 60% | 6 fields, 0 methods |
| SyncClient | src\utils\sync.rs | struct | 60% | 1 fields, 1 methods |
| PaginationParams | src-tauri\src\api\forum.rs | struct | 40% | 2 fields, 0 methods |
| CreateCategoryRequest | src-tauri\src\forum\categories.rs | struct | 100% | 5 fields, 0 methods |
| TopicWithPosts | src-tauri\src\api\forum.rs | struct | 40% | 2 fields, 0 methods |
| AppError | src-tauri\src\core\errors.rs | enum | 20% | 1 fields, 0 methods |
| PostsQuery | src-tauri\src\api\forum_posts.rs | struct | 40% | 2 fields, 0 methods |
| ActivityQuery | src-tauri\src\api\integration.rs | struct | 20% | 1 fields, 0 methods |
| CourseFilter | src-tauri\src\api\lms\courses.rs | struct | 40% | 2 fields, 0 methods |
| EnrollmentRequest | src-tauri\src\api\lms\courses.rs | struct | 60% | 3 fields, 0 methods |
| EnrollmentUpdateRequest | src-tauri\src\api\lms\courses.rs | struct | 40% | 2 fields, 0 methods |
| Claims | src-tauri\src\core\auth.rs | struct | 80% | 8 fields, 0 methods |
| ProjectAnalysis | src-tauri\src\bin\analyze_project.rs | struct | 100% | 5 fields, 0 methods |
| AuthService | src-tauri\src\core\auth.rs | struct | 80% | 2 fields, 1 methods |
| AppConfig | src-tauri\src\core\config.rs | struct | 100% | 3 fields, 1 methods |
| DatabaseConfig | src-tauri\src\core\config.rs | struct | 60% | 3 fields, 0 methods |
| ServerConfig | src-tauri\src\core\config.rs | struct | 100% | 5 fields, 0 methods |
| SyncConfig | src-tauri\src\core\config.rs | struct | 80% | 4 fields, 0 methods |
| ErrorResponse | src-tauri\src\core\errors.rs | struct | 40% | 2 fields, 0 methods |
| AssignmentRepository | src-tauri\src\database\repositories\assignment.rs | struct | 80% | 2 fields, 1 methods |
| CategoryRepository | src-tauri\src\database\repositories\category_repository.rs | struct | 60% | 1 fields, 1 methods |
| CourseRepository | src-tauri\src\database\repositories\course.rs | struct | 60% | 1 fields, 1 methods |
| ForumCategoryRepository | src-tauri\src\repositories\forum_category_repository.rs | struct | 60% | 1 fields, 1 methods |
| ForumTopicRepository | src-tauri\src\repositories\forum_topic_repository.rs | struct | 60% | 1 fields, 1 methods |
| ModuleRepository | src-tauri\src\database\repositories\module.rs | struct | 80% | 2 fields, 1 methods |
| PostRepository | src-tauri\src\database\repositories\post_repository.rs | struct | 60% | 1 fields, 1 methods |
| TopicRepository | src-tauri\src\database\repositories\topic_repository.rs | struct | 60% | 1 fields, 1 methods |
| UserRepository | src-tauri\src\database\repositories\user_repository.rs | struct | 60% | 1 fields, 1 methods |
| CourseSettings | src-tauri\src\lms\models\course.rs | struct | 100% | 11 fields, 0 methods |
| CourseSection | src-tauri\src\lms\models\course.rs | struct | 70% | 7 fields, 0 methods |
| CourseUser | src-tauri\src\lms\models\course.rs | struct | 70% | 7 fields, 0 methods |
| CourseUserRole | src-tauri\src\lms\models\course.rs | enum | 100% | 5 fields, 0 methods |
| ModuleItemType | src-tauri\src\lms\models.rs | enum | 80% | 8 fields, 0 methods |
| CompletionRequirementType | src-tauri\src\lms\models.rs | enum | 100% | 5 fields, 0 methods |
| SubmissionFile | src-tauri\src\lms\models.rs | struct | 80% | 8 fields, 0 methods |
| ContentPage | src-tauri\src\lms\models.rs | struct | 90% | 9 fields, 0 methods |
| CourseVisibility | src-tauri\src\lms\models.rs | enum | 60% | 3 fields, 0 methods |
| HomepageType | src-tauri\src\lms\models.rs | enum | 100% | 5 fields, 0 methods |
| GradingType | src-tauri\src\lms\models.rs | enum | 60% | 6 fields, 0 methods |
| SubmissionType | src-tauri\src\lms\models.rs | enum | 90% | 9 fields, 0 methods |
| EnrollmentState | src-tauri\src\lms\models.rs | enum | 100% | 5 fields, 0 methods |
| AppState | src-tauri\src\main.rs | struct | 20% | 1 fields, 0 methods |
| CourseIdParams | src-tauri\src\main.rs | struct | 20% | 1 fields, 0 methods |
| ThreadIdParams | src-tauri\src\main.rs | struct | 20% | 1 fields, 0 methods |
| SubmissionIdParams | src-tauri\src\main.rs | struct | 20% | 1 fields, 0 methods |
| StudentIdParams | src-tauri\src\main.rs | struct | 20% | 1 fields, 0 methods |
| ForumPostRepository | src-tauri\src\repository\forum_post_repository.rs | struct | 60% | 1 fields, 1 methods |
| IntegrationRepository | src-tauri\src\repository\integration_repository.rs | struct | 60% | 1 fields, 1 methods |
| RepositoryError | src-tauri\src\repository\mod.rs | enum | 20% | 1 fields, 0 methods |
| CreateCategoryPayload | src-tauri\src\routes\categories.rs | struct | 80% | 4 fields, 0 methods |
| UpdateCategoryPayload | src-tauri\src\routes\categories.rs | struct | 100% | 5 fields, 0 methods |
| CreatePostPayload | src-tauri\src\routes\posts.rs | struct | 60% | 3 fields, 0 methods |
| UpdatePostPayload | src-tauri\src\routes\posts.rs | struct | 20% | 1 fields, 0 methods |
| CreateTopicPayload | src-tauri\src\routes\topics.rs | struct | 100% | 5 fields, 0 methods |
| UpdateTopicPayload | src-tauri\src\routes\topics.rs | struct | 60% | 3 fields, 0 methods |
| TopicQuery | src-tauri\src\routes\topics.rs | struct | 40% | 2 fields, 0 methods |
| RegisterUserPayload | src-tauri\src\routes\users.rs | struct | 80% | 4 fields, 0 methods |
| LoginPayload | src-tauri\src\routes\users.rs | struct | 40% | 2 fields, 0 methods |
| UserResponse | src-tauri\src\routes\users.rs | struct | 70% | 7 fields, 0 methods |
| UpdateUserPayload | src-tauri\src\routes\users.rs | struct | 100% | 5 fields, 0 methods |
| SyncService | src-tauri\src\services\sync.rs | struct | 100% | 3 fields, 1 methods |
| ConflictType | src-tauri\src\sync\conflicts.rs | enum | 70% | 7 fields, 0 methods |
| ConflictResolution | src-tauri\src\sync\conflicts.rs | enum | 100% | 5 fields, 0 methods |
| SyncEngine | src-tauri\src\sync\engine.rs | struct | 100% | 4 fields, 1 methods |
| SyncBatch | src-tauri\src\sync\operations.rs | struct | 80% | 6 fields, 1 methods |
| OperationType | src-tauri\src\sync\operations.rs | enum | 80% | 4 fields, 0 methods |
| FileEntry | src-tauri\src\utils\index_project.rs | struct | 100% | 4 fields, 1 methods |

## API Endpoints (49% Complete)

| Handler | File | Completeness |
|---------|------|-------------|
| register | src-tauri\src\api\auth.rs | 30% |
| login | src-tauri\src\api\auth.rs | 40% |
| me | src-tauri\src\api\auth.rs | 50% |
| refresh_token | src-tauri\src\api\auth.rs | 40% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| delete | src-tauri\src\api\forum.rs | 30% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| delete | src-tauri\src\api\forum.rs | 30% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| delete | src-tauri\src\api\forum.rs | 30% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| delete | src-tauri\src\api\forum.rs | 30% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| delete | src-tauri\src\api\forum.rs | 30% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| get | src-tauri\src\api\forum.rs | 50% |
| delete | src-tauri\src\api\forum.rs | 30% |
| get | src-tauri\src\api\forum.rs | 50% |
| get_categories | src-tauri\src\api\forum.rs | 50% |
| create_category | src-tauri\src\api\forum.rs | 30% |
| get_category | src-tauri\src\api\forum.rs | 50% |
| update_category | src-tauri\src\api\forum.rs | 30% |
| delete_category | src-tauri\src\api\forum.rs | 30% |
| get_categories_by_course | src-tauri\src\api\forum.rs | 50% |
| get_topics | src-tauri\src\api\forum.rs | 55% |
| create_topic | src-tauri\src\api\forum.rs | 55% |
| get_topic | src-tauri\src\api\forum.rs | 55% |
| update_topic | src-tauri\src\api\forum.rs | 50% |
| delete_topic | src-tauri\src\api\forum.rs | 50% |
| get_topics_by_category | src-tauri\src\api\forum.rs | 55% |
| get_recent_topics | src-tauri\src\api\forum.rs | 55% |
| get_post | src-tauri\src\api\forum.rs | 50% |
| get_posts_by_topic | src-tauri\src\api\forum.rs | 55% |
| create_post | src-tauri\src\api\forum.rs | 60% |
| update_post | src-tauri\src\api\forum.rs | 50% |
| delete_post | src-tauri\src\api\forum.rs | 50% |
| like_post | src-tauri\src\api\forum.rs | 60% |
| get_tags | src-tauri\src\api\forum.rs | 50% |
| get_topics_by_tag | src-tauri\src\api\forum.rs | 55% |
| get_forum_stats | src-tauri\src\api\forum.rs | 40% |
| search_forum | src-tauri\src\api\forum.rs | 55% |
| get_updated_categories | src-tauri\src\api\forum.rs | 65% |
| get_updated_topics | src-tauri\src\api\forum.rs | 65% |
| get_updated_posts | src-tauri\src\api\forum.rs | 65% |
| get(get_categories | src-tauri\src\api\forum.rs | 20% |
| post(create_category | src-tauri\src\api\forum.rs | 20% |
| get(get_category | src-tauri\src\api\forum.rs | 20% |
| put(update_category | src-tauri\src\api\forum.rs | 20% |
| delete(delete_category | src-tauri\src\api\forum.rs | 20% |
| get(get_topics_by_category | src-tauri\src\api\forum.rs | 20% |
| get(get_categories_by_course | src-tauri\src\api\forum.rs | 20% |
| get(get_topics | src-tauri\src\api\forum.rs | 20% |
| post(create_topic | src-tauri\src\api\forum.rs | 20% |
| get(get_topic | src-tauri\src\api\forum.rs | 20% |
| put(update_topic | src-tauri\src\api\forum.rs | 20% |
| delete(delete_topic | src-tauri\src\api\forum.rs | 20% |
| get(get_posts_by_topic | src-tauri\src\api\forum.rs | 20% |
| post(create_post | src-tauri\src\api\forum.rs | 20% |
| get(get_recent_topics | src-tauri\src\api\forum.rs | 20% |
| get(get_post | src-tauri\src\api\forum.rs | 20% |
| put(update_post | src-tauri\src\api\forum.rs | 20% |
| delete(delete_post | src-tauri\src\api\forum.rs | 20% |
| post(like_post | src-tauri\src\api\forum.rs | 20% |
| get(get_tags | src-tauri\src\api\forum.rs | 20% |
| get(get_topics_by_tag | src-tauri\src\api\forum.rs | 20% |
| get(get_forum_stats | src-tauri\src\api\forum.rs | 20% |
| get(search_forum | src-tauri\src\api\forum.rs | 20% |
| get(get_updated_categories | src-tauri\src\api\forum.rs | 20% |
| get(get_updated_topics | src-tauri\src\api\forum.rs | 20% |
| get(get_updated_posts | src-tauri\src\api\forum.rs | 20% |
| inline_88 | src-tauri\src\api\forum.rs | 30% |
| inline_89 | src-tauri\src\api\forum.rs | 30% |
| inline_90 | src-tauri\src\api\forum.rs | 30% |
| inline_91 | src-tauri\src\api\forum.rs | 30% |
| inline_92 | src-tauri\src\api\forum.rs | 30% |
| inline_93 | src-tauri\src\api\forum.rs | 30% |
| inline_94 | src-tauri\src\api\forum.rs | 30% |
| inline_95 | src-tauri\src\api\forum.rs | 30% |
| inline_96 | src-tauri\src\api\forum.rs | 30% |
| inline_97 | src-tauri\src\api\forum.rs | 30% |
| inline_98 | src-tauri\src\api\forum.rs | 30% |
| inline_99 | src-tauri\src\api\forum.rs | 30% |
| inline_100 | src-tauri\src\api\forum.rs | 30% |
| inline_101 | src-tauri\src\api\forum.rs | 30% |
| inline_102 | src-tauri\src\api\forum.rs | 30% |
| inline_103 | src-tauri\src\api\forum.rs | 30% |
| inline_104 | src-tauri\src\api\forum.rs | 30% |
| inline_105 | src-tauri\src\api\forum.rs | 30% |
| inline_106 | src-tauri\src\api\forum.rs | 30% |
| inline_107 | src-tauri\src\api\forum.rs | 30% |
| inline_108 | src-tauri\src\api\forum.rs | 30% |
| get_posts_for_topic | src-tauri\src\api\forum_posts.rs | 80% |
| get_post | src-tauri\src\api\forum_posts.rs | 80% |
| create_post | src-tauri\src\api\forum_posts.rs | 80% |
| update_post | src-tauri\src\api\forum_posts.rs | 80% |
| delete_post | src-tauri\src\api\forum_posts.rs | 80% |
| mark_as_solution | src-tauri\src\api\forum_posts.rs | 80% |
| like_post | src-tauri\src\api\forum_posts.rs | 80% |
| unlike_post | src-tauri\src\api\forum_posts.rs | 80% |
| get_recent_posts | src-tauri\src\api\forum_posts.rs | 80% |
| get_or_create_course_category | src-tauri\src\api\integration.rs | 90% |
| get_course_category | src-tauri\src\api\integration.rs | 80% |
| create_module_discussion | src-tauri\src\api\integration.rs | 90% |
| get_module_topic | src-tauri\src\api\integration.rs | 80% |
| create_assignment_discussion | src-tauri\src\api\integration.rs | 90% |
| get_assignment_topic | src-tauri\src\api\integration.rs | 80% |
| get_course_forum_activity | src-tauri\src\api\integration.rs | 80% |
| create_router | src-tauri\src\api\mod.rs | 30% |
| inline_126 | src-tauri\src\api\mod.rs | 30% |
| inline_127 | src-tauri\src\api\mod.rs | 30% |
| inline_128 | src-tauri\src\api\mod.rs | 30% |
| inline_129 | src-tauri\src\api\mod.rs | 30% |
| get(health_check | src-tauri\src\api\mod.rs | 20% |
| receive_sync_batch | src-tauri\src\api\sync.rs | 45% |

## UI Components (47% Complete)

| Component | File | Completeness |
|-----------|------|-------------|
| App | src\app.rs | 65% |
| Home | src\components\home.rs | 60% |
| OfflineIndicator | src\components\shared\offline_indicator.rs | 50% |
| NotFound | src\app.rs | 60% |
| AdminCategories | src\components\admin\categories.rs | 70% |
| AdminDashboard | src\components\forum\admin\dashboard.rs | 60% |
| AdminSidebar | src\components\admin\layout.rs | 60% |
| AdminLayout | src\components\forum\admin\admin_layout.rs | 0% |
| NotificationSettings | src\components\admin\notification_settings.rs | 80% |
| AdminUsers | src\components\admin\users.rs | 55% |
| AssignmentDiscussion | src\components\assignment_discussion.rs | 0% |
| AssignmentDiscussions | src\components\assignment_discussions.rs | 0% |
| Login | src\components\auth.rs | 55% |
| Register | src\components\auth.rs | 55% |
| AuthProvider | src\components\auth.rs | 75% |
| UserProfile | src\components\forum\user_profile.rs | 25% |
| LoginForm | src\components\auth.rs | 60% |
| RegisterForm | src\components\auth.rs | 60% |
| ForumCategories | src\components\forum\categories.rs | 60% |
| CategoryForm | src\components\forum\category_form.rs | 0% |
| CategoryDetail | src\components\forum\category_detail.rs | 0% |
| Pagination | src\components\common\pagination.rs | 0% |
| CourseDetail | src\pages\course_detail.rs | 55% |
| CourseList | src\components\courses\course_list.rs | 65% |
| CourseForumActivity | src\components\course_forum_activity.rs | 0% |
| Dashboard | src\features\dashboard\dashboard_view.rs | 60% |
| ActivityLog | src\components\forum\admin\activity_log.rs | 60% |
| CategoryManagement | src\components\forum\admin\category_management.rs | 60% |
| ForumSettings | src\components\forum\admin\forum_settings.rs | 80% |
| ImportExport | src\components\forum\admin\import_export.rs | 70% |
| ModerationQueue | src\components\forum\admin\moderation_queue.rs | 35% |
| ReportedContent | src\components\forum\admin\reported_content.rs | 50% |
| SiteCustomization | src\components\forum\admin\site_customization.rs | 80% |
| UserGroups | src\components\forum\admin\user_groups.rs | 60% |
| UserManagement | src\components\forum\admin\user_management.rs | 40% |
| AllNotifications | src\components\forum\all_notifications.rs | 50% |
| CategoriesList | src\components\forum\categories_list.rs | 55% |
| ForumSearch | src\components\forum\forum_search.rs | 20% |
| ForumThreads | src\components\forum\threads.rs | 65% |
| GroupManagement | src\components\forum\group_management.rs | 45% |
| NotificationsList | src\components\forum\notifications\notifications_list.rs | 50% |
| NotificationsPage | src\components\forum\notifications\notifications_page.rs | 55% |
| NotificationCenter | src\components\forum\notifications\notification_center.rs | 45% |
| NotificationDropdown | src\components\forum\notifications\notification_dropdown.rs | 40% |
| NotificationPreferencesPage | src\components\forum\notifications\notification_preferences.rs | 80% |
| NotificationIndicator | src\components\forum\notification_indicator.rs | 55% |
| ProfileEdit | src\components\forum\profile_edit.rs | 80% |
| RichEditor | src\components\forum\rich_editor.rs | 0% |
| SearchBar | src\components\forum\search_bar.rs | 55% |
| TagAnalytics | src\components\forum\tag_analytics.rs | 60% |
| TagBrowser | src\components\forum\tag_browser.rs | 60% |
| TagCloud | src\components\forum\tag_cloud.rs | 0% |
| TagDetail | src\components\forum\tag_detail.rs | 60% |
| TagFeed | src\components\forum\tag_feed.rs | 60% |
| TagFilter | src\components\forum\tag_filter.rs | 0% |
| TagFollowing | src\components\forum\tag_following.rs | 60% |
| TagManagement | src\components\forum\tag_management.rs | 70% |
| TagSelector | src\components\forum\tag_selector.rs | 0% |
| ThreadDetail | src\components\posts.rs | 0% |
| BookmarkButton | src\components\forum\topics\bookmark_button.rs | 0% |
| SubscriptionButton | src\components\forum\topics\subscription_button.rs | 0% |
| TopicForm | src\components\topics.rs | 45% |
| UserPreferences | src\components\forum\user\preferences.rs | 70% |
| UserSubscriptions | src\components\forum\user\subscriptions.rs | 60% |
| ForumActivityWidget | src\components\forum_activity_widget.rs | 0% |
| AppLayout | src\components\layout.rs | 50% |
| Footer | src\components\layout.rs | 60% |
| Header | src\components\layout.rs | 60% |
| Sidebar | src\components\layout\sidebar.rs | 60% |
| Layout | src\components\layout.rs | 45% |
| AssignmentsList | src\components\lms\assignments.rs | 50% |
| AssignmentDetail | src\pages\assignment_detail.rs | 45% |
| AssignmentForm | src\components\lms\assignments.rs | 55% |
| CoursesList | src\components\lms\courses.rs | 50% |
| CourseForm | src\components\lms\courses.rs | 30% |
| ModulesList | src\components\lms\modules.rs | 50% |
| ModuleDetail | src\pages\module_detail.rs | 45% |
| ModuleForm | src\components\lms\modules.rs | 55% |
| ModuleItemForm | src\components\lms\module_items.rs | 65% |
| ModuleDiscussion | src\components\module_discussion.rs | 0% |
| ModuleDiscussions | src\components\module_discussions.rs | 0% |
| ActivityStream | src\components\shared\activity_stream.rs | 0% |
| ActivityItem | src\components\shared\activity_stream.rs | 40% |
| CourseCategoryLinker | src\components\shared\course_forum_linker.rs | 40% |
| ErrorDisplay | src\components\shared\error_display.rs | 40% |
| IntegrationDashboard | src\components\shared\integration_dashboard.rs | 60% |
| SyncStatus | src\components\sync_status.rs | 25% |
| TopicsList | src\components\topics.rs | 30% |
| CourseForum | src\pages\course_forum.rs | 25% |
| CourseForumSection | src\services\integration_service.rs | 0% |

## Forum Features (81% Complete)

| Feature | File | Completeness |
|---------|------|-------------|
| ForumCategory | shared\src\models\forum.rs | 80% |
| ForumTopic | shared\src\models\forum.rs | 100% |
| ForumPost | shared\src\models\forum.rs | 100% |
| ForumUserPreferences | shared\models\forum.rs | 70% |
| ForumTrustLevel | shared\models\forum.rs | 60% |
| ForumThread | src\app.rs | 80% |
| ForumConfig | src\core\forum.rs | 60% |
| Category | src-tauri\src\models\category.rs | 100% |
| TrustSystem | src\core\forum.rs | 0% |
| PluginConfig | src\core\forum.rs | 0% |
| Hierarchy | src\core\forum.rs | 60% |
| ForumSettings | src\models\admin.rs | 100% |
| ForumSettingsUpdate | src\models\admin.rs | 100% |
| PopularTopic | src\models\admin.rs | 100% |
| NotificationSettings | src\models\notification.rs | 100% |
| Tag | src-tauri\src\models\tag.rs | 80% |
| TagWithTopics | src\models\forum\tag.rs | 40% |
| CreateTagRequest | src\models\forum\tag.rs | 100% |
| UpdateTagRequest | src\models\forum\tag.rs | 100% |
| FollowedTag | src\models\forum\tag.rs | 100% |
| Topic | src-tauri\src\models\topic.rs | 100% |
| Post | src-tauri\src\models\post.rs | 100% |
| ForumStats | src\models\forum.rs | 100% |
| CreateTopicRequest | src-tauri\src\forum\topics.rs | 60% |
| CreatePostRequest | src-tauri\src\api\forum.rs | 40% |
| UpdatePostRequest | src-tauri\src\api\forum.rs | 20% |
| TopicSearchResult | src\models\forum.rs | 90% |
| PostSearchResult | src\models\forum.rs | 80% |
| UserSearchResult | src\models\forum.rs | 70% |
| TopicCreationRequest | src\models\forum.rs | 70% |
| TopicUpdateRequest | src\models\forum.rs | 70% |
| Group | src\models\forum.rs | 100% |
| Site | src\models\forum.rs | 100% |
| SiteFeatures | src\models\forum.rs | 70% |
| PostActionType | src\models\forum.rs | 100% |
| UserFieldType | src\models\forum.rs | 90% |
| GroupMembershipLevel | src\models\forum.rs | 60% |
| SubmissionComment | src\models\lms.rs | 60% |
| Notification | src\models\notification.rs | 100% |
| NotificationPreference | src\models\notification.rs | 100% |
| NotificationPreferences | src\models\notification.rs | 100% |
| NotificationSummary | src\models\notification.rs | 80% |
| NotificationData | src\models\notification.rs | 100% |
| NotificationType | src\models\notification.rs | 100% |
| TopicSubscription | src\models\user.rs | 100% |
| BookmarkedTopic | src\models\user.rs | 100% |
| TopicsResponse | src\services\api.rs | 20% |
| TopicList | src\services\api.rs | 20% |
| CategoryList | src\services\api.rs | 20% |
| TopicSearchResultDto | src\services\forum.rs | 90% |
| PostSearchResultDto | src\services\forum.rs | 80% |
| UserSearchResultDto | src\services\forum.rs | 70% |
| SearchResultDto | src\services\forum.rs | 60% |
| ForumService | src\services\forum_service.rs | 100% |
| PaginationParams | src-tauri\src\api\forum.rs | 40% |
| CreateCategoryRequest | src-tauri\src\forum\categories.rs | 100% |
| TopicWithPosts | src-tauri\src\api\forum.rs | 40% |
| PostsQuery | src-tauri\src\api\forum_posts.rs | 40% |
| CategoryRepository | src-tauri\src\database\repositories\category_repository.rs | 60% |
| ForumCategoryRepository | src-tauri\src\repositories\forum_category_repository.rs | 60% |
| ForumTopicRepository | src-tauri\src\repositories\forum_topic_repository.rs | 60% |
| PostRepository | src-tauri\src\database\repositories\post_repository.rs | 60% |
| TopicRepository | src-tauri\src\database\repositories\topic_repository.rs | 60% |
| ThreadIdParams | src-tauri\src\main.rs | 20% |
| ForumPostRepository | src-tauri\src\repository\forum_post_repository.rs | 60% |
| CreateCategoryPayload | src-tauri\src\routes\categories.rs | 80% |
| UpdateCategoryPayload | src-tauri\src\routes\categories.rs | 100% |
| CreatePostPayload | src-tauri\src\routes\posts.rs | 60% |
| UpdatePostPayload | src-tauri\src\routes\posts.rs | 20% |
| CreateTopicPayload | src-tauri\src\routes\topics.rs | 100% |
| UpdateTopicPayload | src-tauri\src\routes\topics.rs | 60% |
| TopicQuery | src-tauri\src\routes\topics.rs | 40% |

## LMS Integration (83% Complete)

| Feature | File | Completeness |
|---------|------|-------------|
| Course | src-tauri\src\models\course.rs | 100% |
| Module | src-tauri\src\models\course.rs | 80% |
| Assignment | src-tauri\src\models\course.rs | 80% |
| Submission | src-tauri\src\models\course.rs | 70% |
| Enrollment | src-tauri\src\lms\models.rs | 80% |
| CourseStatus | src-tauri\src\models\course.rs | 60% |
| EnrollmentRole | src-tauri\src\lms\models.rs | 100% |
| CreateCourseArgs | src\app.rs | 40% |
| CourseDetail | src\components\courses\course_detail.rs | 60% |
| CanvasConfig | src\config.rs | 40% |
| DiscourseConfig | src\config.rs | 60% |
| ModuleItem | src-tauri\src\lms\models.rs | 100% |
| CompletionRequirement | src-tauri\src\lms\models.rs | 60% |
| AssignmentGroup | src\models\lms.rs | 100% |
| AssignmentGroupRules | src\models\lms.rs | 80% |
| CourseCreationRequest | src\models\lms.rs | 60% |
| ModuleWithItems | src\models\lms.rs | 40% |
| Page | src\models\lms.rs | 100% |
| EnrollmentStatus | src\models\lms.rs | 80% |
| CourseFilter | src-tauri\src\api\lms\courses.rs | 40% |
| EnrollmentRequest | src-tauri\src\api\lms\courses.rs | 60% |
| EnrollmentUpdateRequest | src-tauri\src\api\lms\courses.rs | 40% |
| AssignmentRepository | src-tauri\src\database\repositories\assignment.rs | 80% |
| CourseRepository | src-tauri\src\database\repositories\course.rs | 60% |
| ModuleRepository | src-tauri\src\database\repositories\module.rs | 80% |
| CourseSettings | src-tauri\src\lms\models\course.rs | 100% |
| CourseSection | src-tauri\src\lms\models\course.rs | 70% |
| CourseUser | src-tauri\src\lms\models\course.rs | 70% |
| CourseUserRole | src-tauri\src\lms\models\course.rs | 100% |
| ModuleItemType | src-tauri\src\lms\models.rs | 80% |
| CompletionRequirementType | src-tauri\src\lms\models.rs | 100% |
| SubmissionFile | src-tauri\src\lms\models.rs | 80% |
| ContentPage | src-tauri\src\lms\models.rs | 90% |
| CourseVisibility | src-tauri\src\lms\models.rs | 60% |
| HomepageType | src-tauri\src\lms\models.rs | 100% |
| GradingType | src-tauri\src\lms\models.rs | 60% |
| SubmissionType | src-tauri\src\lms\models.rs | 90% |
| EnrollmentState | src-tauri\src\lms\models.rs | 100% |
| CourseIdParams | src-tauri\src\main.rs | 20% |
| SubmissionIdParams | src-tauri\src\main.rs | 20% |

## Tests (1% Coverage)

| Test | File |
|------|------|
| test_course_new | src\models\lms\tests.rs |
| test_course_available_to_user | src\models\lms\tests.rs |
| test_link_course_to_category | src-tauri\src\repository\integration_repository_test.rs |
| test_link_module_to_topic | src-tauri\src\repository\integration_repository_test.rs |
| test_link_assignment_to_topic | src-tauri\src\repository\integration_repository_test.rs |
| test_get_recent_course_activity | src-tauri\src\repository\integration_repository_test.rs |

## All Source Files

| File Path | Size | Type |
|-----------|------|------|
| shared\lib.rs | 0.03 KB | Other |
| shared\models\course.rs | 1.85 KB | Model |
| shared\models\forum.rs | 1.66 KB | Model |
| shared\models\mod.rs | 0.10 KB | Other |
| shared\models\user.rs | 1.16 KB | Model |
| shared\src\lib.rs | 0.11 KB | Other |
| shared\src\models\course.rs | 0.27 KB | Model |
| shared\src\models\forum.rs | 0.60 KB | Model |
| shared\src\models\mod.rs | 0.10 KB | Other |
| shared\src\models\user.rs | 0.65 KB | Model |
| src-tauri\build.rs | 0.04 KB | Other |
| src-tauri\src\api\auth.rs | 3.03 KB | Model |
| src-tauri\src\api\forum_posts.rs | 8.28 KB | Model |
| src-tauri\src\api\forum.rs | 22.91 KB | Model |
| src-tauri\src\api\integration_test.rs | 9.76 KB | API |
| src-tauri\src\api\integration.rs | 9.89 KB | Model |
| src-tauri\src\api\lms\assignments.rs | 6.88 KB | Model |
| src-tauri\src\api\lms\courses.rs | 14.24 KB | Model |
| src-tauri\src\api\lms\mod.rs | 0.05 KB | Other |
| src-tauri\src\api\lms\modules.rs | 6.47 KB | Model |
| src-tauri\src\api\mod.rs | 1.30 KB | API |
| src-tauri\src\api\sync.rs | 1.33 KB | Model |
| src-tauri\src\auth.rs | 2.24 KB | Model |
| src-tauri\src\bin\analyze_project.rs | 9.49 KB | Model |
| src-tauri\src\core\auth.rs | 2.53 KB | Model |
| src-tauri\src\core\config.rs | 2.42 KB | Model |
| src-tauri\src\core\errors.rs | 2.56 KB | Model |
| src-tauri\src\core\mod.rs | 0.10 KB | Other |
| src-tauri\src\database\course.rs | 0.68 KB | API |
| src-tauri\src\database\forum.rs | 1.06 KB | API |
| src-tauri\src\database\mod.rs | 1.79 KB | Model |
| src-tauri\src\database\repositories.rs | 0.18 KB | Model |
| src-tauri\src\database\repositories\assignment.rs | 17.37 KB | Model |
| src-tauri\src\database\repositories\category_repository.rs | 5.70 KB | Model |
| src-tauri\src\database\repositories\course.rs | 10.78 KB | Model |
| src-tauri\src\database\repositories\forum.rs | 6.17 KB | Model |
| src-tauri\src\database\repositories\mod.rs | 0.56 KB | API |
| src-tauri\src\database\repositories\module.rs | 23.51 KB | Model |
| src-tauri\src\database\repositories\post_repository.rs | 3.90 KB | Model |
| src-tauri\src\database\repositories\topic_repository.rs | 5.02 KB | Model |
| src-tauri\src\database\repositories\user_repository.rs | 6.11 KB | Model |
| src-tauri\src\database\repositories\user.rs | 6.34 KB | Model |
| src-tauri\src\database\schema.rs | 0.31 KB | Model |
| src-tauri\src\database\schema\mod.rs | 1.08 KB | Model |
| src-tauri\src\forum\categories.rs | 1.47 KB | Model |
| src-tauri\src\forum\mod.rs | 0.05 KB | API |
| src-tauri\src\forum\topics.rs | 1.88 KB | Model |
| src-tauri\src\lib.rs | 1.00 KB | Other |
| src-tauri\src\lms\models.rs | 5.78 KB | Model |
| src-tauri\src\lms\models\course.rs | 1.65 KB | Model |
| src-tauri\src\lms\models\mod.rs | 0.20 KB | Other |
| src-tauri\src\lms\models\module.rs | 1.61 KB | Model |
| src-tauri\src\main.rs | 15.09 KB | Model |
| src-tauri\src\models\category.rs | 1.08 KB | Model |
| src-tauri\src\models\course.rs | 1.54 KB | Model |
| src-tauri\src\models\mod.rs | 0.13 KB | API |
| src-tauri\src\models\post.rs | 1.11 KB | Model |
| src-tauri\src\models\tag.rs | 0.62 KB | Model |
| src-tauri\src\models\topic.rs | 1.12 KB | Model |
| src-tauri\src\models\user.rs | 1.50 KB | Model |
| src-tauri\src\repositories\forum_category_repository.rs | 7.00 KB | Model |
| src-tauri\src\repositories\forum_post_repository.rs | 16.17 KB | Model |
| src-tauri\src\repositories\forum_topic_repository.rs | 16.87 KB | Model |
| src-tauri\src\repository\forum_post_repository.rs | 22.65 KB | Model |
| src-tauri\src\repository\integration_repository_test.rs | 9.41 KB | API |
| src-tauri\src\repository\integration_repository.rs | 14.41 KB | Model |
| src-tauri\src\repository\mod.rs | 0.84 KB | Model |
| src-tauri\src\routes\categories.rs | 6.19 KB | Model |
| src-tauri\src\routes\mod.rs | 0.39 KB | API |
| src-tauri\src\routes\posts.rs | 5.78 KB | Model |
| src-tauri\src\routes\topics.rs | 7.71 KB | Model |
| src-tauri\src\routes\users.rs | 15.07 KB | Model |
| src-tauri\src\services\mod.rs | 0.03 KB | Other |
| src-tauri\src\services\sync.rs | 3.97 KB | Model |
| src-tauri\src\sync\conflicts.rs | 8.42 KB | Model |
| src-tauri\src\sync\engine.rs | 12.77 KB | Model |
| src-tauri\src\sync\mod.rs | 0.12 KB | Other |
| src-tauri\src\sync\operations.rs | 4.19 KB | Model |
| src-tauri\src\utils\index_project.rs | 6.35 KB | Model |
| src\app.rs | 24.48 KB | Model |
| src\bin\update_audit.rs | 4.65 KB | Model |
| src\components\admin\categories.rs | 18.69 KB | Model |
| src\components\admin\dashboard.rs | 13.94 KB | Model |
| src\components\admin\layout.rs | 1.92 KB | Model |
| src\components\admin\mod.rs | 0.19 KB | Other |
| src\components\admin\notification_settings.rs | 28.18 KB | Model |
| src\components\admin\users.rs | 33.17 KB | Model |
| src\components\assignment_discussion_test.rs | 5.29 KB | Model |
| src\components\assignment_discussion.rs | 6.00 KB | Model |
| src\components\assignment_discussions.rs | 4.26 KB | Model |
| src\components\auth.rs | 23.30 KB | Model |
| src\components\auth\login.rs | 6.13 KB | Model |
| src\components\auth\mod.rs | 0.08 KB | Other |
| src\components\auth\register.rs | 9.01 KB | Model |
| src\components\categories.rs | 10.50 KB | Model |
| src\components\common\mod.rs | 0.05 KB | Other |
| src\components\common\pagination.rs | 3.93 KB | Model |
| src\components\course_forum_activity.rs | 4.81 KB | Model |
| src\components\courses\course_detail.rs | 5.57 KB | Model |
| src\components\courses\course_list.rs | 3.84 KB | Model |
| src\components\courses\mod.rs | 0.11 KB | Other |
| src\components\dashboard.rs | 7.96 KB | Model |
| src\components\forum_activity_widget.rs | 3.69 KB | Model |
| src\components\forum\admin\activity_log.rs | 20.27 KB | Model |
| src\components\forum\admin\admin_layout.rs | 8.22 KB | Model |
| src\components\forum\admin\category_management.rs | 27.83 KB | Model |
| src\components\forum\admin\dashboard.rs | 21.16 KB | Model |
| src\components\forum\admin\forum_settings.rs | 22.19 KB | Model |
| src\components\forum\admin\import_export.rs | 36.39 KB | Model |
| src\components\forum\admin\mod.rs | 0.72 KB | Other |
| src\components\forum\admin\moderation_queue.rs | 20.31 KB | Model |
| src\components\forum\admin\reported_content.rs | 24.34 KB | Model |
| src\components\forum\admin\site_customization.rs | 69.78 KB | Model |
| src\components\forum\admin\user_groups.rs | 41.43 KB | Model |
| src\components\forum\admin\user_management.rs | 23.91 KB | Model |
| src\components\forum\all_notifications.rs | 11.31 KB | Model |
| src\components\forum\categories_list.rs | 7.82 KB | Model |
| src\components\forum\categories.rs | 3.10 KB | Model |
| src\components\forum\category_detail.rs | 7.54 KB | Model |
| src\components\forum\category_form.rs | 8.22 KB | Model |
| src\components\forum\forum_home.rs | 0.16 KB | UI |
| src\components\forum\forum_nav.rs | 0.37 KB | UI |
| src\components\forum\forum_search.rs | 40.84 KB | Model |
| src\components\forum\forum_threads.rs | 11.59 KB | Model |
| src\components\forum\group_management.rs | 11.12 KB | Model |
| src\components\forum\mod.rs | 1.77 KB | API |
| src\components\forum\notification_indicator.rs | 12.98 KB | Model |
| src\components\forum\notifications\mod.rs | 0.24 KB | UI |
| src\components\forum\notifications\notification_center.rs | 12.75 KB | Model |
| src\components\forum\notifications\notification_dropdown.rs | 18.12 KB | Model |
| src\components\forum\notifications\notification_preferences.rs | 26.03 KB | Model |
| src\components\forum\notifications\notifications_list.rs | 18.40 KB | Model |
| src\components\forum\notifications\notifications_page.rs | 31.14 KB | Model |
| src\components\forum\profile_edit.rs | 11.14 KB | Model |
| src\components\forum\rich_editor.rs | 6.77 KB | Model |
| src\components\forum\search_bar.rs | 6.46 KB | Model |
| src\components\forum\tag_analytics.rs | 12.31 KB | Model |
| src\components\forum\tag_browser.rs | 5.69 KB | Model |
| src\components\forum\tag_cloud.rs | 3.09 KB | Model |
| src\components\forum\tag_detail.rs | 11.50 KB | Model |
| src\components\forum\tag_feed.rs | 8.92 KB | Model |
| src\components\forum\tag_filter.rs | 6.34 KB | Model |
| src\components\forum\tag_following.rs | 19.55 KB | Model |
| src\components\forum\tag_management.rs | 25.83 KB | Model |
| src\components\forum\tag_selector.rs | 9.91 KB | Model |
| src\components\forum\thread_detail.rs | 16.81 KB | Model |
| src\components\forum\threads.rs | 14.09 KB | Model |
| src\components\forum\topic_form.rs | 9.42 KB | Model |
| src\components\forum\topics\bookmark_button.rs | 7.62 KB | Model |
| src\components\forum\topics\subscription_button.rs | 6.08 KB | Model |
| src\components\forum\topics\topic_detail.rs | 0.59 KB | UI |
| src\components\forum\user_profile.rs | 17.86 KB | Model |
| src\components\forum\user\mod.rs | 0.21 KB | Other |
| src\components\forum\user\preferences.rs | 51.11 KB | Model |
| src\components\forum\user\profile.rs | 0.48 KB | UI |
| src\components\forum\user\subscriptions.rs | 44.29 KB | Model |
| src\components\home.rs | 6.35 KB | Model |
| src\components\layout.rs | 11.16 KB | Model |
| src\components\layout\app_layout.rs | 0.47 KB | Model |
| src\components\layout\footer.rs | 0.57 KB | Model |
| src\components\layout\header.rs | 2.45 KB | Model |
| src\components\layout\mod.rs | 0.16 KB | Other |
| src\components\layout\sidebar.rs | 0.97 KB | Model |
| src\components\lms\assignments.rs | 25.16 KB | Model |
| src\components\lms\courses.rs | 26.79 KB | Model |
| src\components\lms\mod.rs | 0.28 KB | UI |
| src\components\lms\module_items.rs | 15.47 KB | Model |
| src\components\lms\modules.rs | 29.48 KB | Model |
| src\components\mod.rs | 0.83 KB | API |
| src\components\module_discussion_test.rs | 6.49 KB | Model |
| src\components\module_discussion.rs | 5.64 KB | Model |
| src\components\module_discussions.rs | 3.81 KB | Model |
| src\components\posts.rs | 9.36 KB | Model |
| src\components\shared\activity_stream.rs | 5.19 KB | Model |
| src\components\shared\course_forum_linker.rs | 5.72 KB | Model |
| src\components\shared\error_display.rs | 0.26 KB | Model |
| src\components\shared\integration_dashboard.rs | 1.91 KB | Model |
| src\components\shared\mod.rs | 0.38 KB | Other |
| src\components\shared\offline_indicator.rs | 0.78 KB | Model |
| src\components\sync_status.rs | 3.81 KB | Model |
| src\components\topics.rs | 9.61 KB | Model |
| src\config.rs | 3.13 KB | Model |
| src\core\forum.rs | 1.42 KB | Model |
| src\features\dashboard\dashboard_view.rs | 3.45 KB | Model |
| src\features\dashboard\mod.rs | 0.06 KB | UI |
| src\features\mod.rs | 0.27 KB | Other |
| src\main.rs | 3.78 KB | Model |
| src\models\admin.rs | 11.90 KB | Model |
| src\models\forum.rs | 17.92 KB | Model |
| src\models\forum\tag.rs | 2.59 KB | Model |
| src\models\lms.rs | 20.65 KB | Model |
| src\models\lms\tests.rs | 0.65 KB | API |
| src\models\mod.rs | 0.63 KB | API |
| src\models\notification.rs | 6.02 KB | Model |
| src\models\search.rs | 2.54 KB | Model |
| src\models\user.rs | 8.80 KB | Model |
| src\pages\assignment_detail.rs | 0.65 KB | Model |
| src\pages\course_detail.rs | 0.67 KB | Model |
| src\pages\course_forum.rs | 10.07 KB | Model |
| src\pages\module_detail.rs | 0.61 KB | Model |
| src\services\admin.rs | 23.34 KB | Model |
| src\services\api.rs | 7.96 KB | Model |
| src\services\api\tests.rs | 1.39 KB | API |
| src\services\errors.rs | 0.52 KB | Model |
| src\services\forum_service.rs | 8.97 KB | Model |
| src\services\forum.rs | 26.87 KB | Model |
| src\services\integration_service.rs | 15.35 KB | Model |
| src\services\lms_service.rs | 27.59 KB | Model |
| src\services\mod.rs | 0.53 KB | Other |
| src\services\notification.rs | 4.15 KB | Model |
| src\services\search.rs | 2.29 KB | Model |
| src\services\user.rs | 9.96 KB | Model |
| src\services\websocket.rs | 7.40 KB | Model |
| src\storage\local_storage.rs | 7.58 KB | Model |
| src\storage\mod.rs | 0.08 KB | Other |
| src\sync\mod.rs | 0.17 KB | Other |
| src\sync\sync_queue.rs | 2.59 KB | Model |
| src\sync\sync_state.rs | 3.67 KB | Model |
| src\utils\api_client.rs | 5.32 KB | Model |
| src\utils\auth.rs | 3.62 KB | Model |
| src\utils\errors.rs | 0.92 KB | Model |
| src\utils\formatting.rs | 1.85 KB | Other |
| src\utils\mod.rs | 0.07 KB | Other |
| src\utils\offline.rs | 1.32 KB | API |
| src\utils\sync.rs | 5.90 KB | Model |
| tests\integration\forum_integration_test.rs | 4.76 KB | Model |
| tools\index_project.rs | 3.35 KB | Model |
| tools\update_audit.rs | 1.62 KB | Model |
