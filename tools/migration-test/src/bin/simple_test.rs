use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Simple Migration Visualization");
    
    // Create output directory
    let output_dir = PathBuf::from("test_output");
    fs::create_dir_all(&output_dir)?;
    
    // Generate a simple HTML report
    let html = generate_html_report();
    
    // Save HTML report to file
    let html_path = output_dir.join("migration_visualization.html");
    fs::write(&html_path, html)?;
    
    println!("Migration visualization generated successfully: {:?}", html_path);
    
    Ok(())
}

fn generate_html_report() -> String {
    // Sample data
    let react_components = 10;
    let ember_components = 5;
    let vue_components = 3;
    let angular_components = 2;
    
    let not_started = 12;
    let in_progress = 2;
    let completed = 4;
    let failed = 1;
    let skipped = 1;
    
    // Generate HTML
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Migration Visualization</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .container {{ display: flex; flex-wrap: wrap; }}
        .chart-container {{ width: 500px; height: 400px; margin: 20px; }}
        h1, h2 {{ color: #333; }}
        .stats {{ margin: 20px; }}
        .progress-bar {{ 
            height: 30px; 
            background-color: #f0f0f0; 
            border-radius: 5px; 
            margin: 10px 0; 
        }}
        .progress {{ 
            height: 100%; 
            background-color: #4CAF50; 
            border-radius: 5px; 
            width: 20%; 
        }}
    </style>
</head>
<body>
    <h1>Migration Visualization</h1>
    
    <div class="stats">
        <h2>Migration Progress</h2>
        <div class="progress-bar">
            <div class="progress"></div>
        </div>
        <p>20% Complete (4 out of 20 components)</p>
    </div>
    
    <div class="container">
        <div class="chart-container">
            <h2>Components by Type</h2>
            <canvas id="typeChart"></canvas>
        </div>
        
        <div class="chart-container">
            <h2>Migration Status</h2>
            <canvas id="statusChart"></canvas>
        </div>
    </div>
    
    <script>
        // Components by Type Chart
        const typeCtx = document.getElementById('typeChart').getContext('2d');
        const typeChart = new Chart(typeCtx, {{
            type: 'pie',
            data: {{
                labels: ['React', 'Ember', 'Vue', 'Angular'],
                datasets: [{{
                    data: [{}, {}, {}, {}],
                    backgroundColor: [
                        'rgba(54, 162, 235, 0.7)',
                        'rgba(255, 99, 132, 0.7)',
                        'rgba(75, 192, 192, 0.7)',
                        'rgba(255, 159, 64, 0.7)'
                    ],
                    borderColor: [
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 99, 132, 1)',
                        'rgba(75, 192, 192, 1)',
                        'rgba(255, 159, 64, 1)'
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
                    title: {{
                        display: true,
                        text: 'Components by Type'
                    }}
                }}
            }}
        }});
        
        // Migration Status Chart
        const statusCtx = document.getElementById('statusChart').getContext('2d');
        const statusChart = new Chart(statusCtx, {{
            type: 'doughnut',
            data: {{
                labels: ['Not Started', 'In Progress', 'Completed', 'Failed', 'Skipped'],
                datasets: [{{
                    data: [{}, {}, {}, {}, {}],
                    backgroundColor: [
                        'rgba(201, 203, 207, 0.7)',
                        'rgba(255, 205, 86, 0.7)',
                        'rgba(75, 192, 192, 0.7)',
                        'rgba(255, 99, 132, 0.7)',
                        'rgba(153, 102, 255, 0.7)'
                    ],
                    borderColor: [
                        'rgb(201, 203, 207)',
                        'rgb(255, 205, 86)',
                        'rgb(75, 192, 192)',
                        'rgb(255, 99, 132)',
                        'rgb(153, 102, 255)'
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
                    title: {{
                        display: true,
                        text: 'Migration Status'
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>
"#,
        react_components,
        ember_components,
        vue_components,
        angular_components,
        not_started,
        in_progress,
        completed,
        failed,
        skipped,
    );
    
    html
}
