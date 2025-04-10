use base64::{engine::general_purpose, Engine as _};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use log::{info, error, warn};
use url::form_urlencoded;

/// User data for Discourse SSO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub roles: Vec<String>,
}

/// Error type for Discourse SSO operations
#[derive(Debug, Error)]
pub enum DiscourseSsoError {
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    
    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),
    
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    
    #[error("SSO error: {0}")]
    SsoError(String),
}

/// Type alias for result with DiscourseSsoError
pub type Result<T> = std::result::Result<T, DiscourseSsoError>;

/// Discourse SSO service
#[derive(Clone)]
pub struct DiscourseSsoService {
    sso_secret: String,
}

impl DiscourseSsoService {
    /// Create a new Discourse SSO service
    pub fn new(sso_secret: &str) -> Self {
        Self {
            sso_secret: sso_secret.to_string(),
        }
    }
    
    /// Validate an incoming SSO request from Discourse
    pub fn validate_sso(&self, sso: &str, sig: &str) -> Result<HashMap<String, String>> {
        info!("Validating Discourse SSO request");
        
        // Verify the signature
        if !self.verify_signature(sso, sig) {
            error!("Invalid Discourse SSO signature");
            return Err(DiscourseSsoError::InvalidSignature);
        }
        
        // Decode the payload
        let payload_bytes = general_purpose::STANDARD.decode(sso)?;
        let payload_str = String::from_utf8(payload_bytes)
            .map_err(|e| DiscourseSsoError::SsoError(format!("Invalid UTF-8: {}", e)))?;
        
        info!("Successfully validated Discourse SSO request");
        
        // Parse the query parameters into a HashMap
        let params = form_urlencoded::parse(payload_str.as_bytes())
            .into_owned()
            .collect::<HashMap<String, String>>();
        
        // Ensure nonce is present
        if !params.contains_key("nonce") {
            warn!("Missing nonce in Discourse SSO payload");
            return Err(DiscourseSsoError::MissingParameter("nonce".to_string()));
        }
        
        Ok(params)
    }
    
    /// Generate Discourse SSO response payload
    pub fn generate_sso_payload(&self, user: &SsoUser, sso: &str, sig: &str) -> Result<String> {
        info!("Generating Discourse SSO payload for user {}", user.name);
        
        // First validate the incoming request
        let params = self.validate_sso(sso, sig)?;
        let nonce = params.get("nonce")
            .ok_or_else(|| DiscourseSsoError::MissingParameter("nonce".to_string()))?;
        
        // Prepare response payload
        let mut response_params = HashMap::new();
        response_params.insert("nonce", nonce.clone());
        response_params.insert("external_id", user.id.clone());
        response_params.insert("email", user.email.clone());
        
        // Create a username from the user's name
        let username = user.name.replace(' ', "_").to_lowercase();
        response_params.insert("username", username);
        response_params.insert("name", user.name.clone());
        
        // Add groups (roles) if present
        if !user.roles.is_empty() {
            response_params.insert("groups", user.roles.join(","));
        }
        
        // Encode the response as a URL query string
        let params_string = response_params.iter()
            .map(|(k, v)| format!("{}={}", k, form_urlencoded::byte_serialize(v.as_bytes()).collect::<String>()))
            .collect::<Vec<String>>()
            .join("&");
        
        // Base64 encode the query string
        let base64_payload = general_purpose::STANDARD.encode(params_string.as_bytes());
        
        // Generate signature for the response
        let sig = self.generate_signature(&base64_payload);
        
        // Combine into the final response
        let response = format!("sso={}&sig={}", 
            form_urlencoded::byte_serialize(base64_payload.as_bytes()).collect::<String>(),
            sig);
        
        info!("Successfully generated Discourse SSO payload");
        
        Ok(response)
    }
    
    /// Verify the signature of an incoming SSO request
    fn verify_signature(&self, sso: &str, sig: &str) -> bool {
        let computed_sig = self.generate_signature(sso);
        computed_sig == sig
    }
    
    /// Generate an HMAC-SHA256 signature for a payload
    fn generate_signature(&self, payload: &str) -> String {
        let mut hmac = Hmac::new(Sha256::new(), self.sso_secret.as_bytes());
        hmac.input(payload.as_bytes());
        let result = hmac.result();
        let code = result.code();
        hex::encode(code)
    }
    
    /// Factory function to create a new service instance
    pub fn create(sso_secret: &str) -> Arc<Self> {
        Arc::new(Self::new(sso_secret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_verify_signature_valid() {
        let service = DiscourseSsoService::new("discourse_sso_test_secret");
        let payload = "bm9uY2U9YWJjMTIz"; // Base64 of "nonce=abc123"
        
        // Generate a valid signature
        let valid_sig = service.generate_signature(payload);
        
        // Verify the signature
        assert!(service.verify_signature(payload, &valid_sig));
    }
    
    #[test]
    fn test_verify_signature_invalid() {
        let service = DiscourseSsoService::new("discourse_sso_test_secret");
        let payload = "bm9uY2U9YWJjMTIz"; // Base64 of "nonce=abc123"
        let invalid_sig = "invalidSignature123456789abcdef";
        
        // Verify the signature should fail
        assert!(!service.verify_signature(payload, invalid_sig));
    }
    
    #[test]
    fn test_validate_sso_success() {
        let service = DiscourseSsoService::new("discourse_sso_test_secret");
        
        // Create a test payload - "nonce=abc123"
        let payload = "bm9uY2U9YWJjMTIz";
        let sig = service.generate_signature(payload);
        
        let result = service.validate_sso(payload, &sig);
        assert!(result.is_ok());
        
        let params = result.unwrap();
        assert_eq!(params.get("nonce"), Some(&"abc123".to_string()));
    }
    
    #[test]
    fn test_validate_sso_invalid_signature() {
        let service = DiscourseSsoService::new("discourse_sso_test_secret");
        let payload = "bm9uY2U9YWJjMTIz"; // Base64 of "nonce=abc123"
        let invalid_sig = "invalidSignature123456789abcdef";
        
        let result = service.validate_sso(payload, invalid_sig);
        assert!(result.is_err());
        
        match result {
            Err(DiscourseSsoError::InvalidSignature) => {}, // This is expected
            _ => panic!("Expected InvalidSignature error"),
        }
    }
    
    #[test]
    fn test_generate_sso_payload() {
        let service = DiscourseSsoService::new("discourse_sso_test_secret");
        
        // Create a test payload - "nonce=abc123"
        let payload = "bm9uY2U9YWJjMTIz";
        let sig = service.generate_signature(payload);
        
        let user = SsoUser {
            id: "123".to_string(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            roles: vec!["student".to_string()],
        };
        
        let result = service.generate_sso_payload(&user, payload, &sig);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.starts_with("sso="));
        assert!(response.contains("&sig="));
    }
}
