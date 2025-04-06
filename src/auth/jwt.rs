use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // Subject (user ID)
    pub exp: usize,        // Expiration time
    pub iat: usize,        // Issued at
    pub role: String,      // User role
    pub canvas_id: String, // Canvas user ID
}

pub fn generate_token(user_id: &str, role: &str, canvas_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
        role: role.to_owned(),
        canvas_id: canvas_id.to_owned(),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
}

pub fn is_token_valid(token: &str) -> bool {
    validate_token(token).is_ok()
}

pub fn get_user_id_from_token(token: &str) -> Option<String> {
    validate_token(token).map(|claims| claims.sub).ok()
}

pub fn get_user_role_from_token(token: &str) -> Option<String> {
    validate_token(token).map(|claims| claims.role).ok()
}