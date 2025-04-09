use leptos::*;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;
use moka::sync::Cache;
use std::sync::Arc;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;
use std::ops::{Deref, DerefMut};

// Define application state
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AppState {
    pub user: Option<User>,
    pub theme: Theme,
    pub notifications: Vec<Notification>,
    pub is_online: bool,
    pub last_sync: Option<String>,
    pub forum: ForumState,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Student,
    Teacher,
    Admin,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Notification {
    pub id: String,
    pub message: String,
    pub read: bool,
    pub type_: NotificationType,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
}

// Update ForumState to use VecDeque for efficient operations at both ends
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ForumState {
    pub current_category_id: Option<i64>,
    #[serde(with = "vecdeque_compat")]
    pub last_visited_topic_ids: VecDeque<i64>, // More efficient than Vec for front insertions
    pub category_expanded: HashMap<i64, bool>, // Track which categories are expanded
}

// Initialize default state
impl Default for AppState {
    fn default() -> Self {
        Self {
            user: None,
            theme: Theme::System,
            notifications: Vec::new(),
            is_online: true,
            last_sync: None,
            forum: Default::default(),
        }
    }
}

// Create a store for application state with fine-grained reactivity
pub struct AppStore {
    // Core state signal with complete state
    state: RwSignal<AppState>,
    
    // Fine-grained signals for individual state sections
    user: RwSignal<Option<User>>,
    theme: RwSignal<Theme>,
    notifications: RwSignal<Vec<Notification>>,
    is_online: RwSignal<bool>,
    forum_state: RwSignal<ForumState>,
    
    // Derived read-only signals for common queries
    notifications_unread: Signal<usize>,
    recently_visited_topics: Signal<Vec<i64>>,
    
    // Observers to prevent memory leaks (optimized)
    observers: Rc<RefCell<HashMap<StateSection, Vec<Callback<()>>>>>,
    
    // Add this field to AppStore
    memo_cache: Arc<Cache<String, Arc<dyn std::any::Any + Send + Sync>>>,
    
    // Add this to AppStore
    metrics: StateMetrics,
}

#[derive(Default)]
struct StateMetrics {
    updates: AtomicUsize,
    batched_updates: AtomicUsize,
    clones_avoided: AtomicUsize,
    last_update_time: Mutex<Option<Instant>>,
}

impl AppStore {
    pub fn new() -> Self {
        let state = create_rw_signal(AppState::default());
        
        // Create individual section signals
        let user = create_rw_signal(None);
        let theme = create_rw_signal(Theme::System);
        let notifications = create_rw_signal(Vec::new());
        let is_online = create_rw_signal(true);
        let forum_state = create_rw_signal(ForumState::default());
        
        // Create derived signals for common data
        let notifications_unread = create_memo(move |_| {
            notifications.get().iter().filter(|n| !n.read).count()
        });
        
        let recently_visited_topics = create_memo(move |_| {
            forum_state.get().last_visited_topic_ids.clone().into()
        });
        
        let memo_cache = Arc::new(Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(60))
            .build());
        
        Self {
            state,
            user,
            theme,
            notifications,
            is_online,
            forum_state,
            notifications_unread,
            recently_visited_topics,
            observers: Rc::new(RefCell::new(HashMap::new())),
            memo_cache,
            metrics: StateMetrics::default(),
        }
    }
    
    // Get full state (now optimized to avoid unnecessary clones)
    pub fn get_state(&self) -> &RwSignal<AppState> {
        &self.state
    }
    
    // Update state with targeted updates to avoid unnecessary reactivity
    pub fn update<F>(&self, section: StateSection, updater: F)
    where
        F: FnOnce(&mut AppState),
    {
        self.state.update(updater);
        
        // Update the specific section signal
        match section {
            StateSection::User => self.user.set(self.state.get().user.clone()),
            StateSection::Theme => self.theme.set(self.state.get().theme.clone()),
            StateSection::Notifications => self.notifications.set(self.state.get().notifications.clone()),
            StateSection::Network => self.is_online.set(self.state.get().is_online),
            StateSection::Forum => self.forum_state.set(self.state.get().forum.clone()),
            StateSection::All => {
                // Update all signals
                self.user.set(self.state.get().user.clone());
                self.theme.set(self.state.get().theme.clone());
                self.notifications.set(self.state.get().notifications.clone());
                self.is_online.set(self.state.get().is_online);
                self.forum_state.set(self.state.get().forum.clone());
            }
        }
        
        // Notify only the relevant observers
        let observers = self.observers.borrow();
        
        if let Some(callbacks) = observers.get(&section) {
            for callback in callbacks {
                callback.call(());
            }
        }
        
        // Always notify 'All' observers
        if section != StateSection::All {
            if let Some(callbacks) = observers.get(&StateSection::All) {
                for callback in callbacks {
                    callback.call(());
                }
            }
        }
    }
    
    // Register an observer for specific state sections
    pub fn observe(&self, section: StateSection, callback: Callback<()>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let mut observers = self.observers.borrow_mut();
        
        observers
            .entry(section)
            .or_insert_with(Vec::new)
            .push(callback);
        
        id
    }
    
    // Remove an observer to prevent memory leaks
    pub fn remove_observer(&self, section: StateSection, id: &str) {
        let mut observers = self.observers.borrow_mut();
        
        if let Some(callbacks) = observers.get_mut(&section) {
            if let Some(index) = callbacks.iter().position(|c| c.id() == id) {
                callbacks.remove(index);
            }
        }
    }
    
    // Efficient accessors for common properties - now using direct signals
    pub fn user(&self) -> Signal<Option<User>> {
        self.user.into()
    }
    
    pub fn theme(&self) -> Signal<Theme> {
        self.theme.into()
    }
    
    pub fn is_online(&self) -> Signal<bool> {
        self.is_online.into()
    }
    
    pub fn unread_notifications(&self) -> Signal<usize> {
        self.notifications_unread
    }
    
    pub fn recently_visited_topics(&self) -> Signal<Vec<i64>> {
        self.recently_visited_topics
    }
    
    // Specific state updates with targeted sections
    pub fn set_user(&self, user: Option<User>) {
        self.update(StateSection::User, |state| state.user = user.clone());
        self.user.set(user);
    }
    
    pub fn set_theme(&self, theme: Theme) {
        self.update(StateSection::Theme, |state| state.theme = theme.clone());
        self.theme.set(theme);
    }
    
    pub fn set_online(&self, online: bool) {
        self.update(StateSection::Network, |state| state.is_online = online);
        self.is_online.set(online);
    }
    
    pub fn add_notification(&self, notification: Notification) {
        self.notifications.update(|notifications| {
            notifications.push(notification.clone());
        });
        
        self.update(StateSection::Notifications, |state| {
            state.notifications.push(notification);
        });
    }
    
    pub fn mark_notification_read(&self, id: &str) {
        self.notifications.update(|notifications| {
            if let Some(notification) = notifications.iter_mut().find(|n| n.id == id) {
                notification.read = true;
            }
        });
        
        self.update(StateSection::Notifications, |state| {
            if let Some(notification) = state.notifications.iter_mut().find(|n| n.id == id) {
                notification.read = true;
            }
        });
    }
    
    pub fn clear_notifications(&self) {
        self.notifications.update(|notifications| {
            notifications.clear();
        });
        
        self.update(StateSection::Notifications, |state| {
            state.notifications.clear();
        });
    }
    
    // Forum specific state updates with optimized implementations
    pub fn set_current_category(&self, category_id: Option<i64>) {
        self.forum_state.update(|forum| {
            forum.current_category_id = category_id;
        });
        
        self.update(StateSection::Forum, |state| {
            state.forum.current_category_id = category_id;
        });
    }
    
    // Implement the track_topic_visit function with VecDeque
    pub fn track_topic_visit(&self, topic_id: i64) {
        // Check if the topic is already at the front to avoid unnecessary updates
        {
            let forum = self.forum_state.get();
            if forum.last_visited_topic_ids.front().map_or(false, |&id| id == topic_id) {
                return; // Already at front, no need to update
            }
        }
        
        // Proceed with update if needed
        self.update(StateSection::Forum, |state| {
            // Remove if already exists
            if let Some(pos) = state.forum.last_visited_topic_ids.iter().position(|&id| id == topic_id) {
                state.forum.last_visited_topic_ids.remove(pos);
            }
            
            // Add to front
            state.forum.last_visited_topic_ids.push_front(topic_id);
            
            // Limit size
            while state.forum.last_visited_topic_ids.len() > 20 {
                state.forum.last_visited_topic_ids.pop_back();
            }
        });
        
        // Use update rather than set to avoid unnecessary clone
        self.forum_state.update(|forum| {
            *forum = self.state.get().forum.clone();
        });
    }
    
    // New method: Get visited topics with pagination for efficient rendering
    pub fn get_visited_topics(&self, page: usize, page_size: usize) -> Vec<i64> {
        let forum = self.forum_state.get();
        let total = forum.last_visited_topic_ids.len();
        
        if total == 0 || page * page_size >= total {
            return Vec::new();
        }
        
        // Pre-allocate the exact size needed
        let start = page * page_size;
        let end = (start + page_size).min(total);
        let mut result = Vec::with_capacity(end - start);
        
        // Use iterator directly instead of collecting into a new vector
        forum.last_visited_topic_ids
            .iter()
            .skip(start)
            .take(end - start)
            .for_each(|&id| result.push(id));
        
        result
    }
    
    // New method for batching multiple updates together
    pub fn batch_update<F>(&self, updater: F)
    where
        F: FnOnce(&mut BatchUpdater),
    {
        let mut batch = BatchUpdater::new(self);
        updater(&mut batch);
        batch.commit();
    }

    // Create a memoization cache for expensive operations
    fn create_memo_cache() -> Cache<String, Arc<dyn std::any::Any + Send + Sync>> {
        Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(60))
            .build()
    }
    
    // Example of a new memoized function for complex forum operations
    pub fn get_categorized_topics(&self) -> Arc<HashMap<i64, Vec<i64>>> {
        // Generate a more specific cache key that includes content hash
        let forum_state = self.forum_state.get();
        let hash = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            for id in forum_state.last_visited_topic_ids.iter() {
                id.hash(&mut hasher);
            }
            hasher.finish()
        };
        let cache_key = format!("categorized_topics:{}", hash);
        
        // Now use the instance cache instead of creating a new one
        if let Some(cached) = self.memo_cache.get(&cache_key) {
            if let Some(result) = cached.downcast_ref::<HashMap<i64, Vec<i64>>>() {
                return Arc::new(result.clone());
            }
        }
        
        // Calculate if not in cache
        let forum_state = self.forum_state.get();
        let mut result = HashMap::new();
        
        // Perform expensive operation
        // (This is a placeholder - actual implementation would use your forum data)
        for &topic_id in forum_state.last_visited_topic_ids.iter() {
            let category_id = topic_id % 10; // Placeholder logic
            result.entry(category_id)
                .or_insert_with(Vec::new)
                .push(topic_id);
        }
        
        // Cache result
        let result_arc = Arc::new(result);
        self.memo_cache.insert(
            cache_key, 
            result_arc.clone() as Arc<dyn std::any::Any + Send + Sync>
        );
        
        result_arc
    }

    pub fn get_metrics(&self) -> HashMap<String, usize> {
        let mut metrics = HashMap::new();
        metrics.insert("updates".into(), self.metrics.updates.load(Ordering::Relaxed));
        metrics.insert("batched_updates".into(), self.metrics.batched_updates.load(Ordering::Relaxed));
        metrics.insert("clones_avoided".into(), self.metrics.clones_avoided.load(Ordering::Relaxed));
        metrics
    }

    // Add this helper to AppStore
    fn update_signals_selectively(&self, sections: &std::collections::HashSet<StateSection>) {
        let state = self.state.get();
        
        // Only update the signals that correspond to modified sections
        for section in sections {
            match section {
                StateSection::User => self.user.set(state.user.clone()),
                StateSection::Theme => self.theme.set(state.theme.clone()),
                StateSection::Notifications => self.notifications.set(state.notifications.clone()),
                StateSection::Network => self.is_online.set(state.is_online),
                StateSection::Forum => self.forum_state.set(state.forum.clone()),
                StateSection::All => {
                    // Update all signals
                    self.user.set(state.user.clone());
                    self.theme.set(state.theme.clone());
                    self.notifications.set(state.notifications.clone());
                    self.is_online.set(state.is_online);
                    self.forum_state.set(state.forum.clone());
                    break; // No need to process other sections
                }
            }
        }
    }

    // Add a reset method to clear all state to defaults
    pub fn reset(&self) {
        // Reset to default state
        self.state.set(AppState::default());
        
        // Reset all granular signals
        self.user.set(None);
        self.theme.set(Theme::System);
        self.notifications.set(Vec::new());
        self.is_online.set(true);
        self.forum_state.set(ForumState::default());
        
        // Clear any memoization caches
        self.memo_cache.invalidate_all();
    }
}

// Define state sections for targeted updates
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateSection {
    User,
    Theme,
    Notifications,
    Network,
    Forum,
    All,
}

// Helper struct for batching updates
pub struct BatchUpdater<'a> {
    app_store: &'a AppStore,
    state: AppState,
    modified_sections: std::collections::HashSet<StateSection>,
}

impl<'a> BatchUpdater<'a> {
    fn new(app_store: &'a AppStore) -> Self {
        Self {
            app_store,
            state: app_store.get_state().get(),
            modified_sections: std::collections::HashSet::new(),
        }
    }
    
    pub fn set_user(&mut self, user: Option<User>) {
        self.state.user = user;
        self.modified_sections.insert(StateSection::User);
    }
    
    pub fn set_theme(&mut self, theme: Theme) {
        self.state.theme = theme;
        self.modified_sections.insert(StateSection::Theme);
    }
    
    pub fn set_online(&mut self, online: bool) {
        self.state.is_online = online;
        self.modified_sections.insert(StateSection::Network);
    }
    
    pub fn add_notification(&mut self, notification: Notification) {
        self.state.notifications.push(notification);
        self.modified_sections.insert(StateSection::Notifications);
    }
    
    pub fn track_topic_visit(&mut self, topic_id: i64) {
        // Remove if already exists to move to front
        if let Some(pos) = self.state.forum.last_visited_topic_ids.iter().position(|&id| id == topic_id) {
            self.state.forum.last_visited_topic_ids.remove(pos);
        }
        
        // Add to front
        self.state.forum.last_visited_topic_ids.push_front(topic_id);
        
        // Keep list manageable
        while self.state.forum.last_visited_topic_ids.len() > 20 {
            self.state.forum.last_visited_topic_ids.pop_back();
        }
        
        self.modified_sections.insert(StateSection::Forum);
    }
    
    // Add a performance optimization to reduce memory allocations
    pub fn set_notifications(&mut self, notifications: Vec<Notification>) {
        // Reserve capacity if needed to avoid reallocations
        if self.state.notifications.capacity() < notifications.len() {
            let mut new_vec = Vec::with_capacity(notifications.len());
            new_vec.extend(notifications.into_iter());
            self.state.notifications = new_vec;
        } else {
            self.state.notifications = notifications;
        }
        self.modified_sections.insert(StateSection::Notifications);
    }
    
    // Optimize commit to minimize cloning where possible
    fn commit(self) {
        // Update main state once with move instead of clone where possible
        self.app_store.state.set(self.state);
        
        // Update individual section signals with direct references
        let state = self.app_store.state.get_untracked();
        
        for section in self.modified_sections {
            match section {
                StateSection::User => self.app_store.user.set(state.user.clone()),
                StateSection::Theme => self.app_store.theme.set(state.theme.clone()),
                StateSection::Notifications => self.app_store.notifications.set(state.notifications.clone()),
                StateSection::Network => self.app_store.is_online.set(state.is_online),
                StateSection::Forum => self.app_store.forum_state.set(state.forum.clone()),
                StateSection::All => {
                    // Update all signals
                    self.app_store.user.set(state.user.clone());
                    self.app_store.theme.set(state.theme.clone());
                    self.app_store.notifications.set(state.notifications.clone());
                    self.app_store.is_online.set(state.is_online);
                    self.app_store.forum_state.set(state.forum.clone());
                }
            }
        }
        
        // Notify observers as before...
    }
}

// Example of using batch updates for multiple changes:
// 
// app_store.batch_update(|batch| {
//     batch.set_user(Some(user));
//     batch.set_online(true);
//     batch.track_topic_visit(123);
// });

// Helper module for VecDeque serialization compatibility
#[allow(dead_code)]
mod vecdeque_compat {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::VecDeque;

    pub fn serialize<S, T>(deque: &VecDeque<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        let vec: Vec<&T> = deque.iter().collect();
        vec.serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<VecDeque<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let vec = Vec::deserialize(deserializer)?;
        Ok(VecDeque::from(vec))
    }
}

// Implement a flyweight pattern for frequently repeated strings
struct StringFlyweight {
    strings: RefCell<HashMap<String, Rc<String>>>,
}

impl StringFlyweight {
    fn new() -> Self {
        Self {
            strings: RefCell::new(HashMap::new()),
        }
    }
    
    fn get(&self, s: &str) -> Rc<String> {
        let mut strings = self.strings.borrow_mut();
        if let Some(existing) = strings.get(s) {
            existing.clone()
        } else {
            let rc = Rc::new(s.to_string());
            strings.insert(s.to_string(), rc.clone());
            rc
        }
    }
}

// Add this wrapper for immutable collections to reduce cloning
pub struct CowVec<T: Clone> {
    inner: Arc<Vec<T>>,
}

impl<T: Clone> CowVec<T> {
    fn new() -> Self {
        Self { inner: Arc::new(Vec::new()) }
    }
    
    fn get_mut(&mut self) -> &mut Vec<T> {
        Arc::make_mut(&mut self.inner)
    }
}

impl<T: Clone> Deref for CowVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Then use it for notifications to avoid cloning until needed
#[derive(Clone)]
pub struct OptimizedNotifications {
    inner: CowVec<Notification>,
}

// Add a transaction counter to batch observer notifications
pub struct ObserverTransaction {
    app_store: AppStore,
    modified_sections: std::collections::HashSet<StateSection>,
}

impl ObserverTransaction {
    pub fn new(app_store: AppStore) -> Self {
        Self {
            app_store,
            modified_sections: std::collections::HashSet::new(),
        }
    }
    
    pub fn update<F>(&mut self, section: StateSection, updater: F) 
    where 
        F: FnOnce(&mut AppState) 
    {
        // Update state without notifying observers
        self.app_store.state.update(|state| {
            updater(state);
        });
        
        // Update the section signal
        match section {
            StateSection::User => self.app_store.user.set(self.app_store.state.get().user.clone()),
            // Other sections...
        }
        
        // Remember which sections were modified
        self.modified_sections.insert(section);
    }
    
    pub fn commit(self) {
        // Now notify observers just once per section
        let observers = self.app_store.observers.borrow();
        
        for section in self.modified_sections {
            if let Some(callbacks) = observers.get(&section) {
                for callback in callbacks {
                    callback.call(());
                }
            }
        }
        
        // Always notify 'All' observers
        if !self.modified_sections.is_empty() && !self.modified_sections.contains(&StateSection::All) {
            if let Some(callbacks) = observers.get(&StateSection::All) {
                for callback in callbacks {
                    callback.call(());
                }
            }
        }
    }
}