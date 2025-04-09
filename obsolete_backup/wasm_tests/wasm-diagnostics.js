/**
 * WebAssembly diagnostics that logs results to a file
 */
const fs = require('fs');
const path = require('path');
const { performance } = require('perf_hooks');

// Create a log file for recording diagnostic results
const LOG_FILE_PATH = path.join(__dirname, 'wasm-diagnostics-log.txt');
let logBuffer = '';

// Helper function to log to both console and file
function log(message) {
  console.log(message);
  logBuffer += message + '\n';
}

// Import the FileSystemUtils bridge
log('Loading FileSystemUtils bridge...');
const FileSystemUtils = require('./fileSystemUtilsRustBridge');
log('FileSystemUtils bridge loaded');

// Run diagnostics
async function runDiagnostics() {
  log('Starting WebAssembly diagnostics...');
  
  // Create an instance of FileSystemUtils
  const startTime = performance.now();
  const fsUtils = new FileSystemUtils(__dirname);
  const initTime = performance.now() - startTime;
  log(`FileSystemUtils instance created in ${initTime.toFixed(2)}ms`);
  
  // Wait for Rust implementation to initialize if needed
  if (fsUtils.rustImplPromise) {
    log('Waiting for Rust implementation to initialize...');
    const rustStartTime = performance.now();
    try {
      await fsUtils.rustImplPromise;
      const rustInitTime = performance.now() - rustStartTime;
      log(`Rust implementation initialized in ${rustInitTime.toFixed(2)}ms`);
    } catch (err) {
      log(`Error initializing Rust implementation: ${err.message}`);
    }
  }
  
  // Get basic diagnostics
  log('\n--- Basic Diagnostics ---');
  log(`Using Rust implementation: ${fsUtils.useRust}`);
  log(`Rust initialized: ${fsUtils.rustInitialized}`);
  
  // Memory usage before operations
  const memBefore = process.memoryUsage();
  log('\n--- Memory Usage Before Operations ---');
  log(`RSS: ${(memBefore.rss / 1024 / 1024).toFixed(2)} MB`);
  log(`Heap Total: ${(memBefore.heapTotal / 1024 / 1024).toFixed(2)} MB`);
  log(`Heap Used: ${(memBefore.heapUsed / 1024 / 1024).toFixed(2)} MB`);
  
  // Test file discovery
  log('\n--- Testing File Discovery ---');
  const discoverStartTime = performance.now();
  try {
    const files = await fsUtils.discoverFiles();
    const discoverTime = performance.now() - discoverStartTime;
    log(`Discovered ${files.length} files in ${discoverTime.toFixed(2)}ms`);
    
    // Log a sample of discovered files
    if (files.length > 0) {
      log('\nSample of discovered files:');
      for (let i = 0; i < Math.min(files.length, 5); i++) {
        log(`- ${files[i]}`);
      }
    }
  } catch (err) {
    log(`Error discovering files: ${err.message}`);
  }
  
  // Test file filtering
  log('\n--- Testing File Filtering ---');
  const filterStartTime = performance.now();
  try {
    const jsFiles = await fsUtils.filterFiles(/\.js$/);
    const filterTime = performance.now() - filterStartTime;
    log(`Found ${jsFiles.length} JavaScript files in ${filterTime.toFixed(2)}ms`);
    
    // Log a sample of filtered files
    if (jsFiles.length > 0) {
      log('\nSample of JavaScript files:');
      for (let i = 0; i < Math.min(jsFiles.length, 5); i++) {
        log(`- ${jsFiles[i]}`);
      }
    }
  } catch (err) {
    log(`Error filtering files: ${err.message}`);
  }
  
  // Memory usage after operations
  const memAfter = process.memoryUsage();
  log('\n--- Memory Usage After Operations ---');
  log(`RSS: ${(memAfter.rss / 1024 / 1024).toFixed(2)} MB`);
  log(`Heap Total: ${(memAfter.heapTotal / 1024 / 1024).toFixed(2)} MB`);
  log(`Heap Used: ${(memAfter.heapUsed / 1024 / 1024).toFixed(2)} MB`);
  
  // Memory change
  log('\n--- Memory Change ---');
  log(`RSS: ${((memAfter.rss - memBefore.rss) / 1024 / 1024).toFixed(2)} MB`);
  log(`Heap Total: ${((memAfter.heapTotal - memBefore.heapTotal) / 1024 / 1024).toFixed(2)} MB`);
  log(`Heap Used: ${((memAfter.heapUsed - memBefore.heapUsed) / 1024 / 1024).toFixed(2)} MB`);
  
  // Available methods
  log('\n--- Available Methods ---');
  log(`Available methods on fsUtils: ${Object.getOwnPropertyNames(Object.getPrototypeOf(fsUtils)).join(', ')}`);
  
  // Full WebAssembly diagnostics
  if (typeof fsUtils.getDiagnostics === 'function') {
    log('\n--- Full WebAssembly Diagnostics ---');
    try {
      const diagnostics = fsUtils.getDiagnostics();
      log(JSON.stringify(diagnostics, null, 2));
    } catch (err) {
      log(`Error getting diagnostics: ${err.message}`);
    }
  } else {
    log('\ngetDiagnostics method not available');
  }
  
  log('\n----- Diagnostics completed -----');
  
  // Write the log to a file
  fs.writeFileSync(LOG_FILE_PATH, logBuffer);
  console.log(`\nDiagnostics log written to: ${LOG_FILE_PATH}`);
}

// Run the diagnostics
runDiagnostics().catch(err => {
  log(`Error during diagnostics: ${err.message}`);
  // Write the log to a file even if there's an error
  fs.writeFileSync(LOG_FILE_PATH, logBuffer);
  console.log(`\nDiagnostics log (with errors) written to: ${LOG_FILE_PATH}`);
});
