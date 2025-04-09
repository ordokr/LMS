use moka::future::Cache;
use std::time::Duration;
use crate::repositories::forum::{Category, Topic};

/// Cache for forum data to reduce database load
pub struct ForumCache {
    categories: Cache<i64, Category>,
    recent_topics: Cache<i64, Topic>,
    topics_by_category: Cache<i64, Vec<i64>>, // Category ID -> Topic IDs
}

impl ForumCache {
    pub fn new() -> Self {
        Self {
            categories: Cache::builder()
                .max_capacity(100) // Forum categories are typically limited
                .time_to_live(Duration::from_secs(300)) // 5 minutes
                .build(),
                
            recent_topics: Cache::builder()
                .max_capacity(1000) // More topics in cache
                .time_to_live(Duration::from_secs(60)) // 1 minute
                .build(),
                
            topics_by_category: Cache::builder()
                .max_capacity(100) // Same as categories
                .time_to_live(Duration::from_secs(120)) // 2 minutes
                .build(),
        }
    }
    
    // Category methods
    pub async fn get_category(&self, id: i64) -> Option<Category> {
        self.categories.get(&id).await
    }
    
    pub async fn insert_category(&self, category: Category) {
        self.categories.insert(category.id, category).await;
    }
    
    pub async fn invalidate_category(&self, id: i64) {
        self.categories.invalidate(&id).await;
        // Also invalidate related topic lists
        self.topics_by_category.invalidate(&id).await;
    }
    
    // Topic methods
    pub async fn get_topic(&self, id: i64) -> Option<Topic> {
        self.recent_topics.get(&id).await
    }
    
    pub async fn insert_topic(&self, topic: Topic) {
        let category_id = topic.category_id;
        self.recent_topics.insert(topic.id, topic).await;
        
        // Update topic list for category if it exists in cache
        if let Some(mut topic_ids) = self.topics_by_category.get(&category_id).await {
            if !topic_ids.contains(&topic.id) {
                topic_ids.push(topic.id);
                self.topics_by_category.insert(category_id, topic_ids).await;
            }
        }
    }
    
    pub async fn invalidate_topic(&self, id: i64, category_id: Option<i64>) {
        self.recent_topics.invalidate(&id).await;
        
        // If we know the category, invalidate its topic list too
        if let Some(cat_id) = category_id {
            self.topics_by_category.invalidate(&cat_id).await;
        }
    }
    
    // Topics by category methods
    pub async fn get_topic_ids_for_category(&self, category_id: i64) -> Option<Vec<i64>> {
        self.topics_by_category.get(&category_id).await
    }
    
    pub async fn set_topic_ids_for_category(&self, category_id: i64, topic_ids: Vec<i64>) {
        self.topics_by_category.insert(category_id, topic_ids).await;
    }
    
    // Cache utilities
    pub async fn clear_all(&self) {
        self.categories.invalidate_all().await;
        self.recent_topics.invalidate_all().await;
        self.topics_by_category.invalidate_all().await;
    }
}