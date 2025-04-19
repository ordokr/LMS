// cmi5 Course Structure
//
// This module provides functionality for parsing and managing cmi5 course structures.

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use uuid::Uuid;
use zip::ZipArchive;
use tracing::{info, error, debug};
use crate::quiz::cmi5::models::{Cmi5Course, AssignableUnit, EntryMode, MoveOn, PassingScoreMethod, Objective};

/// Course structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseStructure {
    /// Course ID
    pub id: String,
    
    /// Course title
    pub title: String,
    
    /// Course description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Course language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    
    /// Course version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    
    /// Course publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    
    /// Assignable units
    #[serde(rename = "au")]
    pub assignable_units: Vec<CourseAssignableUnit>,
}

/// Course assignable unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseAssignableUnit {
    /// ID
    pub id: String,
    
    /// Launch URL
    #[serde(rename = "url")]
    pub url: String,
    
    /// Title
    pub title: String,
    
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Launch parameters
    #[serde(rename = "launchParameters", skip_serializing_if = "Option::is_none")]
    pub launch_parameters: Option<String>,
    
    /// Move on
    #[serde(rename = "moveOn")]
    pub move_on: String,
    
    /// Mastery score
    #[serde(rename = "masteryScore", skip_serializing_if = "Option::is_none")]
    pub mastery_score: Option<f64>,
    
    /// Passing score method
    #[serde(rename = "passingScoreMethod", skip_serializing_if = "Option::is_none")]
    pub passing_score_method: Option<String>,
    
    /// Max attempts
    #[serde(rename = "maxAttempts", skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
    
    /// Activity type
    #[serde(rename = "activityType", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
}

/// Parse a course structure from a cmi5 package
pub async fn parse_course_structure(package_path: &Path) -> Result<Cmi5Course> {
    info!("Parsing course structure from: {}", package_path.display());
    
    // Open the package file
    let file = File::open(package_path)
        .map_err(|e| anyhow!("Failed to open package file: {}", e))?;
    
    // Create a ZIP archive
    let mut archive = ZipArchive::new(file)
        .map_err(|e| anyhow!("Failed to read ZIP archive: {}", e))?;
    
    // Find the cmi5.xml file
    let mut cmi5_xml = None;
    for i in 0..archive.len() {
        let file = archive.by_index(i)
            .map_err(|e| anyhow!("Failed to read file in archive: {}", e))?;
        
        let name = file.name();
        if name.ends_with("cmi5.xml") {
            cmi5_xml = Some(i);
            break;
        }
    }
    
    let cmi5_xml_index = cmi5_xml.ok_or_else(|| anyhow!("cmi5.xml not found in package"))?;
    
    // Read the cmi5.xml file
    let mut cmi5_xml_file = archive.by_index(cmi5_xml_index)
        .map_err(|e| anyhow!("Failed to read cmi5.xml: {}", e))?;
    
    let mut cmi5_xml_content = String::new();
    cmi5_xml_file.read_to_string(&mut cmi5_xml_content)
        .map_err(|e| anyhow!("Failed to read cmi5.xml content: {}", e))?;
    
    // Parse the XML
    let course_structure: CourseStructure = serde_xml_rs::from_str(&cmi5_xml_content)
        .map_err(|e| anyhow!("Failed to parse cmi5.xml: {}", e))?;
    
    // Convert to Cmi5Course
    let assignable_units = course_structure.assignable_units.iter()
        .map(|au| {
            // Parse move on
            let move_on = match au.move_on.as_str() {
                "Completed" => MoveOn::Completed,
                "Passed" => MoveOn::Passed,
                "CompletedAndPassed" => MoveOn::CompletedAndPassed,
                "CompletedOrPassed" => MoveOn::CompletedOrPassed,
                "NotApplicable" => MoveOn::NotApplicable,
                _ => MoveOn::NotApplicable,
            };
            
            // Parse passing score method
            let passing_score_method = au.passing_score_method.as_ref().map(|method| {
                match method.as_str() {
                    "percentage" => PassingScoreMethod::Percentage,
                    "points" => PassingScoreMethod::Points,
                    "scaled" => PassingScoreMethod::Scaled,
                    _ => PassingScoreMethod::Scaled,
                }
            });
            
            AssignableUnit {
                id: au.id.clone(),
                title: au.title.clone(),
                description: au.description.clone(),
                launch_url: au.url.clone(),
                launch_parameters: au.launch_parameters.clone(),
                entry_mode: EntryMode::Normal,
                move_on,
                mastery_score: au.mastery_score,
                passing_score_method,
                max_attempts: au.max_attempts,
                activity_type: au.activity_type.clone(),
            }
        })
        .collect();
    
    let course = Cmi5Course {
        id: course_structure.id,
        title: course_structure.title,
        description: course_structure.description,
        language: course_structure.language,
        version: course_structure.version,
        publisher: course_structure.publisher,
        assignable_units,
        objectives: None,
    };
    
    info!("Successfully parsed course structure: {}", course.title);
    Ok(course)
}

/// Extract a cmi5 package
pub async fn extract_package(package_path: &Path, output_dir: &Path) -> Result<PathBuf> {
    info!("Extracting package from: {} to: {}", package_path.display(), output_dir.display());
    
    // Open the package file
    let file = File::open(package_path)
        .map_err(|e| anyhow!("Failed to open package file: {}", e))?;
    
    // Create a ZIP archive
    let mut archive = ZipArchive::new(file)
        .map_err(|e| anyhow!("Failed to read ZIP archive: {}", e))?;
    
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)
        .map_err(|e| anyhow!("Failed to create output directory: {}", e))?;
    
    // Extract the archive
    archive.extract(output_dir)
        .map_err(|e| anyhow!("Failed to extract archive: {}", e))?;
    
    // Find the cmi5.xml file
    let mut cmi5_xml_path = None;
    for entry in std::fs::read_dir(output_dir)
        .map_err(|e| anyhow!("Failed to read output directory: {}", e))? {
        let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() && path.file_name().unwrap_or_default().to_string_lossy().ends_with("cmi5.xml") {
            cmi5_xml_path = Some(path);
            break;
        }
    }
    
    let cmi5_xml_path = cmi5_xml_path.ok_or_else(|| anyhow!("cmi5.xml not found in extracted package"))?;
    
    info!("Successfully extracted package to: {}", output_dir.display());
    Ok(cmi5_xml_path)
}

/// Create a cmi5 package
pub async fn create_package(course: &Cmi5Course, output_path: &Path) -> Result<()> {
    info!("Creating cmi5 package at: {}", output_path.display());
    
    // Create the output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow!("Failed to create output directory: {}", e))?;
    }
    
    // Create the ZIP file
    let file = File::create(output_path)
        .map_err(|e| anyhow!("Failed to create package file: {}", e))?;
    
    let mut zip = zip::ZipWriter::new(file);
    
    // Create the course structure
    let assignable_units = course.assignable_units.iter()
        .map(|au| {
            // Convert move on
            let move_on = match au.move_on {
                MoveOn::Completed => "Completed",
                MoveOn::Passed => "Passed",
                MoveOn::CompletedAndPassed => "CompletedAndPassed",
                MoveOn::CompletedOrPassed => "CompletedOrPassed",
                MoveOn::NotApplicable => "NotApplicable",
            };
            
            // Convert passing score method
            let passing_score_method = au.passing_score_method.map(|method| {
                match method {
                    PassingScoreMethod::Percentage => "percentage",
                    PassingScoreMethod::Points => "points",
                    PassingScoreMethod::Scaled => "scaled",
                }
            });
            
            CourseAssignableUnit {
                id: au.id.clone(),
                url: au.launch_url.clone(),
                title: au.title.clone(),
                description: au.description.clone(),
                launch_parameters: au.launch_parameters.clone(),
                move_on: move_on.to_string(),
                mastery_score: au.mastery_score,
                passing_score_method: passing_score_method.map(String::from),
                max_attempts: au.max_attempts,
                activity_type: au.activity_type.clone(),
            }
        })
        .collect();
    
    let course_structure = CourseStructure {
        id: course.id.clone(),
        title: course.title.clone(),
        description: course.description.clone(),
        language: course.language.clone(),
        version: course.version.clone(),
        publisher: course.publisher.clone(),
        assignable_units,
    };
    
    // Serialize the course structure to XML
    let course_xml = serde_xml_rs::to_string(&course_structure)
        .map_err(|e| anyhow!("Failed to serialize course structure: {}", e))?;
    
    // Add the cmi5.xml file to the ZIP
    zip.start_file("cmi5.xml", zip::write::FileOptions::default())
        .map_err(|e| anyhow!("Failed to create cmi5.xml in package: {}", e))?;
    
    zip.write_all(course_xml.as_bytes())
        .map_err(|e| anyhow!("Failed to write cmi5.xml to package: {}", e))?;
    
    // Finish the ZIP file
    zip.finish()
        .map_err(|e| anyhow!("Failed to finalize package: {}", e))?;
    
    info!("Successfully created cmi5 package at: {}", output_path.display());
    Ok(())
}
