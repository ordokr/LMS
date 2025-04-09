/**
 * Simple test script for WebAssembly module - CommonJS version
 */
const fs = require('fs');
const path = require('path');

// Check if WebAssembly files exist
console.log('Checking for WebAssembly files...');
const wasmPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm');
const jsPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js');

if (fs.existsSync(wasmPath)) {
  console.log(`✅ WASM binary found at: ${wasmPath}`);
} else {
  console.log(`❌ WASM binary NOT found at: ${wasmPath}`);
}

if (fs.existsSync(jsPath)) {
  console.log(`✅ JS bindings found at: ${jsPath}`);
} else {
  console.log(`❌ JS bindings NOT found at: ${jsPath}`);
}

// Try loading the bridge
console.log('\nAttempting to load the bridge module...');
try {
  const FileSystemUtils = require('./fileSystemUtilsRustBridge');
  console.log('✅ Successfully required FileSystemUtils bridge');
  
  // Create an instance
  console.log('Creating FileSystemUtils instance...');
  const fsUtils = new FileSystemUtils(__dirname);
  console.log('✅ Successfully created FileSystemUtils instance');
  
  // Check if Rust implementation would be used
  console.log('\nChecking if Rust implementation would be used:');
  console.log(`- useRust flag: ${fsUtils.useRust}`);
  console.log(`- rustInitialized: ${fsUtils.rustInitialized}`);
  console.log(`- rustImpl exists: ${fsUtils.rustImpl !== null}`);
  
  // Test a simple method
  console.log('\nTesting a simple method (getDirCategory):');
  console.log(`Category for 'src/models': ${fsUtils.getDirCategory('src/models')}`);
  
} catch (error) {
  console.error('Error loading or using bridge module:', error);
}
