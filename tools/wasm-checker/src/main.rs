// tools/wasm-checker/src/main.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

/// Check if WebAssembly files exist
fn main() {
    let base_dir = std::env::current_dir().expect("Failed to get current directory");
    
    // Define paths to WebAssembly files
    let wasm_binary_path = base_dir.join("wasm").join("fs-utils").join("fs_utils_wasm_standalone_bg.wasm");
    let js_bindings_path = base_dir.join("wasm").join("fs-utils").join("fs_utils_wasm_standalone.js");
    
    println!("Checking WebAssembly files:");
    println!("WASM binary path: {}", wasm_binary_path.display());
    println!("WASM binary exists: {}", wasm_binary_path.exists());
    println!("JS bindings path: {}", js_bindings_path.display());
    println!("JS bindings exists: {}", js_bindings_path.exists());
    
    // Write results to a file as a backup
    write_results_to_file(&base_dir, &wasm_binary_path, &js_bindings_path);
}

/// Write check results to a file
fn write_results_to_file(base_dir: &Path, wasm_binary_path: &PathBuf, js_bindings_path: &PathBuf) {
    let output_path = base_dir.join("wasm-files-check.txt");
    
    let content = format!(
        "WASM binary exists: {}\nJS bindings exist: {}\n",
        wasm_binary_path.exists(),
        js_bindings_path.exists()
    );
    
    match fs::File::create(&output_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            } else {
                println!("Results written to {}", output_path.display());
            }
        },
        Err(e) => {
            eprintln!("Error creating file: {}", e);
        }
    }
}
