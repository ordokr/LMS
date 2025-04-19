// cmi5 Client for communicating with the LRS
//
// This module provides a client for sending xAPI statements to a Learning Record Store (LRS)
// according to the cmi5 specification.

use anyhow::{Result, anyhow};
use reqwest::{Client, header};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{info, error, debug};
use base64::{encode};
use crate::quiz::cmi5::statements::Cmi5Statement;

/// Client for communicating with the LRS
pub struct Cmi5Client {
    /// HTTP client
    client: Client,
    
    /// LRS endpoint
    endpoint: String,
    
    /// Authorization token
    auth_token: Option<String>,
}

/// Response from the LRS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LrsResponse {
    /// Response status
    pub status: u16,
    
    /// Response body
    pub body: Option<Value>,
}

impl Cmi5Client {
    /// Create a new cmi5 client
    pub fn new(endpoint: &str, auth_token: Option<&str>) -> Result<Self> {
        // Create a new HTTP client
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        
        // Add authorization header if provided
        if let Some(token) = auth_token {
            let auth_value = format!("Basic {}", encode(token));
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&auth_value)
                    .map_err(|e| anyhow!("Invalid authorization token: {}", e))?,
            );
        }
        
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;
        
        Ok(Self {
            client,
            endpoint: endpoint.to_string(),
            auth_token: auth_token.map(String::from),
        })
    }
    
    /// Send a statement to the LRS
    pub async fn send_statement(&self, statement: &Cmi5Statement) -> Result<String> {
        debug!("Sending statement to LRS: {:?}", statement);
        
        // Serialize the statement
        let statement_json = serde_json::to_string(statement)
            .map_err(|e| anyhow!("Failed to serialize statement: {}", e))?;
        
        // Send the statement to the LRS
        let statements_endpoint = format!("{}/statements", self.endpoint);
        let response = self.client.post(&statements_endpoint)
            .body(statement_json)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send statement: {}", e))?;
        
        // Check the response
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            
            error!("LRS returned error: {} - {}", status, body);
            return Err(anyhow!("LRS returned error: {} - {}", status, body));
        }
        
        // Parse the response
        let response_body: Value = response.json().await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;
        
        // Extract the statement ID
        let statement_id = response_body.as_array()
            .and_then(|arr| arr.first())
            .and_then(|id| id.as_str())
            .ok_or_else(|| anyhow!("Failed to extract statement ID from response"))?
            .to_string();
        
        debug!("Statement sent successfully with ID: {}", statement_id);
        Ok(statement_id)
    }
    
    /// Get a statement from the LRS
    pub async fn get_statement(&self, statement_id: &str) -> Result<Cmi5Statement> {
        debug!("Getting statement from LRS: {}", statement_id);
        
        // Get the statement from the LRS
        let statement_endpoint = format!("{}/statements?statementId={}", self.endpoint, statement_id);
        let response = self.client.get(&statement_endpoint)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get statement: {}", e))?;
        
        // Check the response
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            
            error!("LRS returned error: {} - {}", status, body);
            return Err(anyhow!("LRS returned error: {} - {}", status, body));
        }
        
        // Parse the response
        let statement: Cmi5Statement = response.json().await
            .map_err(|e| anyhow!("Failed to parse statement: {}", e))?;
        
        debug!("Statement retrieved successfully: {:?}", statement);
        Ok(statement)
    }
    
    /// Get statements from the LRS
    pub async fn get_statements(
        &self,
        actor_id: Option<&str>,
        activity_id: Option<&str>,
        registration_id: Option<&str>,
        since: Option<&str>,
        until: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Cmi5Statement>> {
        debug!("Getting statements from LRS");
        
        // Build the query parameters
        let mut params = Vec::new();
        
        if let Some(actor_id) = actor_id {
            params.push(format!("agent={}", urlencoding::encode(&format!("{{\"mbox\":\"mailto:{}\"}}", actor_id))));
        }
        
        if let Some(activity_id) = activity_id {
            params.push(format!("activity={}", urlencoding::encode(activity_id)));
        }
        
        if let Some(registration_id) = registration_id {
            params.push(format!("registration={}", registration_id));
        }
        
        if let Some(since) = since {
            params.push(format!("since={}", urlencoding::encode(since)));
        }
        
        if let Some(until) = until {
            params.push(format!("until={}", urlencoding::encode(until)));
        }
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        // Build the URL
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        let statements_endpoint = format!("{}/statements{}", self.endpoint, query_string);
        
        // Get the statements from the LRS
        let response = self.client.get(&statements_endpoint)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get statements: {}", e))?;
        
        // Check the response
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            
            error!("LRS returned error: {} - {}", status, body);
            return Err(anyhow!("LRS returned error: {} - {}", status, body));
        }
        
        // Parse the response
        let response_body: Value = response.json().await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;
        
        // Extract the statements
        let statements = response_body.get("statements")
            .and_then(|s| s.as_array())
            .ok_or_else(|| anyhow!("Failed to extract statements from response"))?;
        
        // Parse the statements
        let mut result = Vec::new();
        for statement_value in statements {
            let statement: Cmi5Statement = serde_json::from_value(statement_value.clone())
                .map_err(|e| anyhow!("Failed to parse statement: {}", e))?;
            
            result.push(statement);
        }
        
        debug!("Retrieved {} statements", result.len());
        Ok(result)
    }
    
    /// Get state from the LRS
    pub async fn get_state(
        &self,
        activity_id: &str,
        actor_id: &str,
        state_id: &str,
        registration_id: Option<&str>,
    ) -> Result<Value> {
        debug!("Getting state from LRS: {}", state_id);
        
        // Build the URL
        let mut params = vec![
            format!("activityId={}", urlencoding::encode(activity_id)),
            format!("agent={}", urlencoding::encode(&format!("{{\"mbox\":\"mailto:{}\"}}", actor_id))),
            format!("stateId={}", urlencoding::encode(state_id)),
        ];
        
        if let Some(registration_id) = registration_id {
            params.push(format!("registration={}", registration_id));
        }
        
        let state_endpoint = format!("{}/activities/state?{}", self.endpoint, params.join("&"));
        
        // Get the state from the LRS
        let response = self.client.get(&state_endpoint)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get state: {}", e))?;
        
        // Check the response
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(Value::Null);
        }
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            
            error!("LRS returned error: {} - {}", status, body);
            return Err(anyhow!("LRS returned error: {} - {}", status, body));
        }
        
        // Parse the response
        let state: Value = response.json().await
            .map_err(|e| anyhow!("Failed to parse state: {}", e))?;
        
        debug!("State retrieved successfully: {:?}", state);
        Ok(state)
    }
    
    /// Set state in the LRS
    pub async fn set_state(
        &self,
        activity_id: &str,
        actor_id: &str,
        state_id: &str,
        state: &Value,
        registration_id: Option<&str>,
    ) -> Result<()> {
        debug!("Setting state in LRS: {}", state_id);
        
        // Build the URL
        let mut params = vec![
            format!("activityId={}", urlencoding::encode(activity_id)),
            format!("agent={}", urlencoding::encode(&format!("{{\"mbox\":\"mailto:{}\"}}", actor_id))),
            format!("stateId={}", urlencoding::encode(state_id)),
        ];
        
        if let Some(registration_id) = registration_id {
            params.push(format!("registration={}", registration_id));
        }
        
        let state_endpoint = format!("{}/activities/state?{}", self.endpoint, params.join("&"));
        
        // Serialize the state
        let state_json = serde_json::to_string(state)
            .map_err(|e| anyhow!("Failed to serialize state: {}", e))?;
        
        // Set the state in the LRS
        let response = self.client.put(&state_endpoint)
            .body(state_json)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to set state: {}", e))?;
        
        // Check the response
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            
            error!("LRS returned error: {} - {}", status, body);
            return Err(anyhow!("LRS returned error: {} - {}", status, body));
        }
        
        debug!("State set successfully");
        Ok(())
    }
    
    /// Delete state from the LRS
    pub async fn delete_state(
        &self,
        activity_id: &str,
        actor_id: &str,
        state_id: &str,
        registration_id: Option<&str>,
    ) -> Result<()> {
        debug!("Deleting state from LRS: {}", state_id);
        
        // Build the URL
        let mut params = vec![
            format!("activityId={}", urlencoding::encode(activity_id)),
            format!("agent={}", urlencoding::encode(&format!("{{\"mbox\":\"mailto:{}\"}}", actor_id))),
            format!("stateId={}", urlencoding::encode(state_id)),
        ];
        
        if let Some(registration_id) = registration_id {
            params.push(format!("registration={}", registration_id));
        }
        
        let state_endpoint = format!("{}/activities/state?{}", self.endpoint, params.join("&"));
        
        // Delete the state from the LRS
        let response = self.client.delete(&state_endpoint)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to delete state: {}", e))?;
        
        // Check the response
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            
            error!("LRS returned error: {} - {}", status, body);
            return Err(anyhow!("LRS returned error: {} - {}", status, body));
        }
        
        debug!("State deleted successfully");
        Ok(())
    }
}
