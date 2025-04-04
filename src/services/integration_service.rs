use leptos::*;
use crate::models::forum::{Category, Topic, CreateCategoryRequest, CreateTopicRequest};
use crate::services::course_service::CourseService;
use crate::services::forum_service::ForumService;
use crate::utils::api_client::ApiClient;
use crate::sync::{SyncManager, SyncOperation};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub id: String, // Use String to support both server IDs and local IDs
    pub source_type: EntityType,
    pub source_id: String,
    pub target_type: EntityType,
    pub target_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub id: String,
    pub user_id: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub action_type: ActionType,
    pub created_at: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    Course,
    Module,
    Assignment,
    Category,
    Topic,
    Post,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionType {
    Created,
    Updated,
    Deleted,
    Viewed,
    Commented,
    Submitted,
    Graded,
}

#[derive(Clone)]
pub struct IntegrationService {
    client: ApiClient,
    course_service: CourseService,
    forum_service: ForumService,
    sync_manager: Option<Arc<SyncManager>>, // Optional to maintain backward compatibility
}

impl IntegrationService {
    pub fn new(course_service: CourseService, forum_service: ForumService, 
               sync_manager: Option<Arc<SyncManager>>) -> Self {
        Self {
            client: ApiClient::new(),
            course_service,
            forum_service,
            sync_manager,
        }
    }
    
    // Get category for a course
    pub async fn get_course_category(&self, course_id: i64) -> Result<Category, String> {
        self.client
            .get::<Category>(&format!("/courses/{}/category", course_id))
            .await
            .map_err(|e| e.to_string())
    }
    
    // Get or create a category for a course
    pub async fn ensure_course_category(&self, course_id: i64) -> Result<Category, String> {
        if self.is_offline() {
            // Try to get from local storage first
            match self.get_course_category_local(course_id).await {
                Ok(category) => Ok(category),
                Err(_) => {
                    // Create locally and queue for sync when online
                    let category = Category {
                        id: format!("local-{}-{}", course_id, chrono::Utc::now().timestamp_millis()),
                        name: format!("Course {}", course_id),
                        // Add other required fields with default values
                        // ...
                    };
                    
                    // If we have a sync manager, queue this operation
                    if let Some(sync_manager) = &self.sync_manager {
                        let _ = sync_manager.queue_operation(
                            SyncOperation::Create {
                                entity_type: "course_category".to_string(),
                                data: serde_json::to_value(&category).unwrap_or_default(),
                            }
                        ).await;
                    }
                    
                    // Return the local category
                    Ok(category)
                }
            }
        } else {
            // Original online implementation
            self.client
                .post::<(), Category>(&format!("/courses/{}/category", course_id), ())
                .await
                .map_err(|e| e.to_string())
        }
    }
    
    // Get topic for a module
    pub async fn get_module_topic(&self, module_id: i64) -> Result<Option<Topic>, String> {
        match self.client
            .get::<Topic>(&format!("/courses/0/modules/{}/discussion", module_id))
            .await {
                Ok(topic) => Ok(Some(topic)),
                Err(e) if e.is_not_found() => Ok(None),
                Err(e) => Err(e.to_string())
            }
    }
    
    // Create a discussion topic for a module
    pub async fn create_module_discussion(&self, course_id: i64, module_id: i64) -> Result<Topic, String> {
        self.client
            .post::<(), Topic>(
                &format!("/courses/{}/modules/{}/discussion", course_id, module_id),
                ()
            )
            .await
            .map_err(|e| e.to_string())
    }
    
    // Get topic for an assignment
    pub async fn get_assignment_topic(&self, assignment_id: i64) -> Result<Option<Topic>, String> {
        match self.client
            .get::<Topic>(&format!("/courses/0/assignments/{}/discussion", assignment_id))
            .await {
                Ok(topic) => Ok(Some(topic)),
                Err(e) if e.is_not_found() => Ok(None),
                Err(e) => Err(e.to_string())
            }
    }
    
    // Create a discussion topic for an assignment
    pub async fn create_assignment_discussion(&self, course_id: i64, assignment_id: i64) -> Result<Topic, String> {
        self.client
            .post::<(), Topic>(
                &format!("/courses/{}/assignments/{}/discussion", course_id, assignment_id),
                ()
            )
            .await
            .map_err(|e| e.to_string())
    }
    
    // Get recent forum activity for a course
    pub async fn get_course_forum_activity(&self, course_id: i64, limit: usize) -> Result<Vec<Topic>, String> {
        self.client
            .get::<Vec<Topic>>(&format!("/courses/{}/forum/activity?limit={}", course_id, limit))
            .await
            .map_err(|e| e.to_string())
    }
    
    // Create a general discussion topic for a course
    pub async fn create_general_discussion(&self, course_id: i64) -> Result<Topic, String> {
        // First ensure we have a category
        let category = self.ensure_course_category(course_id).await?;
        
        // Create a general discussion topic
        let request = CreateTopicRequest {
            title: "General Discussion".to_string(),
            category_id: category.id,
            content: "This is a general discussion board for the course. Feel free to ask questions or share your thoughts.".to_string(),
        };
        
        self.forum_service.create_topic(request).await
    }
    
    // Add an is_offline helper method
    fn is_offline(&self) -> bool {
        #[cfg(debug_assertions)]
        let offline = std::env::var("FORCE_OFFLINE").is_ok();
        #[cfg(not(debug_assertions))]
        let offline = self.client.is_offline();
        
        offline
    }
    
    // Add local storage methods
    async fn get_course_category_local(&self, course_id: i64) -> Result<Category, String> {
        // Implement local storage retrieval
        // For now, return an error to force creation path
        Err("Not implemented".to_string())
    }

    // Add a method to create references
    pub async fn create_reference(
        &self, 
        source_type: EntityType,
        source_id: &str,
        target_type: EntityType,
        target_id: &str,
        metadata: Option<serde_json::Value>
    ) -> Result<CrossReference, String> {
        let reference = CrossReference {
            id: format!("local-{}", chrono::Utc::now().timestamp_millis()), // Temporary ID
            source_type,
            source_id: source_id.to_string(),
            target_type,
            target_id: target_id.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata,
        };
        
        if self.is_offline() {
            // Store locally and queue for sync
            if let Some(sync_manager) = &self.sync_manager {
                let _ = sync_manager.queue_operation(
                    SyncOperation::Reference {
                        source_type: format!("{:?}", source_type).to_lowercase(),
                        source_id: source_id.to_string(),
                        target_type: format!("{:?}", target_type).to_lowercase(),
                        target_id: target_id.to_string(),
                    }
                ).await;
            }
            Ok(reference)
        } else {
            // Send to server
            self.client
                .post::<CrossReference, CrossReference>("/api/v1/references", reference)
                .await
                .map_err(|e| e.to_string())
        }
    }

    // Add method to record activities
    pub async fn record_activity(
        &self,
        user_id: &str,
        entity_type: EntityType,
        entity_id: &str,
        action_type: ActionType,
        metadata: Option<serde_json::Value>,
    ) -> Result<ActivityEntry, String> {
        let activity = ActivityEntry {
            id: format!("local-{}", chrono::Utc::now().timestamp_millis()),
            user_id: user_id.to_string(),
            entity_type,
            entity_id: entity_id.to_string(),
            action_type,
            created_at: Utc::now(),
            metadata,
        };
        
        if self.is_offline() {
            // Store locally and queue for sync
            if let Some(sync_manager) = &self.sync_manager {
                let _ = sync_manager.queue_operation(
                    SyncOperation::Create {
                        entity_type: "activity".to_string(),
                        data: serde_json::to_value(&activity).unwrap_or_default(),
                    }
                ).await;
            }
            Ok(activity)
        } else {
            // Send to server
            self.client
                .post::<ActivityEntry, ActivityEntry>("/api/v1/activities", activity)
                .await
                .map_err(|e| e.to_string())
        }
    }
}

/// Module providing course-forum integration components
#[component]
pub fn CourseForumSection(
    cx: Scope,
    #[prop()] course_id: i64,
) -> impl IntoView {
    let integration_service = use_context::<IntegrationService>(cx)
        .expect("IntegrationService should be provided");
    
    let discussions = create_resource(
        cx,
        move || course_id,
        move |id| async move {
            integration_service.get_course_forum_activity(id, 5).await
        }
    );
    
    let create_general_discussion = create_action(cx, move |course_id: &i64| {
        let id = *course_id;
        async move {
            let create_request = CreateTopicRequest {
                title: "General Discussion".to_string(),
                category_id: 0, // Will be replaced in the service
                content: "This is a general discussion board for the course. Feel free to ask questions or share thoughts.".to_string(),
            };
            
            // First ensure we have a category
            match integration_service.ensure_course_category(id).await {
                Ok(category) => {
                    let mut request = create_request;
                    request.category_id = category.id;
                    integration_service.forum_service.create_topic(request).await
                },
                Err(e) => Err(format!("Failed to create category: {}", e).into()),
            }
        }
    });

    view! { cx,
        <div class="course-forum-section">
            <div class="section-header">
                <h2>"Course Discussions"</h2>
                <button 
                    class="new-discussion-btn"
                    on:click=move |_| {
                        create_general_discussion.dispatch(course_id);
                    }
                    disabled=create_general_discussion.pending()
                >
                    "New Discussion"
                </button>
            </div>
            
            <div class="discussion-list">
                {move || match discussions.read(cx) {
                    None => view! { cx, <p>"Loading discussions..."</p> },
                    Some(Ok(topics)) => {
                        if topics.is_empty() {
                            view! { cx, 
                                <p class="no-discussions">
                                    "No discussions yet. Start the first one!"
                                </p> 
                            }
                        } else {
                            view! { cx,
                                <ul class="topic-list">
                                    {topics.into_iter().map(|topic| view! { cx,
                                        <li class="topic-item">
                                            <a href={format!("/forum/t/{}", topic.id)} class="topic-link">
                                                <span class="topic-title">{topic.title}</span>
                                                <span class="topic-stats">
                                                    {topic.reply_count} " replies • " 
                                                    {format_recent_date(topic.last_post_at.unwrap_or(topic.created_at))}
                                                </span>
                                            </a>
                                        </li>
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }
                        }
                    },
                    Some(Err(e)) => view! { cx, <p class="error">"Error: " {e}</p> }
                }}
            </div>
            
            {move || {
                if let Some(Err(e)) = create_general_discussion.value().get() {
                    view! { cx, <p class="error">"Error creating discussion: " {e}</p> }
                } else {
                    view! { cx, <></> }
                }
            }}
            
            <div class="forum-footer">
                <a href={format!("/courses/{}/forum", course_id)} class="view-all-discussions">
                    "View All Discussions"
                </a>
            </div>
        </div>
    }
}

// Helper function for displaying dates
fn format_recent_date(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 30 {
        date.format("%b %d, %Y").to_string()
    } else if diff.num_days() > 0 {
        format!("{} days ago", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{} hours ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{} minutes ago", diff.num_minutes())
    } else {
        "just now".to_string()
    }
}

// Helper function to create URL-friendly slugs
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .replace("--", "-")
}