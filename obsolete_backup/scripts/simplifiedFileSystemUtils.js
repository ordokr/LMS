/**
 * Simplified version of FileSystemUtils
 * This is a basic implementation to support fileSystemUtilsRustBridge.js
 */
const FileSystemUtils = require('./fileSystemUtils');

class SimplifiedFileSystemUtils extends FileSystemUtils {
  constructor(baseDir, excludePatterns = []) {
    super(baseDir, excludePatterns);
    this.simplified = true;
  }
  
  // Override any methods if needed for the simplified version
  async discoverFiles(rootPath, options = {}) {
    console.log(`[Simplified JS Implementation] Discovering files in ${rootPath}`);
    return super.discoverFiles(rootPath, options);
  }
}

module.exports = SimplifiedFileSystemUtils;