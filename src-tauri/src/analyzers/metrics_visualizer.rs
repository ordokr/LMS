use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, Utc};

use crate::analyzers::unified_analyzer::AnalysisResult;

pub struct MetricsVisualizer {
    base_dir: PathBuf,
    output_dir: PathBuf,
}

impl MetricsVisualizer {
    pub fn new(base_dir: PathBuf) -> Self {
        let output_dir = base_dir.join("docs").join("visualizations");
        
        Self {
            base_dir,
            output_dir,
        }
    }
    
    /// Generate visualization dashboard for the latest analysis
    pub fn generate_dashboard(&self, result: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Generating metrics visualization dashboard...");
        
        // Ensure output directory exists
        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir)
                .map_err(|e| format!("Failed to create visualizations directory: {}", e))?;
        }
        
        // Generate the dashboard HTML
        let dashboard_content = self.generate_dashboard_html(result)?;
        let dashboard_path = self.output_dir.join("metrics_dashboard.html");
        
        // Write the dashboard file
        fs::write(&dashboard_path, dashboard_content)
            .map_err(|e| format!("Failed to write dashboard file: {}", e))?;
        
        // Copy the JavaScript files for the charts
        self.copy_chart_assets()?;
        
        println!("Metrics dashboard generated at: {:?}", dashboard_path);
        Ok(dashboard_path)
    }
    
    /// Generate the HTML content for the dashboard
    fn generate_dashboard_html(&self, result: &AnalysisResult) -> Result<String, String> {
        let now = Local::now();
        
        // Create the implementation data for charts
        let implementation_data = self.generate_implementation_chart_data(result);
        
        // Create the components data for charts
        let components_data = self.generate_components_chart_data(result);
        
        // Create the feature areas data for the radar chart
        let feature_areas_data = self.generate_feature_areas_chart_data(result);
        
        // Generate the HTML content
        let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>LMS Project Metrics Dashboard</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css">
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f8f9fa;
            color: #333;
        }}
        .dashboard-header {{
            text-align: center;
            margin-bottom: 30px;
            padding-bottom: 20px;
            border-bottom: 1px solid #ddd;
        }}
        .chart-container {{
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.05);
            padding: 20px;
            margin-bottom: 30px;
        }}
        .metrics-summary {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        .metric-card {{
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.05);
            padding: 20px;
            text-align: center;
        }}
        .metric-value {{
            font-size: 24px;
            font-weight: bold;
            margin: 10px 0;
        }}
        .metric-label {{
            font-size: 14px;
            color: #666;
        }}
        .progress {{
            height: 8px;
            margin-top: 10px;
            margin-bottom: 5px;
        }}
        .good-progress {{
            background-color: #28a745;
        }}
        .medium-progress {{
            background-color: #ffc107;
        }}
        .low-progress {{
            background-color: #dc3545;
        }}
        .chart-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
            gap: 20px;
        }}
        .last-updated {{
            text-align: center;
            margin-top: 20px;
            color: #666;
            font-size: 12px;
        }}
        @media (max-width: 768px) {{
            .chart-grid {{
                grid-template-columns: 1fr;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="dashboard-header">
            <h1>LMS Project Metrics Dashboard</h1>
            <p>A visual overview of the Canvas-Discourse integration project</p>
        </div>
        
        <!-- Summary Metrics -->
        <div class="metrics-summary">
            <div class="metric-card">
                <div class="metric-label">Overall Completion</div>
                <div class="metric-value">{:.1}%</div>
                <div class="progress">
                    <div class="progress-bar {}" role="progressbar" style="width: {:.1}%"></div>
                </div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Models Implementation</div>
                <div class="metric-value">{:.1}%</div>
                <div class="progress">
                    <div class="progress-bar {}" role="progressbar" style="width: {:.1}%"></div>
                </div>
            </div>
            <div class="metric-card">
                <div class="metric-label">API Endpoints</div>
                <div class="metric-value">{:.1}%</div>
                <div class="progress">
                    <div class="progress-bar {}" role="progressbar" style="width: {:.1}%"></div>
                </div>
            </div>
            <div class="metric-card">
                <div class="metric-label">UI Components</div>
                <div class="metric-value">{:.1}%</div>
                <div class="progress">
                    <div class="progress-bar {}" role="progressbar" style="width: {:.1}%"></div>
                </div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Total Code Files</div>
                <div class="metric-value">{}</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Lines of Code</div>
                <div class="metric-value">{}</div>
            </div>
        </div>
        
        <!-- Charts Grid -->
        <div class="chart-grid">
            <!-- Implementation Progress Chart -->
            <div class="chart-container">
                <h3>Implementation Progress</h3>
                <canvas id="implementationChart"></canvas>
            </div>
            
            <!-- Components Chart -->
            <div class="chart-container">
                <h3>Components Breakdown</h3>
                <canvas id="componentsChart"></canvas>
            </div>
            
            <!-- Feature Areas Chart -->
            <div class="chart-container">
                <h3>Feature Areas Coverage</h3>
                <canvas id="featureAreasChart"></canvas>
            </div>
            
            <!-- File Types Chart -->
            <div class="chart-container">
                <h3>File Types Distribution</h3>
                <canvas id="fileTypesChart"></canvas>
            </div>
        </div>
        
        <div class="last-updated">
            Last updated: {:%Y-%m-%d %H:%M:%S}
        </div>
    </div>
    
    <script>
        // Implementation Progress Chart
        const implementationCtx = document.getElementById('implementationChart').getContext('2d');
        const implementationChart = new Chart(implementationCtx, {{
            type: 'bar',
            data: {{
                labels: ['Total', 'Implemented', 'In Progress', 'Not Started'],
                datasets: [
                    {{
                        label: 'Models',
                        data: [{implementation_data.models}],
                        backgroundColor: 'rgba(54, 162, 235, 0.7)',
                    }},
                    {{
                        label: 'API Endpoints',
                        data: [{implementation_data.apis}],
                        backgroundColor: 'rgba(255, 99, 132, 0.7)',
                    }},
                    {{
                        label: 'UI Components',
                        data: [{implementation_data.ui}],
                        backgroundColor: 'rgba(75, 192, 192, 0.7)',
                    }},
                ]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Count'
                        }}
                    }}
                }},
                plugins: {{
                    legend: {{
                        position: 'top',
                    }},
                    title: {{
                        display: false,
                        text: 'Implementation Progress'
                    }}
                }}
            }}
        }});
        
        // Components Chart
        const componentsCtx = document.getElementById('componentsChart').getContext('2d');
        const componentsChart = new Chart(componentsCtx, {{
            type: 'pie',
            data: {{
                labels: ['Models', 'API Endpoints', 'UI Components', 'Tests', 'Other'],
                datasets: [{{
                    data: [{components_data}],
                    backgroundColor: [
                        'rgba(54, 162, 235, 0.7)',
                        'rgba(255, 99, 132, 0.7)',
                        'rgba(75, 192, 192, 0.7)',
                        'rgba(255, 206, 86, 0.7)',
                        'rgba(153, 102, 255, 0.7)',
                    ],
                    borderColor: [
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 99, 132, 1)',
                        'rgba(75, 192, 192, 1)',
                        'rgba(255, 206, 86, 1)',
                        'rgba(153, 102, 255, 1)',
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
                }}
            }}
        }});
        
        // Feature Areas Chart
        const featureAreasCtx = document.getElementById('featureAreasChart').getContext('2d');
        const featureAreasChart = new Chart(featureAreasCtx, {{
            type: 'radar',
            data: {{
                labels: [{feature_areas_data.labels}],
                datasets: [{{
                    label: 'Completion Percentage',
                    data: [{feature_areas_data.values}],
                    fill: true,
                    backgroundColor: 'rgba(54, 162, 235, 0.2)',
                    borderColor: 'rgb(54, 162, 235)',
                    pointBackgroundColor: 'rgb(54, 162, 235)',
                    pointBorderColor: '#fff',
                    pointHoverBackgroundColor: '#fff',
                    pointHoverBorderColor: 'rgb(54, 162, 235)'
                }}]
            }},
            options: {{
                elements: {{
                    line: {{
                        borderWidth: 3
                    }}
                }},
                scales: {{
                    r: {{
                        angleLines: {{
                            display: true
                        }},
                        suggestedMin: 0,
                        suggestedMax: 100
                    }}
                }}
            }}
        }});
        
        // File Types Chart
        const fileTypesCtx = document.getElementById('fileTypesChart').getContext('2d');
        const fileTypesChart = new Chart(fileTypesCtx, {{
            type: 'doughnut',
            data: {{
                labels: ['Rust', 'JavaScript', 'HTML/CSS', 'Markdown', 'Other'],
                datasets: [{{
                    data: [
                        {result.project_status.file_stats.get("rs").unwrap_or(&0)}, 
                        {result.project_status.file_stats.get("js").unwrap_or(&0)}, 
                        {result.project_status.file_stats.get("html").unwrap_or(&0) + result.project_status.file_stats.get("css").unwrap_or(&0)},
                        {result.project_status.file_stats.get("md").unwrap_or(&0)},
                        {result.project_status.file_stats.get("total").unwrap_or(&0) - 
                          (result.project_status.file_stats.get("rs").unwrap_or(&0) + 
                           result.project_status.file_stats.get("js").unwrap_or(&0) + 
                           result.project_status.file_stats.get("html").unwrap_or(&0) + 
                           result.project_status.file_stats.get("css").unwrap_or(&0) + 
                           result.project_status.file_stats.get("md").unwrap_or(&0))}
                    ],
                    backgroundColor: [
                        'rgba(255, 99, 132, 0.7)',
                        'rgba(54, 162, 235, 0.7)',
                        'rgba(255, 206, 86, 0.7)',
                        'rgba(75, 192, 192, 0.7)',
                        'rgba(153, 102, 255, 0.7)',
                    ],
                    borderColor: [
                        'rgba(255, 99, 132, 1)',
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 206, 86, 1)',
                        'rgba(75, 192, 192, 1)',
                        'rgba(153, 102, 255, 1)',
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
                }}
            }}
        }});
    </script>
</body>
</html>
"#,
            // Overall completion percentage
            result.project_status.overall_completion_percentage,
            // Overall completion progress class
            Self::get_progress_class(result.project_status.overall_completion_percentage),
            // Overall completion percentage (for width)
            result.project_status.overall_completion_percentage,
            
            // Models implementation percentage
            result.models.implementation_percentage,
            // Models implementation progress class
            Self::get_progress_class(result.models.implementation_percentage),
            // Models implementation percentage (for width)
            result.models.implementation_percentage,
            
            // API endpoints implementation percentage
            result.api_endpoints.implementation_percentage,
            // API endpoints implementation progress class
            Self::get_progress_class(result.api_endpoints.implementation_percentage),
            // API endpoints implementation percentage (for width)
            result.api_endpoints.implementation_percentage,
            
            // UI components implementation percentage
            result.ui_components.implementation_percentage,
            // UI components implementation progress class
            Self::get_progress_class(result.ui_components.implementation_percentage),
            // UI components implementation percentage (for width)
            result.ui_components.implementation_percentage,
            
            // Total code files
            result.project_status.file_stats.get("total").unwrap_or(&0),
            
            // Lines of code
            result.project_status.total_lines_of_code,
            
            // Last updated timestamp
            now
        );
        
        Ok(html)
    }
    
    /// Generate the implementation chart data for models, APIs, and UI components
    fn generate_implementation_chart_data(&self, result: &AnalysisResult) -> ImplementationChartData {
        // Models data
        let models_total = result.models.total;
        let models_implemented = result.models.implemented;
        let models_in_progress = (models_total as f32 * 0.5).round() as i32; // Estimate
        let models_not_started = models_total - models_implemented - models_in_progress;
        
        // API endpoints data
        let apis_total = result.api_endpoints.total;
        let apis_implemented = result.api_endpoints.implemented;
        let apis_in_progress = (apis_total as f32 * 0.3).round() as i32; // Estimate
        let apis_not_started = apis_total - apis_implemented - apis_in_progress;
        
        // UI components data
        let ui_total = result.ui_components.total;
        let ui_implemented = result.ui_components.implemented;
        let ui_in_progress = (ui_total as f32 * 0.4).round() as i32; // Estimate
        let ui_not_started = ui_total - ui_implemented - ui_in_progress;
        
        ImplementationChartData {
            models: format!("{}, {}, {}, {}", models_total, models_implemented, models_in_progress, models_not_started),
            apis: format!("{}, {}, {}, {}", apis_total, apis_implemented, apis_in_progress, apis_not_started),
            ui: format!("{}, {}, {}, {}", ui_total, ui_implemented, ui_in_progress, ui_not_started),
        }
    }
    
    /// Generate the components chart data
    fn generate_components_chart_data(&self, result: &AnalysisResult) -> String {
        let models = result.models.total;
        let apis = result.api_endpoints.total;
        let ui = result.ui_components.total;
        let tests = result.tests.total;
        let other = 20; // Placeholder for other components
        
        format!("{}, {}, {}, {}, {}", models, apis, ui, tests, other)
    }
    
    /// Generate the feature areas chart data
    fn generate_feature_areas_chart_data(&self, result: &AnalysisResult) -> FeatureAreasChartData {
        let mut labels = Vec::new();
        let mut values = Vec::new();
        
        // Sort feature areas by name for consistent display
        let mut feature_areas: Vec<(&String, &_)> = result.feature_areas.iter().collect();
        feature_areas.sort_by(|a, b| a.0.cmp(b.0));
        
        for (name, feature) in feature_areas {
            labels.push(format!("'{}'", name));
            values.push(feature.completion_percentage.to_string());
        }
        
        FeatureAreasChartData {
            labels: labels.join(", "),
            values: values.join(", "),
        }
    }
    
    /// Copy the chart assets to the output directory
    fn copy_chart_assets(&self) -> Result<(), String> {
        // Chart.js is loaded from CDN, no need to copy assets
        Ok(())
    }
    
    /// Get the appropriate Bootstrap progress bar class based on the percentage
    fn get_progress_class(percentage: f32) -> &'static str {
        if percentage >= 75.0 {
            "progress-bar good-progress"
        } else if percentage >= 40.0 {
            "progress-bar medium-progress"
        } else {
            "progress-bar low-progress"
        }
    }
}

/// Helper struct for implementation chart data
struct ImplementationChartData {
    models: String,
    apis: String,
    ui: String,
}

/// Helper struct for feature areas chart data
struct FeatureAreasChartData {
    labels: String,
    values: String,
}
