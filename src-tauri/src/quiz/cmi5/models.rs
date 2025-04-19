// cmi5 Models
//
// This module defines the data models for cmi5 content and tracking.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Duration, Utc};

/// cmi5 state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cmi5State {
    /// Not initialized
    NotInitialized,
    
    /// Initialized
    Initialized,
    
    /// In progress
    InProgress,
    
    /// Completed
    Completed,
    
    /// Passed
    Passed,
    
    /// Failed
    Failed,
    
    /// Abandoned
    Abandoned,
    
    /// Waived
    Waived,
    
    /// Terminated
    Terminated,
}

/// cmi5 context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5Context {
    /// Registration ID
    pub registration: String,
    
    /// Context activities
    #[serde(rename = "contextActivities")]
    pub context_activities: ContextActivities,
    
    /// Extensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// Context activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextActivities {
    /// Category activities
    pub category: Vec<Activity>,
    
    /// Grouping activities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<Vec<Activity>>,
    
    /// Parent activities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<Activity>>,
    
    /// Other activities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<Activity>>,
}

/// Activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    /// Activity ID
    pub id: String,
    
    /// Activity definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ActivityDefinition>,
}

/// Activity definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDefinition {
    /// Activity name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<HashMap<String, String>>,
    
    /// Activity description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<HashMap<String, String>>,
    
    /// Activity type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    
    /// More info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moreInfo: Option<String>,
    
    /// Extensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// cmi5 launch data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5LaunchData {
    /// Endpoint
    pub endpoint: String,
    
    /// Fetch URL
    pub fetch: String,
    
    /// Actor
    pub actor: serde_json::Value,
    
    /// Registration
    pub registration: String,
    
    /// Activity ID
    #[serde(rename = "activityId")]
    pub activity_id: String,
}

/// cmi5 launch parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5LaunchParameters {
    /// Endpoint
    pub endpoint: String,
    
    /// Actor
    pub actor: String,
    
    /// Registration
    pub registration: String,
    
    /// Activity ID
    #[serde(rename = "activityId")]
    pub activity_id: String,
    
    /// Auth token
    #[serde(rename = "auth-token")]
    pub auth_token: String,
}

/// Assignable unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignableUnit {
    /// ID
    pub id: String,
    
    /// Title
    pub title: String,
    
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Launch URL
    #[serde(rename = "launchURL")]
    pub launch_url: String,
    
    /// Launch parameters
    #[serde(rename = "launchParameters", skip_serializing_if = "Option::is_none")]
    pub launch_parameters: Option<String>,
    
    /// Entry mode
    #[serde(rename = "entryMode")]
    pub entry_mode: EntryMode,
    
    /// Move on
    #[serde(rename = "moveOn")]
    pub move_on: MoveOn,
    
    /// Mastery score
    #[serde(rename = "masteryScore", skip_serializing_if = "Option::is_none")]
    pub mastery_score: Option<f64>,
    
    /// Passing score method
    #[serde(rename = "passingScoreMethod", skip_serializing_if = "Option::is_none")]
    pub passing_score_method: Option<PassingScoreMethod>,
    
    /// Max attempts
    #[serde(rename = "maxAttempts", skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
    
    /// Activity type
    #[serde(rename = "activityType", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
}

/// Entry mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryMode {
    /// Review
    #[serde(rename = "Review")]
    Review,
    
    /// Browse
    #[serde(rename = "Browse")]
    Browse,
    
    /// Normal
    #[serde(rename = "Normal")]
    Normal,
}

/// Move on
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveOn {
    /// Completed
    #[serde(rename = "Completed")]
    Completed,
    
    /// Passed
    #[serde(rename = "Passed")]
    Passed,
    
    /// Completed and passed
    #[serde(rename = "CompletedAndPassed")]
    CompletedAndPassed,
    
    /// Completed or passed
    #[serde(rename = "CompletedOrPassed")]
    CompletedOrPassed,
    
    /// Not applicable
    #[serde(rename = "NotApplicable")]
    NotApplicable,
}

/// Passing score method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassingScoreMethod {
    /// Percentage
    #[serde(rename = "percentage")]
    Percentage,
    
    /// Points
    #[serde(rename = "points")]
    Points,
    
    /// Scaled
    #[serde(rename = "scaled")]
    Scaled,
}

/// cmi5 course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5Course {
    /// ID
    pub id: String,
    
    /// Title
    pub title: String,
    
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    
    /// Publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    
    /// Assignable units
    #[serde(rename = "assignableUnits")]
    pub assignable_units: Vec<AssignableUnit>,
    
    /// Objectives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Vec<Objective>>,
}

/// Objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    /// ID
    pub id: String,
    
    /// Title
    pub title: String,
    
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// cmi5 verb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5Verb {
    /// ID
    pub id: String,
    
    /// Display
    pub display: HashMap<String, String>,
}

/// cmi5 result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5Result {
    /// Score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<Cmi5Score>,
    
    /// Success
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    
    /// Completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<bool>,
    
    /// Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
}

/// cmi5 score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5Score {
    /// Scaled score (between -1 and 1)
    pub scaled: f64,
    
    /// Raw score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<f64>,
    
    /// Minimum score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    
    /// Maximum score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

impl Cmi5Score {
    /// Create a new score
    pub fn new(scaled: f64, raw: Option<f64>, min: Option<f64>, max: Option<f64>) -> Self {
        // Ensure scaled score is between -1 and 1
        let scaled = scaled.max(-1.0).min(1.0);
        
        Self {
            scaled,
            raw,
            min,
            max,
        }
    }
    
    /// Create a score from raw, min, and max values
    pub fn from_raw(raw: f64, min: f64, max: f64) -> Self {
        // Calculate scaled score
        let scaled = if max > min {
            (raw - min) / (max - min)
        } else {
            0.0
        };
        
        Self {
            scaled: scaled.max(-1.0).min(1.0),
            raw: Some(raw),
            min: Some(min),
            max: Some(max),
        }
    }
    
    /// Create a percentage score
    pub fn percentage(percentage: f64) -> Self {
        Self {
            scaled: (percentage / 100.0).max(-1.0).min(1.0),
            raw: Some(percentage),
            min: Some(0.0),
            max: Some(100.0),
        }
    }
}
