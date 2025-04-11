use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use chrono::Local;
use log::info;

/// Generate visual reports for analysis
pub struct VisualReportGenerator<M> {
    metrics: M,
    base_dir: PathBuf,
    reports_dir: PathBuf,
}

impl<M> VisualReportGenerator<M> {
    /// Create a new visual report generator
    pub fn new(metrics: M, base_dir: PathBuf) -> Self {
        let reports_dir = base_dir.join("docs");
        
        Self {
            metrics,
            base_dir,
            reports_dir,
        }
    }
    
    /// Generate a dashboard with charts
    pub async fn generate_dashboard(&self) -> Result<PathBuf>
    where
        M: AsRef<DashboardMetrics>,
    {
        fs::create_dir_all(&self.reports_dir)
            .context(format!("Failed to create reports directory: {:?}", self.reports_dir))?;
        
        // Create HTML dashboard
        let dashboard_path = self.reports_dir.join("dashboard.html");
        
        let metrics = self.metrics.as_ref();
        let today = Local::now().format("%Y-%m-%d").to_string();
        
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Project Analysis Dashboard</title>
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
  <style>
    body {{ font-family: Arial, sans-serif; margin: 0; padding: 20px; color: #333; }}
    .header {{ background-color: #f5f5f5; padding: 20px; border-radius: 5px; margin-bottom: 20px; }}
    .charts-container {{ display: flex; flex-wrap: wrap; gap: 20px; }}
    .chart-wrapper {{ flex: 1 1 calc(50% - 20px); min-width: 300px; background: #fff; border: 1px solid #ddd; border-radius: 5px; padding: 15px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
    .summary {{ background-color: #e8f5e9; padding: 15px; border-radius: 5px; margin: 20px 0; }}
    h1, h2, h3 {{ margin-top: 0; color: #424242; }}
    .metrics-table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
    .metrics-table th, .metrics-table td {{ padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }}
    .metrics-table th {{ background-color: #f5f5f5; }}
    .progress-bar {{ height: 20px; background-color: #e0e0e0; border-radius: 10px; overflow: hidden; }}
    .progress-fill {{ height: 100%; background-color: #4caf50; }}
  </style>
</head>
<body>
  <div class="header">
    <h1>Project Analysis Dashboard</h1>
    <p>Last updated: {}</p>
    <p>Project Status: {}</p>
  </div>
  
  <div class="summary">
    <h2>Project Summary</h2>
    <p><strong>Total Models:</strong> {}</p>
    <p><strong>Total API Endpoints:</strong> {}</p>
    <p><strong>Test Coverage:</strong> {}%</p>
    <p><strong>Code Quality Score:</strong> {}/10</p>
  </div>
  
  <div class="charts-container">
    <div class="chart-wrapper">
      <h3>Implementation Progress</h3>
      <canvas id="progressChart"></canvas>
    </div>
    <div class="chart-wrapper">
      <h3>Code Distribution</h3>
      <canvas id="distributionChart"></canvas>
    </div>
    <div class="chart-wrapper">
      <h3>Test Coverage</h3>
      <canvas id="coverageChart"></canvas>
    </div>
    <div class="chart-wrapper">
      <h3>Performance Metrics</h3>
      <canvas id="performanceChart"></canvas>
    </div>
  </div>
  
  <div class="metrics-table">
    <h2>Detailed Metrics</h2>
    <table class="metrics-table">
      <thead>
        <tr>
          <th>Metric</th>
          <th>Value</th>
          <th>Progress</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Models Implemented</td>
          <td>{} / {}</td>
          <td>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {}%"></div>
            </div>
          </td>
        </tr>
        <tr>
          <td>API Endpoints Implemented</td>
          <td>{} / {}</td>
          <td>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {}%"></div>
            </div>
          </td>
        </tr>
        <tr>
          <td>UI Components Implemented</td>
          <td>{} / {}</td>
          <td>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {}%"></div>
            </div>
          </td>
        </tr>
        <tr>
          <td>Tests Passing</td>
          <td>{} / {}</td>
          <td>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {}%"></div>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
  
  <script>
    // Implementation Progress Chart
    const progressCtx = document.getElementById('progressChart').getContext('2d');
    const progressChart = new Chart(progressCtx, {{
      type: 'bar',
      data: {{
        labels: ['Models', 'API Endpoints', 'UI Components', 'Tests'],
        datasets: [{{
          label: 'Implemented',
          data: [{}, {}, {}, {}],
          backgroundColor: 'rgba(76, 175, 80, 0.6)',
          borderColor: 'rgba(76, 175, 80, 1)',
          borderWidth: 1
        }},
        {{
          label: 'Total',
          data: [{}, {}, {}, {}],
          backgroundColor: 'rgba(224, 224, 224, 0.6)',
          borderColor: 'rgba(224, 224, 224, 1)',
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
    
    // Code Distribution Chart
    const distributionCtx = document.getElementById('distributionChart').getContext('2d');
    const distributionChart = new Chart(distributionCtx, {{
      type: 'pie',
      data: {{
        labels: ['Rust', 'JavaScript', 'TypeScript', 'HTML/CSS', 'Other'],
        datasets: [{{
          label: 'Code Distribution',
          data: [{}, {}, {}, {}, {}],
          backgroundColor: [
            'rgba(76, 175, 80, 0.6)',
            'rgba(33, 150, 243, 0.6)',
            'rgba(156, 39, 176, 0.6)',
            'rgba(255, 152, 0, 0.6)',
            'rgba(158, 158, 158, 0.6)'
          ],
          borderColor: [
            'rgba(76, 175, 80, 1)',
            'rgba(33, 150, 243, 1)',
            'rgba(156, 39, 176, 1)',
            'rgba(255, 152, 0, 1)',
            'rgba(158, 158, 158, 1)'
          ],
          borderWidth: 1
        }}]
      }}
    }});
    
    // Test Coverage Chart
    const coverageCtx = document.getElementById('coverageChart').getContext('2d');
    const coverageChart = new Chart(coverageCtx, {{
      type: 'doughnut',
      data: {{
        labels: ['Covered', 'Not Covered'],
        datasets: [{{
          label: 'Test Coverage',
          data: [{}, {}],
          backgroundColor: [
            'rgba(76, 175, 80, 0.6)',
            'rgba(244, 67, 54, 0.6)'
          ],
          borderColor: [
            'rgba(76, 175, 80, 1)',
            'rgba(244, 67, 54, 1)'
          ],
          borderWidth: 1
        }}]
      }}
    }});
    
    // Performance Metrics Chart
    const performanceCtx = document.getElementById('performanceChart').getContext('2d');
    const performanceChart = new Chart(performanceCtx, {{
      type: 'radar',
      data: {{
        labels: ['API Response Time', 'Build Time', 'Memory Usage', 'Startup Time', 'Query Performance'],
        datasets: [{{
          label: 'Current',
          data: [{}, {}, {}, {}, {}],
          backgroundColor: 'rgba(76, 175, 80, 0.2)',
          borderColor: 'rgba(76, 175, 80, 1)',
          borderWidth: 1,
          pointBackgroundColor: 'rgba(76, 175, 80, 1)'
        }},
        {{
          label: 'Target',
          data: [100, 100, 100, 100, 100],
          backgroundColor: 'rgba(224, 224, 224, 0.2)',
          borderColor: 'rgba(158, 158, 158, 1)',
          borderWidth: 1,
          pointBackgroundColor: 'rgba(158, 158, 158, 1)'
        }}]
      }},
      options: {{
        scales: {{
          r: {{
            beginAtZero: true,
            max: 100
          }}
        }}
      }}
    }});
  </script>
</body>
</html>"#,
            today,
            metrics.overall_phase,
            metrics.total_models,
            metrics.total_api_endpoints,
            metrics.test_coverage,
            metrics.code_quality_score,
            metrics.models_implemented, metrics.total_models,
            self.calculate_percentage(metrics.models_implemented, metrics.total_models),
            metrics.api_endpoints_implemented, metrics.total_api_endpoints,
            self.calculate_percentage(metrics.api_endpoints_implemented, metrics.total_api_endpoints),
            metrics.ui_components_implemented, metrics.total_ui_components,
            self.calculate_percentage(metrics.ui_components_implemented, metrics.total_ui_components),
            metrics.tests_passing, metrics.total_tests,
            self.calculate_percentage(metrics.tests_passing, metrics.total_tests),
            // Implementation progress chart data
            metrics.models_implemented, metrics.api_endpoints_implemented, metrics.ui_components_implemented, metrics.tests_passing,
            metrics.total_models, metrics.total_api_endpoints, metrics.total_ui_components, metrics.total_tests,
            // Code distribution chart data
            metrics.code_distribution.rust, metrics.code_distribution.javascript, 
            metrics.code_distribution.typescript, metrics.code_distribution.html_css, 
            metrics.code_distribution.other,
            // Test coverage chart data
            metrics.test_coverage, 100 - metrics.test_coverage,
            // Performance metrics chart data
            metrics.performance_metrics.api_response_time,
            metrics.performance_metrics.build_time,
            metrics.performance_metrics.memory_usage,
            metrics.performance_metrics.startup_time,
            metrics.performance_metrics.query_performance
        );
        
        fs::write(&dashboard_path, html)
            .context(format!("Failed to write dashboard to file: {:?}", dashboard_path))?;
            
        info!("Dashboard generated at: {:?}", dashboard_path);
        
        Ok(dashboard_path)
    }
    
    /// Generate a progress report for stakeholders
    pub async fn generate_stakeholder_report(&self) -> Result<PathBuf>
    where
        M: AsRef<DashboardMetrics>,
    {
        fs::create_dir_all(&self.reports_dir)
            .context(format!("Failed to create reports directory: {:?}", self.reports_dir))?;
        
        let report_path = self.reports_dir.join("stakeholder_report.md");
        let metrics = self.metrics.as_ref();
        let today = Local::now().format("%Y-%m-%d").to_string();
        
        let content = format!(
            r#"# Project Progress Report

_Generated on: {}_

## Executive Summary

**Project Status**: {}

**Overall Completion**: {}%

**Key Achievements**:
- Implemented {}/{} planned models ({:.1}%)
- Implemented {}/{} API endpoints ({:.1}%)
- Current test coverage: {}%

## Implementation Progress

| Component | Progress | Status |
|-----------|----------|--------|
| Models | {}/{} | {:.1}% |
| API Endpoints | {}/{} | {:.1}% |
| UI Components | {}/{} | {:.1}% |
| Tests | {}/{} | {:.1}% |

## Quality Metrics

- **Code Quality Score**: {}/10
- **Test Coverage**: {}%
- **API Performance**: {}/100

## Next Steps

1. Continue implementation of remaining models
2. Complete API endpoints for core functionality
3. Improve test coverage to target of 80%
4. Optimize performance for critical paths

## Timeline

The project is currently **{}** according to the established timeline.

## Risks and Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Technical debt accumulation | High | Medium | Regular refactoring sessions |
| API performance issues | Medium | Low | Performance testing before releases |
| Integration challenges | Medium | Medium | Early integration testing |

"#,
            today,
            metrics.overall_phase,
            self.calculate_overall_completion(metrics),
            metrics.models_implemented, metrics.total_models,
            self.calculate_percentage(metrics.models_implemented, metrics.total_models),
            metrics.api_endpoints_implemented, metrics.total_api_endpoints,
            self.calculate_percentage(metrics.api_endpoints_implemented, metrics.total_api_endpoints),
            metrics.test_coverage,
            metrics.models_implemented, metrics.total_models,
            self.calculate_percentage(metrics.models_implemented, metrics.total_models),
            metrics.api_endpoints_implemented, metrics.total_api_endpoints,
            self.calculate_percentage(metrics.api_endpoints_implemented, metrics.total_api_endpoints),
            metrics.ui_components_implemented, metrics.total_ui_components,
            self.calculate_percentage(metrics.ui_components_implemented, metrics.total_ui_components),
            metrics.tests_passing, metrics.total_tests,
            self.calculate_percentage(metrics.tests_passing, metrics.total_tests),
            metrics.code_quality_score,
            metrics.test_coverage,
            metrics.performance_metrics.api_response_time,
            self.get_timeline_status(metrics)
        );
        
        fs::write(&report_path, content)
            .context(format!("Failed to write stakeholder report to file: {:?}", report_path))?;
            
        info!("Stakeholder report generated at: {:?}", report_path);
        
        Ok(report_path)
    }
    
    /// Calculate percentage
    fn calculate_percentage(&self, value: usize, total: usize) -> usize {
        if total == 0 {
            return 0;
        }
        (value as f64 / total as f64 * 100.0).round() as usize
    }
    
    /// Calculate overall completion percentage
    fn calculate_overall_completion(&self, metrics: &DashboardMetrics) -> usize {
        let components = [
            (metrics.models_implemented, metrics.total_models),
            (metrics.api_endpoints_implemented, metrics.total_api_endpoints),
            (metrics.ui_components_implemented, metrics.total_ui_components),
            (metrics.tests_passing, metrics.total_tests),
        ];
        
        let mut total_implemented = 0;
        let mut total_planned = 0;
        
        for (implemented, planned) in components {
            total_implemented += implemented;
            total_planned += planned;
        }
        
        self.calculate_percentage(total_implemented, total_planned)
    }
    
    /// Get project timeline status
    fn get_timeline_status(&self, metrics: &DashboardMetrics) -> &'static str {
        let completion = self.calculate_overall_completion(metrics);
        
        match metrics.timeline_status.as_str() {
            "ahead" => "ahead of schedule",
            "behind" => "behind schedule",
            _ => "on schedule",
        }
    }
}

/// Dashboard metrics for project visualization
#[derive(Debug, Clone)]
pub struct DashboardMetrics {
    pub overall_phase: String,
    pub total_models: usize,
    pub models_implemented: usize,
    pub total_api_endpoints: usize,
    pub api_endpoints_implemented: usize,
    pub total_ui_components: usize,
    pub ui_components_implemented: usize,
    pub total_tests: usize,
    pub tests_passing: usize,
    pub test_coverage: usize,
    pub code_quality_score: usize,
    pub code_distribution: CodeDistribution,
    pub performance_metrics: PerformanceMetrics,
    pub timeline_status: String,
}

/// Code distribution metrics
#[derive(Debug, Clone)]
pub struct CodeDistribution {
    pub rust: usize,
    pub javascript: usize,
    pub typescript: usize,
    pub html_css: usize,
    pub other: usize,
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub api_response_time: usize,
    pub build_time: usize,
    pub memory_usage: usize,
    pub startup_time: usize,
    pub query_performance: usize,
}
