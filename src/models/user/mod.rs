// Add these if they don't exist
pub mod profile;
pub mod activity;
pub mod follow;

// Add these re-exports
pub use profile::*;
pub use activity::*;
pub use follow::*;

// Add this struct to your existing user module file

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserSummary {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
}