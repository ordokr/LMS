use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;

/// Generate dashboard
pub fn generate_dashboard(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating dashboard...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Create the dashboard path
    let dashboard_path = docs_dir.join("dashboard.html");
    
    // Generate the content
    let content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>LMS Project Dashboard</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body class="bg-gray-100">
    <div class="container mx-auto px-4 py-8">
        <header class="mb-8">
            <h1 class="text-3xl font-bold text-gray-800">LMS Project Dashboard</h1>
            <p class="text-gray-600">Generated on: {}</p>
        </header>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Overall Progress Card -->
            <div class="bg-white p-6 rounded-lg shadow-md">
                <h2 class="text-xl font-semibold mb-4">Overall Project Progress</h2>
                <div class="text-5xl font-bold text-blue-600 mb-4">{:.1}%</div>
                <div class="w-full bg-gray-200 rounded-full h-4">
                    <div class="bg-blue-600 h-4 rounded-full" style="width: {:.1}%"></div>
                </div>
            </div>
            
            <!-- Technical Debt Card -->
            <div class="bg-white p-6 rounded-lg shadow-md">
                <h2 class="text-xl font-semibold mb-4">Technical Debt</h2>
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <p class="text-gray-600">Total Issues</p>
                        <p class="text-2xl font-bold">{}</p>
                    </div>
                    <div>
                        <p class="text-gray-600">Critical</p>
                        <p class="text-2xl font-bold text-red-600">{}</p>
                    </div>
                    <div>
                        <p class="text-gray-600">High</p>
                        <p class="text-2xl font-bold text-orange-500">{}</p>
                    </div>
                    <div>
                        <p class="text-gray-600">Medium</p>
                        <p class="text-2xl font-bold text-yellow-500">{}</p>
                    </div>
                </div>
            </div>
            
            <!-- Recent Changes Card -->
            <div class="bg-white p-6 rounded-lg shadow-md">
                <h2 class="text-xl font-semibold mb-4">Recent Changes</h2>
                <ul class="list-disc pl-5">
                    {}
                </ul>
            </div>
            
            <!-- Next Steps Card -->
            <div class="bg-white p-6 rounded-lg shadow-md">
                <h2 class="text-xl font-semibold mb-4">Next Steps</h2>
                <ul class="list-disc pl-5">
                    {}
                </ul>
            </div>
        </div>
        
        <div class="mt-8 bg-white p-6 rounded-lg shadow-md">
            <h2 class="text-xl font-semibold mb-4">Implementation Progress</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div>
                    <h3 class="text-lg font-medium mb-2">Models</h3>
                    <div class="text-3xl font-bold text-green-600">{:.1}%</div>
                    <p class="text-gray-600">{}/{} implemented</p>
                </div>
                <div>
                    <h3 class="text-lg font-medium mb-2">API Endpoints</h3>
                    <div class="text-3xl font-bold text-green-600">{:.1}%</div>
                    <p class="text-gray-600">{}/{} implemented</p>
                </div>
                <div>
                    <h3 class="text-lg font-medium mb-2">UI Components</h3>
                    <div class="text-3xl font-bold text-green-600">{:.1}%</div>
                    <p class="text-gray-600">{}/{} implemented</p>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        // Add any JavaScript for interactive charts here
    </script>
</body>
</html>
"#,
        // Header date
        Local::now().format("%Y-%m-%d"),
        
        // Overall progress percentage (value and progress bar)
        result.overall_progress,
        result.overall_progress,
        
        // Technical debt summary
        result.tech_debt_metrics.total_issues,
        result.tech_debt_metrics.critical_issues,
        result.tech_debt_metrics.high_issues,
        result.tech_debt_metrics.medium_issues,
        
        // Recent changes list
        result.recent_changes.iter()
            .map(|change| format!("<li>{}</li>", change))
            .collect::<Vec<_>>()
            .join("\n                    "),
        
        // Next steps list
        result.next_steps.iter()
            .map(|step| format!("<li>{}</li>", step))
            .collect::<Vec<_>>()
            .join("\n                    "),
        
        // Models progress
        result.models.implementation_percentage,
        result.models.implemented,
        result.models.total,
        
        // API endpoints progress
        result.api_endpoints.implementation_percentage,
        result.api_endpoints.implemented,
        result.api_endpoints.total,
        
        // UI components progress
        result.ui_components.implementation_percentage,
        result.ui_components.implemented,
        result.ui_components.total
    );
    
    // Write to file
    fs::write(&dashboard_path, content)
        .map_err(|e| format!("Failed to write dashboard: {}", e))?;
    
    println!("Dashboard generated at: {:?}", dashboard_path);
    
    Ok(())
}
