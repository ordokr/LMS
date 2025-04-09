use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Array, Object, Promise, Reflect};
use web_sys::{IdbDatabase, IdbOpenDbRequest, IdbObjectStore, IdbTransaction, IdbTransactionMode};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

// Database connection singleton
static DB: Lazy<IdbConnection> = Lazy::new(|| {
    IdbConnection::new("forum_app_db", 1)
});

// Get database instance
pub fn get_db() -> &'static IdbConnection {
    &DB
}

// IndexedDB connection abstraction
pub struct IdbConnection {
    db: RwSignal<Option<IdbDatabase>>,
    initialization_promise: RwSignal<Option<Promise>>,
}

impl IdbConnection {
    // Create a new connection to IndexedDB
    pub fn new(db_name: &str, version: u32) -> Self {
        let instance = Self {
            db: create_rw_signal(None),
            initialization_promise: create_rw_signal(None),
        };
        
        // Initialize database
        let db_name = db_name.to_string();
        spawn_local(async move {
            match Self::open_database(&db_name, version).await {
                Ok(db) => instance.db.set(Some(db)),
                Err(e) => log::error!("Failed to open IndexedDB database: {:?}", e),
            }
        });
        
        instance
    }
    
    // Open the database with required object stores
    async fn open_database(db_name: &str, version: u32) -> Result<IdbDatabase, JsValue> {
        let window = web_sys::window().unwrap();
        let indexed_db = window.indexed_db()?;
        
        let open_request = indexed_db.open_with_u32(db_name, version)?;
        
        // Set up upgrade handler
        let upgrade_needed_handler = Closure::once(move |event: web_sys::IdbVersionChangeEvent| {
            let db = event.target()
                .unwrap()
                .dyn_into::<IdbOpenDbRequest>()
                .unwrap()
                .result()
                .unwrap()
                .dyn_into::<IdbDatabase>()
                .unwrap();
            
            // Create object stores if needed
            if !db.object_store_names().contains(&JsValue::from_str("forum_cache")) {
                let opts = web_sys::IdbObjectStoreParameters::new();
                opts.key_path(Some(&JsValue::from_str("key")));
                db.create_object_store_with_params("forum_cache", &opts).unwrap();
            }
            
            if !db.object_store_names().contains(&JsValue::from_str("user_settings")) {
                let opts = web_sys::IdbObjectStoreParameters::new();
                opts.key_path(Some(&JsValue::from_str("key")));
                db.create_object_store_with_params("user_settings", &opts).unwrap();
            }
            
            if !db.object_store_names().contains(&JsValue::from_str("offline_posts")) {
                let opts = web_sys::IdbObjectStoreParameters::new();
                opts.auto_increment(true);
                db.create_object_store_with_params("offline_posts", &opts).unwrap();
            }
        });
        
        open_request.set_onupgradeneeded(Some(upgrade_needed_handler.as_ref().unchecked_ref()));
        
        // Convert request to promise
        let promise = Promise::new(&mut |resolve, reject| {
            let open_success = Closure::once(move |event: web_sys::Event| {
                let target = event.target().unwrap();
                let request = target.dyn_into::<IdbOpenDbRequest>().unwrap();
                let result = request.result().unwrap();
                let db = result.dyn_into::<IdbDatabase>().unwrap();
                resolve.call1(&JsValue::NULL, &db).unwrap();
            });
            
            let open_error = Closure::once(move |event: web_sys::Event| {
                let target = event.target().unwrap();
                let request = target.dyn_into::<IdbOpenDbRequest>().unwrap();
                let error = request.error().unwrap();
                reject.call1(&JsValue::NULL, &error).unwrap();
            });
            
            open_request.set_onsuccess(Some(open_success.as_ref().unchecked_ref()));
            open_request.set_onerror(Some(open_error.as_ref().unchecked_ref()));
            
            open_success.forget();
            open_error.forget();
        });
        
        // Wait for database to open
        let db = JsFuture::from(promise).await?;
        Ok(db.dyn_into::<IdbDatabase>()?)
    }
    
    // Store data in the specified store
    pub async fn store_data<T: Serialize + 'static>(
        &self,
        store_name: &str,
        key: &str,
        data: T,
    ) -> Result<(), JsValue> {
        let Some(db) = self.db.get() else {
            return Err(JsValue::from_str("Database not initialized"));
        };
        
        // Serialize data
        let serialized_data = serde_wasm_bindgen::to_value(&data)?;
        
        // Create object with key
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(key))?;
        Reflect::set(&obj, &JsValue::from_str("data"), &serialized_data)?;
        Reflect::set(&obj, &JsValue::from_str("timestamp"), &JsValue::from_f64(js_sys::Date::now()))?;
        
        // Open transaction
        let transaction = db.transaction_with_str(store_name, IdbTransactionMode::Readwrite)?;
        let store = transaction.object_store(store_name)?;
        
        // Store data
        let put_request = store.put(&obj)?;
        
        // Wait for completion
        let promise = Promise::new(&mut |resolve, reject| {
            let on_success = Closure::once(move |_: web_sys::Event| {
                resolve.call0(&JsValue::NULL).unwrap();
            });
            
            let on_error = Closure::once(move |event: web_sys::Event| {
                let target = event.target().unwrap();
                let request = target.dyn_into::<web_sys::IdbRequest>().unwrap();
                let error = request.error().unwrap();
                reject.call1(&JsValue::NULL, &error).unwrap();
            });
            
            put_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
            put_request.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            
            on_success.forget();
            on_error.forget();
        });
        
        JsFuture::from(promise).await?;
        Ok(())
    }
    
    // Retrieve data from the specified store
    pub async fn get_data<T: for<'de> Deserialize<'de> + 'static>(
        &self,
        store_name: &str,
        key: &str,
    ) -> Result<Option<T>, JsValue> {
        let Some(db) = self.db.get() else {
            return Err(JsValue::from_str("Database not initialized"));
        };
        
        // Open transaction
        let transaction = db.transaction_with_str(store_name, IdbTransactionMode::Readonly)?;
        let store = transaction.object_store(store_name)?;
        
        // Get data
        let get_request = store.get(&JsValue::from_str(key))?;
        
        // Wait for completion
        let promise = Promise::new(&mut |resolve, reject| {
            let on_success = Closure::once(move |event: web_sys::Event| {
                let target = event.target().unwrap();
                let request = target.dyn_into::<web_sys::IdbRequest>().unwrap();
                let result = request.result();
                resolve.call1(&JsValue::NULL, &result).unwrap();
            });
            
            let on_error = Closure::once(move |event: web_sys::Event| {
                let target = event.target().unwrap();
                let request = target.dyn_into::<web_sys::IdbRequest>().unwrap();
                let error = request.error().unwrap();
                reject.call1(&JsValue::NULL, &error).unwrap();
            });
            
            get_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
            get_request.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            
            on_success.forget();
            on_error.forget();
        });
        
        let result = JsFuture::from(promise).await?;
        
        // Check if there's no result
        if result.is_undefined() || result.is_null() {
            return Ok(None);
        }
        
        // Extract data field
        let obj = result.dyn_into::<Object>()?;
        let data = Reflect::get(&obj, &JsValue::from_str("data"))?;
        
        // Deserialize data
        match serde_wasm_bindgen::from_value(data) {
            Ok(deserialized) => Ok(Some(deserialized)),
            Err(_) => Err(JsValue::from_str("Failed to deserialize data")),
        }
    }
    
    // Delete data from the specified store
    pub async fn delete_data(&self, store_name: &str, key: &str) -> Result<(), JsValue> {
        let Some(db) = self.db.get() else {
            return Err(JsValue::from_str("Database not initialized"));
        };
        
        // Open transaction
        let transaction = db.transaction_with_str(store_name, IdbTransactionMode::Readwrite)?;
        let store = transaction.object_store(store_name)?;
        
        // Delete data
        let delete_request = store.delete(&JsValue::from_str(key))?;
        
        // Wait for completion
        let promise = Promise::new(&mut |resolve, reject| {
            let on_success = Closure::once(move |_: web_sys::Event| {
                resolve.call0(&JsValue::NULL).unwrap();
            });
            
            let on_error = Closure::once(move |event: web_sys::Event| {
                let target = event.target().unwrap();
                let request = target.dyn_into::<web_sys::IdbRequest>().unwrap();
                let error = request.error().unwrap();
                reject.call1(&JsValue::NULL, &error).unwrap();
            });
            
            delete_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
            delete_request.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            
            on_success.forget();
            on_error.forget();
        });
        
        JsFuture::from(promise).await?;
        Ok(())
    }
    
    // Clear all data from a store
    pub async fn clear_store(&self, store_name: &str) -> Result<(), JsValue> {
        let Some(db) = self.db.get() else {
            return Err(JsValue::from_str("Database not initialized"));
        };
        
        // Open transaction
        let transaction = db.transaction_with_str(store_name, IdbTransactionMode::Readwrite)?;
        let store = transaction.object_store(store_name)?;
        
        // Clear store
        let clear_request = store.clear()?;
        
        // Wait for completion
        let promise = Promise::new(&mut |resolve, reject| {
            let on_success = Closure::once(move |_: web_sys::Event| {
                resolve.call0(&JsValue::NULL).unwrap();
            });
            
            let on_error = Closure::once(move |event: web_sys::Event| {
                let target = event.target().unwrap();
                let request = target.dyn_into::<web_sys::IdbRequest>().unwrap();
                let error = request.error().unwrap();
                reject.call1(&JsValue::NULL, &error).unwrap();
            });
            
            clear_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
            clear_request.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            
            on_success.forget();
            on_error.forget();
        });
        
        JsFuture::from(promise).await?;
        Ok(())
    }
}