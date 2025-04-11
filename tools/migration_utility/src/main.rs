use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(name = "migration-utility", about = "Utility for managing JavaScript to Rust migration")]
struct Opt {
    /// Command to run (discover, migrate, or clean)
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Discover remaining JavaScript files
    Discover {
        /// Path to search
        #[structopt(short, long, default_value = ".")]
        path: String,
        
        /// Exclude pattern (comma-separated)
        #[structopt(short, long, default_value = "node_modules,coverage,dist,build")]
        exclude: String,
    },
    
    /// Create Rust equivalents for JavaScript files
    Migrate {
        /// JavaScript file to migrate
        #[structopt(short, long)]
        file: String,
        
        /// Target Rust file
        #[structopt(short, long)]
        target: String,
    },
    
    /// Clean up migrated JavaScript files
    Clean {
        /// Path to migration tracking document
        #[structopt(short, long, default_value = "JavaScript to Rust Migration Tracking.md")]
        tracking_doc: String,
        
        /// Dry run (don't actually delete files)
        #[structopt(short, long)]
        dry_run: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    
    match opt.cmd {
        Command::Discover { path, exclude } => {
            discover_js_files(&path, &exclude)?;
        },
        Command::Migrate { file, target } => {
            migrate_js_file(&file, &target)?;
        },
        Command::Clean { tracking_doc, dry_run } => {
            clean_migrated_files(&tracking_doc, dry_run)?;
        },
    }
    
    Ok(())
}

fn discover_js_files(path: &str, exclude: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Discovering JavaScript files in {}...", path);
    
    let exclude_patterns: Vec<&str> = exclude.split(',').collect();
    let mut js_files = Vec::new();
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Skip directories and non-JavaScript files
        if path.is_dir() || path.extension().map_or(true, |ext| ext != "js") {
            continue;
        }
        
        // Check if path contains any exclude pattern
        if exclude_patterns.iter().any(|pat| path.to_string_lossy().contains(pat)) {
            continue;
        }
        
        // Convert to relative path
        let relative_path = match path.strip_prefix(path) {
            Ok(rel_path) => rel_path,
            Err(_) => path,
        };
        
        let path_str = relative_path.to_string_lossy().replace("\\", "/");
        js_files.push(path_str.to_string());
    }
    
    println!("Found {} JavaScript files:", js_files.len());
    for file in &js_files {
        println!("  - {}", file);
    }
    
    // Save to file for later use
    fs::write("remaining_js_files.txt", js_files.join("\n"))?;
    println!("Saved list to remaining_js_files.txt");
    
    Ok(())
}

fn migrate_js_file(js_file: &str, rust_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Migrating {} to {}...", js_file, rust_file);
    
    // Check if JS file exists
    if !Path::new(js_file).exists() {
        return Err(format!("JavaScript file does not exist: {}", js_file).into());
    }
    
    // Create directory structure for Rust file if needed
    if let Some(parent) = Path::new(rust_file).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Read JS file content
    let js_content = fs::read_to_string(js_file)?;
    
    // Here we'd ideally use an AI-assisted tool or a transpiler to convert JS to Rust
    // For now, we'll create a basic Rust file template
    let rust_content = format!(
        "//! Rust implementation of {}\n//!\n//! This file was migrated from JavaScript to Rust\n\n// TODO: Implement this module\n",
        js_file
    );
    
    // Write Rust file
    fs::write(rust_file, rust_content)?;
    println!("Created Rust file template at {}", rust_file);
    
    // Update tracking document
    println!("Remember to update the migration tracking document!");
    
    Ok(())
}

fn clean_migrated_files(tracking_doc: &str, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Cleaning up migrated JavaScript files...");
    
    // Read tracking document
    let content = fs::read_to_string(tracking_doc)?;
    
    // Extract completed migrations
    let mut completed_js_files = Vec::new();
    for line in content.lines() {
        if line.contains("[x]") && line.contains(".js") {
            if let Some(start) = line.find("]") {
                if let Some(end) = line.find("|") {
                    let js_file = line[start+1..end].trim();
                    completed_js_files.push(js_file.to_string());
                }
            }
        }
    }
    
    println!("Found {} completed migrations:", completed_js_files.len());
    for file in &completed_js_files {
        let file_path = Path::new(&file);
        if file_path.exists() {
            println!("  - {} (exists)", file);
            
            if !dry_run {
                // Create backup before deletion
                let backup_path = format!("{}.bak", file);
                fs::copy(file, &backup_path)?;
                println!("    Created backup: {}", backup_path);
                
                // Delete the original file
                fs::remove_file(file)?;
                println!("    Deleted original file");
            } else {
                println!("    (dry run, not deleting)");
            }
        } else {
            println!("  - {} (not found)", file);
        }
    }
    
    Ok(())
}