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
    println!("Testing Simple Dependency Detection");
    
    // Create output directory
    let output_dir = PathBuf::from("test_output");
    fs::create_dir_all(&output_dir)?;
    
    // Create components with dependencies
    let mut components = HashMap::new();
    
    // Create base components
    let button = ComponentMetadata {
        id: "button".to_string(),
        name: "Button".to_string(),
        file_path: "src/components/Button.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::Completed,
        complexity: 2,
        dependencies: Vec::new(),
        dependents: vec!["form".to_string()],
        last_updated: Utc::now(),
        migrated_path: Some("generated/leptos/components/Button.rs".to_string()),
        notes: None,
    };
    
    let form = ComponentMetadata {
        id: "form".to_string(),
        name: "Form".to_string(),
        file_path: "src/components/Form.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::InProgress,
        complexity: 5,
        dependencies: vec!["button".to_string()],
        dependents: Vec::new(),
        last_updated: Utc::now(),
        migrated_path: None,
        notes: Some("Working on validation logic".to_string()),
    };
    
    // Add components to map
    components.insert(button.id.clone(), button);
    components.insert(form.id.clone(), form);
    
    // Generate dependency report
    let report = generate_dependency_report(&components);
    
    // Save report to file
    let report_path = output_dir.join("simple_dependency_report.md");
    fs::write(&report_path, report)?;
    
    println!("Simple dependency report generated successfully: {:?}", report_path);
    
    // Generate HTML visualization
    let html = generate_html_visualization(&components);
    
    // Save HTML to file
    let html_path = output_dir.join("simple_dependency_visualization.html");
    fs::write(&html_path, html)?;
    
    println!("Simple dependency visualization generated successfully: {:?}", html_path);
    
    Ok(())
}

fn generate_dependency_report(components: &HashMap<String, ComponentMetadata>) -> String {
    let mut report = String::new();
    
    report.push_str("# Component Dependency Report\n\n");
    report.push_str(&format!("Generated: {}\n\n", Utc::now()));
    
    // Add migration progress
    let total = components.len();
    let completed = components.values().filter(|c| matches!(c.status, MigrationStatus::Completed)).count();
    let in_progress = components.values().filter(|c| matches!(c.status, MigrationStatus::InProgress)).count();
    let not_started = components.values().filter(|c| matches!(c.status, MigrationStatus::NotStarted)).count();
    
    let completion_percentage = (completed as f32 / total as f32) * 100.0;
    
    report.push_str("## Migration Progress\n\n");
    report.push_str(&format!("- Total Components: {}\n", total));
    report.push_str(&format!("- Completed: {} ({:.1}%)\n", completed, completion_percentage));
    report.push_str(&format!("- In Progress: {}\n", in_progress));
    report.push_str(&format!("- Not Started: {}\n\n", not_started));
    
    // Add component details
    report.push_str("## Component Details\n\n");
    report.push_str("| Component | Type | Status | Complexity | Dependencies | Dependents |\n");
    report.push_str("|-----------|------|--------|------------|--------------|------------|\n");
    
    for component in components.values() {
        let status = match &component.status {
            MigrationStatus::NotStarted => "Not Started",
            MigrationStatus::InProgress => "In Progress",
            MigrationStatus::Completed => "Completed",
            MigrationStatus::Failed(e) => &format!("Failed: {}", e),
            MigrationStatus::Skipped(r) => &format!("Skipped: {}", r),
        };
        
        let dependencies = component.dependencies.iter()
            .map(|id| components.get(id).map_or(id.clone(), |c| c.name.clone()))
            .collect::<Vec<_>>()
            .join(", ");
        
        let dependents = component.dependents.iter()
            .map(|id| components.get(id).map_or(id.clone(), |c| c.name.clone()))
            .collect::<Vec<_>>()
            .join(", ");
        
        report.push_str(&format!(
            "| {} | {:?} | {} | {} | {} | {} |\n",
            component.name,
            component.component_type,
            status,
            component.complexity,
            dependencies,
            dependents
        ));
    }
    
    report.push_str("\n");
    
    // Add dependency graph
    report.push_str("## Dependency Graph\n\n");
    report.push_str("```mermaid\ngraph TD;\n");
    
    for component in components.values() {
        for dep_id in &component.dependencies {
            if let Some(dep) = components.get(dep_id) {
                report.push_str(&format!(
                    "    {}[{}] --> {}[{}];\n",
                    component.id, component.name,
                    dep.id, dep.name
                ));
            }
        }
    }
    
    report.push_str("```\n\n");
    
    report
}

fn generate_html_visualization(components: &HashMap<String, ComponentMetadata>) -> String {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Simple Dependency Visualization</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1, h2 {{ color: #333; }}
        .component {{
            border: 1px solid #ddd;
            border-radius: 5px;
            padding: 10px;
            margin: 10px;
            width: 300px;
        }}
        .completed {{ background-color: #d4edda; }}
        .in-progress {{ background-color: #fff3cd; }}
        .not-started {{ background-color: #f8f9fa; }}
        .dependency {{
            margin-top: 10px;
            padding: 5px;
            background-color: #e9ecef;
            border-radius: 3px;
        }}
    </style>
</head>
<body>
    <h1>Simple Dependency Visualization</h1>
    
    <h2>Components</h2>
    <div class="components">
        {components_html}
    </div>
</body>
</html>
"#,
        components_html = components.values().map(|component| {
            let status_class = match component.status {
                MigrationStatus::Completed => "completed",
                MigrationStatus::InProgress => "in-progress",
                MigrationStatus::NotStarted => "not-started",
                _ => "not-started",
            };
            
            let dependencies_html = if component.dependencies.is_empty() {
                "None".to_string()
            } else {
                component.dependencies.iter()
                    .map(|id| {
                        if let Some(dep) = components.get(id) {
                            format!("<div class=\"dependency\">{} ({})</div>", dep.name, dep.id)
                        } else {
                            format!("<div class=\"dependency\">{} (unknown)</div>", id)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            
            format!(
                r#"<div class="component {status_class}">
    <h3>{name} ({id})</h3>
    <p><strong>Type:</strong> {type:?}</p>
    <p><strong>Status:</strong> {status:?}</p>
    <p><strong>Complexity:</strong> {complexity}</p>
    <p><strong>Dependencies:</strong></p>
    {dependencies_html}
</div>"#,
                status_class = status_class,
                name = component.name,
                id = component.id,
                type = component.component_type,
                status = component.status,
                complexity = component.complexity,
                dependencies_html = dependencies_html
            )
        }).collect::<Vec<_>>().join("\n")
    );
    
    html
}
