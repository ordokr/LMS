use async_trait::async_trait;
use crate::error::Error;

/// Generic repository interface for CRUD operations
#[async_trait]
pub trait Repository<T, ID> {
    /// Find an entity by its ID
    async fn find_by_id(&self, id: &ID) -> Result<Option<T>, Error>;
    
    /// Find all entities
    async fn find_all(&self) -> Result<Vec<T>, Error>;
    
    /// Create a new entity
    async fn create(&self, entity: &T) -> Result<T, Error>;
    
    /// Update an existing entity
    async fn update(&self, entity: &T) -> Result<T, Error>;
    
    /// Delete an entity by its ID
    async fn delete(&self, id: &ID) -> Result<(), Error>;
    
    /// Count all entities
    async fn count(&self) -> Result<i64, Error>;
}

/// Generic repository interface for pagination
#[async_trait]
pub trait PaginatedRepository<T, ID>: Repository<T, ID> {
    /// Find entities with pagination
    async fn find_with_pagination(&self, page: i64, page_size: i64) -> Result<Vec<T>, Error>;
    
    /// Count entities matching a filter
    async fn count_filtered(&self, filter: &str) -> Result<i64, Error>;
}

/// Generic repository interface for filtering
#[async_trait]
pub trait FilteredRepository<T, ID>: Repository<T, ID> {
    /// Find entities matching a filter
    async fn find_by_filter(&self, filter: &str) -> Result<Vec<T>, Error>;
    
    /// Find entities matching a filter with pagination
    async fn find_by_filter_with_pagination(&self, filter: &str, page: i64, page_size: i64) -> Result<Vec<T>, Error>;
}

/// Generic repository interface for sorting
#[async_trait]
pub trait SortedRepository<T, ID>: Repository<T, ID> {
    /// Find entities with sorting
    async fn find_sorted(&self, sort_by: &str, ascending: bool) -> Result<Vec<T>, Error>;
    
    /// Find entities with sorting and pagination
    async fn find_sorted_with_pagination(&self, sort_by: &str, ascending: bool, page: i64, page_size: i64) -> Result<Vec<T>, Error>;
}

/// Generic repository interface for full-featured repositories
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
    ) -> Result<Vec<T>, Error>;
}
