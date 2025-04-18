use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed(String),
    Skipped(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentType {
    React,
    Ember,
    Vue,
    Angular,
    Ruby,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct ComponentMetadata {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub component_type: ComponentType,
    pub status: MigrationStatus,
    pub complexity: u32,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub last_updated: chrono::DateTime<Utc>,
    pub migrated_path: Option<String>,
    pub notes: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Simple Migration");
    
    // Create output directory
    let output_dir = PathBuf::from("test_output");
    fs::create_dir_all(&output_dir)?;
    
    // Create a simple component
    let component = ComponentMetadata {
        id: "react1".to_string(),
        name: "ReactComponent".to_string(),
        file_path: "src/components/ReactComponent.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::NotStarted,
        complexity: 5,
        dependencies: Vec::new(),
        dependents: Vec::new(),
        last_updated: Utc::now(),
        migrated_path: None,
        notes: None,
    };
    
    // Print component details
    println!("Component: {:?}", component);
    
    // Generate a simple report
    let report = generate_simple_report(&component);
    
    // Save report to file
    let report_path = output_dir.join("simple_migration_report.md");
    fs::write(&report_path, report)?;
    
    println!("Simple migration report generated successfully: {:?}", report_path);
    
    Ok(())
}

fn generate_simple_report(component: &ComponentMetadata) -> String {
    let mut report = String::new();
    
    report.push_str("# Simple Migration Report\n\n");
    report.push_str(&format!("Generated: {}\n\n", Utc::now()));
    
    // Add component details
    report.push_str("## Component Details\n\n");
    report.push_str(&format!("- Name: {}\n", component.name));
    report.push_str(&format!("- Type: {:?}\n", component.component_type));
    report.push_str(&format!("- Path: {}\n", component.file_path));
    report.push_str(&format!("- Status: {:?}\n", component.status));
    report.push_str(&format!("- Complexity: {}\n", component.complexity));
    
    report
}
