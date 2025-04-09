use leptos::*;
use wasm_bindgen::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::VisibilityState;
use std::time::{Duration, Instant};

// Global prefetcher that works across components
#[derive(Clone)]
pub struct Prefetcher {
    completed: Rc<RefCell<HashSet<String>>>,
    in_progress: Rc<RefCell<HashSet<String>>>,
    priority_queue: Rc<RefCell<Vec<(String, i32)>>>,
    network_idle_callback: Rc<RefCell<Option<Box<dyn Fn() + 'static>>>>,
    is_processing: Rc<RefCell<bool>>,
    bandwidth_estimate_kbps: Rc<RefCell<Option<f64>>>,
}

#[derive(Clone)]
pub enum PrefetchPriority {
    High = 3,    // User is likely to need this soon
    Medium = 2,  // User might need this
    Low = 1,     // Prefetch when idle
}

impl Prefetcher {
    pub fn new() -> Self {
        let instance = Self {
            completed: Rc::new(RefCell::new(HashSet::new())),
            in_progress: Rc::new(RefCell::new(HashSet::new())),
            priority_queue: Rc::new(RefCell::new(Vec::new())),
            network_idle_callback: Rc::new(RefCell::new(None)),
            is_processing: Rc::new(RefCell::new(false)),
            bandwidth_estimate_kbps: Rc::new(RefCell::new(None)),
        };
        
        // Setup idle callbacks
        instance.setup_idle_detection();
        
        // Start the queue processor
        let instance_clone = instance.clone();
        spawn_local(async move {
            instance_clone.process_queue().await;
        });
        
        instance
    }
    
    // Queue a resource for prefetching
    pub fn queue(
        &self, 
        url: &str, 
        priority: PrefetchPriority,
        deps: Option<Vec<String>>
    ) {
        let url = url.to_string();
        
        // Don't queue if already fetched or in progress
        if self.completed.borrow().contains(&url) || 
           self.in_progress.borrow().contains(&url) {
            return;
        }
        
        // Check if dependencies are met
        if let Some(deps) = deps {
            let completed = self.completed.borrow();
            for dep in &deps {
                if !completed.contains(dep) {
                    // Dependency not met, don't queue yet
                    return;
                }
            }
        }
        
        // Add to queue with priority
        let priority_value = priority as i32;
        self.priority_queue.borrow_mut().push((url, priority_value));
        
        // Sort queue by priority (higher first)
        self.priority_queue.borrow_mut().sort_by(|a, b| b.1.cmp(&a.1));
    }
    
    // Process prefetch queue
    async fn process_queue(&self) {
        loop {
            // Check if we have items to process
            let (url, priority) = {
                let mut queue = self.priority_queue.borrow_mut();
                if queue.is_empty() {
                    *self.is_processing.borrow_mut() = false;
                    // Wait a bit before checking again
                    TimeoutFuture::new(250).await;
                    continue;
                }
                
                *self.is_processing.borrow_mut() = true;
                queue.remove(0)
            };
            
            // Check again if already processed
            if self.completed.borrow().contains(&url) || 
               self.in_progress.borrow().contains(&url) {
                continue;
            }
            
            // Mark as in progress
            self.in_progress.borrow_mut().insert(url.clone());
            
            // Fetch the resource based on its type
            let result = if url.ends_with(".js") {
                self.fetch_script(&url).await
            } else if url.ends_with(".css") {
                self.fetch_stylesheet(&url).await
            } else if url.ends_with(".json") {
                self.fetch_json(&url).await
            } else {
                self.fetch_html(&url).await
            };
            
            // Update tracking
            self.in_progress.borrow_mut().remove(&url);
            
            if result.is_ok() {
                self.completed.borrow_mut().insert(url);
            }
            
            // Add delay between fetches based on priority
            let delay_ms = match priority {
                3 => 0,    // High priority: no delay
                2 => 100,  // Medium priority: small delay
                _ => 500,  // Low priority: larger delay
            };
            
            if delay_ms > 0 {
                TimeoutFuture::new(delay_ms).await;
            }
        }
    }
    
    // Prefetch JS script
    async fn fetch_script(&self, url: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;
        let head = document.head().ok_or("No head element")?;
        
        // Check if script is already in the DOM
        let scripts = head.get_elements_by_tag_name("script");
        for i in 0..scripts.length() {
            if let Some(script) = scripts.item(i) {
                if let Some(script_elem) = script.dyn_ref::<web_sys::HtmlScriptElement>() {
                    if script_elem.src() == url {
                        return Ok(());
                    }
                }
            }
        }
        
        // Create a script element with prefetch attributes
        let script = document.create_element("link")
            .map_err(|_| "Failed to create link element")?;
        
        script.set_attribute("rel", "prefetch")
            .map_err(|_| "Failed to set rel attribute")?;
        script.set_attribute("href", url)
            .map_err(|_| "Failed to set href attribute")?;
        script.set_attribute("as", "script")
            .map_err(|_| "Failed to set as attribute")?;
        
        // Append to head
        head.append_child(&script)
            .map_err(|_| "Failed to append link element")?;
        
        Ok(())
    }
    
    // Prefetch CSS stylesheet
    async fn fetch_stylesheet(&self, url: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;
        let head = document.head().ok_or("No head element")?;
        
        // Check if stylesheet is already in the DOM
        let links = head.get_elements_by_tag_name("link");
        for i in 0..links.length() {
            if let Some(link) = links.item(i) {
                if let Some(link_elem) = link.dyn_ref::<web_sys::HtmlLinkElement>() {
                    if link_elem.href() == url {
                        return Ok(());
                    }
                }
            }
        }
        
        // Create a link element with prefetch attributes
        let link = document.create_element("link")
            .map_err(|_| "Failed to create link element")?;
        
        link.set_attribute("rel", "prefetch")
            .map_err(|_| "Failed to set rel attribute")?;
        link.set_attribute("href", url)
            .map_err(|_| "Failed to set href attribute")?;
        link.set_attribute("as", "style")
            .map_err(|_| "Failed to set as attribute")?;
        
        // Append to head
        head.append_child(&link)
            .map_err(|_| "Failed to append link element")?;
        
        Ok(())
    }
    
    // Prefetch JSON data
    async fn fetch_json(&self, url: &str) -> Result<(), String> {
        let start = web_sys::window()
            .ok_or("No window available")?
            .performance()
            .ok_or("No performance API available")?
            .now();
            
        let response = reqwasm::http::Request::get(url)
            .header("X-Prefetch", "true")
            .header("Purpose", "prefetch")
            .send()
            .await
            .map_err(|e| format!("Failed to prefetch JSON: {}", e))?;
            
        let end = web_sys::window()
            .ok_or("No window available")?
            .performance()
            .ok_or("No performance API available")?
            .now();
            
        // Calculate bandwidth estimate
        let size_bytes = response
            .headers()
            .get("content-length")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
            
        if size_bytes > 0.0 {
            let duration_sec = (end - start) / 1000.0;
            if duration_sec > 0.0 {
                let bandwidth_kbps = (size_bytes / 1024.0) / duration_sec;
                *self.bandwidth_estimate_kbps.borrow_mut() = Some(bandwidth_kbps);
            }
        }
        
        Ok(())
    }
    
    // Prefetch HTML page
    async fn fetch_html(&self, url: &str) -> Result<(), String> {
        // Use prerender hint
        let window = web_sys::window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;
        let head = document.head().ok_or("No head element")?;
        
        // Create a link element with prefetch attributes
        let link = document.create_element("link")
            .map_err(|_| "Failed to create link element")?;
        
        link.set_attribute("rel", "prerender")
            .map_err(|_| "Failed to set rel attribute")?;
        link.set_attribute("href", url)
            .map_err(|_| "Failed to set href attribute")?;
        
        // Append to head
        head.append_child(&link)
            .map_err(|_| "Failed to append link element")?;
        
        Ok(())
    }
    
    // Setup idle detection to process low-priority prefetches
    fn setup_idle_detection(&self) {
        // Watch for network idle
        let self_clone = self.clone();
        let callback = Closure::wrap(Box::new(move || {
            if let Some(callback) = &*self_clone.network_idle_callback.borrow() {
                callback();
            }
        }) as Box<dyn FnMut()>);
        
        if let Some(window) = web_sys::window() {
            if let Err(_) = window.set_onload(Some(callback.as_ref().unchecked_ref())) {
                // Failed to set onload handler
            }
        }
        
        callback.forget();
        
        // Watch for user idle (visibility change)
        let self_clone = self.clone();
        let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Some(window) = web_sys::window() {
                let document = window.document().unwrap();
                if document.visibility_state() == VisibilityState::Hidden {
                    // Page is hidden, pause prefetching
                    *self_clone.is_processing.borrow_mut() = false;
                } else {
                    // Page is visible again, resume prefetching
                    let clone = self_clone.clone();
                    spawn_local(async move {
                        clone.process_queue().await;
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Err(_) = document.add_event_listener_with_callback(
                "visibilitychange",
                callback.as_ref().unchecked_ref(),
            ) {
                // Failed to add event listener
            }
        }
        
        callback.forget();
    }
    
    // Register callback for when network is idle
    pub fn on_network_idle<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        *self.network_idle_callback.borrow_mut() = Some(Box::new(callback));
    }
    
    // Get estimated bandwidth
    pub fn get_bandwidth_estimate(&self) -> Option<f64> {
        *self.bandwidth_estimate_kbps.borrow()
    }
}

// Prefetch manager for forum navigation
pub struct ForumPrefetcher {
    prefetcher: Prefetcher,
    recent_categories: Rc<RefCell<HashSet<i64>>>,
    recent_topics: Rc<RefCell<HashSet<i64>>>,
    // Simple navigation tracking (constant space)
    navigation_sequence: Rc<RefCell<VecDeque<PageVisit>>>,
    transition_counts: Rc<RefCell<HashMap<String, HashMap<String, u32>>>>,
    last_prefetch_time: Rc<RefCell<HashMap<String, Instant>>>,
}

// Simple page visit tracking
#[derive(Debug, Clone)]
struct PageVisit {
    page_key: String,
    timestamp: Instant,
}

impl ForumPrefetcher {
    pub fn new() -> Self {
        let mut nav_sequence = VecDeque::with_capacity(10);  // Only track last 10 visits
        
        Self {
            prefetcher: Prefetcher::new(),
            recent_categories: Rc::new(RefCell::new(HashSet::new())),
            recent_topics: Rc::new(RefCell::new(HashSet::new())),
            navigation_sequence: Rc::new(RefCell::new(nav_sequence)),
            transition_counts: Rc::new(RefCell::new(HashMap::new())),
            last_prefetch_time: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    
    // Enhanced track_category_view
    pub fn track_category_view(&self, category_id: i64) {
        let page_key = format!("category:{}", category_id);
        self.record_page_visit(&page_key);
        
        self.recent_categories.borrow_mut().insert(category_id);
        
        // Standard prefetching
        self.prefetch_category_data(category_id, PrefetchPriority::High);
        self.prefetch_topics_for_category(category_id, PrefetchPriority::Medium);
        
        // Use pattern-based prefetching (lightweight)
        self.prefetch_based_on_patterns(&page_key);
    }
    
    // Enhanced track_topic_view
    pub fn track_topic_view(&self, topic_id: i64, category_id: i64) {
        let page_key = format!("topic:{}", topic_id);
        self.record_page_visit(&page_key);
        
        self.recent_topics.borrow_mut().insert(topic_id);
        
        // Prefetch topic data
        self.prefetch_topic_data(topic_id, PrefetchPriority::High);
        
        // Remember category for this topic
        self.recent_categories.borrow_mut().insert(category_id);
        
        // Prefetch related topics
        self.prefetch_related_topics(topic_id, category_id, PrefetchPriority::Medium);
        
        // Use pattern-based prefetching (lightweight)
        self.prefetch_based_on_patterns(&page_key);
    }
    
    // Record page visit and update transition statistics
    fn record_page_visit(&self, page_key: &str) {
        let mut sequence = self.navigation_sequence.borrow_mut();
        let current_time = Instant::now();
        
        // Update transition counts if we have a previous page
        if let Some(previous) = sequence.back() {
            let previous_key = &previous.page_key;
            
            if previous_key != page_key {  // Don't count refreshes
                let mut counts = self.transition_counts.borrow_mut();
                let transitions = counts.entry(previous_key.clone()).or_insert_with(HashMap::new);
                *transitions.entry(page_key.to_string()).or_insert(0) += 1;
            }
        }
        
        // Add new visit to sequence
        sequence.push_back(PageVisit {
            page_key: page_key.to_string(),
            timestamp: current_time,
        });
        
        // Limit sequence size
        if sequence.len() > 10 {
            sequence.pop_front();
        }
    }
    
    // Prefetch based on observed navigation patterns - very lightweight
    fn prefetch_based_on_patterns(&self, current_page: &str) {
        // Simple time throttling to avoid excessive prefetching
        let now = Instant::now();
        let prefetch_timeout = Duration::from_secs(30);  // Only prefetch the same resource every 30s
        
        // Get transition counts for this page
        let counts = self.transition_counts.borrow();
        if let Some(transitions) = counts.get(current_page) {
            // Find most common transitions
            let mut transitions: Vec<_> = transitions.iter().collect();
            transitions.sort_by(|a, b| b.1.cmp(a.1));
            
            // Prefetch top 2 most likely next pages
            for (next_page, count) in transitions.iter().take(2) {
                // Only prefetch if we've seen this transition multiple times
                if *count >= 2 {
                    let url = self.get_url_for_page(next_page);
                    
                    // Check if we've recently prefetched this
                    let mut last_times = self.last_prefetch_time.borrow_mut();
                    let last_prefetch = last_times.get(&url).cloned().unwrap_or(Instant::now() - prefetch_timeout);
                    
                    if now.duration_since(last_prefetch) > prefetch_timeout {
                        // Not prefetched recently, do it now
                        self.prefetcher.queue(&url, PrefetchPriority::Low, None);
                        last_times.insert(url, now);
                    }
                }
            }
        }
    }
    
    // Convert page key to URL for prefetching
    fn get_url_for_page(&self, page_key: &str) -> String {
        if let Some(prefix_and_id) = page_key.split_once(':') {
            match prefix_and_id.0 {
                "category" => format!("/api/forum/categories/{}/topics?page=1&per_page=20", prefix_and_id.1),
                "topic" => format!("/api/forum/topics/{}", prefix_and_id.1),
                "user" => format!("/api/forum/users/{}", prefix_and_id.1),
                _ => format!("/api/forum/{}", page_key),
            }
        } else {
            format!("/api/forum/{}", page_key)
        }
    }
    
    // Get backend URL for topic data
    fn prefetch_topic_data(&self, topic_id: i64, priority: PrefetchPriority) {
        let url = format!("/api/forum/topics/{}", topic_id);
        self.prefetcher.queue(&url, priority, None);
        
        // Also prefetch posts
        let posts_url = format!("/api/forum/topics/{}/posts?page=1&per_page=20", topic_id);
        let deps = vec![format!("/api/forum/topics/{}", topic_id)];
        self.prefetcher.queue(&posts_url, priority, Some(deps));
    }
    
    // Prefetch category data
    fn prefetch_category_data(&self, category_id: i64, priority: PrefetchPriority) {
        let url = format!("/api/forum/categories/{}", category_id);
        self.prefetcher.queue(&url, priority, None);
    }
    
    // Prefetch topics for a category
    fn prefetch_topics_for_category(&self, category_id: i64, priority: PrefetchPriority) {
        let url = format!("/api/forum/categories/{}/topics?page=1&per_page=20", category_id);
        
        // Dependency on category data
        let deps = vec![format!("/api/forum/categories/{}", category_id)];
        
        self.prefetcher.queue(&url, priority, Some(deps));
    }
    
    // Prefetch related topics
    fn prefetch_related_topics(&self, topic_id: i64, category_id: i64, priority: PrefetchPriority) {
        // Prefetch other topics in same category
        let url = format!("/api/forum/categories/{}/topics?page=1&per_page=10", category_id);
        self.prefetcher.queue(&url, priority, None);
        
        // Prefetch topic recommendations
        let url = format!("/api/forum/topics/{}/related", topic_id);
        self.prefetcher.queue(&url, PrefetchPriority::Low, None);
    }
    
    // Prefetch common components when user is on the forum page
    pub fn prefetch_forum_components(&self) {
        // Prefetch UI components
        self.prefetcher.queue("/assets/js/forum/editor.js", PrefetchPriority::Medium, None);
        self.prefetcher.queue("/assets/js/forum/syntax-highlight.js", PrefetchPriority::Low, None);
        self.prefetcher.queue("/assets/css/forum/theme.css", PrefetchPriority::Medium, None);
        
        // Prefetch forum structure
        self.prefetcher.queue("/api/forum/categories", PrefetchPriority::High, None);
        self.prefetcher.queue("/api/forum/latest", PrefetchPriority::Medium, None);
    }
    
    // Get prefetcher instance
    pub fn get_prefetcher(&self) -> Prefetcher {
        self.prefetcher.clone()
    }
}

// Leptos hook for using prefetching in components
#[hook]
pub fn use_prefetch() -> ForumPrefetcher {
    // Use the same prefetcher instance across the app
    static PREFETCHER: once_cell::sync::Lazy<ForumPrefetcher> = 
        once_cell::sync::Lazy::new(|| ForumPrefetcher::new());
    
    PREFETCHER.clone()
}