use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating database schema visualization...");

    // Create the visualizations directory if it doesn't exist
    let base_dir = PathBuf::from(".");
    let visualizations_dir = base_dir.join("docs").join("visualizations").join("db_schema");
    if !visualizations_dir.exists() {
        fs::create_dir_all(&visualizations_dir)?;
    }

    // Generate Mermaid diagram with example models
    let mermaid_content = generate_example_mermaid_diagram();

    // Generate HTML file with Mermaid diagram
    let html_content = generate_html_with_mermaid(&mermaid_content);

    // Write HTML file
    let html_path = visualizations_dir.join("db_schema.html");
    fs::write(&html_path, html_content)?;

    println!("Database schema visualization generated at: {:?}", html_path);

    // Also generate a Markdown file with the Mermaid diagram
    let md_content = format!("# Database Schema\n\n```mermaid\n{}\n```\n", mermaid_content);
    let md_path = visualizations_dir.join("db_schema.md");
    fs::write(&md_path, md_content)?;

    println!("Database schema Markdown generated at: {:?}", md_path);

    Ok(())
}

/// Generate a Mermaid diagram with example models
fn generate_example_mermaid_diagram() -> String {
    println!("Generating example Mermaid diagram...");

    let mut mermaid = String::from("erDiagram\n");

    // User model
    mermaid.push_str("    User {\n");
    mermaid.push_str("        i64 id\n");
    mermaid.push_str("        String name\n");
    mermaid.push_str("        String email\n");
    mermaid.push_str("        DateTime created_at\n");
    mermaid.push_str("    }\n");

    // Course model
    mermaid.push_str("    Course {\n");
    mermaid.push_str("        i64 id\n");
    mermaid.push_str("        String title\n");
    mermaid.push_str("        String description\n");
    mermaid.push_str("        i64 instructor_id\n");
    mermaid.push_str("        DateTime created_at\n");
    mermaid.push_str("    }\n");

    // Enrollment model
    mermaid.push_str("    Enrollment {\n");
    mermaid.push_str("        i64 id\n");
    mermaid.push_str("        i64 user_id\n");
    mermaid.push_str("        i64 course_id\n");
    mermaid.push_str("        String role\n");
    mermaid.push_str("        DateTime created_at\n");
    mermaid.push_str("    }\n");

    // Assignment model
    mermaid.push_str("    Assignment {\n");
    mermaid.push_str("        i64 id\n");
    mermaid.push_str("        i64 course_id\n");
    mermaid.push_str("        String title\n");
    mermaid.push_str("        String description\n");
    mermaid.push_str("        DateTime due_date\n");
    mermaid.push_str("        f64 points_possible\n");
    mermaid.push_str("        DateTime created_at\n");
    mermaid.push_str("    }\n");

    // Submission model
    mermaid.push_str("    Submission {\n");
    mermaid.push_str("        i64 id\n");
    mermaid.push_str("        i64 assignment_id\n");
    mermaid.push_str("        i64 user_id\n");
    mermaid.push_str("        String content\n");
    mermaid.push_str("        Option<f64> score\n");
    mermaid.push_str("        DateTime submitted_at\n");
    mermaid.push_str("    }\n");

    // Discussion model
    mermaid.push_str("    Discussion {\n");
    mermaid.push_str("        i64 id\n");
    mermaid.push_str("        i64 course_id\n");
    mermaid.push_str("        String title\n");
    mermaid.push_str("        String content\n");
    mermaid.push_str("        i64 user_id\n");
    mermaid.push_str("        DateTime created_at\n");
    mermaid.push_str("    }\n");

    // DiscussionPost model
    mermaid.push_str("    DiscussionPost {\n");
    mermaid.push_str("        i64 id\n");
    mermaid.push_str("        i64 discussion_id\n");
    mermaid.push_str("        i64 user_id\n");
    mermaid.push_str("        String content\n");
    mermaid.push_str("        Option<i64> parent_id\n");
    mermaid.push_str("        DateTime created_at\n");
    mermaid.push_str("    }\n");

    // Add relationships
    mermaid.push_str("    Course 1--* Enrollment : \"has\"\n");
    mermaid.push_str("    User 1--* Enrollment : \"has\"\n");
    mermaid.push_str("    Course 1--* Assignment : \"has\"\n");
    mermaid.push_str("    Assignment 1--* Submission : \"has\"\n");
    mermaid.push_str("    User 1--* Submission : \"makes\"\n");
    mermaid.push_str("    Course 1--* Discussion : \"has\"\n");
    mermaid.push_str("    Discussion 1--* DiscussionPost : \"has\"\n");
    mermaid.push_str("    User 1--* DiscussionPost : \"creates\"\n");

    mermaid
}

/// Generate HTML with Mermaid diagram
fn generate_html_with_mermaid(mermaid_content: &str) -> String {
    println!("Generating HTML with Mermaid diagram...");

    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ordo Database Schema</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #333;
            border-bottom: 1px solid #eee;
            padding-bottom: 10px;
        }}
        .mermaid {{
            margin-top: 20px;
        }}
        .legend {{
            margin-top: 20px;
            padding: 10px;
            background-color: #f9f9f9;
            border-radius: 5px;
        }}
        .legend h3 {{
            margin-top: 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Ordo Database Schema</h1>
        <div class="mermaid">
{mermaid_content}
        </div>
        <div class="legend">
            <h3>Legend</h3>
            <p>This diagram shows the database schema for the Ordo application. Each box represents a table in the database, and the lines represent relationships between tables.</p>
            <p>Cardinality notation:</p>
            <ul>
                <li><strong>1--1</strong>: One-to-one relationship</li>
                <li><strong>1--*</strong>: One-to-many relationship</li>
                <li><strong>*--*</strong>: Many-to-many relationship</li>
            </ul>
        </div>
    </div>
    <script>
        mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
    </script>
</body>
</html>"#, mermaid_content = mermaid_content)
}
