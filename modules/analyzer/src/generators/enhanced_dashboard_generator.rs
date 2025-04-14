use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;

/// Generate enhanced dashboard
pub fn generate_enhanced_dashboard(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating enhanced dashboard...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Create the dashboard path
    let dashboard_path = docs_dir.join("enhanced_dashboard.html");
    
    // Generate the content
    let content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>LMS Project Enhanced Dashboard</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ padding-top: 20px; }}
        .card {{ margin-bottom: 20px; }}
        .progress {{ height: 25px; }}
        .progress-bar {{ font-size: 14px; font-weight: bold; }}
    </style>
</head>
<body>
    <div class="container">
        <header class="mb-4">
            <div class="row">
                <div class="col-md-8">
                    <h1>LMS Project Dashboard</h1>
                    <p class="text-muted">Generated on: {}</p>
                </div>
                <div class="col-md-4 text-end">
                    <div class="btn-group">
                        <a href="central_reference_hub.md" class="btn btn-outline-primary">Central Hub</a>
                        <a href="comprehensive_report.md" class="btn btn-outline-primary">Comprehensive Report</a>
                    </div>
                </div>
            </div>
        </header>
        
        <div class="row">
            <!-- Overall Progress Card -->
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header bg-primary text-white">
                        <h5 class="card-title mb-0">Overall Project Progress</h5>
                    </div>
                    <div class="card-body">
                        <h2 class="display-4 text-center">{:.1}%</h2>
                        <div class="progress mt-3">
                            <div class="progress-bar bg-primary" role="progressbar" style="width: {:.1}%" aria-valuenow="{:.1}" aria-valuemin="0" aria-valuemax="100">{:.1}%</div>
                        </div>
                    </div>
                </div>
            </div>
            
            <!-- Project Statistics Card -->
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header bg-info text-white">
                        <h5 class="card-title mb-0">Project Statistics</h5>
                    </div>
                    <div class="card-body">
                        <div class="row">
                            <div class="col-6">
                                <p class="fw-bold">Total Files</p>
                                <p class="h4">{}</p>
                            </div>
                            <div class="col-6">
                                <p class="fw-bold">Lines of Code</p>
                                <p class="h4">{}</p>
                            </div>
                            <div class="col-6">
                                <p class="fw-bold">Rust Files</p>
                                <p class="h4">{}</p>
                            </div>
                            <div class="col-6">
                                <p class="fw-bold">Haskell Files</p>
                                <p class="h4">{}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="row">
            <!-- Component Progress Card -->
            <div class="col-md-12">
                <div class="card">
                    <div class="card-header bg-success text-white">
                        <h5 class="card-title mb-0">Component Progress</h5>
                    </div>
                    <div class="card-body">
                        <div class="row">
                            <div class="col-md-4">
                                <h5>Models</h5>
                                <div class="progress mb-3">
                                    <div class="progress-bar bg-success" role="progressbar" style="width: {:.1}%" aria-valuenow="{:.1}" aria-valuemin="0" aria-valuemax="100">{:.1}%</div>
                                </div>
                                <p class="text-muted">{}/{} implemented</p>
                            </div>
                            <div class="col-md-4">
                                <h5>API Endpoints</h5>
                                <div class="progress mb-3">
                                    <div class="progress-bar bg-success" role="progressbar" style="width: {:.1}%" aria-valuenow="{:.1}" aria-valuemin="0" aria-valuemax="100">{:.1}%</div>
                                </div>
                                <p class="text-muted">{}/{} implemented</p>
                            </div>
                            <div class="col-md-4">
                                <h5>UI Components</h5>
                                <div class="progress mb-3">
                                    <div class="progress-bar bg-success" role="progressbar" style="width: {:.1}%" aria-valuenow="{:.1}" aria-valuemin="0" aria-valuemax="100">{:.1}%</div>
                                </div>
                                <p class="text-muted">{}/{} implemented</p>
                            </div>
                        </div>
                        <div class="mt-4">
                            <canvas id="componentChart"></canvas>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="row">
            <!-- Technical Debt Card -->
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header bg-danger text-white">
                        <h5 class="card-title mb-0">Technical Debt</h5>
                    </div>
                    <div class="card-body">
                        <div class="row text-center mb-3">
                            <div class="col-12">
                                <h3>Total Issues: {}</h3>
                            </div>
                        </div>
                        <div class="row text-center">
                            <div class="col-3">
                                <div class="alert alert-danger">
                                    <h5>Critical</h5>
                                    <h3>{}</h3>
                                </div>
                            </div>
                            <div class="col-3">
                                <div class="alert alert-warning">
                                    <h5>High</h5>
                                    <h3>{}</h3>
                                </div>
                            </div>
                            <div class="col-3">
                                <div class="alert alert-info">
                                    <h5>Medium</h5>
                                    <h3>{}</h3>
                                </div>
                            </div>
                            <div class="col-3">
                                <div class="alert alert-success">
                                    <h5>Low</h5>
                                    <h3>{}</h3>
                                </div>
                            </div>
                        </div>
                        <div class="mt-3">
                            <canvas id="techDebtChart"></canvas>
                        </div>
                    </div>
                </div>
            </div>
            
            <!-- Feature Areas Card -->
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header bg-primary text-white">
                        <h5 class="card-title mb-0">Feature Areas</h5>
                    </div>
                    <div class="card-body">
                        <div class="table-responsive">
                            <table class="table table-striped">
                                <thead>
                                    <tr>
                                        <th>Feature Area</th>
                                        <th>Progress</th>
                                        <th>Implemented / Total</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {}
                                </tbody>
                            </table>
                        </div>
                        <div class="mt-3">
                            <canvas id="featureAreasChart"></canvas>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="row">
            <!-- Recent Changes Card -->
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header bg-info text-white">
                        <h5 class="card-title mb-0">Recent Changes</h5>
                    </div>
                    <div class="card-body">
                        <ul class="list-group">
                            {}
                        </ul>
                    </div>
                </div>
            </div>
            
            <!-- Next Steps Card -->
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header bg-warning text-dark">
                        <h5 class="card-title mb-0">Next Steps</h5>
                    </div>
                    <div class="card-body">
                        <ul class="list-group">
                            {}
                        </ul>
                    </div>
                </div>
            </div>
        </div>
        
        <footer class="mt-4 mb-4 text-center text-muted">
            <p>LMS Project Dashboard | Generated by Unified Analyzer</p>
        </footer>
    </div>
    
    <script>
        // Component Progress Chart
        const componentCtx = document.getElementById('componentChart').getContext('2d');
        new Chart(componentCtx, {{
            type: 'bar',
            data: {{
                labels: ['Models', 'API Endpoints', 'UI Components'],
                datasets: [
                    {{
                        label: 'Implemented',
                        data: [{}, {}, {}],
                        backgroundColor: 'rgba(40, 167, 69, 0.7)',
                    }},
                    {{
                        label: 'Remaining',
                        data: [{}, {}, {}],
                        backgroundColor: 'rgba(108, 117, 125, 0.7)',
                    }}
                ]
            }},
            options: {{
                responsive: true,
                scales: {{
                    x: {{
                        stacked: true,
                    }},
                    y: {{
                        stacked: true,
                        beginAtZero: true
                    }}
                }}
            }}
        }});
        
        // Technical Debt Chart
        const techDebtCtx = document.getElementById('techDebtChart').getContext('2d');
        new Chart(techDebtCtx, {{
            type: 'pie',
            data: {{
                labels: ['Critical', 'High', 'Medium', 'Low'],
                datasets: [{{
                    data: [{}, {}, {}, {}],
                    backgroundColor: [
                        'rgba(220, 53, 69, 0.7)',
                        'rgba(255, 193, 7, 0.7)',
                        'rgba(23, 162, 184, 0.7)',
                        'rgba(40, 167, 69, 0.7)'
                    ],
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    legend: {{
                        position: 'bottom',
                    }}
                }}
            }}
        }});
        
        // Feature Areas Chart
        const featureAreasCtx = document.getElementById('featureAreasChart').getContext('2d');
        new Chart(featureAreasCtx, {{
            type: 'radar',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Implementation Percentage',
                    data: [{}],
                    backgroundColor: 'rgba(13, 110, 253, 0.2)',
                    borderColor: 'rgba(13, 110, 253, 1)',
                    borderWidth: 2,
                    pointBackgroundColor: 'rgba(13, 110, 253, 1)',
                }}]
            }},
            options: {{
                responsive: true,
                scales: {{
                    r: {{
                        beginAtZero: true,
                        max: 100,
                        ticks: {{
                            stepSize: 20
                        }}
                    }}
                }}
            }}
        }});
    </script>
    
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>
"#,
        // Header date
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        
        // Overall progress percentage (value and progress bar)
        result.overall_progress,
        result.overall_progress,
        result.overall_progress,
        
        // Project statistics
        result.summary.total_files,
        result.summary.lines_of_code,
        result.summary.rust_files,
        result.summary.haskell_files,
        
        // Models progress
        result.models.implementation_percentage,
        result.models.implementation_percentage,
        result.models.implemented,
        result.models.total,
        
        // API endpoints progress
        result.api_endpoints.implementation_percentage,
        result.api_endpoints.implementation_percentage,
        result.api_endpoints.implemented,
        result.api_endpoints.total,
        
        // UI components progress
        result.ui_components.implementation_percentage,
        result.ui_components.implementation_percentage,
        result.ui_components.implemented,
        result.ui_components.total,
        
        // Technical debt summary
        result.tech_debt_metrics.total_issues,
        result.tech_debt_metrics.critical_issues,
        result.tech_debt_metrics.high_issues,
        result.tech_debt_metrics.medium_issues,
        result.tech_debt_metrics.low_issues,
        
        // Feature areas table rows
        result.feature_areas.iter()
            .map(|(area, metrics)| format!(
                "<tr><td>{}</td><td><div class=\"progress\"><div class=\"progress-bar\" role=\"progressbar\" style=\"width: {:.1}%\" aria-valuenow=\"{:.1}\" aria-valuemin=\"0\" aria-valuemax=\"100\">{:.1}%</div></div></td><td>{}/{}</td></tr>",
                area,
                metrics.implementation_percentage,
                metrics.implementation_percentage,
                metrics.implementation_percentage,
                metrics.implemented,
                metrics.total
            ))
            .collect::<Vec<_>>()
            .join("\n                                    "),
        
        // Recent changes list
        result.recent_changes.iter()
            .map(|change| format!("<li class=\"list-group-item\">{}</li>", change))
            .collect::<Vec<_>>()
            .join("\n                            "),
        
        // Next steps list
        result.next_steps.iter()
            .map(|step| format!("<li class=\"list-group-item\">{}</li>", step))
            .collect::<Vec<_>>()
            .join("\n                            "),
        
        // Component chart data - Implemented
        result.models.implemented,
        result.api_endpoints.implemented,
        result.ui_components.implemented,
        
        // Component chart data - Remaining
        result.models.total - result.models.implemented,
        result.api_endpoints.total - result.api_endpoints.implemented,
        result.ui_components.total - result.ui_components.implemented,
        
        // Technical debt chart data
        result.tech_debt_metrics.critical_issues,
        result.tech_debt_metrics.high_issues,
        result.tech_debt_metrics.medium_issues,
        result.tech_debt_metrics.low_issues,
        
        // Feature areas chart labels
        result.feature_areas.keys()
            .map(|area| format!("'{}'", area))
            .collect::<Vec<_>>()
            .join(", "),
        
        // Feature areas chart data
        result.feature_areas.values()
            .map(|metrics| format!("{:.1}", metrics.implementation_percentage))
            .collect::<Vec<_>>()
            .join(", ")
    );
    
    // Write to file
    fs::write(&dashboard_path, content)
        .map_err(|e| format!("Failed to write enhanced dashboard: {}", e))?;
    
    println!("Enhanced dashboard generated at: {:?}", dashboard_path);
    
    Ok(())
}
