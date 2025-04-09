use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::repository::course_category_repository::CourseCategoryRepository;
use crate::repository::topic_mapping_repository::TopicMappingRepository;
use crate::auth::jwt_service::JwtService;
use crate::services::integration_service::IntegrationService;
use crate::services::sync_service::SyncService;
use crate::monitoring::sync_metrics::SyncMonitor;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<Pool<Postgres>>,
    pub course_category_repo: CourseCategoryRepository,
    pub topic_mapping_repo: TopicMappingRepository,
    pub jwt_service: JwtService,
    pub integration_service: IntegrationService,
    pub sync_service: SyncService,
    pub sync_monitor: Arc<SyncMonitor>,
    // Additional fields as needed
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
        
        Self {
            db_pool: Arc::new(db_pool),
            course_category_repo,
            topic_mapping_repo,
            jwt_service,
            integration_service,
            sync_service,
            sync_monitor,
        }
    }
}