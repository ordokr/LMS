use gloo_storage::{LocalStorage, Storage};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::Navigator;

use super::auth::get_auth_token;
use super::offline::is_online;

const SYNC_QUEUE_KEY: &str = "sync_queue";
const DEVICE_ID_KEY: &str = "device_id";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    pub id: String,
    pub operation_type: String, // "create", "update", "delete", "reference"
    pub entity_type: String,    // "course", "assignment", "forum_post", etc.
    pub entity_id: Option<String>,
    pub payload: serde_json::Value,
    pub timestamp: i64,
    pub synced: bool,
}

pub struct SyncClient {
    device_id: String,
}

impl SyncClient {
    pub fn new() -> Self {
        let device_id = match LocalStorage::get(DEVICE_ID_KEY) {
            Ok(id) => id,
            Err(_) => {
                // Generate a new device ID
                let id = uuid::Uuid::new_v4().to_string();
                let _ = LocalStorage::set(DEVICE_ID_KEY, &id);
                id
            }
        };
        
        Self { device_id }
    }
    
    // Queue an operation for sync
    pub fn queue_operation(&self, operation_type: &str, entity_type: &str, entity_id: Option<&str>, payload: serde_json::Value) -> Result<(), String> {
        let mut queue = self.get_queue()?;
        
        let now = js_sys::Date::now() as i64;
        let id = uuid::Uuid::new_v4().to_string();
        
        let operation = SyncOperation {
            id,
            operation_type: operation_type.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.map(ToString::to_string),
            payload,
            timestamp: now,
            synced: false,
        };
        
        queue.push(operation);
        self.save_queue(&queue)?;
        
        // Try to sync immediately if online
        if is_online() {
            wasm_bindgen_futures::spawn_local(async {
                let client = SyncClient::new();
                let _ = client.sync().await;
            });
        }
        
        Ok(())
    }
    
    // Get the current sync queue
    fn get_queue(&self) -> Result<Vec<SyncOperation>, String> {
        match LocalStorage::get(SYNC_QUEUE_KEY) {
            Ok(queue) => Ok(queue),
            Err(_) => Ok(Vec::new()),
        }
    }
    
    // Save the queue to local storage
    fn save_queue(&self, queue: &Vec<SyncOperation>) -> Result<(), String> {
        LocalStorage::set(SYNC_QUEUE_KEY, queue)
            .map_err(|e| format!("Failed to save sync queue: {}", e))
    }
    
    // Try to sync queued operations with server
    pub async fn sync(&self) -> Result<(), String> {
        // Check if online
        if !is_online() {
            return Err("Device is offline".to_string());
        }
        
        // Get auth token
        let token = match get_auth_token() {
            Some(token) => token,
            None => return Err("Not authenticated".to_string()),
        };
        
        // Get pending operations
        let mut queue = self.get_queue()?;
        let pending: Vec<SyncOperation> = queue.iter()
            .filter(|op| !op.synced)
            .cloned()
            .collect();
            
        if pending.is_empty() {
            return Ok(());
        }
        
        // Create sync batch
        let batch = self.create_batch(pending.clone());
        
        // Send to server
        match gloo_net::http::Request::post("/api/sync")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&batch)
            .map_err(|e| format!("Failed to serialize sync batch: {}", e))?
            .send()
            .await
            {
                Ok(response) => {
                    if response.ok() {
                        // Mark operations as synced
                        for op in &pending {
                            if let Some(queue_op) = queue.iter_mut().find(|q| q.id == op.id) {
                                queue_op.synced = true;
                            }
                        }
                        
                        // Save updated queue
                        self.save_queue(&queue)?;
                        
                        // Process server response if needed
                        // ...
                        
                        Ok(())
                    } else {
                        Err(format!("Sync failed: {}", response.status()))
                    }
                },
                Err(e) => Err(format!("Sync request failed: {}", e)),
            }
    }
    
    // Create a sync batch from operations
    fn create_batch(&self, operations: Vec<SyncOperation>) -> serde_json::Value {
        serde_json::json!({
            "device_id": self.device_id,
            "operations": operations,
            "timestamp": js_sys::Date::now() as i64
        })
    }
    
    // Register for online status events to trigger sync
    pub fn register_online_sync() {
        if let Some(window) = web_sys::window() {
            // Create online event handler
            let online_callback = Closure::wrap(Box::new(move || {
                if is_online() {
                    wasm_bindgen_futures::spawn_local(async {
                        let client = SyncClient::new();
                        let _ = client.sync().await;
                    });
                }
            }) as Box<dyn FnMut()>);
            
            // Add event listener
            let _ = window.add_event_listener_with_callback(
                "online", 
                online_callback.as_ref().unchecked_ref()
            );
            
            // Leak the closure to keep it alive
            online_callback.forget();
        }
    }
}