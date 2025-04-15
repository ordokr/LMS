#!/usr/bin/env python
"""
Integrate analyzers between the LMS and PORT projects.
This script will:
1. Find the Canvas and Discourse analyzers in the port project
2. Create Rust versions of these analyzers in the LMS project
3. Create an integrated migration analyzer that uses both
"""

import os
import sys
import shutil
import argparse
from pathlib import Path

# Import the canvas analyzer template function
from canvas_analyzer_template import create_canvas_analyzer_template

def create_directory_if_not_exists(path):
    """Create a directory if it does not exist"""
    if not os.path.exists(path):
        os.makedirs(path)
        print(f"Created directory: {path}")

def find_js_analyzer(port_dir, analyzer_type):
    """Search for a JavaScript analyzer in the port directory"""
    search_terms = []
    if analyzer_type.lower() == "canvas":
        search_terms = ["canvas-analyzer", "canvas_analyzer", "CanvasAnalyzer"]
    else:
        search_terms = ["discourse-analyzer", "discourse_analyzer", "DiscourseAnalyzer"]

    # Look for specific filenames first
    for term in search_terms:
        potential_path = os.path.join(port_dir, f"{term}.js")
        if os.path.exists(potential_path):
            return potential_path

    # First look for filenames
    for root, _, files in os.walk(port_dir):
        for file in files:
            if file.endswith(".js"):
                for term in search_terms:
                    if term.lower() in file.lower():
                        return os.path.join(root, file)

    # Then look in file contents
    for root, _, files in os.walk(port_dir):
        for file in files:
            if file.endswith(".js"):
                filepath = os.path.join(root, file)
                try:
                    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                        for term in search_terms:
                            if term in content and "class" in content and "analyze" in content:
                                return filepath
                except:
                    pass

    return None

def create_discourse_analyzer_template(canvas_js_path, discourse_rust_path):
    """Create a Discourse analyzer template based on the Canvas analyzer"""
    if not canvas_js_path or not os.path.exists(canvas_js_path):
        print("Error: Canvas analyzer path not found. Cannot create template.")
        return False

    content = """use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use regex::Regex;

// Model information extracted from forum code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseModel {
    pub name: String,
    pub file_name: String,
    pub file_path: String,
    pub fields: Vec<String>,
    pub associations: Vec<String>,
    pub line_count: usize,
}

// Discourse analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseAnalysisResult {
    pub models: Vec<DiscourseModel>,
    pub file_stats: FileStats,
}

// File statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub total: usize,
    pub by_extension: HashMap<String, usize>,
    pub lines_by_extension: HashMap<String, usize>,
}

impl Default for DiscourseAnalysisResult {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            file_stats: FileStats {
                total: 0,
                by_extension: HashMap::new(),
                lines_by_extension: HashMap::new(),
            },
        }
    }
}

// Discourse analyzer
pub struct DiscourseAnalyzer {
    pub base_dir: PathBuf,
    pub result: DiscourseAnalysisResult,
}

impl DiscourseAnalyzer {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            result: DiscourseAnalysisResult::default(),
        }
    }

    // Main analysis function
    pub fn analyze(&mut self) -> Result<DiscourseAnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing Discourse forum at {:?}...", self.base_dir);

        // This is a template implementation
        println!("WARNING: Using discourse analyzer template");
        self.result.file_stats.total = 800; // Placeholder

        Ok(self.result.clone())
    }
}
"""

    # Create the parent directory if it doesn't exist
    parent_dir = os.path.dirname(discourse_rust_path)
    os.makedirs(parent_dir, exist_ok=True)

    # Write the content to the file
    with open(discourse_rust_path, 'w', encoding='utf-8') as f:
        f.write(content)

    return True

def create_integrated_migration_analyzer(output_path):
    """Create the integrated migration analyzer template"""
    content = """use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::utils::FileSystemUtils;
use crate::analyzers::canvas_analyzer::CanvasAnalyzer;
use crate::analyzers::discourse_analyzer::DiscourseAnalyzer;

// Integrated migration analysis result
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegratedMigrationResult {
    pub canvas_models: Vec<String>,
    pub discourse_models: Vec<String>,
    pub common_entities: Vec<String>,
    pub migration_paths: Vec<MigrationPath>,
    pub integration_points: Vec<IntegrationPoint>,
}

// Migration path between LMS and forum entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPath {
    pub source_entity: String,
    pub target_entity: String,
    pub complexity: String,
    pub mapping_strategy: String,
}

// Points of integration between systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPoint {
    pub name: String,
    pub canvas_component: String,
    pub discourse_component: String,
    pub data_flow: String,
    pub sync_pattern: String,
}

impl Default for MigrationPath {
    fn default() -> Self {
        Self {
            source_entity: String::new(),
            target_entity: String::new(),
            complexity: "medium".to_string(),
            mapping_strategy: "direct".to_string(),
        }
    }
}

impl Default for IntegrationPoint {
    fn default() -> Self {
        Self {
            name: String::new(),
            canvas_component: String::new(),
            discourse_component: String::new(),
            data_flow: "bidirectional".to_string(),
            sync_pattern: "event-based".to_string(),
        }
    }
}

// Integrated migration analyzer
pub struct IntegratedMigrationAnalyzer {
    pub lms_dir: PathBuf,
    pub canvas_dir: Option<PathBuf>,
    pub discourse_dir: Option<PathBuf>,
    pub fs_utils: Arc<FileSystemUtils>,
    pub result: IntegratedMigrationResult,
}

impl IntegratedMigrationAnalyzer {
    pub fn new(lms_dir: impl Into<PathBuf>, fs_utils: Arc<FileSystemUtils>) -> Self {
        Self {
            lms_dir: lms_dir.into(),
            canvas_dir: None,
            discourse_dir: None,
            fs_utils,
            result: IntegratedMigrationResult::default(),
        }
    }

    // Set Canvas directory
    pub fn with_canvas_dir(&mut self, dir: impl Into<PathBuf>) -> &mut Self {
        self.canvas_dir = Some(dir.into());
        self
    }

    // Set Discourse directory
    pub fn with_discourse_dir(&mut self, dir: impl Into<PathBuf>) -> &mut Self {
        self.discourse_dir = Some(dir.into());
        self
    }

    // Main analysis function
    pub async fn analyze(&mut self) -> Result<IntegratedMigrationResult, Box<dyn std::error::Error + Send + Sync>> {
        println!("Starting integrated migration analysis...");

        // Analyze Canvas if directory is provided
        if let Some(canvas_dir) = &self.canvas_dir {
            if self.fs_utils.exists(canvas_dir) {
                println!("Analyzing Canvas LMS at {:?}...", canvas_dir);
                let mut canvas_analyzer = CanvasAnalyzer::new(canvas_dir);

                match canvas_analyzer.analyze() {
                    Ok(canvas_result) => {
                        println!("Canvas analysis complete. Found {} models.", canvas_result.models.len());

                        // Extract model names for the integrated result
                        self.result.canvas_models = canvas_result.models
                            .iter()
                            .map(|model| model.name.clone())
                            .collect();
                    },
                    Err(e) => {
                        eprintln!("Error analyzing Canvas: {}", e);
                    }
                }
            } else {
                eprintln!("Canvas directory does not exist: {:?}", canvas_dir);
            }
        }

        // Analyze Discourse if directory is provided
        if let Some(discourse_dir) = &self.discourse_dir {
            if self.fs_utils.exists(discourse_dir) {
                println!("Analyzing Discourse forum at {:?}...", discourse_dir);
                let mut discourse_analyzer = DiscourseAnalyzer::new(discourse_dir);

                match discourse_analyzer.analyze() {
                    Ok(discourse_result) => {
                        println!("Discourse analysis complete. Found {} models.", discourse_result.models.len());

                        // Extract model names for the integrated result
                        self.result.discourse_models = discourse_result.models
                            .iter()
                            .map(|model| model.name.clone())
                            .collect();
                    },
                    Err(e) => {
                        eprintln!("Error analyzing Discourse: {}", e);
                    }
                }
            } else {
                eprintln!("Discourse directory does not exist: {:?}", discourse_dir);
            }
        }

        // Find common entities
        self.identify_common_entities();

        // Generate migration paths
        self.generate_migration_paths();

        // Identify integration points
        self.identify_integration_points();

        println!("Integration analysis complete!");
        println!("Identified {} common entities", self.result.common_entities.len());
        println!("Generated {} migration paths", self.result.migration_paths.len());
        println!("Found {} integration points", self.result.integration_points.len());

        // Generate report
        self.generate_report();

        Ok(self.result.clone())
    }

    // Identify common entities between Canvas and Discourse
    fn identify_common_entities(&mut self) {
        // Placeholder implementation - in a real analyzer, this would have more complex logic
        let canvas_lower: Vec<String> = self.result.canvas_models
            .iter()
            .map(|m| m.to_lowercase())
            .collect();

        let discourse_lower: Vec<String> = self.result.discourse_models
            .iter()
            .map(|m| m.to_lowercase())
            .collect();

        // Find potential matches based on common entity names
        let common_entities = vec![
            "User", "Post", "Category", "Topic", "Group", "Notification"
        ];

        for entity in common_entities {
            let entity_lower = entity.to_lowercase();

            if (canvas_lower.contains(&entity_lower) || canvas_lower.contains(&format!("{}s", entity_lower))) &&
               (discourse_lower.contains(&entity_lower) || discourse_lower.contains(&format!("{}s", entity_lower))) {
                self.result.common_entities.push(entity.to_string());
            }
        }
    }

    // Generate migration paths between entities
    fn generate_migration_paths(&mut self) {
        // In a real implementation, this would have more complex logic
        for entity in &self.result.common_entities {
            let path = MigrationPath {
                source_entity: format!("Canvas{}", entity),
                target_entity: format!("Discourse{}", entity),
                complexity: "medium".to_string(),
                mapping_strategy: "direct-mapping".to_string(),
            };

            self.result.migration_paths.push(path);
        }
    }

    // Identify integration points
    fn identify_integration_points(&mut self) {
        // In a real implementation, this would have more complex logic

        // Common integration points in LMS to forum integrations
        let integration_points = vec![
            ("Authentication", "User management", "Users API", "bidirectional", "real-time"),
            ("Content sharing", "Course content", "Post embedding", "canvas-to-discourse", "scheduled"),
            ("Discussion", "Discussions", "Topics API", "bidirectional", "event-based"),
            ("Notifications", "Alerts", "Notification system", "bidirectional", "event-based"),
            ("Groups", "Course groups", "Group system", "canvas-to-discourse", "manual"),
        ];

        for (name, canvas_comp, discourse_comp, flow, pattern) in integration_points {
            let point = IntegrationPoint {
                name: name.to_string(),
                canvas_component: canvas_comp.to_string(),
                discourse_component: discourse_comp.to_string(),
                data_flow: flow.to_string(),
                sync_pattern: pattern.to_string(),
            };

            self.result.integration_points.push(point);
        }
    }

    // Generate an integration report
    fn generate_report(&self) {
        // In a real implementation, this would generate a detailed report
        println!("Generating integration report (placeholder)...");
    }
}
"""

    # Create the parent directory if it doesn't exist
    parent_dir = os.path.dirname(output_path)
    os.makedirs(parent_dir, exist_ok=True)

    # Write the content to the file
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(content)

    return True

def update_modules_mod_file(mod_path):
    """Update the modules mod.rs file to include new analyzers"""
    if not os.path.exists(mod_path):
        # Create a new mod.rs file
        content = """// Canvas analyzer
pub mod canvas_analyzer;

// Discourse analyzer
pub mod discourse_analyzer;
"""
        with open(mod_path, 'w', encoding='utf-8') as f:
            f.write(content)
    else:
        # Read the existing content
        with open(mod_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Check if the analyzers are already included
        if 'pub mod canvas_analyzer;' not in content:
            content += "\n// Canvas analyzer\npub mod canvas_analyzer;\n"

        if 'pub mod discourse_analyzer;' not in content:
            content += "\n// Discourse analyzer\npub mod discourse_analyzer;\n"

        # Write the updated content
        with open(mod_path, 'w', encoding='utf-8') as f:
            f.write(content)

    return True

def update_analyzers_mod_file(mod_path):
    """Update the analyzers mod.rs file to include the integrated migration analyzer"""
    if not os.path.exists(mod_path):
        # Create a new mod.rs file
        content = """// Basic analyzer modules
pub mod unified_analyzer;

// Canvas and Discourse specific analyzers
pub mod modules {
    pub mod canvas_analyzer;
    pub mod discourse_analyzer;
}

// Reexport for easier access
pub use modules::canvas_analyzer::CanvasAnalyzer;
pub use modules::discourse_analyzer::DiscourseAnalyzer;

// Integrated migration analyzer
pub mod integrated_migration_analyzer;
pub use integrated_migration_analyzer::IntegratedMigrationAnalyzer;

// Common analysis result type
pub use unified_analyzer::AnalysisResult;
"""
        with open(mod_path, 'w', encoding='utf-8') as f:
            f.write(content)
    else:
        # Read the existing content
        with open(mod_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Check if the integrated migration analyzer is already included
        if 'pub mod integrated_migration_analyzer;' not in content:
            # Find the end of the module declarations
            lines = content.split('\n')

            # Add the integrated migration analyzer
            lines.append("\n// Integrated migration analyzer")
            lines.append("pub mod integrated_migration_analyzer;")
            lines.append("pub use integrated_migration_analyzer::IntegratedMigrationAnalyzer;")

            # Write the updated content
            with open(mod_path, 'w', encoding='utf-8') as f:
                f.write('\n'.join(lines))

    return True

def create_wrapper_scripts(lms_dir, analyzer_dir):
    """Create wrapper scripts for easy execution of the analyzer"""
    # Create the Windows batch script
    bat_path = os.path.join(lms_dir, "analyze-integrated.bat")
    bat_content = """@echo off
cd %~dp0
cd {0}
if not exist "target" (
    echo Building analyzer...
    cargo build --release
)
target\\release\\unified-analyzer --integrated %*
""".format(os.path.relpath(analyzer_dir, lms_dir))

    with open(bat_path, 'w', encoding='utf-8') as f:
        f.write(bat_content)

    # Create the Unix shell script
    sh_path = os.path.join(lms_dir, "analyze-integrated.sh")
    sh_content = """#!/bin/bash
cd "$(dirname "$0")"
cd {0}
if [ ! -d "target" ]; then
    echo "Building analyzer..."
    cargo build --release
fi
./target/release/unified-analyzer --integrated "$@"
""".format(os.path.relpath(analyzer_dir, lms_dir))

    with open(sh_path, 'w', encoding='utf-8') as f:
        f.write(sh_content)

    # Make the shell script executable on Unix systems
    try:
        os.chmod(sh_path, 0o755)
    except:
        print("Note: Could not make shell script executable. You may need to run 'chmod +x {0}'".format(sh_path))

    return True

def main():
    parser = argparse.ArgumentParser(description="Integrate analyzers between the LMS and PORT projects")
    parser.add_argument("--lms-dir", default=None, help="Path to the LMS project directory")
    parser.add_argument("--port-dir", default=None, help="Path to the PORT project directory")

    args = parser.parse_args()

    # Get the current directory
    current_dir = os.path.dirname(os.path.abspath(__file__))

    # Determine the LMS directory
    lms_dir = args.lms_dir
    if not lms_dir:
        # Try to find the LMS directory
        if current_dir.endswith("tools"):
            lms_dir = os.path.dirname(current_dir)
        else:
            lms_dir = current_dir

    # Determine the PORT directory
    port_dir = args.port_dir
    if not port_dir:
        # Try to find the PORT directory
        potential_port_dir = os.path.join(os.path.dirname(lms_dir), "port")
        if os.path.exists(potential_port_dir):
            port_dir = potential_port_dir
        else:
            potential_port_dir = os.path.join(os.path.dirname(os.path.dirname(lms_dir)), "port")
            if os.path.exists(potential_port_dir):
                port_dir = potential_port_dir

    if not port_dir or not os.path.exists(port_dir):
        print("Error: Could not find the PORT project directory. Please specify with --port-dir.")
        return 1

    print(f"Using LMS directory: {lms_dir}")
    print(f"Using PORT directory: {port_dir}")

    # Create a new directory for the unified analyzer
    unified_analyzer_dir = os.path.join(lms_dir, "tools", "unified-analyzer")
    create_directory_if_not_exists(unified_analyzer_dir)

    # Create the unified analyzer src directory
    unified_src_dir = os.path.join(unified_analyzer_dir, "src")
    create_directory_if_not_exists(unified_src_dir)

    # Create the analyzers directory and module file
    analyzers_dir = os.path.join(unified_src_dir, "analyzers")
    create_directory_if_not_exists(analyzers_dir)

    # Create the modules directory for specific analyzers
    modules_dir = os.path.join(analyzers_dir, "modules")
    create_directory_if_not_exists(modules_dir)

    # Create Cargo.toml for the unified analyzer
    cargo_toml_path = os.path.join(unified_analyzer_dir, "Cargo.toml")
    if not os.path.exists(cargo_toml_path):
        with open(cargo_toml_path, 'w', encoding='utf-8') as f:
            f.write("""[package]
name = "unified-analyzer"
version = "0.1.0"
edition = "2021"
description = "Unified analyzer for LMS migration"

[dependencies]
tokio = { version = "1.29", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.3", features = ["derive"] }
regex = "1.8"
chrono = "0.4"
async-trait = "0.1"
itertools = "0.11"
futures = "0.3"
log = "0.4"
env_logger = "0.10"
anyhow = "1.0"
thiserror = "1.0"
walkdir = "2.3"
""")
        print(f"Created Cargo.toml at {cargo_toml_path}")

    # Create main.rs for the unified analyzer
    main_rs_path = os.path.join(unified_src_dir, "main.rs")
    if not os.path.exists(main_rs_path):
        create_main_rs(main_rs_path)
        print(f"Created main.rs at {main_rs_path}")

    # Look for the Canvas analyzer in the port project
    canvas_js_path = find_js_analyzer(port_dir, "canvas")
    if canvas_js_path:
        print(f"Found Canvas analyzer at {canvas_js_path}")

        # Create the Rust version of the Canvas analyzer
        canvas_rust_path = os.path.join(modules_dir, "canvas_analyzer.rs")
        create_canvas_analyzer_template(canvas_rust_path)
        print(f"Created Canvas analyzer template at {canvas_rust_path}")
    else:
        print("Warning: Canvas analyzer not found in the PORT project. Creating a template.")
        canvas_rust_path = os.path.join(modules_dir, "canvas_analyzer.rs")
        create_canvas_analyzer_template(canvas_rust_path)
        print(f"Created Canvas analyzer template at {canvas_rust_path}")

    # Look for the Discourse analyzer in the port project
    discourse_js_path = find_js_analyzer(port_dir, "discourse")
    if discourse_js_path:
        print(f"Found Discourse analyzer at {discourse_js_path}")

        # Create the Rust version of the Discourse analyzer
        discourse_rust_path = os.path.join(modules_dir, "discourse_analyzer.rs")
        create_discourse_analyzer_template(canvas_js_path, discourse_rust_path)
        print(f"Created Discourse analyzer template at {discourse_rust_path}")
    else:
        print(f"Note: Discourse analyzer not found. Creating a template based on Canvas analyzer.")
        discourse_rust_path = os.path.join(modules_dir, "discourse_analyzer.rs")
        create_discourse_analyzer_template(canvas_js_path, discourse_rust_path)
        print(f"Created Discourse analyzer template at {discourse_rust_path}")

    # Create the integrated migration analyzer
    migration_analyzer_path = os.path.join(analyzers_dir, "integrated_migration_analyzer.rs")
    create_integrated_migration_analyzer(migration_analyzer_path)
    print(f"Created Integrated Migration analyzer at {migration_analyzer_path}")

    # Update mod.rs file to include new analyzers
    modules_mod_path = os.path.join(modules_dir, "mod.rs")
    update_modules_mod_file(modules_mod_path)
    print(f"Updated modules mod.rs file at {modules_mod_path}")

    analyzers_mod_path = os.path.join(analyzers_dir, "mod.rs")
    update_analyzers_mod_file(analyzers_mod_path)
    print(f"Updated analyzers mod.rs file at {analyzers_mod_path}")

    # Create wrapper scripts for easy execution
    create_wrapper_scripts(lms_dir, unified_analyzer_dir)

    print("\nIntegration complete!")
    print("You can now run the integrated analyzer using:")
    print("  - analyze-integrated.bat (Windows)")
    print("  - ./analyze-integrated.sh (Linux/Mac)")
    print("\nExample:")
    print("  analyze-integrated.bat --canvas ../port/canvas --discourse ../port/port")

    return 0

if __name__ == "__main__":
    sys.exit(main())