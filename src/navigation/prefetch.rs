use leptos::*;
use leptos_router::*;
use futures::future::BoxFuture;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;

// Route-based prefetching system
pub struct PrefetchManager {
    fetch_handlers: Rc<RefCell<HashMap<String, Box<dyn Fn() -> BoxFuture<'static, ()>>>>>,
    fetched_routes: Rc<RefCell<HashMap<String, bool>>>,
    prefetch_timeout: Option<Timeout>,
    hover_timeout: Option<Timeout>,
}

impl PrefetchManager {
    pub fn new() -> Self {
        Self {
            fetch_handlers: Rc::new(RefCell::new(HashMap::new())),
            fetched_routes: Rc::new(RefCell::new(HashMap::new())),
            prefetch_timeout: None,
            hover_timeout: None,
        }
    }
    
    pub fn register_prefetch<F, Fut>(&mut self, route: &str, handler: F)
    where
        F: Fn() -> Fut + 'static,
        Fut: std::future::Future<Output = ()> + 'static,
    {
        let mut handlers = self.fetch_handlers.borrow_mut();
        
        handlers.insert(
            route.to_string(), 
            Box::new(move || Box::pin(handler()))
        );
    }
    
    pub fn prefetch(&mut self, route: &str) {
        if self.fetched_routes.borrow().contains_key(route) {
            return; // Already prefetched
        }
        
        let handlers = self.fetch_handlers.clone();
        let fetched_routes = self.fetched_routes.clone();
        let route_str = route.to_string();
        
        self.prefetch_timeout = None;
        
        // Prefetch the route data
        spawn_local(async move {
            if let Some(handler) = handlers.borrow().get(&route_str) {
                handler().await;
                fetched_routes.borrow_mut().insert(route_str, true);
            }
        });
    }
    
    pub fn on_link_hover(&mut self, route: &str) {
        let route = route.to_string();
        
        // Clear any existing timers
        self.hover_timeout = None;
        
        // Set new timer for prefetch
        let manager = self.clone();
        self.hover_timeout = Some(Timeout::new(200, move || {
            manager.prefetch(&route);
        }));
    }
}

impl Clone for PrefetchManager {
    fn clone(&self) -> Self {
        Self {
            fetch_handlers: self.fetch_handlers.clone(),
            fetched_routes: self.fetched_routes.clone(),
            prefetch_timeout: None,
            hover_timeout: None,
        }
    }
}

// Create a prefetchable link component
#[component]
pub fn PrefetchLink(
    #[prop(into)] to: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] active_class: Option<String>,
    children: Children,
) -> impl IntoView {
    // Get prefetch manager from context
    let prefetch_manager = use_context::<PrefetchManager>().unwrap();
    
    // Clone for closures
    let to_hover = to.clone();
    let manager_hover = prefetch_manager.clone();
    
    view! {
        <a
            href={to.clone()}
            class={class}
            class:active={move || is_active_route(&to)}
            class={active_class.unwrap_or_default()}
            on:mouseenter=move |_| {
                manager_hover.on_link_hover(&to_hover);
            }
        >
            {children()}
        </a>
    }
}

// Helper function to check if route is active
fn is_active_route(route: &str) -> bool {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let path = location.pathname().unwrap();
    
    path == route || path.starts_with(&format!("{}/", route))
}

// High-performance router with data prefetching
#[component]
pub fn OptimizedRouter(children: Children) -> impl IntoView {
    // Create and provide prefetch manager
    let prefetch_manager = PrefetchManager::new();
    provide_context(prefetch_manager.clone());
    
    // Add navigation observer
    create_effect(move |_| {
        // Get current route
        let window = web_sys::window().unwrap();
        let location = window.location();
        let path = location.pathname().unwrap();
        
        // Prefetch potential next routes based on current route
        let manager = prefetch_manager.clone();
        
        match path.as_str() {
            "/" => {
                // On homepage, prefetch forum index
                spawn_local(async move {
                    // Small delay to allow initial render to complete
                    gloo_timers::future::TimeoutFuture::new(300).await;
                    manager.prefetch("/forum");
                });
            },
            "/forum" => {
                // On forum index, prefetch popular categories
                spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(300).await;
                    manager.prefetch("/forum/categories/1");
                    manager.prefetch("/forum/categories/2");
                });
            },
            _ => {}
        }
    });
    
    view! {
        <Router>
            {children()}
        </Router>
    }
}