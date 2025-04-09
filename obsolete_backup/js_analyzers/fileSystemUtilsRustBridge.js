// filepath: c:\Users\Tim\Desktop\LMS\fileSystemUtilsRustBridge.js
/**
 * Bridge between JavaScript and the Rust implementation of FileSystemUtils
 * This file provides a compatible API for existing JavaScript code
 */
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Check if the Rust version is built
function checkRustBuild() {
  try {
    // Check if the WebAssembly module exists
    const wasmPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm');
    const jsPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js');
    
    if (fs.existsSync(wasmPath) && fs.existsSync(jsPath)) {
      console.log("WebAssembly module found, will attempt to use Rust implementation");
      return true;
    } else {
      console.log("WebAssembly module not found, falling back to JavaScript implementation");
      return false;
    }
  } catch (error) {
    console.error("Error checking Rust build:", error);
    return false;
  }
}

// Get an instance of the Rust implementation through WASM
async function getRustImplementation() {
  // Check if wasm files exist (both JavaScript and WASM binary)
  const wasmPaths = [
    // From the root directory
    path.join(process.cwd(), 'wasm', 'fs_utils_wasm_standalone.js'),
    path.join(process.cwd(), 'wasm', 'fs_utils_wasm_standalone_bg.wasm'),
    
    // From common locations
    path.join(process.cwd(), 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js'),
    path.join(process.cwd(), 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm')
  ];

  // Check if necessary files exist
  const wasmJsFileExists = wasmPaths.some(p => p.endsWith('.js') && fs.existsSync(p));
  const wasmBinFileExists = wasmPaths.some(p => p.endsWith('.wasm') && fs.existsSync(p));

  if (!wasmJsFileExists || !wasmBinFileExists) {
    console.log(`WASM files missing: JS file exists: ${wasmJsFileExists}, WASM binary exists: ${wasmBinFileExists}`);
    return null;
  }

  // Try different approaches to load the WebAssembly module
  const possibleModulePaths = [
    './wasm/fs_utils_wasm_standalone.js',
    './wasm/fs-utils/fs_utils_wasm_standalone.js',
    path.join(process.cwd(), 'wasm', 'fs_utils_wasm_standalone.js'),
    path.join(process.cwd(), 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js'),
  ];

  // First try CommonJS require
  for (const modulePath of possibleModulePaths) {
    try {
      if (fs.existsSync(modulePath)) {
        console.log(`Attempting to load WASM module with require() from: ${modulePath}`);
        const rustModule = require(modulePath);
        if (rustModule && typeof rustModule === 'object') {
          console.log("Successfully loaded WASM module with require()");
          return rustModule;
        }
      }
    } catch (requireError) {
      console.log(`Failed to load WASM module with require() from ${modulePath}:`, requireError.message);
    }
  }

  // Then try dynamic import (ES modules)
  for (const modulePath of possibleModulePaths) {
    try {
      if (fs.existsSync(modulePath)) {
        console.log(`Attempting to load WASM module with import() from: ${modulePath}`);
        // Use dynamic import for ES modules
        const rustModule = await import(modulePath);
        if (rustModule && typeof rustModule === 'object') {
          console.log("Successfully loaded WASM module with import()");
          return rustModule;
        }
      }
    } catch (importError) {
      console.log(`Failed to load WASM module with import() from ${modulePath}:`, importError.message);
    }
  }

  console.error("All attempts to load WASM module failed");
  return null;
}

// Import the implementations we'll use
const OriginalFileSystemUtils = require('./fileSystemUtils');
const SimplifiedFileSystemUtils = require('./simplifiedFileSystemUtils');

/**
 * Adapter class that presents the same interface as the original FileSystemUtils
 * but delegates to the Rust implementation when available
 */
class FileSystemUtils {
  /**
   * Creates a new FileSystemUtils instance
   * @param {string} baseDir - The base directory to operate on
   * @param {string[]} excludePatterns - Patterns to exclude
   */
  constructor(baseDir, excludePatterns = []) {
    console.log(`Initializing FileSystemUtils bridge with baseDir: ${baseDir}`);
    
    // Try to use Rust implementation if available
    this.useRust = checkRustBuild();
    this.rustImpl = null;
    this.rustInitialized = false;
    
    // Always create the JS implementation as fallback and for file discovery
    console.log("Creating JavaScript implementation as fallback and for file discovery");
    this.jsImpl = new SimplifiedFileSystemUtils(baseDir, excludePatterns);
    
    // Store constructor arguments for possible later use
    this.baseDir = baseDir;
    this.excludePatterns = excludePatterns;
    
    if (this.useRust) {
      console.log("WebAssembly files detected, initializing Rust implementation asynchronously");
      
      // Initialize the Rust implementation asynchronously
      this.rustImplPromise = getRustImplementation().then(impl => {
        if (impl) {
          try {
            console.log("Creating Rust implementation instance...");
            // Check if impl is a constructor or an object
            if (typeof impl === 'function') {
              try {
                this.rustImpl = new impl(baseDir);
              } catch (constructorError) {
                console.log("Failed to use impl as constructor:", constructorError.message);
                // Fall back to using the module directly
                this.rustImpl = impl;
              }
            } else {
              // Handle the case where impl is not a constructor
              console.log("Rust implementation is not a constructor, using as module directly");
              this.rustImpl = impl;
            }
            
            this.rustInitialized = true;
            console.log("✅ Rust implementation initialized successfully");
            
            // Log available methods for debugging
            const methods = typeof this.rustImpl === 'object' ? 
              Object.getOwnPropertyNames(this.rustImpl).filter(m => typeof this.rustImpl[m] === 'function') :
              Object.getOwnPropertyNames(Object.getPrototypeOf(this.rustImpl)).filter(m => m !== 'constructor');
            
            if (methods.length > 0) {
              console.log(`Available Rust methods: ${methods.join(', ')}`);
            } else {
              console.log("No methods found on Rust implementation");
            }
          } catch (error) {
            console.error(`Failed to initialize Rust implementation: ${error}`);
            console.error(`Stack trace: ${error.stack}`);
            // Fall back to JS implementation
            this.rustImpl = null;
            this.rustInitialized = false;
            this.useRust = false;
          }
        } else {
          console.log("No Rust implementation available, using JavaScript implementation");
          this.rustImpl = null;
          this.rustInitialized = false;
          this.useRust = false;
        }
        return this.rustImpl;
      }).catch(error => {
        console.error(`Error initializing Rust implementation: ${error}`);
        this.rustImpl = null;
        this.rustInitialized = false;
        this.useRust = false;
        return null;
      });
    } else {
      console.log("Using JavaScript implementation only (WebAssembly not detected)");
    }
  }

  /**
   * Discovers all relevant files in the base directory, excluding specified patterns.
   * Populates project structure information.
   */  async discoverFiles(rootPath, options = {}) {
    if (!fs.existsSync(rootPath)) {
      console.error(`Root path does not exist: ${rootPath}`);
      return [];
    }
  
    const start = Date.now();
    console.log(`Starting file discovery from ${rootPath}`);
    
    try {
      // Use Rust implementation if available
      if (this.rustImpl) {
        console.log("Using Rust implementation for file discovery");
        try {
          // First, clear any previous file list
          this.rustImpl.clearFiles();
          console.log("Cleared previous file list in Rust implementation");
          
          // Discover files using native JS first
          console.log("Discovering files with JS implementation to prepare for Rust processing");
          const allFiles = await this.jsImpl.discoverFiles(rootPath, options);
          console.log(`Found ${allFiles.length} files with JS implementation`);
          
          // Process files in batches to avoid memory issues
          const BATCH_SIZE = 500;
          const batches = [];
          
          for (let i = 0; i < allFiles.length; i += BATCH_SIZE) {
            batches.push(allFiles.slice(i, i + BATCH_SIZE));
          }
          
          console.log(`Split ${allFiles.length} files into ${batches.length} batches`);
          
          // Process batches
          for (let i = 0; i < batches.length; i++) {
            const batch = batches[i];
            console.log(`Processing batch ${i+1}/${batches.length} (${batch.length} files)`);
            
            // Add this batch to the Rust implementation
            for (const file of batch) {
              try {
                // Add both path and stats to the Rust implementation
                this.rustImpl.addFile(file.path, JSON.stringify({
                  size: file.stats.size,
                  isDirectory: file.stats.isDirectory(),
                  isFile: file.stats.isFile(),
                  mtime: file.stats.mtime.getTime(),
                  // Convert other stats properties as needed
                }));
              } catch (addError) {
                console.error(`Error adding file to Rust implementation: ${file.path}`, addError);
                // Continue with other files
              }
            }
            
            // Allow for garbage collection between batches
            if (i < batches.length - 1) {
              console.log(`Pausing briefly to allow garbage collection after batch ${i+1}`);
              await new Promise(resolve => setTimeout(resolve, 100));
            }
          }
          
          // Finally, apply filters using Rust's implementation
          console.log("Applying filters using Rust implementation");
          const excludePatterns = options.exclude || [];
          const includePatterns = options.include || [];
          
          // Convert pattern arrays to JSON strings to pass to Rust
          const filteredFiles = this.rustImpl.getFilteredFiles(
            JSON.stringify(excludePatterns),
            JSON.stringify(includePatterns)
          );
          
          console.log(`Rust implementation returned ${filteredFiles.length} files after filtering`);
          const end = Date.now();
          console.log(`File discovery completed in ${end - start}ms using Rust implementation`);
          
          // Parse the results if they're returned as a string
          if (typeof filteredFiles === 'string') {
            try {
              return JSON.parse(filteredFiles);
            } catch (parseError) {
              console.error("Error parsing filtered files result from Rust:", parseError);
              // Fall back to JS implementation
              console.log("Falling back to JS implementation due to parse error");
              return this.jsImpl.discoverFiles(rootPath, options);
            }
          }
          
          return filteredFiles;
          
        } catch (error) {
          console.error("Error using Rust implementation for file discovery:", error);
          console.log("Falling back to JS implementation due to Rust error");
          // If the Rust implementation fails, fall back to JS
          return this.jsImpl.discoverFiles(rootPath, options);
        }
      } else {
        console.log("Using JS implementation for file discovery (Rust not available)");
        // Use JavaScript implementation
        return this.jsImpl.discoverFiles(rootPath, options);
      }
    } catch (error) {
      console.error("Unexpected error during file discovery:", error);
      throw error;
    }
  }

  /**
   * Reads the content of discovered text files, skipping binaries and large files.
   * Populates the fileContents map and keyword index.
   */  readFileContents(files) {
    console.log(`Reading file contents for ${files ? (Array.isArray(files) ? files.length : 'object') : 'undefined'} files...`);
    
    // Ensure files is iterable
    if (!files || (!Array.isArray(files) && !(files[Symbol.iterator]))) {
      console.error("Error: 'files' is not iterable in readFileContents");
      
      // Return an empty map as fallback
      return new Map();
    }
    
    if (this.useRust && this.rustInitialized && this.rustImpl) {
      try {
        // Try to use Rust implementation
        return this.jsImpl.readFileContents(files);
      } catch (error) {
        console.error(`Error using Rust implementation for readFileContents: ${error}`);
        // Fall back to JS implementation
      }
    }
    
    // Use JavaScript implementation
    return this.jsImpl.readFileContents(files);
  }

  /**
   * Finds files matching a list of regex patterns.
   */
  findFilesByPatterns(patterns) {
    // For patterns, we'll use JS implementation for now since
    // passing complex regex patterns to Rust is more complex
    return this.jsImpl.findFilesByPatterns(patterns);
  }

  /**
   * Gets statistics about a directory (file count, total size).
   */
  getDirectoryStats(dirPath) {
    // Use JS implementation for now
    return this.jsImpl.getDirectoryStats(dirPath);
  }

  // --- Getters for accessed properties ---
  async getAllFiles() {
    if (this.rustInitialized && this.rustImpl) {
      try {
        // Check for method naming conventions - both camelCase and snake_case
        if (typeof this.rustImpl.get_all_files === 'function') {
          const filesArray = await this.rustImpl.get_all_files();
          return filesArray;
        } else if (typeof this.rustImpl.getAllFiles === 'function') {
          const filesArray = await this.rustImpl.getAllFiles();
          return filesArray;
        } else {
          console.log("Neither get_all_files nor getAllFiles method found in Rust implementation");
          // Fall back to JS implementation
        }
      } catch (error) {
        console.error("Error getting files from Rust implementation:", error);
      }
    }
    
    return this.jsImpl.getAllFiles();
  }

  async getFileContentsMap() {
    // Rust implementation doesn't expose the full map directly due to WASM limitations
    // So we'll use the JS implementation's map
    return this.jsImpl.getFileContentsMap();
  }
  
  async getFileContent(filePath) {
    if (this.rustInitialized && this.rustImpl) {
      try {
        const content = await this.rustImpl.get_file_content(filePath);
        if (content !== null) {
          return content;
        }
      } catch (error) {
        console.error(`Error getting file content for ${filePath} from Rust implementation:`, error);
      }
    }
    
    return this.jsImpl.getFileContentsMap().get(filePath);
  }

  async getProjectStructure() {
    if (this.rustInitialized && this.rustImpl) {
      try {
        const structure = await this.rustImpl.get_project_structure();
        return structure;
      } catch (error) {
        console.error("Error getting project structure from Rust implementation:", error);
      }
    }
    
    return this.jsImpl.getProjectStructure();
  }

  getKeywordIndex() {
    // Keyword index is complex to transfer over WASM boundary 
    // so we'll use the JS implementation
    return this.jsImpl.getKeywordIndex();
  }

  /**
   * Determines the category of a directory based on its path.
   */
  getDirCategory(dirPath) {
    if (this.rustInitialized && this.rustImpl) {
      try {
        return this.rustImpl.get_dir_category(dirPath);
      } catch (error) {
        console.error(`Error getting directory category for ${dirPath} from Rust implementation:`, error);
      }
    }
    
    return this.jsImpl.getDirCategory(dirPath);
  }

  /**
   * Filter files by pattern
   */
  filterFiles(pattern) {
    if (this.useRust && this.rustInitialized && this.rustImpl) {
      try {
        // Check if the Rust implementation has the expected method
        if (typeof this.rustImpl.filter_files === 'function') {
          // Rust implementation may use snake_case naming
          return this.rustImpl.filter_files(pattern.toString());
        } else if (typeof this.rustImpl.filterFiles === 'function') {
          // Or it might use camelCase
          return this.rustImpl.filterFiles(pattern.toString());
        } else {
          console.error("Error filtering files with pattern " + pattern + " using Rust implementation: filter_files method not found");
          // Fall back to JS implementation
        }
      } catch (error) {
        console.error(`Error filtering files with pattern ${pattern} using Rust implementation: ${error}`);
        // Fall back to JS implementation
      }
    }
    
    // Use JavaScript implementation
    return this.jsImpl.filterFiles(pattern);
  }

  /**
   * Get file statistics
   */
  getFileStats() {
    if (this.rustInitialized && this.rustImpl) {
      try {
        // Check for method naming conventions - both camelCase and snake_case
        if (typeof this.rustImpl.get_file_stats === 'function') {
          return this.rustImpl.get_file_stats();
        } else if (typeof this.rustImpl.getFileStats === 'function') {
          return this.rustImpl.getFileStats();
        } else {
          console.error("Error getting file stats from Rust implementation: method not found");
          // Fall back to JS implementation
        }
      } catch (error) {
        console.error(`Error getting file stats from Rust implementation: ${error}`);
        // Fall back to JS implementation
      }
    }
    
    // Fall back to JS implementation or provide basic stats
    if (this.jsImpl && typeof this.jsImpl.getFileStats === 'function') {
      return this.jsImpl.getFileStats();
    } else {
      // Provide a minimal fallback if JS implementation doesn't exist or lacks the method
      return {
        total: this.files ? this.files.length : 0,
        js: 0,
        rust: 0,
        other: 0,
        byExtension: {}
      };
    }
  }

  /**
   * Get diagnostic information about the WebAssembly integration
   * This helps with debugging the integration between JavaScript and Rust
   */
  getDiagnostics() {
    const wasmPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm');
    const jsPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js');
    
    const diagnostics = {
      environment: {
        nodejs: typeof process !== 'undefined',
        version: process?.version || 'unknown'
      },
      webassembly: {
        supported: typeof WebAssembly === 'object',
        filesExist: {
          wasmBinary: fs.existsSync(wasmPath),
          jsBindings: fs.existsSync(jsPath)
        },
        filePaths: {
          wasmBinary: wasmPath,
          jsBindings: jsPath
        }
      },
      bridge: {
        useRust: this.useRust,
        rustInitialized: this.rustInitialized,
        rustImplAvailable: this.rustImpl !== null
      }
    };
    
    // Add methods available on the Rust implementation if it exists
    if (this.rustImpl) {
      try {
        diagnostics.rustImpl = {
          type: typeof this.rustImpl,
          methods: Object.getOwnPropertyNames(
            Object.getPrototypeOf(this.rustImpl)
          ).filter(m => m !== 'constructor')
        };
      } catch (error) {
        diagnostics.rustImpl = {
          error: `Error getting methods: ${error.message}`
        };
      }
    }
    
    return diagnostics;
  }
}

module.exports = FileSystemUtils;
