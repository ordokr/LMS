use super::app_state::{AppState, AppStore, StateSection};
use wasm_bindgen::prelude::*;
use web_sys::Storage;
use leptos::*;
use serde_json;

pub struct StatePersistence {
    app_store: AppStore,
    storage_key: String,
    auto_save: bool,
}

impl StatePersistence {
    pub fn new(app_store: AppStore, storage_key: &str, auto_save: bool) -> Self {
        let instance = Self {
            app_store,
            storage_key: storage_key.to_string(),
            auto_save,
        };
        
        // Set up auto-save if enabled
        if auto_save {
            instance.setup_auto_save();
        }
        
        // Try to load state on initialization
        instance.load();
        
        instance
    }
    
    // Set up automatic state saving when it changes
    fn setup_auto_save(&self) {
        let storage_key = self.storage_key.clone();
        let app_store = self.app_store.clone();
        
        // Use the observer pattern to save only when needed
        self.app_store.observe(StateSection::All, Callback::new(move |_| {
            let state = app_store.get_state().get();
            let storage = Self::get_local_storage();
            
            // Only stringify what we need to persist
            match serde_json::to_string(&state) {
                Ok(json) => {
                    if let Some(storage) = storage {
                        let _ = storage.set_item(&storage_key, &json);
                    }
                },
                Err(e) => log::error!("Failed to serialize state: {}", e),
            }
        }));
    }
    
    // Save current state to storage
    pub fn save(&self) -> bool {
        let state = self.app_store.get_state().get();
        let storage = Self::get_local_storage();
        
        if let Some(storage) = storage {
            match serde_json::to_string(&state) {
                Ok(json) => {
                    if storage.set_item(&self.storage_key, &json).is_ok() {
                        return true;
                    }
                },
                Err(e) => log::error!("Failed to serialize state: {}", e),
            }
        }
        
        false
    }
    
    // Load state from storage
    pub fn load(&self) -> bool {
        let storage = Self::get_local_storage();
        
        if let Some(storage) = storage {
            if let Ok(Some(json)) = storage.get_item(&self.storage_key) {
                match serde_json::from_str::<AppState>(&json) {
                    Ok(loaded_state) => {
                        // Update the state all at once
                        self.app_store.get_state().set(loaded_state);
                        return true;
                    },
                    Err(e) => log::error!("Failed to deserialize state: {}", e),
                }
            }
        }
        
        false
    }
    
    // Clear persisted state
    pub fn clear(&self) -> bool {
        let storage = Self::get_local_storage();
        
        if let Some(storage) = storage {
            return storage.remove_item(&self.storage_key).is_ok();
        }
        
        false
    }
    
    // Helper to get localStorage
    fn get_local_storage() -> Option<Storage> {
        match web_sys::window() {
            Some(window) => match window.local_storage() {
                Ok(storage) => storage,
                Err(_) => None,
            },
            None => None,
        }
    }
}