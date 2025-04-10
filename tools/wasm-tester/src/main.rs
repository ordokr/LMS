// tools/wasm-tester/src/main.rs
use std::path::Path;
use std::process::Command;

mod file_system_utils_test;

/// Test the WebAssembly integration with FileSystemUtils
///
/// This program tests the integration between Rust and our Rust WebAssembly
/// implementation of file system utilities.
fn main() {
    println!("----- Starting WebAssembly Integration Test -----");
    println!("Running FileSystemUtils tests...");
    
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    run_integration_test(&current_dir);
}

fn run_integration_test(base_dir: &Path) {
    println!("Creating FileSystemUtils instance...");
    
    // Create a new instance with the current directory as the base
    let result = file_system_utils_test::test_wasm_integration(base_dir);
    
    match result {
        Ok(true) => {
            println!("✅ All WebAssembly tests passed!");
            
            // Optionally run browser tests
            if std::env::var("RUN_BROWSER_TESTS").unwrap_or_default() == "1" {
                println!("Running browser integration tests...");
                run_browser_tests(base_dir);
            }
        },
        Ok(false) => {
            println!("❌ Some WebAssembly tests failed");
            std::process::exit(1);
        },
        Err(e) => {
            println!("❌ Error during WebAssembly tests: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("----- WebAssembly Integration Test Complete -----");
}

fn run_browser_tests(base_dir: &Path) {
    // Start a local server for browser tests
    let server_process = Command::new("npx")
        .args(["http-server", "-p", "8080"])
        .current_dir(base_dir)
        .spawn();
    
    if let Ok(mut server) = server_process {
        println!("Browser test server started on port 8080");
        println!("Running headless browser tests...");
        
        // Run tests with headless browser
        let test_result = Command::new("npx")
            .args(["playwright", "test", "wasm-browser-test.spec.js"])
            .current_dir(base_dir)
            .status();
        
        // Stop the server
        let _ = server.kill();
        
        match test_result {
            Ok(status) if status.success() => {
                println!("✅ Browser integration tests passed!");
            },
            Ok(_) => {
                println!("❌ Browser integration tests failed");
                std::process::exit(1);
            },
            Err(e) => {
                println!("❌ Error running browser tests: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("⚠️ Could not start browser test server, skipping browser tests");
    }
}
