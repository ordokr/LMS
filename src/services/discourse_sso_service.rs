use crate::config::Config;
use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use url::form_urlencoded;
use std::collections::HashMap;
use std::error::Error;

type HmacSha256 = Hmac<Sha256>;

/// Error type for Discourse SSO operations
#[derive(Debug, thiserror::Error)]
pub enum DiscourseSSOError {
    #[error("Invalid SSO signature")]
    InvalidSignature,
    #[error("Failed to decode SSO payload: {0}")]
    DecodingError(String),
    #[error("Missing nonce in SSO payload")]
    MissingNonce,
    #[error("HMAC error: {0}")]
    HmacError(String),
    #[error("Encoding error: {0}")]
    EncodingError(String),
}

/// Generate Discourse SSO payload for a user
///
/// # Arguments
///
/// * `user` - User object
/// * `sso` - Base64 encoded SSO payload from Discourse
/// * `sig` - Signature from Discourse
/// * `config` - Application configuration
///
/// # Returns
///
/// * `Result<String, DiscourseSSOError>` - URL-encoded payload for Discourse SSO
pub fn generate_discourse_sso_payload(
    user: &impl UserInfo,
    sso: &str,
    sig: &str,
    config: &Config,
) -> Result<String, DiscourseSSOError> {
    // Verify the signature from Discourse
    let mut mac = HmacSha256::new_from_slice(config.discourse_sso_secret.as_bytes())
        .map_err(|e| DiscourseSSOError::HmacError(e.to_string()))?;
    
    mac.update(sso.as_bytes());
    let computed_sig = hex::encode(mac.finalize().into_bytes());
    
    if computed_sig != sig {
        return Err(DiscourseSSOError::InvalidSignature);
    }
    
    // Decode the payload
    let decoded_payload = general_purpose::STANDARD
        .decode(sso)
        .map_err(|e| DiscourseSSOError::DecodingError(e.to_string()))?;
    
    let payload_str = String::from_utf8(decoded_payload)
        .map_err(|e| DiscourseSSOError::DecodingError(e.to_string()))?;
    
    // Parse the payload to get the nonce
    let params: HashMap<String, String> = form_urlencoded::parse(payload_str.as_bytes())
        .into_owned()
        .collect();
    
    let nonce = params.get("nonce")
        .ok_or(DiscourseSSOError::MissingNonce)?;
    
    // Prepare response payload
    let mut response_params = HashMap::new();
    response_params.insert("nonce", nonce.as_str());
    response_params.insert("external_id", &user.id().to_string());
    response_params.insert("email", user.email());
    response_params.insert("username", &user.name().replace(" ", "_").to_lowercase());
    response_params.insert("name", user.name());
    
    // Additional optional parameters
    if let Some(roles) = user.roles() {
        response_params.insert("groups", &roles.join(","));
    }
    
    // Encode the response
    let return_payload = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(response_params.iter())
        .finish();
    
    let base64_payload = general_purpose::STANDARD.encode(return_payload.as_bytes());
    
    // Generate signature for the response
    let mut return_mac = HmacSha256::new_from_slice(config.discourse_sso_secret.as_bytes())
        .map_err(|e| DiscourseSSOError::HmacError(e.to_string()))?;
    
    return_mac.update(base64_payload.as_bytes());
    let return_sig = hex::encode(return_mac.finalize().into_bytes());
    
    // Combine and return final payload
    Ok(format!("sso={}&sig={}", urlencoding::encode(&base64_payload), return_sig))
}

/// Trait for user information needed for SSO
pub trait UserInfo {
    fn id(&self) -> &str;
    fn email(&self) -> &str;
    fn name(&self) -> &str;
    fn roles(&self) -> Option<&Vec<String>>;
}
