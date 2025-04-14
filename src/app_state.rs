use axum::http::Uri;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::repository::course_category_repository::CourseCategoryRepository;
use crate::repository::topic_mapping_repository::TopicMappingRepository;
use crate::auth::jwt_service::JwtService;
use crate::services::integration_service::IntegrationService;
use crate::services::sync_service::SyncService;
use crate::monitoring::sync_metrics::SyncMonitor;
use crate::services::canvas_client::CanvasClient;
use crate::services::file_storage_service::FileStorageService;
use crate::services::unified_auth_service::{UnifiedAuthService, OAuth2Config};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<Pool<Postgres>>,
    pub course_category_repo: CourseCategoryRepository,
    pub topic_mapping_repo: TopicMappingRepository,
    pub jwt_service: JwtService,
    pub integration_service: IntegrationService,
    pub sync_service: SyncService,
    pub sync_monitor: Arc<SyncMonitor>,
    pub auth_service: UnifiedAuthService,
    pub canvas_client: Arc<CanvasClient>,
    pub file_storage: Arc<FileStorageService>,
}

impl AppState {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_jwt_secret_do_not_use_in_production".to_string());
        
        let course_category_repo = CourseCategoryRepository::new(Arc::new(db_pool.clone()));
        let topic_mapping_repo = TopicMappingRepository::new(Arc::new(db_pool.clone()));
        let jwt_service = JwtService::new(jwt_secret.as_bytes());
        
        let integration_service = IntegrationService::new(
            CourseCategoryRepository::new(Arc::new(db_pool.clone())),
            JwtService::new(jwt_secret.as_bytes())
        );
        
        let sync_service = SyncService::new(
            TopicMappingRepository::new(Arc::new(db_pool.clone())),
            CourseCategoryRepository::new(Arc::new(db_pool.clone()))
        );
        
        // Initialize sync monitor with a capacity of 100 recent attempts
        let sync_monitor = Arc::new(SyncMonitor::new(100));
        
        // Initialize Canvas client
        let canvas_api_url = std::env::var("CANVAS_API_URL")
            .unwrap_or_else(|_| "https://canvas.example.com/api".to_string());
        let canvas_client = Arc::new(CanvasClient::new(&canvas_api_url));
        
        // Initialize OAuth2 configuration
        let oauth2_config = OAuth2Config {
            client_id: std::env::var("CANVAS_CLIENT_ID")
                .unwrap_or_else(|_| "canvas_client_id".to_string()),
            client_secret: std::env::var("CANVAS_CLIENT_SECRET")
                .unwrap_or_else(|_| "canvas_client_secret".to_string()),
            auth_url: std::env::var("CANVAS_AUTH_URL")
                .unwrap_or_else(|_| "https://canvas.example.com/oauth2/auth".to_string()),
            token_url: std::env::var("CANVAS_TOKEN_URL")
                .unwrap_or_else(|_| "https://canvas.example.com/oauth2/token".to_string()),
            redirect_url: std::env::var("CANVAS_REDIRECT_URL")
                .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
            scopes: vec!["read".to_string()],
        };
        
        // Initialize unified authentication service        let jwt_service_arc = Arc::new(JwtService::new(jwt_secret.as_bytes()));
        let auth_service = UnifiedAuthService::new(jwt_service_arc, oauth2_config);
        
        // Initialize file storage service
        let storage_path = std::env::var("FILE_STORAGE_PATH")
            .unwrap_or_else(|_| "./storage/files".to_string());
        let file_storage = Arc::new(FileStorageService::new(db_pool.clone(), &storage_path));
        
        Self {
            db_pool: Arc::new(db_pool),
            course_category_repo,
            topic_mapping_repo,
            jwt_service,
            integration_service,
            sync_service,
            sync_monitor,
            auth_service,
            canvas_client,
            file_storage,
        }
    }
}
