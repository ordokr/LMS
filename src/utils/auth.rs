use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::{Storage, window};
use gloo_storage::{LocalStorage, SessionStorage, Storage as GlooStorage};

const TOKEN_KEY: &str = "auth_token";
const USER_KEY: &str = "current_user";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
    pub role: String,
}

// Save authentication token to local storage
pub fn set_auth_token(token: &str) -> Result<(), &'static str> {
    let window = window().ok_or("Failed to get window object")?;
    let storage = window.local_storage().map_err(|_| "Failed to access local storage")?
        .ok_or("Local storage not available")?;
    
    storage.set_item("auth_token", token).map_err(|_| "Failed to write to local storage")
}

// Get authentication token from local storage
pub fn get_auth_token() -> Result<String, &'static str> {
    let window = window().ok_or("Failed to get window object")?;
    let storage = window.local_storage().map_err(|_| "Failed to access local storage")?
        .ok_or("Local storage not available")?;
    
    storage.get_item("auth_token").map_err(|_| "Failed to read from local storage")?
        .ok_or("No auth token found")
}

// Remove authentication token from local storage
pub fn clear_auth_token() -> Result<(), &'static str> {
    let window = window().ok_or("Failed to get window object")?;
    let storage = window.local_storage().map_err(|_| "Failed to access local storage")?
        .ok_or("Local storage not available")?;
    
    storage.remove_item("auth_token").map_err(|_| "Failed to remove from local storage")
}

// Check if user is authenticated
pub fn is_authenticated() -> bool {
    get_auth_token().is_ok()
}

// Save current user to local storage
pub fn set_current_user(user: &User) {
    if let Err(e) = LocalStorage::set(USER_KEY, user) {
        console_error(&format!("Failed to save user data: {}", e));
    }
}

// Get current user from local storage
pub fn get_current_user() -> Option<User> {
    match LocalStorage::get::<User>(USER_KEY) {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

// Get current user ID from local storage
pub fn get_current_user_id() -> Option<i64> {
    get_current_user().map(|user| user.id)
}

// Remove current user from local storage
pub fn remove_current_user() {
    LocalStorage::delete(USER_KEY);
}

// Logout user by removing auth token and current user from local storage
pub fn logout() {
    clear_auth_token().ok();
    remove_current_user();
}

// Parse JWT token (simplified, not cryptographically secure)
pub fn parse_jwt(token: &str) -> Option<JwtClaims> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    
    // Base64 decode payload
    let payload = parts[1];
    let padding_len = (4 - payload.len() % 4) % 4;
    let payload_padded = format!("{}{}", payload, "=".repeat(padding_len));
    
    let decoded = base64::decode_config(&payload_padded, base64::URL_SAFE)
        .ok()?;
        
    let claims = String::from_utf8(decoded).ok()?;
    serde_json::from_str::<JwtClaims>(&claims).ok()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,        // User ID
    pub name: String,       // User's name
    pub email: String,      // User's email
    pub roles: Vec<String>, // User's roles
    pub exp: usize,         // Expiration time
}

fn console_error(message: &str) {
    web_sys::console::error_1(&message.into());
}