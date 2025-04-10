// tools/project-analyzer/src/docs_cleanup.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// Script to consolidate documentation and ensure project_status.md is the
/// single source of truth for project information
pub fn cleanup_documentation() -> io::Result<()> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let docs_dir = base_dir.join("docs");
    
    // Ensure docs directory exists
    fs::create_dir_all(&docs_dir)?;
    
    // List of files that should be moved to docs directory
    let files_to_move = vec![
        "implementation_details.md",
        "relationship_map.md"
    ];
    
    // Move files to docs directory
    for file in files_to_move {
        let file_path = base_dir.join(file);
        if file_path.exists() {
            let target_path = docs_dir.join(file);
            fs::copy(&file_path, &target_path)?;
            fs::remove_file(&file_path)?;
            println!("Moved {} to docs directory", file);
        }
    }
    
    // Process other document consolidation
    consolidate_project_documents(&docs_dir)?;
    
    // Update references in summary files
    update_summary_references(&docs_dir)?;
    
    println!("Documentation cleanup completed successfully");
    Ok(())
}

/// Consolidate project documents into a single source of truth
fn consolidate_project_documents(docs_dir: &Path) -> io::Result<()> {
    // Find all markdown files in docs directory
    let entries = fs::read_dir(docs_dir)?;
    
    // Load main project status document
    let project_status_path = docs_dir.join("project_status.md");
    let mut project_status_content = if project_status_path.exists() {
        fs::read_to_string(&project_status_path)?
    } else {
        String::from("# Project Status\n\n## Overview\n\nThis document serves as the single source of truth for project status.\n\n")
    };
    
    // Temporary storage for consolidated sections
    let mut sections: Vec<(String, String)> = Vec::new();
    
    // Process each markdown file
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "md") 
            && path.file_name().map_or(false, |name| name != "project_status.md") {
            
            // Read file content
            let content = fs::read_to_string(&path)?;
            
            // Extract sections
            extract_sections(&content, &mut sections);
        }
    }
    
    // Append unique sections to project status document
    for (title, content) in sections {
        if !project_status_content.contains(&title) {
            project_status_content.push_str(&format!("\n## {}\n\n{}\n", title, content));
        }
    }
    
    // Write updated project status document
    fs::write(&project_status_path, project_status_content)?;
    
    Ok(())
}

/// Extract markdown sections (h2) from content
fn extract_sections(content: &str, sections: &mut Vec<(String, String)>) {
    let mut lines = content.lines().peekable();
    
    while let Some(line) = lines.next() {
        if line.starts_with("## ") {
            let title = line.trim_start_matches("## ").trim().to_string();
            let mut section_content = String::new();
            
            // Collect all content until the next h2 or end of file
            while let Some(next_line) = lines.peek() {
                if next_line.starts_with("## ") {
                    break;
                }
                section_content.push_str(lines.next().unwrap());
                section_content.push('\n');
            }
            
            sections.push((title, section_content.trim().to_string()));
        }
    }
}

/// Update references to moved files in summary documents
fn update_summary_references(docs_dir: &Path) -> io::Result<()> {
    let summary_path = docs_dir.join("SUMMARY.md");
    
    if summary_path.exists() {
        let summary_content = fs::read_to_string(&summary_path)?;
        let updated_content = summary_content
            .replace("../implementation_details.md", "implementation_details.md")
            .replace("../relationship_map.md", "relationship_map.md");
        
        fs::write(&summary_path, updated_content)?;
        println!("Updated references in SUMMARY.md");
    }
    
    Ok(())
}
