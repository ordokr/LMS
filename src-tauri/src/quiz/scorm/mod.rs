use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use zip::ZipArchive;
use std::fs::{self, File};
use std::io::{Read, Write};
use tracing::{debug, info, warn, error};
use xml::reader::{EventReader, XmlEvent};

/// SCORM version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScormVersion {
    /// SCORM 1.2
    V1_2,
    
    /// SCORM 2004 3rd Edition
    V2004_3RD,
    
    /// SCORM 2004 4th Edition
    V2004_4TH,
}

/// SCORM package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScormPackageMetadata {
    /// Package ID
    pub id: Uuid,
    
    /// Package title
    pub title: String,
    
    /// Package description
    pub description: Option<String>,
    
    /// SCORM version
    pub version: ScormVersion,
    
    /// Package identifier
    pub identifier: String,
    
    /// Package version
    pub package_version: Option<String>,
    
    /// Package author
    pub author: Option<String>,
    
    /// Package organization
    pub organization: Option<String>,
    
    /// Package duration
    pub duration: Option<String>,
    
    /// Package keywords
    pub keywords: Vec<String>,
    
    /// Package launch URL
    pub launch_url: String,
    
    /// Created at timestamp
    pub created_at: DateTime<Utc>,
    
    /// Updated at timestamp
    pub updated_at: DateTime<Utc>,
}

/// SCORM activity state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScormActivityState {
    /// Not attempted
    NotAttempted,
    
    /// Attempted
    Attempted,
    
    /// Completed
    Completed,
    
    /// Incomplete
    Incomplete,
    
    /// Passed
    Passed,
    
    /// Failed
    Failed,
    
    /// Unknown
    Unknown,
}

/// SCORM interaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScormInteractionType {
    /// True/false
    TrueFalse,
    
    /// Multiple choice
    MultipleChoice,
    
    /// Fill in the blank
    FillIn,
    
    /// Matching
    Matching,
    
    /// Performance
    Performance,
    
    /// Sequencing
    Sequencing,
    
    /// Likert
    Likert,
    
    /// Numeric
    Numeric,
    
    /// Other
    Other,
}

/// SCORM interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScormInteraction {
    /// Interaction ID
    pub id: String,
    
    /// Interaction type
    pub interaction_type: ScormInteractionType,
    
    /// Interaction description
    pub description: Option<String>,
    
    /// Learner response
    pub learner_response: Option<String>,
    
    /// Correct response
    pub correct_response: Option<String>,
    
    /// Result
    pub result: Option<String>,
    
    /// Weighting
    pub weighting: Option<f32>,
    
    /// Latency
    pub latency: Option<String>,
    
    /// Timestamp
    pub timestamp: Option<DateTime<Utc>>,
}

/// SCORM session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScormSession {
    /// Session ID
    pub id: Uuid,
    
    /// Package ID
    pub package_id: Uuid,
    
    /// User ID
    pub user_id: Uuid,
    
    /// Session state
    pub state: ScormActivityState,
    
    /// Session score
    pub score: Option<f32>,
    
    /// Session max score
    pub max_score: Option<f32>,
    
    /// Session min score
    pub min_score: Option<f32>,
    
    /// Session time
    pub total_time: Option<String>,
    
    /// Session interactions
    pub interactions: Vec<ScormInteraction>,
    
    /// Session data
    pub data: HashMap<String, String>,
    
    /// Created at timestamp
    pub created_at: DateTime<Utc>,
    
    /// Updated at timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Completed at timestamp
    pub completed_at: Option<DateTime<Utc>>,
}

/// SCORM service
pub struct ScormService {
    /// Package storage directory
    package_dir: PathBuf,
    
    /// Packages
    packages: HashMap<Uuid, ScormPackageMetadata>,
    
    /// Sessions
    sessions: HashMap<Uuid, ScormSession>,
}

impl ScormService {
    /// Create a new SCORM service
    pub fn new(package_dir: PathBuf) -> Result<Self> {
        // Create package directory if it doesn't exist
        if !package_dir.exists() {
            fs::create_dir_all(&package_dir)?;
        }
        
        Ok(Self {
            package_dir,
            packages: HashMap::new(),
            sessions: HashMap::new(),
        })
    }
    
    /// Import a SCORM package
    pub fn import_package(&mut self, package_path: &Path) -> Result<Uuid> {
        // Open the package file
        let file = File::open(package_path)?;
        
        // Create a ZIP archive
        let mut archive = ZipArchive::new(file)?;
        
        // Check if the package contains an imsmanifest.xml file
        if !archive.file_names().any(|name| name == "imsmanifest.xml") {
            return Err(anyhow!("Package does not contain an imsmanifest.xml file"));
        }
        
        // Extract the manifest file
        let mut manifest_file = archive.by_name("imsmanifest.xml")?;
        let mut manifest_content = String::new();
        manifest_file.read_to_string(&mut manifest_content)?;
        
        // Parse the manifest file
        let metadata = self.parse_manifest(&manifest_content)?;
        
        // Create a directory for the package
        let package_dir = self.package_dir.join(metadata.id.to_string());
        fs::create_dir_all(&package_dir)?;
        
        // Extract the package
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = package_dir.join(file.name());
            
            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(parent) = outpath.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }
                
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
        
        // Store the package metadata
        self.packages.insert(metadata.id, metadata.clone());
        
        Ok(metadata.id)
    }
    
    /// Parse a SCORM manifest file
    fn parse_manifest(&self, manifest_content: &str) -> Result<ScormPackageMetadata> {
        // Create a new XML parser
        let parser = EventReader::from_str(manifest_content);
        
        // Parse the manifest
        let mut title = None;
        let mut description = None;
        let mut version = None;
        let mut identifier = None;
        let mut package_version = None;
        let mut author = None;
        let mut organization = None;
        let mut duration = None;
        let mut keywords = Vec::new();
        let mut launch_url = None;
        
        // In a real implementation, we would parse the XML properly
        // For now, we'll just extract some basic information
        
        // Check for SCORM version
        if manifest_content.contains("http://www.adlnet.org/xsd/adlcp_rootv1p2") {
            version = Some(ScormVersion::V1_2);
        } else if manifest_content.contains("http://www.adlnet.org/xsd/adlcp_v1p3") {
            version = Some(ScormVersion::V2004_3RD);
        } else if manifest_content.contains("http://www.adlnet.org/xsd/adlcp_v1p4") {
            version = Some(ScormVersion::V2004_4TH);
        } else {
            return Err(anyhow!("Unknown SCORM version"));
        }
        
        // Extract identifier
        if let Some(start) = manifest_content.find("identifier=\"") {
            let start = start + 12;
            if let Some(end) = manifest_content[start..].find("\"") {
                identifier = Some(manifest_content[start..start + end].to_string());
            }
        }
        
        // Extract title
        if let Some(start) = manifest_content.find("<title>") {
            let start = start + 7;
            if let Some(end) = manifest_content[start..].find("</title>") {
                title = Some(manifest_content[start..start + end].to_string());
            }
        }
        
        // Extract launch URL
        if let Some(start) = manifest_content.find("href=\"") {
            let start = start + 6;
            if let Some(end) = manifest_content[start..].find("\"") {
                launch_url = Some(manifest_content[start..start + end].to_string());
            }
        }
        
        // Create package metadata
        let metadata = ScormPackageMetadata {
            id: Uuid::new_v4(),
            title: title.unwrap_or_else(|| "Untitled SCORM Package".to_string()),
            description,
            version: version.unwrap_or(ScormVersion::V1_2),
            identifier: identifier.unwrap_or_else(|| Uuid::new_v4().to_string()),
            package_version,
            author,
            organization,
            duration,
            keywords,
            launch_url: launch_url.unwrap_or_else(|| "index.html".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(metadata)
    }
    
    /// Get a package by ID
    pub fn get_package(&self, id: &Uuid) -> Option<&ScormPackageMetadata> {
        self.packages.get(id)
    }
    
    /// Get all packages
    pub fn get_packages(&self) -> Vec<&ScormPackageMetadata> {
        self.packages.values().collect()
    }
    
    /// Delete a package
    pub fn delete_package(&mut self, id: &Uuid) -> Result<()> {
        // Remove the package from the map
        if self.packages.remove(id).is_none() {
            return Err(anyhow!("Package not found"));
        }
        
        // Delete the package directory
        let package_dir = self.package_dir.join(id.to_string());
        if package_dir.exists() {
            fs::remove_dir_all(package_dir)?;
        }
        
        // Remove any sessions for this package
        self.sessions.retain(|_, session| session.package_id != *id);
        
        Ok(())
    }
    
    /// Create a new session
    pub fn create_session(&mut self, package_id: &Uuid, user_id: &Uuid) -> Result<Uuid> {
        // Check if the package exists
        if !self.packages.contains_key(package_id) {
            return Err(anyhow!("Package not found"));
        }
        
        // Create a new session
        let session = ScormSession {
            id: Uuid::new_v4(),
            package_id: *package_id,
            user_id: *user_id,
            state: ScormActivityState::NotAttempted,
            score: None,
            max_score: None,
            min_score: None,
            total_time: None,
            interactions: Vec::new(),
            data: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        };
        
        // Store the session
        let session_id = session.id;
        self.sessions.insert(session_id, session);
        
        Ok(session_id)
    }
    
    /// Get a session by ID
    pub fn get_session(&self, id: &Uuid) -> Option<&ScormSession> {
        self.sessions.get(id)
    }
    
    /// Get all sessions for a package
    pub fn get_sessions_for_package(&self, package_id: &Uuid) -> Vec<&ScormSession> {
        self.sessions.values()
            .filter(|session| session.package_id == *package_id)
            .collect()
    }
    
    /// Get all sessions for a user
    pub fn get_sessions_for_user(&self, user_id: &Uuid) -> Vec<&ScormSession> {
        self.sessions.values()
            .filter(|session| session.user_id == *user_id)
            .collect()
    }
    
    /// Update a session
    pub fn update_session(&mut self, session: ScormSession) -> Result<()> {
        // Check if the session exists
        if !self.sessions.contains_key(&session.id) {
            return Err(anyhow!("Session not found"));
        }
        
        // Update the session
        self.sessions.insert(session.id, session);
        
        Ok(())
    }
    
    /// Delete a session
    pub fn delete_session(&mut self, id: &Uuid) -> Result<()> {
        // Remove the session from the map
        if self.sessions.remove(id).is_none() {
            return Err(anyhow!("Session not found"));
        }
        
        Ok(())
    }
    
    /// Get the launch URL for a package
    pub fn get_launch_url(&self, package_id: &Uuid) -> Result<String> {
        // Get the package
        let package = self.get_package(package_id)
            .ok_or_else(|| anyhow!("Package not found"))?;
        
        // Build the launch URL
        let launch_url = format!("/scorm/{}/{}", package_id, package.launch_url);
        
        Ok(launch_url)
    }
    
    /// Handle a SCORM API call
    pub fn handle_api_call(&mut self, session_id: &Uuid, function: &str, args: &[&str]) -> Result<String> {
        // Get the session
        let mut session = self.get_session(session_id)
            .ok_or_else(|| anyhow!("Session not found"))?
            .clone();
        
        // Handle the API call
        let result = match function {
            "Initialize" => {
                session.state = ScormActivityState::Attempted;
                session.updated_at = Utc::now();
                "true"
            },
            "Terminate" => {
                if session.state == ScormActivityState::Completed || session.state == ScormActivityState::Passed || session.state == ScormActivityState::Failed {
                    session.completed_at = Some(Utc::now());
                }
                session.updated_at = Utc::now();
                "true"
            },
            "GetValue" => {
                if args.len() != 1 {
                    return Err(anyhow!("Invalid number of arguments for GetValue"));
                }
                
                let key = args[0];
                
                // Handle special keys
                match key {
                    "cmi.core.score.raw" => session.score.map(|s| s.to_string()).unwrap_or_default(),
                    "cmi.core.score.max" => session.max_score.map(|s| s.to_string()).unwrap_or_default(),
                    "cmi.core.score.min" => session.min_score.map(|s| s.to_string()).unwrap_or_default(),
                    "cmi.core.lesson_status" => match session.state {
                        ScormActivityState::NotAttempted => "not attempted",
                        ScormActivityState::Attempted => "attempted",
                        ScormActivityState::Completed => "completed",
                        ScormActivityState::Incomplete => "incomplete",
                        ScormActivityState::Passed => "passed",
                        ScormActivityState::Failed => "failed",
                        ScormActivityState::Unknown => "unknown",
                    },
                    "cmi.core.total_time" => session.total_time.clone().unwrap_or_default(),
                    _ => session.data.get(key).cloned().unwrap_or_default(),
                }
            },
            "SetValue" => {
                if args.len() != 2 {
                    return Err(anyhow!("Invalid number of arguments for SetValue"));
                }
                
                let key = args[0];
                let value = args[1];
                
                // Handle special keys
                match key {
                    "cmi.core.score.raw" => {
                        if let Ok(score) = value.parse::<f32>() {
                            session.score = Some(score);
                        }
                    },
                    "cmi.core.score.max" => {
                        if let Ok(max_score) = value.parse::<f32>() {
                            session.max_score = Some(max_score);
                        }
                    },
                    "cmi.core.score.min" => {
                        if let Ok(min_score) = value.parse::<f32>() {
                            session.min_score = Some(min_score);
                        }
                    },
                    "cmi.core.lesson_status" => {
                        session.state = match value {
                            "not attempted" => ScormActivityState::NotAttempted,
                            "attempted" => ScormActivityState::Attempted,
                            "completed" => ScormActivityState::Completed,
                            "incomplete" => ScormActivityState::Incomplete,
                            "passed" => ScormActivityState::Passed,
                            "failed" => ScormActivityState::Failed,
                            _ => ScormActivityState::Unknown,
                        };
                    },
                    "cmi.core.total_time" => {
                        session.total_time = Some(value.to_string());
                    },
                    _ => {
                        session.data.insert(key.to_string(), value.to_string());
                    },
                }
                
                session.updated_at = Utc::now();
                "true"
            },
            "Commit" => {
                session.updated_at = Utc::now();
                "true"
            },
            "GetLastError" => "0", // No error
            "GetErrorString" => "",
            "GetDiagnostic" => "",
            _ => return Err(anyhow!("Unknown API function: {}", function)),
        }.to_string();
        
        // Update the session
        self.update_session(session)?;
        
        Ok(result)
    }
    
    /// Export a package to a SCORM-compliant ZIP file
    pub fn export_package(&self, package_id: &Uuid, output_path: &Path) -> Result<()> {
        // Get the package
        let package = self.get_package(package_id)
            .ok_or_else(|| anyhow!("Package not found"))?;
        
        // Create a new ZIP file
        let file = File::create(output_path)?;
        let mut zip = zip::ZipWriter::new(file);
        
        // Add the package files
        let package_dir = self.package_dir.join(package_id.to_string());
        self.add_directory_to_zip(&mut zip, &package_dir, "")?;
        
        // Finish the ZIP file
        zip.finish()?;
        
        Ok(())
    }
    
    /// Add a directory to a ZIP file
    fn add_directory_to_zip<W: Write + Seek>(&self, zip: &mut zip::ZipWriter<W>, dir: &Path, prefix: &str) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            let name = if prefix.is_empty() {
                path.file_name().unwrap().to_string_lossy().to_string()
            } else {
                format!("{}/{}", prefix, path.file_name().unwrap().to_string_lossy())
            };
            
            if path.is_dir() {
                zip.add_directory(name, Default::default())?;
                self.add_directory_to_zip(zip, &path, &name)?;
            } else {
                zip.start_file(name, Default::default())?;
                let mut file = File::open(path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            }
        }
        
        Ok(())
    }
}
