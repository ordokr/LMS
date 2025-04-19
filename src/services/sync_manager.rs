use crate::api::{
    CanvasApiClient, DiscourseApiClient, CanvasApi, DiscourseApi,
    CanvasUser, CanvasCourse, CanvasDiscussion, CanvasDiscussionEntry,
    DiscourseUser, DiscourseCategory, DiscourseTopic, DiscoursePost
};
use crate::services::{
    ModelMapperService, EntityMapping, SyncStatus,
    SyncService, SyncResult, SyncOptions, SyncDirection,
    ModelConversionService, ErrorHandlingService, ErrorSeverity, ErrorCategory,
    ErrorHandlingExt, SyncStrategy, ConflictResolutionStrategy,
    IncrementalSyncService, BidirectionalSyncService
};
use crate::services::api_config_service::ApiConfigService;
use anyhow::{Result, anyhow};
use log::{info, error, debug, warn};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Synchronization manager service
///
/// Coordinates the synchronization between Canvas and Discourse
pub struct SyncManager {
    api_config: Arc<Mutex<ApiConfigService>>,
    model_mapper: Arc<ModelMapperService>,
    sync_service: Arc<SyncService>,
    model_conversion: Arc<ModelConversionService>,
    sync_state: Arc<Mutex<SyncState>>,
    error_service: Option<Arc<ErrorHandlingService>>,
    sync_strategies: HashMap<String, Box<dyn SyncStrategy + Send + Sync>>,
    default_strategy: String,
}

impl SyncManager {
    /// Create a new synchronization manager
    pub fn new(
        api_config: Arc<Mutex<ApiConfigService>>,
        model_mapper: Arc<ModelMapperService>,
        sync_service: Arc<SyncService>,
        model_conversion: Arc<ModelConversionService>,
        error_service: Option<Arc<ErrorHandlingService>> = None,
    ) -> Self {
        // Initialize sync strategies
        let mut sync_strategies: HashMap<String, Box<dyn SyncStrategy + Send + Sync>> = HashMap::new();

        // Add basic strategy (from SyncService)
        sync_strategies.insert("basic".to_string(), Box::new(sync_service.clone()) as Box<dyn SyncStrategy + Send + Sync>);

        // Add incremental strategy
        sync_strategies.insert("incremental".to_string(), Box::new(IncrementalSyncService::new()) as Box<dyn SyncStrategy + Send + Sync>);

        // Add bidirectional strategy
        sync_strategies.insert("bidirectional".to_string(), Box::new(BidirectionalSyncService::new(ConflictResolutionStrategy::MostRecent)) as Box<dyn SyncStrategy + Send + Sync>);

        Self {
            api_config,
            model_mapper,
            sync_service,
            model_conversion,
            sync_state: Arc::new(Mutex::new(SyncState::new())),
            error_service,
            sync_strategies,
            default_strategy: "basic".to_string(),
        }
    }

    /// Set the error handling service
    pub fn with_error_service(mut self, error_service: Arc<ErrorHandlingService>) -> Self {
        self.error_service = Some(error_service);
        self
    }

    /// Add a synchronization strategy
    pub fn add_strategy(&mut self, name: &str, strategy: Box<dyn SyncStrategy + Send + Sync>) {
        self.sync_strategies.insert(name.to_string(), strategy);
    }

    /// Get a synchronization strategy
    pub fn get_strategy(&self, name: &str) -> Option<&Box<dyn SyncStrategy + Send + Sync>> {
        self.sync_strategies.get(name)
    }

    /// Set the default synchronization strategy
    pub fn set_default_strategy(&mut self, name: &str) -> Result<()> {
        if self.sync_strategies.contains_key(name) {
            self.default_strategy = name.to_string();
            Ok(())
        } else {
            Err(anyhow!("Synchronization strategy not found: {}", name))
        }
    }

    /// Get the default synchronization strategy
    pub fn get_default_strategy(&self) -> &str {
        &self.default_strategy
    }

    /// Get all available synchronization strategies
    pub fn get_available_strategies(&self) -> Vec<String> {
        self.sync_strategies.keys().cloned().collect()
    }

    /// Get the current synchronization state
    pub async fn get_sync_state(&self) -> SyncState {
        let state = self.sync_state.lock().await;
        state.clone()
    }

    /// Check if a synchronization is currently running
    pub async fn is_syncing(&self) -> bool {
        let state = self.sync_state.lock().await;
        state.is_syncing
    }

    /// Start a full synchronization
    pub async fn start_full_sync(&self, direction: SyncDirection, strategy_name: Option<&str>) -> Result<()> {
        // Check if a sync is already running
        if self.is_syncing().await {
            let error_message = "A synchronization is already running";

            // Log the error with the error handling service if available
            if let Some(error_service) = &self.error_service {
                error_service.handle_error(
                    error_message.to_string(),
                    ErrorSeverity::Warning,
                    ErrorCategory::Synchronization,
                    "sync_manager".to_string(),
                    None,
                    None,
                    None,
                    false,
                    0,
                ).await?;
            }

            return Err(anyhow!(error_message));
        }

        // Update the sync state
        {
            let mut state = self.sync_state.lock().await;
            state.start_sync(direction.clone());
        }

        // Start the sync in a separate task
        let sync_state = self.sync_state.clone();
        let sync_service = self.sync_service.clone();
        let api_config = self.api_config.clone();
        let error_service = self.error_service.clone();
        let strategy_name_str = strategy_name.map(|s| s.to_string()).unwrap_or_else(|| self.default_strategy.clone());

        tokio::spawn(async move {
            // Create sync options
            let options = SyncOptions {
                force: false,
                dry_run: false,
                entity_types: vec![
                    "user".to_string(),
                    "course".to_string(),
                    "discussion".to_string(),
                    "comment".to_string(),
                    "tag".to_string(),
                ],
                specific_ids: None,
                sync_direction: direction,
            };

            // Update the sync state
            {
                let mut state = sync_state.lock().await;
                state.update_progress(0.1, "syncing_users");
            }

            // Sync users
            match Self::sync_entity_type("user", api_config.clone(), sync_service.clone(), options.clone(), error_service.clone(), &strategy_name_str).await {
                Ok(result) => {
                    let mut state = sync_state.lock().await;
                    state.add_result(result);
                    state.update_progress(0.3, "syncing_courses");
                },
                Err(e) => {
                    error!("Failed to sync users: {}", e);
                    let mut state = sync_state.lock().await;
                    state.update_progress(0.3, "syncing_courses");
                }
            }

            // Sync courses
            match Self::sync_entity_type("course", api_config.clone(), sync_service.clone(), options.clone(), error_service.clone(), &strategy_name_str).await {
                Ok(result) => {
                    let mut state = sync_state.lock().await;
                    state.add_result(result);
                    state.update_progress(0.5, "syncing_discussions");
                },
                Err(e) => {
                    error!("Failed to sync courses: {}", e);
                    let mut state = sync_state.lock().await;
                    state.update_progress(0.5, "syncing_discussions");
                }
            }

            // Sync discussions
            match Self::sync_entity_type("discussion", api_config.clone(), sync_service.clone(), options.clone(), error_service.clone(), &strategy_name_str).await {
                Ok(result) => {
                    let mut state = sync_state.lock().await;
                    state.add_result(result);
                    state.update_progress(0.7, "syncing_comments");
                },
                Err(e) => {
                    error!("Failed to sync discussions: {}", e);
                    let mut state = sync_state.lock().await;
                    state.update_progress(0.7, "syncing_comments");
                }
            }

            // Sync comments
            match Self::sync_entity_type("comment", api_config.clone(), sync_service.clone(), options.clone(), error_service.clone(), &strategy_name_str).await {
                Ok(result) => {
                    let mut state = sync_state.lock().await;
                    state.add_result(result);
                    state.update_progress(0.9, "syncing_tags");
                },
                Err(e) => {
                    error!("Failed to sync comments: {}", e);
                    let mut state = sync_state.lock().await;
                    state.update_progress(0.9, "syncing_tags");
                }
            }

            // Sync tags
            match Self::sync_entity_type("tag", api_config.clone(), sync_service.clone(), options.clone(), error_service.clone(), &strategy_name_str).await {
                Ok(result) => {
                    let mut state = sync_state.lock().await;
                    state.add_result(result);
                    state.complete_sync();
                },
                Err(e) => {
                    error!("Failed to sync tags: {}", e);
                    let mut state = sync_state.lock().await;
                    state.complete_sync();
                }
            }
        });

        Ok(())
    }

    /// Sync a specific entity
    pub async fn sync_entity(&self, entity_type: &str, entity_id: &str, direction: SyncDirection, strategy_name: Option<&str>) -> Result<SyncResult> {
        // Check if a sync is already running
        if self.is_syncing().await {
            let error_message = "A synchronization is already running";

            // Log the error with the error handling service if available
            if let Some(error_service) = &self.error_service {
                error_service.handle_error(
                    error_message.to_string(),
                    ErrorSeverity::Warning,
                    ErrorCategory::Synchronization,
                    "sync_manager".to_string(),
                    Some(entity_type.to_string()),
                    Some(entity_id.to_string()),
                    None,
                    false,
                    0,
                ).await?;
            }

            return Err(anyhow!(error_message));
        }

        // Update the sync state
        {
            let mut state = self.sync_state.lock().await;
            state.start_sync(direction.clone());
            state.update_entity(entity_type, entity_id);
        }

        // Create sync options
        let options = SyncOptions {
            force: false,
            dry_run: false,
            entity_types: vec![entity_type.to_string()],
            specific_ids: Some(vec![Uuid::parse_str(entity_id).map_err(|e| {
                // Log the error with the error handling service if available
                if let Some(error_service) = &self.error_service {
                    let _ = error_service.handle_error(
                        format!("Invalid UUID format for entity ID: {}", e),
                        ErrorSeverity::Error,
                        ErrorCategory::Validation,
                        "sync_manager".to_string(),
                        Some(entity_type.to_string()),
                        Some(entity_id.to_string()),
                        None,
                        false,
                        0,
                    );
                }

                anyhow!("Invalid UUID format for entity ID: {}", e)
            })?]),
            sync_direction: direction.clone(),
        };

        // Get the API clients
        let api_config_guard = self.api_config.lock().await;
        let canvas_client = api_config_guard.get_canvas_client()?;
        let discourse_client = api_config_guard.get_discourse_client()?;
        drop(api_config_guard);

        // Get the strategy to use
        let strategy_name = strategy_name.unwrap_or(&self.default_strategy);
        let strategy = self.sync_strategies.get(strategy_name).ok_or_else(|| {
            anyhow!("Synchronization strategy not found: {}", strategy_name)
        })?;

        // Check if the strategy supports this entity type
        if !strategy.supports_entity_type(entity_type) {
            let error_message = format!("Strategy '{}' does not support entity type: {}", strategy_name, entity_type);

            // Log the error with the error handling service if available
            if let Some(error_service) = &self.error_service {
                error_service.handle_error(
                    error_message.clone(),
                    ErrorSeverity::Error,
                    ErrorCategory::Validation,
                    "sync_manager".to_string(),
                    Some(entity_type.to_string()),
                    Some(entity_id.to_string()),
                    None,
                    false,
                    0,
                ).await?;
            }

            return Err(anyhow!(error_message));
        }

        // Sync the entity with the selected strategy
        let entity_uuid = Uuid::parse_str(entity_id)?;
        let result = if let Some(error_service) = &self.error_service {
            strategy.sync_entity(
                entity_type,
                entity_uuid,
                canvas_client,
                discourse_client,
                &options,
                Some(error_service.clone()),
            ).await.handle_error(
                error_service,
                &format!("Failed to sync entity {}: {}", entity_type, entity_id),
                ErrorSeverity::Error,
                ErrorCategory::Synchronization,
                "sync_manager",
                Some(entity_type),
                Some(entity_id),
                Some(&format!("Direction: {:?}, Strategy: {}", direction, strategy_name)),
                true,
                3,
            )
        } else {
            strategy.sync_entity(
                entity_type,
                entity_uuid,
                canvas_client,
                discourse_client,
                &options,
                None,
            ).await
        };

        // Update the sync state
        {
            let mut state = self.sync_state.lock().await;
            state.complete_sync();
        }

        result
    }

    /// Cancel the current synchronization
    pub async fn cancel_sync(&self) -> Result<()> {
        // Check if a sync is running
        if !self.is_syncing().await {
            return Err(anyhow!("No synchronization is currently running"));
        }

        // Update the sync state
        {
            let mut state = self.sync_state.lock().await;
            state.reset();
        }

        Ok(())
    }

    /// Sync an entity type using the specified strategy
    async fn sync_entity_type(
        entity_type: &str,
        api_config: Arc<Mutex<ApiConfigService>>,
        sync_service: Arc<SyncService>,
        options: SyncOptions,
        error_service: Option<Arc<ErrorHandlingService>>,
        strategy_name: &str,
    ) -> Result<SyncResult> {
        let started_at = Utc::now();
        let mut canvas_updates = 0;
        let mut discourse_updates = 0;
        let mut errors = Vec::new();

        // Get API clients
        let api_config_guard = api_config.lock().await;
        let canvas_client = api_config_guard.get_canvas_client()?;
        let discourse_client = api_config_guard.get_discourse_client()?;
        drop(api_config_guard); // Release the lock

        // Get the strategy
        let strategy: Box<dyn SyncStrategy + Send + Sync> = match strategy_name {
            "basic" => Box::new(sync_service.clone()),
            "incremental" => Box::new(IncrementalSyncService::new()),
            "bidirectional" => Box::new(BidirectionalSyncService::new(ConflictResolutionStrategy::MostRecent)),
            _ => Box::new(sync_service.clone()), // Default to basic strategy
        };

        // Check if the strategy supports this entity type
        if !strategy.supports_entity_type(entity_type) {
            return Err(anyhow!("Strategy '{}' does not support entity type: {}", strategy_name, entity_type));
        }

        // Sync the entity type
        let result = strategy.sync_entity(
            entity_type,
            Uuid::nil(), // Nil UUID for bulk sync
            canvas_client,
            discourse_client,
            &options,
            error_service,
        ).await?;

        Ok(result)
    }

    /// Sync users between Canvas and Discourse
    async fn sync_users(
        api_config: Arc<Mutex<ApiConfigService>>,
        sync_service: Arc<SyncService>,
        options: SyncOptions,
        error_service: Option<Arc<ErrorHandlingService>>,
    ) -> Result<SyncResult> {
        let started_at = Utc::now();
        let mut canvas_updates = 0;
        let mut discourse_updates = 0;
        let mut errors = Vec::new();

        // Get API clients
        let api_config_guard = api_config.lock().await;
        let canvas_client = api_config_guard.get_canvas_client()?;
        let discourse_client = api_config_guard.get_discourse_client()?;
        drop(api_config_guard); // Release the lock

        // Sync Canvas users to Discourse
        if options.sync_direction == SyncDirection::CanvasToDiscourse ||
           options.sync_direction == SyncDirection::Bidirectional {
            // Get Canvas users
            match canvas_client.get_courses(None).await {
                Ok(courses) => {
                    for course in courses {
                        match canvas_client.get_course_users(&course.id).await {
                            Ok(users) => {
                                for user in users {
                                    // Check if user exists in Discourse
                                    let email = match &user.email {
                                        Some(email) => email.clone(),
                                        None => {
                                            errors.push(format!("Canvas user {} has no email", user.id));
                                            continue;
                                        }
                                    };

                                    // Try to find user in Discourse by email
                                    let mut params = HashMap::new();
                                    params.insert("filter".to_string(), email.clone());

                                    match discourse_client.get_users(Some(params)).await {
                                        Ok(discourse_users) => {
                                            if discourse_users.is_empty() {
                                                // Create user in Discourse
                                                let username = email.split('@').next().unwrap_or(&email);
                                                let name = user.name.clone();

                                                let data = serde_json::json!({
                                                    "name": name,
                                                    "email": email,
                                                    "username": username,
                                                    "active": true,
                                                    "approved": true,
                                                });

                                                match discourse_client.create_user(&data).await {
                                                    Ok(_) => {
                                                        canvas_updates += 1;
                                                        info!("Created Discourse user for Canvas user {}", user.id);
                                                    },
                                                    Err(e) => {
                                                        errors.push(format!("Failed to create Discourse user for Canvas user {}: {}", user.id, e));
                                                    }
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            errors.push(format!("Failed to search for Discourse user by email {}: {}", email, e));
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                errors.push(format!("Failed to get users for course {}: {}", course.id, e));
                            }
                        }
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Canvas courses: {}", e));
                }
            }
        }

        // Sync Discourse users to Canvas
        if options.sync_direction == SyncDirection::DiscourseToCanvas ||
           options.sync_direction == SyncDirection::Bidirectional {
            // This is typically not done as Canvas users are usually managed by the institution
            // But we could implement it if needed
            discourse_updates = 0;
        }

        let completed_at = Utc::now();

        Ok(SyncResult {
            id: Uuid::new_v4(),
            entity_type: "user".to_string(),
            entity_id: Uuid::nil(), // Not applicable for bulk sync
            canvas_updates,
            discourse_updates,
            errors,
            status: if errors.is_empty() { SyncStatus::Synced } else { SyncStatus::Error },
            started_at,
            completed_at,
        })
    }

    /// Sync courses between Canvas and Discourse
    async fn sync_courses(
        api_config: Arc<Mutex<ApiConfigService>>,
        sync_service: Arc<SyncService>,
        options: SyncOptions,
        error_service: Option<Arc<ErrorHandlingService>>,
    ) -> Result<SyncResult> {
        let started_at = Utc::now();
        let mut canvas_updates = 0;
        let mut discourse_updates = 0;
        let mut errors = Vec::new();

        // Get API clients
        let api_config_guard = api_config.lock().await;
        let canvas_client = api_config_guard.get_canvas_client()?;
        let discourse_client = api_config_guard.get_discourse_client()?;
        drop(api_config_guard); // Release the lock

        // Sync Canvas courses to Discourse categories
        if options.sync_direction == SyncDirection::CanvasToDiscourse ||
           options.sync_direction == SyncDirection::Bidirectional {
            // Get Canvas courses
            match canvas_client.get_courses(None).await {
                Ok(courses) => {
                    for course in courses {
                        // Check if category exists in Discourse
                        let course_code = course.course_code.clone();

                        match discourse_client.get_categories().await {
                            Ok(categories) => {
                                let category_exists = categories.iter().any(|c| c.name == course.name);

                                if !category_exists {
                                    // Create category in Discourse
                                    let data = serde_json::json!({
                                        "name": course.name,
                                        "color": "0088CC",
                                        "text_color": "FFFFFF",
                                        "description": course.public_description.clone().unwrap_or_default(),
                                        "custom_fields": {
                                            "canvas_course_id": course.id,
                                            "canvas_course_code": course_code,
                                        }
                                    });

                                    match discourse_client.create_category(&data).await {
                                        Ok(_) => {
                                            canvas_updates += 1;
                                            info!("Created Discourse category for Canvas course {}", course.id);
                                        },
                                        Err(e) => {
                                            errors.push(format!("Failed to create Discourse category for Canvas course {}: {}", course.id, e));
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                errors.push(format!("Failed to get Discourse categories: {}", e));
                            }
                        }
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Canvas courses: {}", e));
                }
            }
        }

        // Sync Discourse categories to Canvas courses
        if options.sync_direction == SyncDirection::DiscourseToCanvas ||
           options.sync_direction == SyncDirection::Bidirectional {
            // This is typically not done as Canvas courses are usually managed by the institution
            // But we could implement it if needed
            discourse_updates = 0;
        }

        let completed_at = Utc::now();

        Ok(SyncResult {
            id: Uuid::new_v4(),
            entity_type: "course".to_string(),
            entity_id: Uuid::nil(), // Not applicable for bulk sync
            canvas_updates,
            discourse_updates,
            errors,
            status: if errors.is_empty() { SyncStatus::Synced } else { SyncStatus::Error },
            started_at,
            completed_at,
        })
    }

    /// Sync discussions between Canvas and Discourse
    async fn sync_discussions(
        api_config: Arc<Mutex<ApiConfigService>>,
        sync_service: Arc<SyncService>,
        options: SyncOptions,
        error_service: Option<Arc<ErrorHandlingService>>,
    ) -> Result<SyncResult> {
        let started_at = Utc::now();
        let mut canvas_updates = 0;
        let mut discourse_updates = 0;
        let mut errors = Vec::new();

        // Get API clients
        let api_config_guard = api_config.lock().await;
        let canvas_client = api_config_guard.get_canvas_client()?;
        let discourse_client = api_config_guard.get_discourse_client()?;
        drop(api_config_guard); // Release the lock

        // Sync Canvas discussions to Discourse topics
        if options.sync_direction == SyncDirection::CanvasToDiscourse ||
           options.sync_direction == SyncDirection::Bidirectional {
            // Get Canvas courses
            match canvas_client.get_courses(None).await {
                Ok(courses) => {
                    for course in courses {
                        // Get discussions for this course
                        match canvas_client.get_discussions(&course.id).await {
                            Ok(discussions) => {
                                for discussion in discussions {
                                    // Find the corresponding Discourse category
                                    match discourse_client.get_categories().await {
                                        Ok(categories) => {
                                            let category = categories.iter().find(|c| {
                                                if let Some(custom_fields) = &c.custom_fields {
                                                    if let Some(canvas_course_id) = custom_fields.get("canvas_course_id") {
                                                        if let Some(id_str) = canvas_course_id.as_str() {
                                                            return id_str == course.id;
                                                        }
                                                    }
                                                }
                                                false
                                            });

                                            if let Some(category) = category {
                                                // Check if topic exists in Discourse
                                                match discourse_client.get_topics_by_category(&category.id.to_string()).await {
                                                    Ok(topics) => {
                                                        let topic_exists = topics.iter().any(|t| {
                                                            if let Some(custom_fields) = &t.custom_fields {
                                                                if let Some(canvas_discussion_id) = custom_fields.get("canvas_discussion_id") {
                                                                    if let Some(id_str) = canvas_discussion_id.as_str() {
                                                                        return id_str == discussion.id;
                                                                    }
                                                                }
                                                            }
                                                            false
                                                        });

                                                        if !topic_exists {
                                                            // Create topic in Discourse
                                                            let title = discussion.title.clone();
                                                            let message = discussion.message.clone().unwrap_or_default();

                                                            match discourse_client.create_topic_structured(
                                                                &category.id.to_string(),
                                                                &title,
                                                                &message,
                                                                None,
                                                            ).await {
                                                                Ok(topic) => {
                                                                    // Update topic with custom fields
                                                                    let data = serde_json::json!({
                                                                        "custom_fields": {
                                                                            "canvas_discussion_id": discussion.id,
                                                                            "canvas_course_id": course.id,
                                                                        }
                                                                    });

                                                                    match discourse_client.update_topic(&topic.id.to_string(), &data).await {
                                                                        Ok(_) => {
                                                                            canvas_updates += 1;
                                                                            info!("Created Discourse topic for Canvas discussion {}", discussion.id);
                                                                        },
                                                                        Err(e) => {
                                                                            errors.push(format!("Failed to update Discourse topic with custom fields: {}", e));
                                                                        }
                                                                    }
                                                                },
                                                                Err(e) => {
                                                                    errors.push(format!("Failed to create Discourse topic for Canvas discussion {}: {}", discussion.id, e));
                                                                }
                                                            }
                                                        }
                                                    },
                                                    Err(e) => {
                                                        errors.push(format!("Failed to get Discourse topics for category {}: {}", category.id, e));
                                                    }
                                                }
                                            } else {
                                                errors.push(format!("No Discourse category found for Canvas course {}", course.id));
                                            }
                                        },
                                        Err(e) => {
                                            errors.push(format!("Failed to get Discourse categories: {}", e));
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                errors.push(format!("Failed to get discussions for course {}: {}", course.id, e));
                            }
                        }
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Canvas courses: {}", e));
                }
            }
        }

        // Sync Discourse topics to Canvas discussions
        if options.sync_direction == SyncDirection::DiscourseToCanvas ||
           options.sync_direction == SyncDirection::Bidirectional {
            // Get Discourse categories
            match discourse_client.get_categories().await {
                Ok(categories) => {
                    for category in categories {
                        // Check if this category is linked to a Canvas course
                        let canvas_course_id = if let Some(custom_fields) = &category.custom_fields {
                            if let Some(id) = custom_fields.get("canvas_course_id") {
                                if let Some(id_str) = id.as_str() {
                                    Some(id_str.to_string())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if let Some(course_id) = canvas_course_id {
                            // Get topics for this category
                            match discourse_client.get_topics_by_category(&category.id.to_string()).await {
                                Ok(topics) => {
                                    for topic in topics {
                                        // Check if this topic is already linked to a Canvas discussion
                                        let canvas_discussion_id = if let Some(custom_fields) = &topic.custom_fields {
                                            if let Some(id) = custom_fields.get("canvas_discussion_id") {
                                                if let Some(id_str) = id.as_str() {
                                                    Some(id_str.to_string())
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        };

                                        if canvas_discussion_id.is_none() {
                                            // Create discussion in Canvas
                                            let title = topic.title.clone();
                                            let message = if let Some(post) = &topic.first_post {
                                                post.raw.clone()
                                            } else {
                                                "".to_string()
                                            };

                                            match canvas_client.create_discussion(&course_id, &title, &message, true).await {
                                                Ok(discussion) => {
                                                    // Update topic with custom fields
                                                    let data = serde_json::json!({
                                                        "custom_fields": {
                                                            "canvas_discussion_id": discussion.id,
                                                            "canvas_course_id": course_id,
                                                        }
                                                    });

                                                    match discourse_client.update_topic(&topic.id.to_string(), &data).await {
                                                        Ok(_) => {
                                                            discourse_updates += 1;
                                                            info!("Created Canvas discussion for Discourse topic {}", topic.id);
                                                        },
                                                        Err(e) => {
                                                            errors.push(format!("Failed to update Discourse topic with custom fields: {}", e));
                                                        }
                                                    }
                                                },
                                                Err(e) => {
                                                    errors.push(format!("Failed to create Canvas discussion for Discourse topic {}: {}", topic.id, e));
                                                }
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    errors.push(format!("Failed to get Discourse topics for category {}: {}", category.id, e));
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Discourse categories: {}", e));
                }
            }
        }

        let completed_at = Utc::now();

        Ok(SyncResult {
            id: Uuid::new_v4(),
            entity_type: "discussion".to_string(),
            entity_id: Uuid::nil(), // Not applicable for bulk sync
            canvas_updates,
            discourse_updates,
            errors,
            status: if errors.is_empty() { SyncStatus::Synced } else { SyncStatus::Error },
            started_at,
            completed_at,
        })
    }

    /// Sync comments between Canvas and Discourse
    async fn sync_comments(
        api_config: Arc<Mutex<ApiConfigService>>,
        sync_service: Arc<SyncService>,
        options: SyncOptions,
        error_service: Option<Arc<ErrorHandlingService>>,
    ) -> Result<SyncResult> {
        let started_at = Utc::now();
        let mut canvas_updates = 0;
        let mut discourse_updates = 0;
        let mut errors = Vec::new();

        // Get API clients
        let api_config_guard = api_config.lock().await;
        let canvas_client = api_config_guard.get_canvas_client()?;
        let discourse_client = api_config_guard.get_discourse_client()?;
        drop(api_config_guard); // Release the lock

        // Sync Canvas discussion entries to Discourse posts
        if options.sync_direction == SyncDirection::CanvasToDiscourse ||
           options.sync_direction == SyncDirection::Bidirectional {
            // Get Canvas courses
            match canvas_client.get_courses(None).await {
                Ok(courses) => {
                    for course in courses {
                        // Get discussions for this course
                        match canvas_client.get_discussions(&course.id).await {
                            Ok(discussions) => {
                                for discussion in discussions {
                                    // Find the corresponding Discourse topic
                                    let discourse_topic_id = self.find_discourse_topic_for_canvas_discussion(
                                        &discourse_client,
                                        &discussion.id,
                                    ).await;

                                    if let Some(topic_id) = discourse_topic_id {
                                        // Get discussion entries
                                        match canvas_client.get_discussion_entries(&course.id, &discussion.id).await {
                                            Ok(entries) => {
                                                for entry in entries {
                                                    // Skip the first entry (it's the discussion itself)
                                                    if entry.parent_id.is_none() {
                                                        continue;
                                                    }

                                                    // Check if this entry is already synced to Discourse
                                                    let discourse_post_id = self.find_discourse_post_for_canvas_entry(
                                                        &discourse_client,
                                                        &entry.id,
                                                    ).await;

                                                    if discourse_post_id.is_none() {
                                                        // Create post in Discourse
                                                        let message = entry.message.clone();

                                                        // If this is a reply to another entry, find the corresponding Discourse post
                                                        let parent_post_id = if let Some(parent_id) = &entry.parent_id {
                                                            self.find_discourse_post_for_canvas_entry(
                                                                &discourse_client,
                                                                parent_id,
                                                            ).await
                                                        } else {
                                                            None
                                                        };

                                                        // Create the post
                                                        let create_result = if let Some(parent_id) = parent_post_id {
                                                            // Reply to a specific post
                                                            discourse_client.create_post_structured(
                                                                &topic_id,
                                                                &message,
                                                            ).await
                                                        } else {
                                                            // Reply to the topic
                                                            discourse_client.create_post_structured(
                                                                &topic_id,
                                                                &message,
                                                            ).await
                                                        };

                                                        match create_result {
                                                            Ok(post) => {
                                                                // Update post with custom fields
                                                                let data = serde_json::json!({
                                                                    "custom_fields": {
                                                                        "canvas_entry_id": entry.id,
                                                                        "canvas_discussion_id": discussion.id,
                                                                    }
                                                                });

                                                                match discourse_client.update_post(&post.id.to_string(), &data).await {
                                                                    Ok(_) => {
                                                                        canvas_updates += 1;
                                                                        info!("Created Discourse post for Canvas discussion entry {}", entry.id);
                                                                    },
                                                                    Err(e) => {
                                                                        errors.push(format!("Failed to update Discourse post with custom fields: {}", e));
                                                                    }
                                                                }
                                                            },
                                                            Err(e) => {
                                                                errors.push(format!("Failed to create Discourse post for Canvas discussion entry {}: {}", entry.id, e));
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                errors.push(format!("Failed to get entries for discussion {}: {}", discussion.id, e));
                                            }
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                errors.push(format!("Failed to get discussions for course {}: {}", course.id, e));
                            }
                        }
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Canvas courses: {}", e));
                }
            }
        }

        // Sync Discourse posts to Canvas discussion entries
        if options.sync_direction == SyncDirection::DiscourseToCanvas ||
           options.sync_direction == SyncDirection::Bidirectional {
            // Get Discourse categories
            match discourse_client.get_categories().await {
                Ok(categories) => {
                    for category in categories {
                        // Check if this category is linked to a Canvas course
                        let canvas_course_id = if let Some(custom_fields) = &category.custom_fields {
                            if let Some(id) = custom_fields.get("canvas_course_id") {
                                if let Some(id_str) = id.as_str() {
                                    Some(id_str.to_string())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if let Some(course_id) = canvas_course_id {
                            // Get topics for this category
                            match discourse_client.get_topics_by_category(&category.id.to_string()).await {
                                Ok(topics) => {
                                    for topic in topics {
                                        // Check if this topic is linked to a Canvas discussion
                                        let canvas_discussion_id = if let Some(custom_fields) = &topic.custom_fields {
                                            if let Some(id) = custom_fields.get("canvas_discussion_id") {
                                                if let Some(id_str) = id.as_str() {
                                                    Some(id_str.to_string())
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        };

                                        if let Some(discussion_id) = canvas_discussion_id {
                                            // Get posts for this topic
                                            match discourse_client.get_posts_by_topic(&topic.id.to_string()).await {
                                                Ok(posts) => {
                                                    for post in posts {
                                                        // Skip the first post (it's the topic itself)
                                                        if post.post_number <= 1 {
                                                            continue;
                                                        }

                                                        // Check if this post is already synced to Canvas
                                                        let canvas_entry_id = if let Some(custom_fields) = &post.custom_fields {
                                                            if let Some(id) = custom_fields.get("canvas_entry_id") {
                                                                if let Some(id_str) = id.as_str() {
                                                                    Some(id_str.to_string())
                                                                } else {
                                                                    None
                                                                }
                                                            } else {
                                                                None
                                                            }
                                                        } else {
                                                            None
                                                        };

                                                        if canvas_entry_id.is_none() {
                                                            // Create entry in Canvas
                                                            let message = post.raw.clone();

                                                            // If this is a reply to another post, find the corresponding Canvas entry
                                                            let parent_entry_id = if let Some(reply_to_post_number) = post.reply_to_post_number {
                                                                // Find the post it's replying to
                                                                let parent_post = posts.iter().find(|p| p.post_number == reply_to_post_number);

                                                                if let Some(parent) = parent_post {
                                                                    if let Some(custom_fields) = &parent.custom_fields {
                                                                        if let Some(id) = custom_fields.get("canvas_entry_id") {
                                                                            if let Some(id_str) = id.as_str() {
                                                                                Some(id_str.to_string())
                                                                            } else {
                                                                                None
                                                                            }
                                                                        } else {
                                                                            None
                                                                        }
                                                                    } else {
                                                                        None
                                                                    }
                                                                } else {
                                                                    None
                                                                }
                                                            } else {
                                                                None
                                                            };

                                                            // Create the entry
                                                            let create_result = if let Some(parent_id) = parent_entry_id {
                                                                // Reply to a specific entry
                                                                canvas_client.reply_to_discussion_entry(
                                                                    &course_id,
                                                                    &discussion_id,
                                                                    &parent_id,
                                                                    &message,
                                                                ).await
                                                            } else {
                                                                // Reply to the discussion
                                                                canvas_client.create_discussion_entry(
                                                                    &course_id,
                                                                    &discussion_id,
                                                                    &message,
                                                                ).await
                                                            };

                                                            match create_result {
                                                                Ok(entry) => {
                                                                    // Update post with custom fields
                                                                    let data = serde_json::json!({
                                                                        "custom_fields": {
                                                                            "canvas_entry_id": entry.id,
                                                                            "canvas_discussion_id": discussion_id,
                                                                        }
                                                                    });

                                                                    match discourse_client.update_post(&post.id.to_string(), &data).await {
                                                                        Ok(_) => {
                                                                            discourse_updates += 1;
                                                                            info!("Created Canvas discussion entry for Discourse post {}", post.id);
                                                                        },
                                                                        Err(e) => {
                                                                            errors.push(format!("Failed to update Discourse post with custom fields: {}", e));
                                                                        }
                                                                    }
                                                                },
                                                                Err(e) => {
                                                                    errors.push(format!("Failed to create Canvas discussion entry for Discourse post {}: {}", post.id, e));
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                Err(e) => {
                                                    errors.push(format!("Failed to get Discourse posts for topic {}: {}", topic.id, e));
                                                }
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    errors.push(format!("Failed to get Discourse topics for category {}: {}", category.id, e));
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Discourse categories: {}", e));
                }
            }
        }

        let completed_at = Utc::now();

        Ok(SyncResult {
            id: Uuid::new_v4(),
            entity_type: "comment".to_string(),
            entity_id: Uuid::nil(), // Not applicable for bulk sync
            canvas_updates,
            discourse_updates,
            errors,
            status: if errors.is_empty() { SyncStatus::Synced } else { SyncStatus::Error },
            started_at,
            completed_at,
        })
    }

    /// Sync tags between Canvas and Discourse
    async fn sync_tags(
        api_config: Arc<Mutex<ApiConfigService>>,
        sync_service: Arc<SyncService>,
        options: SyncOptions,
        error_service: Option<Arc<ErrorHandlingService>>,
    ) -> Result<SyncResult> {
        let started_at = Utc::now();
        let mut canvas_updates = 0;
        let mut discourse_updates = 0;
        let mut errors = Vec::new();

        // Get API clients
        let api_config_guard = api_config.lock().await;
        let discourse_client = api_config_guard.get_discourse_client()?;
        drop(api_config_guard); // Release the lock

        // Sync Discourse tags to local tags
        if options.sync_direction == SyncDirection::DiscourseToCanvas ||
           options.sync_direction == SyncDirection::Bidirectional {
            // Get Discourse tags
            match discourse_client.get_tags().await {
                Ok(tags) => {
                    for tag in tags {
                        // Create a mapping for this tag
                        let tag_id = Uuid::new_v4();

                        // Create a mapping
                        self.model_mapper.create_mapping(
                            "tag",
                            None,
                            Some(&tag.id),
                            tag_id,
                        );

                        discourse_updates += 1;
                        info!("Created mapping for Discourse tag {}", tag.name);
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Discourse tags: {}", e));
                }
            }

            // Get Discourse tag groups
            match discourse_client.get_tag_groups().await {
                Ok(tag_groups) => {
                    for tag_group in tag_groups {
                        // Create a mapping for this tag group
                        let tag_group_id = Uuid::new_v4();

                        // Create a mapping
                        self.model_mapper.create_mapping(
                            "tag_group",
                            None,
                            Some(&tag_group.id),
                            tag_group_id,
                        );

                        discourse_updates += 1;
                        info!("Created mapping for Discourse tag group {}", tag_group.name);
                    }
                },
                Err(e) => {
                    errors.push(format!("Failed to get Discourse tag groups: {}", e));
                }
            }
        }

        let completed_at = Utc::now();

        Ok(SyncResult {
            id: Uuid::new_v4(),
            entity_type: "tag".to_string(),
            entity_id: Uuid::nil(), // Not applicable for bulk sync
            canvas_updates,
            discourse_updates,
            errors,
            status: if errors.is_empty() { SyncStatus::Synced } else { SyncStatus::Error },
            started_at,
            completed_at,
        })
    }

    /// Find a Discourse topic for a Canvas discussion
    async fn find_discourse_topic_for_canvas_discussion(
        &self,
        discourse_client: &DiscourseApiClient,
        canvas_discussion_id: &str,
    ) -> Option<String> {
        // Get all categories
        match discourse_client.get_categories().await {
            Ok(categories) => {
                for category in categories {
                    // Get topics for this category
                    match discourse_client.get_topics_by_category(&category.id.to_string()).await {
                        Ok(topics) => {
                            for topic in topics {
                                if let Some(custom_fields) = &topic.custom_fields {
                                    if let Some(id) = custom_fields.get("canvas_discussion_id") {
                                        if let Some(id_str) = id.as_str() {
                                            if id_str == canvas_discussion_id {
                                                return Some(topic.id.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Err(_) => continue,
                    }
                }
            },
            Err(_) => return None,
        }

        None
    }

    /// Find a Discourse post for a Canvas discussion entry
    async fn find_discourse_post_for_canvas_entry(
        &self,
        discourse_client: &DiscourseApiClient,
        canvas_entry_id: &str,
    ) -> Option<String> {
        // This would require searching through all posts in all topics
        // which is not efficient. In a real implementation, we would use a database
        // to store these mappings.
        None
    }
}

/// Synchronization state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub is_syncing: bool,
    pub last_sync: Option<DateTime<Utc>>,
    pub current_sync_started: Option<DateTime<Utc>>,
    pub current_sync_progress: f32, // 0.0 to 1.0
    pub current_sync_stage: String,
    pub current_sync_entity_type: Option<String>,
    pub current_sync_entity_id: Option<String>,
    pub current_sync_direction: Option<SyncDirection>,
    pub current_sync_results: Vec<SyncResult>,
    pub error_count: usize,
    pub success_count: usize,
}

impl SyncState {
    /// Create a new synchronization state
    pub fn new() -> Self {
        Self {
            is_syncing: false,
            last_sync: None,
            current_sync_started: None,
            current_sync_progress: 0.0,
            current_sync_stage: "idle".to_string(),
            current_sync_entity_type: None,
            current_sync_entity_id: None,
            current_sync_direction: None,
            current_sync_results: Vec::new(),
            error_count: 0,
            success_count: 0,
        }
    }

    /// Reset the synchronization state
    pub fn reset(&mut self) {
        self.is_syncing = false;
        self.current_sync_started = None;
        self.current_sync_progress = 0.0;
        self.current_sync_stage = "idle".to_string();
        self.current_sync_entity_type = None;
        self.current_sync_entity_id = None;
        self.current_sync_direction = None;
        self.current_sync_results = Vec::new();
        self.error_count = 0;
        self.success_count = 0;
    }

    /// Start a new synchronization
    pub fn start_sync(&mut self, direction: SyncDirection) {
        self.reset();
        self.is_syncing = true;
        self.current_sync_started = Some(Utc::now());
        self.current_sync_stage = "starting".to_string();
        self.current_sync_direction = Some(direction);
    }

    /// Complete the synchronization
    pub fn complete_sync(&mut self) {
        self.is_syncing = false;
        self.last_sync = Some(Utc::now());
        self.current_sync_progress = 1.0;
        self.current_sync_stage = "completed".to_string();
    }

    /// Update the synchronization progress
    pub fn update_progress(&mut self, progress: f32, stage: &str) {
        self.current_sync_progress = progress;
        self.current_sync_stage = stage.to_string();
    }

    /// Update the current entity being synchronized
    pub fn update_entity(&mut self, entity_type: &str, entity_id: &str) {
        self.current_sync_entity_type = Some(entity_type.to_string());
        self.current_sync_entity_id = Some(entity_id.to_string());
    }

    /// Add a synchronization result
    pub fn add_result(&mut self, result: SyncResult) {
        if result.status == SyncStatus::Error {
            self.error_count += 1;
        } else {
            self.success_count += 1;
        }

        self.current_sync_results.push(result);
    }
}
