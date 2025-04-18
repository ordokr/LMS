# Module Categories Implementation

_Last updated: 2025-04-18_

## Overview

This document outlines the detailed implementation plans for various module categories in the Ordo project. It identifies which components should be modular and which should remain as core, non-modular parts of the application.

## Module Categories Outline

### Core Components (Non-Modular)
1. **Core Infrastructure**
   - Database Layer (SQLite/Redb)
   - Sync Engine
   - Authentication System
   - Background Job System

2. **User Management**
   - User Profiles
   - Role Management
   - Group Management

3. **Course Management**
   - Course Creation/Management
   - Enrollment Management
   - Basic Content Organization

4. **Core UI Framework**
   - Main Application Shell
   - Navigation System
   - Theming Engine
   - Accessibility Features

5. **Security & Compliance**
   - Data Encryption
   - Audit Logging
   - Privacy Controls
   - Backup & Recovery

### Modular Components
1. **Assessment & Evaluation Systems**
   - Rubric Builder
   - Peer Assessment
   - Plagiarism Detection
   - Custom Grading Schemes

2. **Communication Tools**
   - Messaging System
   - Announcement System
   - Video Conferencing
   - Office Hours Scheduler

3. **Content Creation & Management**
   - Interactive Content Builder
   - Media Library
   - E-Book Integration
   - Markdown/LaTeX Editor

4. **Analytics & Reporting**
   - Learning Analytics Dashboard
   - Engagement Metrics
   - Custom Report Builder
   - Export Tools

5. **Integration Modules**
   - LTI Connector
   - External API Connectors
   - Authentication Providers
   - Blockchain Certification

## Core Components (Non-Modular)

The following components should remain as core, non-modular parts of the application as they provide essential functionality that other modules depend on.

### 1. Core Infrastructure

```rust
// src-tauri/src/core/infrastructure.rs
pub struct CoreInfrastructure {
    pub database: HybridStore,
    pub sync_engine: SyncEngine,
    pub auth_system: AuthSystem,
    pub job_system: JobSystem,
}

impl CoreInfrastructure {
    pub fn new(config: &Config) -> Self {
        let database = HybridStore::new(config.database_path.clone());
        let sync_engine = SyncEngine::new(database.clone());
        let auth_system = AuthSystem::new(database.clone());
        let job_system = JobSystem::new();

        Self {
            database,
            sync_engine,
            auth_system,
            job_system,
        }
    }
}
```

**Implementation Details:**

- **Database Layer**: The hybrid storage system combining SQLite for structured data and Redb for ephemeral state
- **Sync Engine**: The offline-first synchronization system with conflict resolution
- **Authentication System**: Core user authentication and authorization
- **Background Job System**: The task scheduling and processing system

**Rationale:**

These components form the foundation of the application and are required by all modules. Making them modular would add unnecessary complexity and potential performance overhead.

### 2. User Management

```rust
// src-tauri/src/core/user/mod.rs
pub struct UserManager {
    store: HybridStore,
    auth_system: Arc<AuthSystem>,
    event_bus: Arc<EventBus>,
}

impl UserManager {
    pub fn new(store: HybridStore, auth_system: Arc<AuthSystem>, event_bus: Arc<EventBus>) -> Self {
        Self {
            store,
            auth_system,
            event_bus,
        }
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        // Store user in database
        let user_id = self.store.insert_user(&user).await?;

        // Create authentication credentials
        self.auth_system.create_credentials(user_id, &user.email, &user.password).await?;

        // Notify system of new user
        self.event_bus.publish(UserEvent::Created { user_id }).await?;

        Ok(user)
    }

    // Other user management functions...
}
```

**Implementation Details:**

- **User Profiles**: Core user information storage and retrieval
- **Role Management**: Basic roles (student, instructor, admin) and permissions
- **Group Management**: User grouping functionality for courses and teams

**Rationale:**

User management is a fundamental part of the application that all modules interact with. It needs to be consistent across the entire application and cannot be easily modularized without introducing complexity.

### 3. Course Management

```rust
// src-tauri/src/core/course/mod.rs
pub struct CourseManager {
    store: HybridStore,
    user_manager: Arc<UserManager>,
    event_bus: Arc<EventBus>,
}

impl CourseManager {
    pub fn new(store: HybridStore, user_manager: Arc<UserManager>, event_bus: Arc<EventBus>) -> Self {
        Self {
            store,
            user_manager,
            event_bus,
        }
    }

    pub async fn create_course(&self, course: Course) -> Result<Course> {
        // Store course in database
        let course_id = self.store.insert_course(&course).await?;

        // Add creator as instructor
        self.user_manager.add_role(course.creator_id, Role::Instructor, course_id).await?;

        // Notify system of new course
        self.event_bus.publish(CourseEvent::Created { course_id }).await?;

        Ok(course)
    }

    // Other course management functions...
}
```

**Implementation Details:**

- **Course Creation/Management**: Basic course structure and metadata
- **Enrollment Management**: Core enrollment functionality
- **Basic Content Organization**: Folders, modules, sections structure

**Rationale:**

Course management is a central part of an LMS and provides the organizational structure that other modules build upon. It needs to be consistent and available to all parts of the application.

### 4. Core UI Framework

```rust
// src/components/core/app_shell.rs
#[component]
pub fn AppShell() -> impl IntoView {
    let (sidebar_open, set_sidebar_open) = create_signal(false);

    view! {
        <div class="app-shell">
            <Header on_menu_click=move |_| set_sidebar_open.update(|v| *v = !*v) />

            <Sidebar open=sidebar_open />

            <main class="main-content">
                <Outlet/>
            </main>

            <Footer />

            <Notifications />
        </div>
    }
}
```

**Implementation Details:**

- **Main Application Shell**: The primary UI container and layout
- **Navigation System**: Core navigation components (header, sidebar, breadcrumbs)
- **Theming Engine**: Basic theming capabilities and design system
- **Accessibility Features**: Core accessibility support and compliance

**Rationale:**

The core UI framework provides the consistent user experience and navigation structure for the entire application. It needs to be uniform across all modules to maintain usability and accessibility.

### 5. Security & Compliance

```rust
// src-tauri/src/core/security/mod.rs
pub struct SecurityManager {
    credential_manager: CredentialManager,
    encryption_service: EncryptionService,
    audit_logger: AuditLogger,
    backup_service: BackupService,
}

impl SecurityManager {
    pub fn new(config: &Config) -> Self {
        let credential_manager = CredentialManager::new(&config.security.pepper);
        let encryption_service = EncryptionService::new(&config.security.encryption_key);
        let audit_logger = AuditLogger::new(&config.security.audit_log_path);
        let backup_service = BackupService::new(&config.security.backup_path);

        Self {
            credential_manager,
            encryption_service,
            audit_logger,
            backup_service,
        }
    }

    pub fn encrypt_sensitive_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.encryption_service.encrypt(data)
    }

    pub fn log_security_event(&self, event: SecurityEvent) -> Result<()> {
        self.audit_logger.log(event)
    }

    // Other security functions...
}
```

**Implementation Details:**

- **Data Encryption**: Core security features for sensitive data
- **Audit Logging**: Basic activity tracking for security and compliance
- **Privacy Controls**: Core privacy features and data protection
- **Backup & Recovery**: Data protection mechanisms and disaster recovery

**Rationale:**

Security and compliance are critical aspects that must be consistently implemented across the entire application. These components cannot be optional and must be tightly integrated with the core infrastructure.

## Modular Components

The following components can be implemented as modular extensions to the core application, allowing for flexibility and customization.

### 1. Assessment & Evaluation Systems

```rust
// src-tauri/src/modules/assessment/mod.rs
#[cfg(feature = "assessment-module")]
pub struct AssessmentModule {
    rubric_builder: RubricBuilder,
    peer_assessment: PeerAssessment,
    plagiarism_detection: Option<PlagiarismDetection>,
    grading_schemes: Vec<Box<dyn GradingScheme>>,
}

#[cfg(feature = "assessment-module")]
impl AssessmentModule {
    pub fn new(config: &Config) -> Self {
        let rubric_builder = RubricBuilder::new();
        let peer_assessment = PeerAssessment::new();

        let plagiarism_detection = if config.enable_plagiarism_detection {
            Some(PlagiarismDetection::new(&config.plagiarism_api_key))
        } else {
            None
        };

        let mut grading_schemes: Vec<Box<dyn GradingScheme>> = Vec::new();
        grading_schemes.push(Box::new(PercentageGrading::new()));
        grading_schemes.push(Box::new(LetterGrading::new()));

        if config.enable_custom_grading {
            grading_schemes.push(Box::new(CustomGrading::new(&config.custom_grading_path)));
        }

        Self {
            rubric_builder,
            peer_assessment,
            plagiarism_detection,
            grading_schemes,
        }
    }

    pub fn register(&self, app: &mut App) {
        app.register_extension(self.rubric_builder.extension());
        app.register_extension(self.peer_assessment.extension());

        if let Some(plagiarism) = &self.plagiarism_detection {
            app.register_extension(plagiarism.extension());
        }

        for scheme in &self.grading_schemes {
            app.register_extension(scheme.extension());
        }
    }
}
```

**Implementation Details:**

- **Rubric Builder**: A module for creating and managing assessment rubrics
  - Custom rubric templates
  - Rubric sharing and importing
  - Analytics on rubric effectiveness

- **Peer Assessment**: Allow students to evaluate each other's work
  - Anonymous review workflows
  - Calibrated peer review
  - Review distribution algorithms

- **Plagiarism Detection**: Integrate plagiarism checking
  - Text similarity analysis
  - Source code plagiarism detection
  - External API integrations

- **Custom Grading Schemes**: Support for different grading systems
  - Letter grades (A-F)
  - Percentages
  - Pass/fail
  - Custom scales

**Integration Points:**

- Hooks into course content for assessment creation
- Extensions to the gradebook for custom grading schemes
- Event listeners for submission events
- Database extensions for assessment data

**Feature Flags:**

```toml
# Cargo.toml
[features]
assessment-module = ["rubric-builder", "peer-assessment"]
rubric-builder = []
peer-assessment = []
plagiarism-detection = []
custom-grading = []
```

### 2. Communication Tools

```rust
// src-tauri/src/modules/communication/mod.rs
#[cfg(feature = "communication-module")]
pub struct CommunicationModule {
    messaging: MessagingSystem,
    announcements: AnnouncementSystem,
    video_conferencing: Option<VideoConferencing>,
    office_hours: Option<OfficeHoursScheduler>,
}

#[cfg(feature = "communication-module")]
impl CommunicationModule {
    pub fn new(config: &Config) -> Self {
        let messaging = MessagingSystem::new(&config.database);
        let announcements = AnnouncementSystem::new(&config.database);

        let video_conferencing = if config.enable_video_conferencing {
            Some(VideoConferencing::new(&config.video_api_key))
        } else {
            None
        };

        let office_hours = if config.enable_office_hours {
            Some(OfficeHoursScheduler::new(&config.database))
        } else {
            None
        };

        Self {
            messaging,
            announcements,
            video_conferencing,
            office_hours,
        }
    }

    pub fn register(&self, app: &mut App) {
        app.register_extension(self.messaging.extension());
        app.register_extension(self.announcements.extension());

        if let Some(video) = &self.video_conferencing {
            app.register_extension(video.extension());
        }

        if let Some(scheduler) = &self.office_hours {
            app.register_extension(scheduler.extension());
        }
    }
}
```

**Implementation Details:**

- **Messaging System**: Private messaging between users
  - One-to-one messaging
  - Group messaging
  - Message threading
  - File attachments

- **Announcement System**: Course-wide announcements
  - Targeted announcements
  - Scheduled announcements
  - Announcement templates
  - Read receipts

- **Video Conferencing**: Integration with video platforms
  - Built-in conferencing
  - External provider integration (Zoom, Teams, etc.)
  - Recording and playback
  - Screen sharing

- **Office Hours Scheduler**: Tool for scheduling and managing office hours
  - Availability management
  - Booking system
  - Reminder notifications
  - Integration with calendar

**Integration Points:**

- User system for contact management
- Course system for context-aware messaging
- Notification system for alerts
- Calendar integration for scheduling

**Feature Flags:**

```toml
# Cargo.toml
[features]
communication-module = ["messaging", "announcements"]
messaging = []
announcements = []
video-conferencing = []
office-hours = []
```

### 3. Content Creation & Management

```rust
// src-tauri/src/modules/content/mod.rs
#[cfg(feature = "content-module")]
pub struct ContentModule {
    interactive_builder: InteractiveContentBuilder,
    media_library: MediaLibrary,
    ebook_integration: Option<EBookIntegration>,
    advanced_editor: AdvancedEditor,
}

#[cfg(feature = "content-module")]
impl ContentModule {
    pub fn new(config: &Config) -> Self {
        let interactive_builder = InteractiveContentBuilder::new();
        let media_library = MediaLibrary::new(&config.media_storage_path);

        let ebook_integration = if config.enable_ebook_integration {
            Some(EBookIntegration::new(&config.ebook_api_key))
        } else {
            None
        };

        let advanced_editor = AdvancedEditor::new();

        Self {
            interactive_builder,
            media_library,
            ebook_integration,
            advanced_editor,
        }
    }

    pub fn register(&self, app: &mut App) {
        app.register_extension(self.interactive_builder.extension());
        app.register_extension(self.media_library.extension());
        app.register_extension(self.advanced_editor.extension());

        if let Some(ebook) = &self.ebook_integration {
            app.register_extension(ebook.extension());
        }
    }
}
```

**Implementation Details:**

- **Interactive Content Builder**: For creating interactive learning materials
  - Interactive exercises
  - Embedded quizzes
  - Branching scenarios
  - Drag-and-drop activities

- **Media Library**: Centralized media management
  - Image, video, and audio storage
  - Media organization and tagging
  - Usage tracking
  - Bulk operations

- **E-Book Integration**: Support for digital textbooks
  - EPUB/PDF reader
  - Annotation tools
  - Bookmarking
  - Integration with external providers

- **Markdown/LaTeX Editor**: Advanced content editing capabilities
  - Rich text editing
  - LaTeX equation support
  - Code syntax highlighting
  - Version history

**Integration Points:**

- Course content system for embedding created content
- Assignment system for interactive assessments
- Storage system for media files
- User system for content permissions

**Feature Flags:**

```toml
# Cargo.toml
[features]
content-module = ["interactive-builder", "media-library", "advanced-editor"]
interactive-builder = []
media-library = []
ebook-integration = []
advanced-editor = []
```

### 4. Analytics & Reporting

```rust
// src-tauri/src/modules/analytics/mod.rs
#[cfg(feature = "analytics-module")]
pub struct AnalyticsModule {
    learning_dashboard: LearningAnalyticsDashboard,
    engagement_metrics: EngagementMetrics,
    report_builder: ReportBuilder,
    export_tools: ExportTools,
}

#[cfg(feature = "analytics-module")]
impl AnalyticsModule {
    pub fn new(config: &Config) -> Self {
        let learning_dashboard = LearningAnalyticsDashboard::new(&config.database);
        let engagement_metrics = EngagementMetrics::new(&config.database);
        let report_builder = ReportBuilder::new(&config.database);
        let export_tools = ExportTools::new();

        Self {
            learning_dashboard,
            engagement_metrics,
            report_builder,
            export_tools,
        }
    }

    pub fn register(&self, app: &mut App) {
        app.register_extension(self.learning_dashboard.extension());
        app.register_extension(self.engagement_metrics.extension());
        app.register_extension(self.report_builder.extension());
        app.register_extension(self.export_tools.extension());
    }
}
```

**Implementation Details:**

- **Learning Analytics Dashboard**: Visualizations of student performance
  - Performance trends
  - Comparative analytics
  - Predictive insights
  - Customizable views

- **Engagement Metrics**: Track student participation
  - Activity tracking
  - Time-on-task metrics
  - Interaction patterns
  - Participation scores

- **Custom Report Builder**: Allow instructors to create custom reports
  - Report templates
  - Scheduled reports
  - Data filtering and aggregation
  - Visualization options

- **Export Tools**: Data export in various formats
  - CSV export
  - Excel export
  - PDF reports
  - Data API access

**Integration Points:**

- Course data for analytics processing
- User activity tracking
- Assignment and quiz results
- Gradebook integration

**Feature Flags:**

```toml
# Cargo.toml
[features]
analytics-module = ["learning-dashboard", "engagement-metrics", "report-builder", "export-tools"]
learning-dashboard = []
engagement-metrics = []
report-builder = []
export-tools = []
```

### 5. Integration Modules

```rust
// src-tauri/src/modules/integration/mod.rs
#[cfg(feature = "integration-module")]
pub struct IntegrationModule {
    lti_connector: LTIConnector,
    external_api: ExternalAPIConnector,
    auth_providers: Vec<Box<dyn AuthProvider>>,
    blockchain_certification: Option<BlockchainCertification>,
}

#[cfg(feature = "integration-module")]
impl IntegrationModule {
    pub fn new(config: &Config) -> Self {
        let lti_connector = LTIConnector::new(&config.lti_config);
        let external_api = ExternalAPIConnector::new();

        let mut auth_providers: Vec<Box<dyn AuthProvider>> = Vec::new();
        if config.enable_oauth {
            auth_providers.push(Box::new(OAuthProvider::new(&config.oauth_config)));
        }
        if config.enable_saml {
            auth_providers.push(Box::new(SAMLProvider::new(&config.saml_config)));
        }
        if config.enable_ldap {
            auth_providers.push(Box::new(LDAPProvider::new(&config.ldap_config)));
        }

        let blockchain_certification = if config.enable_blockchain_certification {
            Some(BlockchainCertification::new(&config.blockchain_config))
        } else {
            None
        };

        Self {
            lti_connector,
            external_api,
            auth_providers,
            blockchain_certification,
        }
    }

    pub fn register(&self, app: &mut App) {
        app.register_extension(self.lti_connector.extension());
        app.register_extension(self.external_api.extension());

        for provider in &self.auth_providers {
            app.register_extension(provider.extension());
        }

        if let Some(blockchain) = &self.blockchain_certification {
            app.register_extension(blockchain.extension());
        }
    }
}
```

**Implementation Details:**

- **LTI Connector**: Learning Tools Interoperability support
  - LTI 1.3 compliance
  - Tool registration
  - Deep linking
  - Grade passback

- **External API Connectors**: Integration with third-party services
  - REST API clients
  - Webhook support
  - API key management
  - Rate limiting

- **Authentication Providers**: Support for different authentication methods
  - OAuth 2.0
  - SAML
  - LDAP
  - Custom providers

- **Blockchain Certification**: Secure credential verification
  - Certificate issuance
  - Verification portal
  - Badge integration
  - Credential wallet

**Integration Points:**

- Authentication system for identity federation
- Course system for tool integration
- User system for credential management
- Gradebook for grade passback

**Feature Flags:**

```toml
# Cargo.toml
[features]
integration-module = ["lti-connector", "external-api"]
lti-connector = []
external-api = []
auth-providers = []
blockchain-certification = []
```

## Implementation Considerations

For effective modularization of these additional areas, the following considerations should be taken into account:

### 1. Consistent Interface Definitions

```rust
// src-tauri/src/modules/mod.rs
pub trait Module: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn is_enabled(&self) -> bool;
    fn dependencies(&self) -> Vec<&'static str> { Vec::new() }
    fn initialize(&mut self) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
    fn extension(&self) -> Box<dyn Extension>;
}

pub trait Extension: Send + Sync + 'static {
    fn register(&self, app: &mut App) -> Result<()>;
}
```

- Define clear module interfaces for each category
- Standardize event types for inter-module communication
- Create consistent data models that modules can extend

### 2. Dependency Management

```rust
// src-tauri/src/modules/registry.rs
pub struct ModuleRegistry {
    modules: HashMap<&'static str, Box<dyn Module>>,
    services: ServiceRegistry,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            services: ServiceRegistry::new(),
        }
    }

    pub fn register<M: Module + 'static>(&mut self, module: M) -> Result<()> {
        let id = module.id();

        // Check dependencies
        for dep in module.dependencies() {
            if !self.modules.contains_key(dep) {
                return Err(Error::MissingDependency(id.to_string(), dep.to_string()));
            }
        }

        self.modules.insert(id, Box::new(module));
        Ok(())
    }
}
```

- Ensure modules only depend on core components, not other modules
- Use dependency injection for service access
- Implement a service locator pattern for module discovery

### 3. Feature Flag Strategy

```rust
// src-tauri/src/config.rs
#[derive(Deserialize, Serialize)]
pub struct ModuleConfig {
    pub assessment: AssessmentModuleConfig,
    pub communication: CommunicationModuleConfig,
    pub content: ContentModuleConfig,
    pub analytics: AnalyticsModuleConfig,
    pub integration: IntegrationModuleConfig,
}

#[derive(Deserialize, Serialize)]
pub struct AssessmentModuleConfig {
    pub enabled: bool,
    pub enable_rubric_builder: bool,
    pub enable_peer_assessment: bool,
    pub enable_plagiarism_detection: bool,
    pub enable_custom_grading: bool,
    pub plagiarism_api_key: Option<String>,
    pub custom_grading_path: Option<String>,
}
```

- Create a hierarchical feature flag system
- Allow for both compile-time and runtime feature toggling
- Implement user-level feature preferences

### 4. Performance Considerations

```rust
// src-tauri/src/modules/loader.rs
pub struct ModuleLoader {
    registry: ModuleRegistry,
    config: ModuleConfig,
}

impl ModuleLoader {
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            registry: ModuleRegistry::new(),
            config,
        }
    }

    pub async fn load_modules(&mut self) -> Result<()> {
        // Load core modules first
        self.load_core_modules().await?;

        // Load optional modules in parallel
        let mut tasks = Vec::new();

        if self.config.assessment.enabled {
            tasks.push(tokio::spawn(self.load_assessment_module()));
        }

        if self.config.communication.enabled {
            tasks.push(tokio::spawn(self.load_communication_module()));
        }

        // Wait for all modules to load
        for task in tasks {
            task.await??;
        }

        Ok(())
    }
}
```

- Implement lazy loading for module assets
- Consider module-specific database migrations
- Optimize module initialization sequence

### 5. Testing Strategy

```rust
// tests/modules/assessment_tests.rs
#[cfg(feature = "assessment-module")]
mod assessment_tests {
    use ordo::modules::assessment::*;

    #[test]
    fn test_rubric_builder() {
        let module = AssessmentModule::new(&test_config());
        let rubric = module.rubric_builder.create_rubric("Test Rubric");
        assert_eq!(rubric.name, "Test Rubric");
    }

    #[test]
    fn test_peer_assessment() {
        let module = AssessmentModule::new(&test_config());
        let assignment = create_test_assignment();
        let result = module.peer_assessment.assign_reviewers(&assignment, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }
}
```

- Create module-specific test suites
- Implement integration tests for module combinations
- Test performance with various module configurations

## Conclusion

The Ordo project's architecture should maintain a balance between core components that provide essential functionality and modular components that can be enabled or disabled based on user needs. This approach allows for flexibility and customization while ensuring a consistent and reliable user experience.

Core components should focus on providing the fundamental infrastructure, user management, course management, UI framework, and security features that are essential for the application to function. These components should be tightly integrated and optimized for performance and reliability.

Modular components should be designed with clear interfaces, minimal dependencies, and consistent behavior. They should be able to be enabled or disabled without affecting the core functionality of the application. This approach allows for a more tailored experience for different educational contexts and user needs.

By following these guidelines, the Ordo project can achieve a balance between flexibility and consistency, allowing it to adapt to a wide range of educational scenarios while maintaining a high level of quality and reliability.
