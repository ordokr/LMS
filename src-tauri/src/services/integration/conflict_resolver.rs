use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::error::Error;
use chrono::{DateTime, Utc};
use log::{info, warn};
use crate::sync::version_vector::{VersionVector, CausalRelation};

/// Conflict resolution strategies when the same entity is modified in both systems
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConflictStrategy {
    /// Use data from Canvas
    PreferCanvas,
    /// Use data from Discourse
    PreferDiscourse,
    /// Use the most recently updated content
    PreferMostRecent,
    /// Merge changes when possible, preferring Canvas for conflicts
    MergePreferCanvas,
    /// Merge changes when possible, preferring Discourse for conflicts
    MergePreferDiscourse,
}

/// Conflict resolver for bidirectional synchronization
pub struct ConflictResolver {
    strategy: ConflictStrategy,
}

impl ConflictResolver {
    /// Create a new conflict resolver with the specified strategy
    pub fn new(strategy: ConflictStrategy) -> Self {
        Self { strategy }
    }
    
    /// Resolve conflicts between a Canvas topic and a Discourse topic using version vectors
    pub fn resolve_topic_conflict_with_version_vector(
        &self, 
        canvas_topic: &Topic, 
        discourse_topic: &Topic,
        canvas_vector: &VersionVector,
        discourse_vector: &VersionVector
    ) -> Result<Topic, Error> {
        // First, determine the causal relation between the version vectors
        match canvas_vector.causal_relation(discourse_vector) {
            CausalRelation::Identical => {
                // No conflict - versions are identical
                info!("No conflict - topic versions are identical");
                Ok(canvas_topic.clone()) // Either would work
            },
            CausalRelation::HappensBefore => {
                // Canvas is an ancestor of Discourse - use Discourse
                info!("Canvas topic is ancestor of Discourse topic: {}", canvas_topic.id);
                Ok(discourse_topic.clone())
            },
            CausalRelation::HappensAfter => {
                // Discourse is an ancestor of Canvas - use Canvas
                info!("Discourse topic is ancestor of Canvas topic: {}", discourse_topic.id);
                Ok(canvas_topic.clone())
            },
            CausalRelation::Concurrent => {
                // True conflict - use conflict resolution strategy
                warn!("Concurrent modifications detected for topic: Canvas ID: {:?}, Discourse ID: {:?}", 
                      canvas_topic.canvas_discussion_id, discourse_topic.discourse_topic_id);
                
                // Apply the configured resolution strategy
                self.resolve_topic_conflict(canvas_topic, discourse_topic)
            }
        }
    }
    
    /// Resolve conflicts between a Canvas post and a Discourse post using version vectors
    pub fn resolve_post_conflict_with_version_vector(
        &self, 
        canvas_post: &Post, 
        discourse_post: &Post,
        canvas_vector: &VersionVector,
        discourse_vector: &VersionVector
    ) -> Result<Post, Error> {
        // First, determine the causal relation between the version vectors
        match canvas_vector.causal_relation(discourse_vector) {
            CausalRelation::Identical => {
                // No conflict - versions are identical
                info!("No conflict - post versions are identical");
                Ok(canvas_post.clone()) // Either would work
            },
            CausalRelation::HappensBefore => {
                // Canvas is an ancestor of Discourse - use Discourse
                info!("Canvas post is ancestor of Discourse post: {}", canvas_post.id);
                Ok(discourse_post.clone())
            },
            CausalRelation::HappensAfter => {
                // Discourse is an ancestor of Canvas - use Canvas
                info!("Discourse post is ancestor of Canvas post: {}", discourse_post.id);
                Ok(canvas_post.clone())
            },
            CausalRelation::Concurrent => {
                // True conflict - use conflict resolution strategy
                warn!("Concurrent modifications detected for post: Canvas ID: {:?}, Discourse ID: {:?}", 
                      canvas_post.canvas_entry_id, discourse_post.discourse_post_id);
                
                // Apply the configured resolution strategy
                self.resolve_post_conflict(canvas_post, discourse_post)
            }
        }
    }
    
    /// Resolve conflicts between a Canvas topic and a Discourse topic
    pub fn resolve_topic_conflict(&self, canvas_topic: &Topic, discourse_topic: &Topic) -> Result<Topic, Error> {
        match self.strategy {
            ConflictStrategy::PreferCanvas => Ok(canvas_topic.clone()),
            ConflictStrategy::PreferDiscourse => Ok(discourse_topic.clone()),
            ConflictStrategy::PreferMostRecent => {
                if canvas_topic.updated_at > discourse_topic.updated_at {
                    info!("Using Canvas topic (more recent): {}", canvas_topic.id);
                    Ok(canvas_topic.clone())
                } else {
                    info!("Using Discourse topic (more recent): {}", discourse_topic.id);
                    Ok(discourse_topic.clone())
                }
            },
            ConflictStrategy::MergePreferCanvas | ConflictStrategy::MergePreferDiscourse => {
                // Start with the preferred base
                let base = if self.strategy == ConflictStrategy::MergePreferCanvas {
                    canvas_topic.clone()
                } else {
                    discourse_topic.clone()
                };
                
                // Smart merge logic
                let mut result = base;
                
                // Keep the most recent content if it was updated
                if canvas_topic.updated_at > discourse_topic.updated_at && 
                   canvas_topic.updated_at > result.updated_at {
                    result.content = canvas_topic.content.clone();
                    result.title = canvas_topic.title.clone();
                } else if discourse_topic.updated_at > canvas_topic.updated_at && 
                          discourse_topic.updated_at > result.updated_at {
                    result.content = discourse_topic.content.clone();
                    result.title = discourse_topic.title.clone();
                }
                
                // Combine tags from both sources
                let mut combined_tags = result.tags.clone();
                
                if self.strategy == ConflictStrategy::MergePreferCanvas {
                    for tag in &canvas_topic.tags {
                        if !combined_tags.contains(tag) {
                            combined_tags.push(tag.clone());
                        }
                    }
                    for tag in &discourse_topic.tags {
                        if !combined_tags.contains(tag) {
                            combined_tags.push(tag.clone());
                        }
                    }
                } else {
                    for tag in &discourse_topic.tags {
                        if !combined_tags.contains(tag) {
                            combined_tags.push(tag.clone());
                        }
                    }
                    for tag in &canvas_topic.tags {
                        if !combined_tags.contains(tag) {
                            combined_tags.push(tag.clone());
                        }
                    }
                }
                
                result.tags = combined_tags;
                
                // Use the most recent views/post counts
                if discourse_topic.updated_at > canvas_topic.updated_at {
                    result.views = discourse_topic.views;
                    result.post_count = discourse_topic.post_count;
                }
                
                // Keep track of sync time
                result.updated_at = Utc::now();
                
                Ok(result)
            }
        }
    }
    
    /// Resolve conflicts between a Canvas post and a Discourse post
    pub fn resolve_post_conflict(&self, canvas_post: &Post, discourse_post: &Post) -> Result<Post, Error> {
        match self.strategy {
            ConflictStrategy::PreferCanvas => Ok(canvas_post.clone()),
            ConflictStrategy::PreferDiscourse => Ok(discourse_post.clone()),
            ConflictStrategy::PreferMostRecent => {
                if canvas_post.updated_at > discourse_post.updated_at {
                    info!("Using Canvas post (more recent): {}", canvas_post.id);
                    Ok(canvas_post.clone())
                } else {
                    info!("Using Discourse post (more recent): {}", discourse_post.id);
                    Ok(discourse_post.clone())
                }
            },
            ConflictStrategy::MergePreferCanvas | ConflictStrategy::MergePreferDiscourse => {
                // Start with the preferred base
                let base = if self.strategy == ConflictStrategy::MergePreferCanvas {
                    canvas_post.clone()
                } else {
                    discourse_post.clone()
                };
                
                // Smart merge logic
                let mut result = base;
                
                // Keep the most recent content if it was updated
                if canvas_post.updated_at > discourse_post.updated_at && 
                   canvas_post.updated_at > result.updated_at {
                    result.content = canvas_post.content.clone();
                } else if discourse_post.updated_at > canvas_post.updated_at && 
                          discourse_post.updated_at > result.updated_at {
                    result.content = discourse_post.content.clone();
                }
                
                // Keep Discourse-specific data
                if let Some(html) = &discourse_post.html_content {
                    result.html_content = Some(html.clone());
                }
                
                if discourse_post.likes > result.likes {
                    result.likes = discourse_post.likes;
                }
                
                // Keep track of sync time
                result.updated_at = Utc::now();
                
                Ok(result)
            }
        }
    }
    
    /// Determine whether a sync is needed based on timestamps
    pub fn needs_sync(&self, local_updated_at: DateTime<Utc>, remote_updated_at: DateTime<Utc>, last_sync: Option<DateTime<Utc>>) -> bool {
        // If never synced, then yes
        if last_sync.is_none() {
            return true;
        }
        
        let last_sync = last_sync.unwrap();
        
        // If either has been updated since the last sync, then yes
        local_updated_at > last_sync || remote_updated_at > last_sync
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    
    #[test]
    fn test_prefer_canvas_strategy() {
        let resolver = ConflictResolver::new(ConflictStrategy::PreferCanvas);
        
        let canvas_topic = Topic {
            id: Uuid::new_v4(),
            title: "Canvas Topic".to_string(),
            content: "Canvas Content".to_string(),
            author_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            views: 10,
            post_count: 5,
            tags: vec!["canvas".to_string()],
            // Add other fields as needed
            ..Default::default()
        };
        
        let discourse_topic = Topic {
            id: Uuid::new_v4(),
            title: "Discourse Topic".to_string(),
            content: "Discourse Content".to_string(),
            author_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            views: 20,
            post_count: 10,
            tags: vec!["discourse".to_string()],
            // Add other fields as needed
            ..Default::default()
        };
        
        let result = resolver.resolve_topic_conflict(&canvas_topic, &discourse_topic).unwrap();
        
        assert_eq!(result.title, "Canvas Topic");
        assert_eq!(result.content, "Canvas Content");
        assert_eq!(result.tags, vec!["canvas"]);
    }
    
    #[test]
    fn test_merge_strategies() {
        let resolver = ConflictResolver::new(ConflictStrategy::MergePreferCanvas);
        
        let canvas_post = Post {
            id: Uuid::new_v4(),
            topic_id: Uuid::new_v4(),
            author_id: Uuid::new_v4(),
            content: "Canvas Content".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            likes: 5,
            // Add other fields as needed
            ..Default::default()
        };
        
        let discourse_post = Post {
            id: Uuid::new_v4(),
            topic_id: Uuid::new_v4(),
            author_id: Uuid::new_v4(),
            content: "Discourse Content".to_string(),
            html_content: Some("<p>Discourse Content</p>".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now() + chrono::Duration::seconds(60), // More recent
            likes: 10,
            // Add other fields as needed
            ..Default::default()
        };
        
        let result = resolver.resolve_post_conflict(&canvas_post, &discourse_post).unwrap();
        
        // Should prefer discourse content because it's more recent
        assert_eq!(result.content, "Discourse Content");
        assert_eq!(result.html_content, Some("<p>Discourse Content</p>".to_string()));
        // Should take the higher like count
        assert_eq!(result.likes, 10);
    }
}
