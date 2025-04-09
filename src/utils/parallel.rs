use futures::{stream, StreamExt};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::{Semaphore, RwLock};
use tokio::task;
use wasm_bindgen::prelude::*;
use js_sys::{Promise, Array, Reflect};
use wasm_bindgen_futures::JsFuture;

// Efficient parallel processing with controlled concurrency
pub struct ParallelProcessor<T, R> {
    concurrency: usize,
    items: Vec<T>,
    processor: Arc<dyn Fn(T) -> impl std::future::Future<Output = Result<R, String>> + Send + Sync>,
    results: Arc<RwLock<Vec<Option<Result<R, String>>>>>,
    semaphore: Arc<Semaphore>,
}

impl<T, R> ParallelProcessor<T, R>
where
    T: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    pub fn new<F, Fut>(items: Vec<T>, concurrency: usize, processor: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<R, String>> + Send + 'static,
    {
        Self {
            concurrency: concurrency.max(1),
            items,
            processor: Arc::new(processor),
            results: Arc::new(RwLock::new(vec![None; items.len()])),
            semaphore: Arc::new(Semaphore::new(concurrency.max(1))),
        }
    }
    
    // Process items in parallel with controlled concurrency
    pub async fn process(self) -> Result<Vec<R>, String> {
        let mut handles = Vec::with_capacity(self.items.len());
        
        // Process each item
        for (index, item) in self.items.into_iter().enumerate() {
            let permit = match self.semaphore.clone().acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => return Err("Semaphore closed".to_string()),
            };
            
            let processor = self.processor.clone();
            let results = self.results.clone();
            
            // Spawn task
            let handle = task::spawn(async move {
                let result = processor(item).await;
                let mut results_guard = results.write().await;
                results_guard[index] = Some(result);
                drop(permit);
            });
            
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            if let Err(e) = handle.await {
                return Err(format!("Task panicked: {}", e));
            }
        }
        
        // Collect results
        let results = self.results.read().await;
        let mut output = Vec::with_capacity(results.len());
        
        for result_opt in results.iter() {
            match result_opt {
                Some(Ok(ref value)) => output.push(value.clone()),
                Some(Err(e)) => return Err(e.clone()),
                None => return Err("Missing result".to_string()),
            }
        }
        
        Ok(output)
    }
    
    // Process with progress reporting
    pub async fn process_with_progress<F>(
        self, 
        progress_callback: F
    ) -> Result<Vec<R>, String>
    where
        F: Fn(usize, usize) + Send + Sync + 'static,
    {
        let total = self.items.len();
        let completed = Arc::new(RwLock::new(0));
        let progress_callback = Arc::new(progress_callback);
        
        let mut handles = Vec::with_capacity(total);
        
        // Process each item
        for (index, item) in self.items.into_iter().enumerate() {
            let permit = match self.semaphore.clone().acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => return Err("Semaphore closed".to_string()),
            };
            
            let processor = self.processor.clone();
            let results = self.results.clone();
            let completed = completed.clone();
            let progress_callback = progress_callback.clone();
            
            // Spawn task
            let handle = task::spawn(async move {
                let result = processor(item).await;
                let mut results_guard = results.write().await;
                results_guard[index] = Some(result);
                
                // Update and report progress
                let mut completed_guard = completed.write().await;
                *completed_guard += 1;
                progress_callback(*completed_guard, total);
                
                drop(permit);
            });
            
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            if let Err(e) = handle.await {
                return Err(format!("Task panicked: {}", e));
            }
        }
        
        // Collect results
        let results = self.results.read().await;
        let mut output = Vec::with_capacity(results.len());
        
        for result_opt in results.iter() {
            match result_opt {
                Some(Ok(ref value)) => output.push(value.clone()),
                Some(Err(e)) => return Err(e.clone()),
                None => return Err("Missing result".to_string()),
            }
        }
        
        Ok(output)
    }
    
    // Check if browser supports parallel processing
    pub async fn is_parallel_supported() -> bool {
        match web_sys::window() {
            Some(window) => match window.navigator().hardware_concurrency() {
                cores if cores > 1 => true,
                _ => false,
            },
            None => false,
        }
    }
}

// Chunk processing for large datasets
pub async fn process_in_chunks<T, R, F, Fut>(
    items: Vec<T>,
    chunk_size: usize,
    concurrency: usize,
    processor: F,
) -> Result<Vec<R>, String>
where
    T: Clone + Send + Sync + 'static,
    R: Send + Sync + 'static,
    F: Fn(T) -> Fut + Send + Sync + Clone + 'static,
    Fut: std::future::Future<Output = Result<R, String>> + Send + 'static,
{
    if items.is_empty() {
        return Ok(Vec::new());
    }
    
    // Process in chunks to avoid memory issues
    let chunk_size = chunk_size.max(1);
    let chunks: Vec<Vec<T>> = items
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    let mut all_results = Vec::with_capacity(items.len());
    
    for chunk in chunks {
        let processor = processor.clone();
        let processor_fn = move |item: T| processor(item);
        
        let parallel = ParallelProcessor::new(chunk, concurrency, processor_fn);
        let chunk_results = parallel.process().await?;
        
        all_results.extend(chunk_results);
    }
    
    Ok(all_results)
}

// WASM bindings for parallel processing from JavaScript
#[wasm_bindgen]
pub async fn parallel_process(
    js_items: &JsValue,
    concurrency: usize,
    processor: &js_sys::Function,
) -> Result<JsValue, JsValue> {
    let items: Vec<JsValue> = js_items
        .into_serde()
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize items: {}", e)))?;
    
    if items.is_empty() {
        return Ok(JsValue::from(Array::new()));
    }
    
    // Create semaphore for concurrency control
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let results = Arc::new(RwLock::new(vec![None::<JsValue>; items.len()]));
    
    // Create tasks
    let mut handles = Vec::with_capacity(items.len());
    
    for (index, item) in items.into_iter().enumerate() {
        let permit = semaphore.clone().acquire_owned().await.map_err(|e| {
            JsValue::from_str(&format!("Semaphore error: {}", e))
        })?;
        
        let processor = processor.clone();
        let results = results.clone();
        
        // Process asynchronously
        let handle = wasm_bindgen_futures::spawn_local(async move {
            let this = JsValue::null();
            let args = Array::new();
            args.push(&item);
            
            // Call JS processor function
            let result = Reflect::apply(&processor, &this, &args).ok();
            
            // Wait if it returned a Promise
            let result = if result.clone().and_then(|r| r.dyn_into::<Promise>().ok()).is_some() {
                let promise = Promise::resolve(&result.unwrap());
                match JsFuture::from(promise).await {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
            } else {
                result
            };
            
            // Store result
            let mut results_guard = results.write().await;
            results_guard[index] = result;
            
            drop(permit);
        });
        
        handles.push(handle);
    }
    
    // Wait for completions by polling
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let total = items.len();
    
    while Arc::strong_count(&results) > 1 {
        // Yield to event loop to allow workers to make progress
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &resolve, 10
                )
                .unwrap();
        });
        
        JsFuture::from(promise).await.map_err(|e| {
            JsValue::from_str(&format!("Failed to yield to event loop: {:?}", e))
        })?;
    }
    
    // Collect results
    let results = Arc::try_unwrap(results)
        .map_err(|_| JsValue::from_str("Failed to get results"))?
        .into_inner();
    
    let output_array = Array::new();
    
    for result in results {
        if let Some(value) = result {
            output_array.push(&value);
        } else {
            return Err(JsValue::from_str("Missing result"));
        }
    }
    
    Ok(output_array.into())
}