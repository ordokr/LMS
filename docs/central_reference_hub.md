# LMS Project: Central Reference Hub

_Last updated: 2025-04-09T21:47:28.600766700-04:00_

## Project Overview

The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.

## Technology Stack

### Frontend
- Leptos
- Tauri

### Backend
- Rust
- Haskell

### Database
- SQLite
- sqlx

### Search
- MeiliSearch

### AI Integration
- Local AI Model implementation via LM Studio or the like

### Blockchain
- Custom Rust implementation

### Authentication
- JWT

## Architecture Principles
- Clean Architecture
- SOLID
- Offline-first

## Design Patterns
- CQRS
- Event Sourcing
- Repository Pattern

## Project Statistics

- **Total Files**: 1128
- **Lines of Code**: 265618
- **Rust Files**: 645
- **Haskell Files**: 5

### File Types

| Extension | Count |
|-----------|-------|
| sh | 2 |
| db | 1 |
| yaml | 3 |
| ico | 1 |
| pyc | 1 |
| lock | 2 |
| png | 16 |
| icns | 1 |
| rs | 645 |
| c | 1 |
| info | 1 |
| hs | 5 |
| new | 1 |
| example | 1 |
| y | 2 |
| py | 11 |
|  | 1 |
| h | 1 |
| html | 62 |
| clean | 1 |
| rb | 1 |
| js | 112 |
| json | 20 |
| x | 2 |
| css | 24 |
| toml | 15 |
| ps1 | 1 |
| xml | 1 |
| bat | 6 |
| svg | 4 |
| code-workspace | 1 |
| css or appropriate CSS file | 1 |
| ts | 6 |
| txt | 2 |
| sql | 27 |
| pdb | 1 |
| log | 9 |
| wasm | 2 |
| md | 129 |

## Integration Status

| Integration | Source | Target | Status |
|-------------|--------|--------|--------|
| Canvas Course Management | Canvas | LMS | Completed |
| Discourse Forums | Discourse | LMS | Planned |
| Blockchain Certification | Native | LMS | Completed |

## Recent Updates

- **Canvas Course Management**: Workflow states and API endpoints finalized.
- **Blockchain Certification**: Certificate creation and verification integrated with the LMS.
- **Batch Model Generation**: Rust models updated to align with the latest configurations.

## Documentation Links

- [Architecture Documentation](./architecture/overview.md)
- [Models Documentation](./models/overview.md)
- [Integration Documentation](./integration/overview.md)
- [Blockchain Implementation](../rag_knowledge_base/integration/blockchain_implementation.md)
- [Analyzer Reference](./analyzer_reference.md)

## AI Development Guidance

This project is built with Rust and Haskell as the primary languages. When developing new features or modifying existing ones, adhere to the following principles:

1. **Rust-First Approach**: Implement core functionality in Rust whenever possible.
2. **Functional Paradigm**: Use functional programming patterns, especially for complex business logic.
3. **No JavaScript Dependencies**: Avoid JavaScript/TypeScript dependencies unless absolutely necessary.
4. **Performance Focus**: Prioritize performance in all implementations.
5. **Offline-First**: Design features to work offline by default.
6. **Security**: Implement proper authentication and authorization checks.

## AI Coding Agent Guidance

This section provides guidance for AI coding agents working on this project.

### Project Goals

The primary goal is to create a unified LMS system by integrating Canvas and Discourse, prioritizing performance, security, and offline-first capabilities.

### Architectural Constraints

- **Languages**: Use Rust and Haskell exclusively. Avoid JavaScript/TypeScript.
- **Frameworks**: Adhere to Rust-idiomatic frameworks (e.g., Leptos, Tauri, Axum) and avoid introducing incompatible technologies.
- **Data Storage**: Utilize SQLite for local data storage and consider MeiliSearch for search indexing.

### Key Components

The following components were detected:
- **blockchainHash**: Haskell function/component (.\haskell-integration\src\Blockchain\Verification.hs)
- **verifyBlock**: Haskell function/component (.\haskell-integration\src\Blockchain\Verification.hs)
- **generateTests**: Haskell function/component (.\haskell-integration\src\Blockchain\Verification.hs)
- **generateTestCasesFromProofs**: Haskell function/component (.\haskell-integration\src\Blockchain\Verification.hs)
- **parseRequirements**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **requirementToJson**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **A.object [ "type" .= ("complete_assignment"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **A.object [ "type" .= ("score_above"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **A.object [ "type" .= ("and"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **A.object [ "type" .= ("or"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **A.object [ "type" .= ("not"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **A.object [ "type" .= ("complete_all_modules"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **A.object [ "type" .= ("minimum_post_count"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **requirementFromJson**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **rtype <- obj .: "type"**: Haskell function/component (.\haskell-integration\src\Parser\CompletionRules.hs)
- **parseQueryLanguage**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **optimizeQuery**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **pushDownFilters**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **reorderConditions**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **reorderJoins**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **eliminateUnusedColumns**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **collectUsedColumns**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **isColumnUsed**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **collectColumnUsage**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **collectConditionUsage**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **collectExprUsage**: Haskell function/component (.\haskell-integration\src\Parser\QueryLanguage.hs)
- **{ queryType**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **, tables**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **, conditions**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **, projections**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **executeQuery**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **runQueryWithMemTracking**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **executeQueryPure**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **matchesConditions**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **matchCondition**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **queryMemLimit**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **logMemoryWarning**: Haskell function/component (.\haskell-integration\src\Query\Optimizer.hs)
- **{ opId**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **, opType**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **, entityId**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **, payload**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **{ resolvedOpId**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **, success**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **, conflicts**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **processBatch**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **processOps**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **groupByKey**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)
- **resolveConflicts**: Haskell function/component (.\haskell-integration\src\Sync\CRDT.hs)

### Models

The following data models were detected:
- **FileTypeStats**: Native (.\fs-utils-wasm\src\lib.rs)
- **Query**: Haskell (.\haskell-integration\src\Query\Optimizer.hs)
- **QueryType**: Haskell (.\haskell-integration\src\Query\Optimizer.hs)
- **Condition**: Haskell (.\haskell-integration\src\Query\Optimizer.hs)
- **SyncOperation**: Haskell (.\haskell-integration\src\Sync\CRDT.hs)
- **OperationType**: Haskell (.\haskell-integration\src\Sync\CRDT.hs)
- **ResolvedOperation**: Haskell (.\haskell-integration\src\Sync\CRDT.hs)
- **Course**: Canvas (.\lms-integration\src\api\canvas_client.rs)
- **Category**: Discourse (.\lms-integration\src\api\discourse_client.rs)
- **CategoryResponse**: Discourse (.\lms-integration\src\api\discourse_client.rs)
- **SyncTransaction**: Native (.\lms-integration\src\models\sync_transaction.rs)
- **Course**: Native (.\shared\models\course.rs)
- **Module**: Native (.\shared\models\course.rs)
- **Assignment**: Native (.\shared\models\course.rs)
- **Submission**: Native (.\shared\models\course.rs)
- **Enrollment**: Native (.\shared\models\course.rs)
- **ForumCategory**: Native (.\shared\models\forum.rs)
- **ForumTopic**: Native (.\shared\models\forum.rs)
- **ForumPost**: Native (.\shared\models\forum.rs)
- **ForumUserPreferences**: Native (.\shared\models\forum.rs)
- **ForumTrustLevel**: Native (.\shared\models\forum.rs)
- **User**: Native (.\shared\models\user.rs)
- **UserRole**: Native (.\shared\models\user.rs)
- **UserProfile**: Native (.\shared\models\user.rs)
- **LoginRequest**: Native (.\shared\models\user.rs)
- **RegisterRequest**: Native (.\shared\models\user.rs)
- **AuthResponse**: Native (.\shared\models\user.rs)
- **Course**: Native (.\shared\src\models\course.rs)
- **ForumCategory**: Native (.\shared\src\models\forum.rs)
- **ForumTopic**: Native (.\shared\src\models\forum.rs)
- **ForumPost**: Native (.\shared\src\models\forum.rs)
- **User**: Native (.\shared\src\models\user.rs)
- **LoginRequest**: Native (.\shared\src\models\user.rs)
- **RegisterRequest**: Native (.\shared\src\models\user.rs)
- **AuthResponse**: Native (.\shared\src\models\user.rs)
- **CanvasCourse**: Canvas (.\src\adapters\canvas_adapter.rs)
- **CanvasUser**: Canvas (.\src\adapters\canvas_adapter.rs)
- **CanvasEnrollment**: Canvas (.\src\adapters\canvas_adapter.rs)
- **CanvasAssignment**: Canvas (.\src\adapters\canvas_adapter.rs)
- **CanvasApiConfig**: Canvas (.\src\api\canvas_client.rs)
- **DiscourseApiConfig**: Discourse (.\src\api\discourse_client.rs)
- **CreateDiscussionMappingRequest**: Native (.\src\api\discussion_routes.rs)
- **UpdateDiscussionMappingRequest**: Native (.\src\api\discussion_routes.rs)
- **Category**: Native (.\src\api\forum.rs)
- **Topic**: Native (.\src\api\forum.rs)
- **NewCategory**: Native (.\src\api\forum.rs)
- **NewTopic**: Native (.\src\api\forum.rs)
- **BatchRequest**: Native (.\src\api\forum.rs)
- **SingleRequest**: Native (.\src\api\forum.rs)
- **BatchResponse**: Native (.\src\api\forum.rs)
- **SingleResponse**: Native (.\src\api\forum.rs)
- **CreateMappingRequest**: Native (.\src\api\integration.rs)
- **MappingResponse**: Native (.\src\api\integration.rs)
- **UpdateMappingRequest**: Native (.\src\api\integration.rs)
- **GenerateSSOTokenRequest**: Native (.\src\api\integration.rs)
- **TokenResponse**: Native (.\src\api\integration.rs)
- **CreateMappingRequest**: Native (.\src\api\mapping_routes.rs)
- **UpdateMappingRequest**: Native (.\src\api\mapping_routes.rs)
- **SearchResult**: Native (.\src\api\search_client.rs)
- **CreateTopicMappingRequest**: Native (.\src\api\topic_mapping.rs)
- **CreatePostMappingRequest**: Native (.\src\api\topic_mapping.rs)
- **ToggleSyncRequest**: Native (.\src\api\topic_mapping.rs)
- **UpdateTimeRequest**: Native (.\src\api\topic_mapping.rs)
- **TopicMappingResponse**: Native (.\src\api\topic_mapping.rs)
- **PostMappingResponse**: Native (.\src\api\topic_mapping.rs)
- **SyncStatusResponse**: Native (.\src\api\topic_mapping.rs)
- **CanvasWebhookPayload**: Native (.\src\api\webhooks.rs)
- **CanvasEventData**: Native (.\src\api\webhooks.rs)
- **DiscourseWebhookPayload**: Native (.\src\api\webhooks.rs)
- **WebhookResponse**: Native (.\src\api\webhooks.rs)
- **GreetArgs**: Native (.\src\app.rs)
- **ForumThread**: Native (.\src\app.rs)
- **Claims**: Native (.\src\auth\jwt.rs)
- **Claims**: Native (.\src\auth\jwt_service.rs)
- **RefreshTokenData**: Native (.\src\auth\jwt_service.rs)
- **RefreshTokenData**: Native (.\src\auth\refresh_token.rs)
- **DiscoursePayload**: Native (.\src\auth\sso.rs)
- **AuditEntry**: Native (.\src\bin\update_audit.rs)
- **CanvasTopic**: Canvas (.\src\clients\canvas_client.rs)
- **DiscourseTopic**: Discourse (.\src\clients\discourse_client.rs)
- **PostStream**: Discourse (.\src\clients\discourse_client.rs)
- **Post**: Discourse (.\src\clients\discourse_client.rs)
- **DashboardStats**: Native (.\src\components\admin\dashboard.rs)
- **ActivityItem**: Native (.\src\components\admin\dashboard.rs)
- **SystemHealth**: Native (.\src\components\admin\dashboard.rs)
- **LoginRequest**: Native (.\src\components\auth\login.rs)
- **LoginResponse**: Native (.\src\components\auth\login.rs)
- **RegisterRequest**: Native (.\src\components\auth\register.rs)
- **RegisterResponse**: Native (.\src\components\auth\register.rs)
- **SyncStatus**: Canvas (.\src\components\canvas_sync_status.rs)
- **CourseDetail**: Native (.\src\components\courses\course_detail.rs)
- **Module**: Native (.\src\components\courses\course_detail.rs)
- **Course**: Native (.\src\components\courses\course_list.rs)
- **RuleParseResult**: Native (.\src\components\course_builder.rs)
- **RuleRequest**: Native (.\src\components\course_builder.rs)
- **CourseIntegrationSettings**: Native (.\src\components\course_integration_settings.rs)
- **SearchHit**: Native (.\src\components\forum\optimized_search.rs)
- **SearchResponse**: Native (.\src\components\forum\optimized_search.rs)
- **SearchStatus**: Native (.\src\components\forum\optimized_search.rs)
- **SearchArgs**: Native (.\src\components\forum\optimized_search.rs)
- **SearchHit**: Native (.\src\components\forum\search.rs)
- **SearchResponse**: Native (.\src\components\forum\search.rs)
- **ItemContent**: Native (.\src\components\module_item_manager.rs)
- **Module**: Native (.\src\components\module_manager.rs)
- **ModuleItem**: Native (.\src\components\module_manager.rs)
- **ModuleRequest**: Native (.\src\components\module_manager.rs)
- **ModuleItemRequest**: Native (.\src\components\module_manager.rs)
- **SyncHistoryEntry**: Native (.\src\components\sync_history.rs)
- **SyncHistoryStats**: Native (.\src\components\sync_history.rs)
- **ContentTypeStats**: Native (.\src\components\sync_history.rs)
- **SyncHistoryFilters**: Native (.\src\components\sync_history.rs)
- **SyncStatus**: Native (.\src\components\sync_status.rs)
- **Config**: Native (.\src\config.rs)
- **CanvasConfig**: Native (.\src\config.rs)
- **DiscourseConfig**: Native (.\src\config.rs)
- **CreateAssignmentRequest**: Native (.\src\controllers\assignment_controller.rs)
- **AssignmentResponse**: Native (.\src\controllers\assignment_controller.rs)
- **AssignmentTopicResponse**: Native (.\src\controllers\assignment_controller.rs)
- **CreateAssignmentTopicRequest**: Native (.\src\controllers\assignment_controller.rs)
- **AssignmentWithTopicResponse**: Native (.\src\controllers\assignment_controller.rs)
- **CreateTopicFromAssignmentRequest**: Native (.\src\controllers\assignment_controller.rs)
- **MapTopicToAssignmentRequest**: Native (.\src\controllers\assignment_controller.rs)
- **LoginRequest**: Native (.\src\controllers\auth_controller.rs)
- **LoginResponse**: Native (.\src\controllers\auth_controller.rs)
- **RegisterRequest**: Native (.\src\controllers\auth_controller.rs)
- **CanvasDiscussionPayload**: Canvas (.\src\controllers\canvas_integration_controller.rs)
- **CanvasReplyPayload**: Canvas (.\src\controllers\canvas_integration_controller.rs)
- **ImportResponse**: Canvas (.\src\controllers\canvas_integration_controller.rs)
- **CreateCourseRequest**: Native (.\src\controllers\course_controller.rs)
- **CourseResponse**: Native (.\src\controllers\course_controller.rs)
- **MapCategoryRequest**: Native (.\src\controllers\course_controller.rs)
- **CreateTopicRequest**: Native (.\src\controllers\topic_controller.rs)
- **TopicResponse**: Native (.\src\controllers\topic_controller.rs)
- **TopicWithPostsResponse**: Native (.\src\controllers\topic_controller.rs)
- **UpdateTopicRequest**: Native (.\src\controllers\topic_controller.rs)
- **ForumConfig**: Native (.\src\core\forum.rs)
- **Category**: Native (.\src\core\forum.rs)
- **TrustSystem**: Native (.\src\core\forum.rs)
- **PluginConfig**: Native (.\src\core\forum.rs)
- **Hierarchy**: Native (.\src\core\forum.rs)
- **ForumSettings**: Native (.\src\models\admin.rs)
- **ForumSettingsUpdate**: Native (.\src\models\admin.rs)
- **ReportedContent**: Native (.\src\models\admin.rs)
- **ActivityLog**: Native (.\src\models\admin.rs)
- **ActivityLogPage**: Native (.\src\models\admin.rs)
- **DashboardStats**: Native (.\src\models\admin.rs)
- **PopularTopic**: Native (.\src\models\admin.rs)
- **TopContributor**: Native (.\src\models\admin.rs)
- **ActivityData**: Native (.\src\models\admin.rs)
- **TimeSeriesData**: Native (.\src\models\admin.rs)
- **DistributionData**: Native (.\src\models\admin.rs)
- **UserManagementPage**: Native (.\src\models\admin.rs)
- **NotificationSettings**: Native (.\src\models\admin.rs)
- **UserGroup**: Native (.\src\models\admin.rs)
- **UserGroupCreate**: Native (.\src\models\admin.rs)
- **UserGroupUpdate**: Native (.\src\models\admin.rs)
- **GroupMember**: Native (.\src\models\admin.rs)
- **GroupMember**: Native (.\src\models\admin.rs)
- **SiteCustomization**: Native (.\src\models\admin.rs)
- **ExportOptions**: Native (.\src\models\admin.rs)
- **ImportOptions**: Native (.\src\models\admin.rs)
- **ImportStats**: Native (.\src\models\admin.rs)
- **BackupInfo**: Native (.\src\models\admin.rs)
- **Setting**: Native (.\src\models\admin.rs)
- **Assignment**: Native (.\src\models\assignment.rs)
- **Category**: Native (.\src\models\category.rs)
- **Course**: Native (.\src\models\course\mod.rs)
- **Course**: Native (.\src\models\course.rs)
- **DiscussionMapping**: Native (.\src\models\discussion.rs)
- **DiscussionSyncSummary**: Native (.\src\models\discussion.rs)
- **Category**: Native (.\src\models\forum\category.rs)
- **CategoryRequest**: Native (.\src\models\forum\category.rs)
- **Post**: Native (.\src\models\forum\post.rs)
- **PostRequest**: Native (.\src\models\forum\post.rs)
- **Tag**: Native (.\src\models\forum\tag.rs)
- **TagWithTopics**: Native (.\src\models\forum\tag.rs)
- **CreateTagRequest**: Native (.\src\models\forum\tag.rs)
- **UpdateTagRequest**: Native (.\src\models\forum\tag.rs)
- **FollowedTag**: Native (.\src\models\forum\tag.rs)
- **Topic**: Native (.\src\models\forum\topic.rs)
- **TopicRequest**: Native (.\src\models\forum\topic.rs)
- **TopicSummary**: Native (.\src\models\forum\topic.rs)
- **Category**: Native (.\src\models\forum.rs)
- **Topic**: Native (.\src\models\forum.rs)
- **Post**: Native (.\src\models\forum.rs)
- **ForumStats**: Native (.\src\models\forum.rs)
- **CreateTopicRequest**: Native (.\src\models\forum.rs)
- **CreatePostRequest**: Native (.\src\models\forum.rs)
- **UpdatePostRequest**: Native (.\src\models\forum.rs)
- **TopicCreationRequest**: Native (.\src\models\forum.rs)
- **TopicUpdateRequest**: Native (.\src\models\forum.rs)
- **Group**: Native (.\src\models\forum.rs)
- **Site**: Native (.\src\models\forum.rs)
- **SiteFeatures**: Native (.\src\models\forum.rs)
- **PostActionType**: Native (.\src\models\forum.rs)
- **UserFieldType**: Native (.\src\models\forum.rs)
- **Course**: Native (.\src\models\lms.rs)
- **Module**: Native (.\src\models\lms.rs)
- **ModuleItem**: Native (.\src\models\lms.rs)
- **CompletionRequirement**: Native (.\src\models\lms.rs)
- **Assignment**: Native (.\src\models\lms.rs)
- **AssignmentGroup**: Native (.\src\models\lms.rs)
- **AssignmentGroupRules**: Native (.\src\models\lms.rs)
- **Enrollment**: Native (.\src\models\lms.rs)
- **CourseCreationRequest**: Native (.\src\models\lms.rs)
- **ModuleWithItems**: Native (.\src\models\lms.rs)
- **Submission**: Native (.\src\models\lms.rs)
- **Page**: Native (.\src\models\lms.rs)
- **Submission**: Native (.\src\models\lms.rs)
- **SubmissionComment**: Native (.\src\models\lms.rs)
- **Page**: Native (.\src\models\lms.rs)
- **CourseCategoryMapping**: Native (.\src\models\mapping.rs)
- **CourseCategory**: Native (.\src\models\mapping.rs)
- **SyncSummary**: Native (.\src\models\mapping.rs)
- **DiscussionTopicMapping**: Native (.\src\models\mapping.rs)
- **Module**: Native (.\src\models\module.rs)
- **ModuleCreate**: Native (.\src\models\module.rs)
- **ModuleUpdate**: Native (.\src\models\module.rs)
- **ModuleItem**: Native (.\src\models\module.rs)
- **ModuleItemCreate**: Native (.\src\models\module.rs)
- **ModuleItemUpdate**: Native (.\src\models\module.rs)
- **Notification**: Native (.\src\models\notification.rs)
- **NotificationPreference**: Native (.\src\models\notification.rs)
- **NotificationPreferences**: Native (.\src\models\notification.rs)
- **NotificationSummary**: Native (.\src\models\notification.rs)
- **NotificationData**: Native (.\src\models\notification.rs)
- **NotificationSettings**: Native (.\src\models\notification.rs)
- **Notification**: Native (.\src\models\notifications.rs)
- **NotificationSummary**: Native (.\src\models\notifications.rs)
- **Post**: Native (.\src\models\post.rs)
- **SearchRequest**: Native (.\src\models\search.rs)
- **SearchResult**: Native (.\src\models\search.rs)
- **SearchResponse**: Native (.\src\models\search.rs)
- **SearchFilters**: Native (.\src\models\search.rs)
- **SearchSuggestion**: Native (.\src\models\search.rs)
- **SearchStats**: Native (.\src\models\search.rs)
- **SyncQueueItem**: Native (.\src\models\sync_queue.rs)
- **SyncStatus**: Native (.\src\models\sync_state.rs)
- **Transaction**: Native (.\src\models\sync_transaction.rs)
- **TransactionStep**: Native (.\src\models\sync_transaction.rs)
- **Topic**: Native (.\src\models\topic.rs)
- **TopicMapping**: Native (.\src\models\topic_mapping.rs)
- **PostMapping**: Native (.\src\models\topic_mapping.rs)
- **UserActivity**: Native (.\src\models\user\activity.rs)
- **UserFollow**: Native (.\src\models\user\follow.rs)
- **TopicSubscription**: Native (.\src\models\user\follow.rs)
- **CategorySubscription**: Native (.\src\models\user\follow.rs)
- **UserSummary**: Native (.\src\models\user\mod.rs)
- **UserProfile**: Native (.\src\models\user\profile.rs)
- **UserProfileUpdate**: Native (.\src\models\user\profile.rs)
- **UserPermissionMapping**: Native (.\src\models\user_permission.rs)
- **SyncMetrics**: Native (.\src\monitoring\sync_metrics.rs)
- **TopicSearchResultDto**: Native (.\src\services\forum.rs)
- **PostSearchResultDto**: Native (.\src\services\forum.rs)
- **UserSearchResultDto**: Native (.\src\services\forum.rs)
- **SyncEvent**: Native (.\src\services\integration\sync_service.rs)
- **CrossReference**: Native (.\src\services\integration_service.rs)
- **ActivityEntry**: Native (.\src\services\integration_service.rs)
- **AppState**: Native (.\src\state\app_state.rs)
- **User**: Native (.\src\state\app_state.rs)
- **Notification**: Native (.\src\state\app_state.rs)
- **ForumState**: Native (.\src\state\app_state.rs)
- **SyncQueue**: Native (.\src\sync\sync_queue.rs)
- **SyncState**: Native (.\src\sync\sync_state.rs)
- **User**: Native (.\src\utils\auth.rs)
- **JwtClaims**: Native (.\src\utils\auth.rs)
- **JsProjectStructure**: Native (.\src\utils\file_system_utils.rs)
- **JsDirCategories**: Native (.\src\utils\file_system_utils.rs)
- **JsFileTypeStats**: Native (.\src\utils\file_system_utils.rs)
- **CachedData**: Native (.\src\utils\resource.rs)
- **SyncOperation**: Native (.\src\utils\sync.rs)
- **GeminiConfig**: Native (.\src-tauri\src\ai\gemini_analyzer.rs)
- **GeminiRequest**: Native (.\src-tauri\src\ai\gemini_analyzer.rs)
- **GeminiContent**: Native (.\src-tauri\src\ai\gemini_analyzer.rs)
- **GeminiPart**: Native (.\src-tauri\src\ai\gemini_analyzer.rs)
- **GeminiGenerationConfig**: Native (.\src-tauri\src\ai\gemini_analyzer.rs)
- **GeminiResponse**: Native (.\src-tauri\src\ai\gemini_analyzer.rs)
- **GeminiCandidate**: Native (.\src-tauri\src\ai\gemini_analyzer.rs)
- **AnalysisRequest**: Native (.\src-tauri\src\analyzers\analysis_commands.rs)
- **AnalysisProgress**: Native (.\src-tauri\src\analyzers\analysis_commands.rs)
- **AnalysisResponse**: Native (.\src-tauri\src\analyzers\analysis_commands.rs)
- **AnalysisCommand**: Native (.\src-tauri\src\analyzers\analysis_runner.rs)
- **AnalysisResult**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ProjectStatus**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ModelMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ModelInfo**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ModelRelationship**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ApiEndpointMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ApiEndpointInfo**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ComponentMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **UiComponentInfo**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **CodeQualityMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ComplexityMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **TechDebtMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **TechDebtItem**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **SolidViolations**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **CodeViolation**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **DesignPatternUsage**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **DesignPatternImplementation**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **TestMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **TestInfo**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **IntegrationMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **IntegrationPoint**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **IntegrationConflict**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **ArchitectureInfo**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **SyncSystemInfo**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **BlockchainInfo**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **FeatureAreaMetrics**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **Recommendation**: Native (.\src-tauri\src\analyzers\unified_analyzer.rs)
- **CourseRequest**: Native (.\src-tauri\src\api\courses.rs)
- **CreateCourseRequest**: Native (.\src-tauri\src\api\courses.rs)
- **UpdateCourseRequest**: Native (.\src-tauri\src\api\courses.rs)
- **CreateMappingRequest**: Native (.\src-tauri\src\api\discussion_routes.rs)
- **UpdateMappingRequest**: Native (.\src-tauri\src\api\discussion_routes.rs)
- **CreateMappingRequest**: Native (.\src-tauri\src\api\forum_mapping.rs)
- **MappingResponse**: Native (.\src-tauri\src\api\forum_mapping.rs)
- **PostsQuery**: Native (.\src-tauri\src\api\forum_posts.rs)
- **TopicQuery**: Native (.\src-tauri\src\api\forum_topics.rs)
- **CreateTopicRequest**: Native (.\src-tauri\src\api\forum_topics.rs)
- **ActivityQuery**: Native (.\src-tauri\src\api\integration.rs)
- **TopicMappingResponse**: Native (.\src-tauri\src\api\integration_commands.rs)
- **SyncTopicRequest**: Native (.\src-tauri\src\api\integration_commands.rs)
- **CourseIntegrationSettings**: Native (.\src-tauri\src\api\integration_settings.rs)
- **CourseFilter**: Native (.\src-tauri\src\api\lms\courses.rs)
- **EnrollmentRequest**: Native (.\src-tauri\src\api\lms\courses.rs)
- **EnrollmentUpdateRequest**: Native (.\src-tauri\src\api\lms\courses.rs)
- **CanvasSyncResponse**: Native (.\src-tauri\src\api\module_commands.rs)
- **SearchQuery**: Native (.\src-tauri\src\api\search.rs)
- **SearchResponse**: Native (.\src-tauri\src\api\search.rs)
- **SyncStatus**: Native (.\src-tauri\src\api\sync_status.rs)
- **Claims**: Native (.\src-tauri\src\auth\jwt_handler.rs)
- **Claims**: Native (.\src-tauri\src\auth.rs)
- **BenchmarkConfig**: Native (.\src-tauri\src\benchmark\mod.rs)
- **Scenario**: Native (.\src-tauri\src\benchmark\mod.rs)
- **BenchmarkResults**: Native (.\src-tauri\src\benchmark\mod.rs)
- **LatencyStats**: Native (.\src-tauri\src\benchmark\mod.rs)
- **ActionStats**: Native (.\src-tauri\src\benchmark\mod.rs)
- **ProjectAnalysis**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **ProjectSummary**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **Component**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **Model**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **Route**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **Integration**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **TechStack**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **ArchitectureInfo**: Native (.\src-tauri\src\bin\project_analyzer.rs)
- **ForumPost**: Native (.\src-tauri\src\blockchain\anchoring.rs)
- **CourseAchievement**: Native (.\src-tauri\src\blockchain\anchoring.rs)
- **ConsensusConfig**: Native (.\src-tauri\src\blockchain\config.rs)
- **StorageConfig**: Native (.\src-tauri\src\blockchain\config.rs)
- **PerformanceConfig**: Native (.\src-tauri\src\blockchain\config.rs)
- **ChainConfig**: Native (.\src-tauri\src\blockchain\config.rs)
- **BlockchainEntity**: Native (.\src-tauri\src\blockchain\core.rs)
- **BlockData**: Native (.\src-tauri\src\blockchain\core.rs)
- **UserId**: Native (.\src-tauri\src\blockchain\domain.rs)
- **AchievementRecord**: Native (.\src-tauri\src\blockchain\domain.rs)
- **AchievementTx**: Native (.\src-tauri\src\blockchain\memory.rs)
- **MetricsSnapshot**: Native (.\src-tauri\src\blockchain\metrics.rs)
- **CompactBlock**: Native (.\src-tauri\src\blockchain\mod.rs)
- **HashOnly**: Native (.\src-tauri\src\blockchain\mod.rs)
- **LmsBlock**: Native (.\src-tauri\src\blockchain\mod.rs)
- **GradeSubmission**: Native (.\src-tauri\src\blockchain\sync.rs)
- **CertificateIssuance**: Native (.\src-tauri\src\blockchain\sync.rs)
- **CourseCompletion**: Native (.\src-tauri\src\blockchain\sync.rs)
- **ForumPost**: Native (.\src-tauri\src\blockchain\sync.rs)
- **ProfileUpdate**: Native (.\src-tauri\src\blockchain\sync.rs)
- **BadgeAwarded**: Native (.\src-tauri\src\blockchain\sync.rs)
- **CustomEvent**: Native (.\src-tauri\src\blockchain\sync.rs)
- **TopicCacheStats**: Native (.\src-tauri\src\cache\topic_cache.rs)
- **TopicData**: Native (.\src-tauri\src\cache\topic_cache.rs)
- **Config**: Native (.\src-tauri\src\config\mod.rs)
- **AppConfig**: Native (.\src-tauri\src\config\mod.rs)
- **ForumConfig**: Native (.\src-tauri\src\config\mod.rs)
- **DatabaseConfig**: Native (.\src-tauri\src\config\mod.rs)
- **SearchConfig**: Native (.\src-tauri\src\config\mod.rs)
- **MediaConfig**: Native (.\src-tauri\src\config\mod.rs)
- **WebSocketConfig**: Native (.\src-tauri\src\config\mod.rs)
- **RefreshClaims**: Native (.\src-tauri\src\core\auth.rs)
- **RefreshClaims**: Native (.\src-tauri\src\core\auth.rs)
- **AppConfig**: Native (.\src-tauri\src\core\config.rs)
- **DatabaseConfig**: Native (.\src-tauri\src\core\config.rs)
- **ServerConfig**: Native (.\src-tauri\src\core\config.rs)
- **SyncConfig**: Native (.\src-tauri\src\core\config.rs)
- **ErrorResponse**: Native (.\src-tauri\src\core\errors.rs)
- **SystemHealth**: Native (.\src-tauri\src\core\monitoring.rs)
- **SyncStatistics**: Native (.\src-tauri\src\core\monitoring.rs)
- **ConflictMetrics**: Native (.\src-tauri\src\core\monitoring.rs)
- **MonitoringData**: Native (.\src-tauri\src\core\monitoring.rs)
- **CreateCategoryRequest**: Native (.\src-tauri\src\forum\categories.rs)
- **CreateTopicRequest**: Native (.\src-tauri\src\forum\topics.rs)
- **Course**: Native (.\src-tauri\src\lms\models\course.rs)
- **CourseSettings**: Native (.\src-tauri\src\lms\models\course.rs)
- **CourseSection**: Native (.\src-tauri\src\lms\models\course.rs)
- **CourseUser**: Native (.\src-tauri\src\lms\models\course.rs)
- **Module**: Native (.\src-tauri\src\lms\models\module.rs)
- **ModuleItem**: Native (.\src-tauri\src\lms\models\module.rs)
- **CompletionRequirement**: Native (.\src-tauri\src\lms\models\module.rs)
- **Course**: Native (.\src-tauri\src\lms\models.rs)
- **Module**: Native (.\src-tauri\src\lms\models.rs)
- **ModuleItem**: Native (.\src-tauri\src\lms\models.rs)
- **CompletionRequirement**: Native (.\src-tauri\src\lms\models.rs)
- **Assignment**: Native (.\src-tauri\src\lms\models.rs)
- **Submission**: Native (.\src-tauri\src\lms\models.rs)
- **SubmissionFile**: Native (.\src-tauri\src\lms\models.rs)
- **ContentPage**: Native (.\src-tauri\src\lms\models.rs)
- **Enrollment**: Native (.\src-tauri\src\lms\models.rs)
- **JwtClaims**: Native (.\src-tauri\src\models\auth.rs)
- **UserAuthProfile**: Native (.\src-tauri\src\models\auth.rs)
- **AuthResponse**: Native (.\src-tauri\src\models\auth.rs)
- **LoginRequest**: Native (.\src-tauri\src\models\auth.rs)
- **RegisterRequest**: Native (.\src-tauri\src\models\auth.rs)
- **RefreshTokenRequest**: Native (.\src-tauri\src\models\auth.rs)
- **Category**: Native (.\src-tauri\src\models\category.rs)
- **Assignment**: Native (.\src-tauri\src\models\content\assignment.rs)
- **Submission**: Native (.\src-tauri\src\models\content\submission.rs)
- **SubmissionComment**: Native (.\src-tauri\src\models\content\submission.rs)
- **CommentRow**: Native (.\src-tauri\src\models\content\submission.rs)
- **Course**: Native (.\src-tauri\src\models\course\course.rs)
- **LegacyCourse**: Native (.\src-tauri\src\models\course\course.rs)
- **Enrollment**: Native (.\src-tauri\src\models\course\enrollment.rs)
- **Module**: Native (.\src-tauri\src\models\course\module.rs)
- **ModuleItem**: Native (.\src-tauri\src\models\course\module.rs)
- **Course**: Native (.\src-tauri\src\models\course.rs)
- **Module**: Native (.\src-tauri\src\models\course.rs)
- **Assignment**: Native (.\src-tauri\src\models\course.rs)
- **Submission**: Native (.\src-tauri\src\models\course.rs)
- **CourseCreate**: Native (.\src-tauri\src\models\course.rs)
- **Discussion**: Native (.\src-tauri\src\models\discussion.rs)
- **DiscussionCreate**: Native (.\src-tauri\src\models\discussion.rs)
- **DiscussionMapping**: Native (.\src-tauri\src\models\discussion_mapping.rs)
- **CanvasDiscussionEntry**: Native (.\src-tauri\src\models\discussion_mapping.rs)
- **DiscourseTopic**: Native (.\src-tauri\src\models\discussion_mapping.rs)
- **DiscoursePost**: Native (.\src-tauri\src\models\discussion_mapping.rs)
- **SyncResult**: Native (.\src-tauri\src\models\discussion_mapping.rs)
- **Category**: Native (.\src-tauri\src\models\forum\category.rs)
- **TopicMapping**: Native (.\src-tauri\src\models\forum\mapping.rs)
- **PostMapping**: Native (.\src-tauri\src\models\forum\mapping.rs)
- **Post**: Native (.\src-tauri\src\models\forum\post.rs)
- **CanvasDiscussionEntry**: Native (.\src-tauri\src\models\forum\post.rs)
- **DiscoursePost**: Native (.\src-tauri\src\models\forum\post.rs)
- **Topic**: Native (.\src-tauri\src\models\forum\topic.rs)
- **UserId**: Native (.\src-tauri\src\models\ids.rs)
- **CourseId**: Native (.\src-tauri\src\models\ids.rs)
- **AssignmentId**: Native (.\src-tauri\src\models\ids.rs)
- **TopicId**: Native (.\src-tauri\src\models\ids.rs)
- **CourseCategory**: Native (.\src-tauri\src\models\integration.rs)
- **CourseCategoryMapping**: Native (.\src-tauri\src\models\integration.rs)
- **CourseCategoryCreate**: Native (.\src-tauri\src\models\integration.rs)
- **CourseCategoryUpdate**: Native (.\src-tauri\src\models\integration.rs)
- **CourseCategoryMapping**: Native (.\src-tauri\src\models\mapping.rs)
- **Module**: Native (.\src-tauri\src\models\module.rs)
- **ModuleCreate**: Native (.\src-tauri\src\models\module.rs)
- **ModuleUpdate**: Native (.\src-tauri\src\models\module.rs)
- **ModuleItem**: Native (.\src-tauri\src\models\module.rs)
- **ModuleItemCreate**: Native (.\src-tauri\src\models\module.rs)
- **ModuleItemUpdate**: Native (.\src-tauri\src\models\module.rs)
- **Notification**: Native (.\src-tauri\src\models\notification.rs)
- **NotificationCreate**: Native (.\src-tauri\src\models\notification.rs)
- **Post**: Native (.\src-tauri\src\models\post.rs)
- **Submission**: Native (.\src-tauri\src\models\submission.rs)
- **SubmissionCreate**: Native (.\src-tauri\src\models\submission.rs)
- **SyncConfig**: Native (.\src-tauri\src\models\sync_config.rs)
- **Tag**: Native (.\src-tauri\src\models\tag.rs)
- **Topic**: Native (.\src-tauri\src\models\topic.rs)
- **UserPreferences**: Native (.\src-tauri\src\models\user\preferences.rs)
- **Profile**: Native (.\src-tauri\src\models\user\profile.rs)
- **User**: Native (.\src-tauri\src\models\user\user.rs)
- **User**: Native (.\src-tauri\src\models\user.rs)
- **UserProfile**: Native (.\src-tauri\src\models\user.rs)
- **UserProfileUpdate**: Native (.\src-tauri\src\models\user.rs)
- **AuthRequest**: Native (.\src-tauri\src\models\user.rs)
- **AuthResponse**: Native (.\src-tauri\src\models\user.rs)
- **RegisterRequest**: Native (.\src-tauri\src\models\user.rs)
- **MetricSummary**: Native (.\src-tauri\src\monitoring\metrics.rs)
- **Query**: Native (.\src-tauri\src\parser_integration.rs)
- **ProfilerEntry**: Native (.\src-tauri\src\profiler.rs)
- **Category**: Native (.\src-tauri\src\repositories\forum.rs)
- **NewCategory**: Native (.\src-tauri\src\repositories\forum.rs)
- **Topic**: Native (.\src-tauri\src\repositories\forum.rs)
- **NewTopic**: Native (.\src-tauri\src\repositories\forum.rs)
- **PaginationParams**: Native (.\src-tauri\src\repositories\forum.rs)
- **Category**: Native (.\src-tauri\src\repositories\forum_optimized.rs)
- **Topic**: Native (.\src-tauri\src\repositories\forum_optimized.rs)
- **NewTopic**: Native (.\src-tauri\src\repositories\forum_optimized.rs)
- **CreateCategoryPayload**: Native (.\src-tauri\src\routes\categories.rs)
- **UpdateCategoryPayload**: Native (.\src-tauri\src\routes\categories.rs)
- **CreatePostPayload**: Native (.\src-tauri\src\routes\posts.rs)
- **UpdatePostPayload**: Native (.\src-tauri\src\routes\posts.rs)
- **RegisterUserPayload**: Native (.\src-tauri\src\routes\users.rs)
- **LoginPayload**: Native (.\src-tauri\src\routes\users.rs)
- **AuthResponse**: Native (.\src-tauri\src\routes\users.rs)
- **UserResponse**: Native (.\src-tauri\src\routes\users.rs)
- **UpdateUserPayload**: Native (.\src-tauri\src\routes\users.rs)
- **MeiliSearchConfig**: Native (.\src-tauri\src\search\embedded.rs)
- **SearchStatus**: Native (.\src-tauri\src\search\manager.rs)
- **TopicDocument**: Native (.\src-tauri\src\search\meilisearch.rs)
- **CategoryDocument**: Native (.\src-tauri\src\search\meilisearch.rs)
- **SearchOptions**: Native (.\src-tauri\src\search\meilisearch.rs)
- **DiscussionTopic**: Canvas (.\src-tauri\src\services\integration\canvas_integration.rs)
- **DiscussionEntry**: Canvas (.\src-tauri\src\services\integration\canvas_integration.rs)
- **Attachment**: Canvas (.\src-tauri\src\services\integration\canvas_integration.rs)
- **CanvasUser**: Canvas (.\src-tauri\src\services\integration\canvas_integration.rs)
- **Course**: Canvas (.\src-tauri\src\services\integration\canvas_integration.rs)
- **Topic**: Discourse (.\src-tauri\src\services\integration\discourse_integration.rs)
- **Post**: Discourse (.\src-tauri\src\services\integration\discourse_integration.rs)
- **Category**: Discourse (.\src-tauri\src\services\integration\discourse_integration.rs)
- **User**: Discourse (.\src-tauri\src\services\integration\discourse_integration.rs)
- **CreateTopicRequest**: Native (.\src-tauri\src\services\sync.rs)
- **TopicResponse**: Native (.\src-tauri\src\services\sync.rs)
- **UpdateTopicRequest**: Native (.\src-tauri\src\services\sync.rs)
- **SyncStatus**: Native (.\src-tauri\src\services\sync_manager.rs)
- **Course**: Native (.\src-tauri\src\shared\models\course.rs)
- **SyncOperation**: Native (.\src-tauri\src\sync\operations.rs)
- **SyncBatch**: Native (.\src-tauri\src\sync\operations.rs)
- **Task**: Native (.\src-tauri\src\tasks\queue.rs)
- **BlockEvent**: Native (.\src-tauri\src\telemetry\logging.rs)
- **CertificateEvent**: Native (.\src-tauri\src\telemetry\logging.rs)
- **FileEntry**: Native (.\src-tauri\src\utils\index_project.rs)
- **TestModel**: Native (.\src-tauri\tests\date_utils_tests.rs)
- **Assignment**: Native (.\src-ui\components\assignments\assignment_topic_mapper.rs)
- **Topic**: Native (.\src-ui\components\assignments\assignment_topic_mapper.rs)
- **MapTopicRequest**: Native (.\src-ui\components\assignments\assignment_topic_mapper.rs)
- **ApiResponse**: Native (.\src-ui\components\assignments\assignment_topic_mapper.rs)
- **CategoryResponse**: Native (.\src-ui\components\assignments\assignment_topic_mapper.rs)
- **Category**: Native (.\src-ui\components\assignments\assignment_topic_mapper.rs)
- **ApiResponse**: Native (.\src-ui\components\assignments\assignment_topic_mapper.rs)
- **Category**: Native (.\src-ui\components\courses\course_category_mapper.rs)
- **OfflineAction**: Native (.\src-ui\components\sync\offline_sync_manager.rs)
- **SyncResult**: Native (.\src-ui\components\sync\offline_sync_manager.rs)
- **SyncResponse**: Native (.\src-ui\components\sync\offline_sync_manager.rs)
- **Topic**: Native (.\src-ui\components\topics\topic_list.rs)
- **User**: Native (.\src-ui\hooks\use_auth.rs)
- **LoginResponse**: Native (.\src-ui\hooks\use_auth.rs)
- **LoginRequest**: Native (.\src-ui\hooks\use_auth.rs)
- **RegisterData**: Native (.\src-ui\hooks\use_auth.rs)
- **Course**: Native (.\src-ui\pages\courses\courses_list.rs)
- **CourseParams**: Native (.\src-ui\pages\courses\course_detail.rs)
- **Course**: Native (.\src-ui\pages\courses\course_detail.rs)
- **TopicParams**: Native (.\src-ui\pages\topics\topic_detail.rs)
- **Topic**: Native (.\src-ui\pages\topics\topic_detail.rs)
- **Post**: Native (.\src-ui\pages\topics\topic_detail.rs)
- **TopicWithPosts**: Native (.\src-ui\pages\topics\topic_detail.rs)
- **Assignment**: Native (.\src-ui\pages\topics\topic_detail.rs)
- **GeminiInsights**: Native (.\tools\project-analyzer\src\main.rs)
- **ProjectAnalysis**: Native (.\tools\project-analyzer\src\main.rs)
- **ProjectSummary**: Native (.\tools\project-analyzer\src\main.rs)
- **Component**: Native (.\tools\project-analyzer\src\main.rs)
- **Model**: Native (.\tools\project-analyzer\src\main.rs)
- **Route**: Native (.\tools\project-analyzer\src\main.rs)
- **Integration**: Native (.\tools\project-analyzer\src\main.rs)
- **TechStack**: Native (.\tools\project-analyzer\src\main.rs)
- **ArchitectureInfo**: Native (.\tools\project-analyzer\src\main.rs)
- **AnalyzerConfig**: Native (.\tools\project-analyzer\src\main.rs)
- **AuditEntry**: Native (.\tools\update_audit.rs)

## Gemini Insights

### Code Quality

Gemini API response placeholder

### Potential Conflicts

Gemini API response placeholder

### Architecture Adherence

Gemini API response placeholder

### Next Steps

Gemini API response placeholder

### Potential Next Steps

- Implement missing features in Rust, following the existing architecture.
- Refactor existing code to improve performance and maintainability.
- Integrate Gemini for code analysis and automated documentation.
- Implement robust testing strategies to ensure code quality and security.
