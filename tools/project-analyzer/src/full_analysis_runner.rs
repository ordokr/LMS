use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;
use std::process::Command;
use std::io::Write;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use clap::{Parser, ValueEnum};

use crate::unified_project_analyzer::UnifiedProjectAnalyzer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AnalysisArgs {
    /// Base directory to analyze
    #[arg(short, long, default_value_t = String::from("."))]
    base_dir: String,

    /// Path to Canvas source code
    #[arg(long, default_value_t = String::from("../port/canvas"))]
    canvas: String,

    /// Path to Discourse source code
    #[arg(long, default_value_t = String::from("../port/discourse"))]
    discourse: String,

    /// Skip using cached results
    #[arg(long, default_value_t = false)]
    skip_cache: bool,

    /// Skip source system analysis
    #[arg(long, default_value_t = false)]
    skip_source_analysis: bool,

    /// Type of report to generate
    #[arg(short, long, value_enum, default_value_t = ReportType::Full)]
    report: ReportType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ReportType {
    Full,
    Summary,
    Source,
    CodeQuality,
}

/// Run a comprehensive analysis on source systems and target LMS application
pub async fn run_full_analysis(args: AnalysisArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("===================================");
    println!("🔍 STARTING COMPREHENSIVE ANALYSIS 🔍");
    println!("===================================\n");

    let start_time = Instant::now();
    
    // Base directories
    let lms_dir = std::env::current_dir()?;
    let port_dir = PathBuf::from("C:\\Users\\Tim\\Desktop\\port");
    let canvas_dir = if Path::new(&args.canvas).is_absolute() {
        PathBuf::from(&args.canvas)
    } else {
        port_dir.join("canvas")
    };
    let discourse_dir = if Path::new(&args.discourse).is_absolute() {
        PathBuf::from(&args.discourse)
    } else {
        port_dir.join("discourse")
    };
    
    // Create summary directory if it doesn't exist
    let summary_dir = lms_dir.join("analysis_summary");
    if !summary_dir.exists() {
        fs::create_dir_all(&summary_dir)?;
    }
    
    // Step 1: Analyze source systems (Canvas)
    println!("1️⃣ Analyzing Canvas (Source System)...");
    if canvas_dir.exists() {
        let canvas_output = run_source_analysis(&canvas_dir, "Canvas")?;
        fs::write(
            summary_dir.join("canvas_analysis.md"),
            canvas_output
        )?;
        println!("✅ Canvas analysis complete");
    } else {
        println!("⚠️ Canvas directory not found, skipping source analysis");
    }
    
    // Step 2: Analyze source systems (Discourse)
    println!("\n2️⃣ Analyzing Discourse (Source System)...");
    if discourse_dir.exists() {
        let discourse_output = run_source_analysis(&discourse_dir, "Discourse")?;
        fs::write(
            summary_dir.join("discourse_analysis.md"),
            discourse_output
        )?;
        println!("✅ Discourse analysis complete");
    } else {
        println!("⚠️ Discourse directory not found, skipping source analysis");
    }
    
    // Step 3: Analyze LMS application
    println!("\n3️⃣ Analyzing LMS Application...");
    let analyzer = UnifiedProjectAnalyzer::new(
        lms_dir.to_string_lossy().to_string(),
        None,
        !args.skip_cache,
    );
    
    await analyzer.analyze()?;
    
    println!("✅ LMS analysis complete");
    
    // Step 4: Generate consolidated report
    println!("\n4️⃣ Generating Consolidated Report...");
    generate_consolidated_report(&summary_dir, &lms_dir, &canvas_dir, &discourse_dir)?;
    
    let duration = start_time.elapsed();
    println!("\n✅ Comprehensive analysis completed in {:.2}s", duration.as_secs_f32());
    println!("📄 Master report generated at: {}", summary_dir.join("master_report.md").display());
    println!("📊 Central Reference Hub: {}", lms_dir.join("docs").join("central_reference_hub.md").display());
    
    Ok(())
}

/// Run analysis on source systems (Canvas/Discourse)
fn run_source_analysis(source_dir: &Path, system_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Analyzing {} codebase at {}...", system_name, source_dir.display());
    
    // Count files by type
    let file_stats = count_files_by_type(source_dir)?;
    
    // Count lines of code
    let loc_stats = count_lines_of_code(source_dir)?;
    
    // Analyze models
    let models = find_models(source_dir, &system_name.to_lowercase())?;
    
    // Analyze controllers
    let controllers = find_controllers(source_dir, &system_name.to_lowercase())?;
    
    // Format the output as markdown
    let mut output = format!("# {} Source Code Analysis\n\n", system_name);
    output.push_str(&format!("_Analysis performed on {}_\n\n", chrono::Local::now().date_naive()));
    
    // High level stats
    output.push_str("## Overview\n\n");
    output.push_str(&format!("- **Total Files**: {}\n", file_stats.total));
    output.push_str(&format!("- **Lines of Code**: {}\n", loc_stats.total.to_formatted_string()));
    output.push_str(&format!("- **Models**: {}\n", models.len()));
    output.push_str(&format!("- **Controllers**: {}\n\n", controllers.len()));
    
    // Files by type
    output.push_str("## File Types\n\n");
    output.push_str("| Extension | Count | Lines of Code |\n");
    output.push_str("|-----------|-------|---------------|\n");
    
    for (ext, count) in &file_stats.by_extension {
        let loc = loc_stats.by_extension.get(ext).unwrap_or(&0);
        output.push_str(&format!("| {} | {} | {} |\n", ext.as_deref().unwrap_or("(no extension)"), count, loc.to_formatted_string()));
    }
    output.push_str("\n");
    
    // Models
    output.push_str("## Models\n\n");
    if !models.is_empty() {
        output.push_str("| Model | File | Fields | Associations |\n");
        output.push_str("|-------|------|--------|-------------|\n");
        
        for model in &models {
            let associations = if model.associations.is_empty() {
                "none".to_string()
            } else {
                model.associations.join(", ")
            };
            output.push_str(&format!("| {} | {} | {} | {} |\n", model.name, model.file, model.field_count, associations));
        }
    } else {
        output.push_str("No models found in the analyzed codebase.\n");
    }
    output.push_str("\n");
    
    // Controllers
    output.push_str("## Controllers\n\n");
    if !controllers.is_empty() {
        output.push_str("| Controller | File | Actions | Routes |\n");
        output.push_str("|------------|------|---------|--------|\n");
        
        for controller in &controllers {
            let routes = if controller.routes.is_empty() {
                "none".to_string()
            } else {
                controller.routes.join(", ")
            };
            output.push_str(&format!("| {} | {} | {} | {} |\n", controller.name, controller.file, controller.actions.len(), routes));
        }
    } else {
        output.push_str("No controllers found in the analyzed codebase.\n");
    }
    
    Ok(output)
}

/// Count files by type
fn count_files_by_type(dir: &Path) -> Result<FileStats, Box<dyn std::error::Error>> {
    let mut total = 0;
    let mut by_extension: HashMap<Option<String>, usize> = HashMap::new();
    
    for entry in walkdir::WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        total += 1;
        let path = entry.path();
        let ext = path.extension().map(|e| e.to_string_lossy().to_lowercase());
        
        *by_extension.entry(ext.map(|s| s.to_string())).or_insert(0) += 1;
    }
    
    Ok(FileStats {
        total,
        by_extension,
    })
}

/// Count lines of code
fn count_lines_of_code(dir: &Path) -> Result<LocStats, Box<dyn std::error::Error>> {
    let mut total = 0;
    let mut by_extension: HashMap<Option<String>, usize> = HashMap::new();
    
    for entry in walkdir::WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().map(|e| e.to_string_lossy().to_lowercase());
        
        // Skip binary files
        if let Some(ref ext_str) = ext {
            if ["jpg", "jpeg", "png", "gif", "pdf", "zip", "jar"].contains(&ext_str.as_str()) {
                continue;
            }
        }
        
        match fs::read_to_string(path) {
            Ok(content) => {
                let line_count = content.lines().count();
                total += line_count;
                *by_extension.entry(ext.map(|s| s.to_string())).or_insert(0) += line_count;
            }
            Err(_) => {
                // Skip files that can't be read
            }
        }
    }
    
    Ok(LocStats {
        total,
        by_extension,
    })
}

/// Find models in the source code
fn find_models(dir: &Path, system: &str) -> Result<Vec<ModelInfo>, Box<dyn std::error::Error>> {
    let mut models = Vec::new();
    let model_dirs = if system == "canvas" {
        vec!["app/models"]
    } else if system == "discourse" {
        vec!["app/models"]
    } else {
        vec![]
    };
    
    for model_dir in model_dirs {
        let full_model_dir = dir.join(model_dir);
        if !full_model_dir.exists() {
            continue;
        }
        
        for entry in fs::read_dir(full_model_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "rb") {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        // Simple model name extraction
                        let model_name = path.file_stem()
                            .map(|name| name.to_string_lossy().to_string())
                            .unwrap_or_default()
                            .replace("_", " ")
                            .split_whitespace()
                            .map(|word| {
                                let mut chars = word.chars();
                                match chars.next() {
                                    None => String::new(),
                                    Some(c) => c.to_uppercase().chain(chars).collect(),
                                }
                            })
                            .collect::<String>();
                        
                        // Field count estimation - count attr_ usage
                        let field_count = content.matches("attr_").count();
                        
                        // Association estimation
                        let mut associations = Vec::new();
                        for cap in regex::Regex::new(r"(?:has_many|has_one|belongs_to)\s+:([a-z_]+)")?.captures_iter(&content) {
                            if let Some(m) = cap.get(1) {
                                associations.push(m.as_str().to_string());
                            }
                        }
                        
                        models.push(ModelInfo {
                            name: model_name,
                            file: path.strip_prefix(dir)?.to_string_lossy().to_string(),
                            field_count,
                            associations,
                        });
                    }
                    Err(_) => {
                        // Skip files that can't be read
                    }
                }
            }
        }
    }
    
    Ok(models)
}

/// Find controllers in the source code
fn find_controllers(dir: &Path, system: &str) -> Result<Vec<ControllerInfo>, Box<dyn std::error::Error>> {
    let mut controllers = Vec::new();
    let controller_dirs = if system == "canvas" {
        vec!["app/controllers"]
    } else if system == "discourse" {
        vec!["app/controllers"]
    } else {
        vec![]
    };
    
    for controller_dir in controller_dirs {
        let full_controller_dir = dir.join(controller_dir);
        if !full_controller_dir.exists() {
            continue;
        }
        
        walk_controller_dir(&full_controller_dir, dir, &mut controllers)?;
    }
    
    Ok(controllers)
}

/// Recursively walk controller directory and find controllers
fn walk_controller_dir(current_dir: &Path, base_dir: &Path, controllers: &mut Vec<ControllerInfo>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            walk_controller_dir(&path, base_dir, controllers)?;
        } else if path.extension().map_or(false, |ext| ext == "rb") {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    // Simple controller name extraction
                    let controller_name = path.file_stem()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or_default()
                        .replace("_controller", "")
                        .replace("_", " ")
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(c) => c.to_uppercase().chain(chars).collect(),
                            }
                        })
                        .collect::<String>() + "Controller";
                    
                    // Action extraction - find def methods
                    let mut actions = Vec::new();
                    for cap in regex::Regex::new(r"def\s+([a-z_]+)")?.captures_iter(&content) {
                        if let Some(m) = cap.get(1) {
                            actions.push(m.as_str().to_string());
                        }
                    }
                    
                    // Route estimation - simple guess based on controller name and actions
                    let base_path = "/".to_string() + &controller_name.replace("Controller", "").to_lowercase();
                    let routes = actions.iter()
                        .filter(|action| ["index", "new", "create", "show", "edit", "update", "destroy"].contains(&action.as_str()))
                        .map(|action| format!("{}/{}", base_path, action))
                        .collect();
                    
                    controllers.push(ControllerInfo {
                        name: controller_name,
                        file: path.strip_prefix(base_dir)?.to_string_lossy().to_string(),
                        actions,
                        routes,
                    });
                }
                Err(_) => {
                    // Skip files that can't be read
                }
            }
        }
    }
    
    Ok(())
}

/// Generate a consolidated master report
fn generate_consolidated_report(
    summary_dir: &Path,
    lms_dir: &Path,
    canvas_dir: &Path,
    discourse_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let central_hub_path = lms_dir.join("docs").join("central_reference_hub.md");
    let canvas_analysis_path = summary_dir.join("canvas_analysis.md");
    let discourse_analysis_path = summary_dir.join("discourse_analysis.md");
    
    let central_hub_content = if central_hub_path.exists() {
        fs::read_to_string(&central_hub_path)?
    } else {
        String::new()
    };
    
    let canvas_analysis = if canvas_analysis_path.exists() {
        fs::read_to_string(&canvas_analysis_path)?
    } else {
        String::new()
    };
    
    let discourse_analysis = if discourse_analysis_path.exists() {
        fs::read_to_string(&discourse_analysis_path)?
    } else {
        String::new()
    };
    
    // Create master report
    let mut master_report = String::from("# Full Project Analysis Report\n\n");
    master_report.push_str(&format!("_Generated on {}_\n\n", chrono::Local::now().date_naive()));
    
    // Overview of all systems
    master_report.push_str("## Systems Overview\n\n");
    master_report.push_str("| System | Directory | Status |\n");
    master_report.push_str("|--------|-----------|--------|\n");
    master_report.push_str(&format!("| Target LMS | {} | ✅ Analyzed |\n", lms_dir.display()));
    master_report.push_str(&format!("| Canvas Source | {} | {} |\n", 
        canvas_dir.display(), 
        if !canvas_analysis.is_empty() { "✅ Analyzed" } else { "❌ Not Found" }
    ));
    master_report.push_str(&format!("| Discourse Source | {} | {} |\n\n", 
        discourse_dir.display(),
        if !discourse_analysis.is_empty() { "✅ Analyzed" } else { "❌ Not Found" }
    ));
    
    // Add section pointing to Central Reference Hub
    if !central_hub_content.is_empty() {
        master_report.push_str("## 🔍 Central Reference Hub\n\n");
        master_report.push_str("The Central Reference Hub contains comprehensive information about the integration project.\n");
        master_report.push_str(&format!("It is available at: `{}`\n\n", central_hub_path.display()));
        
        // Add key metrics from the hub - extract JSON metrics
        if let Some(metrics_match) = regex::Regex::new(r"```json\n([\s\S]*?)\n```")?.find(&central_hub_content) {
            let metrics_text = &central_hub_content[metrics_match.start()..metrics_match.end()];
            // Just extract the project_stats section for display
            if let Some(stats_match) = regex::Regex::new(r#""project_stats": (\{[^}]+\})"#)?.find(metrics_text) {
                master_report.push_str("### Key Metrics\n\n");
                master_report.push_str(&format!("```json\n{}\n```\n\n", &metrics_text[stats_match.start()..stats_match.end()]));
            }
        }
        
        // Add link to view full hub
        master_report.push_str("For complete integration details and project status, please refer to the Central Reference Hub.\n\n");
    }
    
    // Add source system analysis summaries
    if !canvas_analysis.is_empty() {
        master_report.push_str("## 📊 Canvas Source System\n\n");
        if let Some(overview_match) = regex::Regex::new(r"## Overview\n\n([\s\S]*?)\n\n")?.find(&canvas_analysis) {
            master_report.push_str(&canvas_analysis[overview_match.start()..overview_match.end()]);
            master_report.push_str("\n");
        }
        master_report.push_str(&format!("[View full Canvas analysis]({})\n\n", 
            canvas_analysis_path.strip_prefix(summary_dir)?.display()));
    }
    
    if !discourse_analysis.is_empty() {
        master_report.push_str("## 📊 Discourse Source System\n\n");
        if let Some(overview_match) = regex::Regex::new(r"## Overview\n\n([\s\S]*?)\n\n")?.find(&discourse_analysis) {
            master_report.push_str(&discourse_analysis[overview_match.start()..overview_match.end()]);
            master_report.push_str("\n");
        }
        master_report.push_str(&format!("[View full Discourse analysis]({})\n\n", 
            discourse_analysis_path.strip_prefix(summary_dir)?.display()));
    }
    
    // Add completion timeline
    master_report.push_str("## 📈 Completion Timeline\n\n");
    // Extract timeline from central hub
    if let Some(timeline_match) = regex::Regex::new(r"## 📈 Project Trajectories\n\n([\s\S]*?)\n\n")?.find(&central_hub_content) {
        master_report.push_str(&central_hub_content[timeline_match.start()..timeline_match.end()]);
        master_report.push_str("\n");
    } else {
        master_report.push_str("Timeline information not available. Please refer to the Central Reference Hub.\n\n");
    }
    
    // Next steps
    master_report.push_str("## Next Steps\n\n");
    master_report.push_str("1. Review the Central Reference Hub for detailed implementation status\n");
    master_report.push_str("2. Check source system analyses for legacy code patterns that need migration\n");
    master_report.push_str("3. Follow the implementation tasks outlined in the Central Reference Hub\n");
    master_report.push_str("4. Run this comprehensive analysis regularly to track progress\n\n");
    
    // Write master report
    fs::write(summary_dir.join("master_report.md"), master_report)?;
    
    Ok(())
}

/// File statistics
#[derive(Debug)]
struct FileStats {
    total: usize,
    by_extension: HashMap<Option<String>, usize>,
}

/// Lines of code statistics
#[derive(Debug)]
struct LocStats {
    total: usize,
    by_extension: HashMap<Option<String>, usize>,
}

/// Model information
#[derive(Debug)]
struct ModelInfo {
    name: String,
    file: String,
    field_count: usize,
    associations: Vec<String>,
}

/// Controller information
#[derive(Debug)]
struct ControllerInfo {
    name: String,
    file: String,
    actions: Vec<String>,
    routes: Vec<String>,
}

/// Format number with commas
trait NumberFormat {
    fn to_formatted_string(&self) -> String;
}

impl NumberFormat for usize {
    fn to_formatted_string(&self) -> String {
        let mut result = String::new();
        let digits = self.to_string();
        
        for (i, digit) in digits.chars().enumerate() {
            if i > 0 && (digits.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(digit);
        }
        
        result
    }
}
