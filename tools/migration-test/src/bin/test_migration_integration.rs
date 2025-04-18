use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MigrationStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed(String),
    Skipped(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComponentType {
    React,
    Ember,
    Vue,
    Angular,
    Ruby,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationStats {
    pub total_components: usize,
    pub not_started: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub completion_percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationTracker {
    pub components: HashMap<String, ComponentMetadata>,
    #[serde(skip)]
    file_path: Option<PathBuf>,
    pub started_at: chrono::DateTime<Utc>,
    pub last_updated: chrono::DateTime<Utc>,
    pub stats: MigrationStats,
}

impl Default for MigrationTracker {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            components: HashMap::new(),
            file_path: None,
            started_at: now,
            last_updated: now,
            stats: MigrationStats::default(),
        }
    }
}

impl MigrationTracker {
    pub fn get_progress_string(&self) -> String {
        format!(
            "Migration Progress: {:.1}% ({}/{} components)\n\
             - Not Started: {}\n\
             - In Progress: {}\n\
             - Completed: {}\n\
             - Failed: {}\n\
             - Skipped: {}\n",
            self.stats.completion_percentage,
            self.stats.completed,
            self.stats.total_components,
            self.stats.not_started,
            self.stats.in_progress,
            self.stats.completed,
            self.stats.failed,
            self.stats.skipped
        )
    }
    
    pub fn update_stats(&mut self) {
        let total = self.components.len();
        let not_started = self.components.values().filter(|c| matches!(c.status, MigrationStatus::NotStarted)).count();
        let in_progress = self.components.values().filter(|c| matches!(c.status, MigrationStatus::InProgress)).count();
        let completed = self.components.values().filter(|c| matches!(c.status, MigrationStatus::Completed)).count();
        let failed = self.components.values().filter(|c| matches!(c.status, MigrationStatus::Failed(_))).count();
        let skipped = self.components.values().filter(|c| matches!(c.status, MigrationStatus::Skipped(_))).count();
        
        let completion_percentage = if total > 0 {
            (completed as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        self.stats = MigrationStats {
            total_components: total,
            not_started,
            in_progress,
            completed,
            failed,
            skipped,
            completion_percentage,
        };
    }
    
    pub fn add_component(&mut self, component: ComponentMetadata) {
        self.components.insert(component.id.clone(), component);
        self.update_stats();
    }
    
    pub fn get_components_by_type(&self, component_type: &ComponentType) -> Vec<&ComponentMetadata> {
        self.components
            .values()
            .filter(|c| &c.component_type == component_type)
            .collect()
    }
}

/// Integration for the migration system
pub struct MigrationIntegration {
    /// Migration tracker
    pub tracker: MigrationTracker,
    /// Path to the Canvas source code
    pub canvas_path: PathBuf,
    /// Path to the Discourse source code
    pub discourse_path: PathBuf,
    /// Output directory for generated code
    pub output_dir: PathBuf,
}

impl MigrationIntegration {
    /// Create a new migration integration
    pub fn new(
        base_dir: &PathBuf,
        canvas_path: &str,
        discourse_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Create output directory
        let output_dir = base_dir.join("generated").join("leptos");
        fs::create_dir_all(&output_dir)?;
        
        Ok(Self {
            tracker: MigrationTracker::default(),
            canvas_path: PathBuf::from(canvas_path),
            discourse_path: PathBuf::from(discourse_path),
            output_dir,
        })
    }
    
    /// Initialize the migration process
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing migration process...");
        
        // Add some test components
        let react_component = ComponentMetadata {
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
        
        let ember_component = ComponentMetadata {
            id: "ember1".to_string(),
            name: "EmberComponent".to_string(),
            file_path: "src/components/EmberComponent.js".to_string(),
            component_type: ComponentType::Ember,
            status: MigrationStatus::NotStarted,
            complexity: 8,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        let vue_component = ComponentMetadata {
            id: "vue1".to_string(),
            name: "VueComponent".to_string(),
            file_path: "src/components/VueComponent.vue".to_string(),
            component_type: ComponentType::Vue,
            status: MigrationStatus::NotStarted,
            complexity: 3,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        let angular_component = ComponentMetadata {
            id: "angular1".to_string(),
            name: "AngularComponent".to_string(),
            file_path: "src/components/AngularComponent.ts".to_string(),
            component_type: ComponentType::Angular,
            status: MigrationStatus::NotStarted,
            complexity: 7,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        // Add dependencies
        let react_component2 = ComponentMetadata {
            id: "react2".to_string(),
            name: "ReactComponent2".to_string(),
            file_path: "src/components/ReactComponent2.jsx".to_string(),
            component_type: ComponentType::React,
            status: MigrationStatus::NotStarted,
            complexity: 3,
            dependencies: vec!["react1".to_string()],
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        let ember_component2 = ComponentMetadata {
            id: "ember2".to_string(),
            name: "EmberComponent2".to_string(),
            file_path: "src/components/EmberComponent2.js".to_string(),
            component_type: ComponentType::Ember,
            status: MigrationStatus::NotStarted,
            complexity: 4,
            dependencies: vec!["ember1".to_string()],
            dependents: Vec::new(),
            last_updated: Utc::now(),
            migrated_path: None,
            notes: None,
        };
        
        // Update dependents
        let mut react_component_with_dependents = react_component.clone();
        react_component_with_dependents.dependents = vec!["react2".to_string()];
        
        let mut ember_component_with_dependents = ember_component.clone();
        ember_component_with_dependents.dependents = vec!["ember2".to_string()];
        
        // Add components to tracker
        self.tracker.add_component(react_component_with_dependents);
        self.tracker.add_component(ember_component_with_dependents);
        self.tracker.add_component(vue_component);
        self.tracker.add_component(angular_component);
        self.tracker.add_component(react_component2);
        self.tracker.add_component(ember_component2);
        
        println!("Migration initialized. Found {} components.", self.tracker.components.len());
        println!("{}", self.tracker.get_progress_string());
        
        Ok(())
    }
    
    /// Migrate the next batch of components
    pub fn migrate_next_batch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Migrating next batch of components...");
        
        // Get components to migrate (those with no dependencies)
        let components_to_migrate: Vec<_> = self.tracker.components.values()
            .filter(|c| matches!(c.status, MigrationStatus::NotStarted) && c.dependencies.is_empty())
            .take(2)
            .cloned()
            .collect();
        
        if components_to_migrate.is_empty() {
            println!("No components left to migrate.");
            return Ok(());
        }
        
        println!("Migrating batch of {} components...", components_to_migrate.len());
        
        // Simulate migration of each component
        for component in components_to_migrate {
            println!("Migrating component: {} ({})", component.name, component.file_path);
            
            // Update status to in progress
            if let Some(comp) = self.tracker.components.get_mut(&component.id) {
                comp.status = MigrationStatus::InProgress;
            }
            
            // Simulate migration (50% chance of success)
            let success = rand::random::<bool>();
            
            if success {
                // Update migrated path
                let migrated_path = format!("generated/leptos/components/{}/{}.rs", 
                    match component.component_type {
                        ComponentType::React => "react",
                        ComponentType::Ember => "ember",
                        ComponentType::Vue => "vue",
                        ComponentType::Angular => "angular",
                        ComponentType::Ruby => "ruby",
                        ComponentType::Other(ref s) => s,
                    },
                    component.name.to_lowercase()
                );
                
                if let Some(comp) = self.tracker.components.get_mut(&component.id) {
                    comp.migrated_path = Some(migrated_path);
                    comp.status = MigrationStatus::Completed;
                }
                
                println!("Successfully migrated component: {}", component.name);
            } else {
                // Update status to failed
                if let Some(comp) = self.tracker.components.get_mut(&component.id) {
                    comp.status = MigrationStatus::Failed("Simulated failure".to_string());
                }
                
                println!("Failed to migrate component: {}", component.name);
            }
        }
        
        // Update stats
        self.tracker.update_stats();
        
        println!("Batch migration complete.");
        println!("{}", self.tracker.get_progress_string());
        
        Ok(())
    }
    
    /// Generate a migration report
    pub fn generate_report(&self, base_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating migration report...");
        
        // Generate report
        let report = self.generate_markdown_report();
        
        // Save report to file
        let report_path = base_dir.join("reports").join("migration_report.md");
        fs::create_dir_all(report_path.parent().unwrap())?;
        fs::write(&report_path, report)?;
        
        println!("Migration report generated successfully: {:?}", report_path);
        
        Ok(())
    }
    
    /// Generate a visualization of the migration progress
    pub fn generate_visualization(&self, base_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating migration visualization...");
        
        // Generate HTML report with visualizations
        let html = self.generate_html_report();
        
        // Save HTML report to file
        let html_path = base_dir.join("reports").join("migration_visualization.html");
        fs::create_dir_all(html_path.parent().unwrap())?;
        fs::write(&html_path, html)?;
        
        println!("Migration visualization generated successfully: {:?}", html_path);
        
        Ok(())
    }
    
    /// Generate a markdown report
    fn generate_markdown_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Migration Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now()));
        
        // Add progress summary
        report.push_str("## Progress Summary\n\n");
        report.push_str(&self.tracker.get_progress_string());
        report.push_str("\n");
        
        // Add completed components
        report.push_str("## Completed Components\n\n");
        let completed = self.tracker.components.values()
            .filter(|c| matches!(c.status, MigrationStatus::Completed))
            .collect::<Vec<_>>();
            
        if completed.is_empty() {
            report.push_str("No components have been completed yet.\n\n");
        } else {
            report.push_str("| Component | Type | Original Path | Migrated Path |\n");
            report.push_str("|-----------|------|--------------|---------------|\n");
            
            for component in completed {
                let component_type = match component.component_type {
                    ComponentType::React => "React",
                    ComponentType::Ember => "Ember",
                    ComponentType::Vue => "Vue",
                    ComponentType::Angular => "Angular",
                    ComponentType::Ruby => "Ruby",
                    ComponentType::Other(ref s) => s,
                };
                
                let migrated_path = component.migrated_path.as_deref().unwrap_or("N/A");
                
                report.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    component.name,
                    component_type,
                    component.file_path,
                    migrated_path
                ));
            }
            
            report.push_str("\n");
        }
        
        // Add failed components
        report.push_str("## Failed Components\n\n");
        let failed = self.tracker.components.values()
            .filter(|c| matches!(c.status, MigrationStatus::Failed(_)))
            .collect::<Vec<_>>();
        
        if failed.is_empty() {
            report.push_str("No components have failed migration.\n\n");
        } else {
            report.push_str("| Component | Type | Error |\n");
            report.push_str("|-----------|------|-------|\n");
            
            for component in failed {
                let component_type = match component.component_type {
                    ComponentType::React => "React",
                    ComponentType::Ember => "Ember",
                    ComponentType::Vue => "Vue",
                    ComponentType::Angular => "Angular",
                    ComponentType::Ruby => "Ruby",
                    ComponentType::Other(ref s) => s,
                };
                
                let error = match &component.status {
                    MigrationStatus::Failed(e) => e,
                    _ => "Unknown error",
                };
                
                report.push_str(&format!(
                    "| {} | {} | {} |\n",
                    component.name,
                    component_type,
                    error
                ));
            }
            
            report.push_str("\n");
        }
        
        // Add dependency graph
        report.push_str("## Dependency Graph\n\n");
        report.push_str("```mermaid\ngraph TD;\n");
        
        for (id, component) in &self.tracker.components {
            for dep_id in &component.dependencies {
                if let Some(dep) = self.tracker.components.get(dep_id) {
                    report.push_str(&format!(
                        "    {}[{}] --> {}[{}];\n",
                        id, component.name,
                        dep_id, dep.name
                    ));
                }
            }
        }
        
        report.push_str("```\n\n");
        
        report
    }
    
    /// Generate an HTML report with visualizations
    fn generate_html_report(&self) -> String {
        // Calculate component counts by type
        let react_components = self.tracker.get_components_by_type(&ComponentType::React).len();
        let ember_components = self.tracker.get_components_by_type(&ComponentType::Ember).len();
        let vue_components = self.tracker.get_components_by_type(&ComponentType::Vue).len();
        let angular_components = self.tracker.get_components_by_type(&ComponentType::Angular).len();
        
        // Calculate component counts by status
        let not_started = self.tracker.stats.not_started;
        let in_progress = self.tracker.stats.in_progress;
        let completed = self.tracker.stats.completed;
        let failed = self.tracker.stats.failed;
        let skipped = self.tracker.stats.skipped;
        
        // Generate table rows
        let mut table_rows = String::new();
        for component in self.tracker.components.values() {
            let component_type = match component.component_type {
                ComponentType::React => "React",
                ComponentType::Ember => "Ember",
                ComponentType::Vue => "Vue",
                ComponentType::Angular => "Angular",
                ComponentType::Ruby => "Ruby",
                ComponentType::Other(ref s) => s,
            };
            
            let status = match &component.status {
                MigrationStatus::NotStarted => "Not Started",
                MigrationStatus::InProgress => "In Progress",
                MigrationStatus::Completed => "Completed",
                MigrationStatus::Failed(_) => "Failed",
                MigrationStatus::Skipped(_) => "Skipped",
            };
            
            table_rows.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                component.name,
                component_type,
                status,
                component.complexity,
                component.dependencies.len(),
                component.dependents.len(),
            ));
        }
        
        // Generate HTML
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Migration Visualization</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .container {{ display: flex; flex-wrap: wrap; }}
        .chart-container {{ width: 500px; height: 400px; margin: 20px; }}
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
    <h1>Migration Visualization</h1>
    
    <div class="stats">
        <h2>Migration Progress</h2>
        <div class="progress-bar">
            <div class="progress"></div>
        </div>
        <p>{:.1}% Complete ({} out of {} components)</p>
    </div>
    
    <div class="container">
        <div class="chart-container">
            <h2>Components by Type</h2>
            <canvas id="typeChart"></canvas>
        </div>
        
        <div class="chart-container">
            <h2>Migration Status</h2>
            <canvas id="statusChart"></canvas>
        </div>
    </div>
    
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
        // Components by Type Chart
        const typeCtx = document.getElementById('typeChart').getContext('2d');
        const typeChart = new Chart(typeCtx, {{
            type: 'pie',
            data: {{
                labels: ['React', 'Ember', 'Vue', 'Angular'],
                datasets: [{{
                    data: [{}, {}, {}, {}],
                    backgroundColor: [
                        'rgba(54, 162, 235, 0.7)',
                        'rgba(255, 99, 132, 0.7)',
                        'rgba(75, 192, 192, 0.7)',
                        'rgba(255, 159, 64, 0.7)'
                    ],
                    borderColor: [
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 99, 132, 1)',
                        'rgba(75, 192, 192, 1)',
                        'rgba(255, 159, 64, 1)'
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
                        text: 'Components by Type'
                    }}
                }}
            }}
        }});
        
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
    </script>
</body>
</html>
"#,
            self.tracker.stats.completion_percentage,
            self.tracker.stats.completion_percentage,
            self.tracker.stats.completed,
            self.tracker.stats.total_components,
            react_components,
            ember_components,
            vue_components,
            angular_components,
            not_started,
            in_progress,
            completed,
            failed,
            skipped,
            table_rows = table_rows,
        );
        
        html
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Migration Integration");
    
    // Create base directory
    let base_dir = PathBuf::from("test_output");
    fs::create_dir_all(&base_dir)?;
    
    // Create migration integration
    let mut migration_integration = MigrationIntegration::new(
        &base_dir,
        "C:\\Users\\Tim\\Desktop\\port\\canvas",
        "C:\\Users\\Tim\\Desktop\\port\\discourse",
    )?;
    
    // Initialize migration
    migration_integration.initialize()?;
    
    // Migrate first batch
    migration_integration.migrate_next_batch()?;
    
    // Generate report
    migration_integration.generate_report(&base_dir)?;
    
    // Generate visualization
    migration_integration.generate_visualization(&base_dir)?;
    
    println!("Migration integration test completed successfully!");
    println!("Report saved to test_output/reports/migration_report.md");
    println!("Visualization saved to test_output/reports/migration_visualization.html");
    
    Ok(())
}
