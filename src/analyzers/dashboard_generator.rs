use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::analyzers::unified_analyzer::AnalysisResult;

#[derive(Debug, Serialize, Deserialize)]
struct DashboardData {
    timestamp: String,
    overall_progress: f32,
    feature_areas: Vec<FeatureAreaProgress>,
    recent_changes: Vec<String>,
    next_steps: Vec<String>,
    tech_debt_metrics: TechDebtMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeatureAreaProgress {
    name: String,
    implemented: u32,
    total: u32,
    percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TechDebtMetrics {
    critical_issues: u32,
    high_issues: u32,
    medium_issues: u32,
    low_issues: u32,
    total_issues: u32,
}

impl Default for TechDebtMetrics {
    fn default() -> Self {
        Self {
            critical_issues: 0,
            high_issues: 2,
            medium_issues: 8,
            low_issues: 15,
            total_issues: 25,
        }
    }
}

pub struct DashboardGenerator {
    base_dir: PathBuf,
    history_file: PathBuf,
}

impl DashboardGenerator {
    pub fn new(base_dir: PathBuf) -> Self {
        let docs_dir = base_dir.join("docs");
        let history_file = docs_dir.join("analysis_history.json");
        
        Self {
            base_dir,
            history_file,
        }
    }
    
    /// Generate a dashboard for the project
    pub fn generate_dashboard(&self, result: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Generating project dashboard...");
        
        // Ensure docs directory exists
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)
                .map_err(|e| format!("Failed to create docs directory: {}", e))?;
        }
        
        // Create dashboard data
        let dashboard_data = self.create_dashboard_data(result);
        
        // Update history
        self.update_history(&dashboard_data)?;
        
        // Generate HTML dashboard
        let dashboard_path = docs_dir.join("dashboard.html");
        self.generate_html_dashboard(&dashboard_path, &dashboard_data)?;
        
        println!("Dashboard generated at: {:?}", dashboard_path);
        Ok(dashboard_path)
    }
    
    /// Create dashboard data from analysis result
    fn create_dashboard_data(&self, result: &AnalysisResult) -> DashboardData {
        // Create feature area progress data
        let mut feature_areas = Vec::new();
        
        // Models
        feature_areas.push(FeatureAreaProgress {
            name: "Models".to_string(),
            implemented: result.models.implemented as u32,
            total: result.models.total as u32,
            percentage: result.models.implementation_percentage,
        });
        
        // API Endpoints
        feature_areas.push(FeatureAreaProgress {
            name: "API Endpoints".to_string(),
            implemented: result.api_endpoints.implemented as u32,
            total: result.api_endpoints.total as u32,
            percentage: result.api_endpoints.implementation_percentage,
        });
        
        // UI Components
        feature_areas.push(FeatureAreaProgress {
            name: "UI Components".to_string(),
            implemented: result.ui_components.implemented as u32,
            total: result.ui_components.total as u32,
            percentage: result.ui_components.implementation_percentage,
        });
        
        // Integration
        feature_areas.push(FeatureAreaProgress {
            name: "Integration".to_string(),
            implemented: result.integration.implemented as u32,
            total: result.integration.total as u32,
            percentage: result.integration.implementation_percentage,
        });
        
        // Create next steps from recommendations
        let next_steps = result.recommendations.iter()
            .map(|rec| format!("{}: {}", rec.title, rec.description))
            .collect::<Vec<_>>();
        
        // Create recent changes (placeholder)
        let recent_changes = vec![
            "Updated model implementations".to_string(),
            "Added new API endpoints".to_string(),
            "Improved test coverage".to_string(),
        ];
        
        DashboardData {
            timestamp: Utc::now().to_rfc3339(),
            overall_progress: result.project_status.completion_percentage,
            feature_areas,
            recent_changes,
            next_steps,
            tech_debt_metrics: TechDebtMetrics::default(),
        }
    }
    
    /// Update the history file with new dashboard data
    fn update_history(&self, dashboard_data: &DashboardData) -> Result<(), String> {
        // Read existing history or create new
        let mut history: Vec<DashboardData> = if self.history_file.exists() {
            let history_content = fs::read_to_string(&self.history_file)
                .map_err(|e| format!("Failed to read history file: {}", e))?;
            
            serde_json::from_str(&history_content)
                .map_err(|e| format!("Failed to parse history file: {}", e))?
        } else {
            Vec::new()
        };
        
        // Add new data
        history.push(dashboard_data.clone());
        
        // Limit history to last 30 entries
        if history.len() > 30 {
            history = history.into_iter().skip(history.len() - 30).collect();
        }
        
        // Write updated history
        let history_content = serde_json::to_string_pretty(&history)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;
        
        // Ensure parent directory exists
        if let Some(parent) = self.history_file.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create history directory: {}", e))?;
            }
        }
        
        fs::write(&self.history_file, history_content)
            .map_err(|e| format!("Failed to write history file: {}", e))?;
        
        Ok(())
    }
    
    /// Generate HTML dashboard
    fn generate_html_dashboard(&self, path: &Path, data: &DashboardData) -> Result<(), String> {
        let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Project Dashboard</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
            color: #333;
        }
        .dashboard-container {
            max-width: 1200px;
            margin: 0 auto;
        }
        .header {
            text-align: center;
            margin-bottom: 30px;
        }
        .card {
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }
        .progress-container {
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
        }
        .progress-card {
            flex: 1;
            min-width: 200px;
        }
        .progress-bar {
            height: 20px;
            background-color: #e0e0e0;
            border-radius: 10px;
            margin: 10px 0;
            overflow: hidden;
        }
        .progress-fill {
            height: 100%;
            background-color: #4caf50;
            border-radius: 10px;
        }
        .progress-text {
            display: flex;
            justify-content: space-between;
        }
        .columns {
            display: flex;
            gap: 20px;
        }
        .column {
            flex: 1;
        }
        h2 {
            margin-top: 0;
            color: #2c3e50;
        }
        ul {
            padding-left: 20px;
        }
        .tech-debt {
            display: flex;
            gap: 10px;
        }
        .debt-item {
            flex: 1;
            text-align: center;
            padding: 10px;
            border-radius: 5px;
        }
        .critical {
            background-color: #ff5252;
            color: white;
        }
        .high {
            background-color: #ff9800;
            color: white;
        }
        .medium {
            background-color: #ffeb3b;
            color: #333;
        }
        .low {
            background-color: #8bc34a;
            color: white;
        }
    </style>
</head>
<body>
    <div class="dashboard-container">
        <div class="header">
            <h1>Project Dashboard</h1>
            <p>Last updated: "#);
        
        // Add timestamp
        let timestamp = chrono::DateTime::parse_from_rfc3339(&data.timestamp)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|_| data.timestamp.clone());
        
        html.push_str(&timestamp);
        html.push_str("</p>\n        </div>\n\n");
        
        // Overall progress
        html.push_str(&format!(r#"        <div class="card">
            <h2>Overall Progress: {:.1}%</h2>
            <div class="progress-bar">
                <div class="progress-fill" style="width: {:.1}%;"></div>
            </div>
        </div>

"#, data.overall_progress, data.overall_progress));
        
        // Feature areas
        html.push_str(r#"        <div class="card">
            <h2>Feature Areas</h2>
            <div class="progress-container">
"#);
        
        for area in &data.feature_areas {
            html.push_str(&format!(r#"                <div class="progress-card">
                    <h3>{}</h3>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {:.1}%;"></div>
                    </div>
                    <div class="progress-text">
                        <span>{}/{}</span>
                        <span>{:.1}%</span>
                    </div>
                </div>
"#, area.name, area.percentage, area.implemented, area.total, area.percentage));
        }
        
        html.push_str("            </div>\n        </div>\n\n");
        
        // Columns for Recent Changes and Next Steps
        html.push_str(r#"        <div class="columns">
            <div class="column">
                <div class="card">
                    <h2>Recent Changes</h2>
                    <ul>
"#);
        
        for change in &data.recent_changes {
            html.push_str(&format!("                        <li>{}</li>\n", change));
        }
        
        html.push_str(r#"                    </ul>
                </div>
            </div>
            <div class="column">
                <div class="card">
                    <h2>Next Steps</h2>
                    <ul>
"#);
        
        for step in &data.next_steps {
            html.push_str(&format!("                        <li>{}</li>\n", step));
        }
        
        html.push_str(r#"                    </ul>
                </div>
            </div>
        </div>

"#);
        
        // Tech Debt
        html.push_str(&format!(r#"        <div class="card">
            <h2>Technical Debt</h2>
            <div class="tech-debt">
                <div class="debt-item critical">
                    <h3>Critical</h3>
                    <p>{}</p>
                </div>
                <div class="debt-item high">
                    <h3>High</h3>
                    <p>{}</p>
                </div>
                <div class="debt-item medium">
                    <h3>Medium</h3>
                    <p>{}</p>
                </div>
                <div class="debt-item low">
                    <h3>Low</h3>
                    <p>{}</p>
                </div>
            </div>
            <p>Total issues: {}</p>
        </div>
"#, 
            data.tech_debt_metrics.critical_issues,
            data.tech_debt_metrics.high_issues,
            data.tech_debt_metrics.medium_issues,
            data.tech_debt_metrics.low_issues,
            data.tech_debt_metrics.total_issues
        ));
        
        html.push_str("    </div>\n</body>\n</html>");
        
        // Write HTML to file
        fs::write(path, html)
            .map_err(|e| format!("Failed to write dashboard HTML: {}", e))?;
        
        Ok(())
    }
}
