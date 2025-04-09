use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;
use std::str::FromStr;

// Newtype pattern for domain safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

// Similar implementations for CourseId, AchievementId, etc.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRecord {
    pub user: UserId,
    pub course: CourseId,
    #[serde(with = "hex")]
    pub tx_hash: [u8; 32],
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub achievement_type: AchievementType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AchievementType {
    CourseCompletion,
    BadgeEarned,
    CertificateIssued,
}

// Enforce type safety at API boundaries
pub trait BlockchainStore {
    fn record_achievement(&mut self, achievement: AchievementRecord) -> Result<(), BlockchainError>;
    fn verify_achievement(&self, user: UserId, achievement_hash: [u8; 32]) -> Result<bool, BlockchainError>;
}

#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("blockchain storage error: {0}")]
    Storage(String),
    #[error("signature verification failed")]
    SignatureVerification,
    #[error("invalid achievement format")]
    InvalidFormat,
}