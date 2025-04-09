use leptos::*;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

// Topic update granularity for fine-grained reactivity
#[derive(Clone, Debug)]
pub enum TopicUpdate {
    NewPost { topic_id: i64, post_id: i64 },
    TitleChanged { topic_id: i64, new_title: String },
    Pinned { topic_id: i64 },
    Unpinned { topic_id: i64 },
    CategoryChanged { topic_id: i64, new_category_id: i64 },
}

// Forum state with granular updates
pub struct ForumState {
    categories: RwSignal<Vec<Category>>,
    topics: StoredValue<HashMap<i64, RwSignal<Topic>>>,
    posts: StoredValue<HashMap<i64, RwSignal<Post>>>,
    current_category_id: RwSignal<Option<i64>>,
    current_topic_id: RwSignal<Option<i64>>,
    pending_updates: StoredValue<VecDeque<TopicUpdate>>,
    last_batch_time: StoredValue<Instant>,
    update_scheduled: StoredValue<bool>,
}

impl ForumState {
    pub fn new() -> Self {
        let instance = Self {
            categories: create_rw_signal(Vec::new()),
            topics: store_value(HashMap::new()),
            posts: store_value(HashMap::new()),
            current_category_id: create_rw_signal(None),
            current_topic_id: create_rw_signal(None),
            pending_updates: store_value(VecDeque::new()),
            last_batch_time: store_value(Instant::now()),
            update_scheduled: store_value(false),
        };

        // Schedule batch processing
        let instance_clone = instance.clone();
        set_timeout(move || instance_clone.process_batch_updates(), Duration::from_millis(100));
        
        instance
    }
    
    // Get a topic with memoized derived signals for performance
    pub fn get_topic(&self, topic_id: i64) -> Option<Memo<Topic>> {
        let topics = self.topics.get_value();
        topics.get(&topic_id).map(|topic_signal| {
            create_memo(move |_| topic_signal.get())
        })
    }
    
    // Apply targeted updates
    pub fn apply_update(&self, update: TopicUpdate) {
        match update {
            TopicUpdate::NewPost { topic_id, post_id } => {
                // Update only the specific topic's post count
                if let Some(topic_signal) = self.topics.get_value().get(&topic_id) {
                    topic_signal.update(|topic| {
                        topic.post_count += 1;
                        topic.last_post_id = Some(post_id);
                        topic.updated_at = chrono::Utc::now();
                    });
                }
            },
            TopicUpdate::TitleChanged { topic_id, new_title } => {
                if let Some(topic_signal) = self.topics.get_value().get(&topic_id) {
                    topic_signal.update(|topic| {
                        topic.title = new_title;
                    });
                }
            },
            TopicUpdate::Pinned { topic_id } => {
                if let Some(topic_signal) = self.topics.get_value().get(&topic_id) {
                    topic_signal.update(|topic| {
                        topic.pinned = true;
                    });
                }
            },
            TopicUpdate::Unpinned { topic_id } => {
                if let Some(topic_signal) = self.topics.get_value().get(&topic_id) {
                    topic_signal.update(|topic| {
                        topic.pinned = false;
                    });
                }
            },
            TopicUpdate::CategoryChanged { topic_id, new_category_id } => {
                if let Some(topic_signal) = self.topics.get_value().get(&topic_id) {
                    topic_signal.update(|topic| {
                        topic.category_id = new_category_id;
                    });
                }
            }
        }
    }
    
    // Queue an update for batch processing
    pub fn queue_update(&self, update: TopicUpdate) {
        let mut updates = self.pending_updates.get_value();
        updates.push_back(update);
        self.pending_updates.set_value(updates);
        
        // Schedule processing if not already scheduled
        if !*self.update_scheduled.get_value() {
            self.schedule_batch_processing();
        }
    }
    
    // Schedule batch processing
    fn schedule_batch_processing(&self) {
        *self.update_scheduled.set_value() = true;
        
        let self_clone = self.clone();
        set_timeout(move || self_clone.process_batch_updates(), Duration::from_millis(50));
    }
    
    // Process all queued updates in a batch
    fn process_batch_updates(&self) {
        let now = Instant::now();
        let mut updates = self.pending_updates.get_value();
        
        // Track which topics were updated to avoid redundant UI updates
        let mut updated_topics = HashSet::new();
        
        // Process all pending updates
        while let Some(update) = updates.pop_front() {
            match &update {
                TopicUpdate::NewPost { topic_id, .. } => updated_topics.insert(*topic_id),
                TopicUpdate::TitleChanged { topic_id, .. } => updated_topics.insert(*topic_id),
                TopicUpdate::Pinned { topic_id } => updated_topics.insert(*topic_id),
                TopicUpdate::Unpinned { topic_id } => updated_topics.insert(*topic_id),
                TopicUpdate::CategoryChanged { topic_id, .. } => updated_topics.insert(*topic_id),
            };
            
            // Apply the individual update
            self.apply_update(update);
        }
        
        // Update categories that contain updated topics
        if !updated_topics.is_empty() {
            self.refresh_categories_containing_topics(&updated_topics);
        }
        
        // Save updated queue
        self.pending_updates.set_value(updates);
        self.last_batch_time.set_value(now);
        *self.update_scheduled.get_value() = false;
        
        // If new updates came in during processing, schedule another batch
        if !self.pending_updates.get_value().is_empty() {
            self.schedule_batch_processing();
        }
    }
    
    // Refresh categories that contain the given topics
    fn refresh_categories_containing_topics(&self, topic_ids: &HashSet<i64>) {
        let topics = self.topics.get_value();
        let mut affected_categories = HashSet::new();
        
        // Find all affected categories
        for topic_id in topic_ids {
            if let Some(topic_signal) = topics.get(topic_id) {
                let topic = topic_signal.get();
                affected_categories.insert(topic.category_id);
            }
        }
        
        // Refresh category signals
        if !affected_categories.is_empty() {
            self.refresh_categories(&affected_categories);
        }
    }
    
    // Refresh category signals
    fn refresh_categories(&self, category_ids: &HashSet<i64>) {
        // Update the categories signal to trigger reactive updates
        // This is more efficient than updating the entire categories list
        self.categories.update(|categories| {
            for category in categories.iter_mut() {
                if category_ids.contains(&category.id) {
                    // Mark category as refreshed (causes UI to update)
                    category.last_updated = chrono::Utc::now();
                }
            }
        });
    }
}