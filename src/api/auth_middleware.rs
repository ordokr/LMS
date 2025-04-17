use axum::{async_trait, extract::{FromRequestParts, TypedHeader}, http::{request::Parts, StatusCode}, response::IntoResponse};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error as JwtError};
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"super_secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String, // e.g., "admin", "user"
}

pub struct AuthUser {
    pub user_id: String,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser {
    type Rejection = (StatusCode, String);
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(auth_header) = TypedHeader::<headers::Authorization<headers::authorization::Bearer>>::from_request_parts(parts, _state).await.map_err(|_| (StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header".to_string()))?;
        let token = auth_header.token();
        let token_data: TokenData<Claims> = decode(token, &DecodingKey::from_secret(SECRET), &Validation::new(Algorithm::HS256)).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
        Ok(AuthUser {
            user_id: token_data.claims.sub,
            role: token_data.claims.role,
        })
    }
}

pub fn require_role(user: &AuthUser, required: &str) -> Result<(), (StatusCode, String)> {
    if user.role == required || user.role == "admin" {
        Ok(())
    } else {
        Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()))
    }
}
