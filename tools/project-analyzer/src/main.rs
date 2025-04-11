//! Project Analyzer for JavaScript to Rust Migration
//!
//! This tool analyzes the project to determine the status of JavaScript to Rust migration,
//! identifies remaining JavaScript files that need to be migrated, and updates the migration
//! tracking document with the latest statistics.

use std::fs::{self, File};
use std::io::Write;
use std::error::Error;
use structopt::StructOpt;
use regex::Regex;

mod js_migration_analyzer;
use js_migration_analyzer::{JsMigrationAnalyzer, MigrationStatus};

mod template_generator;

#[derive(Debug, StructOpt)]
#[structopt(name = "project-analyzer", about = "Analyzes JavaScript to Rust migration progress")]
struct Opt {
    /// Path to the migration tracking document
    #[structopt(short, long, default_value = "JavaScript to Rust Migration Tracking.md")]
    tracking_doc: String,

    /// Path to the project root
    #[structopt(short, long, default_value = ".")]
    project_root: String,

    /// Whether to update the tracking document with the analysis results
    #[structopt(short, long)]
    update: bool,
    
    /// Generate a JSON report of migration status
    #[structopt(short, long)]
    json: bool,
    
    /// Path to output JSON report (defaults to migration-status.json)
    #[structopt(long, default_value = "migration-status.json")]
    json_output: String,
    
    /// Generate templates for high priority files
    #[structopt(long)]
    template_high_priority: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    
    println!("Analyzing JavaScript to Rust migration progress...");
    
    // Create the analyzer with the project root
    let mut analyzer = JsMigrationAnalyzer::new(opt.project_root.clone())
        .with_tracking(opt.tracking_doc.clone());
    
    // Discover JavaScript files
    analyzer.discover_js_files();
    
    // Generate migration plan
    let migration_plan = analyzer.generate_migration_plan()?;
    
    // Print migration plan
    println!("\n{}", migration_plan);
    
    // Update tracking document if requested
    if opt.update {
        update_tracking_document(&opt.tracking_doc, &migration_plan)?;
        println!("Updated tracking document: {}", opt.tracking_doc);
    }
    
    // Generate JSON report if requested
    if opt.json {
        let analysis = analyzer.analyze_js_files();
        generate_json_report(&opt.json_output, &analysis)?;
        println!("Generated JSON report: {}", opt.json_output);
    }
    
    // Generate templates for high priority files if requested
    if opt.template_high_priority {
        let analysis = analyzer.analyze_js_files();
        generate_templates_for_high_priority(&analysis)?;
    }
    
    Ok(())
}

fn update_tracking_document(path: &str, migration_plan: &str) -> Result<(), Box<dyn Error>> {
    // Read the existing document
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    // Extract statistics from migration plan
    let total_regex = Regex::new(r"Total JavaScript files: (\d+)").unwrap();
    let completed_regex = Regex::new(r"Completed migrations: (\d+)").unwrap();
    let in_progress_regex = Regex::new(r"In-progress migrations: (\d+)").unwrap();
    let not_started_regex = Regex::new(r"Not started migrations: (\d+)").unwrap();
    let not_needed_regex = Regex::new(r"Not needed migrations: (\d+)").unwrap();
    
    let total = total_regex.captures(migration_plan)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap_or("TBD");
        
    let completed = completed_regex.captures(migration_plan)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap_or("TBD");
        
    let in_progress = in_progress_regex.captures(migration_plan)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap_or("TBD");
        
    let not_started = not_started_regex.captures(migration_plan)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap_or("TBD");
        
    let not_needed = not_needed_regex.captures(migration_plan)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap_or("TBD");
    
    // Update document with new statistics and in-progress/not-started sections
    let mut updated_content = Vec::new();
    let mut in_progress_section = false;
    let mut not_started_section = false;
    let mut progress_section = false;
    
    for line in lines {
        if line.contains("## Migration Progress") {
            progress_section = true;
            updated_content.push(line.to_string());
            continue;
        }
        
        if progress_section && line.trim().is_empty() {
            progress_section = false;
            updated_content.push("".to_string());
            updated_content.push(format!("- Total JavaScript files: {}", total));
            updated_content.push(format!("- Migration completed: {}", completed));
            updated_content.push(format!("- Migration not started: {}", not_started));
            updated_content.push(format!("- Migration in progress: {}", in_progress));
            updated_content.push(format!("- Migration not needed: {}", not_needed));
            updated_content.push("".to_string());
            continue;
        }
        
        if progress_section {
            // Skip existing progress stats
            continue;
        }
        
        if line.contains("## In Progress Migrations") {
            in_progress_section = true;
            updated_content.push(line.to_string());
            
            // Extract in-progress section from migration plan
            for plan_line in migration_plan.lines() {
                if plan_line.contains("## In Progress Migrations") {
                    continue;
                } else if plan_line.contains("## Not Started Migrations") {
                    break;
                }
                
                if !plan_line.trim().is_empty() {
                    updated_content.push(plan_line.to_string());
                }
            }
            
            continue;
        }
        
        if line.contains("## Not Started Migrations") {
            not_started_section = true;
            in_progress_section = false;
            updated_content.push(line.to_string());
            
            // Extract not-started section from migration plan
            for plan_line in migration_plan.lines() {
                if plan_line.contains("## Not Started Migrations") {
                    continue;
                } else if plan_line.contains("## Rules for JavaScript to Rust Migration") {
                    break;
                }
                
                if !plan_line.trim().is_empty() {
                    updated_content.push(plan_line.to_string());
                }
            }
            
            continue;
        }
        
        if line.contains("## Rules for JavaScript to Rust Migration") {
            not_started_section = false;
        }
        
        // Skip lines in sections we're replacing
        if (in_progress_section || not_started_section) && !line.contains("##") {
            continue;
        }
        
        // Keep other lines
        updated_content.push(line.to_string());
    }
    
    // Write updated content
    let mut file = File::create(path)?;
    for line in updated_content {
        writeln!(file, "{}", line)?;
    }
    
    Ok(())
}

fn generate_json_report(path: &str, analysis: &js_migration_analyzer::JsMigrationAnalysis) -> Result<(), Box<dyn Error>> {
    let mut report = serde_json::Map::new();
    
    report.insert("totalJsFiles".to_string(), serde_json::Value::Number(analysis.total_js_files.into()));
    report.insert("completedMigrations".to_string(), serde_json::Value::Number(analysis.completed_migrations.into()));
    report.insert("inProgressMigrations".to_string(), serde_json::Value::Number(analysis.in_progress_migrations.into()));
    report.insert("notStartedMigrations".to_string(), serde_json::Value::Number(analysis.not_started_migrations.into()));
    report.insert("notNeededMigrations".to_string(), serde_json::Value::Number(analysis.not_needed_migrations.into()));
    
    let completion_percentage = if analysis.total_js_files > analysis.not_needed_migrations {
        (analysis.completed_migrations as f64 / 
        (analysis.total_js_files - analysis.not_needed_migrations) as f64) * 100.0
    } else {
        100.0 // All files are either completed or not needed
    };
    
    report.insert("completionPercentage".to_string(), 
        serde_json::Value::Number(serde_json::Number::from_f64(completion_percentage).unwrap_or(serde_json::Number::from(0))));
    
    // Add mappings
    let mut mappings_array = Vec::new();
    for file in &analysis.js_files {
        let mut mapping_obj = serde_json::Map::new();
        mapping_obj.insert("jsFile".to_string(), serde_json::Value::String(file.relative_path.clone()));
        
        if let Some(rust_path) = &file.rust_path {
            mapping_obj.insert("rustFile".to_string(), serde_json::Value::String(rust_path.clone()));
        } else {
            mapping_obj.insert("rustFile".to_string(), serde_json::Value::String("N/A".to_string()));
        }
        
        let status_str = match file.migration_status {
            MigrationStatus::Completed => "completed",
            MigrationStatus::InProgress => "inProgress",
            MigrationStatus::NotStarted => "notStarted",
            MigrationStatus::NotNeeded => "notNeeded",
        };
        
        mapping_obj.insert("status".to_string(), serde_json::Value::String(status_str.to_string()));
        mapping_obj.insert("priorityScore".to_string(), serde_json::Value::Number(file.priority_score.into()));
        
        mappings_array.push(serde_json::Value::Object(mapping_obj));
    }
    
    report.insert("mappings".to_string(), serde_json::Value::Array(mappings_array));
    
    // Add high priority files
    let high_priority_array = analysis.high_priority_files.iter()
        .map(|f| serde_json::Value::String(f.clone()))
        .collect();
        
    report.insert("highPriorityFiles".to_string(), serde_json::Value::Array(high_priority_array));
    
    // Write to file
    let json_content = serde_json::to_string_pretty(&serde_json::Value::Object(report))?;
    fs::write(path, json_content)?;
    
    Ok(())
}

fn generate_templates_for_high_priority(analysis: &js_migration_analyzer::JsMigrationAnalysis) -> Result<(), Box<dyn Error>> {
    println!("Generating Rust templates for high priority files...");
    
    let mut success_count = 0;
    let mut failure_count = 0;
    
    for js_file in &analysis.high_priority_files {
        // Find the corresponding JsFile info
        if let Some(file_info) = analysis.js_files.iter().find(|f| f.relative_path == *js_file) {
            if let Some(rust_path) = &file_info.rust_path {
                println!("Generating template for {} -> {}", js_file, rust_path);
                
                match template_generator::generate_rust_template(js_file, rust_path) {
                    Ok(_) => {
                        println!("Successfully generated template for {}", js_file);
                        success_count += 1;
                    },
                    Err(e) => {
                        println!("Failed to generate template for {}: {}", js_file, e);
                        failure_count += 1;
                    }
                }
            }
        }
    }
    
    println!("Template generation completed: {} succeeded, {} failed", success_count, failure_count);
    Ok(())
}


