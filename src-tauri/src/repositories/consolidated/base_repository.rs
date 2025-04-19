use async_trait::async_trait;
use std::fmt::Debug;
use crate::errors::error::{Error, Result};

/// Generic repository interface for CRUD operations
#[async_trait]
pub trait Repository<T, ID>: Send + Sync + Debug {
    /// Find an entity by its ID
    async fn find_by_id(&self, id: &ID) -> Result<Option<T>>;
    
    /// Find all entities
    async fn find_all(&self) -> Result<Vec<T>>;
    
    /// Create a new entity
    async fn create(&self, entity: &T) -> Result<T>;
    
    /// Update an existing entity
    async fn update(&self, entity: &T) -> Result<T>;
    
    /// Delete an entity by its ID
    async fn delete(&self, id: &ID) -> Result<()>;
    
    /// Count all entities
    async fn count(&self) -> Result<i64>;
}

/// Repository interface for pagination
#[async_trait]
pub trait PaginatedRepository<T, ID>: Repository<T, ID> {
    /// Find entities with pagination
    async fn find_with_pagination(&self, page: i64, page_size: i64) -> Result<Vec<T>>;
    
    /// Count entities matching a filter
    async fn count_filtered(&self, filter: &str) -> Result<i64>;
}

/// Repository interface for filtering
#[async_trait]
pub trait FilteredRepository<T, ID>: Repository<T, ID> {
    /// Find entities matching a filter
    async fn find_by_filter(&self, filter: &str) -> Result<Vec<T>>;
    
    /// Find entities matching a filter with pagination
    async fn find_by_filter_with_pagination(&self, filter: &str, page: i64, page_size: i64) -> Result<Vec<T>>;
}

/// Repository interface for sorting
#[async_trait]
pub trait SortedRepository<T, ID>: Repository<T, ID> {
    /// Find entities with sorting
    async fn find_sorted(&self, sort_by: &str, ascending: bool) -> Result<Vec<T>>;
    
    /// Find entities with sorting and pagination
    async fn find_sorted_with_pagination(&self, sort_by: &str, ascending: bool, page: i64, page_size: i64) -> Result<Vec<T>>;
}

/// Repository interface for full-featured repositories
#[async_trait]
pub trait FullRepository<T, ID>: Repository<T, ID> + PaginatedRepository<T, ID> + FilteredRepository<T, ID> + SortedRepository<T, ID> {
    /// Find entities with filtering, sorting, and pagination
    async fn find_with_filter_sort_pagination(
        &self,
        filter: &str,
        sort_by: &str,
        ascending: bool,
        page: i64,
        page_size: i64
    ) -> Result<Vec<T>>;
}

/// Repository configuration
#[derive(Debug, Clone)]
pub struct RepositoryConfig {
    /// Repository name
    pub name: String,
    
    /// Database connection string
    pub connection_string: Option<String>,
    
    /// Additional configuration parameters
    pub parameters: std::collections::HashMap<String, String>,
}

impl RepositoryConfig {
    /// Create a new repository configuration
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            connection_string: None,
            parameters: std::collections::HashMap::new(),
        }
    }
    
    /// Set the database connection string
    pub fn with_connection_string(mut self, connection_string: &str) -> Self {
        self.connection_string = Some(connection_string.to_string());
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
}

/// Repository factory for creating repositories
#[async_trait]
pub trait RepositoryFactory: Send + Sync + Debug {
    /// Create a repository
    async fn create_repository<T, ID>(&self, config: RepositoryConfig) -> Result<Box<dyn Repository<T, ID>>>
    where
        T: Send + Sync + 'static,
        ID: Send + Sync + 'static;
}

/// Repository registry for managing repositories
#[derive(Debug, Default)]
pub struct RepositoryRegistry {
    /// Registered repositories
    repositories: std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl RepositoryRegistry {
    /// Create a new repository registry
    pub fn new() -> Self {
        Self {
            repositories: std::collections::HashMap::new(),
        }
    }
    
    /// Register a repository
    pub fn register<T, ID, R>(&mut self, name: &str, repository: R) -> Result<()>
    where
        T: Send + Sync + 'static,
        ID: Send + Sync + 'static,
        R: Repository<T, ID> + 'static,
    {
        if self.repositories.contains_key(name) {
            return Err(Error::conflict(format!("Repository '{}' already registered", name)));
        }
        
        self.repositories.insert(name.to_string(), Box::new(repository));
        Ok(())
    }
    
    /// Get a repository by name
    pub fn get<T, ID, R>(&self, name: &str) -> Result<&R>
    where
        T: Send + Sync + 'static,
        ID: Send + Sync + 'static,
        R: Repository<T, ID> + 'static,
    {
        let repository = self.repositories.get(name)
            .ok_or_else(|| Error::not_found(format!("Repository '{}' not found", name)))?;
        
        repository.downcast_ref::<R>()
            .ok_or_else(|| Error::internal(format!("Repository '{}' is not of the expected type", name)))
    }
}

/// Get the global repository registry
pub fn get_repository_registry() -> &'static RepositoryRegistry {
    use once_cell::sync::Lazy;
    
    static REPOSITORY_REGISTRY: Lazy<RepositoryRegistry> = Lazy::new(RepositoryRegistry::new);
    
    &REPOSITORY_REGISTRY
}
