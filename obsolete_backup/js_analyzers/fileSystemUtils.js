/**
 * Original FileSystemUtils implementation
 * This is a simplified version to support fileSystemUtilsRustBridge.js
 */
const fs = require('fs');
const path = require('path');

class FileSystemUtils {
  constructor(baseDir, excludePatterns = []) {
    this.baseDir = baseDir;
    this.excludePatterns = excludePatterns;
    this.files = [];
    this.fileContents = new Map();
    this.projectStructure = {};
    this.keywordIndex = {};
  }

  async discoverFiles(rootPath, options = {}) {
    console.log(`[JS Implementation] Discovering files in ${rootPath}`);
    const files = this._walkDirectory(rootPath, options);
    this.files = files;
    return files;
  }

  _walkDirectory(dir, options = {}) {
    const excludePatterns = options.exclude || this.excludePatterns || [];
    const result = [];
    
    try {
      const items = fs.readdirSync(dir);
      
      for (const item of items) {
        const fullPath = path.join(dir, item);
        
        // Skip excluded patterns
        if (excludePatterns.some(pattern => 
          typeof pattern === 'string' 
            ? fullPath.includes(pattern) 
            : pattern.test(fullPath)
        )) {
          continue;
        }
        
        try {
          const stats = fs.statSync(fullPath);
          
          if (stats.isDirectory()) {
            // Recursively process subdirectories
            result.push(...this._walkDirectory(fullPath, options));
          } else {
            // Add file with stats
            result.push({ path: fullPath, stats });
          }
        } catch (error) {
          console.error(`Error processing ${fullPath}:`, error.message);
        }
      }
    } catch (error) {
      console.error(`Error reading directory ${dir}:`, error.message);
    }
    
    return result;
  }

  readFileContents(files) {
    const contentsMap = new Map();
    
    for (const file of files) {
      try {
        // Skip directories
        if (file.stats && file.stats.isDirectory()) continue;
        
        const filePath = typeof file === 'string' ? file : file.path;
        
        // Skip files larger than 1MB
        const stats = file.stats || fs.statSync(filePath);
        if (stats.size > 1024 * 1024) {
          continue;
        }
        
        // Read file content
        const content = fs.readFileSync(filePath, 'utf8');
        contentsMap.set(filePath, content);
      } catch (error) {
        // Skip binary files and files with encoding issues
        console.error(`Error reading file content: ${error.message}`);
      }
    }
    
    this.fileContents = contentsMap;
    return contentsMap;
  }

  // Basic implementations of other required methods
  findFilesByPatterns(patterns) {
    return this.files.filter(file => 
      patterns.some(pattern => pattern.test(file.path))
    );
  }

  getDirectoryStats(dirPath) {
    const files = this.files.filter(file => 
      file.path.startsWith(dirPath) && 
      !fs.statSync(file.path).isDirectory()
    );
    
    const totalSize = files.reduce((sum, file) => sum + file.stats.size, 0);
    
    return {
      fileCount: files.length,
      totalSize
    };
  }

  getAllFiles() {
    return this.files;
  }

  getFileContentsMap() {
    return this.fileContents;
  }

  getProjectStructure() {
    return this.projectStructure;
  }

  getKeywordIndex() {
    return this.keywordIndex;
  }

  getDirCategory(dirPath) {
    if (dirPath.includes('node_modules')) return 'dependencies';
    if (dirPath.includes('test') || dirPath.includes('spec')) return 'tests';
    if (dirPath.includes('docs')) return 'documentation';
    if (dirPath.includes('src')) return 'source';
    return 'other';
  }

  filterFiles(pattern) {
    return this.files
      .filter(file => pattern.test(file.path))
      .map(file => file.path);
  }

  getFileStats() {
    return {
      totalFiles: this.files.length,
      totalSize: this.files.reduce((sum, file) => sum + (file.stats?.size || 0), 0),
      byExtension: {}
    };
  }
}

module.exports = FileSystemUtils;