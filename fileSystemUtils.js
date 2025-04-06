const fs = require('fs');
const path = require('path');

/**
 * Utility class for file system operations related to project analysis.
 */
class FileSystemUtils {
    constructor(baseDir, excludePatterns = []) {
        this.baseDir = baseDir;
        this.excludePatterns = excludePatterns;
        this.allFiles = [];
        this.fileContents = new Map();
        this.projectStructure = {
            directories: new Set(),
            filesByType: {},
            filesByDir: {},
            dirCategories: {
                api: new Set(),
                models: new Set(),
                ui: new Set(),
                tests: new Set(),
                services: new Set()
            }
        };
        this.keywordIndex = new Map(); // Index for keywords found in files
    }

    /**
     * Discovers all relevant files in the base directory, excluding specified patterns.
     * Populates project structure information.
     */
    discoverFiles() {
        console.log("Discovering files...");
        this.allFiles = this._walkDir(this.baseDir);
        console.log(`Found ${this.allFiles.length} files`);
        return this.allFiles;
    }

    _walkDir(dir, filelist = []) {
        try {
            const files = fs.readdirSync(dir);

            for (const file of files) {
                const filepath = path.join(dir, file);
                const stat = fs.statSync(filepath);
                const relativePath = path.relative(this.baseDir, filepath);

                // Skip if matches any exclude pattern
                if (this.excludePatterns.some(pattern => pattern.test(relativePath))) {
                    continue;
                }

                if (stat.isDirectory()) {
                    // Track directory for project structure analysis
                    if (!relativePath.startsWith('.')) { // Avoid hidden dirs like .git
                        this.projectStructure.directories.add(relativePath || '.');
                        this._categorizeDirectory(relativePath);
                    }
                    filelist = this._walkDir(filepath, filelist);
                } else {
                    // Process file
                    const ext = path.extname(filepath).toLowerCase();

                    // Track by file type
                    if (!this.projectStructure.filesByType[ext]) {
                        this.projectStructure.filesByType[ext] = [];
                    }
                    this.projectStructure.filesByType[ext].push(relativePath);

                    // Track by directory
                    const dirName = path.dirname(relativePath);
                    if (!this.projectStructure.filesByDir[dirName]) {
                        this.projectStructure.filesByDir[dirName] = [];
                    }
                    this.projectStructure.filesByDir[dirName].push(relativePath);

                    filelist.push(filepath);
                }
            }

            return filelist;
        } catch (err) {
            console.error(`Error reading directory ${dir}:`, err.message);
            return filelist;
        }
    }

    _categorizeDirectory(relativePath) {
        const categories = this.projectStructure.dirCategories;
        if (relativePath.includes('api') || relativePath.includes('routes')) {
            categories.api.add(relativePath);
        } else if (relativePath.includes('model') || relativePath.includes('entity')) {
            categories.models.add(relativePath);
        } else if (relativePath.includes('component') || relativePath.includes('ui') || relativePath.includes('pages') || relativePath.includes('features')) {
             // Added pages/features
            categories.ui.add(relativePath);
        } else if (relativePath.includes('test') || relativePath.includes('spec')) {
            categories.tests.add(relativePath);
        } else if (relativePath.includes('service') || relativePath.includes('util') || relativePath.includes('core') || relativePath.includes('shared')) {
             // Added core/shared
            categories.services.add(relativePath);
        }
    }

    /**
     * Reads the content of discovered text files, skipping binaries and large files.
     * Populates the fileContents map and keyword index.
     */
    readFileContents() {
        console.log("Reading file contents with advanced processing...");

        const binarySignatures = [ /* ... signatures ... */ ]; // Keep signatures as before
        const skipExtensions = new Set([ /* ... extensions ... */ ]); // Keep skip extensions
        const textExtensions = new Set([ /* ... extensions ... */ ]); // Keep text extensions

        const fileStats = { read: 0, skipped: 0, binary: 0, tooLarge: 0, error: 0, emptyFile: 0 };

        for (const filePath of this.allFiles) {
            try {
                const ext = path.extname(filePath).toLowerCase();
                if (skipExtensions.has(ext)) {
                    fileStats.skipped++;
                    continue;
                }

                const stats = fs.statSync(filePath);
                if (stats.size === 0) {
                    fileStats.emptyFile++;
                    continue;
                }

                const isKnownText = textExtensions.has(ext);
                const sizeLimit = isKnownText ? 5 * 1024 * 1024 : 1024 * 1024;
                if (stats.size > sizeLimit) {
                    fileStats.tooLarge++;
                    continue;
                }

                if (!isKnownText && this._isLikelyBinary(filePath, binarySignatures)) {
                    fileStats.binary++;
                    continue;
                }

                const content = fs.readFileSync(filePath, 'utf8');
                this.fileContents.set(filePath, content);
                this._indexFileKeywords(filePath, content); // Index keywords
                fileStats.read++;

            } catch (err) {
                fileStats.error++;
                console.error(`Error reading ${filePath}:`, err.message);
            }
        }

        console.log(`Read ${fileStats.read} files, ${fileStats.skipped + fileStats.binary + fileStats.tooLarge + fileStats.emptyFile} skipped`);
        console.log(`  Skipped: ${fileStats.skipped} by extension, ${fileStats.binary} binary, ${fileStats.tooLarge} too large, ${fileStats.emptyFile} empty`);
        return this.fileContents;
    }

    _isLikelyBinary(filePath, binarySignatures) {
        try {
            const fd = fs.openSync(filePath, 'r');
            const buffer = Buffer.alloc(8);
            fs.readSync(fd, buffer, 0, 8, 0);
            fs.closeSync(fd);

            const isBinarySig = binarySignatures.some(signature => {
                for (let i = 0; i < signature.length; i++) {
                    if (buffer[i] !== signature[i]) return false;
                }
                return true;
            });

            const hasNullByte = buffer.includes(0x00);
            return isBinarySig || hasNullByte;
        } catch (err) {
            console.error(`Error checking binary status for ${filePath}:`, err.message);
            return true; // Assume binary on error
        }
    }

     /**
     * Index file keywords for faster searching.
     * (Internal helper method)
     */
    _indexFileKeywords(filePath, content) {
        // List of important keywords to index
        const keywords = [
            // Models
            'struct', 'enum', 'trait', 'impl', 'class', 'interface', 'type', 'model', 'entity',
            // API
            'fn', 'function', 'route', 'get', 'post', 'put', 'delete', 'api', 'endpoint', 'handler', 'router', 'controller',
            // UI
            'component', 'function', 'render', 'return', 'useState', 'useEffect', 'props', 'view', 'page', 'layout',
            // Tests
            'test', 'describe', 'it', 'expect', 'assert', 'mock',
            // General
            'use', 'mod', 'import', 'require', 'export', 'async', 'await', 'pub', 'static', 'const'
        ];

        // Check for each keyword
        for (const keyword of keywords) {
            if (content.includes(keyword)) {
                if (!this.keywordIndex.has(keyword)) {
                    this.keywordIndex.set(keyword, new Set());
                }
                this.keywordIndex.get(keyword).add(filePath);
            }
        }
    }


    /**
     * Finds files matching a list of regex patterns.
     */
    findFilesByPatterns(patterns) {
        return this.allFiles.filter(filePath => {
            const relativePath = path.relative(this.baseDir, filePath);
            return patterns.some(pattern => pattern.test(relativePath));
        });
    }

    /**
     * Gets statistics about a directory (file count, total size).
     */
    getDirectoryStats(dirPath) {
        let fileCount = 0;
        let totalSize = 0;
        const fullPath = path.join(this.baseDir, dirPath);

        const walk = (dir) => {
            try {
                const files = fs.readdirSync(dir);
                for (const file of files) {
                    const filepath = path.join(dir, file);
                    const stat = fs.statSync(filepath);
                    const relativePath = path.relative(this.baseDir, filepath);

                    if (this.excludePatterns.some(pattern => pattern.test(relativePath))) {
                        continue;
                    }

                    if (stat.isDirectory()) {
                        walk(filepath);
                    } else {
                        fileCount++;
                        totalSize += stat.size;
                    }
                }
            } catch (err) {
                console.error(`Error getting stats for directory ${dir}:`, err.message);
            }
        };

        walk(fullPath);
        return { fileCount, totalSize };
    }

    // --- Getters for accessed properties ---
    getAllFiles() {
        return this.allFiles;
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

    /**
     * Determines the category of a directory based on its path.
     * @param {string} dirPath - The directory path to categorize
     * @return {string} The category name (models, api, ui, tests, etc.)
     */
    getDirCategory(dirPath) {
        // Normalize path for consistent pattern matching
        const normalizedPath = dirPath.replace(/\\/g, '/').toLowerCase();
        
        if (normalizedPath.includes('/models') || normalizedPath.includes('/entities')) {
          return 'models';
        } else if (normalizedPath.includes('/api') || normalizedPath.includes('/routes')) {
          return 'api';
        } else if (normalizedPath.includes('/components') || normalizedPath.includes('/views') || normalizedPath.includes('/pages')) {
          return 'ui';
        } else if (normalizedPath.includes('/tests') || normalizedPath.includes('/__tests__') || normalizedPath.includes('/test')) {
          return 'tests';
        } else if (normalizedPath.includes('/services') || normalizedPath.includes('/lib')) {
          return 'service';
        } else if (normalizedPath.includes('/utils') || normalizedPath.includes('/helpers')) {
          return 'utility';
        } else if (normalizedPath.includes('/docs') || normalizedPath.includes('/documentation')) {
          return 'documentation';
        } else if (normalizedPath.includes('/config')) {
          return 'configuration';
        }
        
        return 'other';
    }

    /**
     * Filter files by pattern
     * @param {RegExp} pattern - The regex pattern to match file paths
     * @returns {string[]} - Filtered file paths
     */
    filterFiles(pattern) {
        if (!this.files) return [];
        return Object.keys(this.files)
            .filter(file => pattern.test(file));
    }

    /**
     * Get file statistics
     * @returns {Object} File statistics with counts by file type
     */
    getFileStats() {
        const allFiles = this.getAllFiles();
        
        // Count files by type
        const js = allFiles.filter(file => /\.(js|jsx|ts|tsx)$/.test(file)).length;
        const rust = allFiles.filter(file => /\.rs$/.test(file)).length;
        const other = allFiles.length - js - rust;
        
        return {
          total: allFiles.length,
          js,
          rust,
          other
        };
    }
}

// Re-add the binary signatures and skip/text extensions here
FileSystemUtils.prototype._isLikelyBinary = function(filePath, binarySignatures = []) {
    // Binary file signatures/magic numbers (hex) - Moved here for encapsulation
    const defaultBinarySignatures = [
      [0xFF, 0xD8], // JPEG
      [0x89, 0x50, 0x4E, 0x47], // PNG
      [0x47, 0x49, 0x46], // GIF
      [0x50, 0x4B, 0x03, 0x04], // ZIP/JAR/DOCX
      [0x25, 0x50, 0x44, 0x46], // PDF
    ];
    const signaturesToCheck = binarySignatures.length > 0 ? binarySignatures : defaultBinarySignatures;

    try {
        const fd = fs.openSync(filePath, 'r');
        const buffer = Buffer.alloc(8); // Read first 8 bytes
        fs.readSync(fd, buffer, 0, 8, 0);
        fs.closeSync(fd);

        const isBinarySig = signaturesToCheck.some(signature => {
            for (let i = 0; i < signature.length; i++) {
                if (buffer[i] !== signature[i]) return false;
            }
            return true;
        });

        const hasNullByte = buffer.includes(0x00);
        return isBinarySig || hasNullByte;
    } catch (err) {
        console.error(`Error checking binary status for ${filePath}:`, err.message);
        return true; // Assume binary on error
    }
};

FileSystemUtils.prototype.readFileContents = function() {
    console.log("Reading file contents with advanced processing...");

    // Moved here for encapsulation
    const binarySignatures = [
      [0xFF, 0xD8], [0x89, 0x50, 0x4E, 0x47], [0x47, 0x49, 0x46],
      [0x50, 0x4B, 0x03, 0x04], [0x25, 0x50, 0x44, 0x46]
    ];
    const skipExtensions = new Set([
      '.jpg', '.jpeg', '.png', '.gif', '.bmp', '.ico', '.webp',
      '.mp3', '.mp4', '.avi', '.mov', '.wav', '.flac',
      '.zip', '.tar', '.gz', '.rar', '.7z',
      '.pdf', '.doc', '.docx', '.xls', '.xlsx', '.ppt', '.pptx',
      '.sqlite', '.db', '.jar', '.class', '.pdb', '.lock', // Added .pdb, .lock
      '.icns', // Added .icns
    ]);
     const textExtensions = new Set([
      '.rs', '.ts', '.tsx', '.js', '.jsx', '.vue', '.svelte',
      '.html', '.css', '.scss', '.sass', '.less',
      '.json', '.toml', '.yaml', '.yml',
      '.md', '.markdown', '.txt', '.gitignore', '.taurignore', // Added gitignore/taurignore
      '.sh', '.bash', '.zsh', '.fish', '.bat', '.ps1', // Added bat/ps1
      '.c', '.cpp', '.h', '.hpp', '.cs', '.go', '.py', '.rb',
      '.sql', '.code-workspace', '.svg' // Added sql, code-workspace, svg
    ]);


    const fileStats = { read: 0, skipped: 0, binary: 0, tooLarge: 0, error: 0, emptyFile: 0 };

    for (const filePath of this.allFiles) {
        try {
            const ext = path.extname(filePath).toLowerCase();
            // Handle files with no extension - treat as text unless binary detected
            if (ext === '' && !this._isLikelyBinary(filePath, binarySignatures)) {
                 // Allow reading files with no extension if they seem like text
            } else if (skipExtensions.has(ext)) {
                fileStats.skipped++;
                continue;
            }


            const stats = fs.statSync(filePath);
            if (stats.size === 0) {
                fileStats.emptyFile++;
                continue;
            }

            const isKnownText = textExtensions.has(ext) || ext === ''; // Include no-extension files here
            const sizeLimit = isKnownText ? 5 * 1024 * 1024 : 1024 * 1024;
            if (stats.size > sizeLimit) {
                fileStats.tooLarge++;
                continue;
            }

            // Check binary status only if not explicitly known text or empty extension
            if (!isKnownText && this._isLikelyBinary(filePath, binarySignatures)) {
                fileStats.binary++;
                continue;
            }

            const content = fs.readFileSync(filePath, 'utf8');
            this.fileContents.set(filePath, content);
            this._indexFileKeywords(filePath, content);
            fileStats.read++;

        } catch (err) {
            // Handle potential permission errors more gracefully
            if (err.code === 'EACCES' || err.code === 'EPERM') {
                 console.warn(`Permission denied reading ${filePath}`);
                 fileStats.error++;
            } else {
                fileStats.error++;
                console.error(`Error reading ${filePath}:`, err.message);
            }
        }
    }

    console.log(`Read ${fileStats.read} files, ${fileStats.skipped + fileStats.binary + fileStats.tooLarge + fileStats.emptyFile + fileStats.error} skipped/error`);
    console.log(`  Skipped: ${fileStats.skipped} by extension, ${fileStats.binary} binary, ${fileStats.tooLarge} too large, ${fileStats.emptyFile} empty`);
    console.log(`  Errors: ${fileStats.error}`);
    return this.fileContents;
};


module.exports = FileSystemUtils;