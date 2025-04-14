use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{FormData, Headers, Request, RequestInit, RequestMode, Response};
use js_sys::{Array, Uint8Array};
use leptos::*;
use gloo_file::{Blob, File as GlooFile};
use std::collections::HashMap;

use crate::services::api::fetch_with_token;
use crate::models::common::ErrorResponse;
use crate::services::auth::AuthService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub url: Option<String>,
    pub created_at: String,
    pub user_id: String,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentResponse {
    pub attachment: Attachment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentsResponse {
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub file_id: String,
}

/// Handles attachment uploads and management
#[derive(Clone)]
pub struct AttachmentService {
    base_url: String,
}

impl AttachmentService {
    pub fn new() -> Self {
        Self {
            base_url: "/api/attachments".to_string(),
        }
    }
    
    /// Upload a file to the server
    pub async fn upload_file(
        &self,
        file: &web_sys::File,
        context_type: Option<&str>,
        context_id: Option<&str>,
        is_public: bool,
    ) -> Result<UploadResponse, String> {
        let form_data = FormData::new()
            .map_err(|_| "Failed to create FormData".to_string())?;
            
        // Add file to form data
        form_data.append_with_blob_and_filename(
            "file",
            file,
            &file.name(),
        ).map_err(|_| "Failed to append file to FormData".to_string())?;
        
        // Add context information if provided
        if let Some(context_type) = context_type {
            form_data.append_with_str("contextType", context_type)
                .map_err(|_| "Failed to append contextType".to_string())?;
        }
        
        if let Some(context_id) = context_id {
            form_data.append_with_str("contextId", context_id)
                .map_err(|_| "Failed to append contextId".to_string())?;
        }
        
        // Add visibility flag
        form_data.append_with_str("isPublic", &is_public.to_string())
            .map_err(|_| "Failed to append isPublic".to_string())?;
        
        // Create request
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&form_data));
        
        // Send request
        let url = &format!("{}/upload", self.base_url);
        let response = fetch_with_token(url, &opts).await
            .map_err(|e| format!("Failed to upload file: {}", e))?;
        
        if !response.ok() {
            let error_text = response.text().await
                .map_err(|_| "Failed to get error response text".to_string())?;
                
            // Try to parse as JSON error
            match serde_json::from_str::<ErrorResponse>(&error_text) {
                Ok(error) => return Err(error.message),
                Err(_) => return Err(format!("Upload failed: {}", error_text)),
            }
        }
        
        // Parse response
        let result = response.json::<UploadResponse>().await
            .map_err(|_| "Failed to parse upload response".to_string())?;
            
        Ok(result)
    }
    
    /// Upload multiple files
    pub async fn upload_files(
        &self,
        files: Vec<&web_sys::File>,
        context_type: Option<&str>,
        context_id: Option<&str>,
        is_public: bool,
    ) -> Result<Vec<UploadResponse>, String> {
        let mut results = Vec::new();
        
        for file in files {
            let result = self.upload_file(file, context_type, context_id, is_public).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Upload file from base64 data
    pub async fn upload_from_base64(
        &self,
        base64_data: &str,
        filename: &str,
        content_type: &str,
        context_type: Option<&str>,
        context_id: Option<&str>,
        is_public: bool,
    ) -> Result<UploadResponse, String> {
        // Create request data
        let mut data = HashMap::new();
        data.insert("base64Content", base64_data);
        data.insert("filename", filename);
        data.insert("contentType", content_type);
        
        if let Some(context_type) = context_type {
            data.insert("contextType", context_type);
        }
        
        if let Some(context_id) = context_id {
            data.insert("contextId", context_id);
        }
        
        data.insert("isPublic", &is_public.to_string());
        
        // Create request
        let mut opts = RequestInit::new();
        opts.method("POST");
        
        let body = serde_json::to_string(&data)
            .map_err(|_| "Failed to serialize request body".to_string())?;
            
        opts.body(Some(&JsValue::from_str(&body)));
        
        // Add JSON content type header
        let headers = Headers::new().unwrap();
        headers.append("Content-Type", "application/json").unwrap();
        opts.headers(&headers);
        
        // Send request
        let url = &format!("{}/upload-base64", self.base_url);
        let response = fetch_with_token(url, &opts).await
            .map_err(|e| format!("Failed to upload base64 file: {}", e))?;
        
        if !response.ok() {
            let error_text = response.text().await
                .map_err(|_| "Failed to get error response text".to_string())?;
            return Err(format!("Upload failed: {}", error_text));
        }
        
        // Parse response
        let result = response.json::<UploadResponse>().await
            .map_err(|_| "Failed to parse upload response".to_string())?;
            
        Ok(result)
    }
    
    /// Create an attachment reference to an entity
    pub async fn create_attachment(
        &self,
        file_id: &str,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Attachment, String> {
        let request = AttachmentRequest {
            file_id: file_id.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
        };
        
        // Create request
        let mut opts = RequestInit::new();
        opts.method("POST");
        
        let body = serde_json::to_string(&request)
            .map_err(|_| "Failed to serialize request body".to_string())?;
            
        opts.body(Some(&JsValue::from_str(&body)));
        
        // Add JSON content type header
        let headers = Headers::new().unwrap();
        headers.append("Content-Type", "application/json").unwrap();
        opts.headers(&headers);
        
        // Send request
        let url = &format!("{}/reference", self.base_url);
        let response = fetch_with_token(url, &opts).await
            .map_err(|e| format!("Failed to create attachment reference: {}", e))?;
        
        if !response.ok() {
            let error_text = response.text().await
                .map_err(|_| "Failed to get error response text".to_string())?;
            return Err(format!("Create attachment reference failed: {}", error_text));
        }
        
        // Parse response
        let result = response.json::<AttachmentResponse>().await
            .map_err(|_| "Failed to parse attachment response".to_string())?;
            
        Ok(result.attachment)
    }
    
    /// Get attachments for an entity
    pub async fn get_attachments(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Vec<Attachment>, String> {
        // Create request
        let mut opts = RequestInit::new();
        opts.method("GET");
        
        // Send request
        let url = &format!("{}/entity/{}/{}", self.base_url, entity_type, entity_id);
        let response = fetch_with_token(url, &opts).await
            .map_err(|e| format!("Failed to get attachments: {}", e))?;
        
        if !response.ok() {
            let error_text = response.text().await
                .map_err(|_| "Failed to get error response text".to_string())?;
            return Err(format!("Get attachments failed: {}", error_text));
        }
        
        // Parse response
        let result = response.json::<AttachmentsResponse>().await
            .map_err(|_| "Failed to parse attachments response".to_string())?;
            
        Ok(result.attachments)
    }
    
    /// Delete an attachment reference
    pub async fn delete_attachment(
        &self,
        entity_type: &str,
        entity_id: &str,
        file_id: &str,
    ) -> Result<(), String> {
        // Create request
        let mut opts = RequestInit::new();
        opts.method("DELETE");
        
        // Send request
        let url = &format!("{}/reference/{}/{}/{}", self.base_url, entity_type, entity_id, file_id);
        let response = fetch_with_token(url, &opts).await
            .map_err(|e| format!("Failed to delete attachment reference: {}", e))?;
        
        if !response.ok() {
            let error_text = response.text().await
                .map_err(|_| "Failed to get error response text".to_string())?;
            return Err(format!("Delete attachment reference failed: {}", error_text));
        }
        
        Ok(())
    }
    
    /// Delete a file completely
    pub async fn delete_file(&self, file_id: &str) -> Result<(), String> {
        // Create request
        let mut opts = RequestInit::new();
        opts.method("DELETE");
        
        // Send request
        let url = &format!("{}/{}", self.base_url, file_id);
        let response = fetch_with_token(url, &opts).await
            .map_err(|e| format!("Failed to delete file: {}", e))?;
        
        if !response.ok() {
            let error_text = response.text().await
                .map_err(|_| "Failed to get error response text".to_string())?;
            return Err(format!("Delete file failed: {}", error_text));
        }
        
        Ok(())
    }
    
    /// List files by user
    pub async fn list_files_by_user(&self, limit: usize, offset: usize) -> Result<Vec<Attachment>, String> {
        // Create request
        let mut opts = RequestInit::new();
        opts.method("GET");
        
        // Send request
        let url = &format!("{}/user?limit={}&offset={}", self.base_url, limit, offset);
        let response = fetch_with_token(url, &opts).await
            .map_err(|e| format!("Failed to list files: {}", e))?;
        
        if !response.ok() {
            let error_text = response.text().await
                .map_err(|_| "Failed to get error response text".to_string())?;
            return Err(format!("List files failed: {}", error_text));
        }
        
        // Parse response
        let result = response.json::<AttachmentsResponse>().await
            .map_err(|_| "Failed to parse attachments response".to_string())?;
            
        Ok(result.attachments)
    }
}
