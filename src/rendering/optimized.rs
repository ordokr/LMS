use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node, HtmlElement};

// Rendering statistics tracker to identify bottlenecks
#[derive(Default, Clone)]
pub struct RenderStats {
    component_renders: Rc<RefCell<HashMap<String, usize>>>,
    dom_operations: AtomicUsize,
    largest_update_nodes: Rc<RefCell<usize>>,
    total_render_time_ms: Rc<RefCell<Vec<f64>>>,
    excessive_renders: Rc<RefCell<HashSet<String>>>,
}

impl RenderStats {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn track_component_render(&self, component_name: &str) {
        let mut renders = self.component_renders.borrow_mut();
        *renders.entry(component_name.to_string()).or_insert(0) += 1;
        
        // Track excessive renders (more than 5 renders in short succession)
        if *renders.get(component_name).unwrap() > 5 {
            let mut excessive = self.excessive_renders.borrow_mut();
            excessive.insert(component_name.to_string());
        }
    }
    
    pub fn track_dom_operation(&self) {
        self.dom_operations.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn track_render_time(&self, time_ms: f64) {
        let mut times = self.total_render_time_ms.borrow_mut();
        times.push(time_ms);
        
        // Keep only the last 100 measurements
        if times.len() > 100 {
            times.remove(0);
        }
    }
    
    pub fn get_stats(&self) -> HashMap<String, String> {
        let mut stats = HashMap::new();
        
        // Average render time
        let times = self.total_render_time_ms.borrow();
        if !times.is_empty() {
            let avg_time = times.iter().sum::<f64>() / times.len() as f64;
            stats.insert("avg_render_time_ms".to_string(), format!("{:.2}", avg_time));
        }
        
        // Top 5 most frequently rendered components
        let renders = self.component_renders.borrow();
        let mut render_counts: Vec<_> = renders.iter().collect();
        render_counts.sort_by(|a, b| b.1.cmp(a.1));
        
        let top_renders = render_counts.iter()
            .take(5)
            .map(|(name, count)| format!("{}: {}", name, count))
            .collect::<Vec<_>>()
            .join(", ");
            
        stats.insert("top_renders".to_string(), top_renders);
        
        // Excessive renders
        let excessive = self.excessive_renders.borrow();
        if !excessive.is_empty() {
            let excessive_list = excessive.iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
                
            stats.insert("excessive_renders".to_string(), excessive_list);
        }
        
        // DOM operations
        stats.insert(
            "dom_operations".to_string(), 
            self.dom_operations.load(Ordering::Relaxed).to_string()
        );
        
        stats
    }
    
    pub fn reset(&self) {
        self.component_renders.borrow_mut().clear();
        self.dom_operations.store(0, Ordering::Relaxed);
        self.total_render_time_ms.borrow_mut().clear();
        self.excessive_renders.borrow_mut().clear();
    }
}

// Global instance
thread_local! {
    static RENDER_STATS: RenderStats = RenderStats::new();
}

// Instrumentation for components
#[hook]
pub fn use_performance_tracking(component_name: &str) {
    let component_name = component_name.to_string();
    
    create_effect(move |_| {
        let start = web_sys::window()
            .unwrap()
            .performance()
            .unwrap()
            .now();
            
        RENDER_STATS.with(|stats| {
            stats.track_component_render(&component_name);
        });
        
        leptos::request_animation_frame(move || {
            let end = web_sys::window()
                .unwrap()
                .performance()
                .unwrap()
                .now();
                
            let duration = end - start;
            
            RENDER_STATS.with(|stats| {
                stats.track_render_time(duration);
            });
        });
    });
}

// Optimized rendering helper functions
pub fn lazy_memo<T, F>(
    source: impl Fn() -> bool + 'static,
    f: F,
) -> impl Fn() -> T
where
    T: Clone + 'static,
    F: Fn() -> T + 'static,
{
    let (value, set_value) = create_signal(None::<T>);
    
    create_effect(move |_| {
        if source() {
            set_value.set(Some(f()));
        }
    });
    
    move || value.get().unwrap_or_else(f)
}

// DOM operation tracker
pub fn track_dom_ops<F>(f: F)
where
    F: FnOnce(),
{
    RENDER_STATS.with(|stats| {
        stats.track_dom_operation();
    });
    
    f();
}

// Export stats for debugging
#[wasm_bindgen]
pub fn get_render_stats() -> JsValue {
    let stats = RENDER_STATS.with(|s| s.get_stats());
    JsValue::from_serde(&stats).unwrap()
}

#[wasm_bindgen]
pub fn reset_render_stats() {
    RENDER_STATS.with(|s| s.reset());
}

// Optimized event delegation system
pub struct EventDelegation {
    root: Element,
    handlers: Rc<RefCell<HashMap<String, Box<dyn Fn(&web_sys::Event)>>>>,
}

impl EventDelegation {
    pub fn new(root_selector: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root = document
            .query_selector(root_selector)
            .unwrap()
            .unwrap_or_else(|| document.body().unwrap().dyn_into::<Element>().unwrap());
        
        let instance = Self {
            root,
            handlers: Rc::new(RefCell::new(HashMap::new())),
        };
        
        instance.setup_delegation();
        instance
    }
    
    fn setup_delegation(&self) {
        let handlers = self.handlers.clone();
        
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let current_elem = target.dyn_ref::<Element>();
            
            if let Some(elem) = current_elem {
                // Find element with data-event attribute
                let mut current = Some(elem.clone());
                
                while let Some(element) = current {
                    if let Some(event_name) = element.get_attribute("data-event") {
                        let handlers_ref = handlers.borrow();
                        if let Some(handler) = handlers_ref.get(&event_name) {
                            handler(&event);
                            break;
                        }
                    }
                    
                    current = element.parent_element();
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        // Add global event listeners
        self.root.add_event_listener_with_callback(
            "click",
            closure.as_ref().unchecked_ref(),
        ).unwrap();
        
        // Prevent closure from being dropped
        closure.forget();
    }
    
    pub fn register<F>(&self, event_name: &str, handler: F)
    where
        F: Fn(&web_sys::Event) + 'static,
    {
        let mut handlers = self.handlers.borrow_mut();
        handlers.insert(event_name.to_string(), Box::new(handler));
    }
}

// Example of optimized render-once component
#[component]
pub fn RenderOnce<F, V>(
    render_fn: F,
    #[prop(optional)] deps: Vec<ReadSignal<dyn std::any::Any>>,
) -> impl IntoView
where
    F: Fn() -> V + 'static,
    V: IntoView + 'static,
{
    let rendered = create_memo(move |_| {
        // This will only run once, unless deps change
        render_fn().into_view()
    });
    
    view! { {move || rendered.get()} }
}