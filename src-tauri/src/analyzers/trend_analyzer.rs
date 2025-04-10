use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, Utc};

use crate::analyzers::unified_analyzer::AnalysisResult;

/// Struct for tracking historical trends in the project
pub struct TrendAnalyzer {
    base_dir: PathBuf,
    history_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendPoint {
    pub timestamp: DateTime<Utc>,
    pub overall_completion: f32,
    pub models_completion: f32,
    pub api_completion: f32,
    pub ui_completion: f32,
    pub test_coverage: f32,
    pub total_files: i32,
    pub rust_files: i32,
    pub js_files: i32,
    pub lines_of_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendData {
    pub data_points: Vec<TrendPoint>,
    pub last_updated: DateTime<Utc>,
}

impl TrendAnalyzer {
    pub fn new(base_dir: PathBuf) -> Self {
        let history_dir = base_dir.join("analysis_history");
        
        Self {
            base_dir,
            history_dir,
        }
    }
    
    /// Record the current analysis result as a historical data point
    pub fn record_analysis(&self, result: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Recording analysis for historical trend tracking...");
        
        // Ensure history directory exists
        if !self.history_dir.exists() {
            fs::create_dir_all(&self.history_dir)
                .map_err(|e| format!("Failed to create history directory: {}", e))?;
        }
        
        // Create a trend point from the current analysis
        let trend_point = TrendPoint {
            timestamp: result.timestamp,
            overall_completion: result.project_status.overall_completion_percentage,
            models_completion: result.models.implementation_percentage,
            api_completion: result.api_endpoints.implementation_percentage,
            ui_completion: result.ui_components.implementation_percentage,
            test_coverage: result.tests.coverage_percentage,
            total_files: *result.project_status.file_stats.get("total").unwrap_or(&0),
            rust_files: *result.project_status.file_stats.get("rs").unwrap_or(&0),
            js_files: *result.project_status.file_stats.get("js").unwrap_or(&0),
            lines_of_code: result.project_status.total_lines_of_code,
        };
        
        // Load existing trend data or create new
        let mut trend_data = self.load_trend_data()?;
        
        // Add the new data point
        trend_data.data_points.push(trend_point);
        trend_data.last_updated = Utc::now();
        
        // Save updated trend data
        let trend_file = self.history_dir.join("trend_data.json");
        let json = serde_json::to_string_pretty(&trend_data)
            .map_err(|e| format!("Failed to serialize trend data: {}", e))?;
        
        fs::write(&trend_file, json)
            .map_err(|e| format!("Failed to write trend data: {}", e))?;
        
        // Generate trend report
        self.generate_trend_report(&trend_data)?;
        
        println!("Analysis recorded in trend history");
        Ok(trend_file)
    }
    
    /// Load existing trend data or create new if none exists
    fn load_trend_data(&self) -> Result<TrendData, String> {
        let trend_file = self.history_dir.join("trend_data.json");
        
        if trend_file.exists() {
            // Load existing data
            let json = fs::read_to_string(&trend_file)
                .map_err(|e| format!("Failed to read trend data: {}", e))?;
            
            serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse trend data: {}", e))
        } else {
            // Create new trend data
            Ok(TrendData {
                data_points: Vec::new(),
                last_updated: Utc::now(),
            })
        }
    }
    
    /// Generate a trend report with visualizations
    fn generate_trend_report(&self, trend_data: &TrendData) -> Result<PathBuf, String> {
        // Ensure the docs directory exists
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)
                .map_err(|e| format!("Failed to create docs directory: {}", e))?;
        }
        
        // Generate the trend report
        let trend_report = self.generate_trend_html(trend_data)?;
        let report_path = docs_dir.join("trend_analysis.html");
        
        fs::write(&report_path, trend_report)
            .map_err(|e| format!("Failed to write trend report: {}", e))?;
        
        // Generate markdown summary
        let markdown_summary = self.generate_markdown_summary(trend_data)?;
        let summary_path = docs_dir.join("trend_analysis.md");
        
        fs::write(&summary_path, markdown_summary)
            .map_err(|e| format!("Failed to write trend summary: {}", e))?;
        
        println!("Trend report generated at: {:?}", report_path);
        Ok(report_path)
    }
    
    /// Generate HTML trend report with charts
    fn generate_trend_html(&self, trend_data: &TrendData) -> Result<String, String> {
        // Prepare data for charts
        let timestamps: Vec<String> = trend_data.data_points.iter()
            .map(|p| format!("'{}'", p.timestamp.format("%Y-%m-%d")))
            .collect();
        
        let overall_completion: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.overall_completion.to_string())
            .collect();
        
        let models_completion: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.models_completion.to_string())
            .collect();
        
        let api_completion: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.api_completion.to_string())
            .collect();
        
        let ui_completion: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.ui_completion.to_string())
            .collect();
        
        let test_coverage: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.test_coverage.to_string())
            .collect();
        
        let total_files: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.total_files.to_string())
            .collect();
        
        let rust_files: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.rust_files.to_string())
            .collect();
        
        let js_files: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.js_files.to_string())
            .collect();
        
        let lines_of_code: Vec<String> = trend_data.data_points.iter()
            .map(|p| p.lines_of_code.to_string())
            .collect();
        
        // Calculate growth rates for key metrics
        let growth_rates = self.calculate_growth_rates(trend_data);
        
        // Generate HTML
        let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>LMS Project Trend Analysis</title>
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
        .metric-growth {{
            font-size: 14px;
            margin-top: 5px;
        }}
        .growth-positive {{
            color: #28a745;
        }}
        .growth-negative {{
            color: #dc3545;
        }}
        .growth-neutral {{
            color: #6c757d;
        }}
        .metric-label {{
            font-size: 14px;
            color: #666;
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
            <h1>LMS Project Trend Analysis</h1>
            <p>Historical trends and progress analysis for the Canvas-Discourse integration project</p>
        </div>
        
        <!-- Growth Metrics -->
        <div class="metrics-summary">
            <div class="metric-card">
                <div class="metric-label">Overall Completion Growth</div>
                <div class="metric-value">{:.1}%</div>
                <div class="metric-growth {}">{:+.2}% per week</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">Models Implementation Growth</div>
                <div class="metric-value">{:.1}%</div>
                <div class="metric-growth {}">{:+.2}% per week</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">API Implementation Growth</div>
                <div class="metric-value">{:.1}%</div>
                <div class="metric-growth {}">{:+.2}% per week</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">UI Implementation Growth</div>
                <div class="metric-value">{:.1}%</div>
                <div class="metric-growth {}">{:+.2}% per week</div>
            </div>
        </div>
        
        <!-- Charts Grid -->
        <div class="chart-grid">
            <!-- Overall Completion Trend Chart -->
            <div class="chart-container">
                <h3>Overall Completion Trend</h3>
                <canvas id="overallTrendChart"></canvas>
            </div>
            
            <!-- Component Completion Trend Chart -->
            <div class="chart-container">
                <h3>Component Completion Trends</h3>
                <canvas id="componentTrendChart"></canvas>
            </div>
            
            <!-- Code Size Trend Chart -->
            <div class="chart-container">
                <h3>Code Base Size Trend</h3>
                <canvas id="codeSizeTrendChart"></canvas>
            </div>
            
            <!-- File Types Trend Chart -->
            <div class="chart-container">
                <h3>File Types Trend</h3>
                <canvas id="fileTypesTrendChart"></canvas>
            </div>
        </div>
        
        <div class="last-updated">
            Last updated: {:%Y-%m-%d %H:%M:%S}
        </div>
    </div>
    
    <script>
        // Overall Completion Trend Chart
        const overallTrendCtx = document.getElementById('overallTrendChart').getContext('2d');
        const overallTrendChart = new Chart(overallTrendCtx, {{
            type: 'line',
            data: {{
                labels: [{timestamps.join(", ")}],
                datasets: [{{
                    label: 'Overall Completion (%)',
                    data: [{overall_completion.join(", ")}],
                    borderColor: 'rgb(54, 162, 235)',
                    backgroundColor: 'rgba(54, 162, 235, 0.1)',
                    tension: 0.3,
                    fill: true
                }}]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true,
                        suggestedMax: 100,
                        title: {{
                            display: true,
                            text: 'Completion Percentage'
                        }}
                    }}
                }},
                plugins: {{
                    legend: {{
                        position: 'top',
                    }}
                }}
            }}
        }});
        
        // Component Completion Trend Chart
        const componentTrendCtx = document.getElementById('componentTrendChart').getContext('2d');
        const componentTrendChart = new Chart(componentTrendCtx, {{
            type: 'line',
            data: {{
                labels: [{timestamps.join(", ")}],
                datasets: [
                    {{
                        label: 'Models',
                        data: [{models_completion.join(", ")}],
                        borderColor: 'rgb(54, 162, 235)',
                        backgroundColor: 'rgba(54, 162, 235, 0.1)',
                        tension: 0.3,
                        fill: false
                    }},
                    {{
                        label: 'API Endpoints',
                        data: [{api_completion.join(", ")}],
                        borderColor: 'rgb(255, 99, 132)',
                        backgroundColor: 'rgba(255, 99, 132, 0.1)',
                        tension: 0.3,
                        fill: false
                    }},
                    {{
                        label: 'UI Components',
                        data: [{ui_completion.join(", ")}],
                        borderColor: 'rgb(75, 192, 192)',
                        backgroundColor: 'rgba(75, 192, 192, 0.1)',
                        tension: 0.3,
                        fill: false
                    }},
                    {{
                        label: 'Test Coverage',
                        data: [{test_coverage.join(", ")}],
                        borderColor: 'rgb(255, 206, 86)',
                        backgroundColor: 'rgba(255, 206, 86, 0.1)',
                        tension: 0.3,
                        fill: false
                    }}
                ]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true,
                        suggestedMax: 100,
                        title: {{
                            display: true,
                            text: 'Completion Percentage'
                        }}
                    }}
                }},
                plugins: {{
                    legend: {{
                        position: 'top',
                    }}
                }}
            }}
        }});
        
        // Code Size Trend Chart
        const codeSizeTrendCtx = document.getElementById('codeSizeTrendChart').getContext('2d');
        const codeSizeTrendChart = new Chart(codeSizeTrendCtx, {{
            type: 'line',
            data: {{
                labels: [{timestamps.join(", ")}],
                datasets: [{{
                    label: 'Lines of Code',
                    data: [{lines_of_code.join(", ")}],
                    borderColor: 'rgb(153, 102, 255)',
                    backgroundColor: 'rgba(153, 102, 255, 0.1)',
                    tension: 0.3,
                    fill: true,
                    yAxisID: 'y'
                }}]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        type: 'linear',
                        display: true,
                        position: 'left',
                        title: {{
                            display: true,
                            text: 'Lines of Code'
                        }}
                    }}
                }},
                plugins: {{
                    legend: {{
                        position: 'top',
                    }}
                }}
            }}
        }});
        
        // File Types Trend Chart
        const fileTypesTrendCtx = document.getElementById('fileTypesTrendChart').getContext('2d');
        const fileTypesTrendChart = new Chart(fileTypesTrendCtx, {{
            type: 'line',
            data: {{
                labels: [{timestamps.join(", ")}],
                datasets: [
                    {{
                        label: 'Total Files',
                        data: [{total_files.join(", ")}],
                        borderColor: 'rgb(54, 162, 235)',
                        backgroundColor: 'rgba(54, 162, 235, 0.1)',
                        tension: 0.3,
                        fill: false
                    }},
                    {{
                        label: 'Rust Files',
                        data: [{rust_files.join(", ")}],
                        borderColor: 'rgb(255, 99, 132)',
                        backgroundColor: 'rgba(255, 99, 132, 0.1)',
                        tension: 0.3,
                        fill: false
                    }},
                    {{
                        label: 'JavaScript Files',
                        data: [{js_files.join(", ")}],
                        borderColor: 'rgb(255, 206, 86)',
                        backgroundColor: 'rgba(255, 206, 86, 0.1)',
                        tension: 0.3,
                        fill: false
                    }}
                ]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Number of Files'
                        }}
                    }}
                }},
                plugins: {{
                    legend: {{
                        position: 'top',
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>
"#,
            // Overall completion growth
            if let Some(point) = trend_data.data_points.last() { point.overall_completion } else { 0.0 },
            Self::get_growth_class(growth_rates.overall_completion),
            growth_rates.overall_completion,
            
            // Models implementation growth
            if let Some(point) = trend_data.data_points.last() { point.models_completion } else { 0.0 },
            Self::get_growth_class(growth_rates.models_completion),
            growth_rates.models_completion,
            
            // API implementation growth
            if let Some(point) = trend_data.data_points.last() { point.api_completion } else { 0.0 },
            Self::get_growth_class(growth_rates.api_completion),
            growth_rates.api_completion,
            
            // UI implementation growth
            if let Some(point) = trend_data.data_points.last() { point.ui_completion } else { 0.0 },
            Self::get_growth_class(growth_rates.ui_completion),
            growth_rates.ui_completion,
            
            // Last updated timestamp
            Local::now()
        );
        
        Ok(html)
    }
    
    /// Generate markdown summary of trends
    fn generate_markdown_summary(&self, trend_data: &TrendData) -> Result<String, String> {
        // Calculate growth rates
        let growth_rates = self.calculate_growth_rates(trend_data);
        
        // Get the latest data point
        let latest = trend_data.data_points.last();
        
        // Generate markdown
        let mut markdown = String::new();
        
        // Header
        markdown.push_str("# LMS Project Trend Analysis\n\n");
        markdown.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
        
        // Overview
        markdown.push_str("## Overview\n\n");
        markdown.push_str("This document analyzes historical trends in the LMS project implementation.\n\n");
        
        // Latest Status
        if let Some(point) = latest {
            markdown.push_str("## Current Status\n\n");
            markdown.push_str("| Metric | Value |\n");
            markdown.push_str("|--------|-------|\n");
            markdown.push_str(&format!("| Overall Completion | {:.1}% |\n", point.overall_completion));
            markdown.push_str(&format!("| Models Implementation | {:.1}% |\n", point.models_completion));
            markdown.push_str(&format!("| API Implementation | {:.1}% |\n", point.api_completion));
            markdown.push_str(&format!("| UI Implementation | {:.1}% |\n", point.ui_completion));
            markdown.push_str(&format!("| Test Coverage | {:.1}% |\n", point.test_coverage));
            markdown.push_str(&format!("| Total Files | {} |\n", point.total_files));
            markdown.push_str(&format!("| Lines of Code | {} |\n", point.lines_of_code));
            markdown.push_str("\n");
        }
        
        // Growth Rates
        markdown.push_str("## Growth Rates\n\n");
        markdown.push_str("| Metric | Weekly Growth |\n");
        markdown.push_str("|--------|---------------|\n");
        markdown.push_str(&format!("| Overall Completion | {:+.2}% |\n", growth_rates.overall_completion));
        markdown.push_str(&format!("| Models Implementation | {:+.2}% |\n", growth_rates.models_completion));
        markdown.push_str(&format!("| API Implementation | {:+.2}% |\n", growth_rates.api_completion));
        markdown.push_str(&format!("| UI Implementation | {:+.2}% |\n", growth_rates.ui_completion));
        markdown.push_str(&format!("| Test Coverage | {:+.2}% |\n", growth_rates.test_coverage));
        markdown.push_str("\n");
        
        // Historical Data Points
        markdown.push_str("## Historical Data Points\n\n");
        markdown.push_str("| Date | Overall | Models | APIs | UI | Tests |\n");
        markdown.push_str("|------|---------|--------|------|----|-----------|\n");
        
        for point in &trend_data.data_points {
            markdown.push_str(&format!("| {} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |\n",
                point.timestamp.format("%Y-%m-%d"),
                point.overall_completion,
                point.models_completion,
                point.api_completion,
                point.ui_completion,
                point.test_coverage
            ));
        }
        markdown.push_str("\n");
        
        // Analysis and Recommendations
        markdown.push_str("## Analysis and Recommendations\n\n");
        
        // Overall progress analysis
        if growth_rates.overall_completion > 5.0 {
            markdown.push_str("- **Excellent Progress**: The project is advancing rapidly, maintain this momentum.\n");
        } else if growth_rates.overall_completion > 2.0 {
            markdown.push_str("- **Good Progress**: The project is advancing at a sustainable rate.\n");
        } else if growth_rates.overall_completion > 0.0 {
            markdown.push_str("- **Slow Progress**: The project is advancing slower than ideal, consider increasing resources.\n");
        } else {
            markdown.push_str("- **Stalled Progress**: The project appears to be stalled, immediate attention is required.\n");
        }
        
        // Component-specific recommendations
        let mut slowest_component = "unknown";
        let mut slowest_rate = f32::MAX;
        
        if growth_rates.models_completion < slowest_rate {
            slowest_component = "Models";
            slowest_rate = growth_rates.models_completion;
        }
        
        if growth_rates.api_completion < slowest_rate {
            slowest_component = "API Endpoints";
            slowest_rate = growth_rates.api_completion;
        }
        
        if growth_rates.ui_completion < slowest_rate {
            slowest_component = "UI Components";
            slowest_rate = growth_rates.ui_completion;
        }
        
        if growth_rates.test_coverage < slowest_rate {
            slowest_component = "Tests";
        }
        
        markdown.push_str(&format!("- **Focus Area**: The slowest progressing component is **{}**. Consider dedicating more resources to this area.\n", slowest_component));
        
        // Code size analysis
        if trend_data.data_points.len() >= 2 {
            let first = &trend_data.data_points[0];
            let last = trend_data.data_points.last().unwrap();
            
            let loc_growth = last.lines_of_code as f32 - first.lines_of_code as f32;
            let days = (last.timestamp - first.timestamp).num_days();
            
            if days > 0 {
                let daily_loc = loc_growth / days as f32;
                markdown.push_str(&format!("- **Code Growth**: Adding approximately {:.0} lines of code per day.\n", daily_loc));
            }
            
            // JavaScript to Rust migration trend
            let js_change = last.js_files as f32 - first.js_files as f32;
            let rust_change = last.rust_files as f32 - first.rust_files as f32;
            
            if js_change < 0.0 && rust_change > 0.0 {
                markdown.push_str("- **Migration Trend**: Successfully migrating from JavaScript to Rust as planned.\n");
            } else if js_change > 0.0 {
                markdown.push_str("- **Migration Warning**: JavaScript codebase is growing instead of shrinking. Review the migration strategy.\n");
            }
        }
        
        // Timeline projection
        if let (Some(latest), Some(earliest)) = (trend_data.data_points.last(), trend_data.data_points.first()) {
            if latest.overall_completion > earliest.overall_completion && latest.overall_completion < 100.0 {
                let completion_diff = latest.overall_completion - earliest.overall_completion;
                let days_diff = (latest.timestamp - earliest.timestamp).num_days();
                
                if days_diff > 0 {
                    let daily_progress = completion_diff / days_diff as f32;
                    let days_to_completion = if daily_progress > 0.0 {
                        (100.0 - latest.overall_completion) / daily_progress
                    } else {
                        999.0 // Large number to indicate slow progress
                    };
                    
                    let projected_completion = Utc::now() + chrono::Duration::days(days_to_completion as i64);
                    
                    markdown.push_str(&format!("- **Projected Completion**: At the current rate, the project will be completed around **{}**.\n", 
                        projected_completion.format("%Y-%m-%d")));
                }
            }
        }
        
        // Final note
        markdown.push_str("\n_Note: This is an automatically generated analysis based on historical data. Use this information to guide project planning and resource allocation._\n");
        
        Ok(markdown)
    }
    
    /// Calculate growth rates for key metrics
    fn calculate_growth_rates(&self, trend_data: &TrendData) -> GrowthRates {
        // Default growth rates
        let mut rates = GrowthRates {
            overall_completion: 0.0,
            models_completion: 0.0,
            api_completion: 0.0,
            ui_completion: 0.0,
            test_coverage: 0.0,
        };
        
        // Need at least two data points to calculate growth
        if trend_data.data_points.len() < 2 {
            return rates;
        }
        
        // Get the first and last data points
        let first = &trend_data.data_points[0];
        let last = trend_data.data_points.last().unwrap();
        
        // Calculate time difference in weeks
        let time_diff = (last.timestamp - first.timestamp).num_seconds() as f32 / 604800.0; // seconds in a week
        
        // If time diff is too small, return default rates
        if time_diff < 0.1 {
            return rates;
        }
        
        // Calculate weekly growth rates
        rates.overall_completion = (last.overall_completion - first.overall_completion) / time_diff;
        rates.models_completion = (last.models_completion - first.models_completion) / time_diff;
        rates.api_completion = (last.api_completion - first.api_completion) / time_diff;
        rates.ui_completion = (last.ui_completion - first.ui_completion) / time_diff;
        rates.test_coverage = (last.test_coverage - first.test_coverage) / time_diff;
        
        rates
    }
    
    /// Get the appropriate CSS class for growth rate
    fn get_growth_class(rate: f32) -> &'static str {
        if rate > 0.5 {
            "growth-positive"
        } else if rate < -0.5 {
            "growth-negative"
        } else {
            "growth-neutral"
        }
    }
}

/// Helper struct for growth rates
struct GrowthRates {
    overall_completion: f32,
    models_completion: f32,
    api_completion: f32,
    ui_completion: f32,
    test_coverage: f32,
}
