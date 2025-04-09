const fs = require('fs');
const path = require('path');

/**
 * Generate a visual HTML dashboard for the integration status
 */
async function generateVisualDashboard() {
  console.log('Generating visual dashboard...');
  
  // Load configuration
  const configPath = path.join(__dirname, 'config.json');
  if (!fs.existsSync(configPath)) {    console.error('Configuration file not found. Please run "cargo run --bin analyze full" first.');
    return;
  }
  
  const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
  
  // Load conflicts data
  let conflicts = [];
  const conflictsPath = path.join(config.paths.analysis, 'conflicts', 'port_conflicts.md');
  if (fs.existsSync(conflictsPath)) {
    // In a real implementation, you would parse the markdown properly
    // This is a simplified version
    const content = fs.readFileSync(conflictsPath, 'utf8');
    const match = content.match(/Total conflicts detected: (\d+)/);
    const conflictCount = match ? parseInt(match[1]) : 0;
    
    // Create dummy conflicts for visualization
    for (let i = 0; i < conflictCount; i++) {
      conflicts.push({
        type: ['Model Duplication', 'API Path Conflict', 'Naming Inconsistency'][i % 3],
        severity: ['High', 'Medium', 'Low'][i % 3]
      });
    }
  }
  
  // Group conflicts by type
  const conflictsByType = {};
  conflicts.forEach(conflict => {
    if (!conflictsByType[conflict.type]) conflictsByType[conflict.type] = 0;
    conflictsByType[conflict.type]++;
  });
  
  // Calculate overall percentages
  const canvasOverall = calculateOverall(config.port.canvas.completion);
  const discourseOverall = calculateOverall(config.port.discourse.completion);
  const totalOverall = Math.round((canvasOverall + discourseOverall) / 2);
  
  // Create dashboard HTML
  const htmlContent = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Canvas-Discourse Integration Dashboard</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .header {
            background-color: #2c3e50;
            color: white;
            padding: 20px;
            border-radius: 5px;
            margin-bottom: 20px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
        }
        .dashboard-grid {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 20px;
            margin-bottom: 20px;
        }
        .card {
            background: white;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
        }
        .progress-container {
            margin-bottom: 15px;
        }
        .progress-label {
            display: flex;
            justify-content: space-between;
            margin-bottom: 5px;
        }
        .progress-bar {
            height: 20px;
            background-color: #e0e0e0;
            border-radius: 10px;
            overflow: hidden;
        }
        .progress-value {
            height: 100%;
            background-color: #3498db;
            border-radius: 10px;
        }
        .warning { background-color: #e74c3c; }
        .success { background-color: #2ecc71; }
        .medium { background-color: #f39c12; }
        .conflicts {
            margin-top: 10px;
        }
        .conflict-type {
            display: inline-block;
            padding: 5px 10px;
            border-radius: 15px;
            margin: 5px;
            background-color: #e74c3c;
            color: white;
        }
        .charts {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 20px;
        }
        .chart {
            min-height: 300px;
        }
        footer {
            text-align: center;
            margin-top: 40px;
            color: #7f8c8d;
            font-size: 0.9em;
        }
        .next-steps {
            background-color: #f8f9fa;
            border-left: 4px solid #3498db;
            padding: 15px;
        }
        .next-steps h3 {
            margin-top: 0;
        }
        h2 {
            border-bottom: 2px solid #3498db;
            padding-bottom: 5px;
        }
    </style>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body>
    <div class="header">
        <h1>Canvas-Discourse Integration Dashboard</h1>
        <p>Generated on ${new Date().toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}</p>
    </div>
    
    <div class="dashboard-grid">
        <div class="card">
            <h2>Overall Integration Progress</h2>
            <div class="progress-container">
                <div class="progress-label">
                    <span>Total Progress</span>
                    <span>${totalOverall}%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-value ${totalOverall > 80 ? 'success' : totalOverall > 50 ? 'medium' : 'warning'}" style="width: ${totalOverall}%"></div>
                </div>
            </div>
            
            <div class="progress-container">
                <div class="progress-label">
                    <span>Canvas</span>
                    <span>${canvasOverall}%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-value" style="width: ${canvasOverall}%"></div>
                </div>
            </div>
            
            <div class="progress-container">
                <div class="progress-label">
                    <span>Discourse</span>
                    <span>${discourseOverall}%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-value" style="width: ${discourseOverall}%"></div>
                </div>
            </div>
        </div>
        
        <div class="card">
            <h2>Identified Conflicts</h2>
            <div class="conflicts">
                <p>Total conflicts: <strong>${conflicts.length}</strong></p>
                ${Object.entries(conflictsByType).map(([type, count]) => 
                    `<div class="conflict-type">${type}: ${count}</div>`
                ).join('')}
            </div>
            ${conflicts.length > 0 ? `
            <div class="next-steps">
                <h3>Resolution Priority</h3>
                <ol>
                    <li>Address model duplication conflicts first</li>
                    <li>Resolve API path conflicts</li>
                    <li>Standardize authentication approaches</li>
                    <li>Fix naming inconsistencies</li>
                </ol>
            </div>` : ''}
        </div>
    </div>
    
    <div class="card">
        <h2>Component Completion Status</h2>
        <div class="charts">
            <div class="chart">
                <canvas id="canvasChart"></canvas>
            </div>
            <div class="chart">
                <canvas id="discourseChart"></canvas>
            </div>
        </div>
    </div>
    
    <div class="card">
        <h2>Next Integration Milestones</h2>
        <div class="next-steps">
            <h3>Short-term Goals</h3>
            <ul>
                <li><strong>April 30, 2025:</strong> Complete submission workflow integration</li>
                <li><strong>May 15, 2025:</strong> Implement unified notification system</li>
                <li><strong>June 1, 2025:</strong> Add offline file synchronization</li>
            </ul>
            <h3>Documentation Updates</h3>
            <ul>
                <li>Daily automated conflict checks</li>
                <li>Weekly documentation refresh</li>
                <li>Monthly integration status reports</li>
            </ul>
        </div>
    </div>
    
    <footer>
        <p>Generated by the Canvas-Discourse LMS Integration Documentation System</p>        <p>Run 'cargo run --bin analyze full' to update this dashboard</p>
    </footer>
    
    <script>
        // Canvas Chart
        const canvasCtx = document.getElementById('canvasChart').getContext('2d');
        new Chart(canvasCtx, {
            type: 'radar',
            data: {
                labels: ['Models', 'Controllers', 'Services', 'UI', 'Tests'],
                datasets: [{
                    label: 'Canvas Completion',
                    data: [
                        ${config.port.canvas.completion.models},
                        ${config.port.canvas.completion.controllers},
                        ${config.port.canvas.completion.services},
                        ${config.port.canvas.completion.ui},
                        ${config.port.canvas.completion.tests}
                    ],
                    fill: true,
                    backgroundColor: 'rgba(52, 152, 219, 0.2)',
                    borderColor: 'rgb(52, 152, 219)',
                    pointBackgroundColor: 'rgb(52, 152, 219)',
                    pointBorderColor: '#fff',
                    pointHoverBackgroundColor: '#fff',
                    pointHoverBorderColor: 'rgb(52, 152, 219)'
                }]
            },
            options: {
                elements: {
                    line: { borderWidth: 3 }
                },
                scales: {
                    r: {
                        angleLines: { display: true },
                        suggestedMin: 0,
                        suggestedMax: 100
                    }
                },
                plugins: {
                    title: {
                        display: true,
                        text: 'Canvas Component Completion'
                    }
                }
            }
        });
        
        // Discourse Chart
        const discourseCtx = document.getElementById('discourseChart').getContext('2d');
        new Chart(discourseCtx, {
            type: 'radar',
            data: {
                labels: ['Models', 'Controllers', 'Services', 'UI', 'Tests'],
                datasets: [{
                    label: 'Discourse Completion',
                    data: [
                        ${config.port.discourse.completion.models},
                        ${config.port.discourse.completion.controllers},
                        ${config.port.discourse.completion.services},
                        ${config.port.discourse.completion.ui},
                        ${config.port.discourse.completion.tests}
                    ],
                    fill: true,
                    backgroundColor: 'rgba(46, 204, 113, 0.2)',
                    borderColor: 'rgb(46, 204, 113)',
                    pointBackgroundColor: 'rgb(46, 204, 113)',
                    pointBorderColor: '#fff',
                    pointHoverBackgroundColor: '#fff',
                    pointHoverBorderColor: 'rgb(46, 204, 113)'
                }]
            },
            options: {
                elements: {
                    line: { borderWidth: 3 }
                },
                scales: {
                    r: {
                        angleLines: { display: true },
                        suggestedMin: 0,
                        suggestedMax: 100
                    }
                },
                plugins: {
                    title: {
                        display: true,
                        text: 'Discourse Component Completion'
                    }
                }
            }
        });
    </script>
</body>
</html>`;

  // Write the dashboard HTML file
  const dashboardPath = path.join(__dirname, 'dashboard.html');
  fs.writeFileSync(dashboardPath, htmlContent);
  
  console.log(`Visual dashboard generated at ${dashboardPath}`);
  return dashboardPath;
}

/**
 * Calculate the overall completion percentage
 */
function calculateOverall(completion) {
  const sum = Object.values(completion).reduce((total, val) => total + val, 0);
  return Math.round(sum / Object.values(completion).length);
}

// Export the function
module.exports = { generateVisualDashboard };

// If run directly, execute the generator
if (require.main === module) {
  generateVisualDashboard()
    .then(dashboardPath => console.log(`Dashboard generated at: ${dashboardPath}`))
    .catch(error => console.error('Failed to generate visual dashboard:', error));
}