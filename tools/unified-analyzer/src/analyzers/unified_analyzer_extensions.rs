use std::fs;
use chrono::Utc;
use crate::analyzers::unified_analyzer::{UnifiedProjectAnalyzer, AnalysisResult};

impl UnifiedProjectAnalyzer {
    /// Generate AI knowledge base
    pub async fn generate_ai_knowledge_base(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating AI knowledge base...");
        
        let result = self.result.lock().await.clone();
        
        // Create AI knowledge directory
        let knowledge_dir = self.base_dir.join("ai_knowledge");
        if !knowledge_dir.exists() {
            fs::create_dir_all(&knowledge_dir)?;
        }
        
        // Generate structured knowledge from analysis
        let knowledge_file = knowledge_dir.join("project_knowledge.json");
        let knowledge_content = self.generate_ai_knowledge_content(&result)?;
        
        fs::write(&knowledge_file, knowledge_content)?;
        println!("AI knowledge base generated at {:?}", knowledge_file);
        
        // Generate AI agent guidance
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)?;
        }
        
        let guidance_file = docs_dir.join("ai_agent_guidance.md");
        let guidance_content = self.generate_ai_guidance_content(&result)?;
        
        fs::write(&guidance_file, guidance_content)?;
        println!("AI agent guidance generated at {:?}", guidance_file);
        
        Ok(())
    }
    
    /// Generate AI knowledge content
    fn generate_ai_knowledge_content(&self, result: &AnalysisResult) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        #[derive(serde::Serialize)]
        struct KnowledgeSection {
            title: String,
            content: String,
            keywords: Vec<String>,
            importance: u8,
        }
        
        #[derive(serde::Serialize)]
        struct ProjectKnowledge {
            sections: Vec<KnowledgeSection>,
            last_updated: String,
        }
        
        let mut sections = Vec::new();
        
        // Project overview section
        sections.push(KnowledgeSection {
            title: "Project Overview".to_string(),
            content: format!(
                "The project is currently in the {} phase with {:.1}% completion. \
                The most active area recently has been {}.",
                result.project_status.phase,
                result.project_status.completion_percentage,
                result.project_status.last_active_area
            ),
            keywords: vec!["overview".to_string(), "status".to_string(), "phase".to_string()],
            importance: 10,
        });
        
        // Architecture section
        sections.push(KnowledgeSection {
            title: "Architecture".to_string(),
            content: format!(
                "The project follows these architectural frameworks: {}. \
                And uses these design patterns: {}.",
                result.architecture.frameworks.join(", "),
                result.architecture.design_patterns.join(", ")
            ),
            keywords: vec!["architecture".to_string(), "frameworks".to_string(), "design_patterns".to_string()],
            importance: 9,
        });
        
        // Models section
        sections.push(KnowledgeSection {
            title: "Data Models".to_string(),
            content: format!(
                "The project has implemented {}/{} data models ({:.1}% complete).",
                result.models.implemented,
                result.models.total,
                result.models.implementation_percentage
            ),
            keywords: vec!["models".to_string(), "data".to_string(), "entities".to_string()],
            importance: 8,
        });
        
        let knowledge = ProjectKnowledge {
            sections,
            last_updated: Utc::now().to_rfc3339(),
        };
        
        let json = serde_json::to_string_pretty(&knowledge)?;
        Ok(json)
    }
    
    /// Generate AI guidance content
    fn generate_ai_guidance_content(&self, result: &AnalysisResult) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut content = String::from("# AI Development Guidance\n\n");
        content.push_str("This document provides guidance for AI agents working on this project.\n\n");
        
        // Project overview
        content.push_str("## Project Overview\n\n");
        content.push_str(&format!(
            "The project is currently in the **{}** phase with **{:.1}%** completion. \
            The most active area recently has been **{}**.\n\n",
            result.project_status.phase,
            result.project_status.completion_percentage,
            result.project_status.last_active_area
        ));
        
        // Architecture guidance
        content.push_str("## Architecture Guidance\n\n");
        content.push_str("When developing new features or modifying existing ones, adhere to the following frameworks:\n\n");
        
        for framework in &result.architecture.frameworks {
            content.push_str(&format!("- **{}**\n", framework));
        }
        
        content.push_str("\nImplement these design patterns when appropriate:\n\n");
        
        for pattern in &result.architecture.design_patterns {
            content.push_str(&format!("- **{}**\n", pattern));
        }
        
        // Development priorities
        content.push_str("\n## Development Priorities\n\n");
        content.push_str("Focus on these areas in order of priority:\n\n");
        
        // Sort recommendations by priority
        let mut recommendations = result.recommendations.clone();
        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        for rec in recommendations.iter().take(5) {
            content.push_str(&format!("1. **{}**: {}\n", rec.area, rec.description));
        }
        
        // Code quality guidance
        content.push_str("\n## Code Quality Guidance\n\n");
        content.push_str("Maintain or improve these code quality metrics:\n\n");
        
        for (metric, value) in &result.code_quality.metrics {
            content.push_str(&format!("- **{}**: {:.1} or better\n", metric, value));
        }
        
        Ok(content)
    }
    
    /// Generate metrics visualizations
    pub async fn generate_metrics_visualizations(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating metrics visualizations...");
        
        let result = self.result.lock().await.clone();
        
        // Create visualizations directory
        let vis_dir = self.base_dir.join("docs").join("visualizations");
        if !vis_dir.exists() {
            fs::create_dir_all(&vis_dir)?;
        }
        
        // Generate metrics dashboard
        let dashboard_path = vis_dir.join("metrics_dashboard.html");
        let dashboard_content = self.generate_metrics_dashboard_html(&result)?;
        
        fs::write(&dashboard_path, dashboard_content)?;
        println!("Metrics dashboard generated at {:?}", dashboard_path);
        
        // Generate metrics report
        let report_path = self.base_dir.join("docs").join("metrics_report.md");
        let report_content = self.generate_metrics_report_content(&result)?;
        
        fs::write(&report_path, report_content)?;
        println!("Metrics report generated at {:?}", report_path);
        
        Ok(())
    }
    
    /// Generate metrics dashboard HTML
    fn generate_metrics_dashboard_html(&self, result: &AnalysisResult) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Project Metrics Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {
            font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
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
    </style>
</head>
<body>
    <div class="header">
        <h1>Project Metrics Dashboard</h1>
        <p>Last updated: "#);
        
        // Add timestamp
        html.push_str(&Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string());
        html.push_str("</p>\n    </div>\n\n");
        
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
"#);
        
        // Add JavaScript for charts
        html.push_str("    <script>\n");
        
        // Implementation Progress Chart
        html.push_str(&format!(r#"        // Implementation Progress Chart
        const implementationCtx = document.getElementById("implementationChart").getContext("2d");
        const implementationChart = new Chart(implementationCtx, {{
            type: "bar",
            data: {{
                labels: ["Models", "API Endpoints", "UI Components", "Integration"],
                datasets: [{{
                    label: "Implementation Percentage",
                    data: [{:.1}, {:.1}, {:.1}, {:.1}],
                    backgroundColor: [
                        "rgba(54, 162, 235, 0.5)",
                        "rgba(255, 99, 132, 0.5)",
                        "rgba(75, 192, 192, 0.5)",
                        "rgba(255, 159, 64, 0.5)"
                    ],
                    borderColor: [
                        "rgba(54, 162, 235, 1)",
                        "rgba(255, 99, 132, 1)",
                        "rgba(75, 192, 192, 1)",
                        "rgba(255, 159, 64, 1)"
                    ],
                    borderWidth: 1
                }}]
            }},
            options: {{
                scales: {{
                    y: {{
                        beginAtZero: true,
                        max: 100
                    }}
                }}
            }}
        }});
"#, 
            result.models.implementation_percentage,
            result.api_endpoints.implementation_percentage,
            result.ui_components.implementation_percentage,
            result.integration.implementation_percentage
        ));
        
        // Code Quality Chart
        html.push_str("        // Code Quality Chart\n");
        html.push_str("        const qualityCtx = document.getElementById(\"qualityChart\").getContext(\"2d\");\n");
        html.push_str("        const qualityChart = new Chart(qualityCtx, {\n");
        html.push_str("            type: \"radar\",\n");
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
        html.push_str("                    label: \"Code Quality\",\n");
        html.push_str(&format!("                    data: [{}],\n", metrics_values.join(", ")));
        html.push_str("                    backgroundColor: \"rgba(75, 192, 192, 0.2)\",\n");
        html.push_str("                    borderColor: \"rgba(75, 192, 192, 1)\",\n");
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
        
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>");
        
        Ok(html)
    }
    
    /// Generate metrics report content
    fn generate_metrics_report_content(&self, result: &AnalysisResult) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
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
            result.integration.implemented_points, result.integration.total_points, result.integration.implementation_percentage));
        
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
        
        Ok(content)
    }
    
    /// Generate project dashboard
    pub async fn generate_project_dashboard(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Generating project dashboard...");
        
        let result = self.result.lock().await.clone();
        
        // Create docs directory
        let docs_dir = self.base_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)?;
        }
        
        // Generate dashboard HTML
        let dashboard_path = docs_dir.join("dashboard.html");
        let dashboard_content = self.generate_project_dashboard_html(&result)?;
        
        fs::write(&dashboard_path, dashboard_content)?;
        println!("Project dashboard generated at {:?}", dashboard_path);
        
        // Update history
        self.update_dashboard_history(&result)?;
        
        Ok(())
    }
    
    /// Generate project dashboard HTML
    fn generate_project_dashboard_html(&self, result: &AnalysisResult) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Project Dashboard</title>
    <style>
        body {
            font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
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
    </style>
</head>
<body>
    <div class="dashboard-container">
        <div class="header">
            <h1>Project Dashboard</h1>
            <p>Last updated: "#);
        
        // Add timestamp
        html.push_str(&Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string());
        html.push_str("</p>\n        </div>\n\n");
        
        // Overall progress
        html.push_str(&format!(r#"        <div class="card">
            <h2>Overall Progress: {:.1}%</h2>
            <div class="progress-bar">
                <div class="progress-fill" style="width: {:.1}%;"></div>
            </div>
        </div>

"#, result.project_status.completion_percentage, result.project_status.completion_percentage));
        
        // Feature areas
        html.push_str(r#"        <div class="card">
            <h2>Feature Areas</h2>
            <div class="progress-container">
"#);
        
        // Models
        html.push_str(&format!(r#"                <div class="progress-card">
                    <h3>Models</h3>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {:.1}%;"></div>
                    </div>
                    <div class="progress-text">
                        <span>{}/{}</span>
                        <span>{:.1}%</span>
                    </div>
                </div>
"#, 
            result.models.implementation_percentage,
            result.models.implemented,
            result.models.total,
            result.models.implementation_percentage
        ));
        
        // API Endpoints
        html.push_str(&format!(r#"                <div class="progress-card">
                    <h3>API Endpoints</h3>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {:.1}%;"></div>
                    </div>
                    <div class="progress-text">
                        <span>{}/{}</span>
                        <span>{:.1}%</span>
                    </div>
                </div>
"#, 
            result.api_endpoints.implementation_percentage,
            result.api_endpoints.implemented,
            result.api_endpoints.total,
            result.api_endpoints.implementation_percentage
        ));
        
        // UI Components
        html.push_str(&format!(r#"                <div class="progress-card">
                    <h3>UI Components</h3>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {:.1}%;"></div>
                    </div>
                    <div class="progress-text">
                        <span>{}/{}</span>
                        <span>{:.1}%</span>
                    </div>
                </div>
"#, 
            result.ui_components.implementation_percentage,
            result.ui_components.implemented,
            result.ui_components.total,
            result.ui_components.implementation_percentage
        ));
        
        html.push_str("            </div>\n        </div>\n\n");
        
        // Columns for Recent Changes and Next Steps
        html.push_str(r#"        <div class="columns">
            <div class="column">
                <div class="card">
                    <h2>Recent Changes</h2>
                    <ul>
                        <li>Updated model implementations</li>
                        <li>Added new API endpoints</li>
                        <li>Improved test coverage</li>
                    </ul>
                </div>
            </div>
            <div class="column">
                <div class="card">
                    <h2>Next Steps</h2>
                    <ul>
"#);
        
        // Add recommendations as next steps
        for rec in result.recommendations.iter().take(5) {
            html.push_str(&format!("                        <li>{}: {}</li>\n", rec.area, rec.description));
        }
        
        html.push_str(r#"                    </ul>
                </div>
            </div>
        </div>
"#);
        
        html.push_str("    </div>\n</body>\n</html>");
        
        Ok(html)
    }
    
    /// Update dashboard history
    fn update_dashboard_history(&self, result: &AnalysisResult) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        #[derive(serde::Serialize, serde::Deserialize, Clone)]
        struct HistoryEntry {
            timestamp: String,
            completion_percentage: f32,
            models_percentage: f32,
            api_endpoints_percentage: f32,
            ui_components_percentage: f32,
        }
        
        // Create history directory
        let history_dir = self.base_dir.join("docs").join("history");
        if !history_dir.exists() {
            fs::create_dir_all(&history_dir)?;
        }
        
        // Create history file path
        let history_file = history_dir.join("dashboard_history.json");
        
        // Read existing history or create new
        let mut history: Vec<HistoryEntry> = if history_file.exists() {
            let content = fs::read_to_string(&history_file)?;
            serde_json::from_str(&content)?
        } else {
            Vec::new()
        };
        
        // Add new entry
        history.push(HistoryEntry {
            timestamp: Utc::now().to_rfc3339(),
            completion_percentage: result.project_status.completion_percentage,
            models_percentage: result.models.implementation_percentage,
            api_endpoints_percentage: result.api_endpoints.implementation_percentage,
            ui_components_percentage: result.ui_components.implementation_percentage,
        });
        
        // Limit history to last 30 entries
        if history.len() > 30 {
            let skip_count = history.len() - 30;
            history = history.clone().into_iter().skip(skip_count).collect();
        }
        
        // Write updated history
        let content = serde_json::to_string_pretty(&history)?;
        fs::write(&history_file, content)?;
        
        Ok(())
    }
}
