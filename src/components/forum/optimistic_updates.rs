use leptos::*;
use std::collections::HashMap;

// Optimistic update manager for forum actions
pub struct OptimisticUpdateManager {
    pending_posts: RwSignal<HashMap<String, PendingPost>>,
    pending_topics: RwSignal<HashMap<String, PendingTopic>>,
    pending_edits: RwSignal<HashMap<String, PendingEdit>>,
    error_handler: SignalSetter<Option<String>>,
}

#[derive(Clone, Debug)]
struct PendingPost {
    local_id: String,
    topic_id: i64,
    content: String,
    author: String,
    timestamp: f64,
    status: SubmissionStatus,
}

#[derive(Clone, Debug)]
struct PendingTopic {
    local_id: String,
    category_id: i64,
    title: String,
    content: String,
    author: String,
    timestamp: f64,
    status: SubmissionStatus,
}

#[derive(Clone, Debug)]
struct PendingEdit {
    entity_id: String, // Can be for post or topic
    field: String,     // What's being edited
    old_value: String, // For rollback
    new_value: String, // Optimistic update
    status: SubmissionStatus,
}

#[derive(Clone, Debug, PartialEq)]
enum SubmissionStatus {
    Pending,
    Sending,
    Success { server_id: i64 },
    Failed { reason: String },
}

impl OptimisticUpdateManager {
    pub fn new(error_handler: SignalSetter<Option<String>>) -> Self {
        Self {
            pending_posts: create_rw_signal(HashMap::new()),
            pending_topics: create_rw_signal(HashMap::new()),
            pending_edits: create_rw_signal(HashMap::new()),
            error_handler,
        }
    }
    
    // Create a new post optimistically
    pub fn create_post(&self, topic_id: i64, content: String, author: String) -> String {
        let local_id = format!("pending_post_{}", js_sys::Date::now());
        
        // Add to pending posts
        self.pending_posts.update(|posts| {
            posts.insert(local_id.clone(), PendingPost {
                local_id: local_id.clone(),
                topic_id,
                content,
                author,
                timestamp: js_sys::Date::now(),
                status: SubmissionStatus::Pending,
            });
        });
        
        local_id
    }
    
    // Submit the optimistic post to the server
    pub async fn submit_post(&self, local_id: &str) -> Result<i64, String> {
        let mut post_to_submit = None;
        
        // Update status to sending
        self.pending_posts.update(|posts| {
            if let Some(post) = posts.get_mut(local_id) {
                post.status = SubmissionStatus::Sending;
                post_to_submit = Some(post.clone());
            }
        });
        
        let Some(post) = post_to_submit else {
            return Err("Post not found".to_string());
        };
        
        // Call API to create post
        match create_post_api(&post.topic_id, &post.content).await {
            Ok(server_id) => {
                // Update with success
                self.pending_posts.update(|posts| {
                    if let Some(post) = posts.get_mut(local_id) {
                        post.status = SubmissionStatus::Success { server_id };
                    }
                });
                
                // Clean up successful posts after a delay
                let local_id = local_id.to_string();
                let this = self.clone();
                set_timeout(move || {
                    this.pending_posts.update(|posts| {
                        posts.remove(&local_id);
                    });
                }, std::time::Duration::from_secs(5));
                
                Ok(server_id)
            },
            Err(e) => {
                // Mark as failed
                self.pending_posts.update(|posts| {
                    if let Some(post) = posts.get_mut(local_id) {
                        post.status = SubmissionStatus::Failed { reason: e.clone() };
                    }
                });
                
                // Notify error handler
                self.error_handler.set(Some(format!("Failed to submit post: {}", e)));
                
                Err(e)
            }
        }
    }
    
    // Create a new topic optimistically
    pub fn create_topic(&self, category_id: i64, title: String, content: String, author: String) -> String {
        let local_id = format!("pending_topic_{}", js_sys::Date::now());
        
        // Add to pending topics
        self.pending_topics.update(|topics| {
            topics.insert(local_id.clone(), PendingTopic {
                local_id: local_id.clone(),
                category_id,
                title,
                content,
                author,
                timestamp: js_sys::Date::now(),
                status: SubmissionStatus::Pending,
            });
        });
        
        local_id
    }
    
    // Submit optimistic topic to server
    pub async fn submit_topic(&self, local_id: &str) -> Result<i64, String> {
        let mut topic_to_submit = None;
        
        // Update status to sending
        self.pending_topics.update(|topics| {
            if let Some(topic) = topics.get_mut(local_id) {
                topic.status = SubmissionStatus::Sending;
                topic_to_submit = Some(topic.clone());
            }
        });
        
        let Some(topic) = topic_to_submit else {
            return Err("Topic not found".to_string());
        };
        
        // Call API to create topic
        match create_topic_api(
            &topic.category_id,
            &topic.title,
            &topic.content
        ).await {
            Ok(server_id) => {
                // Update with success
                self.pending_topics.update(|topics| {
                    if let Some(topic) = topics.get_mut(local_id) {
                        topic.status = SubmissionStatus::Success { server_id };
                    }
                });
                
                // Clean up successful topics after a delay
                let local_id = local_id.to_string();
                let this = self.clone();
                set_timeout(move || {
                    this.pending_topics.update(|topics| {
                        topics.remove(&local_id);
                    });
                }, std::time::Duration::from_secs(5));
                
                Ok(server_id)
            },
            Err(e) => {
                // Mark as failed
                self.pending_topics.update(|topics| {
                    if let Some(topic) = topics.get_mut(local_id) {
                        topic.status = SubmissionStatus::Failed { reason: e.clone() };
                    }
                });
                
                // Notify error handler
                self.error_handler.set(Some(format!("Failed to submit topic: {}", e)));
                
                Err(e)
            }
        }
    }
    
    // Edit content optimistically
    pub fn edit_content(&self, entity_id: String, field: String, old_value: String, new_value: String) -> String {
        let edit_id = format!("edit_{}_{}", entity_id, js_sys::Date::now());
        
        // Add to pending edits
        self.pending_edits.update(|edits| {
            edits.insert(edit_id.clone(), PendingEdit {
                entity_id,
                field,
                old_value,
                new_value,
                status: SubmissionStatus::Pending,
            });
        });
        
        edit_id
    }
    
    // Submit optimistic edit to server
    pub async fn submit_edit(&self, edit_id: &str) -> Result<(), String> {
        let mut edit_to_submit = None;
        
        // Update status to sending
        self.pending_edits.update(|edits| {
            if let Some(edit) = edits.get_mut(edit_id) {
                edit.status = SubmissionStatus::Sending;
                edit_to_submit = Some(edit.clone());
            }
        });
        
        let Some(edit) = edit_to_submit else {
            return Err("Edit not found".to_string());
        };
        
        // Call API to submit edit
        let result = match edit.field.as_str() {
            "post_content" => {
                edit_post_api(&edit.entity_id, &edit.new_value).await
            },
            "topic_title" => {
                edit_topic_title_api(&edit.entity_id, &edit.new_value).await
            },
            _ => Err("Unsupported edit type".to_string()),
        };
        
        match result {
            Ok(_) => {
                // Update with success
                self.pending_edits.update(|edits| {
                    if let Some(edit) = edits.get_mut(edit_id) {
                        // Use a fake ID since we don't need it
                        edit.status = SubmissionStatus::Success { server_id: 0 };
                    }
                });
                
                // Clean up successful edits after a delay
                let edit_id = edit_id.to_string();
                let this = self.clone();
                set_timeout(move || {
                    this.pending_edits.update(|edits| {
                        edits.remove(&edit_id);
                    });
                }, std::time::Duration::from_secs(5));
                
                Ok(())
            },
            Err(e) => {
                // Mark as failed
                self.pending_edits.update(|edits| {
                    if let Some(edit) = edits.get_mut(edit_id) {
                        edit.status = SubmissionStatus::Failed { reason: e.clone() };
                    }
                });
                
                // Notify error handler
                self.error_handler.set(Some(format!("Failed to submit edit: {}", e)));
                
                Err(e)
            }
        }
    }
    
    // Retry failed submissions
    pub async fn retry(&self, id: &str) -> Result<(), String> {
        // Check which collection this ID belongs to
        if id.starts_with("pending_post_") {
            self.submit_post(id).await.map(|_| ())
        } else if id.starts_with("pending_topic_") {
            self.submit_topic(id).await.map(|_| ())
        } else if id.starts_with("edit_") {
            self.submit_edit(id).await
        } else {
            Err("Unknown ID type".to_string())
        }
    }
    
    // Get optimistic post and topic lists
    pub fn get_optimistic_posts(&self, topic_id: i64) -> Signal<Vec<OptimisticPost>> {
        create_memo(move |_| {
            let posts = self.pending_posts.get();
            posts.iter()
                .filter(|(_, post)| post.topic_id == topic_id)
                .map(|(id, post)| OptimisticPost {
                    id: id.clone(),
                    content: post.content.clone(),
                    author: post.author.clone(),
                    timestamp: post.timestamp,
                    status: post.status.clone(),
                })
                .collect()
        }).into()
    }
    
    pub fn get_optimistic_topics(&self, category_id: i64) -> Signal<Vec<OptimisticTopic>> {
        create_memo(move |_| {
            let topics = self.pending_topics.get();
            topics.iter()
                .filter(|(_, topic)| topic.category_id == category_id)
                .map(|(id, topic)| OptimisticTopic {
                    id: id.clone(),
                    title: topic.title.clone(),
                    content: topic.content.clone(),
                    author: topic.author.clone(),
                    timestamp: topic.timestamp,
                    status: topic.status.clone(),
                })
                .collect()
        }).into()
    }
    
    pub fn get_optimistic_edits(&self, entity_id: &str) -> Signal<Vec<OptimisticEdit>> {
        let entity_id = entity_id.to_string();
        create_memo(move |_| {
            let edits = self.pending_edits.get();
            edits.iter()
                .filter(|(_, edit)| edit.entity_id == entity_id)
                .map(|(id, edit)| OptimisticEdit {
                    id: id.clone(),
                    field: edit.field.clone(),
                    old_value: edit.old_value.clone(),
                    new_value: edit.new_value.clone(),
                    status: edit.status.clone(),
                })
                .collect()
        }).into()
    }
}

// Public types for components to use
#[derive(Clone, Debug)]
pub struct OptimisticPost {
    pub id: String,
    pub content: String,
    pub author: String,
    pub timestamp: f64,
    pub status: SubmissionStatus,
}

#[derive(Clone, Debug)]
pub struct OptimisticTopic {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub timestamp: f64,
    pub status: SubmissionStatus,
}

#[derive(Clone, Debug)]
pub struct OptimisticEdit {
    pub id: String,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
    pub status: SubmissionStatus,
}

// Placeholder API functions - replace with your actual API calls
async fn create_post_api(topic_id: &i64, content: &str) -> Result<i64, String> {
    // Call your API here
    // For now, just simulate a delay
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    // Simulate server response
    Ok(js_sys::Date::now() as i64)
}

async fn create_topic_api(category_id: &i64, title: &str, content: &str) -> Result<i64, String> {
    // Call your API here
    // For now, just simulate a delay
    gloo_timers::future::TimeoutFuture::new(1000).await;
    
    // Simulate server response
    Ok(js_sys::Date::now() as i64)
}

async fn edit_post_api(post_id: &str, new_content: &str) -> Result<(), String> {
    // Call your API here
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    Ok(())
}

async fn edit_topic_title_api(topic_id: &str, new_title: &str) -> Result<(), String> {
    // Call your API here
    gloo_timers::future::TimeoutFuture::new(500).await;
    
    Ok(())
}