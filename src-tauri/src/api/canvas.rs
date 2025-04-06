// Add these methods to your Canvas API client

impl CanvasClient {
    // Get a specific discussion
    pub async fn get_discussion(&self, discussion_id: &str) -> Result<CanvasDiscussion, Error> {
        let url = format!("{}/api/v1/discussion_topics/{}", self.base_url, discussion_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to get discussion: HTTP {}", response.status())));
        }
        
        response.json::<CanvasDiscussion>().await
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
    
    // Update a discussion
    pub async fn update_discussion(&self, discussion_id: &str, title: &str, message: &str) -> Result<(), Error> {
        let url = format!("{}/api/v1/discussion_topics/{}", self.base_url, discussion_id);
        
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&serde_json::json!({
                "title": title,
                "message": message
            }))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to update discussion: HTTP {}", response.status())));
        }
        
        Ok(())
    }
    
    // Get discussion entries
    pub async fn get_discussion_entries(&self, discussion_id: &str) -> Result<Vec<CanvasDiscussionEntry>, Error> {
        let url = format!("{}/api/v1/discussion_topics/{}/entries", self.base_url, discussion_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to get discussion entries: HTTP {}", response.status())));
        }
        
        response.json::<Vec<CanvasDiscussionEntry>>().await
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
    
    // Create a new discussion entry
    pub async fn create_discussion_entry(&self, discussion_id: &str, message: &str, user_id: Option<&str>) -> Result<String, Error> {
        let url = format!("{}/api/v1/discussion_topics/{}/entries", self.base_url, discussion_id);
        
        let mut request = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&serde_json::json!({
                "message": message
            }));
            
        // If user_id is provided, add it to the request
        if let Some(uid) = user_id {
            request = request.json(&serde_json::json!({
                "message": message,
                "user_id": uid
            }));
        }
        
        let response = request.send().await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to create discussion entry: HTTP {}", response.status())));
        }
        
        let entry: serde_json::Value = response.json().await?;
        let id = entry["id"].as_str()
            .ok_or_else(|| Error::DeserializationError("Failed to get entry ID".into()))?
            .to_string();
            
        Ok(id)
    }
    
    // Update a discussion entry
    pub async fn update_discussion_entry(&self, discussion_id: &str, entry_id: &str, message: &str) -> Result<(), Error> {
        let url = format!("{}/api/v1/discussion_topics/{}/entries/{}", self.base_url, discussion_id, entry_id);
        
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&serde_json::json!({
                "message": message
            }))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::ApiError(format!("Failed to update discussion entry: HTTP {}", response.status())));
        }
        
        Ok(())
    }
}