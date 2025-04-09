use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenData {
    pub user_id: String,
    pub token_id: String,
    pub expires_at: i64,
    pub revoked: bool,
}

#[derive(Debug)]
pub struct RefreshTokenStore {
    tokens: Mutex<HashMap<String, RefreshTokenData>>,
}

impl RefreshTokenStore {
    pub fn new() -> Self {
        Self {
            tokens: Mutex::new(HashMap::new()),
        }
    }

    pub fn generate_token(&self, user_id: &str, expires_in_days: i64) -> String {
        let token = Uuid::new_v4().to_string();
        let token_id = Uuid::new_v4().to_string();
        
        let expires_at = Utc::now()
            .checked_add_signed(Duration::days(expires_in_days))
            .expect("valid timestamp")
            .timestamp();
            
        let token_data = RefreshTokenData {
            user_id: user_id.to_owned(),
            token_id,
            expires_at,
            revoked: false,
        };
        
        self.tokens.lock().unwrap().insert(token.clone(), token_data);
        
        token
    }
    
    pub fn validate_token(&self, token: &str) -> Option<RefreshTokenData> {
        let tokens = self.tokens.lock().unwrap();
        
        if let Some(token_data) = tokens.get(token) {
            // Check if token is expired or revoked
            if token_data.revoked || token_data.expires_at < Utc::now().timestamp() {
                return None;
            }
            
            return Some(token_data.clone());
        }
        
        None
    }
    
    pub fn revoke_token(&self, token: &str) -> bool {
        let mut tokens = self.tokens.lock().unwrap();
        
        if let Some(mut token_data) = tokens.get_mut(token) {
            token_data.revoked = true;
            return true;
        }
        
        false
    }
    
    pub fn revoke_all_for_user(&self, user_id: &str) -> usize {
        let mut tokens = self.tokens.lock().unwrap();
        let mut count = 0;
        
        for token_data in tokens.values_mut() {
            if token_data.user_id == user_id && !token_data.revoked {
                token_data.revoked = true;
                count += 1;
            }
        }
        
        count
    }
    
    pub fn cleanup_expired(&self) -> usize {
        let mut tokens = self.tokens.lock().unwrap();
        let now = Utc::now().timestamp();
        let token_keys: Vec<String> = tokens
            .iter()
            .filter(|(_, data)| data.expires_at < now)
            .map(|(key, _)| key.clone())
            .collect();
            
        let count = token_keys.len();
        
        for key in token_keys {
            tokens.remove(&key);
        }
        
        count
    }
}
