use super::jwt;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use std::env;

/// SSO payload for Discourse authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoursePayload {
    pub external_id: String, // Canvas user ID
    pub email: String,
    pub username: Option<String>,
    pub name: Option<String>,
    pub admin: Option<bool>,
    pub moderator: Option<bool>,
    pub groups: Option<Vec<String>>,
    pub return_sso_url: Option<String>,
}

/// Generate SSO payload for Discourse authentication
pub fn generate_discourse_sso(user: &User, return_url: Option<&str>) -> Result<String, String> {
    let payload = DiscoursePayload {
        external_id: format!("canvas_{}", user.canvas_id),
        email: user.email.clone(),
        username: Some(user.username.clone()),
        name: Some(user.display_name.clone().unwrap_or_else(|| user.username.clone())),
        admin: if user.role == "admin" { Some(true) } else { Some(false) },
        moderator: if user.role == "teacher" { Some(true) } else { Some(false) },
        groups: Some(vec![user.role.clone()]),
        return_sso_url: return_url.map(String::from),
    };
    
    // Serialize and encode payload
    let payload_json = serde_json::to_string(&payload)
        .map_err(|e| format!("Failed to serialize payload: {}", e))?;
    
    let payload_base64 = base64::encode(payload_json);
    
    Ok(payload_base64)
}

/// Generate JWT token for Canvas-Discourse SSO
pub fn generate_sso_token(user: &User, return_url: Option<&str>) -> Result<String, String> {
    jwt::generate_token(
        &user.id, 
        &user.role, 
        &user.canvas_id,
        user.discourse_id.as_deref(),
        Some(&user.email),
        user.display_name.as_deref()
    ).map_err(|e| format!("Failed to generate SSO token: {}", e))
}

/// Generate SSO URL for Discourse authentication
pub fn generate_sso_url(user: &User, return_url: Option<&str>) -> Result<String, String> {
    let token = generate_sso_token(user, return_url)?;
    
    let discourse_base_url = env::var("DISCOURSE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    let sso_path = "/auth/canvas/sso";
    
    let mut url = format!("{}{}", discourse_base_url, sso_path);
    url.push_str(&format!("?token={}", token));
    
    if let Some(return_to) = return_url {
        url.push_str(&format!("&return_url={}", urlencoding::encode(return_to)));
    }
    
    Ok(url)
}

/// Parse JWT token and extract Canvas user ID
pub fn extract_canvas_id_from_token(token: &str) -> Option<String> {
    jwt::validate_token(token).ok().map(|claims| claims.canvas_id)
}

/// Parse JWT token and extract Discourse user ID
pub fn extract_discourse_id_from_token(token: &str) -> Option<String> {
    jwt::validate_token(token).ok().and_then(|claims| claims.discourse_id)
}
