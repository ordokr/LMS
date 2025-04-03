use serde::{Serialize, Deserialize};
use web_sys::{window, Storage};
use std::fmt::{self, Display};

use crate::models::forum::{Category, Topic, Post};
use crate::models::lms::{Course, Module, Assignment};

#[derive(Debug)]
pub enum LocalStorageError {
    WindowUnavailable,
    StorageUnavailable,
    SerializationError(String),
    DeserializationError(String),
    StorageError(String),
    NotFound,
}

impl Display for LocalStorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WindowUnavailable => write!(f, "Window object is not available"),
            Self::StorageUnavailable => write!(f, "LocalStorage is not available"),
            Self::SerializationError(msg) => write!(f, "Failed to serialize data: {}", msg),
            Self::DeserializationError(msg) => write!(f, "Failed to deserialize data: {}", msg),
            Self::StorageError(msg) => write!(f, "Storage operation failed: {}", msg),
            Self::NotFound => write!(f, "Item not found in storage"),
        }
    }
}

impl std::error::Error for LocalStorageError {}

#[derive(Clone)]
pub struct LocalStorage {
    storage: Option<Storage>,
}

impl LocalStorage {
    pub fn new() -> Self {
        let storage = window()
            .and_then(|window| window.local_storage().ok())
            .flatten();
        
        Self { storage }
    }
    
    /// Set an item in local storage
    pub fn set_item<T: Serialize>(&self, key: &str, value: &T) -> Result<(), LocalStorageError> {
        let storage = self.get_storage()?;
        
        let json_string = serde_json::to_string(value)
            .map_err(|e| LocalStorageError::SerializationError(e.to_string()))?;
        
        storage.set_item(key, &json_string)
            .map_err(|e| LocalStorageError::StorageError(format!("Failed to set item: {:?}", e)))?;
        
        Ok(())
    }
    
    /// Get an item from local storage
    pub fn get_item<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, LocalStorageError> {
        let storage = self.get_storage()?;
        
        match storage.get_item(key) {
            Ok(Some(value)) => {
                let deserialized = serde_json::from_str(&value)
                    .map_err(|e| LocalStorageError::DeserializationError(e.to_string()))?;
                Ok(Some(deserialized))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(LocalStorageError::StorageError(format!("Failed to get item: {:?}", e))),
        }
    }
    
    /// Remove an item from local storage
    pub fn remove_item(&self, key: &str) -> Result<(), LocalStorageError> {
        let storage = self.get_storage()?;
        
        storage.remove_item(key)
            .map_err(|e| LocalStorageError::StorageError(format!("Failed to remove item: {:?}", e)))?;
        
        Ok(())
    }
    
    /// Get the storage object
    fn get_storage(&self) -> Result<&Storage, LocalStorageError> {
        self.storage.as_ref().ok_or(LocalStorageError::StorageUnavailable)
    }
    
    // Entity-specific storage methods
    
    /// Store a forum category
    pub fn store_category(&self, category: &Category) -> Result<(), LocalStorageError> {
        self.set_item(&format!("category_{}", category.id), category)
    }
    
    /// Get a forum category
    pub fn get_category(&self, id: i64) -> Result<Option<Category>, LocalStorageError> {
        self.get_item(&format!("category_{}", id))
    }
    
    /// Store a forum topic
    pub fn store_topic(&self, topic: &Topic) -> Result<(), LocalStorageError> {
        self.set_item(&format!("topic_{}", topic.id), topic)
    }
    
    /// Get a forum topic
    pub fn get_topic(&self, id: i64) -> Result<Option<Topic>, LocalStorageError> {
        self.get_item(&format!("topic_{}", id))
    }
    
    /// Store a forum post
    pub fn store_post(&self, post: &Post) -> Result<(), LocalStorageError> {
        self.set_item(&format!("post_{}", post.id), post)
    }
    
    /// Get a forum post
    pub fn get_post(&self, id: i64) -> Result<Option<Post>, LocalStorageError> {
        self.get_item(&format!("post_{}", id))
    }
    
    /// Store a course
    pub fn store_course(&self, course: &Course) -> Result<(), LocalStorageError> {
        self.set_item(&format!("course_{}", course.id), course)
    }
    
    /// Get a course
    pub fn get_course(&self, id: i64) -> Result<Option<Course>, LocalStorageError> {
        self.get_item(&format!("course_{}", id))
    }
    
    /// Store a module
    pub fn store_module(&self, module: &Module) -> Result<(), LocalStorageError> {
        self.set_item(&format!("module_{}", module.id), module)
    }
    
    /// Get a module
    pub fn get_module(&self, id: i64) -> Result<Option<Module>, LocalStorageError> {
        self.get_item(&format!("module_{}", id))
    }
    
    /// Store an assignment
    pub fn store_assignment(&self, assignment: &Assignment) -> Result<(), LocalStorageError> {
        self.set_item(&format!("assignment_{}", assignment.id), assignment)
    }
    
    /// Get an assignment
    pub fn get_assignment(&self, id: i64) -> Result<Option<Assignment>, LocalStorageError> {
        self.get_item(&format!("assignment_{}", id))
    }
    
    /// Store ID mapping (local ID to server ID)
    pub fn store_id_mapping(&self, entity_type: &str, local_id: i64, server_id: i64) -> Result<(), LocalStorageError> {
        let key = format!("id_mapping_{}_{}", entity_type, local_id);
        self.set_item(&key, &server_id)
    }
    
    /// Get server ID from local ID
    pub fn get_server_id(&self, entity_type: &str, local_id: i64) -> Result<Option<i64>, LocalStorageError> {
        let key = format!("id_mapping_{}_{}", entity_type, local_id);
        self.get_item(&key)
    }
    
    /// Store relationship between course and forum category
    pub fn link_course_to_category(&self, course_id: i64, category_id: i64) -> Result<(), LocalStorageError> {
        // Store bidirectional links
        self.set_item(&format!("course_category_{}", course_id), &category_id)?;
        self.set_item(&format!("category_course_{}", category_id), &course_id)?;
        Ok(())
    }
    
    /// Get category ID for a course
    pub fn get_category_id_for_course(&self, course_id: i64) -> Result<Option<i64>, LocalStorageError> {
        self.get_item(&format!("course_category_{}", course_id))
    }
    
    /// Store relationship between module and forum topic
    pub fn link_module_to_topic(&self, module_id: i64, topic_id: i64) -> Result<(), LocalStorageError> {
        self.set_item(&format!("module_topic_{}", module_id), &topic_id)?;
        self.set_item(&format!("topic_module_{}", topic_id), &module_id)?;
        Ok(())
    }
    
    /// Get topic ID for a module
    pub fn get_topic_id_for_module(&self, module_id: i64) -> Result<Option<i64>, LocalStorageError> {
        self.get_item(&format!("module_topic_{}", module_id))
    }
    
    /// Store relationship between assignment and forum topic
    pub fn link_assignment_to_topic(&self, assignment_id: i64, topic_id: i64) -> Result<(), LocalStorageError> {
        self.set_item(&format!("assignment_topic_{}", assignment_id), &topic_id)?;
        self.set_item(&format!("topic_assignment_{}", topic_id), &assignment_id)?;
        Ok(())
    }
    
    /// Get topic ID for an assignment
    pub fn get_topic_id_for_assignment(&self, assignment_id: i64) -> Result<Option<i64>, LocalStorageError> {
        self.get_item(&format!("assignment_topic_{}", assignment_id))
    }
}