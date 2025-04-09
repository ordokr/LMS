const fs = require('fs');
const path = require('path');

// Function to write all outputs to a log file
function log(message) {
  fs.appendFileSync('wasm-test-log.txt', message + '\n', 'utf8');
  // Also try console.log for terminal output
  console.log(message);
}

try {
  log('----- WebAssembly Integration Test -----');
  
  // Check WebAssembly files
  const wasmBinaryPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm');
  const jsBindingsPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js');
  
  const wasmExists = fs.existsSync(wasmBinaryPath);
  const jsExists = fs.existsSync(jsBindingsPath);
  
  log(`WASM binary exists: ${wasmExists}`);
  log(`JS bindings exist: ${jsExists}`);
  
  if (wasmExists && jsExists) {
    // Try to load the bridge module
    log('\nLoading FileSystemUtils bridge...');
    const FileSystemUtils = require('./fileSystemUtilsRustBridge');
    log('✅ Bridge loaded successfully');
    
    // Create an instance
    log('\nCreating FileSystemUtils instance...');
    const fsUtils = new FileSystemUtils(__dirname);
    log('✅ Instance created successfully');
    
    // Check rust flags
    log(`\nFlags:`);
    log(`- Using Rust (useRust): ${fsUtils.useRust}`);
    log(`- Rust initialized (rustInitialized): ${fsUtils.rustInitialized}`);
    
    // Test a simple function
    log('\nTesting a simple function:');
    const category = fsUtils.getDirCategory('src/models');
    log(`Category for 'src/models': ${category}`);
    
    // Get a list of all JS files
    log('\nFiltering for JavaScript files:');
    const jsFiles = fsUtils.findFilesByPatterns([/\.js$/]);
    log(`Found ${jsFiles.length} JavaScript files`);
    
    // List the first 5 JS files
    log('\nFirst 5 JavaScript files:');
    for (let i = 0; i < Math.min(jsFiles.length, 5); i++) {
      log(`- ${jsFiles[i]}`);
    }
    
    log('\n----- Test completed successfully -----');
  } else {
    log('\n❌ WebAssembly files not found. Integration test skipped.');
  }
} catch (error) {
  log(`\n❌ Error during test: ${error.message}`);
  log(`Stack trace: ${error.stack}`);
}
