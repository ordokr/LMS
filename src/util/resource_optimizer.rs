use leptos::*;
use futures::future::join_all;
use std::future::Future;
use std::rc::Rc;
use std::time::Duration;

// Optimized resource that avoids unnecessary refetches
pub struct OptimizedResource<T: Clone + 'static> {
    inner: Resource<(usize, bool), Result<T, String>>,
    loading: Signal<bool>,
    data: Signal<Option<T>>,
    error: Signal<Option<String>>,
    refetch_counter: Rc<std::cell::RefCell<usize>>,
    stale_time: Rc<std::cell::RefCell<Duration>>,
    last_fetched: Rc<std::cell::RefCell<Option<instant::Instant>>>,
}

impl<T: Clone + 'static> OptimizedResource<T> {
    pub fn new<Fu, Fut, D>(
        deps: D,
        fetcher: Fu,
        stale_time: Duration,
    ) -> Self 
    where
        Fu: Fn(D) -> Fut + 'static,
        Fut: Future<Output = Result<T, String>> + 'static,
        D: Clone + 'static + PartialEq,
    {
        let refetch_counter = Rc::new(std::cell::RefCell::new(0));
        let counter = refetch_counter.clone();
        let stale_time_rc = Rc::new(std::cell::RefCell::new(stale_time));
        let last_fetched = Rc::new(std::cell::RefCell::new(None));
        let last_fetched_clone = last_fetched.clone();
        let stale_time_clone = stale_time_rc.clone();
        
        let inner = create_resource(
            move || (*counter.borrow(), false),
            move |_| {
                let deps = deps.clone();
                let fetcher = fetcher.clone();
                let last_fetched = last_fetched_clone.clone();
                let stale_time = stale_time_clone.clone();
                
                async move {
                    // Check if we should refetch based on stale time
                    if let Some(last_fetch_time) = *last_fetched.borrow() {
                        let elapsed = instant::Instant::now().duration_since(last_fetch_time);
                        if elapsed < *stale_time.borrow() {
                            // Return a special marker to indicate we should use cached data
                            return Err("__use_cached__".to_string());
                        }
                    }
                    
                    // Actually fetch
                    let result = fetcher(deps.clone()).await;
                    
                    if result.is_ok() {
                        // Update last fetched time
                        *last_fetched.borrow_mut() = Some(instant::Instant::now());
                    }
                    
                    result
                }
            }
        );
        
        // Create signals that handle the special cache case
        let resource_signal = inner.signal();
        let data = Signal::derive(move || {
            match resource_signal.get() {
                Some(Ok(data)) => Some(data),
                Some(Err(ref e)) if e == "__use_cached__" => {
                    // This means use the previous data, handled below
                    None
                },
                _ => None,
            }
        });
        
        let error = Signal::derive(move || {
            match resource_signal.get() {
                Some(Err(ref e)) if e != "__use_cached__" => Some(e.clone()),
                _ => None,
            }
        });
        
        let loading = Signal::derive(move || {
            resource_signal.get().is_none()
        });
        
        Self {
            inner,
            loading,
            data,
            error,
            refetch_counter,
            stale_time: stale_time_rc,
            last_fetched,
        }
    }
    
    // Access signals
    pub fn loading(&self) -> Signal<bool> {
        self.loading
    }
    
    pub fn data(&self) -> Signal<Option<T>> {
        self.data
    }
    
    pub fn error(&self) -> Signal<Option<String>> {
        self.error
    }
    
    // Force refetch regardless of stale time
    pub fn refetch(&self) {
        let mut counter = self.refetch_counter.borrow_mut();
        *counter += 1;
        drop(counter);
        self.inner.refetch();
    }
    
    // Update stale time
    pub fn set_stale_time(&self, duration: Duration) {
        *self.stale_time.borrow_mut() = duration;
    }
    
    // Reset last fetched time to force a refetch on next access
    pub fn invalidate(&self) {
        *self.last_fetched.borrow_mut() = None;
    }
}

// Fetch multiple resources in parallel with batching
pub async fn fetch_parallel<T, E, Args, Fn, Fut>(
    args: Vec<Args>,
    fetch_fn: Fn,
) -> Vec<Result<T, E>>
where
    Args: Clone + Send + 'static,
    Fn: Fn(Args) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Result<T, E>> + Send,
{
    // Create futures
    let futures: Vec<_> = args.into_iter()
        .map(|arg| {
            let fetch = fetch_fn.clone();
            async move { fetch(arg).await }
        })
        .collect();
    
    // Execute in parallel
    join_all(futures).await
}

// Create hook for the optimized resource
#[hook]
pub fn use_optimized_resource<T, D, Fu, Fut>(
    deps: D,
    fetcher: Fu,
    stale_time: Duration,
) -> OptimizedResource<T>
where
    T: Clone + 'static,
    D: Clone + 'static + PartialEq,
    Fu: Fn(D) -> Fut + 'static,
    Fut: Future<Output = Result<T, String>> + 'static,
{
    OptimizedResource::new(deps, fetcher, stale_time)
}