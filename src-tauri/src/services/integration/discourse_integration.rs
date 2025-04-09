use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::forum::category::Category;
use crate::models::user::user::User;
use crate::db::DB;
use crate::error::Error;
use uuid::Uuid;
use chrono::Utc;
use async_trait::async_trait;

// Define Discourse API client models (simplified for example)
pub mod discourse_api {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Topic {
        pub id: i64,
        pub title: String,
        pub fancy_title: String,
        pub posts_count: Option<i32>,
        pub created_at: DateTime<Utc>,
        pub views: Option<i32>,
        pub reply_count: Option<i32>,
        pub last_posted_at: Option<DateTime<Utc>>,
        pub closed: Option<bool>,
        pub archived: Option<bool>,
        pub pinned: Option<bool>,
        pub category_id: Option<i64>,
        pub user_id: i64,
        pub tags: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Post {
        pub id: i64,
        pub user_id: i64,
        pub topic_id: i64,
        pub post_number: i32,
        pub raw: String,
        pub cooked: String, // HTML content
        pub created_at: DateTime<Utc>,
        pub updated_at: Option<DateTime<Utc>>,
        pub reply_to_post_number: Option<i32>,
        pub like_count: Option<i32>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Category {
        pub id: i64,
        pub name: String,
        pub slug: String,
        pub description: Option<String>,
        pub color: String,
        pub parent_category_id: Option<i64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub id: i64,
        pub username: String,
        pub name: Option<String>,
        pub avatar_template: String,
        pub created_at: DateTime<Utc>,
        pub last_seen_at: Option<DateTime<Utc>>,
        pub trust_level: i32,
    }
}

#[async_trait]
pub trait DiscourseIntegration {
    async fn sync_topic(&self, discourse_topic_id: i64) -> Result<Topic, Error>;
    async fn sync_category(&self, discourse_category_id: i64) -> Result<Category, Error>;
    async fn sync_user(&self, discourse_user_id: i64) -> Result<User, Error>;
    async fn push_topic_to_discourse(&self, topic: &Topic) -> Result<i64, Error>;
    async fn push_post_to_discourse(&self, post: &Post) -> Result<i64, Error>;
}

pub struct DiscourseIntegrationService {
    db: DB,
    api_url: String,
    api_key: String,
    api_username: String,
}

impl DiscourseIntegrationService {
    pub fn new(db: DB, api_url: String, api_key: String, api_username: String) -> Self {
        DiscourseIntegrationService {
            db,
            api_url,
            api_key,
            api_username,
        }
    }

    async fn fetch_discourse_topic(&self, topic_id: i64) -> Result<discourse_api::Topic, Error> {
        // In a real implementation, this would make an HTTP request to the Discourse API
        let client = reqwest::Client::new();
        let url = format!("{}/t/{}.json", self.api_url, topic_id);
        
        let response = client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await
            .map_err(|e| Error::ExternalApi(format!("Discourse API error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::ExternalApi(format!(
                "Discourse API returned status: {}", response.status()
            )));
        }
        
        let discourse_topic = response
            .json::<discourse_api::Topic>()
            .await
            .map_err(|e| Error::Parsing(format!("Failed to parse Discourse topic: {}", e)))?;
        
        Ok(discourse_topic)
    }

    async fn fetch_discourse_posts(&self, topic_id: i64) -> Result<Vec<discourse_api::Post>, Error> {
        // In a real implementation, this would make an HTTP request to the Discourse API
        let client = reqwest::Client::new();
        let url = format!("{}/t/{}/posts.json", self.api_url, topic_id);
        
        let response = client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await
            .map_err(|e| Error::ExternalApi(format!("Discourse API error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::ExternalApi(format!(
                "Discourse API returned status: {}", response.status()
            )));
        }
        
        // Discourse returns a wrapper object with a "post_stream" field containing posts
        let wrapper: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Parsing(format!("Failed to parse Discourse response: {}", e)))?;
        
        let posts = serde_json::from_value::<Vec<discourse_api::Post>>(
            wrapper["post_stream"]["posts"].clone()
        )
        .map_err(|e| Error::Parsing(format!("Failed to parse Discourse posts: {}", e)))?;
        
        Ok(posts)
    }

    async fn fetch_discourse_category(&self, category_id: i64) -> Result<discourse_api::Category, Error> {
        // In a real implementation, this would make an HTTP request to the Discourse API
        let client = reqwest::Client::new();
        let url = format!("{}/c/{}.json", self.api_url, category_id);
        
        let response = client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await
            .map_err(|e| Error::ExternalApi(format!("Discourse API error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::ExternalApi(format!(
                "Discourse API returned status: {}", response.status()
            )));
        }
        
        // Discourse category API returns a wrapper object with "category" field
        let wrapper: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Parsing(format!("Failed to parse Discourse response: {}", e)))?;
        
        let category = serde_json::from_value::<discourse_api::Category>(
            wrapper["category"].clone()
        )
        .map_err(|e| Error::Parsing(format!("Failed to parse Discourse category: {}", e)))?;
        
        Ok(category)
    }

    async fn fetch_discourse_user(&self, user_id: i64) -> Result<discourse_api::User, Error> {
        // In a real implementation, this would make an HTTP request to the Discourse API
        let client = reqwest::Client::new();
        let url = format!("{}/users/{}.json", self.api_url, user_id);
        
        let response = client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await
            .map_err(|e| Error::ExternalApi(format!("Discourse API error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::ExternalApi(format!(
                "Discourse API returned status: {}", response.status()
            )));
        }
        
        // Discourse user API returns a wrapper object with "user" field
        let wrapper: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Parsing(format!("Failed to parse Discourse response: {}", e)))?;
        
        let user = serde_json::from_value::<discourse_api::User>(
            wrapper["user"].clone()
        )
        .map_err(|e| Error::Parsing(format!("Failed to parse Discourse user: {}", e)))?;
        
        Ok(user)
    }

    async fn convert_discourse_topic_to_model(&self, discourse_topic: discourse_api::Topic) 
        -> Result<Topic, Error> {
        // First, try to find if we already have this topic in our database
        let existing_topic = Topic::find_by_discourse_id(&self.db, discourse_topic.id).await;
        
        // Get or create the author
        let author_id = match self.sync_user(discourse_topic.user_id).await {
            Ok(user) => user.id,
            Err(_) => Uuid::nil(), // Fallback
        };
        
        // Get or create the category if it exists
        let category_id = if let Some(category_id) = discourse_topic.category_id {
            match self.sync_category(category_id).await {
                Ok(category) => Some(category.id),
                Err(_) => None,
            }
        } else {
            None
        };
        
        if let Ok(mut topic) = existing_topic {
            // Update existing topic
            topic.title = discourse_topic.title;
            topic.author_id = author_id;
            topic.category_id = category_id;
            topic.pinned = discourse_topic.pinned.unwrap_or(false);
            topic.locked = discourse_topic.closed.unwrap_or(false);
            topic.views = discourse_topic.views.unwrap_or(0);
            topic.post_count = discourse_topic.posts_count.unwrap_or(0);
            topic.last_post_at = discourse_topic.last_posted_at;
            topic.updated_at = Utc::now();
            topic.sync_status = crate::models::forum::topic::SyncStatus::SyncedWithDiscourse;
            
            // Update tags if available
            if let Some(tags) = discourse_topic.tags {
                topic.tags = tags;
            }
            
            // Update in database
            topic.update(&self.db).await?;
            
            Ok(topic)
        } else {
            // Create new topic
            let mut topic = Topic::new(
                discourse_topic.title,
                author_id,
                "".to_string(), // We'll get content from first post
            );
            
            // Set Discourse-specific fields
            topic.discourse_topic_id = Some(discourse_topic.id);
            topic.category_id = category_id;
            topic.pinned = discourse_topic.pinned.unwrap_or(false);
            topic.locked = discourse_topic.closed.unwrap_or(false);
            topic.created_at = discourse_topic.created_at;
            topic.views = discourse_topic.views.unwrap_or(0);
            topic.post_count = discourse_topic.posts_count.unwrap_or(0);
            topic.last_post_at = discourse_topic.last_posted_at;
            topic.allow_rating = true; // Discourse allows post ratings/likes
            topic.sync_status = crate::models::forum::topic::SyncStatus::SyncedWithDiscourse;
            
            // Set tags if available
            if let Some(tags) = discourse_topic.tags {
                topic.tags = tags;
            }
            
            // Save to database
            topic.create(&self.db).await?;
            
            Ok(topic)
        }
    }

    async fn convert_discourse_post_to_model(&self, 
                                          discourse_post: discourse_api::Post,
                                          local_topic_id: Uuid) -> Result<Post, Error> {
        // First, try to find if we already have this post in our database
        let existing_post = Post::find_by_discourse_id(&self.db, discourse_post.id).await;
        
        // Get or create the author
        let author_id = match self.sync_user(discourse_post.user_id).await {
            Ok(user) => user.id,
            Err(_) => Uuid::nil(), // Fallback
        };
        
        if let Ok(mut post) = existing_post {
            // Update existing post
            post.content = discourse_post.raw;
            post.html_content = Some(discourse_post.cooked);
            post.updated_at = discourse_post.updated_at.unwrap_or_else(Utc::now);
            post.likes = discourse_post.like_count.unwrap_or(0);
            post.sync_status = crate::models::forum::post::SyncStatus::SyncedWithDiscourse;
            
            // Update in database
            post.update(&self.db).await?;
            
            Ok(post)
        } else {
            // Create new post
            let mut post = Post::new(
                local_topic_id,
                author_id,
                discourse_post.raw,
            );
            
            // Set Discourse-specific fields
            post.discourse_post_id = Some(discourse_post.id);
            post.html_content = Some(discourse_post.cooked);
            post.created_at = discourse_post.created_at;
            post.updated_at = discourse_post.updated_at.unwrap_or_else(Utc::now);
            post.likes = discourse_post.like_count.unwrap_or(0);
            post.sync_status = crate::models::forum::post::SyncStatus::SyncedWithDiscourse;
            
            // Handle parent relationship if this is a reply
            if let Some(reply_to) = discourse_post.reply_to_post_number {
                // Find the parent post by post number
                // This is a simplified approach - in a real implementation,
                // you would need to store the Discourse post_number in your model
                // and use it to look up the parent
                
                // For now, we'll just create without parent
            }
            
            // If this is the first post (post_number == 1), update the topic content
            if discourse_post.post_number == 1 {
                if let Ok(mut topic) = Topic::find(&self.db, local_topic_id).await {
                    topic.content = discourse_post.raw;
                    let _ = topic.update(&self.db).await;
                }
            }
            
            // Save to database
            post.create(&self.db).await?;
            
            Ok(post)
        }
    }

    async fn update_post_references(&self, topic_id: Uuid) -> Result<(), Error> {
        // This function updates parent-child relationships between posts
        // after all posts for a topic have been synced
        
        // Get all posts for this topic
        let posts = Post::find_by_topic_id(&self.db, topic_id).await?;
        
        // Get a mapping of discourse_post_id to our post id
        let mut post_map = std::collections::HashMap::new();
        for post in &posts {
            if let Some(discourse_id) = post.discourse_post_id {
                post_map.insert(discourse_id, post.id);
            }
        }
        
        // Get Discourse posts again to have reply information
        if let Some(discourse_topic_id) = Topic::find(&self.db, topic_id).await?.discourse_topic_id {
            let discourse_posts = self.fetch_discourse_posts(discourse_topic_id).await?;
            
            // Update parent references
            for discourse_post in discourse_posts {
                if let Some(reply_to_number) = discourse_post.reply_to_post_number {
                    // Find the post in our database
                    if let Some(post_id) = post_map.get(&discourse_post.id) {
                        // Find the parent post ID
                        for parent in &discourse_posts {
                            if parent.post_number == reply_to_number {
                                if let Some(parent_id) = post_map.get(&parent.id) {
                                    // Update the parent reference
                                    if let Ok(mut post) = Post::find(&self.db, *post_id).await {
                                        post.parent_id = Some(*parent_id);
                                        let _ = post.update(&self.db).await;
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl DiscourseIntegration for DiscourseIntegrationService {
    async fn sync_topic(&self, discourse_topic_id: i64) -> Result<Topic, Error> {
        // Fetch the topic from Discourse
        let discourse_topic = self.fetch_discourse_topic(discourse_topic_id).await?;
        
        // Convert to our model
        let topic = self.convert_discourse_topic_to_model(discourse_topic).await?;
        
        // Fetch all posts for this topic
        let discourse_posts = self.fetch_discourse_posts(discourse_topic_id).await?;
        
        // Convert each post
        for discourse_post in discourse_posts {
            let _ = self.convert_discourse_post_to_model(discourse_post, topic.id).await;
            // Note: We're ignoring individual post sync errors to ensure the topic sync completes
        }
        
        // Update parent-child relationships between posts
        let _ = self.update_post_references(topic.id).await;
        
        // Update post count
        let mut topic_with_count = topic.clone();
        let _ = topic_with_count.update_post_count(&self.db).await;
        
        Ok(topic)
    }
    
    async fn sync_category(&self, discourse_category_id: i64) -> Result<Category, Error> {
        // First check if we already have this category
        let existing_category = Category::find_by_discourse_id(&self.db, discourse_category_id).await;
        
        if let Ok(category) = existing_category {
            return Ok(category);
        }
        
        // If not found, fetch from Discourse
        let discourse_category = self.fetch_discourse_category(discourse_category_id).await?;
        
        // Create new category
        let mut category = Category::new(
            discourse_category.name,
            discourse_category.description,
        );
        
        category.discourse_category_id = Some(discourse_category_id);
        category.slug = discourse_category.slug;
        category.color = discourse_category.color;
        
        // Handle parent category if it exists
        if let Some(parent_id) = discourse_category.parent_category_id {
            // Recursively sync parent category
            if let Ok(parent) = self.sync_category(parent_id).await {
                category.parent_id = Some(parent.id);
            }
        }
        
        // Save to database
        category.create(&self.db).await?;
        
        Ok(category)
    }
    
    async fn sync_user(&self, discourse_user_id: i64) -> Result<User, Error> {
        // First check if we already have this user
        let existing_user = User::find_by_discourse_id(&self.db, discourse_user_id).await;
        
        if let Ok(user) = existing_user {
            return Ok(user);
        }
        
        // If not found, fetch from Discourse
        let discourse_user = self.fetch_discourse_user(discourse_user_id).await?;
        
        // Create new user
        let mut user = User::new(
            discourse_user.name.unwrap_or_else(|| discourse_user.username.clone()),
            format!("{}@example.com", discourse_user.username), // Discourse API doesn't expose emails
            discourse_user.username,
        );
        
        user.discourse_user_id = Some(discourse_user_id);
        user.trust_level = Some(discourse_user.trust_level);
        user.created_at = discourse_user.created_at;
        user.last_seen_at = discourse_user.last_seen_at;
        
        // Convert avatar template to URL
        if !discourse_user.avatar_template.is_empty() {
            let avatar_url = if discourse_user.avatar_template.starts_with("http") {
                discourse_user.avatar_template
            } else {
                format!("{}{}", self.api_url, discourse_user.avatar_template)
                    .replace("{size}", "120")
            };
            user.avatar_url = Some(avatar_url);
        }
        
        // Save to database
        user.create(&self.db).await?;
        
        Ok(user)
    }
    
    async fn push_topic_to_discourse(&self, topic: &Topic) -> Result<i64, Error> {
        // In a real implementation, this would create or update a topic in Discourse
        // For this example, we'll just return a mock Discourse ID
        
        if let Some(existing_id) = topic.discourse_topic_id {
            // If we already have a Discourse ID, this is an update
            
            // Make PUT request to Discourse API
            // PUT /t/{topic_id}.json
            
            Ok(existing_id)
        } else {
            // If no Discourse ID, this is a create
            
            // Make POST request to Discourse API
            // POST /posts.json
            
            let new_id = 1000 + rand::random::<i64>() % 9000;
            
            Ok(new_id)
        }
    }
    
    async fn push_post_to_discourse(&self, post: &Post) -> Result<i64, Error> {
        // In a real implementation, this would create or update a post in Discourse
        // For this example, we'll just return a mock Discourse ID
        
        // Ensure we have the Discourse topic ID for the parent topic
        let topic = Topic::find(&self.db, post.topic_id).await?;
        
        let discourse_topic_id = if let Some(id) = topic.discourse_topic_id {
            id
        } else {
            // If the topic doesn't exist in Discourse yet, push it first
            let discourse_id = self.push_topic_to_discourse(&topic).await?;
            discourse_id
        };
        
        if let Some(existing_id) = post.discourse_post_id {
            // If we already have a Discourse post ID, this is an update
            
            // Make PUT request to Discourse API
            // PUT /posts/{post_id}.json
            
            Ok(existing_id)
        } else {
            // If no Discourse ID, this is a create
            
            // Make POST request to Discourse API
            // POST /posts.json
            
            let new_id = 2000 + rand::random::<i64>() % 9000;
            
            Ok(new_id)
        }
    }
}