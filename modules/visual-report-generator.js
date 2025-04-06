/**
 * Generate visual reports for analysis
 */
const fs = require('fs');
const path = require('path');

class VisualReportGenerator {
  constructor(metrics, baseDir) {
    this.metrics = metrics;
    this.baseDir = baseDir;
    this.reportsDir = path.join(baseDir, 'docs');
  }
  
  /**
   * Generate a dashboard with charts
   */
  async generateDashboard() {
    if (!fs.existsSync(this.reportsDir)) {
      fs.mkdirSync(this.reportsDir, { recursive: true });
    }
    
    // Create HTML dashboard
    const dashboardPath = path.join(this.reportsDir, 'dashboard.html');
    
    const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Project Analysis Dashboard</title>
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
  <style>
    body { font-family: Arial, sans-serif; margin: 0; padding: 20px; color: #333; }
    .header { background-color: #f5f5f5; padding: 20px; border-radius: 5px; margin-bottom: 20px; }
    .charts-container { display: flex; flex-wrap: wrap; gap: 20px; }
    .chart-wrapper { flex: 1 1 calc(50% - 20px); min-width: 300px; background: #fff; border: 1px solid #ddd; border-radius: 5px; padding: 15px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
    .summary { background-color: #e8f5e9; padding: 15px; border-radius: 5px; margin: 20px 0; }
    h1, h2, h3 { margin-top: 0; color: #424242; }
    .metrics-table { width: 100%; border-collapse: collapse; margin: 20px 0; }
    .metrics-table th, .metrics-table td { padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }
    .metrics-table th { background-color: #f5f5f5; }
    .progress-bar { height: 20px; background-color: #e0e0e0; border-radius: 10px; overflow: hidden; }
    .progress-fill { height: 100%; background-color: #4caf50; }
  </style>
</head>
<body>
  <div class="header">
    <h1>Project Analysis Dashboard</h1>
    <p>Last updated: ${new Date().toISOString().split('T')[0]}</p>
    <p>Project Status: ${this.metrics.overallPhase}</p>
  </div>
  
  <div class="summary">
    <h2>Project Overview</h2>
    <table class="metrics-table">
      <tr>
        <th>Metric</th>
        <th>Value</th>
        <th>Progress</th>
      </tr>
      <tr>
        <td>Models</td>
        <td>${this.metrics.models.implemented}/${this.metrics.models.total}</td>
        <td>
          <div class="progress-bar">
            <div class="progress-fill" style="width: ${this.getPercentage(this.metrics.models.implemented, this.metrics.models.total)}%"></div>
          </div>
        </td>
      </tr>
      <tr>
        <td>API Endpoints</td>
        <td>${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total}</td>
        <td>
          <div class="progress-bar">
            <div class="progress-fill" style="width: ${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}%"></div>
          </div>
        </td>
      </tr>
      <tr>
        <td>UI Components</td>
        <td>${this.metrics.uiComponents.implemented}/${this.metrics.uiComponents.total}</td>
        <td>
          <div class="progress-bar">
            <div class="progress-fill" style="width: ${this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}%"></div>
          </div>
        </td>
      </tr>
      <tr>
        <td>Test Coverage</td>
        <td>${this.metrics.tests.coverage}%</td>
        <td>
          <div class="progress-bar">
            <div class="progress-fill" style="width: ${this.metrics.tests.coverage}%"></div>
          </div>
        </td>
      </tr>
    </table>
  </div>
  
  <div class="charts-container">
    <div class="chart-wrapper">
      <h3>Implementation Progress</h3>
      <canvas id="progressChart"></canvas>
    </div>
    
    <div class="chart-wrapper">
      <h3>Code Quality</h3>
      <canvas id="qualityChart"></canvas>
    </div>
    
    <div class="chart-wrapper">
      <h3>SOLID Violations</h3>
      <canvas id="solidChart"></canvas>
    </div>
    
    <div class="chart-wrapper">
      <h3>Time Estimates</h3>
      <canvas id="timelineChart"></canvas>
    </div>
  </div>
  
  <script>
    // Implementation Progress Chart
    const progressCtx = document.getElementById('progressChart').getContext('2d');
    new Chart(progressCtx, {
      type: 'bar',
      data: {
        labels: ['Models', 'API Endpoints', 'UI Components'],
        datasets: [{
          label: 'Implemented',
          data: [
            ${this.metrics.models.implemented},
            ${this.metrics.apiEndpoints.implemented},
            ${this.metrics.uiComponents.implemented}
          ],
          backgroundColor: '#4caf50'
        }, {
          label: 'Remaining',
          data: [
            ${this.metrics.models.total - this.metrics.models.implemented},
            ${this.metrics.apiEndpoints.total - this.metrics.apiEndpoints.implemented},
            ${this.metrics.uiComponents.total - this.metrics.uiComponents.implemented}
          ],
          backgroundColor: '#e0e0e0'
        }]
      },
      options: {
        scales: {
          y: {
            beginAtZero: true,
            stacked: true
          },
          x: {
            stacked: true
          }
        }
      }
    });
    
    // Code Quality Chart
    const qualityCtx = document.getElementById('qualityChart').getContext('2d');
    new Chart(qualityCtx, {
      type: 'radar',
      data: {
        labels: ['Test Coverage', 'Low Complexity', 'Low Tech Debt', 'Pattern Usage', 'SOLID Adherence'],
        datasets: [{
          label: 'Current State',
          data: [
            ${this.metrics.tests.coverage},
            ${Math.max(0, 100 - this.metrics.codeQuality.complexity.average * 10)},
            ${Math.max(0, 100 - this.metrics.codeQuality.techDebt.score)},
            ${this.getPatternScore()},
            ${this.getSolidScore()}
          ],
          fill: true,
          backgroundColor: 'rgba(54, 162, 235, 0.2)',
          borderColor: 'rgb(54, 162, 235)',
          pointBackgroundColor: 'rgb(54, 162, 235)',
          pointBorderColor: '#fff',
          pointHoverBackgroundColor: '#fff',
          pointHoverBorderColor: 'rgb(54, 162, 235)'
        }, {
          label: 'Target',
          data: [80, 70, 80, 80, 80],
          fill: true,
          backgroundColor: 'rgba(255, 99, 132, 0.2)',
          borderColor: 'rgb(255, 99, 132)',
          pointBackgroundColor: 'rgb(255, 99, 132)',
          pointBorderColor: '#fff',
          pointHoverBackgroundColor: '#fff',
          pointHoverBorderColor: 'rgb(255, 99, 132)'
        }]
      },
      options: {
        scales: {
          r: {
            angleLines: {
              display: true
            },
            suggestedMin: 0,
            suggestedMax: 100
          }
        }
      }
    });
    
    // SOLID Violations Chart
    const solidCtx = document.getElementById('solidChart').getContext('2d');
    new Chart(solidCtx, {
      type: 'doughnut',
      data: {
        labels: ['SRP', 'OCP', 'LSP', 'ISP', 'DIP'],
        datasets: [{
          label: 'Violations',
          data: [
            ${this.metrics.codeQuality.solidViolations.srp.length},
            ${this.metrics.codeQuality.solidViolations.ocp.length},
            ${this.metrics.codeQuality.solidViolations.lsp.length},
            ${this.metrics.codeQuality.solidViolations.isp.length},
            ${this.metrics.codeQuality.solidViolations.dip.length}
          ],
          backgroundColor: [
            '#f44336',
            '#ff9800',
            '#ffeb3b',
            '#4caf50',
            '#2196f3'
          ]
        }]
      }
    });
    
    // Timeline Chart
    const timelineCtx = document.getElementById('timelineChart').getContext('2d');
    new Chart(timelineCtx, {
      type: 'bar',
      data: {
        labels: ['Models', 'API Endpoints', 'UI Components', 'Project Completion'],
        datasets: [{
          label: 'Estimated Weeks Remaining',
          data: [
            ${this.getWeeksRemaining('models')},
            ${this.getWeeksRemaining('apiEndpoints')},
            ${this.getWeeksRemaining('uiComponents')},
            ${this.getWeeksRemaining('project')}
          ],
          backgroundColor: '#673ab7'
        }]
      },
      options: {
        scales: {
          y: {
            beginAtZero: true
          }
        }
      }
    });
  </script>
</body>
</html>`;
    
    fs.writeFileSync(dashboardPath, html);
    console.log(`Visual dashboard generated at ${dashboardPath}`);
    return dashboardPath;
  }
  
  /**
   * Calculate pattern usage score
   */
  getPatternScore() {
    // Calculate a score based on pattern implementations and violations
    const implementations = 
      this.metrics.codeQuality.designPatterns.polymorphism.implementations.length +
      this.metrics.codeQuality.designPatterns.dependencyInjection.implementations.length +
      this.metrics.codeQuality.designPatterns.ioc.implementations.length;
    
    const violations = 
      this.metrics.codeQuality.designPatterns.polymorphism.violations.length +
      this.metrics.codeQuality.designPatterns.dependencyInjection.violations.length +
      this.metrics.codeQuality.designPatterns.ioc.violations.length;
    
    if (implementations + violations === 0) return 50; // No data
    return Math.min(100, Math.max(0, 100 * implementations / (implementations + violations)));
  }
  
  /**
   * Calculate SOLID adherence score
   */
  getSolidScore() {
    // Count total violations
    const violations = 
      this.metrics.codeQuality.solidViolations.srp.length +
      this.metrics.codeQuality.solidViolations.ocp.length +
      this.metrics.codeQuality.solidViolations.lsp.length +
      this.metrics.codeQuality.solidViolations.isp.length +
      this.metrics.codeQuality.solidViolations.dip.length;
    
    // Estimate total number of files
    const totalFiles = this.metrics.models.details.length + 
                      this.metrics.apiEndpoints.details.length +
                      this.metrics.uiComponents.details.length;
    
    if (totalFiles === 0) return 50; // No data
    
    // Calculate violations per file ratio and convert to score
    const violationsPerFile = violations / totalFiles;
    return Math.min(100, Math.max(0, 100 - (violationsPerFile * 50)));
  }
  
  /**
   * Get weeks remaining for a particular metric
   */
  getWeeksRemaining(metric) {
    if (!this.metrics.predictions.estimates[metric]) return 0;
    const estimatedDate = new Date(this.metrics.predictions.estimates[metric].date);
    const now = new Date();
    const diffTime = Math.abs(estimatedDate - now);
    const diffWeeks = Math.ceil(diffTime / (1000 * 60 * 60 * 24 * 7));
    return diffWeeks;
  }
  
  /**
   * Calculate percentage safely
   */
  getPercentage(value, total) {
    if (total === 0) return 0;
    return Math.round((value / total) * 100);
  }
}

module.exports = VisualReportGenerator;