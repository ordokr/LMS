import os
import json
import re
from datetime import datetime

def generate_dashboard():
    """Generate an HTML dashboard for port status."""
    # Check if audit report exists
    if not os.path.exists("audit_report.json"):
        print("Run the audit first: python tools/audit_codebase.py")
        return
    
    # Load audit data
    with open("audit_report.json", "r") as f:
        audit_data = json.load(f)
    
    # Count models by source
    canvas_models = [m for m in audit_data if m['source'] == 'Canvas']
    discourse_models = [m for m in audit_data if m['source'] == 'Discourse']
    
    # Calculate average completion
    avg_completion = sum(m['percentage'] for m in audit_data) / len(audit_data) if audit_data else 0
    
    # Generate HTML
    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Canvas/Discourse Port Dashboard</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            margin: 0;
            padding: 0;
            background-color: #f5f5f5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }}
        .header {{
            background-color: #2c3e50;
            color: white;
            padding: 20px;
            margin-bottom: 20px;
        }}
        .stats-container {{
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
            margin-bottom: 20px;
        }}
        .stat-card {{
            background-color: white;
            border-radius: 8px;
            padding: 20px;
            flex: 1;
            min-width: 200px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        .stat-card h3 {{
            margin-top: 0;
            color: #2c3e50;
        }}
        .stat-card .value {{
            font-size: 24px;
            font-weight: bold;
            margin: 10px 0;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            background-color: white;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        th, td {{
            padding: 12px 15px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}
        th {{
            background-color: #2c3e50;
            color: white;
        }}
        tr:hover {{
            background-color: #f1f1f1;
        }}
        .progress-bar {{
            height: 10px;
            background-color: #e0e0e0;
            border-radius: 5px;
            overflow: hidden;
        }}
        .progress-bar-fill {{
            height: 100%;
            background-color: #27ae60;
        }}
        .low {{
            background-color: #e74c3c;
        }}
        .medium {{
            background-color: #f39c12;
        }}
        .high {{
            background-color: #27ae60;
        }}
    </style>
</head>
<body>
    <div class="header">
        <div class="container">
            <h1>Canvas/Discourse Port Dashboard</h1>
            <p>Generated on {datetime.now().strftime('%Y-%m-%d %H:%M')}</p>
        </div>
    </div>
    
    <div class="container">
        <div class="stats-container">
            <div class="stat-card">
                <h3>Overall Completion</h3>
                <div class="value">{avg_completion:.1f}%</div>
                <div class="progress-bar">
                    <div class="progress-bar-fill {'low' if avg_completion < 50 else 'medium' if avg_completion < 80 else 'high'}" style="width: {avg_completion}%;"></div>
                </div>
            </div>
            <div class="stat-card">
                <h3>Canvas Models</h3>
                <div class="value">{len(canvas_models)}</div>
                <p>Models ported from Canvas</p>
            </div>
            <div class="stat-card">
                <h3>Discourse Models</h3>
                <div class="value">{len(discourse_models)}</div>
                <p>Models ported from Discourse</p>
            </div>
        </div>
        
        <h2>Model Completion Status</h2>
        <table>
            <thead>
                <tr>
                    <th>Model</th>
                    <th>Source</th>
                    <th>Completion</th>
                    <th>File</th>
                </tr>
            </thead>
            <tbody>
"""
    
    # Sort models by completion percentage
    sorted_models = sorted(audit_data, key=lambda x: x['percentage'], reverse=True)
    
    for model in sorted_models:
        color_class = 'low' if model['percentage'] < 50 else 'medium' if model['percentage'] < 80 else 'high'
        html += f"""
                <tr>
                    <td>{model['model']}</td>
                    <td>{model['source']}</td>
                    <td>
                        <div class="progress-bar">
                            <div class="progress-bar-fill {color_class}" style="width: {model['percentage']}%;"></div>
                        </div>
                        {model['percentage']:.1f}%
                    </td>
                    <td>{os.path.basename(model['file'])}</td>
                </tr>
"""
    
    html += """
            </tbody>
        </table>
    </div>
</body>
</html>
"""
    
    # Write HTML file
    with open("port_dashboard.html", "w") as f:
        f.write(html)
    
    print("Generated dashboard at port_dashboard.html")

if __name__ == "__main__":
    generate_dashboard()