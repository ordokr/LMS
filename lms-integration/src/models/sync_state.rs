//! Synchronization State model
//!
//! Manages the mapping between Canvas and Discourse entities and tracks synchronization status

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type definition for entity ID mapping
pub type EntityMapping = HashMap<String, String>;

/// Synchronization state between Canvas and Discourse
pub struct SyncState {
    /// Maps Canvas course IDs to Discourse category IDs
    course_category_map: RwLock<EntityMapping>,
    
    /// Maps Canvas module IDs to Discourse subcategory IDs
    module_subcategory_map: RwLock<EntityMapping>,
    
    /// Maps Canvas assignment IDs to Discourse topic IDs
    assignment_topic_map: RwLock<EntityMapping>,
    
    /// Maps Canvas user IDs to Discourse user IDs
    user_map: RwLock<EntityMapping>,
}

impl SyncState {
    /// Create a new synchronization state
    pub fn new() -> Self {
        Self {
            course_category_map: RwLock::new(HashMap::new()),
            module_subcategory_map: RwLock::new(HashMap::new()),
            assignment_topic_map: RwLock::new(HashMap::new()),
            user_map: RwLock::new(HashMap::new()),
        }
    }
    
    /// Load synchronization state from database or file
    pub async fn load_from_storage() -> Result<Self> {
        // TODO: Implement loading from database
        Ok(Self::new())
    }
    
    /// Save synchronization state to database or file
    pub async fn save_to_storage(&self) -> Result<()> {
        // TODO: Implement saving to database
        Ok(())
    }
    
    // Course-Category mapping methods
    
    /// Get Discourse category ID for a Canvas course ID
    pub fn get_category_for_course(&self, course_id: &str) -> Option<String> {
        self.course_category_map
            .read()
            .unwrap()
            .get(course_id)
            .cloned()
    }
    
    /// Set mapping between Canvas course and Discourse category
    pub fn set_course_category_mapping(&self, course_id: &str, category_id: &str) -> Result<()> {
        let mut map = self.course_category_map.write().unwrap();
        map.insert(course_id.to_string(), category_id.to_string());
        Ok(())
    }
    
    // Module-Subcategory mapping methods
    
    /// Get Discourse subcategory ID for a Canvas module ID
    pub fn get_subcategory_for_module(&self, module_id: &str) -> Option<String> {
        self.module_subcategory_map
            .read()
            .unwrap()
            .get(module_id)
            .cloned()
    }
    
    /// Set mapping between Canvas module and Discourse subcategory
    pub fn set_module_subcategory_mapping(&self, module_id: &str, subcategory_id: &str) -> Result<()> {
        let mut map = self.module_subcategory_map.write().unwrap();
        map.insert(module_id.to_string(), subcategory_id.to_string());
        Ok(())
    }
    
    // Assignment-Topic mapping methods
    
    /// Get Discourse topic ID for a Canvas assignment ID
    pub fn get_topic_for_assignment(&self, assignment_id: &str) -> Option<String> {
        self.assignment_topic_map
            .read()
            .unwrap()
            .get(assignment_id)
            .cloned()
    }
    
    /// Set mapping between Canvas assignment and Discourse topic
    pub fn set_assignment_topic_mapping(&self, assignment_id: &str, topic_id: &str) -> Result<()> {
        let mut map = self.assignment_topic_map.write().unwrap();
        map.insert(assignment_id.to_string(), topic_id.to_string());
        Ok(())
    }
    
    // User mapping methods
    
    /// Get Discourse user ID for a Canvas user ID
    pub fn get_discourse_user_id(&self, canvas_user_id: &str) -> Option<String> {
        self.user_map
            .read()
            .unwrap()
            .get(canvas_user_id)
            .cloned()
    }
    
    /// Set mapping between Canvas user and Discourse user
    pub fn set_user_mapping(&self, canvas_user_id: &str, discourse_user_id: &str) -> Result<()> {
        let mut map = self.user_map.write().unwrap();
        map.insert(canvas_user_id.to_string(), discourse_user_id.to_string());
        Ok(())
    }
}
