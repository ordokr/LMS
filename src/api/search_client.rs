use leptos::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::AbortController;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub hits: Vec<serde_json::Value>,
    pub total_hits: usize,
    pub processing_time_ms: u64,
}

// Adaptive client that can adjust to server load and network conditions
pub struct AdaptiveSearchClient {
    base_url: String,
    pending_abort: Option<AbortController>,
    timeout_ms: u32,
    retry_count: u32,
    debounce_ms: u32,
    debounce_timer: Option<i32>,
}

impl AdaptiveSearchClient {
    pub fn new() -> Self {
        Self {
            base_url: "/api/search".to_string(),
            pending_abort: None,
            timeout_ms: 3000,     // Start with 3s timeout
            retry_count: 1,       // Start with 1 retry
            debounce_ms: 300,     // Start with 300ms debounce
            debounce_timer: None,
        }
    }
    
    // Adaptive search with auto-tuning parameters
    pub async fn search(
        &mut self,
        query: &str,
        category_id: Option<i64>,
        limit: usize,
        offset: usize,
    ) -> Result<SearchResult, String> {
        // Don't search if query is too short
        if query.trim().len() < 2 {
            return Err("Query too short".to_string());
        }
        
        // Cancel any pending requests
        if let Some(controller) = &self.pending_abort {
            controller.abort();
        }
        
        // Create new abort controller
        let controller = AbortController::new().map_err(|_| "Failed to create abort controller")?;
        let signal = controller.signal();
        self.pending_abort = Some(controller);
        
        // Build URL
        let mut url = format!("{}/topics?q={}&limit={}&offset={}", self.base_url, query, limit, offset);
        if let Some(cat_id) = category_id {
            url.push_str(&format!("&category_id={}", cat_id));
        }
        
        // Set up request
        let mut opts = web_sys::RequestInit::new();
        opts.method("GET");
        opts.signal(Some(&signal));
        
        // Create request
        let request = web_sys::Request::new_with_str_and_init(&url, &opts)
            .map_err(|_| "Failed to create request".to_string())?;
            
        // Set headers
        request.headers().set("Accept", "application/json")
            .map_err(|_| "Failed to set headers".to_string())?;
            
        // Start timer for performance tracking
        let start_time = js_sys::Date::now();
        
        // Send request with timeout
        let promise = window().fetch_with_request(&request);
        let timeout_promise = js_sys::Promise::new(&mut |resolve, _| {
            let timeout_id = window().set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, 
                self.timeout_ms
            ).unwrap();
            
            // Prevent timeout from being garbage collected
            let _ = js_sys::Function::new_no_args(&format!("return {}", timeout_id));
        });
        
        // Race between fetch and timeout
        let race_promise = js_sys::Promise::race(&js_sys::Array::of2(
            &promise, 
            &timeout_promise
        ));
        
        // Convert to Future and await
        let result = JsFuture::from(race_promise).await;
        
        // Calculate response time
        let response_time = js_sys::Date::now() - start_time;
        
        // Adapt client settings based on response time
        self.adapt_settings(response_time as u32);
        
        match result {
            Ok(val) => {
                // Check if we got a timeout
                if val.is_undefined() {
                    return Err("Search request timed out".to_string());
                }
                
                let response = val.dyn_into::<web_sys::Response>()
                    .map_err(|_| "Failed to convert to Response".to_string())?;
                    
                if !response.ok() {
                    return Err(format!("Search request failed: {}", response.status()));
                }
                
                // Parse JSON
                let json = JsFuture::from(response.json()
                    .map_err(|_| "Failed to parse response as JSON".to_string())?)
                    .await
                    .map_err(|_| "Failed to parse response as JSON".to_string())?;
                    
                // Convert to SearchResult
                let result: SearchResult = serde_wasm_bindgen::from_value(json)
                    .map_err(|e| format!("Failed to deserialize response: {}", e))?;
                    
                Ok(result)
            },
            Err(_) => Err("Search request failed".to_string()),
        }
    }
    
    // Debounced search wrapper for UI components
    pub fn debounced_search(
        &mut self,
        query: &str,
        category_id: Option<i64>,
        limit: usize,
        offset: usize,
        callback: js_sys::Function,
    ) {
        // Clear previous timer if exists
        if let Some(timer_id) = self.debounce_timer {
            window().clear_timeout_with_handle(timer_id);
        }
        
        // Capture variables for closure
        let query = query.to_string();
        let category_id = category_id;
        let limit = limit;
        let offset = offset;
        let mut client = self.clone();
        
        // Create new timer
        let timer_id = window().set_timeout_with_callback_and_timeout_and_arguments_0(
            &Closure::once_into_js(move || {
                // Create async executor
                wasm_bindgen_futures::spawn_local(async move {
                    let result = client.search(&query, category_id, limit, offset).await;
                    
                    // Convert result to JS value
                    let js_value = match result {
                        Ok(data) => {
                            serde_wasm_bindgen::to_value(&data).unwrap_or(JsValue::NULL)
                        },
                        Err(err) => {
                            let obj = js_sys::Object::new();
                            js_sys::Reflect::set(&obj, &JsValue::from_str("error"), &JsValue::from_str(&err))
                                .unwrap_or_default();
                            obj.into()
                        }
                    };
                    
                    // Call the callback
                    let _ = callback.call1(&JsValue::NULL, &js_value);
                });
            }),
            self.debounce_ms,
        ).unwrap_or(0);
        
        self.debounce_timer = Some(timer_id);
    }
    
    // Adapt settings based on response time
    fn adapt_settings(&mut self, response_time_ms: u32) {
        if response_time_ms < 100 {
            // Very fast - decrease debounce time
            self.debounce_ms = (self.debounce_ms - 50).max(100);
            self.timeout_ms = 2000;
        } else if response_time_ms > 1000 {
            // Slow - increase debounce and timeout
            self.debounce_ms = (self.debounce_ms + 100).min(800);
            self.timeout_ms = (self.timeout_ms + 1000).min(8000);
            self.retry_count = (self.retry_count + 1).min(3);
        }
    }
    
    // Create clone for use in closures
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            pending_abort: None,
            timeout_ms: self.timeout_ms,
            retry_count: self.retry_count,
            debounce_ms: self.debounce_ms,
            debounce_timer: None,
        }
    }
}