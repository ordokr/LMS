use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Check if PGO data available and configure compiler flags accordingly
    let pgo_data_path = env::var("PGO_DATA_PATH").ok();
    
    if let Some(pgo_path) = pgo_data_path {
        println!("cargo:rustc-flags=-C profile-use={}", pgo_path);
    }
    
    // Use hardware-specific optimizations if available
    if cfg!(target_arch = "x86_64") {
        // Check CPU features using RUSTFLAGS or environment detection
        if std::env::var("RUSTFLAGS").unwrap_or_default().contains("target-cpu=native") {
            println!("cargo:rustc-flags=-C target-cpu=native");
        } else {
            // Fallback to generic optimizations
            // println!("cargo:rustc-flags=-C target-cpu=generic");
        }
    }
    
    // Compile WASM components during build
    println!("cargo:rerun-if-changed=blockchain-wasm/src");
    
    // Build WASM package when needed
    if std::env::var("CARGO_FEATURE_BLOCKCHAIN_FULL").is_ok() {
        Command::new("wasm-pack")
            .args(&["build", "blockchain-wasm", "--target", "bundler"])
            .status()
            .expect("Failed to build WASM components");
    }

    // Link Haskell libraries
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=HSrts");
    println!("cargo:rustc-link-lib=static=HSlms-haskell-integration-0.1.0");
    
    // Additional GHC RTS libraries
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=dl");
    
    // Invalidate the built crate if the header changes
    println!("cargo:rerun-if-changed=include/lms_bridge.h");
}
