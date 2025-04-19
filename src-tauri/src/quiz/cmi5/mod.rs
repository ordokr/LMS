// cmi5 Module for Ordo LMS
//
// This module implements the cmi5 specification (an xAPI Profile) for standardized
// communication between the LMS and learning content.
//
// References:
// - Official Specification: https://aicc.github.io/CMI-5_Spec_Current/
// - Best Practices: https://aicc.github.io/CMI-5_Spec_Current/best_practices/
// - Samples: https://aicc.github.io/CMI-5_Spec_Current/samples/

mod client;
mod models;
mod statements;
mod course_structure;
mod launch;

pub use client::Cmi5Client;
pub use models::{
    Cmi5State, Cmi5Context, Cmi5LaunchData, Cmi5LaunchParameters,
    AssignableUnit, Cmi5Course, Cmi5Verb, Cmi5Result, Cmi5Score
};
pub use statements::{
    Cmi5Statement, Cmi5StatementBuilder, StatementType,
    create_initialized_statement, create_completed_statement,
    create_passed_statement, create_failed_statement,
    create_terminated_statement, create_satisfied_statement,
    create_abandoned_statement, create_waived_statement
};
pub use course_structure::{CourseStructure, parse_course_structure};
pub use launch::{LaunchService, LaunchParameters};

use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::path::Path;
use tracing::{info, error, warn};

/// Main cmi5 service for the Ordo LMS
pub struct Cmi5Service {
    /// Client for communicating with the LRS
    client: Arc<Cmi5Client>,
    
    /// Active sessions
    sessions: Arc<Mutex<HashMap<String, Cmi5Session>>>,
    
    /// Course structures
    courses: Arc<Mutex<HashMap<String, Cmi5Course>>>,
    
    /// Launch service
    launch_service: Arc<LaunchService>,
}

/// cmi5 session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmi5Session {
    /// Session ID
    pub id: String,
    
    /// Actor (learner) ID
    pub actor_id: String,
    
    /// Course ID
    pub course_id: String,
    
    /// Assignable Unit ID
    pub au_id: String,
    
    /// Registration ID
    pub registration_id: String,
    
    /// Launch mode
    pub launch_mode: LaunchMode,
    
    /// Launch parameters
    pub launch_parameters: Cmi5LaunchParameters,
    
    /// Current state
    pub state: Cmi5State,
    
    /// Session start time
    pub start_time: chrono::DateTime<chrono::Utc>,
    
    /// Session end time (if completed)
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Session result
    pub result: Option<Cmi5Result>,
}

/// Launch mode for cmi5 content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaunchMode {
    /// Normal launch mode
    Normal,
    
    /// Browse launch mode
    Browse,
    
    /// Review launch mode
    Review,
}

impl Cmi5Service {
    /// Create a new cmi5 service
    pub fn new(
        endpoint: &str,
        auth_token: Option<&str>,
        launch_service: Arc<LaunchService>,
    ) -> Result<Self> {
        let client = Arc::new(Cmi5Client::new(endpoint, auth_token)?);
        
        Ok(Self {
            client,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            courses: Arc::new(Mutex::new(HashMap::new())),
            launch_service,
        })
    }
    
    /// Import a cmi5 course package
    pub async fn import_course(&self, package_path: &Path) -> Result<String> {
        info!("Importing cmi5 course from: {}", package_path.display());
        
        // Parse the course structure from the package
        let course = parse_course_structure(package_path).await?;
        let course_id = course.id.clone();
        
        // Store the course structure
        let mut courses = self.courses.lock().await;
        courses.insert(course_id.clone(), course);
        
        info!("Successfully imported cmi5 course: {}", course_id);
        Ok(course_id)
    }
    
    /// Launch an assignable unit
    pub async fn launch_assignable_unit(
        &self,
        course_id: &str,
        au_id: &str,
        actor_id: &str,
        launch_mode: LaunchMode,
    ) -> Result<String> {
        info!("Launching assignable unit: {} for actor: {}", au_id, actor_id);
        
        // Get the course structure
        let courses = self.courses.lock().await;
        let course = courses.get(course_id)
            .ok_or_else(|| anyhow!("Course not found: {}", course_id))?;
        
        // Find the assignable unit
        let au = course.assignable_units.iter()
            .find(|au| au.id == au_id)
            .ok_or_else(|| anyhow!("Assignable unit not found: {}", au_id))?;
        
        // Generate a registration ID
        let registration_id = Uuid::new_v4().to_string();
        
        // Create launch parameters
        let launch_parameters = self.launch_service.create_launch_parameters(
            course_id,
            au_id,
            actor_id,
            &registration_id,
            launch_mode,
        ).await?;
        
        // Create a new session
        let session_id = Uuid::new_v4().to_string();
        let session = Cmi5Session {
            id: session_id.clone(),
            actor_id: actor_id.to_string(),
            course_id: course_id.to_string(),
            au_id: au_id.to_string(),
            registration_id,
            launch_mode,
            launch_parameters: launch_parameters.clone(),
            state: Cmi5State::NotInitialized,
            start_time: chrono::Utc::now(),
            end_time: None,
            result: None,
        };
        
        // Store the session
        let mut sessions = self.sessions.lock().await;
        sessions.insert(session_id.clone(), session);
        
        // Generate the launch URL
        let launch_url = self.launch_service.generate_launch_url(
            &au.launch_url,
            &launch_parameters,
        )?;
        
        info!("Successfully created launch URL for session: {}", session_id);
        Ok(launch_url)
    }
    
    /// Initialize a session
    pub async fn initialize_session(&self, session_id: &str) -> Result<()> {
        info!("Initializing session: {}", session_id);
        
        // Get the session
        let mut sessions = self.sessions.lock().await;
        let session = sessions.get_mut(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
        
        // Check if the session is already initialized
        if session.state != Cmi5State::NotInitialized {
            return Err(anyhow!("Session already initialized"));
        }
        
        // Create and send the initialized statement
        let statement = create_initialized_statement(
            &session.actor_id,
            &session.au_id,
            &session.registration_id,
        );
        
        self.client.send_statement(&statement).await?;
        
        // Update the session state
        session.state = Cmi5State::Initialized;
        
        info!("Successfully initialized session: {}", session_id);
        Ok(())
    }
    
    /// Complete a session
    pub async fn complete_session(
        &self,
        session_id: &str,
        score: Option<Cmi5Score>,
        success: Option<bool>,
    ) -> Result<()> {
        info!("Completing session: {}", session_id);
        
        // Get the session
        let mut sessions = self.sessions.lock().await;
        let session = sessions.get_mut(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
        
        // Check if the session is initialized
        if session.state != Cmi5State::Initialized && session.state != Cmi5State::InProgress {
            return Err(anyhow!("Session not initialized or in progress"));
        }
        
        // Create and send the completed statement
        let statement = create_completed_statement(
            &session.actor_id,
            &session.au_id,
            &session.registration_id,
        );
        
        self.client.send_statement(&statement).await?;
        
        // If success is provided, send the appropriate statement
        if let Some(success) = success {
            let statement = if success {
                create_passed_statement(
                    &session.actor_id,
                    &session.au_id,
                    &session.registration_id,
                    score.clone(),
                )
            } else {
                create_failed_statement(
                    &session.actor_id,
                    &session.au_id,
                    &session.registration_id,
                    score.clone(),
                )
            };
            
            self.client.send_statement(&statement).await?;
        }
        
        // Create and send the terminated statement
        let statement = create_terminated_statement(
            &session.actor_id,
            &session.au_id,
            &session.registration_id,
        );
        
        self.client.send_statement(&statement).await?;
        
        // Update the session state and result
        session.state = Cmi5State::Completed;
        session.end_time = Some(chrono::Utc::now());
        session.result = Some(Cmi5Result {
            score,
            success,
            completion: Some(true),
            duration: Some(session.end_time.unwrap() - session.start_time),
        });
        
        info!("Successfully completed session: {}", session_id);
        Ok(())
    }
    
    /// Abandon a session
    pub async fn abandon_session(&self, session_id: &str) -> Result<()> {
        info!("Abandoning session: {}", session_id);
        
        // Get the session
        let mut sessions = self.sessions.lock().await;
        let session = sessions.get_mut(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
        
        // Check if the session is initialized or in progress
        if session.state != Cmi5State::Initialized && session.state != Cmi5State::InProgress {
            return Err(anyhow!("Session not initialized or in progress"));
        }
        
        // Create and send the abandoned statement
        let statement = create_abandoned_statement(
            &session.actor_id,
            &session.au_id,
            &session.registration_id,
        );
        
        self.client.send_statement(&statement).await?;
        
        // Update the session state
        session.state = Cmi5State::Abandoned;
        session.end_time = Some(chrono::Utc::now());
        
        info!("Successfully abandoned session: {}", session_id);
        Ok(())
    }
    
    /// Waive a session
    pub async fn waive_session(&self, session_id: &str, reason: &str) -> Result<()> {
        info!("Waiving session: {}", session_id);
        
        // Get the session
        let mut sessions = self.sessions.lock().await;
        let session = sessions.get_mut(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
        
        // Create and send the waived statement
        let statement = create_waived_statement(
            &session.actor_id,
            &session.au_id,
            &session.registration_id,
            reason,
        );
        
        self.client.send_statement(&statement).await?;
        
        // Update the session state
        session.state = Cmi5State::Waived;
        session.end_time = Some(chrono::Utc::now());
        
        info!("Successfully waived session: {}", session_id);
        Ok(())
    }
    
    /// Get session state
    pub async fn get_session_state(&self, session_id: &str) -> Result<Cmi5State> {
        let sessions = self.sessions.lock().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
        
        Ok(session.state.clone())
    }
    
    /// Get session result
    pub async fn get_session_result(&self, session_id: &str) -> Result<Option<Cmi5Result>> {
        let sessions = self.sessions.lock().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
        
        Ok(session.result.clone())
    }
    
    /// Get all sessions for an actor
    pub async fn get_actor_sessions(&self, actor_id: &str) -> Result<Vec<Cmi5Session>> {
        let sessions = self.sessions.lock().await;
        let actor_sessions = sessions.values()
            .filter(|session| session.actor_id == actor_id)
            .cloned()
            .collect();
        
        Ok(actor_sessions)
    }
    
    /// Get all sessions for a course
    pub async fn get_course_sessions(&self, course_id: &str) -> Result<Vec<Cmi5Session>> {
        let sessions = self.sessions.lock().await;
        let course_sessions = sessions.values()
            .filter(|session| session.course_id == course_id)
            .cloned()
            .collect();
        
        Ok(course_sessions)
    }
    
    /// Get a course by ID
    pub async fn get_course(&self, course_id: &str) -> Result<Cmi5Course> {
        let courses = self.courses.lock().await;
        let course = courses.get(course_id)
            .ok_or_else(|| anyhow!("Course not found: {}", course_id))?
            .clone();
        
        Ok(course)
    }
    
    /// Get all courses
    pub async fn get_all_courses(&self) -> Result<Vec<Cmi5Course>> {
        let courses = self.courses.lock().await;
        let all_courses = courses.values().cloned().collect();
        
        Ok(all_courses)
    }
}
