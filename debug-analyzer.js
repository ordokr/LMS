const fs = require('fs');
const path = require('path');
const glob = require('glob');

/**
 * Debug script to check what files are found and what content they have
 */
function debugAnalyzer() {
  const baseDir = path.resolve(__dirname);
  console.log(`Base directory: ${baseDir}`);
  
  // Check if src directory exists
  const srcPath = path.join(baseDir, 'src');
  console.log(`src directory exists: ${fs.existsSync(srcPath)}`);
  
  // List all files in src/models directory to check file extensions
  console.log("\nListing files in src/models:");
  const modelsPath = path.join(baseDir, 'src/models');
  if (fs.existsSync(modelsPath)) {
    listFilesRecursively(modelsPath, baseDir);
  }
  
  // List all files in tests directory
  console.log("\nListing files in tests:");
  const testsPath = path.join(baseDir, 'tests');
  if (fs.existsSync(testsPath)) {
    listFilesRecursively(testsPath, baseDir);
  }
  
  // Try different glob patterns
  console.log("\nTrying different glob patterns:");
  
  const patterns = [
    '**/*.rs',
    'src/**/*.rs',
    'src/models/*.rs',
    'src/models/**/*',
    'src/**/*',
    'tests/**/*'
  ];
  
  patterns.forEach(pattern => {
    try {
      const files = glob.sync(path.join(baseDir, pattern));
      console.log(`Pattern '${pattern}': Found ${files.length} files`);
      if (files.length > 0) {
        console.log(`  First few matches:`);
        files.slice(0, 3).forEach(file => {
          console.log(`  - ${path.relative(baseDir, file)}`);
        });
      }
    } catch (err) {
      console.log(`Pattern '${pattern}': Error: ${err.message}`);
    }
  });
  
  // Check for non-standard extensions
  console.log("\nSearching for potential Rust files with other extensions:");
  ['src/models', 'tests'].forEach(dirPath => {
    const fullPath = path.join(baseDir, dirPath);
    if (fs.existsSync(fullPath)) {
      const files = glob.sync(path.join(fullPath, '**/*.*'));
      const extensions = new Set();
      files.forEach(file => {
        const ext = path.extname(file);
        if (ext) extensions.add(ext);
      });
      console.log(`Extensions in ${dirPath}: ${Array.from(extensions).join(', ') || 'none'}`);
    }
  });
  
  // Try to read a specific file in models by name
  console.log("\nTrying to read a specific file in models:");
  const potentialModelFiles = [
    'src/models/forum.rs',
    'src/models/lms.rs',
    'src/models/admin.rs',
    'src/models/mod.rs'
  ];
  
  for (const filePath of potentialModelFiles) {
    const fullPath = path.join(baseDir, filePath);
    console.log(`Checking ${filePath}: ${fs.existsSync(fullPath) ? 'exists' : 'does not exist'}`);
    
    if (fs.existsSync(fullPath)) {
      try {
        const stat = fs.statSync(fullPath);
        console.log(`File size: ${stat.size} bytes`);
        
        if (stat.size > 0) {
          const content = fs.readFileSync(fullPath, 'utf8');
          console.log(`First 200 characters:\n${content.substring(0, 200)}`);
          break;
        }
      } catch (err) {
        console.log(`Error reading file: ${err.message}`);
      }
    }
  }
}

// Helper function to list all files in a directory recursively
function listFilesRecursively(dirPath, baseDir, level = 0) {
  if (level > 3) return; // Prevent infinite recursion
  
  try {
    const items = fs.readdirSync(dirPath);
    for (const item of items) {
      const itemPath = path.join(dirPath, item);
      const stat = fs.statSync(itemPath);
      
      if (stat.isDirectory()) {
        console.log(`${' '.repeat(level * 2)}📁 ${item}/`);
        listFilesRecursively(itemPath, baseDir, level + 1);
      } else {
        const ext = path.extname(item);
        const size = stat.size;
        console.log(`${' '.repeat(level * 2)}📄 ${item} (${ext || 'no extension'}, ${size} bytes)`);
      }
    }
  } catch (err) {
    console.log(`Error listing files: ${err.message}`);
  }
}

debugAnalyzer();