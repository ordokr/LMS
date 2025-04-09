use async_trait::async_trait;
use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::forum::mapping::{TopicMapping, PostMapping};
use crate::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait IntegrationEventListener: Send + Sync {
    /// Called when a topic is successfully synced from Canvas to Discourse
    async fn on_topic_synced_to_discourse(&self, topic: &Topic, mapping: &TopicMapping) -> Result<(), Error>;
    
    /// Called when a topic is successfully synced from Discourse to Canvas
    async fn on_topic_synced_to_canvas(&self, topic: &Topic, mapping: &TopicMapping) -> Result<(), Error>;
    
    /// Called when a post is successfully synced from Canvas to Discourse
    async fn on_post_synced_to_discourse(&self, post: &Post, mapping: &PostMapping) -> Result<(), Error>;
    
    /// Called when a post is successfully synced from Discourse to Canvas
    async fn on_post_synced_to_canvas(&self, post: &Post, mapping: &PostMapping) -> Result<(), Error>;
    
    /// Called when a sync operation fails
    async fn on_sync_failure(&self, error: &Error, entity_type: &str, entity_id: &str) -> Result<(), Error>;
}

/// Integration event dispatcher that notifies all registered listeners
pub struct IntegrationEventDispatcher {
    listeners: Vec<Arc<dyn IntegrationEventListener>>,
}

impl IntegrationEventDispatcher {
    pub fn new() -> Self {
        IntegrationEventDispatcher {
            listeners: Vec::new(),
        }
    }
    
    pub fn register_listener(&mut self, listener: Arc<dyn IntegrationEventListener>) {
        self.listeners.push(listener);
    }
    
    pub async fn topic_synced_to_discourse(&self, topic: &Topic, mapping: &TopicMapping) -> Result<(), Error> {
        for listener in &self.listeners {
            if let Err(e) = listener.on_topic_synced_to_discourse(topic, mapping).await {
                eprintln!("Error in event listener: {}", e);
            }
        }
        Ok(())
    }
    
    pub async fn topic_synced_to_canvas(&self, topic: &Topic, mapping: &TopicMapping) -> Result<(), Error> {
        for listener in &self.listeners {
            if let Err(e) = listener.on_topic_synced_to_canvas(topic, mapping).await {
                eprintln!("Error in event listener: {}", e);
            }
        }
        Ok(())
    }
    
    pub async fn post_synced_to_discourse(&self, post: &Post, mapping: &PostMapping) -> Result<(), Error> {
        for listener in &self.listeners {
            if let Err(e) = listener.on_post_synced_to_discourse(post, mapping).await {
                eprintln!("Error in event listener: {}", e);
            }
        }
        Ok(())
    }
    
    pub async fn post_synced_to_canvas(&self, post: &Post, mapping: &PostMapping) -> Result<(), Error> {
        for listener in &self.listeners {
            if let Err(e) = listener.on_post_synced_to_canvas(post, mapping).await {
                eprintln!("Error in event listener: {}", e);
            }
        }
        Ok(())
    }
    
    pub async fn sync_failure(&self, error: &Error, entity_type: &str, entity_id: &str) -> Result<(), Error> {
        for listener in &self.listeners {
            if let Err(e) = listener.on_sync_failure(error, entity_type, entity_id).await {
                eprintln!("Error in event listener: {}", e);
            }
        }
        Ok(())
    }
}

/// A logging integration event listener
pub struct LoggingEventListener;

#[async_trait]
impl IntegrationEventListener for LoggingEventListener {
    async fn on_topic_synced_to_discourse(&self, topic: &Topic, mapping: &TopicMapping) -> Result<(), Error> {
        println!("Topic '{}' synced to Discourse: Topic ID {}", topic.title, mapping.discourse_topic_id);
        Ok(())
    }
    
    async fn on_topic_synced_to_canvas(&self, topic: &Topic, mapping: &TopicMapping) -> Result<(), Error> {
        println!("Topic '{}' synced to Canvas: Topic ID {}", topic.title, mapping.canvas_topic_id);
        Ok(())
    }
    
    async fn on_post_synced_to_discourse(&self, post: &Post, mapping: &PostMapping) -> Result<(), Error> {
        println!("Post synced to Discourse: Post ID {}", mapping.discourse_post_id);
        Ok(())
    }
    
    async fn on_post_synced_to_canvas(&self, post: &Post, mapping: &PostMapping) -> Result<(), Error> {
        println!("Post synced to Canvas: Entry ID {}", mapping.canvas_entry_id);
        Ok(())
    }
    
    async fn on_sync_failure(&self, error: &Error, entity_type: &str, entity_id: &str) -> Result<(), Error> {
        println!("Sync failure for {} {}: {}", entity_type, entity_id, error);
        Ok(())
    }
}