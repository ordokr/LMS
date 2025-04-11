use crate::shared::logger::Logger;
use crate::shared::db::Database;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use thiserror::Error;

/// Error types for the model mapper
#[derive(Error, Debug)]
pub enum MapperError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid entity type: {0}")]
    InvalidEntityType(String),
    
    #[error("Invalid system name: {0}")]
    InvalidSystemName(String),
    
    #[error("Mapping not found: {0}/{1}")]
    MappingNotFound(String, String),
    
    #[error("Caching error: {0}")]
    CachingError(String),
}

/// Entity mapping structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMapping {
    pub entity_type: String,
    pub source_id: String,
    pub source_system: String,
    pub target_system: String,
    pub target_id: String,
}

/// Cache key for entity mappings
type CacheKey = String;

/// Model Mapper
/// 
/// Maps data models between Canvas and Discourse systems,
/// providing bidirectional transformation and persistent mapping records.
pub struct ModelMapper {
    logger: Arc<Logger>,
    db: Arc<Database>,
    mapping_cache: Mutex<HashMap<CacheKey, EntityMapping>>,
}

impl ModelMapper {
    /// Create a new model mapper instance
    pub fn new(logger: Arc<Logger>, db: Arc<Database>) -> Self {
        ModelMapper {
            logger,
            db,
            mapping_cache: Mutex::new(HashMap::new()),
        }
    }
    
    /// Save a mapping between Canvas and Discourse entities
    /// 
    /// # Arguments
    /// 
    /// * `entity_type` - Type of entity (user, course, etc.)
    /// * `source_id` - ID in the source system (usually Canvas)
    /// * `target_id` - ID in the target system (usually Discourse)
    /// * `source_system` - Source system ('canvas' or 'discourse')
    /// 
    /// # Returns
    /// 
    /// * `Result<EntityMapping, MapperError>` - Created or updated mapping
    pub async fn save_mapping(
        &self,
        entity_type: &str,
        source_id: &str,
        target_id: &str,
        source_system: Option<&str>,
    ) -> Result<EntityMapping, MapperError> {
        let source_system = source_system.unwrap_or("canvas");
        self.validate_system_name(source_system)?;
        
        // Determine target system
        let target_system = if source_system == "canvas" { "discourse" } else { "canvas" };
        
        // Create cache key
        let cache_key = format!("{}:{}:{}", entity_type, source_id, source_system);
        
        // Check if mapping already exists
        let existing_mapping = self.db.find_entity_mapping(entity_type, source_id, source_system)
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))?;
        
        let mapping = if let Some(existing) = existing_mapping {
            // Update existing mapping
            self.db.update_entity_mapping(entity_type, source_id, source_system, target_id)
                .await
                .map_err(|e| MapperError::DatabaseError(e.to_string()))?;
            
            EntityMapping {
                entity_type: entity_type.to_string(),
                source_id: source_id.to_string(),
                source_system: source_system.to_string(),
                target_system: target_system.to_string(),
                target_id: target_id.to_string(),
            }
        } else {
            // Create new mapping
            self.db.create_entity_mapping(entity_type, source_id, source_system, target_system, target_id)
                .await
                .map_err(|e| MapperError::DatabaseError(e.to_string()))?
        };
        
        // Update cache
        let mut cache = self.mapping_cache.lock().await;
        cache.insert(cache_key, mapping.clone());
        
        Ok(mapping)
    }
    
    /// Get a mapping between Canvas and Discourse entities
    /// 
    /// # Arguments
    /// 
    /// * `entity_type` - Type of entity (user, course, etc.)
    /// * `source_id` - ID in the source system
    /// * `source_system` - Source system ('canvas' or 'discourse')
    /// 
    /// # Returns
    /// 
    /// * `Result<EntityMapping, MapperError>` - Retrieved mapping
    pub async fn get_mapping(
        &self,
        entity_type: &str,
        source_id: &str,
        source_system: Option<&str>,
    ) -> Result<EntityMapping, MapperError> {
        let source_system = source_system.unwrap_or("canvas");
        self.validate_system_name(source_system)?;
        
        // Create cache key
        let cache_key = format!("{}:{}:{}", entity_type, source_id, source_system);
        
        // Check cache first
        {
            let cache = self.mapping_cache.lock().await;
            if let Some(mapping) = cache.get(&cache_key) {
                return Ok(mapping.clone());
            }
        }
        
        // Cache miss - query the database
        let mapping = self.db.find_entity_mapping(entity_type, source_id, source_system)
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))?
            .ok_or_else(|| MapperError::MappingNotFound(entity_type.to_string(), source_id.to_string()))?;
        
        // Update cache
        let mut cache = self.mapping_cache.lock().await;
        cache.insert(cache_key, mapping.clone());
        
        Ok(mapping)
    }
    
    /// Delete a mapping between Canvas and Discourse entities
    /// 
    /// # Arguments
    /// 
    /// * `entity_type` - Type of entity (user, course, etc.)
    /// * `source_id` - ID in the source system
    /// * `source_system` - Source system ('canvas' or 'discourse')
    /// 
    /// # Returns
    /// 
    /// * `Result<bool, MapperError>` - True if mapping was deleted
    pub async fn delete_mapping(
        &self,
        entity_type: &str,
        source_id: &str,
        source_system: Option<&str>,
    ) -> Result<bool, MapperError> {
        let source_system = source_system.unwrap_or("canvas");
        self.validate_system_name(source_system)?;
        
        // Create cache key
        let cache_key = format!("{}:{}:{}", entity_type, source_id, source_system);
        
        // Delete from database
        let deleted = self.db.delete_entity_mapping(entity_type, source_id, source_system)
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))?;
        
        // Remove from cache if exists
        if deleted {
            let mut cache = self.mapping_cache.lock().await;
            cache.remove(&cache_key);
        }
        
        Ok(deleted)
    }
    
    /// Get the target ID for a source entity
    /// 
    /// # Arguments
    /// 
    /// * `entity_type` - Type of entity (user, course, etc.)
    /// * `source_id` - ID in the source system
    /// * `source_system` - Source system ('canvas' or 'discourse')
    /// 
    /// # Returns
    /// 
    /// * `Result<String, MapperError>` - Target ID
    pub async fn get_target_id(
        &self,
        entity_type: &str,
        source_id: &str,
        source_system: Option<&str>,
    ) -> Result<String, MapperError> {
        let mapping = self.get_mapping(entity_type, source_id, source_system).await?;
        Ok(mapping.target_id)
    }
    
    /// Find all mappings for an entity type
    /// 
    /// # Arguments
    /// 
    /// * `entity_type` - Type of entity (user, course, etc.)
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<EntityMapping>, MapperError>` - List of entity mappings
    pub async fn find_mappings_by_type(
        &self,
        entity_type: &str,
    ) -> Result<Vec<EntityMapping>, MapperError> {
        self.db.find_entity_mappings_by_type(entity_type)
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))
    }
    
    /// Clear the mapping cache
    pub async fn clear_cache(&self) {
        let mut cache = self.mapping_cache.lock().await;
        cache.clear();
    }
    
    /// Helper method to validate system names
    fn validate_system_name(&self, system_name: &str) -> Result<(), MapperError> {
        match system_name {
            "canvas" | "discourse" => Ok(()),
            _ => Err(MapperError::InvalidSystemName(system_name.to_string())),
        }
    }
}
