use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;
use std::error::Error;
use lms::utils::file_system::FileSystemUtils;
use lms::analyzers::project_structure::ProjectStructure;
use lms::analyzers::ast_analyzer::{AstAnalyzer, CodeMetrics};
use lms::ai::gemini_analyzer::{GeminiAnalyzer, CodeInsight};
use tokio;

#[derive(Debug)]
struct ProjectAnalysis {
    route_components: Vec<String>,
    defined_components: Vec<String>,
    missing_components: Vec<String>,
    name_mismatches: HashMap<String, String>,
}

fn analyze_project() -> Result<ProjectAnalysis, String> {
    let mut route_components = HashSet::new();
    let mut defined_components = HashSet::new();
    let mut name_mismatches = HashMap::new();
    
    // Process all Rust files in src directory
    let src_path = Path::new("src");
    if (!src_path.exists()) {
        return Err("src directory not found. Make sure you're running from project root".to_string());
    }
    
    for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(path)
                .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
            
            // Check for route definitions
            if content.contains("route!") || content.contains("Route::new") {
                extract_route_components(&content, &mut route_components);
            }
            
            // Check for component definitions
            extract_defined_components(&content, &mut defined_components);
        }
    }
    
    // Find potential name mismatches
    find_potential_mismatches(&route_components, &defined_components, &mut name_mismatches);
    
    // Calculate missing components
    let missing_components: HashSet<_> = route_components
        .difference(&defined_components)
        .cloned()
        .collect();
    
    Ok(ProjectAnalysis {
        route_components: route_components.into_iter().collect(),
        defined_components: defined_components.into_iter().collect(),
        missing_components: missing_components.into_iter().collect(),
        name_mismatches,
    })
}

fn extract_route_components(content: &str, components: &mut HashSet<String>) {
    // Find components used in routes
    // Common patterns in Leptos route definitions
    let patterns = [
        "view=|", 
        "view:", 
        "<", 
        "component="
    ];
    
    for line in content.lines() {
        let line = line.trim();
        
        for pattern in &patterns {
            if let Some(start_pos) = line.find(pattern) {
                let start_pos = start_pos + pattern.len();
                let mut component_name = String::new();
                
                for c in line[start_pos..].chars() {
                    if c.is_alphanumeric() || c == '_' {
                        component_name.push(c);
                    } else {
                        break;
                    }
                }
                
                if !component_name.is_empty() && component_name.chars().next().unwrap().is_uppercase() {
                    components.insert(component_name);
                }
            }
        }
    }
}

fn extract_defined_components(content: &str, components: &mut HashSet<String>) {
    // Look for component definitions
    let patterns = [
        "#[component]", 
        "struct", 
        "pub struct", 
        "pub fn"
    ];
    
    for line in content.lines() {
        let line = line.trim();
        
        for pattern in &patterns {
            if line.starts_with(pattern) {
                let remaining = &line[pattern.len()..].trim();
                if let Some(component_name) = remaining.split(&[' ', '(', '<', '{'][..])
                    .next()
                    .filter(|s| !s.is_empty() && s.chars().next().unwrap().is_uppercase())
                {
                    components.insert(component_name.to_string());
                }
            }
        }
    }
    
    // Also check exports in mod.rs files
    if content.contains("pub use") {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("pub use") {
                // Extract component names from pub use statements
                if line.contains(" as ") {
                    // Handle aliased imports like "ForumCategories as CategoriesList"
                    if let Some(as_index) = line.find(" as ") {
                        let after_as = &line[as_index + 4..];
                        if let Some(end_index) = after_as.find(&[',', ';', '}'][..]) {
                            let component_name = &after_as[..end_index];
                            if !component_name.is_empty() && component_name.chars().next().unwrap().is_uppercase() {
                                components.insert(component_name.to_string());
                            }
                        }
                    }
                } else {
                    // Handle direct imports
                    let parts: Vec<&str> = line.split(&['{', '}', ',', ';'][..]).collect();
                    for part in parts {
                        let part = part.trim();
                        if !part.is_empty() && !part.contains("::") && !part.contains("pub use") && 
                           part.chars().next().map_or(false, |c| c.is_uppercase()) {
                            components.insert(part.to_string());
                        }
                    }
                }
            }
        }
    }
}

fn find_potential_mismatches(
    route_components: &HashSet<String>,
    defined_components: &HashSet<String>,
    mismatches: &mut HashMap<String, String>
) {
    for route_comp in route_components {
        if !defined_components.contains(route_comp) {
            // Look for components with similar names
            for defined_comp in defined_components {
                if is_similar_name(route_comp, defined_comp) {
                    mismatches.insert(route_comp.clone(), defined_comp.clone());
                    break;
                }
            }
        }
    }
}

fn is_similar_name(name1: &str, name2: &str) -> bool {
    // Check for common variations (like Categories vs ForumCategories)
    if name2.contains(name1) || name1.contains(name2) {
        return true;
    }
    
    // Check for singular/plural variations
    if name1.ends_with('s') && name1[..name1.len()-1] == *name2 {
        return true;
    }
    if name2.ends_with('s') && name2[..name2.len()-1] == *name1 {
        return true;
    }
    
    false
}

fn generate_analysis_report(metrics: &CodeMetrics, fs_utils: &FileSystemUtils) -> String {
    let mut report = String::from("# Project Analysis Report\n\n");
    
    // Basic statistics
    report.push_str("## Overview\n\n");
    report.push_str(&format!("- **Files Analyzed**: {}\n", metrics.file_count));
    report.push_str(&format!("- **Total Complexity**: {}\n", metrics.total_complexity));
    report.push_str(&format!("- **Average Complexity**: {:.2}\n", metrics.average_complexity));
    report.push_str(&format!("- **Components Detected**: {}\n", metrics.components.len()));
    
    // List most complex files
    report.push_str("\n## Most Complex Files\n\n");
    let mut complexity_scores: Vec<(PathBuf, &u32)> = metrics.file_complexity.iter().collect();
    complexity_scores.sort_by(|a, b| b.1.cmp(a.1));
    
    for (i, (path, complexity)) in complexity_scores.iter().take(10).enumerate() {
        report.push_str(&format!("{}. **{}** - Complexity: {}\n", 
            i + 1, 
            path.file_name().unwrap_or_default().to_string_lossy(),
            complexity
        ));
    }
    
    // Component summary
    if !metrics.components.is_empty() {
        report.push_str("\n## Component Summary\n\n");
        
        let mut components_by_type: HashMap<String, Vec<&crate::analyzers::ast_analyzer::ComponentInfo>> = HashMap::new();
        
        for component in &metrics.components {
            components_by_type
                .entry(component.type_name.clone())
                .or_insert_with(Vec::new)
                .push(component);
        }
        
        for (type_name, components) in &components_by_type {
            report.push_str(&format!("### {} Components ({})\n\n", type_name, components.len()));
            
            for component in components {
                report.push_str(&format!("- **{}** - Complexity: {}, LOC: {}\n", 
                    component.name,
                    component.complexity,
                    component.lines_of_code
                ));
            }
            
            report.push_str("\n");
        }
    }
    
    // Project structure
    report.push_str("\n## Project Structure\n\n");
    
    let structure = fs_utils.get_project_structure();
    
    // Add section for API directories
    let api_dirs = structure.get_directories_by_category("api");
    if !api_dirs.is_empty() {
        report.push_str("### API Directories\n\n");
        for dir in api_dirs {
            report.push_str(&format!("- {}\n", dir));
        }
        report.push_str("\n");
    }
    
    // Add section for Model directories
    let model_dirs = structure.get_directories_by_category("models");
    if !model_dirs.is_empty() {
        report.push_str("### Model Directories\n\n");
        for dir in model_dirs {
            report.push_str(&format!("- {}\n", dir));
        }
        report.push_str("\n");
    }
    
    // Add section for UI directories
    let ui_dirs = structure.get_directories_by_category("ui");
    if !ui_dirs.is_empty() {
        report.push_str("### UI Directories\n\n");
        for dir in ui_dirs {
            report.push_str(&format!("- {}\n", dir));
        }
        report.push_str("\n");
    }
    
    report
}

fn generate_component_report(analysis: &ProjectAnalysis) -> String {
    let mut report = String::from("# Component Analysis Report\n\n");
    
    // Basic statistics
    report.push_str("## Overview\n\n");
    report.push_str(&format!("- **Defined Components**: {}\n", analysis.defined_components.len()));
    report.push_str(&format!("- **Route Components**: {}\n", analysis.route_components.len()));
    
    // Missing components
    if !analysis.missing_components.is_empty() {
        report.push_str("\n## Missing Components\n\n");
        report.push_str("These components are referenced in routes but not found in the codebase:\n\n");
        
        for (i, component) in analysis.missing_components.iter().enumerate() {
            report.push_str(&format!("{}. **{}**\n", i + 1, component));
        }
    }
    
    // Name mismatches
    if !analysis.name_mismatches.is_empty() {
        report.push_str("\n## Potential Name Mismatches\n\n");
        report.push_str("These components have similar names and might be the same component:\n\n");
        
        for (route_comp, defined_comp) in &analysis.name_mismatches {
            report.push_str(&format!("- Route uses **{}** but found **{}**\n", route_comp, defined_comp));
        }
    }
    
    // List all defined components
    report.push_str("\n## All Defined Components\n\n");
    for component in &analysis.defined_components {
        let is_used = analysis.route_components.contains(component);
        report.push_str(&format!("- **{}**{}\n", 
            component,
            if !is_used { " (unused in routes)" } else { "" }
        ));
    }
    
    report
}

fn generate_insights_report(insights: &HashMap<PathBuf, lms::ai::gemini_analyzer::CodeInsight>) -> String {
    let mut report = String::from("# AI Code Insights\n\n");
    
    if insights.is_empty() {
        report.push_str("No insights generated.\n");
        return report;
    }
    
    for (file_path, insight) in insights {
        report.push_str(&format!("## {}\n\n", file_path.display()));
        
        // Summary
        report.push_str("### Summary\n\n");
        report.push_str(&format!("{}\n\n", insight.summary));
        
        // Code quality score
        report.push_str(&format!("**Code Quality Score**: {}/100\n\n", insight.code_quality_score));
        
        // Recommendations
        if !insight.recommendations.is_empty() {
            report.push_str("### Recommendations\n\n");
            for recommendation in &insight.recommendations {
                report.push_str(&format!("- {}\n", recommendation));
            }
            report.push_str("\n");
        }
        
        // Improvement suggestions
        if !insight.improvement_suggestions.is_empty() {
            report.push_str("### Improvement Suggestions\n\n");
            for suggestion in &insight.improvement_suggestions {
                let priority_str = match suggestion.priority {
                    lms::ai::gemini_analyzer::Priority::Low => "Low",
                    lms::ai::gemini_analyzer::Priority::Medium => "Medium",
                    lms::ai::gemini_analyzer::Priority::High => "High",
                    lms::ai::gemini_analyzer::Priority::Critical => "Critical",
                };
                
                report.push_str(&format!("#### {}\n\n", suggestion.description));
                report.push_str(&format!("- **Priority**: {}\n", priority_str));
                report.push_str(&format!("- **Effort Estimate**: {}\n\n", suggestion.effort_estimate));
                
                if let Some(code_snippet) = &suggestion.code_snippet {
                    report.push_str("```rust\n");
                    report.push_str(code_snippet);
                    report.push_str("\n```\n\n");
                }
            }
        }
        
        report.push_str("---\n\n");
    }
    
    report
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    println!("Starting LMS Analysis Engine (Rust-based)...");
    
    // Parse command line arguments
    let args = std::env::args().collect::<Vec<String>>();
    let project_path = args.get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));
    
    // Check if we should generate AI insights
    let generate_ai = args.iter().any(|arg| arg == "--ai" || arg == "-a");
    
    // Get output directory
    let output_dir = args.iter()
        .position(|arg| arg == "--output" || arg == "-o")
        .and_then(|pos| args.get(pos + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| project_path.join("analysis_output"));
    
    println!("Analyzing project at: {:?}", project_path);
    println!("Outputs will be written to: {:?}", output_dir);
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;
    
    // Initialize FileSystemUtils with default exclude patterns
    let exclude_patterns = vec![
        r"node_modules", r"\.git", r"dist", r"build", r"target", r"\.vscode"
    ];
    let mut fs_utils = FileSystemUtils::new(project_path.clone(), exclude_patterns);
    
    // Discover files
    println!("Scanning file system...");
    let files = fs_utils.discover_files();
    println!("Found {} files", files.len());
    
    // Read file contents
    println!("Reading and analyzing file contents...");
    fs_utils.read_file_contents();
    
    // Get project structure
    let project_structure = fs_utils.get_project_structure();
    println!("Found {} directories", project_structure.directories.len());
    
    // Print file type stats
    let file_stats = fs_utils.get_file_stats();
    println!("File types: {} Rust, {} JS/TS, {} other", 
             file_stats.rust, file_stats.js, file_stats.other);
    
    // Run AST analysis
    println!("Performing code analysis...");
    let ast_analyzer = AstAnalyzer::new();
    let metrics = ast_analyzer.analyze_project_code(&fs_utils);
    
    // Generate basic analysis report
    println!("Generating analysis report...");
    let analysis_report = generate_analysis_report(&metrics, &fs_utils);
    
    // Write analysis report
    let report_path = output_dir.join("analysis_report.md");
    fs::write(&report_path, analysis_report)?;
    println!("Analysis report written to: {:?}", report_path);
    
    // Check for component analysis
    if let Ok(component_analysis) = analyze_project() {
        println!("Component analysis complete:");
        println!("  Found {} defined components", component_analysis.defined_components.len());
        println!("  Found {} route components", component_analysis.route_components.len());
        
        if !component_analysis.missing_components.is_empty() {
            println!("  Warning: {} missing components", component_analysis.missing_components.len());
        }
        
        if !component_analysis.name_mismatches.is_empty() {
            println!("  Found {} potential name mismatches", component_analysis.name_mismatches.len());
        }
        
        // Save component analysis
        let component_report = generate_component_report(&component_analysis);
        let component_path = output_dir.join("component_report.md");
        fs::write(&component_path, component_report)?;
        println!("Component report written to: {:?}", component_path);
    }
    
    // Generate AI insights if requested
    if generate_ai {
        println!("Generating AI code insights...");
        // Here we're using a mock API key - in a real implementation this would come from configuration
        let gemini_analyzer = GeminiAnalyzer::new("mock-api-key".to_string(), project_path.clone());
        
        let insights = gemini_analyzer.generate_code_insights(&fs_utils, &metrics).await?;
        println!("Generated insights for {} files", insights.len());
        
        // Generate project overview
        let overview = gemini_analyzer.generate_project_overview(&fs_utils, &metrics).await?;
        let overview_path = output_dir.join("project_overview.md");
        fs::write(&overview_path, overview)?;
        println!("Project overview written to: {:?}", overview_path);
        
        // Generate component analysis
        let component_analysis = gemini_analyzer.analyze_components(&metrics).await?;
        let component_analysis_path = output_dir.join("ai_component_analysis.md");
        fs::write(&component_analysis_path, component_analysis)?;
        println!("AI component analysis written to: {:?}", component_analysis_path);
        
        // Generate detailed insights report
        let insights_report = generate_insights_report(&insights);
        let insights_path = output_dir.join("code_insights.md");
        fs::write(&insights_path, insights_report)?;
        println!("AI code insights written to: {:?}", insights_path);
    }
    
    let elapsed = start_time.elapsed();
    println!("Analysis complete in {:.2?}!", elapsed);
    Ok(())
}