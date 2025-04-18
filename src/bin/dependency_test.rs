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
    println!("Testing Dependency Detection");
    
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
        dependents: vec!["form".to_string(), "navbar".to_string()],
        last_updated: Utc::now(),
        migrated_path: Some("generated/leptos/components/Button.rs".to_string()),
        notes: None,
    };
    
    let input = ComponentMetadata {
        id: "input".to_string(),
        name: "Input".to_string(),
        file_path: "src/components/Input.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::Completed,
        complexity: 3,
        dependencies: Vec::new(),
        dependents: vec!["form".to_string()],
        last_updated: Utc::now(),
        migrated_path: Some("generated/leptos/components/Input.rs".to_string()),
        notes: None,
    };
    
    // Create components that depend on base components
    let form = ComponentMetadata {
        id: "form".to_string(),
        name: "Form".to_string(),
        file_path: "src/components/Form.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::InProgress,
        complexity: 5,
        dependencies: vec!["button".to_string(), "input".to_string()],
        dependents: vec!["login".to_string(), "signup".to_string()],
        last_updated: Utc::now(),
        migrated_path: None,
        notes: Some("Working on validation logic".to_string()),
    };
    
    let navbar = ComponentMetadata {
        id: "navbar".to_string(),
        name: "Navbar".to_string(),
        file_path: "src/components/Navbar.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::NotStarted,
        complexity: 4,
        dependencies: vec!["button".to_string()],
        dependents: vec!["app".to_string()],
        last_updated: Utc::now(),
        migrated_path: None,
        notes: None,
    };
    
    // Create top-level components
    let login = ComponentMetadata {
        id: "login".to_string(),
        name: "Login".to_string(),
        file_path: "src/pages/Login.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::NotStarted,
        complexity: 6,
        dependencies: vec!["form".to_string()],
        dependents: vec!["app".to_string()],
        last_updated: Utc::now(),
        migrated_path: None,
        notes: None,
    };
    
    let signup = ComponentMetadata {
        id: "signup".to_string(),
        name: "Signup".to_string(),
        file_path: "src/pages/Signup.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::NotStarted,
        complexity: 7,
        dependencies: vec!["form".to_string()],
        dependents: vec!["app".to_string()],
        last_updated: Utc::now(),
        migrated_path: None,
        notes: None,
    };
    
    let app = ComponentMetadata {
        id: "app".to_string(),
        name: "App".to_string(),
        file_path: "src/App.jsx".to_string(),
        component_type: ComponentType::React,
        status: MigrationStatus::NotStarted,
        complexity: 8,
        dependencies: vec!["navbar".to_string(), "login".to_string(), "signup".to_string()],
        dependents: Vec::new(),
        last_updated: Utc::now(),
        migrated_path: None,
        notes: None,
    };
    
    // Add components to map
    components.insert(button.id.clone(), button);
    components.insert(input.id.clone(), input);
    components.insert(form.id.clone(), form);
    components.insert(navbar.id.clone(), navbar);
    components.insert(login.id.clone(), login);
    components.insert(signup.id.clone(), signup);
    components.insert(app.id.clone(), app);
    
    // Generate dependency report
    let report = generate_dependency_report(&components);
    
    // Save report to file
    let report_path = output_dir.join("dependency_report.md");
    fs::write(&report_path, report)?;
    
    println!("Dependency report generated successfully: {:?}", report_path);
    
    // Generate HTML visualization
    let html = generate_html_visualization(&components);
    
    // Save HTML to file
    let html_path = output_dir.join("dependency_visualization.html");
    fs::write(&html_path, html)?;
    
    println!("Dependency visualization generated successfully: {:?}", html_path);
    
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
    
    // Add migration order recommendation
    report.push_str("## Recommended Migration Order\n\n");
    
    // Simple topological sort (not handling cycles)
    let mut migration_order = Vec::new();
    let mut visited = HashMap::new();
    
    for id in components.keys() {
        if !visited.contains_key(id) {
            visit(id, components, &mut visited, &mut migration_order);
        }
    }
    
    for (i, id) in migration_order.iter().enumerate() {
        if let Some(component) = components.get(id) {
            report.push_str(&format!("{}. {} ({})\n", i + 1, component.name, component.file_path));
        }
    }
    
    report
}

fn visit(
    id: &str,
    components: &HashMap<String, ComponentMetadata>,
    visited: &mut HashMap<String, bool>,
    order: &mut Vec<String>
) {
    visited.insert(id.to_string(), true);
    
    if let Some(component) = components.get(id) {
        for dep_id in &component.dependencies {
            if !visited.contains_key(dep_id) {
                visit(dep_id, components, visited, order);
            }
        }
    }
    
    order.push(id.to_string());
}

fn generate_html_visualization(components: &HashMap<String, ComponentMetadata>) -> String {
    // Calculate component counts by status
    let completed = components.values().filter(|c| matches!(c.status, MigrationStatus::Completed)).count();
    let in_progress = components.values().filter(|c| matches!(c.status, MigrationStatus::InProgress)).count();
    let not_started = components.values().filter(|c| matches!(c.status, MigrationStatus::NotStarted)).count();
    let failed = components.values().filter(|c| matches!(c.status, MigrationStatus::Failed(_))).count();
    let skipped = components.values().filter(|c| matches!(c.status, MigrationStatus::Skipped(_))).count();
    
    // Generate table rows
    let mut table_rows = String::new();
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
        
        table_rows.push_str(&format!(
            "<tr><td>{}</td><td>{:?}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            component.name,
            component.component_type,
            status,
            component.complexity,
            dependencies,
            dependents
        ));
    }
    
    // Generate nodes and edges for network graph
    let mut nodes = String::new();
    let mut edges = String::new();
    
    for (i, component) in components.values().enumerate() {
        // Add node
        let color = match component.status {
            MigrationStatus::NotStarted => "#C9CBCF",
            MigrationStatus::InProgress => "#FFCD56",
            MigrationStatus::Completed => "#4BC0C0",
            MigrationStatus::Failed(_) => "#FF6384",
            MigrationStatus::Skipped(_) => "#9966FF",
        };
        
        nodes.push_str(&format!(
            "        {{ id: {}, label: '{}', color: '{}' }},\n",
            i, component.name, color
        ));
        
        // Add edges
        for dep_id in &component.dependencies {
            if let Some(dep) = components.get(dep_id) {
                if let Some(dep_index) = components.values().position(|c| c.id == dep.id) {
                    edges.push_str(&format!(
                        "        {{ from: {}, to: {} }},\n",
                        i, dep_index
                    ));
                }
            }
        }
    }
    
    // Generate HTML
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Component Dependency Visualization</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/vis-network@9.1.2/dist/vis-network.min.js"></script>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .container {{ display: flex; flex-wrap: wrap; }}
        .chart-container {{ width: 500px; height: 400px; margin: 20px; }}
        .network-container {{ width: 100%; height: 600px; margin: 20px; }}
        h1, h2 {{ color: #333; }}
        .stats {{ margin: 20px; }}
        .progress-bar {{ 
            height: 30px; 
            background-color: #f0f0f0; 
            border-radius: 5px; 
            margin: 10px 0; 
        }}
        .progress {{ 
            height: 100%; 
            background-color: #4CAF50; 
            border-radius: 5px; 
            width: {:.1}%; 
        }}
        table {{ border-collapse: collapse; width: 100%; margin: 20px 0; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
    </style>
</head>
<body>
    <h1>Component Dependency Visualization</h1>
    
    <div class="stats">
        <h2>Migration Progress</h2>
        <div class="progress-bar">
            <div class="progress"></div>
        </div>
        <p>{:.1}% Complete ({} out of {} components)</p>
    </div>
    
    <div class="container">
        <div class="chart-container">
            <h2>Migration Status</h2>
            <canvas id="statusChart"></canvas>
        </div>
        
        <div class="chart-container">
            <h2>Complexity Distribution</h2>
            <canvas id="complexityChart"></canvas>
        </div>
    </div>
    
    <h2>Dependency Network</h2>
    <div class="network-container" id="network"></div>
    
    <h2>Component Details</h2>
    <table>
        <tr>
            <th>Component</th>
            <th>Type</th>
            <th>Status</th>
            <th>Complexity</th>
            <th>Dependencies</th>
            <th>Dependents</th>
        </tr>
        {table_rows}
    </table>
    
    <script>
        // Migration Status Chart
        const statusCtx = document.getElementById('statusChart').getContext('2d');
        const statusChart = new Chart(statusCtx, {{
            type: 'doughnut',
            data: {{
                labels: ['Not Started', 'In Progress', 'Completed', 'Failed', 'Skipped'],
                datasets: [{{
                    data: [{}, {}, {}, {}, {}],
                    backgroundColor: [
                        'rgba(201, 203, 207, 0.7)',
                        'rgba(255, 205, 86, 0.7)',
                        'rgba(75, 192, 192, 0.7)',
                        'rgba(255, 99, 132, 0.7)',
                        'rgba(153, 102, 255, 0.7)'
                    ],
                    borderColor: [
                        'rgb(201, 203, 207)',
                        'rgb(255, 205, 86)',
                        'rgb(75, 192, 192)',
                        'rgb(255, 99, 132)',
                        'rgb(153, 102, 255)'
                    ],
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    legend: {{
                        position: 'right',
                    }},
                    title: {{
                        display: true,
                        text: 'Migration Status'
                    }}
                }}
            }}
        }});
        
        // Complexity Chart
        const complexityCtx = document.getElementById('complexityChart').getContext('2d');
        const complexityData = [
            {complexities}
        ];
        const complexityChart = new Chart(complexityCtx, {{
            type: 'bar',
            data: {{
                labels: ['1-2', '3-4', '5-6', '7-8', '9+'],
                datasets: [{{
                    label: 'Components',
                    data: [
                        complexityData.filter(c => c <= 2).length,
                        complexityData.filter(c => c > 2 && c <= 4).length,
                        complexityData.filter(c => c > 4 && c <= 6).length,
                        complexityData.filter(c => c > 6 && c <= 8).length,
                        complexityData.filter(c => c > 8).length
                    ],
                    backgroundColor: 'rgba(54, 162, 235, 0.7)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true,
                        ticks: {{
                            stepSize: 1
                        }}
                    }}
                }}
            }}
        }});
        
        // Network Visualization
        const nodes = [
{nodes}
        ];
        
        const edges = [
{edges}
        ];
        
        const container = document.getElementById('network');
        const data = {{
            nodes: new vis.DataSet(nodes),
            edges: new vis.DataSet(edges)
        }};
        const options = {{
            nodes: {{
                shape: 'dot',
                size: 16,
                font: {{
                    size: 12
                }}
            }},
            edges: {{
                arrows: 'to',
                smooth: {{
                    type: 'cubicBezier',
                    forceDirection: 'vertical',
                    roundness: 0.4
                }}
            }},
            layout: {{
                hierarchical: {{
                    direction: 'UD',
                    sortMethod: 'directed',
                    levelSeparation: 150,
                    nodeSpacing: 150
                }}
            }},
            physics: false
        }};
        const network = new vis.Network(container, data, options);
    </script>
</body>
</html>
"#,
        (completed as f32 / components.len() as f32) * 100.0,
        (completed as f32 / components.len() as f32) * 100.0,
        completed,
        components.len(),
        table_rows = table_rows,
        not_started,
        in_progress,
        completed,
        failed,
        skipped,
        complexities = components.values().map(|c| c.complexity.to_string()).collect::<Vec<_>>().join(", "),
        nodes = nodes,
        edges = edges
    )
}
