use uuid::Uuid;
use crate::{
    db::{
        topic_repository::TopicRepository, 
        post_repository::PostRepository,
    },
    models::{topic::Topic, post::Post},
};

pub struct TopicService {
    topic_repo: TopicRepository,
    post_repo: PostRepository,
}

impl TopicService {
    pub fn new(topic_repo: TopicRepository, post_repo: PostRepository) -> Self {
        Self { topic_repo, post_repo }
    }

    /// Creates a new topic with an initial post
    pub async fn create_topic_with_post(
        &self,
        title: String,
        category_id: Uuid,
        author_id: Uuid,
        content: String,
        assignment_id: Option<Uuid>,
    ) -> Result<(Topic, Post), String> {
        // Create topic
        let topic = Topic::new(title, category_id, author_id, assignment_id);
        
        let topic = self.topic_repo.create_topic(&topic)
            .await
            .map_err(|e| format!("Failed to create topic: {}", e))?;
        
        // Create initial post
        let post = Post::new(
            topic.id,
            author_id,
            content,
            true, // is_first_post
            None, // No parent post
        );
        
        let post = self.post_repo.create_post(&post)
            .await
            .map_err(|e| format!("Failed to create initial post: {}", e))?;
        
        Ok((topic, post))
    }

    /// Gets a topic with all its posts
    pub async fn get_topic_with_posts(&self, topic_id: &Uuid) -> Result<(Topic, Vec<Post>), String> {
        // Get topic
        let topic = self.topic_repo.find_topic_by_id(topic_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Topic with ID {} not found", topic_id))?;
        
        // Get all posts for this topic
        let posts = self.post_repo.list_posts_by_topic(topic_id)
            .await
            .map_err(|e| format!("Failed to fetch posts: {}", e))?;
        
        // Record a view (we'd typically do this based on the user viewing,
        // but for simplicity we're using the author)
        if let Err(e) = self.topic_repo.record_topic_view(topic_id, &topic.author_id).await {
            log::warn!("Failed to record topic view: {}", e);
        }
        
        Ok((topic, posts))
    }

    /// Adds a post to an existing topic
    pub async fn create_reply(
        &self,
        topic_id: &Uuid,
        author_id: Uuid,
        content: String,
        parent_id: Option<Uuid>,
    ) -> Result<Post, String> {
        // Check if topic exists
        let topic = self.topic_repo.find_topic_by_id(topic_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Topic with ID {} not found", topic_id))?;
        
        // Check if topic is closed
        if topic.closed {
            return Err("Cannot post to a closed topic".to_string());
        }
        
        // If parent_id is provided, ensure it exists
        if let Some(parent_id) = parent_id {
            self.post_repo.find_post_by_id(&parent_id)
                .await
                .map_err(|e| format!("Database error: {}", e))?
                .ok_or_else(|| format!("Parent post with ID {} not found", parent_id))?;
        }
        
        // Create post
        let post = Post::new(
            *topic_id,
            author_id,
            content,
            false, // not first post
            parent_id,
        );
        
        let post = self.post_repo.create_post(&post)
            .await
            .map_err(|e| format!("Failed to create post: {}", e))?;
        
        // Update topic's updated_at timestamp
        let mut updated_topic = topic;
        updated_topic.updated_at = chrono::Utc::now();
        
        self.topic_repo.update_topic(&updated_topic)
            .await
            .map_err(|e| format!("Failed to update topic timestamp: {}", e))?;
        
        Ok(post)
    }
}