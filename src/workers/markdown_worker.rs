use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};
use leptos::*;
use pulldown_cmark::{html, Options, Parser};
use once_cell::sync::Lazy;
use std::sync::RwLock;

// Store worker instance
static MARKDOWN_WORKER: Lazy<RwLock<Option<Worker>>> = Lazy::new(|| RwLock::new(None));

// Initialize worker
pub fn init_markdown_worker() -> Result<(), JsValue> {
    // Create worker URL from inline worker code
    let worker_code = r#"
        self.onmessage = function(e) {
            const { id, markdown, options } = e.data;
            
            // Process markdown using wasm
            import('../pkg/markdown_worker.js')
                .then(module => {
                    const result = module.process_markdown(markdown, options);
                    self.postMessage({ id, html: result, success: true });
                })
                .catch(error => {
                    self.postMessage({ id, error: error.toString(), success: false });
                });
        };
    "#;
    
    let blob = web_sys::Blob::new_with_str_sequence_and_options(
        &js_sys::Array::of1(&JsValue::from_str(worker_code)),
        web_sys::BlobPropertyBag::new().type_("text/javascript"),
    )?;
    
    let url = web_sys::Url::create_object_url_with_blob(&blob)?;
    let worker = Worker::new(&url)?;
    
    // Store worker instance
    *MARKDOWN_WORKER.write().unwrap() = Some(worker);
    
    Ok(())
}

// Process markdown text in the worker
pub fn process_markdown_in_worker(markdown: &str, callback: Box<dyn Fn(String) + 'static>) -> Result<(), JsValue> {
    let worker_guard = MARKDOWN_WORKER.read().unwrap();
    let worker = worker_guard.as_ref().ok_or_else(|| JsValue::from_str("Worker not initialized"))?;
    
    // Generate unique ID for this request
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Set up message handler
    let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        let result = event.data();
        let result_obj = js_sys::Object::from(result);
        
        let id = js_sys::Reflect::get(&result_obj, &JsValue::from_str("id"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
            
        // Only process if it's our request
        if id == request_id {
            let success = js_sys::Reflect::get(&result_obj, &JsValue::from_str("success"))
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
                
            if success {
                let html = js_sys::Reflect::get(&result_obj, &JsValue::from_str("html"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();
                    
                callback(html);
            } else {
                // Handle error
                log::error!("Markdown processing error");
                callback(String::from("<p>Error processing markdown</p>"));
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    
    worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    
    // Send data to worker
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&message, &JsValue::from_str("id"), &JsValue::from_str(&request_id));
    let _ = js_sys::Reflect::set(&message, &JsValue::from_str("markdown"), &JsValue::from_str(markdown));
    
    worker.post_message(&message)?;
    
    // We don't want callback to be dropped
    callback.forget();
    
    Ok(())
}

// WASM-compatible markdown processor (compiled to a separate wasm module)
#[wasm_bindgen]
pub fn process_markdown(markdown: &str, options_json: &str) -> String {
    // Parse options
    let options_obj: serde_json::Value = match serde_json::from_str(options_json) {
        Ok(obj) => obj,
        Err(_) => return format!("<p>Error parsing options</p>"),
    };
    
    // Configure markdown parser
    let mut options = Options::empty();
    
    // Enable GitHub-flavored Markdown
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    
    // Parse and render
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

// Leptos hook for markdown rendering with worker
#[hook]
pub fn use_markdown_renderer() -> impl Fn(String) -> Signal<String> {
    // Initialize worker if needed
    if MARKDOWN_WORKER.read().unwrap().is_none() {
        if let Err(e) = init_markdown_worker() {
            log::error!("Failed to initialize markdown worker: {:?}", e);
        }
    }
    
    let markdown_store = store_value(HashMap::<String, String>::new());
    
    // Return function that processes markdown
    move |markdown: String| -> Signal<String> {
        let (html_signal, set_html) = create_signal(String::new());
        
        // Check if we've already rendered this content
        let content_hash = {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            markdown.hash(&mut hasher);
            format!("{:x}", hasher.finish())
        };
        
        // Check cache
        let mut store = markdown_store.get_value();
        if let Some(cached_html) = store.get(&content_hash) {
            set_html.set(cached_html.clone());
        } else {
            // Set loading state
            set_html.set("<p>Loading...</p>".to_string());
            
            // Process in worker
            let content_hash_clone = content_hash.clone();
            let markdown_clone = markdown.clone();
            
            spawn_local(async move {
                let (tx, rx) = futures::channel::oneshot::channel::<String>();
                
                if let Err(e) = process_markdown_in_worker(&markdown_clone, Box::new(move |html| {
                    let _ = tx.send(html);
                })) {
                    log::error!("Failed to process markdown: {:?}", e);
                    set_html.set("<p>Error rendering content</p>".to_string());
                    return;
                }
                
                // Wait for response
                match rx.await {
                    Ok(html) => {
                        // Update cache
                        let mut store = markdown_store.get_value();
                        store.insert(content_hash_clone, html.clone());
                        markdown_store.set_value(store);
                        
                        // Update signal
                        set_html.set(html);
                    }
                    Err(_) => {
                        set_html.set("<p>Error rendering content</p>".to_string());
                    }
                }
            });
        }
        
        html_signal
    }
}