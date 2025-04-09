const fs = require('fs');
const path = require('path');

// Define paths
const wasmBinaryPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm');
const jsBindingsPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js');

// Log file existence and sizes
const wasmExists = fs.existsSync(wasmBinaryPath);
const jsExists = fs.existsSync(jsBindingsPath);
const wasmSize = wasmExists ? fs.statSync(wasmBinaryPath).size : 0;
const jsSize = jsExists ? fs.statSync(jsBindingsPath).size : 0;

console.log('WebAssembly Files Check:');
console.log(`WASM binary exists: ${wasmExists}, size: ${wasmSize} bytes`);
console.log(`JS bindings exist: ${jsExists}, size: ${jsSize} bytes`);

// Try to load the bridge
console.log('\nLoading FileSystemUtils bridge...');
try {
  const FileSystemUtils = require('./fileSystemUtilsRustBridge');
  console.log('Bridge loaded successfully');
  
  // Log the bridge version if available
  if (typeof FileSystemUtils.version === 'function') {
    console.log(`Bridge version: ${FileSystemUtils.version()}`);
  }  // Create an instance
  console.log('Creating FileSystemUtils instance...');
  const fsUtils = new FileSystemUtils(__dirname);
  console.log('Instance created successfully');
  
  // Check rust flags
  console.log(`Using Rust implementation: ${fsUtils.useRust}`);
  console.log(`Rust initialized: ${fsUtils.rustInitialized}`);
  console.log(`Rust implementation available: ${fsUtils.rustImpl !== null}`);
  
  // Wait for Rust implementation to initialize if it's being initialized asynchronously
  const testFunctions = async () => {
    if (fsUtils.rustImplPromise) {
      console.log('Waiting for Rust implementation to initialize asynchronously...');
      try {
        await fsUtils.rustImplPromise;
        console.log('Rust implementation initialized successfully');
      } catch (err) {
        console.error('Error initializing Rust implementation:', err);
      }
    }
    
    // Log a simple function call
    console.log('\nTesting getDirCategory function:');
    try {
      const category = fsUtils.getDirCategory('src/models');
      console.log(`Category for 'src/models': ${category}`);
    } catch (err) {
      console.error('Error calling getDirCategory:', err);
    }
    
    // Test file discovery
    console.log('\nTesting file discovery:');
    try {
      console.log('Discovering files...');
      const files = await fsUtils.discoverFiles();
      console.log(`Discovered ${files.length} files`);
      
      // Print first 5 files as a sample
      if (files.length > 0) {
        console.log('First 5 files:');
        for (let i = 0; i < Math.min(files.length, 5); i++) {
          console.log(`- ${files[i]}`);
        }
      }
    } catch (err) {
      console.error('Error discovering files:', err);
    }
    
    // Test file filtering
    console.log('\nTesting file filtering:');
    try {
      console.log('Filtering JavaScript files...');
      const jsFiles = await fsUtils.filterFiles(/\.js$/);
      console.log(`Found ${jsFiles.length} JavaScript files`);
      
      // Print first 5 JS files as a sample
      if (jsFiles.length > 0) {
        console.log('First 5 JavaScript files:');
        for (let i = 0; i < Math.min(jsFiles.length, 5); i++) {
          console.log(`- ${jsFiles[i]}`);
        }
      }
    } catch (err) {
      console.error('Error filtering files:', err);
    }
    
    // Test performance if available
    if (typeof fsUtils.getPerformanceMetrics === 'function') {
      console.log('\nPerformance metrics:');
      try {
        const metrics = fsUtils.getPerformanceMetrics();
        console.log(JSON.stringify(metrics, null, 2));
      } catch (err) {
        console.error('Error getting performance metrics:', err);
      }
    }
    
    // Get diagnostics if available
    if (typeof fsUtils.getDiagnostics === 'function') {
      console.log('\nWebAssembly diagnostics:');
      try {
        const diagnostics = fsUtils.getDiagnostics();
        console.log(JSON.stringify(diagnostics, null, 2));
      } catch (err) {
        console.error('Error getting diagnostics:', err);
      }
    }
    
    console.log('\n----- Test completed successfully -----');
  };
  
  // Run the async tests
  testFunctions().catch(err => {
    console.error('Error during async tests:', err);
  });
} catch (error) {
  console.error('Error in test:', error);
}
