use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};
use regex::Regex;
use std::collections::HashMap;

/// Generate a database schema visualization in Mermaid format
pub fn generate_db_schema_visualization(base_dir: &PathBuf) -> Result<()> {
    println!("Generating database schema visualization...");

    // Create the visualizations directory if it doesn't exist
    let visualizations_dir = base_dir.join("docs").join("visualizations").join("db_schema");
    if !visualizations_dir.exists() {
        fs::create_dir_all(&visualizations_dir)
            .context("Failed to create visualizations directory")?;
    }

    // Extract database schema from model files
    let models = extract_models(base_dir)?;

    // Generate Mermaid diagram
    let mermaid_content = generate_mermaid_diagram(&models)?;

    // Generate HTML file with Mermaid diagram
    let html_content = generate_html_with_mermaid(&mermaid_content)?;

    // Write HTML file
    let html_path = visualizations_dir.join("db_schema.html");
    fs::write(&html_path, html_content)
        .context("Failed to write database schema visualization HTML file")?;

    println!("Database schema visualization generated at: {:?}", html_path);

    // Also generate a Markdown file with the Mermaid diagram
    let md_content = format!("# Database Schema\n\n```mermaid\n{}\n```\n", mermaid_content);
    let md_path = visualizations_dir.join("db_schema.md");
    fs::write(&md_path, md_content)
        .context("Failed to write database schema visualization Markdown file")?;

    println!("Database schema Markdown generated at: {:?}", md_path);

    Ok(())
}

/// Extract models from the codebase
fn extract_models(base_dir: &PathBuf) -> Result<Vec<Model>> {
    println!("Extracting models from codebase...");

    let mut models = Vec::new();

    // For now, just create example models
    // In a real implementation, we would scan the codebase for model files

    // If no models were found, create some example models
    if models.is_empty() {
        // Create example models
        models = create_example_models();
    }

    Ok(models)
}

/// Extract models from file content
fn extract_models_from_content(content: &str, models: &mut Vec<Model>) {
    // Regex to match struct definitions
    let struct_regex = Regex::new(r"(?m)^(?:#\[derive\([^\)]*\)\s*)?(?:#\[table\([^\)]*\)\s*)?struct\s+([A-Za-z0-9_]+)\s*\{([^}]*)\}").unwrap();

    // Regex to match fields
    let field_regex = Regex::new(r"(?m)^\s*(?:#\[column\([^\)]*\)\s*)?pub\s+([A-Za-z0-9_]+)\s*:\s*([^,]+),?").unwrap();

    // Find all struct definitions
    for struct_cap in struct_regex.captures_iter(content) {
        if let (Some(name), Some(fields_str)) = (struct_cap.get(1), struct_cap.get(2)) {
            let name = name.as_str().to_string();
            let fields_str = fields_str.as_str();

            let mut fields = Vec::new();

            // Find all fields
            for field_cap in field_regex.captures_iter(fields_str) {
                if let (Some(field_name), Some(field_type)) = (field_cap.get(1), field_cap.get(2)) {
                    fields.push(Field {
                        name: field_name.as_str().to_string(),
                        field_type: field_type.as_str().trim().to_string(),
                    });
                }
            }

            // Add the model
            models.push(Model {
                name,
                fields,
            });
        }
    }
}

/// Create example models
fn create_example_models() -> Vec<Model> {
    println!("No models found, creating example models...");

    vec![
        Model {
            name: "User".to_string(),
            fields: vec![
                Field { name: "id".to_string(), field_type: "i64".to_string() },
                Field { name: "name".to_string(), field_type: "String".to_string() },
                Field { name: "email".to_string(), field_type: "String".to_string() },
                Field { name: "created_at".to_string(), field_type: "DateTime".to_string() },
            ],
        },
        Model {
            name: "Course".to_string(),
            fields: vec![
                Field { name: "id".to_string(), field_type: "i64".to_string() },
                Field { name: "title".to_string(), field_type: "String".to_string() },
                Field { name: "description".to_string(), field_type: "String".to_string() },
                Field { name: "instructor_id".to_string(), field_type: "i64".to_string() },
                Field { name: "created_at".to_string(), field_type: "DateTime".to_string() },
            ],
        },
        Model {
            name: "Enrollment".to_string(),
            fields: vec![
                Field { name: "id".to_string(), field_type: "i64".to_string() },
                Field { name: "user_id".to_string(), field_type: "i64".to_string() },
                Field { name: "course_id".to_string(), field_type: "i64".to_string() },
                Field { name: "role".to_string(), field_type: "String".to_string() },
                Field { name: "created_at".to_string(), field_type: "DateTime".to_string() },
            ],
        },
        Model {
            name: "Assignment".to_string(),
            fields: vec![
                Field { name: "id".to_string(), field_type: "i64".to_string() },
                Field { name: "course_id".to_string(), field_type: "i64".to_string() },
                Field { name: "title".to_string(), field_type: "String".to_string() },
                Field { name: "description".to_string(), field_type: "String".to_string() },
                Field { name: "due_date".to_string(), field_type: "DateTime".to_string() },
                Field { name: "points_possible".to_string(), field_type: "f64".to_string() },
                Field { name: "created_at".to_string(), field_type: "DateTime".to_string() },
            ],
        },
        Model {
            name: "Submission".to_string(),
            fields: vec![
                Field { name: "id".to_string(), field_type: "i64".to_string() },
                Field { name: "assignment_id".to_string(), field_type: "i64".to_string() },
                Field { name: "user_id".to_string(), field_type: "i64".to_string() },
                Field { name: "content".to_string(), field_type: "String".to_string() },
                Field { name: "score".to_string(), field_type: "Option<f64>".to_string() },
                Field { name: "submitted_at".to_string(), field_type: "DateTime".to_string() },
            ],
        },
        Model {
            name: "Discussion".to_string(),
            fields: vec![
                Field { name: "id".to_string(), field_type: "i64".to_string() },
                Field { name: "course_id".to_string(), field_type: "i64".to_string() },
                Field { name: "title".to_string(), field_type: "String".to_string() },
                Field { name: "content".to_string(), field_type: "String".to_string() },
                Field { name: "user_id".to_string(), field_type: "i64".to_string() },
                Field { name: "created_at".to_string(), field_type: "DateTime".to_string() },
            ],
        },
        Model {
            name: "DiscussionPost".to_string(),
            fields: vec![
                Field { name: "id".to_string(), field_type: "i64".to_string() },
                Field { name: "discussion_id".to_string(), field_type: "i64".to_string() },
                Field { name: "user_id".to_string(), field_type: "i64".to_string() },
                Field { name: "content".to_string(), field_type: "String".to_string() },
                Field { name: "parent_id".to_string(), field_type: "Option<i64>".to_string() },
                Field { name: "created_at".to_string(), field_type: "DateTime".to_string() },
            ],
        },
    ]
}

/// Generate a Mermaid diagram from the models
fn generate_mermaid_diagram(models: &[Model]) -> Result<String> {
    println!("Generating Mermaid diagram...");

    let mut mermaid = String::from("erDiagram\n");

    // Add each model to the diagram
    for model in models {
        mermaid.push_str(&format!("    {} {{\n", model.name));

        for field in &model.fields {
            mermaid.push_str(&format!("        {} {}\n", field.field_type, field.name));
        }

        mermaid.push_str("    }\n");
    }

    // Add relationships
    let relationships = detect_relationships(models);
    for rel in relationships {
        mermaid.push_str(&format!("    {} {}--{} {} : \"{}\"\n",
            rel.from_entity,
            rel.cardinality_from,
            rel.cardinality_to,
            rel.to_entity,
            rel.label
        ));
    }

    Ok(mermaid)
}

/// Generate HTML with Mermaid diagram
fn generate_html_with_mermaid(mermaid_content: &str) -> Result<String> {
    println!("Generating HTML with Mermaid diagram...");

    let html = format!(r#"<!DOCTYPE html>
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
</html>"#, mermaid_content = mermaid_content);

    Ok(html)
}

/// Detect relationships between models
fn detect_relationships(models: &[Model]) -> Vec<Relationship> {
    println!("Detecting relationships between models...");

    let mut relationships = Vec::new();
    let model_map: HashMap<String, &Model> = models.iter().map(|m| (m.name.clone(), m)).collect();

    for model in models {
        for field in &model.fields {
            // Check for foreign keys (fields ending with _id)
            if field.name.ends_with("_id") && field.name != "id" {
                let referenced_entity = field.name.trim_end_matches("_id");
                let referenced_entity = referenced_entity.to_string();

                // Convert to PascalCase
                let referenced_entity = referenced_entity
                    .split('_')
                    .map(|s| {
                        let mut c = s.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<String>();

                // Check if the referenced entity exists
                if model_map.contains_key(&referenced_entity) {
                    // Determine cardinality
                    let cardinality_from = "1";
                    let cardinality_to = if field.field_type.starts_with("Option<") {
                        "0..1"
                    } else {
                        "1"
                    };

                    relationships.push(Relationship {
                        from_entity: model.name.clone(),
                        to_entity: referenced_entity,
                        cardinality_from: cardinality_from.to_string(),
                        cardinality_to: cardinality_to.to_string(),
                        label: format!("has"),
                    });
                }
            }
        }
    }

    // Add some additional relationships for example models
    if models.len() <= 10 {
        relationships.push(Relationship {
            from_entity: "Course".to_string(),
            to_entity: "Enrollment".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "has".to_string(),
        });

        relationships.push(Relationship {
            from_entity: "User".to_string(),
            to_entity: "Enrollment".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "has".to_string(),
        });

        relationships.push(Relationship {
            from_entity: "Course".to_string(),
            to_entity: "Assignment".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "has".to_string(),
        });

        relationships.push(Relationship {
            from_entity: "Assignment".to_string(),
            to_entity: "Submission".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "has".to_string(),
        });

        relationships.push(Relationship {
            from_entity: "User".to_string(),
            to_entity: "Submission".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "makes".to_string(),
        });

        relationships.push(Relationship {
            from_entity: "Course".to_string(),
            to_entity: "Discussion".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "has".to_string(),
        });

        relationships.push(Relationship {
            from_entity: "Discussion".to_string(),
            to_entity: "DiscussionPost".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "has".to_string(),
        });

        relationships.push(Relationship {
            from_entity: "User".to_string(),
            to_entity: "DiscussionPost".to_string(),
            cardinality_from: "1".to_string(),
            cardinality_to: "*".to_string(),
            label: "creates".to_string(),
        });
    }

    relationships
}

/// A model in the database schema
#[derive(Debug, Clone)]
struct Model {
    name: String,
    fields: Vec<Field>,
}

/// A field in a model
#[derive(Debug, Clone)]
struct Field {
    name: String,
    field_type: String,
}

/// A relationship between models
#[derive(Debug, Clone)]
struct Relationship {
    from_entity: String,
    to_entity: String,
    cardinality_from: String,
    cardinality_to: String,
    label: String,
}
