use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("blockchain storage error: {0}")]
    Storage(String),
    
    #[error("signature verification failed")]
    SignatureVerification,
    
    #[error("invalid achievement format")]
    InvalidFormat,
    
    #[error("resource limit exceeded: {0}")]
    ResourceLimit(String),
    
    #[error("consensus error: {0}")]
    Consensus(String),
    
    #[error("network error: {0}")]
    Network(String),
    
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("serialization error: {0}")]
    Serialization(String),
    
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<String> for BlockchainError {
    fn from(s: String) -> Self {
        BlockchainError::Unknown(s)
    }
}

impl From<&str> for BlockchainError {
    fn from(s: &str) -> Self {
        BlockchainError::Unknown(s.to_string())
    }
}