use super::migration_manager::{MigrationManager, MigrationConfig};
use super::migration_tracker::{ComponentType, MigrationStatus};
use super::component_prioritizer::PrioritizationFactors;
use crate::analyzers::modules::{
    enhanced_react_analyzer::EnhancedReactAnalyzer,
    enhanced_ember_analyzer::EnhancedEmberAnalyzer,
    enhanced_vue_analyzer::EnhancedVueAnalyzer,
    enhanced_angular_analyzer::EnhancedAngularAnalyzer,
};
use std::path::{Path, PathBuf};
use std::fs;

/// Integration for the migration system with the unified analyzer
pub struct MigrationIntegration {
    /// Migration manager
    pub migration_manager: MigrationManager,
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
        base_dir: &Path,
        canvas_path: &str,
        discourse_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Create output directory
        let output_dir = base_dir.join("generated").join("leptos");
        fs::create_dir_all(&output_dir)?;
        
        // Create migration config
        let migration_config = MigrationConfig {
            tracker_file_path: base_dir.join("migration_tracker.json"),
            output_dir: output_dir.clone(),
            source_dirs: vec![
                PathBuf::from(canvas_path),
                PathBuf::from(discourse_path),
            ],
            auto_detect_dependencies: true,
            skip_on_error: true,
            batch_size: 5,
            prioritization_factors: PrioritizationFactors::default(),
        };
        
        // Initialize migration manager
        let migration_manager = MigrationManager::new(migration_config)?;
        
        Ok(Self {
            migration_manager,
            canvas_path: PathBuf::from(canvas_path),
            discourse_path: PathBuf::from(discourse_path),
            output_dir,
        })
    }
    
    /// Initialize the migration process
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing migration process...");
        
        // Initialize migration
        self.migration_manager.initialize()?;
        
        println!("Migration initialized successfully.");
        println!("{}", self.migration_manager.tracker.get_progress_string());
        
        Ok(())
    }
    
    /// Migrate the next batch of components
    pub fn migrate_next_batch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Migrating next batch of components...");
        
        // Migrate next batch
        self.migration_manager.migrate_batch()?;
        
        println!("Batch migration completed successfully.");
        println!("{}", self.migration_manager.tracker.get_progress_string());
        
        Ok(())
    }
    
    /// Generate a migration report
    pub fn generate_report(&self, base_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating migration report...");
        
        // Generate report
        let report = self.migration_manager.generate_report();
        
        // Save report to file
        let report_path = base_dir.join("docs").join("migration_report.md");
        fs::create_dir_all(report_path.parent().unwrap())?;
        fs::write(&report_path, report)?;
        
        println!("Migration report generated successfully: {:?}", report_path);
        
        Ok(())
    }
    
    /// Generate a visualization of the migration progress
    pub fn generate_visualization(&self, base_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating migration visualization...");
        
        // Generate HTML report with visualizations
        let html = self.generate_html_report();
        
        // Save HTML report to file
        let html_path = base_dir.join("docs").join("migration_visualization.html");
        fs::create_dir_all(html_path.parent().unwrap())?;
        fs::write(&html_path, html)?;
        
        println!("Migration visualization generated successfully: {:?}", html_path);
        
        Ok(())
    }
    
    /// Generate an HTML report with visualizations
    fn generate_html_report(&self) -> String {
        let tracker = &self.migration_manager.tracker;
        
        // Calculate component counts by type
        let react_components = tracker.get_components_by_type(&ComponentType::React).len();
        let ember_components = tracker.get_components_by_type(&ComponentType::Ember).len();
        let vue_components = tracker.get_components_by_type(&ComponentType::Vue).len();
        let angular_components = tracker.get_components_by_type(&ComponentType::Angular).len();
        
        // Calculate component counts by status
        let not_started = tracker.stats.not_started;
        let in_progress = tracker.stats.in_progress;
        let completed = tracker.stats.completed;
        let failed = tracker.stats.failed;
        let skipped = tracker.stats.skipped;
        
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
            tracker.stats.completion_percentage,
            tracker.stats.completion_percentage,
            tracker.stats.completed,
            tracker.stats.total_components,
            react_components,
            ember_components,
            vue_components,
            angular_components,
            not_started,
            in_progress,
            completed,
            failed,
            skipped,
            table_rows = self.generate_table_rows(),
        );
        
        html
    }
    
    /// Generate table rows for the component details table
    fn generate_table_rows(&self) -> String {
        let mut rows = String::new();
        
        for component in self.migration_manager.tracker.components.values() {
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
                MigrationStatus::Failed(e) => "Failed",
                MigrationStatus::Skipped(e) => "Skipped",
            };
            
            rows.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                component.name,
                component_type,
                status,
                component.complexity,
                component.dependencies.len(),
                component.dependents.len(),
            ));
        }
        
        rows
    }
}
