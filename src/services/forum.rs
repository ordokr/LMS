use crate::models::forum::{Category, Topic, Post};
use reqwest::Client;

pub struct ForumService;

impl ForumService {
    // Category methods
    pub async fn get_category(id: i64) -> Result<Category, String> {
        // For testing, return a placeholder
        // In production, implement the API call
        Ok(Category {
            id: id.to_string(),
            name: "Test Category".to_string(),
            description: Some("This is a test category".to_string()),
            ..Default::default()
        })
        
        // Production code would be:
        /*
        let response = match reqwest::get(&format!("/api/forum/categories/{}", id)).await {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Network error: {}", e)),
        };
        
        if response.status().is_success() {
            match response.json::<Category>().await {
                Ok(category) => Ok(category),
                Err(e) => Err(format!("Failed to parse category: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Keep this method for backward compatibility
    pub async fn create_category(name: &str, description: &str) -> Result<Category, String> {
        // Create a Category object and call the other method
        let category = Category {
            id: "".to_string(), // Will be assigned by server
            name: name.to_string(),
            description: Some(description.to_string()),
            ..Default::default()
        };
        
        Self::create_category_obj(&category).await
    }
    
    // Rename your second create_category method to avoid duplicate definition
    pub async fn create_category_obj(category: &Category) -> Result<Category, String> {
        // For testing, return a placeholder
        // In production, implement the API call
        Ok(Category {
            id: "new-category-id".to_string(),
            name: category.name.clone(),
            description: category.description.clone(),
            ..Default::default()
        })
        
        // Production code would be:
        /*
        let client = Client::new();
        let response = match client.post("/api/forum/categories")
            .json(category)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Network error: {}", e)),
            };
        
        if response.status().is_success() {
            match response.json::<Category>().await {
                Ok(created) => Ok(created),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Keep this method for backward compatibility
    pub async fn update_category(id: &str, name: &str, description: &str) -> Result<Category, String> {
        // Create a Category object and call the other method
        let category = Category {
            id: id.to_string(),
            name: name.to_string(),
            description: Some(description.to_string()),
            ..Default::default()
        };
        
        Self::update_category_obj(&category).await
    }
    
    // Rename your second update_category method to avoid duplicate definition
    pub async fn update_category_obj(category: &Category) -> Result<Category, String> {
        // For testing, return a placeholder
        // In production, implement the API call
        Ok(Category {
            id: category.id.clone(),
            name: category.name.clone(),
            description: category.description.clone(),
            ..Default::default()
        })
        
        // Production code would be:
        /*
        let client = Client::new();
        let response = match client.put(&format!("/api/forum/categories/{}", category.id))
            .json(category)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Network error: {}", e)),
            };
        
        if response.status().is_success() {
            match response.json::<Category>().await {
                Ok(updated) => Ok(updated),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Topic methods
    pub async fn get_topics(category_id: Option<i64>) -> Result<Vec<Topic>, String> {
        // For testing, return placeholders
        // In production, implement the API call
        let cat_id = category_id.map(|id| id.to_string()).unwrap_or_else(|| "all".to_string());
        Ok(vec![
            Topic {
                id: format!("topic-1-{}", cat_id),
                title: format!("Test Topic 1 for {}", cat_id),
                category_id: cat_id.clone(),
                author_id: "test-user".to_string(),
                created_at: chrono::Utc::now(),
                ..Default::default()
            },
            Topic {
                id: format!("topic-2-{}", cat_id),
                title: format!("Test Topic 2 for {}", cat_id),
                category_id: cat_id,
                author_id: "test-user".to_string(),
                created_at: chrono::Utc::now(),
                ..Default::default()
            }
        ])
        
        // Production code would be:
        /*
        let url = match category_id {
            Some(id) => format!("/api/forum/categories/{}/topics", id),
            None => "/api/forum/topics".to_string()
        };
        
        let response = match reqwest::get(&url).await {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Network error: {}", e)),
        };
        
        if response.status().is_success() {
            match response.json::<Vec<Topic>>().await {
                Ok(topics) => Ok(topics),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Update to accept i64 instead of &str
    pub async fn get_topic(id: i64) -> Result<Topic, String> {
        // For testing, return a placeholder
        // In production, implement the API call
        Ok(Topic {
            id: id.to_string(),
            title: "Test Topic".to_string(),
            category_id: "test-category".to_string(),
            author_id: "test-user".to_string(),
            created_at: chrono::Utc::now(),
            // Add new fields with default values
            pinned: false,
            locked: false,
            ..Default::default()
        })
        
        // Production code would be:
        /*
        let response = match reqwest::get(&format!("/api/forum/topics/{}", id)).await {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Network error: {}", e)),
        };
        
        if response.status().is_success() {
            match response.json::<Topic>().await {
                Ok(topic) => Ok(topic),
                Err(e) => Err(format!("Failed to parse topic: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Keep this method for backward compatibility
    pub async fn create_topic(category_id: &str, title: &str, content: &str) -> Result<Topic, String> {
        // Convert string ID to i64 and call the enhanced method
        match category_id.parse::<i64>() {
            Ok(id) => Self::create_topic_enhanced(id, title, "", false, false, content).await,
            Err(_) => Err("Invalid category ID format".to_string())
        }
    }
    
    // Add the enhanced method
    pub async fn create_topic_enhanced(
        category_id: i64, 
        title: &str, 
        slug: &str,
        pinned: bool,
        locked: bool,
        content: &str
    ) -> Result<Topic, String> {
        // For testing, return a placeholder
        // In production, implement the API call
        Ok(Topic {
            id: "new-topic-id".to_string(),
            title: title.to_string(),
            category_id: category_id.to_string(),
            author_id: "current-user".to_string(),
            created_at: chrono::Utc::now(),
            pinned,
            locked,
            ..Default::default()
        })
        
        // Production code would be:
        /*
        let client = Client::new();
        
        let request = serde_json::json!({
            "category_id": category_id,
            "title": title,
            "slug": slug,
            "pinned": pinned,
            "locked": locked,
            "content": content
        });
        
        let response = match client.post("/api/forum/topics")
            .json(&request)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Network error: {}", e)),
            };
        
        if response.status().is_success() {
            match response.json::<Topic>().await {
                Ok(topic) => Ok(topic),
                Err(e) => Err(format!("Failed to parse topic response: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Keep this method for backward compatibility
    pub async fn update_topic(id: &str, title: &str, content: &str) -> Result<Topic, String> {
        // Convert string ID to i64 and call the enhanced method
        match id.parse::<i64>() {
            Ok(id) => Self::update_topic_enhanced(id, title, "", false, false, content).await,
            Err(_) => Err("Invalid topic ID format".to_string())
        }
    }
    
    // Add the enhanced method
    pub async fn update_topic_enhanced(
        id: i64, 
        title: &str, 
        slug: &str,
        pinned: bool,
        locked: bool,
        content: &str
    ) -> Result<Topic, String> {
        // For testing, return a placeholder
        // In production, implement the API call
        Ok(Topic {
            id: id.to_string(),
            title: title.to_string(),
            category_id: "test-category".to_string(),
            author_id: "test-user".to_string(),
            created_at: chrono::Utc::now(),
            pinned,
            locked,
            ..Default::default()
        })
        
        // Production code would be:
        /*
        let client = Client::new();
        
        let request = serde_json::json!({
            "title": title,
            "slug": slug,
            "pinned": pinned,
            "locked": locked,
            "content": content
        });
        
        let response = match client.put(&format!("/api/forum/topics/{}", id))
            .json(&request)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Network error: {}", e)),
            };
        
        if response.status().is_success() {
            match response.json::<Topic>().await {
                Ok(topic) => Ok(topic),
                Err(e) => Err(format!("Failed to parse topic response: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Add this helper method to get posts for a topic
    pub async fn get_topic_posts(topic_id: i64) -> Result<Vec<Post>, String> {
        // For testing, return placeholder posts
        // In production, implement the API call
        Ok(vec![
            Post {
                id: format!("post-1-{}", topic_id),
                topic_id: topic_id.to_string(),
                content: "This is the first post in this topic".to_string(),
                author_id: "test-user".to_string(),
                created_at: chrono::Utc::now(),
                ..Default::default()
            },
            Post {
                id: format!("post-2-{}", topic_id),
                topic_id: topic_id.to_string(),
                content: "This is a reply to the first post".to_string(),
                author_id: "another-user".to_string(),
                created_at: chrono::Utc::now(),
                ..Default::default()
            }
        ])
        
        // Production code would be:
        /*
        let response = match reqwest::get(&format!("/api/forum/topics/{}/posts", topic_id)).await {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Network error: {}", e)),
        };
        
        if response.status().is_success() {
            match response.json::<Vec<Post>>().await {
                Ok(posts) => Ok(posts),
                Err(e) => Err(format!("Failed to parse posts: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }
    
    // Add this helper method to get the first post of a topic
    pub async fn get_topic_first_post(topic_id: i64) -> Result<Post, String> {
        // For testing, just return the first post from get_topic_posts
        let posts = Self::get_topic_posts(topic_id).await?;
        posts.first().cloned().ok_or_else(|| "No posts found for this topic".to_string())
    }
    
    // Add method to create a reply post
    pub async fn create_post(topic_id: i64, content: &str) -> Result<Post, String> {
        // For testing, return a placeholder
        // In production, implement the API call
        Ok(Post {
            id: "new-post-id".to_string(),
            topic_id: topic_id.to_string(),
            content: content.to_string(),
            author_id: "current-user".to_string(),
            created_at: chrono::Utc::now(),
            ..Default::default()
        })
        
        // Production code would be:
        /*
        let client = Client::new();
        
        let request = serde_json::json!({
            "topic_id": topic_id,
            "content": content
        });
        
        let response = match client.post("/api/forum/posts")
            .json(&request)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Network error: {}", e)),
            };
        
        if response.status().is_success() {
            match response.json::<Post>().await {
                Ok(post) => Ok(post),
                Err(e) => Err(format!("Failed to parse post response: {}", e)),
            }
        } else {
            Err(format!("API error: {}", response.status()))
        }
        */
    }

    /// Search topics, posts, and users
    pub async fn search(query: &str) -> Result<Vec<SearchResult>, String> {
        // Prepare the search query
        let search_query = query.trim();
        if search_query.is_empty() {
            return Ok(vec![]);
        }

        let url = format!("/api/forum/search?q={}", urlencoding::encode(search_query));
        
        match Self::fetch_json::<Vec<SearchResultDto>>(&url).await {
            Ok(results_dto) => {
                // Convert DTOs to domain models
                let results = results_dto.into_iter().map(|dto| match dto {
                    SearchResultDto::Topic(topic) => {
                        SearchResult::Topic(TopicSearchResult {
                            id: topic.id,
                            title: topic.title,
                            excerpt: topic.excerpt,
                            created_at: topic.created_at,
                            author_id: topic.author_id,
                            author_name: topic.author_name,
                            category_id: topic.category_id,
                            category_name: topic.category_name,
                            reply_count: topic.reply_count,
                        })
                    },
                    SearchResultDto::Post(post) => {
                        SearchResult::Post(PostSearchResult {
                            id: post.id,
                            content: post.content,
                            excerpt: post.excerpt,
                            created_at: post.created_at,
                            author_id: post.author_id,
                            author_name: post.author_name,
                            topic_id: post.topic_id,
                            topic_title: post.topic_title,
                        })
                    },
                    SearchResultDto::User(user) => {
                        SearchResult::User(UserSearchResult {
                            id: user.id,
                            name: user.name,
                            avatar_url: user.avatar_url,
                            bio: user.bio,
                            created_at: user.created_at,
                            topic_count: user.topic_count,
                            post_count: user.post_count,
                        })
                    }
                }).collect();
                
                Ok(results)
            },
            Err(e) => Err(format!("Failed to search: {}", e))
        }
    }

    // Get topics created by a user
    pub async fn get_user_topics(user_id: i64, limit: i64) -> Result<Vec<Topic>, String> {
        let url = format!("/api/users/{}/topics?limit={}", user_id, limit);
        Self::fetch_json::<Vec<Topic>>(&url).await
    }
    
    // Get posts created by a user
    pub async fn get_user_posts(user_id: i64, limit: i64) -> Result<Vec<Post>, String> {
        let url = format!("/api/users/{}/posts?limit={}", user_id, limit);
        Self::fetch_json::<Vec<Post>>(&url).await
    }

    // Create a mention notification
    pub async fn create_mention_notification(topic_id: i64, post_id: i64, username: &str) -> Result<(), String> {
        let url = "/api/notifications/mention";
        let payload = serde_json::json!({
            "topic_id": topic_id,
            "post_id": post_id,
            "username": username
        });
        
        Self::post_json(&url, &payload).await
    }
    
    // Create a quote notification
    pub async fn create_quote_notification(topic_id: i64, post_id: i64, quoted_post_id: i64) -> Result<(), String> {
        let url = "/api/notifications/quote";
        let payload = serde_json::json!({
            "topic_id": topic_id,
            "post_id": post_id,
            "quoted_post_id": quoted_post_id
        });
        
        Self::post_json(&url, &payload).await
    }
    
    // Helper method for POST requests with JSON payload
    async fn post_json<T>(url: &str, payload: &T) -> Result<(), String>
    where
        T: serde::Serialize,
    {
        match reqwest::Client::new()
            .post(url)
            .json(payload)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("API error: {}", response.status()))
                }
            },
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    // Get all tags
    pub async fn get_tags() -> Result<Vec<Tag>, String> {
        let url = "/api/forum/tags";
        Self::fetch_json::<Vec<Tag>>(&url).await
    }
    
    // Get popular tags
    pub async fn get_popular_tags(limit: usize) -> Result<Vec<Tag>, String> {
        let url = format!("/api/forum/tags/popular?limit={}", limit);
        Self::fetch_json::<Vec<Tag>>(&url).await
    }
    
    // Get tag by ID
    pub async fn get_tag(id: i64) -> Result<Tag, String> {
        let url = format!("/api/forum/tags/{}", id);
        Self::fetch_json::<Tag>(&url).await
    }
    
    // Get tag by slug
    pub async fn get_tag_by_slug(slug: &str) -> Result<Tag, String> {
        let url = format!("/api/forum/tags/slug/{}", slug);
        Self::fetch_json::<Tag>(&url).await
    }
    
    // Get topics by tag
    pub async fn get_topics_by_tag(tag_slug: &str, page: usize, per_page: usize) -> Result<PaginatedTopics, String> {
        let url = format!("/api/forum/tags/{}/topics?page={}&per_page={}", tag_slug, page, per_page);
        Self::fetch_json::<PaginatedTopics>(&url).await
    }
    
    // Create a new tag (admin only)
    pub async fn create_tag(request: &CreateTagRequest) -> Result<Tag, String> {
        let url = "/api/forum/tags";
        Self::fetch_with_json::<Tag, CreateTagRequest>("POST", &url, request).await
    }
    
    // Update a tag (admin only)
    pub async fn update_tag(id: i64, request: &UpdateTagRequest) -> Result<Tag, String> {
        let url = format!("/api/forum/tags/{}", id);
        Self::fetch_with_json::<Tag, UpdateTagRequest>("PUT", &url, request).await
    }
    
    // Delete a tag (admin only)
    pub async fn delete_tag(id: i64) -> Result<(), String> {
        let url = format!("/api/forum/tags/{}", id);
        Self::fetch_empty("DELETE", &url).await
    }
}

// DTOs for search results (could be in a separate module)
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SearchResultDto {
    #[serde(rename = "topic")]
    Topic(TopicSearchResultDto),
    #[serde(rename = "post")]
    Post(PostSearchResultDto),
    #[serde(rename = "user")]
    User(UserSearchResultDto),
}

#[derive(Debug, Clone, Deserialize)]
pub struct TopicSearchResultDto {
    pub id: i64,
    pub title: String,
    pub excerpt: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub reply_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostSearchResultDto {
    pub id: i64,
    pub content: String,
    pub excerpt: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub topic_id: i64,
    pub topic_title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserSearchResultDto {
    pub id: i64,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub topic_count: Option<i64>,
    pub post_count: Option<i64>,
}