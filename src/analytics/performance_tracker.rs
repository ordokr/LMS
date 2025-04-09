use leptos::*;
use web_sys::{Performance, PerformanceEntry, PerformanceMark, PerformanceMeasure};
use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use once_cell::sync::Lazy;

// Performance metrics tracking
#[derive(Clone, Debug)]
pub struct PerformanceMetrics {
    // Track page load timings
    navigation_timing: RwSignal<Option<NavigationTiming>>,
    // Track component render times
    component_renders: RwSignal<HashMap<String, RenderMetrics>>,
    // Track resource loading
    resource_timing: RwSignal<Vec<ResourceTiming>>,
    // Interaction metrics
    interactions: RwSignal<VecDeque<InteractionMetrics>>,
    // Web Vitals
    cls: RwSignal<Option<f64>>,
    fid: RwSignal<Option<f64>>,
    lcp: RwSignal<Option<f64>>,
    // Enabled state
    enabled: RwSignal<bool>,
}

// Page navigation timing
#[derive(Clone, Debug)]
pub struct NavigationTiming {
    dns: f64,              // DNS lookup time
    connect: f64,          // Connection time
    ttfb: f64,             // Time to first byte
    dom_interactive: f64,  // DOM ready for interaction
    dom_complete: f64,     // DOM fully loaded
    load_event: f64,       // Page load complete
}

// Component render metrics
#[derive(Clone, Debug)]
pub struct RenderMetrics {
    count: u32,                  // Number of renders
    total_time_ms: f64,          // Total rendering time
    avg_render_time_ms: f64,     // Average render time
    max_render_time_ms: f64,     // Maximum render time
    last_render_timestamp: f64,  // Last render timestamp
}

// Resource loading metrics
#[derive(Clone, Debug)]
pub struct ResourceTiming {
    url: String,           // Resource URL
    resource_type: String, // Resource type (script, style, image, etc)
    duration: f64,         // Total loading time
    size: Option<u64>,     // Resource size in bytes
    is_cached: bool,       // Whether resource was served from cache
}

// User interaction metrics
#[derive(Clone, Debug)]
pub struct InteractionMetrics {
    event_type: String,    // Click, input, etc
    target: String,        // Target element description
    response_time_ms: f64, // Time to response
    timestamp: f64,        // When it occurred
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        let instance = Self {
            navigation_timing: create_rw_signal(None),
            component_renders: create_rw_signal(HashMap::new()),
            resource_timing: create_rw_signal(Vec::new()),
            interactions: create_rw_signal(VecDeque::with_capacity(100)),
            cls: create_rw_signal(None),
            fid: create_rw_signal(None),
            lcp: create_rw_signal(None),
            enabled: create_rw_signal(cfg!(debug_assertions)), // Only enabled in debug by default
        };
        
        // Initialize metrics collection
        if *instance.enabled.get_value() {
            instance.init();
        }
        
        instance
    }
    
    // Initialize metrics collection
    fn init(&self) {
        self.collect_navigation_timing();
        self.setup_performance_observer();
        self.setup_interaction_observer();
    }
    
    // Enable or disable metrics collection
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.set(enabled);
        
        if enabled && self.navigation_timing.get().is_none() {
            self.init();
        }
    }
    
    // Collect navigation timing metrics
    fn collect_navigation_timing(&self) {
        let window = web_sys::window().expect("window should be available");
        let performance = window.performance().expect("performance should be available");
        
        // Use Performance API to get navigation timing
        if let Ok(timing) = js_sys::Reflect::get(&performance, &JsValue::from_str("timing")) {
            let timing = timing.unchecked_into::<web_sys::PerformanceTiming>();
            
            let nav_start = timing.navigation_start() as f64;
            let dns_start = timing.domain_lookup_start() as f64;
            let dns_end = timing.domain_lookup_end() as f64;
            let connect_start = timing.connect_start() as f64;
            let connect_end = timing.connect_end() as f64;
            let response_start = timing.response_start() as f64;
            let dom_interactive = timing.dom_interactive() as f64;
            let dom_complete = timing.dom_complete() as f64;
            let load_event_end = timing.load_event_end() as f64;
            
            self.navigation_timing.set(Some(NavigationTiming {
                dns: dns_end - dns_start,
                connect: connect_end - connect_start,
                ttfb: response_start - nav_start,
                dom_interactive: dom_interactive - nav_start,
                dom_complete: dom_complete - nav_start,
                load_event: load_event_end - nav_start,
            }));
        }
    }
    
    // Setup PerformanceObserver to track resource loading and marks/measures
    fn setup_performance_observer(&self) {
        let window = web_sys::window().expect("window should be available");
        let performance = window.performance().expect("performance should be available");
        
        // Track existing resource entries
        if let Ok(entries) = performance.get_entries() {
            for i in 0..entries.length() {
                if let Some(entry) = entries.get(i) {
                    self.process_performance_entry(&entry);
                }
            }
        }
        
        // Setup observer for new entries
        let callback = Closure::wrap(Box::new(move |entries: JsValue, _observer: JsValue| {
            let entries = entries.unchecked_into::<js_sys::Array>();
            for i in 0..entries.length() {
                if let Some(entry) = entries.get(i) {
                    METRICS.process_performance_entry(&entry);
                }
            }
        }) as Box<dyn FnMut(JsValue, JsValue)>);
        
        // Create observer
        if let Ok(observer_class) = js_sys::Reflect::get(&window, &JsValue::from_str("PerformanceObserver")) {
            let observer_class = observer_class.unchecked_into::<js_sys::Function>();
            let observer = observer_class.new_with_args(&JsValue::from(callback.as_ref())).unwrap();
            
            // Observe resource, measure, and paint entries
            let options = js_sys::Object::new();
            let entry_types = js_sys::Array::new();
            entry_types.push(&JsValue::from_str("resource"));
            entry_types.push(&JsValue::from_str("measure"));
            entry_types.push(&JsValue::from_str("mark"));
            entry_types.push(&JsValue::from_str("paint"));
            entry_types.push(&JsValue::from_str("largest-contentful-paint"));
            
            js_sys::Reflect::set(&options, &JsValue::from_str("entryTypes"), &entry_types).unwrap();
            
            let _ = js_sys::Reflect::apply(
                &js_sys::Reflect::get(&observer, &JsValue::from_str("observe")).unwrap(),
                &observer,
                &js_sys::Array::of2(&JsValue::from(options), &JsValue::NULL),
            );
            
            callback.forget();
        }
    }
    
    // Setup observers for user interactions
    fn setup_interaction_observer(&self) {
        let document = web_sys::window()
            .expect("window should be available")
            .document()
            .expect("document should be available");
            
        // Track clicks
        let click_callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let target = event.target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .map(|e| e.tag_name())
                .unwrap_or_else(|| "unknown".to_string());
                
            METRICS.track_interaction("click", &target, event.time_stamp());
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        
        document.add_event_listener_with_callback(
            "click",
            click_callback.as_ref().unchecked_ref(),
        ).unwrap();
        click_callback.forget();
        
        // Track inputs
        let input_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let target = event.target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .map(|e| e.tag_name())
                .unwrap_or_else(|| "unknown".to_string());
                
            METRICS.track_interaction("input", &target, event.time_stamp());
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        document.add_event_listener_with_callback(
            "input",
            input_callback.as_ref().unchecked_ref(),
        ).unwrap();
        input_callback.forget();
    }
    
    // Process a performance entry
    fn process_performance_entry(&self, entry: &JsValue) {
        if !*self.enabled.get_value() {
            return;
        }
        
        let entry_type = js_sys::Reflect::get(&entry, &JsValue::from_str("entryType"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
            
        match entry_type.as_str() {
            "resource" => self.process_resource_entry(entry),
            "measure" => self.process_measure_entry(entry),
            "paint" => self.process_paint_entry(entry),
            "largest-contentful-paint" => self.process_lcp_entry(entry),
            _ => {}
        }
    }
    
    // Process resource timing entry
    fn process_resource_entry(&self, entry: &JsValue) {
        let name = js_sys::Reflect::get(&entry, &JsValue::from_str("name"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
            
        let duration = js_sys::Reflect::get(&entry, &JsValue::from_str("duration"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or_default();
            
        let initiator_type = js_sys::Reflect::get(&entry, &JsValue::from_str("initiatorType"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
            
        // Try to get transferred size if available
        let size = js_sys::Reflect::get(&entry, &JsValue::from_str("transferSize"))
            .ok()
            .and_then(|v| v.as_f64())
            .map(|s| s as u64);
            
        // Determine if resource was cached
        let encoded_size = js_sys::Reflect::get(&entry, &JsValue::from_str("encodedBodySize"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or_default();
            
        let transfer_size = js_sys::Reflect::get(&entry, &JsValue::from_str("transferSize"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or_default();
            
        let is_cached = transfer_size == 0.0 && encoded_size > 0.0;
        
        self.resource_timing.update(|timings| {
            timings.push(ResourceTiming {
                url: name,
                resource_type: initiator_type,
                duration,
                size,
                is_cached,
            });
            
            // Limit size of timings array
            if timings.len() > 100 {
                *timings = timings.split_off(timings.len() - 100);
            }
        });
    }
    
    // Process performance measure entry
    fn process_measure_entry(&self, entry: &JsValue) {
        let name = js_sys::Reflect::get(&entry, &JsValue::from_str("name"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
            
        let duration = js_sys::Reflect::get(&entry, &JsValue::from_str("duration"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or_default();
            
        let start_time = js_sys::Reflect::get(&entry, &JsValue::from_str("startTime"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or_default();
            
        // Check if this is a component render measure
        if name.starts_with("render_") {
            let component_name = name.strip_prefix("render_").unwrap_or(&name).to_string();
            
            self.component_renders.update(|renders| {
                let metrics = renders.entry(component_name).or_insert_with(|| RenderMetrics {
                    count: 0,
                    total_time_ms: 0.0,
                    avg_render_time_ms: 0.0,
                    max_render_time_ms: 0.0,
                    last_render_timestamp: 0.0,
                });
                
                metrics.count += 1;
                metrics.total_time_ms += duration;
                metrics.avg_render_time_ms = metrics.total_time_ms / metrics.count as f64;
                metrics.max_render_time_ms = metrics.max_render_time_ms.max(duration);
                metrics.last_render_timestamp = start_time;
            });
        }
    }
    
    // Process paint timing entries
    fn process_paint_entry(&self, entry: &JsValue) {
        let name = js_sys::Reflect::get(&entry, &JsValue::from_str("name"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();
            
        let start_time = js_sys::Reflect::get(&entry, &JsValue::from_str("startTime"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or_default();
            
        // Track First Contentful Paint
        if name == "first-contentful-paint" {
            // FCP is not exposed directly but used for calculating LCP
        }
    }
    
    // Process Largest Contentful Paint entry
    fn process_lcp_entry(&self, entry: &JsValue) {
        let start_time = js_sys::Reflect::get(&entry, &JsValue::from_str("startTime"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or_default();
            
        self.lcp.set(Some(start_time));
    }
    
    // Track user interaction
    fn track_interaction(&self, event_type: &str, target: &str, timestamp: f64) {
        if !*self.enabled.get_value() {
            return;
        }
        
        // Create performance mark for calculating response time
        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                let _ = performance.mark(&format!("interaction_start_{}", timestamp));
                
                // Use rAF to measure when the browser responds to the interaction
                let callback = Closure::once_into_js(move || {
                    if let Some(window) = web_sys::window() {
                        if let Some(performance) = window.performance() {
                            let _ = performance.mark(&format!("interaction_end_{}", timestamp));
                            let _ = performance.measure(
                                &format!("interaction_time_{}", timestamp),
                                &format!("interaction_start_{}", timestamp),
                                &format!("interaction_end_{}", timestamp),
                            );
                            
                            // Clear marks
                            let _ = performance.clear_marks(&format!("interaction_start_{}", timestamp));
                            let _ = performance.clear_marks(&format!("interaction_end_{}", timestamp));
                            
                            // Get the measure
                            if let Ok(measures) = performance.get_entries_by_name(&format!("interaction_time_{}", timestamp)) {
                                if measures.length() > 0 {
                                    if let Some(measure) = measures.get(0) {
                                        if let Ok(duration) = js_sys::Reflect::get(&measure, &JsValue::from_str("duration")) {
                                            if let Some(duration) = duration.as_f64() {
                                                // Record the interaction
                                                METRICS.record_interaction_time(event_type, target, duration, timestamp);
                                            }
                                        }
                                    }
                                }
                                // Clear measure
                                let _ = performance.clear_measures(&format!("interaction_time_{}", timestamp));
                            }
                        }
                    }
                });
                
                // Request animation frame to measure when browser responds
                window.request_animation_frame(callback.as_ref().unchecked_ref()).unwrap();
            }
        }
    }
    
    // Record interaction response time
    fn record_interaction_time(&self, event_type: &str, target: &str, duration: f64, timestamp: f64) {
        if !*self.enabled.get_value() {
            return;
        }
        
        self.interactions.update(|interactions| {
            interactions.push_back(InteractionMetrics {
                event_type: event_type.to_string(),
                target: target.to_string(),
                response_time_ms: duration,
                timestamp,
            });
            
            // Limit queue size
            while interactions.len() > 100 {
                interactions.pop_front();
            }
        });
        
        // Update First Input Delay if this is the first input
        if self.fid.get().is_none() {
            self.fid.set(Some(duration));
        }
    }
    
    // Start measuring component render time
    pub fn start_component_render(&self, component_name: &str) {
        if !*self.enabled.get_value() {
            return;
        }
        
        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                let _ = performance.mark(&format!("render_start_{}", component_name));
            }
        }
    }
    
    // End measuring component render time
    pub fn end_component_render(&self, component_name: &str) {
        if !*self.enabled.get_value() {
            return;
        }
        
        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                let _ = performance.mark(&format!("render_end_{}", component_name));
                let _ = performance.measure(
                    &format!("render_{}", component_name),
                    &format!("render_start_{}", component_name),
                    &format!("render_end_{}", component_name),
                );
                
                // Clear marks
                let _ = performance.clear_marks(&format!("render_start_{}", component_name));
                let _ = performance.clear_marks(&format!("render_end_{}", component_name));
            }
        }
    }
    
    // Get performance summary
    pub fn get_summary(&self) -> PerformanceSummary {
        PerformanceSummary {
            navigation: self.navigation_timing.get(),
            resources: ResourceSummary {
                total_count: self.resource_timing.get().len(),
                total_size: self.resource_timing.get().iter()
                    .filter_map(|r| r.size)
                    .sum(),
                cached_count: self.resource_timing.get().iter()
                    .filter(|r| r.is_cached)
                    .count(),
                avg_load_time: self.resource_timing.get().iter()
                    .map(|r| r.duration)
                    .sum::<f64>() / self.resource_timing.get().len().max(1) as f64,
            },
            components: self.component_renders.get().iter()
                .map(|(name, metrics)| (name.clone(), metrics.clone()))
                .collect(),
            web_vitals: WebVitals {
                lcp: self.lcp.get(),
                fid: self.fid.get(),
                cls: self.cls.get(),
            },
            interactions: {
                let interactions = self.interactions.get();
                let count = interactions.len();
                let avg_response_time = if count > 0 {
                    interactions.iter()
                        .map(|i| i.response_time_ms)
                        .sum::<f64>() / count as f64
                } else {
                    0.0
                };
                
                InteractionSummary {
                    count,
                    avg_response_time,
                }
            },
        }
    }
}

// Performance summary for reporting
#[derive(Clone, Debug)]
pub struct PerformanceSummary {
    pub navigation: Option<NavigationTiming>,
    pub resources: ResourceSummary,
    pub components: HashMap<String, RenderMetrics>,
    pub web_vitals: WebVitals,
    pub interactions: InteractionSummary,
}

#[derive(Clone, Debug)]
pub struct ResourceSummary {
    pub total_count: usize,
    pub total_size: u64,
    pub cached_count: usize,
    pub avg_load_time: f64,
}

#[derive(Clone, Debug)]
pub struct WebVitals {
    pub lcp: Option<f64>,
    pub fid: Option<f64>,
    pub cls: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct InteractionSummary {
    pub count: usize,
    pub avg_response_time: f64,
}

// Global metrics instance
static METRICS: Lazy<PerformanceMetrics> = Lazy::new(|| PerformanceMetrics::new());

// Hooks for components to use
#[hook]
pub fn use_performance_metrics() -> &'static PerformanceMetrics {
    &METRICS
}

// Component wrapper for measuring render time
#[component]
pub fn MeasureRender<F, IV>(
    #[prop(into)] name: String,
    #[prop(into)] children: F,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let metrics = use_performance_metrics();
    
    // Start measuring
    metrics.start_component_render(&name);
    
    // Render children
    let view = children();
    
    // End measuring in next tick to include rendering time
    create_effect(move |_| {
        set_timeout(move || {
            metrics.end_component_render(&name);
        }, std::time::Duration::from_millis(0));
    });
    
    view
}

// Hook to get performance summary
#[hook]
pub fn use_performance_summary() -> Signal<PerformanceSummary> {
    let (summary, set_summary) = create_signal(METRICS.get_summary());
    
    // Update summary every 5 seconds
    let interval = set_interval(
        move || set_summary.set(METRICS.get_summary()),
        std::time::Duration::from_secs(5),
    );
    
    on_cleanup(move || {
        clear_interval(interval);
    });
    
    summary
}