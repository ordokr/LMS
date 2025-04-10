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
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        let base = base_dir.as_ref().to_path_buf();
        let history_file = base.join("docs").join("analysis_history.json");
        
        Self {
            base_dir: base,
            history_file,
        }
    }
    
    pub fn generate_dashboard(&self, result: &AnalysisResult) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating analysis dashboard...");
        
        // Ensure docs directory exists
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)?;
        }
        
        // Calculate overall progress
        let mut total_features = 0;
        let mut total_implemented = 0;
        
        for (_area, metrics) in &result.feature_areas {
            total_features += metrics.total;
            total_implemented += metrics.implemented;
        }
        
        let overall_progress = if total_features > 0 {
            (total_implemented as f32 / total_features as f32) * 100.0
        } else {
            0.0
        };
        
        // Create feature area progress data
        let mut feature_areas = Vec::new();
        
        for (area, metrics) in &result.feature_areas {
            let percentage = if metrics.total > 0 {
                (metrics.implemented as f32 / metrics.total as f32) * 100.0
            } else {
                0.0
            };
            
            feature_areas.push(FeatureAreaProgress {
                name: area.clone(),
                implemented: metrics.implemented,
                total: metrics.total,
                percentage,
            });
        }
        
        // Sort feature areas by progress percentage (descending)
        feature_areas.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
        
        // Add some mock recent changes and next steps
        // In a real implementation, these could be sourced from Git history and task boards
        let recent_changes = vec![
            "Implemented authentication token refresh flow".to_string(),
            "Added error handling for offline scenarios".to_string(),
            "Fixed regression in content synchronization".to_string(),
            "Improved API response caching".to_string(),
            "Added unit tests for model validators".to_string(),
        ];
        
        let next_steps = vec![
            "Implement conflict resolution UI".to_string(),
            "Add support for multimedia attachments".to_string(),
            "Optimize database queries for large datasets".to_string(),
            "Create comprehensive API documentation".to_string(),
            "Set up continuous integration pipeline".to_string(),
        ];
        
        // Create dashboard data
        let dashboard_data = DashboardData {
            timestamp: Utc::now().to_rfc3339(),
            overall_progress,
            feature_areas,
            recent_changes,
            next_steps,
            tech_debt_metrics: TechDebtMetrics::default(),
        };
        
        // Save dashboard data to history
        self.update_history(&dashboard_data)?;
        
        // Generate HTML dashboard
        let dashboard_html = self.generate_html_dashboard(&dashboard_data)?;
        
        // Write dashboard to file
        let dashboard_path = docs_dir.join("analysis_dashboard.html");
        let mut file = File::create(&dashboard_path)?;
        file.write_all(dashboard_html.as_bytes())?;
        
        println!("Analysis dashboard generated: {:?}", dashboard_path);
        
        Ok(())
    }
    
    fn update_history(&self, current_data: &DashboardData) -> Result<(), Box<dyn std::error::Error>> {
        // Create or load history data
        let mut history: Vec<DashboardData> = if self.history_file.exists() {
            // Read existing history file
            let json_str = fs::read_to_string(&self.history_file)?;
            serde_json::from_str(&json_str)?
        } else {
            // Create new history if file doesn't exist
            Vec::new()
        };
        
        // Add current data to history
        history.push(current_data.clone());
        
        // Keep only the last 30 entries (about a month if run daily)
        if history.len() > 30 {
            history = history.into_iter().skip(history.len() - 30).collect();
        }
        
        // Ensure the directory exists
        if let Some(parent) = self.history_file.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        
        // Write updated history to file
        let json_str = serde_json::to_string_pretty(&history)?;
        let mut file = File::create(&self.history_file)?;
        file.write_all(json_str.as_bytes())?;
        
        Ok(())
    }
    
    fn generate_html_dashboard(&self, data: &DashboardData) -> Result<String, Box<dyn std::error::Error>> {
        // Load history data for historical charts
        let history: Vec<DashboardData> = if self.history_file.exists() {
            let json_str = fs::read_to_string(&self.history_file)?;
            serde_json::from_str(&json_str)?
        } else {
            Vec::new()
        };
        
        // Generate history data for chart.js
        let mut progress_history = String::new();
        let mut labels = String::new();
        
        for (i, entry) in history.iter().enumerate() {
            // Parse the timestamp to get the date part
            let date = entry.timestamp.split('T').next().unwrap_or("Unknown");
            
            if i > 0 {
                progress_history.push_str(",");
                labels.push_str(",");
            }
            
            progress_history.push_str(&format!("{:.1}", entry.overall_progress));
            labels.push_str(&format!("'{}'", date));
        }
        
        // Generate feature area data for the chart
        let mut area_names = String::new();
        let mut area_progress = String::new();
        
        for (i, area) in data.feature_areas.iter().enumerate() {
            if i > 0 {
                area_names.push_str(",");
                area_progress.push_str(",");
            }
            
            area_names.push_str(&format!("'{}'", area.name));
            area_progress.push_str(&format!("{:.1}", area.percentage));
        }
        
        // Generate technical debt chart data
        let tech_debt_labels = "'Critical','High','Medium','Low'";
        let tech_debt_data = format!("{},{},{},{}",
            data.tech_debt_metrics.critical_issues,
            data.tech_debt_metrics.high_issues,
            data.tech_debt_metrics.medium_issues,
            data.tech_debt_metrics.low_issues
        );
        
        // Generate the HTML
        let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>LMS Project Analysis Dashboard</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        .progress-bar-container {{
            height: 24px;
            background-color: #e5e7eb;
            border-radius: 9999px;
            overflow: hidden;
        }}
        .progress-bar {{
            height: 100%;
            background-color: #3b82f6;
            transition: width 0.5s ease-in-out;
        }}
        .card {{
            background-color: white;
            border-radius: 0.5rem;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            padding: 1.5rem;
            margin-bottom: 1.5rem;
        }}
    </style>
</head>
<body class="bg-gray-100 min-h-screen">
    <div class="container mx-auto py-8 px-4">
        <header class="mb-8">
            <h1 class="text-3xl font-bold text-gray-800">LMS Project Analysis Dashboard</h1>
            <p class="text-gray-600">Last updated: {}</p>
        </header>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Overall Progress Card -->
            <div class="card">
                <h2 class="text-xl font-semibold mb-4">Overall Project Progress</h2>
                <div class="text-5xl font-bold text-blue-600 mb-4">{:.1}%</div>
                <div class="progress-bar-container">
                    <div class="progress-bar" style="width: {:.1}%"></div>
                </div>
            </div>
            
            <!-- Feature Status Card -->
            <div class="card">
                <h2 class="text-xl font-semibold mb-4">Implementation Status by Area</h2>
                <div>
                    <canvas id="featureChart"></canvas>
                </div>
            </div>
            
            <!-- Historical Progress Chart -->
            <div class="card">
                <h2 class="text-xl font-semibold mb-4">Progress Over Time</h2>
                <div>
                    <canvas id="historyChart"></canvas>
                </div>
            </div>
            
            <!-- Technical Debt Chart -->
            <div class="card">
                <h2 class="text-xl font-semibold mb-4">Technical Debt</h2>
                <div>
                    <canvas id="techDebtChart"></canvas>
                </div>
                <div class="mt-2 text-sm text-gray-500">
                    Total Issues: {}, Critical: {}, High: {}, Medium: {}, Low: {}
                </div>
            </div>
            
            <!-- Recent Changes -->
            <div class="card">
                <h2 class="text-xl font-semibold mb-4">Recent Changes</h2>
                <ul class="list-disc pl-5 space-y-1">
                    {}
                </ul>
            </div>
            
            <!-- Next Steps -->
            <div class="card">
                <h2 class="text-xl font-semibold mb-4">Recommended Next Steps</h2>
                <ul class="list-disc pl-5 space-y-1">
                    {}
                </ul>
            </div>
        </div>
    </div>
    
    <script>
        // Feature Areas Chart
        const featureCtx = document.getElementById('featureChart').getContext('2d');
        new Chart(featureCtx, {{
            type: 'bar',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Implementation Progress (%)',
                    data: [{}],
                    backgroundColor: 'rgba(59, 130, 246, 0.7)',
                    borderColor: 'rgba(59, 130, 246, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                scales: {{
                    y: {{
                        beginAtZero: true,
                        max: 100,
                        ticks: {{
                            callback: function(value) {{
                                return value + '%';
                            }}
                        }}
                    }}
                }}
            }}
        }});
        
        // Historical Progress Chart
        const historyCtx = document.getElementById('historyChart').getContext('2d');
        new Chart(historyCtx, {{
            type: 'line',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Project Progress (%)',
                    data: [{}],
                    fill: false,
                    borderColor: 'rgba(59, 130, 246, 1)',
                    tension: 0.1
                }}]
            }},
            options: {{
                scales: {{
                    y: {{
                        beginAtZero: true,
                        max: 100,
                        ticks: {{
                            callback: function(value) {{
                                return value + '%';
                            }}
                        }}
                    }}
                }}
            }}
        }});
        
        // Technical Debt Chart
        const techDebtCtx = document.getElementById('techDebtChart').getContext('2d');
        new Chart(techDebtCtx, {{
            type: 'doughnut',
            data: {{
                labels: [{}],
                datasets: [{{
                    data: [{}],
                    backgroundColor: [
                        'rgba(220, 38, 38, 0.7)',  // Critical - Red
                        'rgba(251, 191, 36, 0.7)', // High - Amber
                        'rgba(59, 130, 246, 0.7)', // Medium - Blue
                        'rgba(16, 185, 129, 0.7)'  // Low - Green
                    ],
                    borderColor: [
                        'rgba(220, 38, 38, 1)',
                        'rgba(251, 191, 36, 1)',
                        'rgba(59, 130, 246, 1)',
                        'rgba(16, 185, 129, 1)'
                    ],
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    legend: {{
                        position: 'right',
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>
"#,
            // Header date
            data.timestamp.split('T').next().unwrap_or("Unknown"),
            
            // Overall progress percentage (value and progress bar)
            data.overall_progress,
            data.overall_progress,
            
            // Technical debt summary
            data.tech_debt_metrics.total_issues,
            data.tech_debt_metrics.critical_issues,
            data.tech_debt_metrics.high_issues,
            data.tech_debt_metrics.medium_issues,
            data.tech_debt_metrics.low_issues,
            
            // Recent changes list
            data.recent_changes.iter()
                .map(|change| format!("<li>{}</li>", change))
                .collect::<Vec<_>>()
                .join("\n                    "),
            
            // Next steps list
            data.next_steps.iter()
                .map(|step| format!("<li>{}</li>", step))
                .collect::<Vec<_>>()
                .join("\n                    "),
            
            // Feature chart data
            area_names,
            area_progress,
            
            // History chart data
            labels,
            progress_history,
            
            // Technical debt chart data
            tech_debt_labels,
            tech_debt_data
        );
        
        Ok(html)
    }
}
