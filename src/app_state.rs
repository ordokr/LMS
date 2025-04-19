use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use sqlx::PgPool;
use crate::services::{ModelMapperService, SyncService, ModelConversionService, ApiConfigService, SyncManager, ErrorHandlingService, RecoveryService};
use tokio::sync::Mutex as TokioMutex;

pub struct AppState {
    pub users: Mutex<HashMap<String, String>>, // email -> password
    pub courses: Mutex<HashMap<String, (String, String, Option<String>)>>, // id -> (name, code, description)
    pub assignments: Mutex<HashMap<String, (String, String, u32)>>, // id -> (title, description, points)
    pub db_pool: PgPool,
    pub api_config: Arc<TokioMutex<ApiConfigService>>,
    pub model_mapper: Arc<ModelMapperService>,
    pub sync_service: Arc<SyncService>,
    pub model_conversion: Arc<ModelConversionService>,
    pub sync_manager: Arc<SyncManager>,
    pub error_service: Arc<ErrorHandlingService>,
    pub recovery_service: Arc<RecoveryService>,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Arc<Self> {
        // Create the model mapper service
        let model_mapper = Arc::new(ModelMapperService::new());

        // Create the API config service
        let api_config = match ApiConfigService::from_file("config/api_config.json") {
            Ok(config) => {
                let mut config = config;
                if let Err(e) = config.initialize() {
                    log::error!("Failed to initialize API clients: {}", e);
                }
                Arc::new(TokioMutex::new(config))
            },
            Err(e) => {
                log::error!("Failed to load API config: {}", e);
                Arc::new(TokioMutex::new(ApiConfigService::from_config(crate::services::ApiConfig {
                    canvas: crate::services::CanvasConfig {
                        base_url: "https://canvas.example.com/api/v1".to_string(),
                        api_token: "your_canvas_api_token".to_string(),
                        timeout_seconds: Some(30),
                    },
                    discourse: crate::services::DiscourseConfig {
                        base_url: "https://discourse.example.com".to_string(),
                        api_key: "your_discourse_api_key".to_string(),
                        api_username: "system".to_string(),
                        timeout_seconds: Some(30),
                    },
                }).unwrap()))
            }
        };

        // Create the sync service
        let sync_service = Arc::new(SyncService::new(model_mapper.clone()));

        // Create the model conversion service
        let model_conversion = Arc::new(ModelConversionService::new(model_mapper.clone()));

        // Create the error handling service
        let error_service = Arc::new(ErrorHandlingService::new());

        // Register error handlers
        let api_connection_handler = Box::new(crate::services::error_handling_service::ApiConnectionErrorHandler::new(5));
        let sync_handler = Box::new(crate::services::error_handling_service::SynchronizationErrorHandler::new(3));
        let db_handler = Box::new(crate::services::error_handling_service::DatabaseErrorHandler);
        let config_handler = Box::new(crate::services::error_handling_service::ConfigurationErrorHandler);
        let system_handler = Box::new(crate::services::error_handling_service::SystemErrorHandler);

        tokio::spawn(async move {
            let error_service_clone = error_service.clone();
            error_service_clone.register_handler(api_connection_handler).await;
            error_service_clone.register_handler(sync_handler).await;
            error_service_clone.register_handler(db_handler).await;
            error_service_clone.register_handler(config_handler).await;
            error_service_clone.register_handler(system_handler).await;
        });

        // Create the sync manager
        let sync_manager = Arc::new(SyncManager::new(
            api_config.clone(),
            model_mapper.clone(),
            sync_service.clone(),
            model_conversion.clone(),
        ));

        // Create the recovery service
        let recovery_service = Arc::new(RecoveryService::new(
            error_service.clone(),
            sync_manager.clone(),
            api_config.clone(),
        ));

        // Start the recovery service
        let recovery_service_clone = recovery_service.clone();
        tokio::spawn(async move {
            if let Err(e) = recovery_service_clone.start().await {
                log::error!("Failed to start recovery service: {}", e);
            }
        });

        Arc::new(Self {
            users: Mutex::new(HashMap::new()),
            courses: Mutex::new(HashMap::new()),
            assignments: Mutex::new(HashMap::new()),
            db_pool,
            api_config,
            model_mapper,
            sync_service,
            model_conversion,
            sync_manager,
            error_service,
            recovery_service,
        })
    }
}
