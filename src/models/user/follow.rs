use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserFollow {
    pub id: String,
    pub follower_id: String,
    pub following_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TopicSubscription {
    pub id: String,
    pub user_id: String,
    pub topic_id: String,
    pub subscription_level: SubscriptionLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CategorySubscription {
    pub id: String,
    pub user_id: String,
    pub category_id: String,
    pub subscription_level: SubscriptionLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionLevel {
    Watching,       // All activity generates notifications
    Tracking,       // Only replies to your posts generate notifications
    Normal,         // Default level - occasional notifications
    Muted,          // No notifications
}