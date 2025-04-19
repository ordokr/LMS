use serde::{Serialize, Deserialize};

/// Network connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Connected to the network
    Online,
    
    /// Disconnected from the network
    Offline,
    
    /// Limited connectivity
    Limited,
}

impl ConnectionStatus {
    /// Check if the connection is online
    pub fn is_online(&self) -> bool {
        matches!(self, ConnectionStatus::Online)
    }
    
    /// Check if the connection is offline
    pub fn is_offline(&self) -> bool {
        matches!(self, ConnectionStatus::Offline)
    }
    
    /// Check if the connection has limited connectivity
    pub fn is_limited(&self) -> bool {
        matches!(self, ConnectionStatus::Limited)
    }
}

/// Network sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// All changes are synced
    Synced,
    
    /// Changes are being synced
    Syncing,
    
    /// Changes are pending sync
    Pending,
    
    /// Sync failed
    Failed,
}

impl SyncStatus {
    /// Check if all changes are synced
    pub fn is_synced(&self) -> bool {
        matches!(self, SyncStatus::Synced)
    }
    
    /// Check if changes are being synced
    pub fn is_syncing(&self) -> bool {
        matches!(self, SyncStatus::Syncing)
    }
    
    /// Check if changes are pending sync
    pub fn is_pending(&self) -> bool {
        matches!(self, SyncStatus::Pending)
    }
    
    /// Check if sync failed
    pub fn is_failed(&self) -> bool {
        matches!(self, SyncStatus::Failed)
    }
}

/// Network sync item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncItem {
    /// Unique ID for the sync item
    pub id: String,
    
    /// Type of sync operation
    pub operation: SyncOperation,
    
    /// Entity type
    pub entity_type: String,
    
    /// Entity ID
    pub entity_id: String,
    
    /// Payload data
    pub payload: serde_json::Value,
    
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Number of retry attempts
    pub retry_count: u32,
    
    /// Last retry timestamp
    pub last_retry: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Priority (lower number = higher priority)
    pub priority: u32,
}

/// Type of sync operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOperation {
    /// Create a new entity
    Create,
    
    /// Update an existing entity
    Update,
    
    /// Delete an entity
    Delete,
}

/// Network sync queue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncQueue {
    /// Items pending sync
    pub items: Vec<SyncItem>,
}

impl SyncQueue {
    /// Create a new empty sync queue
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    /// Add an item to the sync queue
    pub fn add_item(&mut self, item: SyncItem) {
        self.items.push(item);
    }
    
    /// Remove an item from the sync queue
    pub fn remove_item(&mut self, id: &str) {
        self.items.retain(|item| item.id != id);
    }
    
    /// Get the number of items in the queue
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    /// Get the next item to sync
    pub fn next_item(&self) -> Option<&SyncItem> {
        self.items.iter().min_by_key(|item| item.priority)
    }
    
    /// Get all items for a specific entity
    pub fn items_for_entity(&self, entity_type: &str, entity_id: &str) -> Vec<&SyncItem> {
        self.items
            .iter()
            .filter(|item| item.entity_type == entity_type && item.entity_id == entity_id)
            .collect()
    }
}
