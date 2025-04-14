// Add these methods to your Discourse API client

impl DiscourseClient {
    // Get a topic
    pub async fn get_topic(&self, topic_id: &str) -> Result<DiscourseTopic, Error> {
        let url = format!("{}/t/{}.json", self.base_url, topic_id);
        
        let response = self.client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to get topic: HTTP {}", response.status())));
        }
        
        response.json::<DiscourseTopic>().await
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
    
    // Get a topic with posts
    pub async fn get_topic_with_posts(&self, topic_id: &str) -> Result<DiscourseTopicWithPosts, Error> {
        let url = format!("{}/t/{}.json", self.base_url, topic_id);
        
        let response = self.client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to get topic: HTTP {}", response.status())));
        }
        
        response.json::<DiscourseTopicWithPosts>().await
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
    
    // Update a topic
    pub async fn update_topic(&self, topic_id: &str, title: &str, content: &str) -> Result<(), Error> {
        let url = format!("{}/t/{}.json", self.base_url, topic_id);
        
        let response = self.client
            .put(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .json(&serde_json::json!({
                "title": title,
                "raw": content
            }))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to update topic: HTTP {}", response.status())));
        }
        
        Ok(())
    }
    
    // Get topic posts
    pub async fn get_topic_posts(&self, topic_id: &str) -> Result<Vec<DiscoursePost>, Error> {
        let url = format!("{}/t/{}/posts.json", self.base_url, topic_id);
        
        let response = self.client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to get topic posts: HTTP {}", response.status())));
        }
        
        let json: serde_json::Value = response.json().await?;
        let posts = json["post_stream"]["posts"].as_array()
            .ok_or_else(|| Error::DeserializationError("Failed to get posts array".into()))?;
            
        let mut result = Vec::new();
        for post in posts {
            let id = post["id"].as_str().unwrap_or_default().to_string();
            let topic_id = post["topic_id"].as_str().unwrap_or_default().to_string();
            let user_id = post["user_id"].as_str().map(|s| s.to_string());
            let content = post["raw"].as_str().unwrap_or_default().to_string();
            let created_at = chrono::DateTime::parse_from_rfc3339(
                post["created_at"].as_str().unwrap_or_default()
            ).unwrap_or_default().with_timezone(&Utc);
            let updated_at = chrono::DateTime::parse_from_rfc3339(
                post["updated_at"].as_str().unwrap_or_default()
            ).unwrap_or_default().with_timezone(&Utc);
            
            // Extract external_id from custom fields if present
            let external_id = post["custom_fields"]["canvas_entry_id"]
                .as_str().map(|s| s.to_string());
            
            result.push(DiscoursePost {
                id,
                topic_id,
                user_id,
                content,
                external_id,
                created_at,
                updated_at,
            });
        }
        
        Ok(result)
    }
    
    // Create a new topic
    pub async fn create_topic(&self, title: &str, content: &str, category_id: Option<&str>) -> Result<DiscourseTopic, Error> {
        let url = format!("{}/posts.json", self.base_url);

        let mut json_payload = serde_json::json!({
            "title": title,
            "raw": content
        });

        if let Some(category) = category_id {
            json_payload["category"] = serde_json::json!(category);
        }

        let response = self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .json(&json_payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to create topic: HTTP {}", response.status())));
        }

        response.json::<DiscourseTopic>().await
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }

    // Create a new post
    pub async fn create_post(&self, topic_id: &str, content: &str) -> Result<DiscoursePost, Error> {
        let url = format!("{}/posts.json", self.base_url);

        let json_payload = serde_json::json!({
            "topic_id": topic_id,
            "raw": content
        });

        let response = self.client
            .post(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .json(&json_payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to create post: HTTP {}", response.status())));
        }

        response.json::<DiscoursePost>().await
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }

    // Get all categories
    pub async fn get_categories(&self) -> Result<Vec<DiscourseCategory>, Error> {
        let url = format!("{}/categories.json", self.base_url);

        let response = self.client
            .get(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to get categories: HTTP {}", response.status())));
        }

        response.json::<Vec<DiscourseCategory>>().await
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
    
    // Update a post
    pub async fn update_post(&self, post_id: &str, content: &str) -> Result<(), Error> {
        let url = format!("{}/posts/{}.json", self.base_url, post_id);
        
        let response = self.client
            .put(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .json(&serde_json::json!({
                "raw": content
            }))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to update post: HTTP {}", response.status())));
        }
        
        Ok(())
    }
    
    // Update a post's external ID (Canvas entry ID)
    pub async fn update_post_external_id(&self, post_id: &str, external_id: &str) -> Result<(), Error> {
        let url = format!("{}/posts/{}.json", self.base_url, post_id);
        
        let response = self.client
            .put(&url)
            .header("Api-Key", &self.api_key)
            .header("Api-Username", &self.api_username)
            .json(&serde_json::json!({
                "custom_fields": {
                    "canvas_entry_id": external_id
                }
            }))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to update post external ID: HTTP {}", response.status())));
        }
        
        Ok(())
    }
}