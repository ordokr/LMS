use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Represents the synchronization state across the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub last_sync_timestamp: i64, // Unix timestamp
    pub entities: HashMap<String, HashMap<i64, EntityStatus>>,
}

/// Represents the sync status for a specific entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityStatus {
    /// Entity exists locally and on server (fully synced)
    Synced,
    /// Entity has been created locally but not yet on server
    PendingCreate,
    /// Entity exists on server but has local modifications
    PendingUpdate,
    /// Entity has been marked for deletion locally but not deleted on server
    PendingDelete,
    /// Entity exists only on server (not yet downloaded)
    RemoteOnly,
}

impl SyncState {
    pub fn new() -> Self {
        Self {
            last_sync_timestamp: 0,
            entities: HashMap::new(),
        }
    }
    
    /// Record a new entity that needs to be synced
    pub fn record_pending_create(&mut self, entity_type: &str, local_id: i64) {
        let entity_map = self.entities
            .entry(entity_type.to_string())
            .or_insert_with(HashMap::new);
        
        entity_map.insert(local_id, EntityStatus::PendingCreate);
    }
    
    /// Record an updated entity that needs to be synced
    pub fn record_pending_update(&mut self, entity_type: &str, id: i64) {
        let entity_map = self.entities
            .entry(entity_type.to_string())
            .or_insert_with(HashMap::new);
        
        entity_map.insert(id, EntityStatus::PendingUpdate);
    }
    
    /// Record an entity that has been deleted locally
    pub fn record_pending_delete(&mut self, entity_type: &str, id: i64) {
        let entity_map = self.entities
            .entry(entity_type.to_string())
            .or_insert_with(HashMap::new);
        
        entity_map.insert(id, EntityStatus::PendingDelete);
    }
    
    /// Mark an entity as synced
    pub fn mark_synced(&mut self, entity_type: &str, id: i64) {
        if let Some(entity_map) = self.entities.get_mut(entity_type) {
            entity_map.insert(id, EntityStatus::Synced);
        }
    }
    
    /// Remove a synced entity from tracking
    pub fn remove_synced(&mut self, entity_type: &str, id: i64) {
        if let Some(entity_map) = self.entities.get_mut(entity_type) {
            entity_map.remove(&id);
        }
    }
    
    /// Get entities of a specific type with a specific status
    pub fn get_entities_with_status(&self, entity_type: &str, status: EntityStatus) -> Vec<i64> {
        match self.entities.get(entity_type) {
            Some(entity_map) => {
                entity_map
                    .iter()
                    .filter_map(|(id, s)| if *s == status { Some(*id) } else { None })
                    .collect()
            }
            None => Vec::new(),
        }
    }
    
    /// Check if there are any pending changes to sync
    pub fn has_pending_changes(&self) -> bool {
        for entity_map in self.entities.values() {
            for status in entity_map.values() {
                match status {
                    EntityStatus::PendingCreate | EntityStatus::PendingUpdate | EntityStatus::PendingDelete => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }
    
    /// Update the last sync timestamp
    pub fn update_sync_timestamp(&mut self, timestamp: i64) {
        self.last_sync_timestamp = timestamp;
    }
}