use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fmt;
use log::{debug, error};

// Error type for Canvas API
#[derive(Debug)]
pub enum CanvasError {
    RequestError(reqwest::Error),
    ApiError { status: u16, message: String },
    ParseError(String),
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CanvasError::RequestError(e) => write!(f, "Request error: {}", e),
            CanvasError::ApiError { status, message } => write!(f, "API error {}: {}", status, message),
            CanvasError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for CanvasError {}

impl From<reqwest::Error> for CanvasError {
    fn from(error: reqwest::Error) -> Self {
        CanvasError::RequestError(error)
    }
}

// Canvas API Client
pub struct CanvasClient {
    client: Client,
    base_url: String,
    auth_token: String,
}

// Canvas Course Model
#[derive(Debug, Deserialize)]
pub struct CanvasCourse {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub course_code: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub workflow_state: String,
    #[serde(default)]
    pub start_at: Option<String>,
    #[serde(default)]
    pub end_at: Option<String>,
}

// Canvas User Model
#[derive(Debug, Deserialize)]
pub struct CanvasUser {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub email: String,
}

// Canvas Enrollment Model
#[derive(Debug, Deserialize)]
pub struct CanvasEnrollment {
    pub id: i64,
    pub user: CanvasUser,
    pub role: String,
    #[serde(default)]
    pub enrollment_state: String,
}

// Canvas Assignment Model
#[derive(Debug, Deserialize)]
pub struct CanvasAssignment {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub points_possible: Option<f64>,
    pub due_at: Option<String>,
    pub unlock_at: Option<String>,
    pub lock_at: Option<String>,
}

impl CanvasClient {
    pub fn new(base_url: &str, auth_token: &str) -> Self {
        let client = Client::new();
        
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            auth_token: auth_token.to_string(),
        }
    }
    
    // Get all courses
    pub async fn get_courses(&self) -> Result<Vec<CanvasCourse>, CanvasError> {
        let url = format!("{}/api/v1/courses", self.base_url);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(CanvasError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }
        
        let courses = response.json::<Vec<CanvasCourse>>().await?;
        Ok(courses)
    }
    
    // Get course by ID
    pub async fn get_course(&self, course_id: i64) -> Result<CanvasCourse, CanvasError> {
        let url = format!("{}/api/v1/courses/{}", self.base_url, course_id);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(CanvasError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }
        
        let course = response.json::<CanvasCourse>().await?;
        Ok(course)
    }
    
    // Get course enrollments
    pub async fn get_course_enrollments(&self, course_id: i64) -> Result<Vec<CanvasEnrollment>, CanvasError> {
        let url = format!("{}/api/v1/courses/{}/enrollments", self.base_url, course_id);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(CanvasError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }
        
        let enrollments = response.json::<Vec<CanvasEnrollment>>().await?;
        Ok(enrollments)
    }
    
    // Get course assignments
    pub async fn get_course_assignments(&self, course_id: i64) -> Result<Vec<CanvasAssignment>, CanvasError> {
        let url = format!("{}/api/v1/courses/{}/assignments", self.base_url, course_id);
        
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(CanvasError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }
        
        let assignments = response.json::<Vec<CanvasAssignment>>().await?;
        Ok(assignments)
    }
}