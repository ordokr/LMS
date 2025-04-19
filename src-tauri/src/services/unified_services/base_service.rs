use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use crate::errors::error::{Error, Result};

/// Base service trait that all services should implement
#[async_trait]
pub trait Service: Send + Sync + Debug {
    /// Get the name of the service
    fn name(&self) -> &str;
    
    /// Initialize the service
    async fn init(&self) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }
    
    /// Shutdown the service
    async fn shutdown(&self) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }
    
    /// Check if the service is healthy
    async fn health_check(&self) -> Result<bool> {
        // Default implementation returns true
        Ok(true)
    }
}

/// Base service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Database connection pool
    pub db_pool: Option<Pool<Sqlite>>,
    
    /// Service name
    pub name: String,
    
    /// Whether the service is enabled
    pub enabled: bool,
    
    /// Additional configuration parameters
    pub parameters: std::collections::HashMap<String, String>,
}

impl ServiceConfig {
    /// Create a new service configuration
    pub fn new(name: &str) -> Self {
        Self {
            db_pool: None,
            name: name.to_string(),
            enabled: true,
            parameters: std::collections::HashMap::new(),
        }
    }
    
    /// Set the database connection pool
    pub fn with_db_pool(mut self, db_pool: Pool<Sqlite>) -> Self {
        self.db_pool = Some(db_pool);
        self
    }
    
    /// Set whether the service is enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Add a configuration parameter
    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Get a configuration parameter
    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }
    
    /// Get a configuration parameter as a specific type
    pub fn get_parameter_as<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
        self.parameters.get(key).and_then(|value| value.parse::<T>().ok())
    }
    
    /// Get the database connection pool
    pub fn get_db_pool(&self) -> Result<&Pool<Sqlite>> {
        self.db_pool.as_ref().ok_or_else(|| Error::internal("Database connection pool not configured"))
    }
}

/// Base service implementation
#[derive(Debug)]
pub struct BaseService {
    /// Service configuration
    config: ServiceConfig,
}

impl BaseService {
    /// Create a new base service
    pub fn new(config: ServiceConfig) -> Self {
        Self { config }
    }
    
    /// Get the service configuration
    pub fn config(&self) -> &ServiceConfig {
        &self.config
    }
}

#[async_trait]
impl Service for BaseService {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    async fn init(&self) -> Result<()> {
        // Base implementation does nothing
        Ok(())
    }
    
    async fn shutdown(&self) -> Result<()> {
        // Base implementation does nothing
        Ok(())
    }
    
    async fn health_check(&self) -> Result<bool> {
        // Check if database connection is available
        if let Some(db_pool) = &self.config.db_pool {
            let result = sqlx::query("SELECT 1").execute(db_pool).await;
            Ok(result.is_ok())
        } else {
            // No database connection, assume healthy
            Ok(true)
        }
    }
}

/// Service registry for managing services
#[derive(Debug, Default)]
pub struct ServiceRegistry {
    /// Registered services
    services: std::collections::HashMap<String, Arc<dyn Service>>,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new() -> Self {
        Self {
            services: std::collections::HashMap::new(),
        }
    }
    
    /// Register a service
    pub fn register<S: Service + 'static>(&mut self, service: S) -> Result<()> {
        let name = service.name().to_string();
        if self.services.contains_key(&name) {
            return Err(Error::conflict(format!("Service '{}' already registered", name)));
        }
        
        self.services.insert(name, Arc::new(service));
        Ok(())
    }
    
    /// Get a service by name
    pub fn get<S: Service + 'static>(&self, name: &str) -> Result<Arc<S>> {
        let service = self.services.get(name).ok_or_else(|| Error::not_found(format!("Service '{}' not found", name)))?;
        
        // Downcast to the specific service type
        let service = Arc::clone(service);
        let service = Arc::downcast::<S>(service)
            .map_err(|_| Error::internal(format!("Service '{}' is not of the expected type", name)))?;
            
        Ok(service)
    }
    
    /// Get all registered services
    pub fn get_all(&self) -> Vec<Arc<dyn Service>> {
        self.services.values().cloned().collect()
    }
    
    /// Initialize all services
    pub async fn init_all(&self) -> Result<()> {
        for service in self.services.values() {
            service.init().await?;
        }
        
        Ok(())
    }
    
    /// Shutdown all services
    pub async fn shutdown_all(&self) -> Result<()> {
        for service in self.services.values() {
            service.shutdown().await?;
        }
        
        Ok(())
    }
    
    /// Check if all services are healthy
    pub async fn health_check_all(&self) -> Result<bool> {
        for service in self.services.values() {
            if !service.health_check().await? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}

/// Get the global service registry
pub fn get_service_registry() -> &'static ServiceRegistry {
    use once_cell::sync::Lazy;
    
    static SERVICE_REGISTRY: Lazy<ServiceRegistry> = Lazy::new(ServiceRegistry::new);
    
    &SERVICE_REGISTRY
}

/// Initialize the service registry with the given services
pub async fn init_services<I>(services: I) -> Result<()>
where
    I: IntoIterator,
    I::Item: Service + 'static,
{
    let registry = get_service_registry();
    
    // Register services
    for service in services {
        let mut registry = SERVICE_REGISTRY.write().await;
        registry.register(service)?;
    }
    
    // Initialize services
    registry.init_all().await
}

/// Shutdown all services
pub async fn shutdown_services() -> Result<()> {
    let registry = get_service_registry();
    registry.shutdown_all().await
}

/// Check if all services are healthy
pub async fn health_check_services() -> Result<bool> {
    let registry = get_service_registry();
    registry.health_check_all().await
}

/// Get a service by name
pub async fn get_service<S: Service + 'static>(name: &str) -> Result<Arc<S>> {
    let registry = get_service_registry();
    registry.get::<S>(name)
}
