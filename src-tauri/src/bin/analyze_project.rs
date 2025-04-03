use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use std::error::Error;

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
    if !src_path.exists() {
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Analyzing project structure...");
    match analyze_project() {
        Ok(analysis) => {
            println!("\n# Project Analysis Results");
            
            println!("\n## Missing Components");
            if analysis.missing_components.is_empty() {
                println!("No missing components found.");
            } else {
                println!("These components are used in routes but not defined:");
                for comp in &analysis.missing_components {
                    println!("- {}", comp);
                }
            }
            
            println!("\n## Name Mismatches");
            if analysis.name_mismatches.is_empty() {
                println!("No name mismatches detected.");
            } else {
                println!("These components might be the same but named differently:");
                for (route_comp, defined_comp) in &analysis.name_mismatches {
                    println!("- Route uses '{}' but found '{}'", route_comp, defined_comp);
                }
            }
            
            println!("\n## Suggested mod.rs Updates");
            if !analysis.missing_components.is_empty() {
                println!("```rust");
                println!("// filepath: c:\\Users\\Tim\\Desktop\\LMS\\src\\components\\mod.rs");
                println!("// Update your forum component exports:");
                println!("pub use forum::{{");
                
                // Check for known mappings
                let mut known_mappings = HashMap::new();
                known_mappings.insert("CategoriesList", "ForumCategories");
                known_mappings.insert("TopicsList", "ForumThreads");
                known_mappings.insert("TopicDetail", "ThreadDetail");
                
                for missing in &analysis.missing_components {
                    if let Some(existing) = known_mappings.get(missing.as_str()) {
                        println!("    {} as {},", existing, missing);
                    } else if let Some(similar) = analysis.name_mismatches.get(missing) {
                        println!("    {} as {}, // Suggested mapping", similar, missing);
                    } else {
                        println!("    // Missing: {} - needs implementation", missing);
                    }
                }
                
                println!("}};");
                println!("```");
            }
            
            println!("\n## All Components Used in Routes");
            for comp in &analysis.route_components {
                println!("- {}", comp);
            }
            
            println!("\n## All Defined Components");
            for comp in &analysis.defined_components {
                println!("- {}", comp);
            }
            
            Ok(())
        },
        Err(e) => {
            eprintln!("Error analyzing project: {}", e);
            Err(e.into())
        }
    }
}