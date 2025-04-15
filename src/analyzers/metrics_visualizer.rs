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
        let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Project Metrics Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
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
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
            grid-gap: 20px;
        }
        .chart-container {
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }
        h1, h2 {
            color: #2c3e50;
        }
        .header {
            text-align: center;
            margin-bottom: 30px;
        }
        .metrics-summary {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
            grid-gap: 15px;
            margin-bottom: 30px;
        }
        .metric-card {
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 15px;
            text-align: center;
        }
        .metric-value {
            font-size: 24px;
            font-weight: bold;
            color: #3498db;
            margin: 10px 0;
        }
        .metric-label {
            font-size: 14px;
            color: #7f8c8d;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>Project Metrics Dashboard</h1>
        <p>Last updated: "#);
        
        // Add timestamp
        html.push_str(&Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string());
        html.push_str("</p>\n    </div>\n\n");
        
        // Add metrics summary cards
        html.push_str("    <div class=\"metrics-summary\">\n");
        
        // Overall completion
        html.push_str(&format!(r#"        <div class="metric-card">
            <div class="metric-label">Overall Completion</div>
            <div class="metric-value">{:.1}%</div>
        </div>
"#, result.project_status.completion_percentage));
        
        // Models completion
        html.push_str(&format!(r#"        <div class="metric-card">
            <div class="metric-label">Models</div>
            <div class="metric-value">{:.1}%</div>
        </div>
"#, result.models.implementation_percentage));
        
        // API completion
        html.push_str(&format!(r#"        <div class="metric-card">
            <div class="metric-label">API Endpoints</div>
            <div class="metric-value">{:.1}%</div>
        </div>
"#, result.api_endpoints.implementation_percentage));
        
        // UI completion
        html.push_str(&format!(r#"        <div class="metric-card">
            <div class="metric-label">UI Components</div>
            <div class="metric-value">{:.1}%</div>
        </div>
"#, result.ui_components.implementation_percentage));
        
        // Test coverage
        html.push_str(&format!(r#"        <div class="metric-card">
            <div class="metric-label">Test Coverage</div>
            <div class="metric-value">{:.1}%</div>
        </div>
"#, result.tests.coverage));
        
        html.push_str("    </div>\n\n");
        
        // Add chart containers
        html.push_str(r#"    <div class="dashboard-container">
        <div class="chart-container">
            <h2>Implementation Progress</h2>
            <canvas id="implementationChart"></canvas>
        </div>
        <div class="chart-container">
            <h2>Code Quality Metrics</h2>
            <canvas id="qualityChart"></canvas>
        </div>
    </div>

    <div class="dashboard-container">
        <div class="chart-container">
            <h2>Project Status</h2>
            <canvas id="statusChart"></canvas>
        </div>
        <div class="chart-container">
            <h2>Test Coverage</h2>
            <canvas id="testingChart"></canvas>
        </div>
    </div>
"#);
        
        // Add JavaScript for charts
        html.push_str("    <script>\n");
        
        // Implementation Progress Chart
        html.push_str(&format!(r#"        // Implementation Progress Chart
        const implementationCtx = document.getElementById('implementationChart').getContext('2d');
        const implementationChart = new Chart(implementationCtx, {{
            type: 'bar',
            data: {{
                labels: ['Models', 'API Endpoints', 'UI Components', 'Integration'],
                datasets: [{{
                    label: 'Implemented',
                    data: [{}, {}, {}, {}],
                    backgroundColor: 'rgba(54, 162, 235, 0.5)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                }},
                {{
                    label: 'Total',
                    data: [{}, {}, {}, {}],
                    backgroundColor: 'rgba(255, 99, 132, 0.5)',
                    borderColor: 'rgba(255, 99, 132, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                scales: {{
                    y: {{
                        beginAtZero: true
                    }}
                }}
            }}
        }});
"#, 
            result.models.implemented,
            result.api_endpoints.implemented,
            result.ui_components.implemented,
            result.integration.implemented,
            result.models.total,
            result.api_endpoints.total,
            result.ui_components.total,
            result.integration.total
        ));
        
        // Code Quality Chart
        html.push_str("        // Code Quality Chart\n");
        html.push_str("        const qualityCtx = document.getElementById('qualityChart').getContext('2d');\n");
        html.push_str("        const qualityChart = new Chart(qualityCtx, {\n");
        html.push_str("            type: 'radar',\n");
        html.push_str("            data: {\n");
        
        // Extract quality metrics
        let mut metrics_labels = Vec::new();
        let mut metrics_values = Vec::new();
        
        for (metric, value) in &result.code_quality.metrics {
            metrics_labels.push(format!("\"{}\"", metric));
            metrics_values.push(value.to_string());
        }
        
        html.push_str(&format!("                labels: [{}],\n", metrics_labels.join(", ")));
        html.push_str("                datasets: [{\n");
        html.push_str("                    label: 'Code Quality',\n");
        html.push_str(&format!("                    data: [{}],\n", metrics_values.join(", ")));
        html.push_str("                    backgroundColor: 'rgba(75, 192, 192, 0.2)',\n");
        html.push_str("                    borderColor: 'rgba(75, 192, 192, 1)',\n");
        html.push_str("                    borderWidth: 1\n");
        html.push_str("                }]\n");
        html.push_str("            },\n");
        html.push_str("            options: {\n");
        html.push_str("                scales: {\n");
        html.push_str("                    r: {\n");
        html.push_str("                        beginAtZero: true,\n");
        html.push_str("                        max: 5\n");
        html.push_str("                    }\n");
        html.push_str("                }\n");
        html.push_str("            }\n");
        html.push_str("        });\n");
        
        // Project Status Chart (Doughnut)
        html.push_str(&format!(r#"        // Project Status Chart
        const statusCtx = document.getElementById('statusChart').getContext('2d');
        const statusChart = new Chart(statusCtx, {{
            type: 'doughnut',
            data: {{
                labels: ['Completed', 'Remaining'],
                datasets: [{{
                    data: [{:.1}, {:.1}],
                    backgroundColor: [
                        'rgba(75, 192, 192, 0.5)',
                        'rgba(201, 203, 207, 0.5)'
                    ],
                    borderColor: [
                        'rgba(75, 192, 192, 1)',
                        'rgba(201, 203, 207, 1)'
                    ],
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    legend: {{
                        position: 'bottom',
                    }},
                    title: {{
                        display: true,
                        text: 'Project Completion: {:.1}%'
                    }}
                }}
            }}
        }});
"#, 
            result.project_status.completion_percentage,
            100.0 - result.project_status.completion_percentage,
            result.project_status.completion_percentage
        ));
        
        // Testing Chart
        html.push_str(&format!(r#"        // Testing Chart
        const testingCtx = document.getElementById('testingChart').getContext('2d');
        const testingChart = new Chart(testingCtx, {{
            type: 'pie',
            data: {{
                labels: ['Passing Tests', 'Failing Tests'],
                datasets: [{{
                    data: [{}, {}],
                    backgroundColor: [
                        'rgba(54, 162, 235, 0.5)',
                        'rgba(255, 99, 132, 0.5)'
                    ],
                    borderColor: [
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 99, 132, 1)'
                    ],
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    legend: {{
                        position: 'bottom',
                    }},
                    title: {{
                        display: true,
                        text: 'Test Coverage: {:.1}%'
                    }}
                }}
            }}
        }});
"#, 
            result.tests.passing,
            result.tests.total - result.tests.passing,
            result.tests.coverage
        ));
        
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>");
        
        Ok(html)
    }
    
    /// Copy chart assets to the output directory
    fn copy_chart_assets(&self) -> Result<(), String> {
        // No need to copy assets as we're using CDN for Chart.js
        Ok(())
    }
    
    /// Generate a simple metrics report in Markdown format
    pub fn generate_metrics_report(&self, result: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Generating metrics report...");
        
        // Ensure docs directory exists
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)
                .map_err(|e| format!("Failed to create docs directory: {}", e))?;
        }
        
        // Generate report content
        let mut content = String::from("# Project Metrics Report\n\n");
        content.push_str(&format!("_Generated on: {}_\n\n", 
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        // Overall status
        content.push_str("## Project Status\n\n");
        content.push_str(&format!("- **Phase**: {}\n", result.project_status.phase));
        content.push_str(&format!("- **Completion**: {:.1}%\n", result.project_status.completion_percentage));
        content.push_str(&format!("- **Last Active Area**: {}\n", result.project_status.last_active_area));
        
        if let Some(date) = result.project_status.estimated_completion_date {
            content.push_str(&format!("- **Estimated Completion Date**: {}\n", 
                date.format("%Y-%m-%d")));
        }
        
        // Implementation metrics
        content.push_str("\n## Implementation Metrics\n\n");
        content.push_str("| Component | Implemented | Total | Percentage |\n");
        content.push_str("|-----------|-------------|-------|------------|\n");
        content.push_str(&format!("| Models | {} | {} | {:.1}% |\n", 
            result.models.implemented, result.models.total, result.models.implementation_percentage));
        content.push_str(&format!("| API Endpoints | {} | {} | {:.1}% |\n", 
            result.api_endpoints.implemented, result.api_endpoints.total, result.api_endpoints.implementation_percentage));
        content.push_str(&format!("| UI Components | {} | {} | {:.1}% |\n", 
            result.ui_components.implemented, result.ui_components.total, result.ui_components.implementation_percentage));
        content.push_str(&format!("| Integration Points | {} | {} | {:.1}% |\n", 
            result.integration.implemented, result.integration.total, result.integration.implementation_percentage));
        
        // Code quality metrics
        content.push_str("\n## Code Quality Metrics\n\n");
        for (metric, value) in &result.code_quality.metrics {
            content.push_str(&format!("- **{}**: {:.1}\n", metric, value));
        }
        
        // Testing metrics
        content.push_str("\n## Testing Metrics\n\n");
        content.push_str(&format!("- **Total Tests**: {}\n", result.tests.total));
        content.push_str(&format!("- **Passing Tests**: {}\n", result.tests.passing));
        content.push_str(&format!("- **Test Coverage**: {:.1}%\n", result.tests.coverage));
        
        // Write the report file
        let report_path = docs_dir.join("metrics_report.md");
        fs::write(&report_path, content)
            .map_err(|e| format!("Failed to write metrics report: {}", e))?;
        
        println!("Metrics report generated at: {:?}", report_path);
        Ok(report_path)
    }
}
