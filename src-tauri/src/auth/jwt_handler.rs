use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,         // Subject (user ID)
    exp: usize,          // Expiration time
    iat: usize,          // Issued at time
    role: String,        // User role (student, instructor, admin)
    canvas_id: String,   // Canvas user ID
    discourse_id: String, // Discourse user ID
}

pub fn generate_token(
    user_id: &str,
    role: &str, 
    canvas_id: &str,
    discourse_id: &str,
    secret: &[u8]
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 24 * 3600; // 24 hours from now
    
    let issued_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
        
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        iat: issued_at as usize,
        role: role.to_owned(),
        canvas_id: canvas_id.to_owned(),
        discourse_id: discourse_id.to_owned(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(token_data.claims)
}