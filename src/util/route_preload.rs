use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/// Routes that should be preloaded
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PreloadRoute {
    Forum,
    ForumCategory,
    ForumTopic,
    Home,
    Profile,
    // Add more routes as needed
}

impl PreloadRoute {
    pub fn to_path(&self) -> String {
        match self {
            Self::Forum => "/forum".to_string(),
            Self::ForumCategory => "/forum/category/1".to_string(), // Default category
            Self::ForumTopic => "/forum/topic/1".to_string(),       // Default topic
            Self::Home => "/".to_string(),
            Self::Profile => "/profile".to_string(),
        }
    }
}

/// Create a router with intelligent preloading
#[component]
pub fn OptimizedRouter<F, IV>(
    #[prop(into)] routes: F,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let routes_view = routes();
    
    // Set up preload manager
    let preload_manager = PreloadManager::new();
    
    provide_context(preload_manager.clone());
    
    // Start monitoring user behavior for predictive preloading
    preload_manager.start_monitoring();
    
    view! {
        <Router>
            {routes_view}
        </Router>
    }
}

/// Route with optimized loading and prefetching behavior
#[component]
pub fn OptimizedRoute<IV, F>(
    path: &'static str,
    view: F,
    #[prop(optional)] preload_routes: Vec<PreloadRoute>,
) -> impl IntoView
where
    IV: IntoView,
    F: Fn() -> IV + Clone + 'static,
{
    let preload_manager = use_context::<PreloadManager>().expect("PreloadManager should be provided");
    
    let route_id = path.to_string();
    let preload_routes = store_value(preload_routes);
    
    let on_mount = move || {
        // Register which routes should be preloaded when this route is active
        if !preload_routes.get_value().is_empty() {
            preload_manager.register_route_preloads(route_id.clone(), preload_routes.get_value());
        }
    };
    
    // Add on_mount to trigger when route becomes active
    create_effect(move |_| {
        on_mount();
    });
    
    view! {
        <Route path={path} view=move || {
            let route_view = view();
            view! {
                {route_view}
            }
        }} />
    }
}

/// Manager for preloading routes
#[derive(Clone)]
pub struct PreloadManager {
    // Map of route -> routes to preload
    preload_map: std::rc::Rc<std::cell::RefCell<HashMap<String, Vec<PreloadRoute>>>>,
    // Currently active route
    active_route: std::rc::Rc<std::cell::RefCell<Option<String>>>,
    // Routes that have been preloaded
    preloaded: std::rc::Rc<std::cell::RefCell<HashMap<String, bool>>>,
}

impl PreloadManager {
    pub fn new() -> Self {
        Self {
            preload_map: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
            active_route: std::rc::Rc::new(std::cell::RefCell::new(None)),
            preloaded: std::rc::Rc::new(std::cell::RefCell::new(HashMap::new())),
        }
    }
    
    /// Register routes that should be preloaded when a specific route is active
    pub fn register_route_preloads(&self, route: String, preload_routes: Vec<PreloadRoute>) {
        let mut map = self.preload_map.borrow_mut();
        map.insert(route.clone(), preload_routes);
        
        // If this is the current route, begin preloading immediately
        let active_route = self.active_route.borrow().clone();
        if let Some(active) = active_route {
            if active == route {
                self.preload_for_route(&route);
            }
        }
    }
    
    /// Start monitoring for route changes
    pub fn start_monitoring(&self) {
        let navigate_listener = window_event_listener("popstate", {
            let this = self.clone();
            move |_| {
                // Update active route
                if let Some(location) = window().location().pathname().ok() {
                    let mut active_route = this.active_route.borrow_mut();
                    *active_route = Some(location.clone());
                    drop(active_route);
                    
                    // Trigger preloading
                    this.preload_for_route(&location);
                }
            }
        });

        // Store the listener to prevent it from being dropped
        provide_context(navigate_listener);
        
        // Also check initial route
        if let Some(location) = window().location().pathname().ok() {
            let mut active_route = self.active_route.borrow_mut();
            *active_route = Some(location.clone());
            drop(active_route);
            
            // Trigger preloading for initial route
            self.preload_for_route(&location);
        }
    }
    
    /// Preload resources for the current route
    fn preload_for_route(&self, route: &str) {
        let map = self.preload_map.borrow();
        
        if let Some(routes_to_preload) = map.get(route) {
            let mut preloaded = self.preloaded.borrow_mut();
            
            for preload_route in routes_to_preload {
                let path = preload_route.to_path();
                
                // Only preload if not already done
                if !preloaded.contains_key(&path) {
                    log::debug!("Preloading route: {}", path);
                    
                    // Mark as preloaded
                    preloaded.insert(path.clone(), true);
                    
                    // Trigger preload
                    self.trigger_preload(&path);
                }
            }
        }
    }
    
    /// Actually trigger the preload
    fn trigger_preload(&self, path: &str) {
        // Use fetch API to preload the page content
        let path = path.to_string();
        
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let base_url = window.location().origin().unwrap();
            let url = format!("{}{}", base_url, path);
            
            // Use a custom header to indicate this is a preload request
            let mut opts = web_sys::RequestInit::new();
            opts.method("GET");
            opts.credentials(web_sys::RequestCredentials::SameOrigin);
            
            let headers = web_sys::Headers::new().unwrap();
            headers.append("X-Purpose", "preload").unwrap();
            opts.headers(&headers);
            
            // Make the request
            match web_sys::Request::new_with_str_and_init(&url, &opts) {
                Ok(request) => {
                    if let Ok(promise) = window.fetch_with_request(&request) {
                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                        log::debug!("Preloaded: {}", path);
                    }
                },
                Err(e) => {
                    log::warn!("Failed to create preload request: {:?}", e);
                }
            }
        });
    }
}