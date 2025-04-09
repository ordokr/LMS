/**
 * Test script to verify the WebAssembly integration with FileSystemUtils
 * 
 * This script tests the integration between JavaScript and our Rust WebAssembly
 * implementation of file system utilities.
 */

const FileSystemUtils = require('./fileSystemUtilsRustBridge');
const path = require('path');

async function runIntegrationTest() {
  console.log('----- Starting WebAssembly Integration Test -----');
  console.log('Creating FileSystemUtils instance...');
  
  // Create a new instance with the current directory as the base
  const fsUtils = new FileSystemUtils(__dirname);
  
  // Wait for the Rust implementation to initialize if available
  if (fsUtils.rustImplPromise) {
    console.log('Waiting for Rust implementation to initialize...');
    await fsUtils.rustImplPromise;
    
    if (fsUtils.rustInitialized) {
      console.log('✅ Rust implementation initialized successfully!');
    } else {
      console.log('⚠️ Rust implementation could not be initialized, using JavaScript fallback.');
    }
  } else {
    console.log('⚠️ Rust implementation not available, using JavaScript fallback.');
  }
  
  // Test file discovery
  console.log('\nDiscovering files...');
  const files = await fsUtils.discoverFiles();
  console.log(`Discovered ${files.length} files`);
  
  // Test file statistics
  console.log('\nGetting file statistics...');
  const stats = await fsUtils.getFileStats();
  console.log(`Total files: ${stats.total}`);
  console.log(`JavaScript files: ${stats.js}`);
  console.log(`Rust files: ${stats.rust}`);
  console.log(`Other files: ${stats.other}`);
  
  // Test file filtering
  console.log('\nFiltering JavaScript files...');
  const jsFiles = await fsUtils.filterFiles(/\.js$/);
  console.log(`Found ${jsFiles.length} JavaScript files`);
  
  console.log('\nFiltering Rust files...');
  const rustFiles = await fsUtils.filterFiles(/\.rs$/);
  console.log(`Found ${rustFiles.length} Rust files`);
  
  // Test file content reading
  console.log('\nReading file contents...');
  await fsUtils.readFileContents();
  
  // Test reading specific files
  if (jsFiles.length > 0) {
    console.log(`\nReading content of a JavaScript file (${path.basename(jsFiles[0])})`);
    const jsContent = await fsUtils.getFileContent(jsFiles[0]);
    console.log(`Content length: ${jsContent ? jsContent.length : 0} characters`);
  }
  
  if (rustFiles.length > 0) {
    console.log(`\nReading content of a Rust file (${path.basename(rustFiles[0])})`);
    const rustContent = await fsUtils.getFileContent(rustFiles[0]);
    console.log(`Content length: ${rustContent ? rustContent.length : 0} characters`);
  }
  
  // Test directory categorization
  console.log('\nTesting directory categorization...');
  const dirCategories = {
    'src/models': await fsUtils.getDirCategory('src/models'),
    'src/components': await fsUtils.getDirCategory('src/components'),
    'src/api': await fsUtils.getDirCategory('src/api'),
    'src/utils': await fsUtils.getDirCategory('src/utils'),
    'src/tests': await fsUtils.getDirCategory('src/tests')
  };
  
  console.log('Directory categories:');
  for (const [dir, category] of Object.entries(dirCategories)) {
    console.log(`  ${dir}: ${category}`);
  }
  
  console.log('\n----- WebAssembly Integration Test Complete -----');
}

// Run the integration test
runIntegrationTest().catch(error => {
  console.error('Error during integration test:', error);
});
