// Build script for the Canvas-Discourse LMS Integration

use std::process::Command;
use std::env;

fn main() {
    // Define latest versions of key dependencies
    const SEAORM_VERSION: &str = "0.12.3"; // Latest as of April 2025
    const SQLITE_VERSION: &str = "3.44.0"; // Latest as of April 2025
    const RUST_VERSION: &str = "1.76.0";   // Latest stable as of April 2025
    
    println!("cargo:rustc-env=DATABASE_ENGINE=sqlite");
    println!("cargo:rustc-env=ORM_FRAMEWORK=sea-orm");
    println!("cargo:rustc-env=SEAORM_VERSION={}", SEAORM_VERSION);
    println!("cargo:rustc-env=SQLITE_VERSION={}", SQLITE_VERSION);
    println!("cargo:rustc-env=MIN_RUST_VERSION={}", RUST_VERSION);
    
    // Print database info during build
    println!("Building with SQLite database v{} and SeaORM v{} (https://www.sea-ql.org/SeaORM/docs/index/)", 
        SQLITE_VERSION, SEAORM_VERSION);
    
    // Check for required dependencies
    if let Ok(output) = Command::new("sqlite3").arg("--version").output() {
        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("Found SQLite: {}", version.trim());
        }
    } else {
        println!("cargo:warning=SQLite3 command-line tool not found. Make sure SQLite is installed.");
    }
    
    // Set link to SeaORM docs for IDE tooltips
    println!("cargo:rustc-env=SEAORM_DOCS=https://www.sea-ql.org/SeaORM/docs/index/");
}