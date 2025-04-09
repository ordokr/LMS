/**
 * File-based WebAssembly diagnostics test
 * This script will write all test results to a file for inspection
 */

const fs = require('fs');
const path = require('path');
const FileSystemUtils = require('./fileSystemUtilsRustBridge');

// Create a log file
const logFile = path.join(__dirname, 'wasm-integration-test-results.txt');
fs.writeFileSync(logFile, '--- WebAssembly Integration Test Results ---\n\n', 'utf8');

// Logging function that writes to both console and file
function log(message) {
  fs.appendFileSync(logFile, message + '\n', 'utf8');
  console.log(message);
}

async function runWasmTest() {
  try {
    log('Starting WebAssembly integration test...');
    log('Date and time: ' + new Date().toLocaleString());
    log('\nChecking WebAssembly files:');
    
    // Check if files exist
    const wasmBinaryPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm');
    const jsBindingsPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js');
    
    log(`WASM binary exists: ${fs.existsSync(wasmBinaryPath)}`);
    log(`JS bindings exist: ${fs.existsSync(jsBindingsPath)}`);
    
    // Create FileSystemUtils instance
    log('\nCreating FileSystemUtils instance:');
    const fsUtils = new FileSystemUtils(__dirname);
    
    // Wait for Rust implementation to initialize if it's being initialized
    if (fsUtils.rustImplPromise) {
      log('Waiting for Rust implementation to initialize...');
      try {
        await fsUtils.rustImplPromise;
        log('Async initialization completed');
      } catch (error) {
        log(`Error during async initialization: ${error.message}`);
      }
    }
    
    // Check Rust implementation status
    log('\nRust implementation status:');
    log(`- useRust: ${fsUtils.useRust}`);
    log(`- rustInitialized: ${fsUtils.rustInitialized}`);
    log(`- rustImpl available: ${fsUtils.rustImpl !== null}`);
    
    // Test basic functionality
    log('\nTesting basic functionality:');
    
    // Test getDirCategory
    log('\n1. Testing getDirCategory:');
    try {
      const category = fsUtils.getDirCategory('src/models');
      log(`Category for 'src/models': ${category}`);
    } catch (error) {
      log(`Error in getDirCategory: ${error.message}`);
    }
    
    // Test discoverFiles
    log('\n2. Testing discoverFiles:');
    try {
      log('Discovering files...');
      const files = await fsUtils.discoverFiles();
      log(`Discovered ${files.length} files`);
    } catch (error) {
      log(`Error in discoverFiles: ${error.message}`);
    }
    
    // Test filtering
    log('\n3. Testing filterFiles:');
    try {
      log('Filtering JavaScript files...');
      const jsFiles = await fsUtils.filterFiles(/\.js$/);
      log(`Found ${jsFiles.length} JavaScript files`);
      
      if (jsFiles.length > 0) {
        log('First 3 JavaScript files:');
        for (let i = 0; i < Math.min(jsFiles.length, 3); i++) {
          log(`- ${jsFiles[i]}`);
        }
      }
    } catch (error) {
      log(`Error in filterFiles: ${error.message}`);
    }
    
    // Check WebAssembly diagnostics
    log('\nWebAssembly Integration Diagnostics:');
    const diagnostics = fsUtils.getDiagnostics();
    log(JSON.stringify(diagnostics, null, 2));
    
    log('\nTest completed successfully!');
  } catch (error) {
    log(`\nUnexpected error during test: ${error.message}`);
    log(`Stack trace: ${error.stack}`);
  }
}

// Run the test and ensure we get results
runWasmTest().then(() => {
  log('\nTest script execution completed. Results written to: ' + logFile);
}).catch(error => {
  log(`\nFatal error in test execution: ${error.message}`);
  log(`Stack trace: ${error.stack}`);
});
