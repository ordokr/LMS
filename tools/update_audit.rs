use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct AuditEntry {
    model: String,
    file: String,
    source: String,
    source_file: String,
    percentage: u8,
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the audit report
    let audit_path = Path::new("audit_report.json");
    let audit_content = fs::read_to_string(audit_path)?;
    let mut audit: Vec<AuditEntry> = serde_json::from_str(&audit_content)?;
    
    // Update entries
    for entry in &mut audit {
        // Check if the file exists
        let file_path = Path::new(&entry.file);
        if file_path.exists() {
            // Read file content
            let file_content = fs::read_to_string(file_path)?;
            
            // Check if model is implemented
            let model_name = entry.model.to_lowercase();
            if file_content.to_lowercase().contains(&format!("struct {}", model_name)) {
                entry.percentage = 100;
                entry.output = format!("Found struct {} implementation", entry.model);
            } else {
                entry.output = format!("Error: Could not find struct {} in the file\n", entry.model);
            }
        } else {
            entry.output = format!("Error: File {} does not exist\n", entry.file);
        }
    }
    
    // Write updated audit report
    let updated_audit = serde_json::to_string_pretty(&audit)?;
    fs::write(audit_path, updated_audit)?;
    
    println!("Audit report updated successfully");
    
    Ok(())
}