/**
 * Advanced diagnostics for WebAssembly file system utilities
 * This script tests file operations with the WebAssembly bridge and monitors performance
 */
const fs = require('fs');
const path = require('path');
const { performance } = require('perf_hooks');

// Import the FileSystemUtils bridge
const FileSystemUtils = require('./fileSystemUtilsRustBridge');

// Create a test file for reading/writing operations
const TEST_FILE_DIR = path.join(__dirname, 'test-data');
const TEST_FILE_PATH = path.join(TEST_FILE_DIR, 'wasm-test-file.txt');

// Create test directory if it doesn't exist
if (!fs.existsSync(TEST_FILE_DIR)) {
  fs.mkdirSync(TEST_FILE_DIR, { recursive: true });
  console.log(`Created test directory: ${TEST_FILE_DIR}`);
}

// Create a test file with some content
const TEST_CONTENT = `This is a test file for WebAssembly integration testing.
It contains multiple lines of text to test file reading capabilities.
Line 1: Basic text content
Line 2: More text content
Line 3: Special characters: !@#$%^&*()_+
Line 4: Numbers: 1234567890
`;

fs.writeFileSync(TEST_FILE_PATH, TEST_CONTENT);
console.log(`Created test file: ${TEST_FILE_PATH}`);

// Run diagnostics
async function runDiagnostics() {
  console.log('Starting WebAssembly diagnostics with file operations...');
  
  // Create an instance of FileSystemUtils
  const startTime = performance.now();
  const fsUtils = new FileSystemUtils(__dirname);
  const initTime = performance.now() - startTime;
  console.log(`FileSystemUtils instance created in ${initTime.toFixed(2)}ms`);
  
  // Wait for Rust implementation to initialize if needed
  if (fsUtils.rustImplPromise) {
    console.log('Waiting for Rust implementation to initialize...');
    const rustStartTime = performance.now();
    try {
      await fsUtils.rustImplPromise;
      const rustInitTime = performance.now() - rustStartTime;
      console.log(`Rust implementation initialized in ${rustInitTime.toFixed(2)}ms`);
    } catch (err) {
      console.error('Error initializing Rust implementation:', err);
    }
  }
  
  // Get basic diagnostics
  console.log('\n--- Basic Diagnostics ---');
  console.log(`Using Rust implementation: ${fsUtils.useRust}`);
  console.log(`Rust initialized: ${fsUtils.rustInitialized}`);
  
  // Memory usage before operations
  const memBefore = process.memoryUsage();
  console.log('\n--- Memory Usage Before Operations ---');
  console.log(`RSS: ${(memBefore.rss / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Total: ${(memBefore.heapTotal / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Used: ${(memBefore.heapUsed / 1024 / 1024).toFixed(2)} MB`);
  
  // Test file discovery
  console.log('\n--- Testing File Discovery ---');
  console.time('discoverFiles');
  const files = await fsUtils.discoverFiles();
  console.timeEnd('discoverFiles');
  console.log(`Discovered ${files.length} files`);
  
  // Test file filtering
  console.log('\n--- Testing File Filtering ---');
  console.time('filterFiles');
  const jsFiles = await fsUtils.filterFiles(/\.js$/);
  console.timeEnd('filterFiles');
  console.log(`Found ${jsFiles.length} JavaScript files`);
  
  // Test reading file content
  console.log('\n--- Testing File Content Reading ---');
  try {
    // First, ensure the test file is in the list of discovered files
    if (!files.includes(TEST_FILE_PATH)) {
      await fsUtils.discoverFiles(); // Try rediscovering
      
      // Manually add the test file if it's still not found
      if (fsUtils.rustInitialized && fsUtils.rustImpl) {
        fsUtils.rustImpl.add_file(TEST_FILE_PATH);
      }
    }
    
    // Read test file content
    console.time('getFileContent');
    const content = await fsUtils.getFileContent(TEST_FILE_PATH);
    console.timeEnd('getFileContent');
    
    if (content) {
      console.log(`Successfully read file content (${content.length} bytes)`);
      console.log('First 100 characters:', content.substring(0, 100));
    } else {
      console.error('Failed to read file content');
    }
  } catch (err) {
    console.error('Error reading file content:', err);
  }
  
  // Memory usage after operations
  const memAfter = process.memoryUsage();
  console.log('\n--- Memory Usage After Operations ---');
  console.log(`RSS: ${(memAfter.rss / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Total: ${(memAfter.heapTotal / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Used: ${(memAfter.heapUsed / 1024 / 1024).toFixed(2)} MB`);
  
  // Memory change
  console.log('\n--- Memory Change ---');
  console.log(`RSS: ${((memAfter.rss - memBefore.rss) / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Total: ${((memAfter.heapTotal - memBefore.heapTotal) / 1024 / 1024).toFixed(2)} MB`);
  console.log(`Heap Used: ${((memAfter.heapUsed - memBefore.heapUsed) / 1024 / 1024).toFixed(2)} MB`);
  
  // Full WebAssembly diagnostics
  if (typeof fsUtils.getDiagnostics === 'function') {
    console.log('\n--- Full WebAssembly Diagnostics ---');
    try {
      const diagnostics = fsUtils.getDiagnostics();
      console.log(JSON.stringify(diagnostics, null, 2));
    } catch (err) {
      console.error('Error getting diagnostics:', err);
    }
  }
  
  // Clean up test file
  try {
    fs.unlinkSync(TEST_FILE_PATH);
    console.log(`\nRemoved test file: ${TEST_FILE_PATH}`);
  } catch (err) {
    console.error('Error removing test file:', err);
  }
  
  console.log('\n----- Diagnostics completed -----');
}

// Run the diagnostics
runDiagnostics().catch(err => {
  console.error('Error during diagnostics:', err);
});
