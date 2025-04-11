// Auto-generated from UserModel.js
// Source: src/models/unified\UserModel.js

use serde::{Deserialize, Serialize};

// Related model imports
use crate::models::date::Date;
use crate::models::canvas::canvas;
use crate::models::discourse::discourse;


#[derive(Debug, Clone, Serialize, Deserialize)]
/// UserModel model - ported from Unified
/// Reference: src/models/unified\UserModel.js
pub struct UserModel {
    // Fields
    pub avatarUrl: Option<String>,
    pub avatar_template: Option<String>,
    pub avatar_url: Option<String>,
    pub badgeCount: Option<i64>,
    pub bio: Option<String>,
    pub canvasId: Option<canvas>,
    pub createdAt: Option<Date>,
    pub created_at: Option<String>,
    pub data: Option<String>,
    pub discourseId: Option<discourse>,
    pub email: Option<String>,
    pub enrollments: Vec<String>,
    pub id: Option<i64>,
    pub lastLogin: Option<String>,
    pub lastSeenAt: Option<String>, // DateTime
    pub last_seen_at: Option<String>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,
    pub timezone: Option<String>,
    pub trustLevel: Option<i64>,
    pub trust_level: Option<String>,
    pub updatedAt: Option<Date>,
    pub updated_at: Option<String>,
    pub username: Option<String>,
    pub website: Option<String>,
    pub websiteUrl: Option<String>,
}

impl UserModel {
    pub fn new() -> Self {
        Self {
            avatarUrl: String::new(),
            avatar_template: None,
            avatar_url: None,
            badgeCount: 0,
            bio: String::new(),
            canvasId: None,
            createdAt: None,
            created_at: String::new(),
            data: None,
            discourseId: None,
            email: String::new(),
            enrollments: Vec::new(),
            id: 0,
            lastLogin: None,
            lastSeenAt: None,
            last_seen_at: String::new(),
            locale: String::new(),
            name: String::new(),
            source: None,
            timezone: String::new(),
            trustLevel: 0,
            trust_level: None,
            updatedAt: None,
            updated_at: String::new(),
            username: String::new(),
            website: None,
            websiteUrl: String::new(),
        }
    }

    /// Get createdAt - hasOne relationship to Date
    pub fn createdAt(&self) -> Option<&Date> {
        self.createdAt.as_ref()
    }

    /// Get updatedAt - hasOne relationship to Date
    pub fn updatedAt(&self) -> Option<&Date> {
        self.updatedAt.as_ref()
    }

    /// Get canvasId - belongsTo relationship to canvas
    pub fn canvasId(&self) -> Option<i64> {
        self.canvasId
    }

    /// Get discourseId - belongsTo relationship to discourse
    pub fn discourseId(&self) -> Option<i64> {
        self.discourseId
    }

    // TODO: Implement if from UserModel
    pub fn if(&self) -> bool {
        // Implementation needed
        false
    }

    // TODO: Implement toCanvasUser from UserModel
    pub fn toCanvasUser(&self) -> bool {
        // Implementation needed
        false
    }

    // TODO: Implement toDiscourseUser from UserModel
    pub fn toDiscourseUser(&self) -> bool {
        // Implementation needed
        false
    }

    // TODO: Implement fromCanvas from UserModel
    pub fn fromCanvas(&self) -> bool {
        // Implementation needed
        false
    }

    // TODO: Implement fromDiscourse from UserModel
    pub fn fromDiscourse(&self) -> bool {
        // Implementation needed
        false
    }

}
