use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Cmi5Session {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub session_id: String,
    pub registration_id: String,
    pub actor_json: String,
    pub activity_id: String,
    pub return_url: Option<String>,
    pub status: Cmi5SessionStatus,
    pub score: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Cmi5SessionStatus {
    Initialized,
    Launched,
    InProgress,
    Completed,
    Passed,
    Failed,
    Abandoned,
    Waived,
}

// Implement FromRow for Cmi5SessionStatus
impl<'r> sqlx::decode::Decode<'r, sqlx::Sqlite> for Cmi5SessionStatus {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as sqlx::decode::Decode<sqlx::Sqlite>>::decode(value)?;
        match value {
            "initialized" => Ok(Cmi5SessionStatus::Initialized),
            "launched" => Ok(Cmi5SessionStatus::Launched),
            "in_progress" => Ok(Cmi5SessionStatus::InProgress),
            "completed" => Ok(Cmi5SessionStatus::Completed),
            "passed" => Ok(Cmi5SessionStatus::Passed),
            "failed" => Ok(Cmi5SessionStatus::Failed),
            "abandoned" => Ok(Cmi5SessionStatus::Abandoned),
            "waived" => Ok(Cmi5SessionStatus::Waived),
            _ => Err(format!("Unknown CMI5 session status: {}", value).into()),
        }
    }
}

// Implement Type for Cmi5SessionStatus
impl sqlx::Type<sqlx::Sqlite> for Cmi5SessionStatus {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <&str as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

// Implement Encode for Cmi5SessionStatus
impl<'q> sqlx::encode::Encode<'q, sqlx::Sqlite> for Cmi5SessionStatus {
    fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> sqlx::encode::IsNull {
        let s = match self {
            Cmi5SessionStatus::Initialized => "initialized",
            Cmi5SessionStatus::Launched => "launched",
            Cmi5SessionStatus::InProgress => "in_progress",
            Cmi5SessionStatus::Completed => "completed",
            Cmi5SessionStatus::Passed => "passed",
            Cmi5SessionStatus::Failed => "failed",
            Cmi5SessionStatus::Abandoned => "abandoned",
            Cmi5SessionStatus::Waived => "waived",
        };
        <&str as sqlx::encode::Encode<sqlx::Sqlite>>::encode_by_ref(&s, args)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCmi5SessionRequest {
    pub quiz_id: String,
    pub registration_id: String,
    pub actor_json: String,
    pub activity_id: String,
    pub return_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchCmi5SessionRequest {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteCmi5SessionRequest {
    pub session_id: String,
    pub score: Option<f64>,
    pub passed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbandonCmi5SessionRequest {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaiveCmi5SessionRequest {
    pub session_id: String,
    pub reason: Option<String>,
}
