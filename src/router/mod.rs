use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

// Router with optimized route matching
pub struct OptimizedRouter {
    routes: HashMap<String, Box<dyn Fn() -> View>>,
    current_route: String,
    history: Vec<String>,
    view_cache: RefCell<HashMap<String, (View, Instant)>>,
    cache_ttl: Duration,
    default_route: Option<String>,
}

impl OptimizedRouter {
    pub fn new(cache_ttl_ms: u64) -> Self {
        Self {
            routes: HashMap::new(),
            current_route: "/".to_string(),
            history: Vec::new(),
            view_cache: RefCell::new(HashMap::new()),
            cache_ttl: Duration::from_millis(cache_ttl_ms),
            default_route: None,
        }
    }
    
    pub fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn() -> View + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(handler));
    }
    
    pub fn set_default_route(&mut self, path: &str) {
        self.default_route = Some(path.to_string());
    }
    
    pub fn current_view(&self) -> View {
        let path = &self.current_route;
        
        // Check cache first
        {
            let cache = self.view_cache.borrow();
            if let Some((view, timestamp)) = cache.get(path) {
                if timestamp.elapsed() < self.cache_ttl {
                    return view.clone();
                }
            }
        }
        
        // Not in cache or expired, render new view
        if let Some(handler) = self.routes.get(path) {
            let view = handler();
            
            // Cache the result
            self.view_cache.borrow_mut().insert(
                path.clone(),
                (view.clone(), Instant::now()),
            );
            
            view
        } else if let Some(ref default) = self.default_route {
            if let Some(handler) = self.routes.get(default) {
                handler()
            } else {
                view! { <div>"Route not found"</div> }
            }
        } else {
            view! { <div>"Route not found"</div> }
        }
    }
    
    pub fn navigate(&mut self, path: &str) {
        // Track history
        self.history.push(self.current_route.clone());
        self.current_route = path.to_string();
    }
    
    pub fn go_back(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.current_route = prev;
            true
        } else {
            false
        }
    }
    
    pub fn invalidate_cache(&self, path: Option<&str>) {
        let mut cache = self.view_cache.borrow_mut();
        if let Some(p) = path {
            cache.remove(p);
        } else {
            cache.clear();
        }
    }
}

// Route parameters extractor with caching
#[derive(Clone)]
pub struct RouteParams {
    cache: Rc<RefCell<HashMap<String, HashMap<String, String>>>>,
}

impl RouteParams {
    pub fn new() -> Self {
        Self {
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    
    pub fn extract(&self, path: &str, route_pattern: &str) -> Option<HashMap<String, String>> {
        let cache_key = format!("{}:{}", path, route_pattern);
        
        // Check cache
        {
            let cache = self.cache.borrow();
            if let Some(params) = cache.get(&cache_key) {
                return Some(params.clone());
            }
        }
        
        // Parse route parameters
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let pattern_segments: Vec<&str> = route_pattern.split('/').filter(|s| !s.is_empty()).collect();
        
        if path_segments.len() != pattern_segments.len() {
            return None;
        }
        
        let mut params = HashMap::new();
        
        for (i, pattern) in pattern_segments.iter().enumerate() {
            if pattern.starts_with(':') {
                let param_name = &pattern[1..];
                params.insert(param_name.to_string(), path_segments[i].to_string());
            } else if pattern != &path_segments[i] {
                return None;
            }
        }
        
        // Cache the result
        self.cache.borrow_mut().insert(cache_key, params.clone());
        
        Some(params)
    }
    
    pub fn clear_cache(&self) {
        self.cache.borrow_mut().clear();
    }
}

// Optimized route component that uses memoization for expensive route resolution
#[component]
pub fn OptimizedRoute(
    #[prop(into)] path: String,
    #[prop(into)] view: Signal<View>,
    #[prop(optional)] fallback: Option<View>,
) -> impl IntoView {
    // Use the URL path from Leptos router
    let route = use_route();
    let path_signal = create_memo(move |_| route.path());
    
    // Memoize whether this route matches the current path
    let matches = create_memo(move |_| {
        let current = path_signal.get();
        let pattern = path.as_str();
        
        // Simple exact match for now, could be enhanced with pattern matching
        current == pattern
    });
    
    // Pre-compute the view once when route matches and reuse it until route changes
    let rendered_view = create_memo(move |_| {
        if matches.get() {
            view.get()
        } else if let Some(ref fallback) = fallback {
            fallback.clone()
        } else {
            view! { <div></div> }
        }
    });
    
    // Create a keyed fragment that will rerender only when route changes
    view! {
        <Show
            when=move || matches.get()
            fallback=move || view! { <div class="hidden"></div> }
        >
            {rendered_view}
        </Show>
    }
}

// Optimized forum routes setup
pub fn setup_forum_routes() -> Router {
    Router::new(
        view! {
            <main>
                <Routes>
                    <Route
                        path="/forum"
                        view=move || view! { <ForumHome/> }
                    />
                    <OptimizedRoute
                        path="/forum/category/:id"
                        view=create_signal(view! { <ForumCategory/> }).0
                    />
                    <OptimizedRoute
                        path="/forum/topic/:id"
                        view=create_signal(view! { <ForumTopic/> }).0
                    />
                    <OptimizedRoute
                        path="/forum/user/:id"
                        view=create_signal(view! { <ForumUserProfile/> }).0
                    />
                    <Route
                        path="/*any"
                        view=move || view! { <NotFound/> }
                    />
                </Routes>
            </main>
        }
    )
}