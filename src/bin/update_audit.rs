use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct AuditEntry {
    model: String,
    file: String,
    source: String,
    source_file: String,
    percentage: u8,
    output: String,
}

#[derive(Debug)]
struct ModelImplementation {
    struct_found: bool,
    method_count: usize,
    methods_implemented: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the audit report
    let audit_path = Path::new("audit_report.json");
    let audit_content = fs::read_to_string(audit_path)?;
    let mut audit: Vec<AuditEntry> = serde_json::from_str(&audit_content)?;
    
    println!("Updating audit report...");
    
    // Update entries
    for entry in &mut audit {
        println!("Checking model: {}", entry.model);
        
        // Check if the file exists
        let file_path = Path::new(&entry.file);
        if !file_path.exists() {
            entry.percentage = 0;
            entry.output = format!("Error: File {} does not exist\n", entry.file);
            continue;
        }
        
        // Read file content
        let file_content = fs::read_to_string(file_path)?;
        
        // Find model implementation
        let model_name = entry.model.to_lowercase();
        let implementation = analyze_model_implementation(&file_content, &model_name);
        
        if implementation.struct_found {
            // Calculate percentage based on method count
            // This is a simple heuristic - you might want to make this more sophisticated
            let percentage = if implementation.method_count > 0 {
                ((implementation.methods_implemented.len() as f32 / implementation.method_count as f32) * 100.0) as u8
            } else {
                // If we found the struct but no methods, consider it 50% implemented
                50
            };
            
            entry.percentage = percentage.min(100);
            entry.output = if percentage >= 100 {
                format!("Found complete implementation for {}\n", entry.model)
            } else {
                format!(
                    "Found partial implementation for {}. Implemented {}/{} methods: {}\n",
                    entry.model,
                    implementation.methods_implemented.len(),
                    implementation.method_count,
                    implementation.methods_implemented.join(", ")
                )
            };
        } else {
            entry.percentage = 0;
            entry.output = format!("Error: Could not find struct {} in the file\n", entry.model);
        }
    }
    
    // Write updated audit report
    let updated_audit = serde_json::to_string_pretty(&audit)?;
    fs::write(audit_path, updated_audit)?;
    
    println!("Audit report updated successfully");
    
    Ok(())
}

fn analyze_model_implementation(file_content: &str, model_name: &str) -> ModelImplementation {
    let mut result = ModelImplementation {
        struct_found: false,
        method_count: 0,
        methods_implemented: Vec::new(),
    };
    
    // Simple parsing to look for struct definition
    let struct_pattern = format!("struct {}", model_name);
    if file_content.to_lowercase().contains(&struct_pattern) {
        result.struct_found = true;
        
        // Look for impl blocks
        let lines: Vec<&str> = file_content.lines().collect();
        let mut in_impl = false;
        
        for (i, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check for impl block start
            if line_lower.contains(&format!("impl {}", model_name)) {
                in_impl = true;
                continue;
            }
            
            // Check for end of impl block
            if in_impl && line.trim() == "}" {
                in_impl = false;
                continue;
            }
            
            // Count methods within the impl block
            if in_impl && line_lower.contains("pub fn ") {
                result.method_count += 1;
                
                // Extract method name
                if let Some(name_start) = line_lower.find("fn ") {
                    if let Some(name_end) = line_lower[name_start + 3..].find('(') {
                        let method_name = line_lower[name_start + 3..name_start + 3 + name_end].trim().to_string();
                        result.methods_implemented.push(method_name);
                    }
                }
            }
        }
    }
    
    result
}