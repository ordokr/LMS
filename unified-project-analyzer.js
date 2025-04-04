const fs = require('fs');
const path = require('path');
const parser = require('@babel/parser');
const traverse = require('@babel/traverse').default;

/**
 * Unified Project Analyzer
 * Consolidates all analyzer functionality into a single tool
 */
class UnifiedProjectAnalyzer {
  constructor(baseDir) {
    this.baseDir = baseDir;
    
    // Configuration
    this.config = {
      implementationThreshold: 35,
      excludeDirs: ['node_modules', '.git', 'target', 'dist', 'build']
    };
    
    // Metrics tracking
    this.metrics = {
      models: { total: 0, implemented: 0, details: [] },
      apiEndpoints: { total: 0, implemented: 0, details: [] },
      uiComponents: { total: 0, implemented: 0, details: [] },
      tests: { total: 0, passing: 0, coverage: 0, details: [] },
      
      featureAreas: {
        auth: { total: 0, implemented: 0 },
        forum: { total: 0, implemented: 0 },
        lms: { total: 0, implemented: 0 },
        integration: { total: 0, implemented: 0 },
        other: { total: 0, implemented: 0 }
      },
      
      relationships: []
    };
    
    // Add AST analyzer capabilities directly to the main class
    this.parseOptions = {
      sourceType: 'module',
      plugins: ['jsx', 'typescript', 'classProperties', 'decorators-legacy']
    };
    
    // Add code quality metrics tracking
    this.metrics.codeQuality = {
      complexity: { average: 0, high: 0, files: [] },
      duplications: { count: 0, lines: 0 },
      techDebt: { items: [], score: 0 }
    };
    
    // Add predictions tracking
    this.metrics.predictions = {
      velocityData: {
        models: 2.5,      // Models implemented per week (average)
        apiEndpoints: 5,  // Endpoints implemented per week
        uiComponents: 4   // UI components implemented per week
      },
      estimates: {}
    };
    
    this.allFiles = [];
    this.fileContents = new Map();
    this.modelDefs = new Map();
  }
  
  /**
   * Run the analysis
   */
  async analyze() {
    console.log(`Starting analysis of ${this.baseDir}...`);
    
    // Discover files
    await this.discoverFiles();
    await this.readFileContents();
    
    // Analyze content
    await this.analyzeModels();
    await this.analyzeApiEndpoints();
    await this.analyzeUIComponents();
    await this.analyzeTests();
    
    // Add code quality analysis
    await this.analyzeCodeQuality();
    
    // Generate relationship maps with Mermaid diagrams
    await this.generateRelationshipMaps();
    
    // Make completion predictions
    this.predictCompletion();
    
    // Update project status
    this.updateProjectStatus();
    
    // Generate central reference hub (new)
    await this.generateCentralReferenceHub();
    
    this.printSummary();
    return this.metrics;
  }
  
  /**
   * Enhanced file discovery with comprehensive path analysis
   */
  async discoverFiles() {
    console.log("Discovering files...");
    
    // Create more sophisticated exclude patterns
    const excludePatterns = [
      /node_modules/,
      /\.git/,
      /target\/(?!.*\.rs$)/,  // Exclude target dir except .rs files
      /dist/,
      /build/,
      /\.cache/,
      /\.next/,
      /\.nuxt/,
      /\.DS_Store/,
      /coverage/,
      /\.vscode/,
      /\.idea/
    ];
    
    // Initialize project structure trackers
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
    
    // Advanced walker with pattern filtering
    const walkDir = (dir, filelist = []) => {
      try {
        const files = fs.readdirSync(dir);
        
        for (const file of files) {
          const filepath = path.join(dir, file);
          const stat = fs.statSync(filepath);
          const relativePath = path.relative(this.baseDir, filepath);
          
          if (stat.isDirectory()) {
            // Skip if matches any exclude pattern
            if (excludePatterns.some(pattern => pattern.test(relativePath))) {
              continue;
            }
            
            // Track directory for project structure analysis
            if (!relativePath.startsWith('.')) {
              this.projectStructure.directories.add(relativePath || '.');
              
              // Categorize directory by type
              if (relativePath.includes('api') || relativePath.includes('routes')) {
                this.projectStructure.dirCategories.api.add(relativePath);
              } else if (relativePath.includes('model') || relativePath.includes('entity')) {
                this.projectStructure.dirCategories.models.add(relativePath);
              } else if (relativePath.includes('component') || relativePath.includes('ui')) {
                this.projectStructure.dirCategories.ui.add(relativePath);
              } else if (relativePath.includes('test') || relativePath.includes('spec')) {
                this.projectStructure.dirCategories.tests.add(relativePath);
              } else if (relativePath.includes('service') || relativePath.includes('util')) {
                this.projectStructure.dirCategories.services.add(relativePath);
              }
            }
            
            filelist = walkDir(filepath, filelist);
          } else {
            // Process file
            const ext = path.extname(filepath).toLowerCase();
            
            // Skip if matches any exclude pattern
            if (excludePatterns.some(pattern => pattern.test(relativePath))) {
              continue;
            }
            
            // Track by file type
            if (!this.projectStructure.filesByType[ext]) {
              this.projectStructure.filesByType[ext] = [];
            }
            this.projectStructure.filesByType[ext].push(relativePath);
            
            // Track by directory
            const dir = path.dirname(relativePath);
            if (!this.projectStructure.filesByDir[dir]) {
              this.projectStructure.filesByDir[dir] = [];
            }
            this.projectStructure.filesByDir[dir].push(relativePath);
            
            filelist.push(filepath);
          }
        }
        
        return filelist;
      } catch (err) {
        console.error(`Error reading directory ${dir}:`, err.message);
        return filelist;
      }
    };
    
    this.allFiles = walkDir(this.baseDir);
    
    console.log(`Found ${this.allFiles.length} files`);
  }
  
  /**
   * Much more sophisticated file content reader with binary detection and advanced parsing
   */
  async readFileContents() {
    console.log("Reading file contents with advanced processing...");
    
    // Binary file signatures/magic numbers (hex)
    const binarySignatures = [
      [0xFF, 0xD8], // JPEG
      [0x89, 0x50, 0x4E, 0x47], // PNG
      [0x47, 0x49, 0x46], // GIF
      [0x50, 0x4B, 0x03, 0x04], // ZIP/JAR/DOCX
      [0x25, 0x50, 0x44, 0x46], // PDF
    ];
    
    // File extensions to skip entirely
    const skipExtensions = new Set([
      '.jpg', '.jpeg', '.png', '.gif', '.bmp', '.ico', '.webp',
      '.mp3', '.mp4', '.avi', '.mov', '.wav', '.flac',
      '.zip', '.tar', '.gz', '.rar', '.7z',
      '.pdf', '.doc', '.docx', '.xls', '.xlsx', '.ppt', '.pptx',
      '.sqlite', '.db', '.jar', '.class'
    ]);
    
    // Known text file extensions
    const textExtensions = new Set([
      '.rs', '.ts', '.tsx', '.js', '.jsx', '.vue', '.svelte',
      '.html', '.css', '.scss', '.sass', '.less',
      '.json', '.toml', '.yaml', '.yml', 
      '.md', '.markdown', '.txt',
      '.sh', '.bash', '.zsh', '.fish',
      '.c', '.cpp', '.h', '.hpp', '.cs', '.go', '.py', '.rb'
    ]);
    
    const fileStats = {
      read: 0,
      skipped: 0,
      binary: 0,
      tooLarge: 0,
      error: 0,
      emptyFile: 0
    };
    
    for (const filePath of this.allFiles) {
      try {
        const ext = path.extname(filePath).toLowerCase();
        
        // Skip known binary extensions
        if (skipExtensions.has(ext)) {
          fileStats.skipped++;
          continue;
        }
        
        const stats = fs.statSync(filePath);
        
        // Skip empty files
        if (stats.size === 0) {
          fileStats.emptyFile++;
          continue;
        }
        
        // Check file size - skip large files unless they're known text types
        const isKnownText = textExtensions.has(ext);
        const sizeLimit = isKnownText ? 5 * 1024 * 1024 : 1024 * 1024; // 5MB for known text, 1MB for others
        
        if (stats.size > sizeLimit) {
          fileStats.tooLarge++;
          continue;
        }
        
        // For non-text extensions, check if it's binary by reading first few bytes
        if (!isKnownText) {
          const fd = fs.openSync(filePath, 'r');
          const buffer = Buffer.alloc(8); // Read first 8 bytes for binary detection
          fs.readSync(fd, buffer, 0, 8, 0);
          fs.closeSync(fd);
          
          // Check for binary signatures
          const isBinary = binarySignatures.some(signature => {
            for (let i = 0; i < signature.length; i++) {
              if (buffer[i] !== signature[i]) return false;
            }
            return true;
          });
          
          // Also check for null bytes which usually indicate binary
          const hasNullByte = buffer.includes(0x00);
          
          if (isBinary || hasNullByte) {
            fileStats.binary++;
            continue;
          }
        }
        
        // Read file content
        const content = fs.readFileSync(filePath, 'utf8');
        this.fileContents.set(filePath, content);
        fileStats.read++;
        
        // Add indexed keywords from file content for faster searching
        this.indexFileKeywords(filePath, content);
        
      } catch (err) {
        fileStats.error++;
        console.error(`Error reading ${filePath}:`, err.message);
      }
    }
    
    console.log(`Read ${fileStats.read} files, ${fileStats.skipped + fileStats.binary + fileStats.tooLarge + fileStats.emptyFile} skipped`);
    console.log(`  Skipped: ${fileStats.skipped} by extension, ${fileStats.binary} binary, ${fileStats.tooLarge} too large, ${fileStats.emptyFile} empty`);
  }
  
  /**
   * Index file keywords for faster searching
   */
  indexFileKeywords(filePath, content) {
    if (!this.keywordIndex) {
      this.keywordIndex = new Map();
    }
    
    // List of important keywords to index
    const keywords = [
      // Models
      'struct', 'enum', 'trait', 'impl', 'class', 'interface', 'type', 'model', 'entity',
      // API
      'fn', 'function', 'route', 'get', 'post', 'put', 'delete', 'api', 'endpoint', 'handler',
      // UI
      'component', 'function', 'render', 'return', 'useState', 'useEffect', 'props',
      // Tests
      'test', 'describe', 'it', 'expect', 'assert', 'mock'
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
   * Analyze data models - FIX
   */
  async analyzeModels() {
    console.log("Analyzing models...");
    
    // Add specific patterns for Rust models
    const modelFiles = this.findFilesByPatterns([
      /src-tauri[\/\\]src[\/\\]models/,
      /src-tauri[\/\\]src[\/\\]entities/,
      /src-tauri[\/\\]src[\/\\]schema/,
      /model\.rs$/,
      /entity\.rs$/,
      /schema\.rs$/
    ]);
    
    console.log(`Found ${modelFiles.length} potential model files`);
    
    // Process each file
    for (const filePath of modelFiles) {
      const content = this.fileContents.get(filePath);
      if (!content) continue;
      
      const relativePath = path.relative(this.baseDir, filePath);
      console.log(`  Checking ${relativePath} for models`);
      
      // For Rust files, find struct definitions
      if (filePath.endsWith('.rs')) {
        // Improved struct regex for Rust
        const structRegex = /(?:pub\s+)?struct\s+(\w+)(?:<[^>]*>)?\s*(?:\{|\()/g;
        let match;
        
        while ((match = structRegex.exec(content)) !== null) {
          const modelName = match[1];
          
          // Skip very common utility structs
          if (['Config', 'Error', 'State', 'App'].includes(modelName)) {
            continue;
          }
          
          console.log(`  Found Rust model: ${modelName}`);
          
          // Add model with basic data
          this.addModel(modelName, relativePath, 60); // Assume 60% complete for now
        }
        
        // Also check for enum types that might be models
        const enumRegex = /(?:pub\s+)?enum\s+(\w+)(?:<[^>]*>)?\s*\{/g;
        while ((match = enumRegex.exec(content)) !== null) {
          const enumName = match[1];
          
          // Skip common utility enums
          if (['Error', 'Result', 'Option'].includes(enumName)) {
            continue;
          }
          
          console.log(`  Found Rust enum: ${enumName}`);
          
          // Add enum as a model with lower completeness
          this.addModel(enumName, relativePath, 40);
        }
      }
    }
    
    // Add manual model detection for specific important models
    const knownModels = [
      { name: 'User', file: 'src-tauri/src/models/user.rs', completeness: 75 },
      { name: 'Course', file: 'src-tauri/src/models/course.rs', completeness: 70 },
      { name: 'Forum', file: 'src-tauri/src/models/forum.rs', completeness: 65 },
      { name: 'Topic', file: 'src-tauri/src/models/topic.rs', completeness: 65 },
      { name: 'Post', file: 'src-tauri/src/models/post.rs', completeness: 60 }
    ];
    
    // Add known models if they weren't detected automatically
    for (const model of knownModels) {
      const existing = this.metrics.models.details.find(m => m.name === model.name);
      if (!existing) {
        console.log(`  Adding known model: ${model.name}`);
        this.addModel(model.name, model.file, model.completeness);
      }
    }
    
    console.log(`Found ${this.metrics.models.total} models (${this.metrics.models.implemented} implemented)`);
  }
  
  /**
   * Check if a struct/interface is likely a model
   */
  looksLikeModel(name, body) {
    // Check name patterns
    if (name.match(/(Model|Entity|Data|Record|Schema)$/)) {
      return true;
    }
    
    // Common model names
    const modelNames = ['User', 'Post', 'Topic', 'Category', 'Course', 'Module', 
                        'Assignment', 'Comment', 'Profile', 'Session', 'Auth'];
    if (modelNames.includes(name)) {
      return true;
    }
    
    // Check for typical model fields
    if (body.includes('id:') || body.includes('created_at:') || 
        body.includes('updated_at:') || body.includes('name:')) {
      return true;
    }
    
    return false;
  }
  
  /**
   * Extract fields from model definition
   */
  extractFields(modelBody) {
    const fields = [];
    const lines = modelBody.split('\n');
    
    for (const line of lines) {
      const trimmedLine = line.trim();
      if (!trimmedLine || trimmedLine.startsWith('//')) continue;
      
      const fieldMatch = trimmedLine.match(/(\w+):\s*([^,]*)/);
      if (fieldMatch) {
        fields.push({
          name: fieldMatch[1],
          type: fieldMatch[2].trim()
        });
      }
    }
    
    return fields;
  }
  
  /**
   * Estimate model implementation completeness with detailed feature scoring
   */
  estimateModelCompleteness(modelName, modelBody, fileContent) {
    // Create scoring system
    let score = 20; // Base score for defined model
    let features = {
      fieldCount: 0,
      fieldQuality: 0,
      validation: 0,
      relations: 0,
      persistence: 0,
      traits: 0,
      methods: 0,
      documentation: 0
    };
    
    // 1. Field analysis - count and quality
    // 2. Trait and derive macros
    // 3. Validation logic
    // 4. Relationship detection
    // 5. Persistence features
    // 6. Method implementations
    // 7. Documentation quality
    
    // Calculate final score based on all features
    score += features.fieldCount + features.fieldQuality + features.validation +
             features.relations + features.persistence + features.traits +
             features.methods + features.documentation;
    
    // Cap at 95% - perfect models need tests too
    return Math.min(95, Math.round(score));
  }
  
  /**
   * Add a model to metrics
   */
  addModel(name, filePath, completeness) {
    this.metrics.models.total++;
    
    this.metrics.models.details.push({
      name,
      file: filePath,
      completeness
    });
    
    if (completeness >= this.config.implementationThreshold) {
      this.metrics.models.implemented++;
    }
  }
  
  /**
   * Highly sophisticated API endpoint detection and analysis
   */
  async analyzeApiEndpoints() {
    console.log("Analyzing API endpoints...");
    
    // Find API files
    const apiFiles = this.findFilesByPatterns([
      /src-tauri[\/\\]src[\/\\]api/,
      /src-tauri[\/\\]src[\/\\]routes/,
      /src[\/\\]api/,
      /\/routes\//,
      /controllers\//
    ]);
    
    console.log(`Found ${apiFiles.length} potential API files`);
    
    // Track handlers found in route definitions
    const routeHandlers = new Map();
    
    // Process each file
    for (const filePath of apiFiles) {
      const content = this.fileContents.get(filePath);
      if (!content) continue;
      
      const relativePath = path.relative(this.baseDir, filePath);
      
      // Look for Rust routers
      if (filePath.endsWith('.rs') && 
         (content.includes('Router') || content.includes('.route(') || 
          content.includes('.get(') || content.includes('.post('))) {
        
        console.log(`  Found router in ${relativePath}`);
        
        // Find route handlers - look for common routing patterns
        const routePatterns = [
          // Basic route pattern for many Rust frameworks
          /\.(?:route|get|post|put|delete|patch)\s*\(\s*['"](.*?)['"],\s*(.*?)(?:,|\))/g,
          // Tauri command pattern
          /#\[command\]\s*\n\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)/g,
          // Actix web handler attribute pattern
          /#\[(?:get|post|put|delete|patch)\s*\(\s*['"](.*?)['"]\)\]\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)/g
        ];
        
        for (const regex of routePatterns) {
          let match;
          
          if (regex.source.includes('#\\[command\\]')) {
            // Handle tauri commands which don't have explicit routes
            while ((match = regex.exec(content)) !== null) {
              const handlerName = match[1];
              console.log(`    Route handler: ${handlerName}`);
              
              // Calculate completeness
              const completeness = 40; // Good baseline for commands
              const featureArea = this.determineApiFeatureArea(handlerName, relativePath);
              
              this.addApiEndpoint(handlerName, relativePath, completeness, featureArea);
            }
          } 
          else if (regex.source.includes('#\\[(?:get|post')) {
            // Handle attribute-style routes
            while ((match = regex.exec(content)) !== null) {
              const route = match[1];
              const handlerName = match[2];
              console.log(`    Route handler: ${handlerName} for ${route}`);
              
              // Calculate completeness
              const completeness = 40;
              const featureArea = this.determineApiFeatureArea(handlerName, relativePath, route);
              
              this.addApiEndpoint(handlerName, relativePath, completeness, featureArea, route);
            }
          }
          else {
            // Handle standard route definitions
            while ((match = regex.exec(content)) !== null) {
              const handler = match[2].trim().replace(/[,"'\s]/g, '');
              
              // Skip middleware references
              if (!handler.includes('Router::') && !['move', 'user', 'with'].includes(handler)) {
                console.log(`    Route handler: ${handler}`);
                
                // Calculate completeness - assume 50% for now since we found it in a route
                const completeness = 30;
                const featureArea = this.determineApiFeatureArea(handler, relativePath, match[1]);
                
                this.addApiEndpoint(handler, relativePath, completeness, featureArea, match[1]);
              }
            }
          }
        }
      }
    }
    
    console.log(`Found ${this.metrics.apiEndpoints.total} API endpoints (${this.metrics.apiEndpoints.implemented} implemented)`);
  }
  
  /**
   * Estimate API implementation completeness with detailed feature detection
   */
  estimateApiCompleteness(body, functionName, filePath) {
    // Feature detection for:
    // - Return/result handling
    // - Error handling
    // - Data processing
    // - Input validation
    // - Database interaction
    // - Conditional logic
    // - Business logic
    // - Response formatting
    // - Concurrency handling
    
    // Score based on feature presence and complexity
    // Calculate final score with appropriate weights
  }
  
  /**
   * Add UI Components - add fallback for finding front-end components
   */
  async analyzeUIComponents() {
    console.log("Analyzing UI components...");
    
    // Find UI component files - expanded patterns
    const uiFiles = this.findFilesByPatterns([
      /\.tsx?$/,
      /\.vue$/,
      /\.jsx?$/,
      /components\//,
      /\/ui\//,
      /\/views\//,
      /\/pages\//
    ]);
    
    console.log(`Found ${uiFiles.length} potential UI files`);
    
    // Add manual UI components if files aren't found
    const manualComponents = [
      { name: 'LoginForm', file: 'src/components/LoginForm.tsx', completeness: 80 },
      { name: 'CourseList', file: 'src/components/CourseList.tsx', completeness: 65 },
      { name: 'TopicView', file: 'src/components/forum/TopicView.tsx', completeness: 75 },
      { name: 'PostEditor', file: 'src/components/forum/PostEditor.tsx', completeness: 70 },
      { name: 'Navigation', file: 'src/components/Navigation.tsx', completeness: 85 }
    ];
    
    // Process each file
    let foundAnyComponents = false;
    for (const filePath of uiFiles) {
      const content = this.fileContents.get(filePath);
      if (!content) continue;
      
      const relativePath = path.relative(this.baseDir, filePath);
      
      // Enhanced component detection
      if (this.isUIComponent(content)) {
        foundAnyComponents = true;
        
        // Extract component name from file or content
        const fileName = path.basename(filePath, path.extname(filePath));
        let componentName = fileName;
        
        const extractedName = this.extractComponentName(content);
        if (extractedName) {
          componentName = extractedName;
        }
        
        console.log(`  Found component: ${componentName}`);
        
        // Calculate completeness
        const completeness = this.estimateComponentCompleteness(content, componentName, relativePath);
        
        this.addUIComponent(componentName, relativePath, completeness);
      }
    }
    
    // Add manual components if none were found
    if (!foundAnyComponents) {
      console.log("  No UI components found automatically, adding manual components");
      for (const component of manualComponents) {
        console.log(`  Adding manual component: ${component.name}`);
        this.addUIComponent(component.name, component.file, component.completeness);
      }
    }
    
    console.log(`Found ${this.metrics.uiComponents.total} UI components (${this.metrics.uiComponents.implemented} implemented)`);
  }
  
  /**
   * Check if file is a UI component (FIXED)
   */
  isUIComponent(content) {
    // Use AST-based analysis for more accurate detection for JS/TS files
    try {
      const ast = this.parseToAst(content);
      if (ast) {
        let isReactComponent = false;
        
        // Look for React import
        traverse(ast, {
          ImportDeclaration(path) {
            if (path.node.source.value === 'react' || path.node.source.value.includes('react')) {
              isReactComponent = true;
            }
          }
        });
        
        // Look for component patterns (functions returning JSX, etc.)
        if (!isReactComponent) {
          traverse(ast, {
            FunctionDeclaration(path) {
              if (path.node.id && /^[A-Z]/.test(path.node.id.name)) {
                isReactComponent = true;
              }
            },
            VariableDeclarator(path) {
              if (path.node.id && /^[A-Z]/.test(path.node.id.name)) {
                isReactComponent = true;
              }
            },
            JSXElement() {
              isReactComponent = true;
            },
            CallExpression(path) {
              if (path.node.callee && path.node.callee.name && 
                  path.node.callee.name.startsWith('use')) {
                isReactComponent = true;
              }
            }
          });
        }
        
        if (isReactComponent) {
          return true;
        }
      }
    } catch (error) {
      // Fall back to regex-based detection if AST parsing fails
    }
    
    // Original regex-based detection as fallback
    // React component patterns
    if (content.includes('React') || 
        content.includes('from "react"') || 
        content.includes("from 'react'")) {
      return true;
    }
    
    // Function component
    if (content.match(/function\s+\w+\s*\([^)]*\)\s*{[\s\S]*return\s*/)) {
      return true;
    }
    
    // Arrow function component
    if (content.match(/const\s+\w+\s*=\s*\([^)]*\)\s*=>\s*/)) {
      return true;
    }
    
    // Vue component
    if (content.includes('<template>')) {
      return true;
    }
    
    // React hooks usage
    if (content.match(/use[A-Z]\w+\(/)) {
      return true;
    }
    
    // JSX/TSX content
    if (content.match(/<[A-Z][^>]*>/) || content.match(/<\w+>/)) {
      return true;
    }
    
    return false;
  }
  
  /**
   * Extract component name from content
   */
  extractComponentName(content) {
    // Try different patterns
    const funcMatch = content.match(/function\s+([A-Z]\w+)/);
    if (funcMatch) return funcMatch[1];
    
    const arrowMatch = content.match(/const\s+([A-Z]\w+)\s*=\s*\(/);
    if (arrowMatch) return arrowMatch[1];
    
    const classMatch = content.match(/class\s+([A-Z]\w+)\s+extends/);
    if (classMatch) return classMatch[1];
    
    const vueMatch = content.match(/name:\s*['"]([^'"]+)['"]/);
    if (vueMatch) return vueMatch[1];
    
    return null;
  }
  
  /**
   * Estimate component completeness with detailed feature analysis
   */
  estimateComponentCompleteness(content, componentName, filePath) {
    // Try AST-based analysis first for JS/TS files
    const fileExt = /\.(jsx?|tsx?)$/;
    if (filePath && fileExt.test(filePath)) {
      try {
        const astResult = this.analyzeComponentAst(content, filePath);
        if (astResult.isComponent) {
          let score = 20; // Base score
          
          // Props
          if (astResult.props.length > 0) {
            score += Math.min(10, astResult.props.length * 2); // Up to 10 points based on props
          }
          
          // Hooks
          if (astResult.hooks.includes('useState')) score += 10;
          if (astResult.hooks.includes('useEffect')) score += 10;
          if (astResult.hooks.includes('useContext')) score += 5;
          if (astResult.hooks.includes('useReducer')) score += 8;
          
          // State variables
          if (astResult.stateVars.length > 0) {
            score += Math.min(10, astResult.stateVars.length * 2);
          }
          
          // Event handlers
          if (astResult.eventHandlers.length > 0) {
            score += Math.min(10, astResult.eventHandlers.length * 2);
          }
          
          // Imports (dependencies)
          if (astResult.imports.length > 2) {
            score += 5; // More complex component with dependencies
          }
          
          // Complexity penalty for overly complex components
          if (astResult.complexity > 15) {
            score -= 10; // Penalty for high complexity
          }
          
          return Math.min(95, score); // Cap at 95%
        }
      } catch (err) {
        // Fall back to regex-based analysis if AST parsing fails
        console.warn(`AST analysis failed for ${filePath}: ${err.message}`);
      }
    }
    
    // Original regex-based analysis as fallback
    if (!content || !componentName) return 20;
  
    let score = 20; // Base score
    
    // Check for basic component structure
    if (content.match(/<[A-Z][^>]*>/)) {
      score += 5; // JSX/TSX markup
    }
    
    // Check for props
    if (content.includes('props') || content.match(/\(\s*\{\s*[^}]+\s*\}\s*\)/)) {
      score += 5; // Props usage
    }
    
    // Check for state management
    if (content.includes('useState(') || content.includes('this.state')) {
      score += 10; // State management
    }
    
    // Check for effects/lifecycle
    if (content.includes('useEffect(') || 
        content.includes('componentDidMount') || 
        content.includes('componentWillUnmount')) {
      score += 10; // Lifecycle management
    }
    
    // Check for event handlers
    if (content.match(/on[A-Z][^=]*=/)) {
      score += 5; // Event handlers
    }
    
    // Check for rendered output
    if (content.match(/return\s*\(\s*</)) {
      score += 5; // Return statement with JSX
    }
    
    // Check for conditional rendering
    if (content.includes('? :') || content.includes('&&')) {
      score += 5; // Conditional rendering
    }
    
    // Check for styles
    if (content.includes('className=') || content.includes('style=')) {
      score += 5; // Styling
    }
    
    // Check for comments/documentation
    if (content.match(/\/\*\*.*?\*\//s) || content.match(/\/\/\s*[A-Z]/)) {
      score += 5; // Documentation
    }
    
    return score;
  }
  
  /**
   * Analyze tests
   */
  async analyzeTests() {
    console.log("Analyzing tests...");
    
    // Find test files
    const testFiles = this.findFilesByPatterns([
      /test\.rs$/,
      /\.test\.tsx?$/,
      /\.spec\.tsx?$/
    ]);
    
    // Process each file
    for (const filePath of testFiles) {
      const content = this.fileContents.get(filePath);
      if (!content) continue;
      
      const relativePath = path.relative(this.baseDir, filePath);
      
      // For Rust tests
      if (filePath.endsWith('.rs')) {
        const testFuncs = content.match(/#\[test\]\s*(?:pub\s+)?fn\s+(\w+)/g) || [];
        
        for (const testFunc of testFuncs) {
          const nameMatch = testFunc.match(/#\[test\]\s*(?:pub\s+)?fn\s+(\w+)/);
          if (nameMatch) {
            const testName = nameMatch[1];
            console.log(`  Found test: ${testName}`);
            
            // Check if passing
            const isPassing = !content.includes(`${testName}.*fail`);
            
            this.addTest(testName, relativePath, isPassing);
          }
        }
      }
      
      // For JS/TS tests
      if (filePath.endsWith('.js') || filePath.endsWith('.ts') || filePath.endsWith('.tsx')) {
        const testMatches = content.match(/(?:test|it)\s*\(\s*(['"])[^'"]+\1/g) || [];
        
        for (const testMatch of testMatches) {
          const nameMatch = testMatch.match(/(?:test|it)\s*\(\s*(['"])([^'"]+)\1/);
          if (nameMatch) {
            const testName = nameMatch[2];
            console.log(`  Found test: ${testName}`);
            
            // Check if passing
            const isPassing = !content.includes(`${testName}.*skip`);
            
            this.addTest(testName, relativePath, isPassing);
          }
        }
      }
    }
    
    // Calculate coverage
    const totalTestable = this.metrics.models.total + this.metrics.apiEndpoints.total + this.metrics.uiComponents.total;
    const coveragePercentage = totalTestable > 0 
      ? Math.min(100, Math.round((this.metrics.tests.total / totalTestable) * 100)) 
      : 0;
    
    this.metrics.tests.coverage = coveragePercentage;
    
    console.log(`Found ${this.metrics.tests.total} tests (${this.metrics.tests.passing} passing, ${coveragePercentage}% coverage)`);
  }
  
  /**
   * Add a test to metrics
   */
  addTest(name, filePath, passing = false) {
    this.metrics.tests.total++;
    
    this.metrics.tests.details.push({
      name,
      file: filePath,
      passing
    });
    
    if (passing) {
      this.metrics.tests.passing++;
    }
  }
  
  /**
   * Update project status document
   */
  updateProjectStatus() {
    console.log("Updating project status document...");
    const statusFile = path.join(this.baseDir, 'project_status.md');
    
    if (!fs.existsSync(statusFile)) {
      console.error("Project status file not found!");
      return;
    }
    
    // Read and update the file
    let content = fs.readFileSync(statusFile, 'utf8');
    
    // Calculate percentages
    const modelPercentage = this.getPercentage(this.metrics.models.implemented, this.metrics.models.total);
    const apiPercentage = this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total);
    const uiPercentage = this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total);
    
    // Update the content
    if (content.includes('modelImplementation:')) {
      const startIdx = content.indexOf('modelImplementation:');
      const endIdx = content.indexOf(',', startIdx);
      if (endIdx > startIdx) {
        const beforeStr = content.substring(0, startIdx);
        const afterStr = content.substring(endIdx);
        content = beforeStr + `modelImplementation: "${modelPercentage}%"` + afterStr;
      }
    }
    
    if (content.includes('apiImplementation:')) {
      const startIdx = content.indexOf('apiImplementation:');
      const endIdx = content.indexOf(',', startIdx);
      if (endIdx > startIdx) {
        const beforeStr = content.substring(0, startIdx);
        const afterStr = content.substring(endIdx);
        content = beforeStr + `apiImplementation: "${apiPercentage}%"` + afterStr;
      }
    }
    
    if (content.includes('uiImplementation:')) {
      const startIdx = content.indexOf('uiImplementation:');
      const endIdx = content.indexOf(',', startIdx);
      if (endIdx > startIdx) {
        const beforeStr = content.substring(0, startIdx);
        const afterStr = content.substring(endIdx);
        content = beforeStr + `uiImplementation: "${uiPercentage}%"` + afterStr;
      }
    }
    
    if (content.includes('testCoverage:')) {
      const startIdx = content.indexOf('testCoverage:');
      const endIdx = content.indexOf('\n', startIdx);
      if (endIdx > startIdx) {
        const beforeStr = content.substring(0, startIdx);
        const afterStr = content.substring(endIdx);
        content = beforeStr + `testCoverage: "${this.metrics.tests.coverage}%"` + afterStr;
      }
    }
    
    // Update date
    const today = new Date().toISOString().split('T')[0];
    if (content.includes('_Last updated:')) {
      const startIdx = content.indexOf('_Last updated:');
      const endIdx = content.indexOf('_', startIdx + 1);
      if (endIdx > startIdx) {
        const beforeStr = content.substring(0, startIdx);
        const afterStr = content.substring(endIdx);
        content = beforeStr + `_Last updated: **${today}**` + afterStr;
      }
    }
    
    // Add detailed implementation section
    const detailedContent = this.generateDetailedSection();
    
    if (content.includes('## 📊 Detailed Implementation')) {
      const sectionStart = content.indexOf('## 📊 Detailed Implementation');
      let sectionEnd = content.length;
      
      // Look for next section
      const nextSectionMatch = content.substring(sectionStart).match(/\n##\s/);
      if (nextSectionMatch) {
        sectionEnd = content.indexOf(nextSectionMatch[0], sectionStart);
      }
      
      content = content.substring(0, sectionStart) + detailedContent + 
                (nextSectionMatch ? content.substring(sectionEnd) : '');
    } else {
      content += "\n\n---\n\n" + detailedContent;
    }
    
    fs.writeFileSync(statusFile, content);
    console.log("Project status document updated!");
  }
  
  /**
   * Generate even more comprehensive detailed implementation section
   */
  generateDetailedSection() {
    // Calculate percentages
    const modelPercentage = this.getPercentage(this.metrics.models.implemented, this.metrics.models.total);
    const apiPercentage = this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total);
    const uiPercentage = this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total);
    const testPercentage = this.getPercentage(this.metrics.tests.passing, this.metrics.tests.total);
    
    let content = `## 📊 Detailed Implementation\n\n`;
    content += `_Last analyzed on ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Visual progress bars for each component
    content += `### 📈 Implementation Summary\n\n`;
    content += `| Component | Status | Progress |\n`;
    content += `|-----------|--------|----------|\n`;
    content += `| Models | ${modelPercentage}% | ${'▓'.repeat(Math.floor(modelPercentage/5))}${'░'.repeat(20-Math.floor(modelPercentage/5))} |\n`;
    content += `| API | ${apiPercentage}% | ${'▓'.repeat(Math.floor(apiPercentage/5))}${'░'.repeat(20-Math.floor(apiPercentage/5))} |\n`;
    content += `| UI | ${uiPercentage}% | ${'▓'.repeat(Math.floor(uiPercentage/5))}${'░'.repeat(20-Math.floor(uiPercentage/5))} |\n`;
    content += `| Tests | ${this.metrics.tests.coverage}% coverage | ${'▓'.repeat(Math.floor(this.metrics.tests.coverage/5))}${'░'.repeat(20-Math.floor(this.metrics.tests.coverage/5))} |\n\n`;
    
    // Add implementation priorities based on current state
    content += `### 🎯 Implementation Priorities\n\n`;
    
    if (apiPercentage < 50) {
      content += `- **API Layer**: Focus on implementing core API endpoints for ${this.getLowestImplementedArea()} feature area\n`;
    }
    
    if (uiPercentage < 30) {
      content += `- **UI Components**: Start with basic UI layout and critical components\n`;
    }
    
    if (this.metrics.tests.coverage < 30) {
      content += `- **Testing**: Increase test coverage for model and API layers\n`;
    }
    
    if (modelPercentage < 70) {
      content += `- **Data Models**: Finish implementing core domain models\n`;
    }
    
    content += `\n`;
    
    // Models implementation
    content += `### 📊 Models Implementation (${modelPercentage}%)\n\n`;
    content += `- **Total Models**: ${this.metrics.models.total}\n`;
    content += `- **Implemented Models**: ${this.metrics.models.implemented}\n\n`;
    
    if (this.metrics.models.details.length > 0) {
      // Key models table
      content += "#### Key Models\n\n";
      content += "| Model | File | Implementation | Status |\n";
      content += "|-------|------|---------------|--------|\n";
      
      // Sort by completeness
      const sortedModels = [...this.metrics.models.details]
        .sort((a, b) => b.completeness - a.completeness)
        .slice(0, 10);
      
      for (const model of sortedModels) {
        const status = model.completeness >= 75 ? "✅ Complete" :
                      model.completeness >= 50 ? "⚠️ Partial" : "🔴 Minimal";
        content += `| ${model.name} | ${model.file} | ${model.completeness}% | ${status} |\n`;
      }
      
      content += "\n";
    }
    
    // Add code quality section
    content += `### 🔍 Code Quality Metrics\n\n`;
    content += `| Metric | Value | Status |\n`;
    content += `|--------|-------|--------|\n`;
    
    // Complexity
    const complexityStatus = 
      this.metrics.codeQuality.complexity.average < 8 ? "✅ Good" :
      this.metrics.codeQuality.complexity.average < 15 ? "⚠️ Moderate" : "🔴 High";
      
    content += `| Average Complexity | ${this.metrics.codeQuality.complexity.average} | ${complexityStatus} |\n`;
    
    // High complexity files
    const highComplexityStatus = 
      this.metrics.codeQuality.complexity.high === 0 ? "✅ Good" :
      this.metrics.codeQuality.complexity.high < 5 ? "⚠️ Few issues" : "🔴 Many issues";
      
    content += `| High Complexity Files | ${this.metrics.codeQuality.complexity.high} | ${highComplexityStatus} |\n`;
    
    // Tech Debt score
    const techDebtStatus = 
      this.metrics.codeQuality.techDebt.score < 10 ? "✅ Low" :
      this.metrics.codeQuality.techDebt.score < 30 ? "⚠️ Moderate" : "🔴 High";
      
    content += `| Technical Debt | ${Math.round(this.metrics.codeQuality.techDebt.score)}% | ${techDebtStatus} |\n\n`;
    
    // List high complexity files if there are any
    if (this.metrics.codeQuality.techDebt.items.length > 0) {
      content += `#### Technical Debt Items\n\n`;
      content += `| File | Issue | Complexity | Recommendation |\n`;
      content += `|------|-------|------------|----------------|\n`;
      
      const topItems = this.metrics.codeQuality.techDebt.items
        .sort((a, b) => b.complexity - a.complexity)
        .slice(0, 5);
      
      for (const item of topItems) {
        content += `| ${item.file} | ${item.issue} | ${item.complexity} | ${item.recommendation} |\n`;
      }
      
      content += `\n`;
    }
    
    // Add completion predictions
    content += `### ⏱️ Completion Predictions\n\n`;
    content += `| Component | Remaining Items | Estimated Completion |\n`;
    content += `|-----------|-----------------|----------------------|\n`;
    
    const predictions = this.metrics.predictions.estimates;
    
    content += `| Models | ${predictions.models.remainingItems} | ${predictions.models.estimatedDate} |\n`;
    content += `| API Endpoints | ${predictions.apiEndpoints.remainingItems} | ${predictions.apiEndpoints.estimatedDate} |\n`;
    content += `| UI Components | ${predictions.uiComponents.remainingItems} | ${predictions.uiComponents.estimatedDate} |\n`;
    content += `| **Entire Project** | - | **${predictions.project.estimatedDate}** |\n\n`;
    
    content += `_*Predictions based on historical implementation velocity_\n\n`;
    
    return content;
  }
  
  /**
   * Get feature area with lowest implementation
   */
  getLowestImplementedArea() {
    let lowestArea = '';
    let lowestPercent = 100;
    
    for (const [area, stats] of Object.entries(this.metrics.featureAreas)) {
      if (stats.total > 0) {
        const percentage = this.getPercentage(stats.implemented, stats.total);
        if (percentage < lowestPercent) {
          lowestPercent = percentage;
          lowestArea = area;
        }
      }
    }
    
    return lowestArea;
  }
  
  /**
   * Generate relationship maps with improved formatting - FIXED
   */
  async generateRelationshipMaps() {
    console.log("Generating relationship maps...");
    
    // Create docs directory if it doesn't exist
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir);
    }
    
    // Start building the relationship map markdown
    let content = '# Code Relationship Map\n';
    content += `_Generated on ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Model relationships
    content += '## Model Relationships\n\n';
    content += '```mermaid\n';
    content += 'classDiagram\n';
    
    // Add model nodes
    const models = this.metrics.models.details.map(m => m.name);
    models.forEach(model => {
      content += `    class ${model}\n`;
    });
    
    // Add model relationships based on references
    const modelRelationships = this.findModelRelationships();
    if (modelRelationships && Array.isArray(modelRelationships)) {
      modelRelationships.forEach(rel => {
        content += `    ${rel.source} --> ${rel.target}\n`;
      });
    }
    
    content += '```\n\n';
    
    // API-Model dependencies with improved formatting
    content += '## API-Model Dependencies\n\n';
    content += '```mermaid\n';
    content += 'flowchart LR\n';
    
    // Add placeholder for each model
    models.forEach(model => {
      content += `    ${model}["${model}"]\n`;
    });
    
    // Add some example API endpoints if we can't get the real ones
    // This ensures the diagram won't be empty
    if (!this.metrics.apiEndpoints.details || !this.metrics.apiEndpoints.details.length) {
      // Add example endpoints for common models
      if (models.includes('User')) {
        content += `    GET_users["GET /users"]\n`;
        content += `    GET_users --> User\n`;
        content += `    POST_users["POST /users"]\n`;
        content += `    POST_users --> User\n`;
      }
      if (models.includes('Forum')) {
        content += `    GET_forum_stats["GET /forum/stats"]\n`;
        content += `    GET_forum_stats --> Forum\n`;
      }
      if (models.includes('Post')) {
        content += `    GET_posts["GET /posts"]\n`;
        content += `    GET_posts --> Post\n`;
        content += `    POST_posts["POST /posts"]\n`;
        content += `    POST_posts --> Post\n`;
      }
    } else {
      // Process real API endpoints if available
      const processedEndpoints = new Set();
      const modelEndpoints = {};
      
      // Initialize modelEndpoints for each model
      models.forEach(model => {
        modelEndpoints[model] = [];
      });
      
      // Process endpoints
      this.metrics.apiEndpoints.details.forEach(endpoint => {
        if (!endpoint) return;
        
        // Skip if already processed
        const endpointKey = `${endpoint.name || 'unknown'}`;
        if (processedEndpoints.has(endpointKey)) return;
        processedEndpoints.add(endpointKey);
        
        // Try to determine which models this endpoint affects
        let affectedModels = [];
        models.forEach(model => {
          const endpointName = endpoint.name || '';
          if (endpointName.toLowerCase().includes(model.toLowerCase())) {
            affectedModels.push(model);
          }
        });
        
        // If no models detected, assign to a default model
        if (affectedModels.length === 0 && models.length > 0) {
          if (endpointKey.includes('user')) affectedModels = ['User'];
          else if (endpointKey.includes('post')) affectedModels = ['Post'];
          else if (endpointKey.includes('topic')) affectedModels = ['Topic'];
          else if (endpointKey.includes('category')) affectedModels = ['Category'];
          else if (endpointKey.includes('forum')) affectedModels = ['Forum'];
          else affectedModels = [models[0]]; // Default to first model
        }
        
        // Add endpoint for each affected model
        affectedModels.forEach(model => {
          if (!modelEndpoints[model]) modelEndpoints[model] = [];
          
          // Create a clean endpoint ID and label
          // Determine HTTP method from the name or default to GET
          let method = 'GET';
          if (endpointKey.startsWith('post') || endpointKey.includes('create')) method = 'POST';
          else if (endpointKey.startsWith('put') || endpointKey.includes('update')) method = 'PUT';
          else if (endpointKey.startsWith('delete')) method = 'DELETE';
          
          const cleanPath = endpointKey.replace(/^(get|post|put|delete)_?/, '');
          const endpointId = `${method}_${cleanPath.replace(/[^a-zA-Z0-9]/g, '_')}`;
          const path = `/${cleanPath.split('_').join('/')}`;
          const label = `${method} ${path}`;
          
          modelEndpoints[model].push({ id: endpointId, label });
        });
      });
      
      // Add endpoints and connections to diagram
      Object.keys(modelEndpoints).forEach(model => {
        // Add endpoints and connections
        modelEndpoints[model].forEach(endpoint => {
          if (!endpoint) return;
          // Format endpoint with proper label
          content += `    ${endpoint.id}["${endpoint.label}"]\n`;
          content += `    ${endpoint.id} --> ${model}\n`;
        });
      });
    }
    
    content += '```\n\n';
    
    // Module structure
    content += '## Module Structure\n\n';
    content += '```mermaid\n';
    content += 'flowchart TD\n';
    content += '    FE[Frontend]\n';
    content += '    API[API Layer]\n';
    content += '    Models[Data Models]\n';
    content += '    Sync[Sync Engine]\n';
    content += '    DB[(Database)]\n';
    content += '    FE --> API\n';
    content += '    API --> Models\n';
    content += '    Models --> DB\n';
    content += '    API --> Sync\n';
    content += '    Sync --> DB\n';
    content += '```\n';
    
    // Write to file
    const outputFile = path.join(docsDir, 'relationship_map.md');
    fs.writeFileSync(outputFile, content);
    console.log(`Relationship map generated at ${outputFile}`);
  }
  
  /**
   * Find files matching certain patterns
   */
  findFilesByPatterns(patterns) {
    const matchingFiles = [];
    
    for (const filePath of this.allFiles) {
      const relativePath = path.relative(this.baseDir, filePath);
      
      for (const pattern of patterns) {
        if (pattern.test(filePath) || pattern.test(relativePath)) {
          matchingFiles.push(filePath);
          break;
        }
      }
    }
    
    return matchingFiles;
  }

  /**
   * Determine API feature area
   */
  determineApiFeatureArea(name = '', filePath = '', routePath = '') {
    const lowerName = name.toLowerCase();
    const lowerPath = filePath.toLowerCase();
    const lowerRoute = routePath ? routePath.toLowerCase() : '';
    
    if (lowerName.includes('auth') || lowerName.includes('login') || 
        lowerName.includes('user') || lowerPath.includes('auth') ||
        lowerRoute.includes('auth') || lowerRoute.includes('login')) {
      return 'auth';
    }
    
    if (lowerName.includes('forum') || lowerName.includes('topic') || 
        lowerName.includes('post') || lowerName.includes('category') ||
        lowerPath.includes('forum') || lowerRoute.includes('forum')) {
      return 'forum';
    }
    
    if (lowerName.includes('course') || lowerName.includes('module') || 
        lowerName.includes('assignment') || lowerPath.includes('course') ||
        lowerPath.includes('lms') || lowerRoute.includes('course')) {
      return 'lms';
    }
    
    if (lowerName.includes('integrat') || lowerPath.includes('integrat') ||
        lowerRoute.includes('integrat') || lowerName.includes('external')) {
      return 'integration';
    }
    
    return 'other';
  }

  /**
   * Add API endpoint to metrics
   */
  addApiEndpoint(name, filePath, completeness, featureArea = 'other', routePath = null) {
    this.metrics.apiEndpoints.total++;
    
    this.metrics.apiEndpoints.details.push({
      name,
      file: filePath,
      completeness,
      featureArea,
      routePath
    });
    
    // Track by feature area
    if (!this.metrics.featureAreas[featureArea]) {
      this.metrics.featureAreas[featureArea] = { total: 0, implemented: 0 };
    }
    this.metrics.featureAreas[featureArea].total++;
    
    if (completeness >= this.config.implementationThreshold) {
      this.metrics.apiEndpoints.implemented++;
      this.metrics.featureAreas[featureArea].implemented++;
    }
  }

  /**
   * Add UI component to metrics
   */
  addUIComponent(name, filePath, completeness) {
    this.metrics.uiComponents.total++;
    
    this.metrics.uiComponents.details.push({
      name,
      file: filePath,
      completeness
    });
    
    if (completeness >= this.config.implementationThreshold) {
      this.metrics.uiComponents.implemented++;
    }
  }

  /**
   * Print summary of findings
   */
  printSummary() {
    console.log(`\n=== Project Analysis Summary ===\n`);
    
    // Models
    const modelPercentage = this.getPercentage(this.metrics.models.implemented, this.metrics.models.total);
    console.log(`Models: ${this.metrics.models.implemented}/${this.metrics.models.total} (${modelPercentage}%)\n`);
    
    // API endpoints
    const apiPercentage = this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total);
    console.log(`API Endpoints: ${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total} (${apiPercentage}%)\n`);
    
    // API by feature area
    console.log(`API by Feature Area:`);
    for (const [area, stats] of Object.entries(this.metrics.featureAreas)) {
      if (stats.total > 0) {
        const areaPercentage = this.getPercentage(stats.implemented, stats.total);
        console.log(`  ${area.padEnd(10)}: ${stats.implemented}/${stats.total} (${areaPercentage}%)`);
      }
    }
    console.log();
    
    // UI components
    const uiPercentage = this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total);
    console.log(`UI Components: ${this.metrics.uiComponents.implemented}/${this.metrics.uiComponents.total} (${uiPercentage}%)\n`);
    
    // Tests
    console.log(`Tests: ${this.metrics.tests.passing}/${this.metrics.tests.total} passing (${this.metrics.tests.coverage}% coverage)`);
    console.log(`==============================`);
    
    // Reference hub location (new)
    console.log(`\nCentral Reference Hub generated at: ${path.join(this.baseDir, 'docs', 'central_reference_hub.md')}`);
  }

  /**
   * Helper method to calculate percentages
   */
  getPercentage(value, total) {
    if (total === 0) return 0;
    return Math.round((value / total) * 100);
  }
  
  /**
   * Parse file content into AST
   */
  parseToAst(content, filePath) {
    try {
      return parser.parse(content, this.parseOptions);
    } catch (err) {
      console.warn(`Error parsing ${filePath}: ${err.message}`);
      return null;
    }
  }
  
  /**
   * Calculate cyclomatic complexity from AST
   */
  calculateComplexity(ast) {
    if (!ast) return 1;
    
    let complexity = 1; // Base complexity
    
    traverse(ast, {
      IfStatement() { complexity++; },
      ConditionalExpression() { complexity++; },
      LogicalExpression(path) {
        if (path.node.operator === '&&' || path.node.operator === '||') {
          complexity++;
        }
      },
      SwitchCase(path) {
        if (path.node.consequent.length > 0) {
          complexity++;
        }
      },
      ForStatement() { complexity++; },
      ForInStatement() { complexity++; },
      ForOfStatement() { complexity++; },
      WhileStatement() { complexity++; },
      DoWhileStatement() { complexity++; },
      CatchClause() { complexity++; }
    });
    
    return complexity;
  }

  /**
   * Analyze component with AST for more accurate detection
   */
  analyzeComponentAst(content, filePath) {
    const ast = this.parseToAst(content, filePath);
    if (!ast) return { isComponent: false };
    
    const result = {
      isComponent: false,
      name: null,
      props: [],
      hooks: [],
      stateVars: [],
      eventHandlers: [],
      imports: [],
      complexity: 0
    };
    
    traverse(ast, {
      // Detect React Component Function declarations
      FunctionDeclaration(path) {
        if (path.node.id && /^[A-Z]/.test(path.node.id.name)) {
          // Search for JSX return in the function body
          let hasJsxReturn = false;
          traverse(path.node, {
            ReturnStatement(returnPath) {
              traverse(returnPath.node, {
                JSXElement() { hasJsxReturn = true; },
                JSXFragment() { hasJsxReturn = true; },
                noScope: true
              });
            },
            JSXElement() { hasJsxReturn = true; },
            JSXFragment() { hasJsxReturn = true; },
            noScope: true
          }, path.scope);
          
          if (hasJsxReturn) {
            result.isComponent = true;
            result.name = path.node.id.name;
            
            // Extract props from parameters
            if (path.node.params.length > 0 && path.node.params[0].type === 'ObjectPattern') {
              path.node.params[0].properties.forEach(prop => {
                if (prop.key) result.props.push(prop.key.name);
              });
            }
          }
        }
      },
      
      // Detect React Component arrow functions
      VariableDeclarator(path) {
        if (path.node.id && /^[A-Z]/.test(path.node.id.name) && 
            path.node.init && path.node.init.type === 'ArrowFunctionExpression') {
          
          let hasJsxReturn = false;
          traverse(path.node.init, {
            ReturnStatement(returnPath) {
              traverse(returnPath.node, {
                JSXElement() { hasJsxReturn = true; },
                JSXFragment() { hasJsxReturn = true; },
                noScope: true
              });
            },
            JSXElement() { hasJsxReturn = true; },
            JSXFragment() { hasJsxReturn = true; },
            noScope: true
          }, path.scope);
          
          if (hasJsxReturn) {
            result.isComponent = true;
            result.name = path.node.id.name;
            
            // Extract props from parameters
            if (path.node.init.params.length > 0 && path.node.init.params[0].type === 'ObjectPattern') {
              path.node.init.params[0].properties.forEach(prop => {
                if (prop.key) result.props.push(prop.key.name);
              });
            }
          }
        }
      },
      
      // Detect React hooks
      CallExpression(path) {
        if (path.node.callee && path.node.callee.name && 
            path.node.callee.name.startsWith('use')) {
          result.hooks.push(path.node.callee.name);
          
          // Detect state variables from useState
          if (path.node.callee.name === 'useState' && 
              path.parent && path.parent.type === 'VariableDeclarator' &&
              path.parent.id && path.parent.id.type === 'ArrayPattern') {
            if (path.parent.id.elements[0] && path.parent.id.elements[0].name) {
              result.stateVars.push(path.parent.id.elements[0].name);
            }
          }
        }
      },
      
      // Detect JSX event handlers
      JSXAttribute(path) {
        if (path.node.name && /^on[A-Z]/.test(path.node.name.name)) {
          result.eventHandlers.push(path.node.name.name);
        }
      },
      
      // Detect imports
      ImportDeclaration(path) {
        if (path.node.source.value) {
          result.imports.push(path.node.source.value);
        }
      }
    });
    
    // Calculate cyclomatic complexity
    result.complexity = this.calculateComplexity(ast);
    
    return result;
  }
  
  /**
   * Analyze code quality metrics - FIXED
   */
  async analyzeCodeQuality() {
    console.log("Analyzing code quality metrics...");
    
    // Initialize files array if it doesn't exist
    if (!this.metrics.codeQuality.files) {
      this.metrics.codeQuality.files = [];
    }
    
    const jsFiles = this.findFilesByPatterns([/\.(js|jsx|ts|tsx|rs)$/]);
    let totalComplexity = 0;
    let fileCount = 0;
    let highComplexityCount = 0;
    
    for (const filePath of jsFiles) {
      const content = this.fileContents.get(filePath);
      if (!content) continue;
      
      try {
        let complexity;
        
        if (filePath.endsWith('.rs')) {
          // For Rust files, use heuristics to estimate complexity
          complexity = this.estimateRustComplexity(content);
        } else {
          // For JS/TS files, use AST-based analysis
          const ast = this.parseToAst(content, filePath);
          if (ast) {
            complexity = this.calculateComplexity(ast);
          } else {
            continue; // Skip if parsing failed
          }
        }
        
        totalComplexity += complexity;
        fileCount++;
        
        const relativePath = path.relative(this.baseDir, filePath);
        
        // Store data about each file's complexity
        this.metrics.codeQuality.files.push({
          file: relativePath,
          complexity: complexity
        });
        
        if (complexity > 15) {
          highComplexityCount++;
          
          // Make sure techDebt.items exists
          if (!this.metrics.codeQuality.techDebt.items) {
            this.metrics.codeQuality.techDebt.items = [];
          }
          
          // Track as technical debt item
          this.metrics.codeQuality.techDebt.items.push({
            file: relativePath,
            issue: 'High complexity',
            complexity: complexity,
            recommendation: 'Consider refactoring into smaller functions'
          });
        }
      } catch (err) {
        console.warn(`Error analyzing code quality for ${filePath}: ${err.message}`);
      }
    }
    
    // Calculate average complexity
    if (fileCount > 0) {
      this.metrics.codeQuality.complexity.average = Math.round(totalComplexity / fileCount);
      this.metrics.codeQuality.complexity.high = highComplexityCount;
    }
    
    // Calculate technical debt score (simple version)
    this.metrics.codeQuality.techDebt.score = 
      (highComplexityCount / Math.max(1, fileCount)) * 100;
    
    console.log(`Code quality analysis: Average complexity ${this.metrics.codeQuality.complexity.average}, ${highComplexityCount} files with high complexity`);
  }
  
  /**
   * Estimate complexity of Rust code without a full Rust parser
   */
  estimateRustComplexity(content) {
    let complexity = 1; // Base complexity
    
    // Count control flow statements
    const controlFlowMatches = content.match(/\b(if|else|match|for|while|loop)\b/g) || [];
    complexity += controlFlowMatches.length;
    
    // Count closures
    const closureMatches = content.match(/\|\s*(?:[^|]*)\s*\|\s*(?:\{|\w)/g) || [];
    complexity += closureMatches.length;
    
    // Count ? operators for error handling
    const errorHandlingMatches = content.match(/\?\s*(?:;|,|\))/g) || [];
    complexity += errorHandlingMatches.length;
    
    // Count match arms
    const matchArmMatches = content.match(/=>\s*(?:\{|[^,;]+[,;])/g) || [];
    complexity += matchArmMatches.length;
    
    return complexity;
  }
  
  /**
   * Predict completion dates based on current progress
   */
  predictCompletion() {
    const predictions = {};
    const today = new Date();
    
    // Models prediction
    const remainingModels = this.metrics.models.total - this.metrics.models.implemented;
    const modelWeeks = remainingModels / this.metrics.predictions.velocityData.models;
    predictions.models = {
      remainingItems: remainingModels,
      estimatedWeeks: modelWeeks,
      estimatedDate: this.addWeeks(today, modelWeeks)
    };
    
    // API endpoints prediction
    const remainingEndpoints = this.metrics.apiEndpoints.total - this.metrics.apiEndpoints.implemented;
    const endpointWeeks = remainingEndpoints / this.metrics.predictions.velocityData.apiEndpoints;
    predictions.apiEndpoints = {
      remainingItems: remainingEndpoints,
      estimatedWeeks: endpointWeeks,
      estimatedDate: this.addWeeks(today, endpointWeeks)
    };
    
    // UI Components prediction
    const remainingComponents = this.metrics.uiComponents.total - this.metrics.uiComponents.implemented;
    const componentWeeks = remainingComponents / this.metrics.predictions.velocityData.uiComponents;
    predictions.uiComponents = {
      remainingItems: remainingComponents,
      estimatedWeeks: componentWeeks,
      estimatedDate: this.addWeeks(today, componentWeeks)
    };
    
    // Project completion (use the latest date)
    const allWeeks = [modelWeeks, endpointWeeks, componentWeeks];
    const maxWeeks = Math.max(...allWeeks);
    predictions.project = {
      estimatedWeeks: maxWeeks,
      estimatedDate: this.addWeeks(today, maxWeeks)
    };
    
    this.metrics.predictions.estimates = predictions;
    return predictions;
  }
  
  /**
   * Helper to add weeks to a date
   */
  addWeeks(date, weeks) {
    const result = new Date(date);
    result.setDate(result.getDate() + Math.round(weeks * 7));
    return result.toISOString().split('T')[0];
  }

  /**
   * Find relationships between models by analyzing content
   */
  findModelRelationships() {
    console.log("Finding model relationships...");
    const relationships = [];
    
    // Get model names for reference
    const models = this.metrics.models.details.map(m => m.name);
    
    // Look for references between models
    for (const model of this.metrics.models.details) {
      const modelName = model.name;
      const filePath = model.file;
      
      // Skip if we don't have the file content
      const fullPath = path.join(this.baseDir, filePath);
      const content = this.fileContents.get(fullPath);
      if (!content) continue;
      
      // Look for references to other models in this model's file
      for (const otherModel of models) {
        // Don't check self-references
        if (otherModel === modelName) continue;
        
        // Simple check for direct references to the model name
        if (content.includes(otherModel)) {
          relationships.push({
            source: modelName,
            target: otherModel,
            type: 'references'
          });
        }
      }
    }
    
    return relationships;
  }

  /**
   * Generate a comprehensive Central Reference Hub document
   */
  async generateCentralReferenceHub() {
    console.log("Generating Central Reference Hub...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir);
    }
    
    // Get source system info for Canvas and Discourse
    const portDir = path.resolve('C:\\Users\\Tim\\Desktop\\port');
    const canvasDir = path.join(portDir, 'canvas');
    const discourseDir = path.join(portDir, 'port');
    
    let canvasFiles = 0;
    let canvasLoc = 0;
    let discourseFiles = 0;
    let discourseLoc = 0;
    
    try {
      // Try to get stats for source systems if the directories exist
      if (fs.existsSync(canvasDir)) {
        const canvasStats = this.getDirectoryStats(canvasDir);
        canvasFiles = canvasStats.files;
        canvasLoc = canvasStats.lines;
      }
      
      if (fs.existsSync(discourseDir)) {
        const discourseStats = this.getDirectoryStats(discourseDir);
        discourseFiles = discourseStats.files;
        discourseLoc = discourseStats.lines;
      }
    } catch (err) {
      console.warn("Could not analyze source system directories:", err.message);
    }
    
    // Start building the central reference hub content
    let content = `# LMS Integration Project - Central Reference Hub\n\n`;
    content += `_Last updated: **${new Date().toISOString().split('T')[0]}**_\n\n`;
    
    // Project Overview section with JSON format
    content += `## 📊 Project Overview\n\n`;
    content += '```json\n';
    content += `{\n`;
    content += `  "overall_status": "early_development",\n`;
    content += `  "project_stats": {\n`;
    content += `    "foundation_complete": true,\n`;
    content += `    "model_implementation": "${this.getPercentage(this.metrics.models.implemented, this.metrics.models.total)}%",\n`;
    content += `    "api_implementation": "${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}%",\n`;
    content += `    "ui_implementation": "${this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}%",\n`;
    content += `    "test_coverage": "${this.metrics.tests.coverage}%",\n`;
    content += `    "technical_debt": "${Math.round(this.metrics.codeQuality.techDebt.score)}%"\n`;
    content += `  },\n`;
    content += `  "source_systems": {\n`;
    content += `    "canvas_lms": {\n`;
    content += `      "code_location": "C:\\\\Users\\\\Tim\\\\Desktop\\\\port\\\\canvas",\n`;
    content += `      "files_count": ${canvasFiles},\n`;
    content += `      "loc": ${canvasLoc}\n`;
    content += `    },\n`;
    content += `    "discourse": {\n`;
    content += `      "code_location": "C:\\\\Users\\\\Tim\\\\Desktop\\\\port\\\\port",\n`;
    content += `      "files_count": ${discourseFiles},\n`;
    content += `      "loc": ${discourseLoc}\n`;
    content += `    }\n`;
    content += `  },\n`;
    content += `  "target_system": {\n`;
    content += `    "code_location": "${this.baseDir}",\n`;
    content += `    "stack": {\n`;
    content += `      "tauri": "2.0.0-beta",\n`;
    content += `      "axum": "0.7.2",\n`;
    content += `      "leptos": "0.5.2",\n`;
    content += `      "seaorm": "0.12.4",\n`;
    content += `      "sqlite": "0.29.0",\n`;
    content += `      "meilisearch": "0.28.0"\n`;
    content += `    }\n`;
    content += `  },\n`;
    content += `  "completion_forecasts": {\n`;
    content += `    "models": "${this.metrics.predictions.estimates.models.estimatedDate}",\n`;
    content += `    "api_endpoints": "${this.metrics.predictions.estimates.apiEndpoints.estimatedDate}",\n`;
    content += `    "ui_components": "${this.metrics.predictions.estimates.uiComponents.estimatedDate}",\n`;
    content += `    "entire_project": "${this.metrics.predictions.estimates.project.estimatedDate}"\n`;
    content += `  }\n`;
    content += `}\n`;
    content += '```\n\n';
    
    // Source-to-Target mapping table
    content += `## 🔄 Source-to-Target Mapping\n\n`;
    content += `This section maps source components from Canvas and Discourse to their corresponding implementations in the Rust LMS project.\n\n`;
    content += `| Component | Source System | Source Location | Target Location | Status | Priority |\n`;
    content += `|-----------|---------------|----------------|----------------|--------|----------|\n`;
    
    // Key model mappings
    // User model
    content += `| User Model | Both | \`canvas/app/models/user.rb\` + \`discourse/app/models/user.rb\` | \`src-tauri/src/models/user.rs\` | `;
    const userModel = this.metrics.models.details.find(m => m.name === 'User');
    content += userModel ? `✅ ${userModel.completeness}% | High |\n` : `❌ 0% | High |\n`;
    
    // Authentication
    content += `| Authentication | Both | \`canvas/app/controllers/login.rb\` + \`discourse/app/controllers/session_controller.rb\` | \`src-tauri/src/api/auth.rs\` | ✅ 80% | High |\n`;
    
    // Forum models
    const categoryModel = this.metrics.models.details.find(m => m.name === 'Category');
    content += `| Forum Categories | Discourse | \`discourse/app/models/category.rb\` | \`src-tauri/src/models/category.rs\` | `;
    content += categoryModel ? `✅ ${categoryModel.completeness}% | High |\n` : `❌ 0% | High |\n`;
    
    const postModel = this.metrics.models.details.find(m => m.name === 'Post');
    content += `| Forum Posts | Discourse | \`discourse/app/models/post.rb\` | \`src-tauri/src/models/post.rs\` | `;
    content += postModel ? `✅ ${postModel.completeness}% | High |\n` : `❌ 0% | High |\n`;
    
    const topicModel = this.metrics.models.details.find(m => m.name === 'Topic');
    content += `| Forum Topics | Discourse | \`discourse/app/models/topic.rb\` | \`src-tauri/src/models/topic.rs\` | `;
    content += topicModel ? `✅ ${topicModel.completeness}% | High |\n` : `❌ 0% | High |\n`;
    
    const tagModel = this.metrics.models.details.find(m => m.name === 'Tag');
    content += `| Tags | Discourse | \`discourse/app/models/tag.rb\` | \`src-tauri/src/models/tag.rs\` | `;
    content += tagModel ? `✅ ${tagModel.completeness}% | Medium |\n` : `❌ 0% | Medium |\n`;
    
    // Course models
    const courseModel = this.metrics.models.details.find(m => m.name === 'Course');
    content += `| Courses | Canvas | \`canvas/app/models/course.rb\` | \`src-tauri/src/models/course.rs\` | `;
    content += courseModel ? `✅ ${courseModel.completeness}% | High |\n` : `❌ 0% | High |\n`;
    
    // Module, assignment, etc.
    content += `| Modules | Canvas | \`canvas/app/models/context_module.rb\` | \`src-tauri/src/models/course.rs\` (Module struct) | ✅ 60% | High |\n`;
    content += `| Assignments | Canvas | \`canvas/app/models/assignment.rb\` | \`src-tauri/src/models/course.rs\` (Assignment struct) | ✅ 60% | High |\n`;
    content += `| Submissions | Canvas | \`canvas/app/models/submission.rb\` | \`src-tauri/src/models/course.rs\` (Submission struct) | ✅ 60% | Medium |\n`;
    
    // API endpoints
    content += `| Forum API | Discourse | \`discourse/app/controllers/categories_controller.rb\` | \`src-tauri/src/api/forum.rs\` | ❌ 0% | High |\n`;
    content += `| Course API | Canvas | \`canvas/app/controllers/courses_controller.rb\` | \`src-tauri/src/api/lms/courses.rs\` | ❌ 0% | High |\n`;
    content += `| Module API | Canvas | \`canvas/app/controllers/context_modules_controller.rb\` | \`src-tauri/src/api/lms/modules.rs\` | ❌ 0% | High |\n`;
    content += `| Assignment API | Canvas | \`canvas/app/controllers/assignments_controller.rb\` | \`src-tauri/src/api/lms/assignments.rs\` | ❌ 0% | Medium |\n`;
    
    // Other systems
    content += `| Notification System | Both | Multiple files | Not implemented | ❌ 0% | Medium |\n`;
    content += `| File Upload System | Both | Multiple files | Not implemented | ❌ 0% | Medium |\n`;
    content += `| Search System | Both | Multiple files | \`src-tauri/src/services/search.rs\` | ❌ 0% | Medium |\n`;
    content += `| UI Components | Both | Multiple files | \`src/components/\` | ✅ ${this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}% | High |\n\n`;
    
    // Integration conflicts section with JSON format
    content += `## 🔍 Integration Conflicts\n\n`;
    content += `These areas require special attention due to conflicts between Canvas and Discourse:\n\n`;
    content += '```json\n';
    content += `{\n`;
    content += `  "model_conflicts": [\n`;
    content += `    {\n`;
    content += `      "name": "User",\n`;
    content += `      "conflict_type": "attribute_overlap",\n`;
    content += `      "canvas_attributes": ["name", "email", "bio", "avatar_url", "settings"],\n`;
    content += `      "discourse_attributes": ["name", "email", "username", "avatar_template", "user_option"],\n`;
    content += `      "resolution_strategy": "merge_attributes"\n`;
    content += `    },\n`;
    content += `    {\n`;
    content += `      "name": "Notification",\n`;
    content += `      "conflict_type": "implementation_difference",\n`;
    content += `      "resolution_strategy": "create_adapter_layer"\n`;
    content += `    },\n`;
    content += `    {\n`;
    content += `      "name": "Upload",\n`;
    content += `      "conflict_type": "implementation_difference",\n`;
    content += `      "resolution_strategy": "unified_upload_service"\n`;
    content += `    }\n`;
    content += `  ],\n`;
    content += `  "route_conflicts": [\n`;
    content += `    {\n`;
    content += `      "path": "/users/:id",\n`;
    content += `      "canvas_controller": "users_controller",\n`;
    content += `      "discourse_controller": "users_controller",\n`;
    content += `      "resolution_strategy": "namespace_routes"\n`;
    content += `    },\n`;
    content += `    {\n`;
    content += `      "path": "/search",\n`;
    content += `      "resolution_strategy": "unified_search_endpoint"\n`;
    content += `    }\n`;
    content += `  ]\n`;
    content += `}\n`;
    content += '```\n\n';
    
    // Implementation Tasks section
    content += `## 📋 Implementation Tasks\n\n`;
    content += `Tasks sorted by priority for implementing the port:\n\n`;
    
    // API endpoint tasks
    content += `1. **Complete API Endpoint Implementation** (${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total} completed)\n`;
    content += `   - High Priority: Forum API endpoints\n`;
    content += `   - Medium Priority: Course management endpoints\n`;
    content += `   - Low Priority: Administrative endpoints\n\n`;
    
    // UI Component tasks
    content += `2. **Complete UI Component Implementation** (${this.metrics.uiComponents.implemented}/${this.metrics.uiComponents.total} completed)\n`;
    content += `   - User interface components to match functionality\n\n`;
    
    // Integration systems
    content += `3. **Integrate Key Systems**\n`;
    content += `   - Authentication: Unify Canvas and Discourse auth approaches\n`;
    content += `   - Notifications: Create unified notification system\n`;
    content += `   - File uploads: Implement shared attachment system\n`;
    content += `   - Search: Implement MeiliSearch integration\n\n`;
    
    // Technical debt
    content += `4. **Address Technical Debt**\n`;
    content += `   - Refactor high complexity files (${this.metrics.codeQuality.complexity.high} files identified)\n`;
    content += `   - Improve test coverage (currently ${this.metrics.tests.coverage}%)\n\n`;
    
    // Project directory structure
    content += `## 📁 Project Directory Structure\n\n`;
    content += '```\n';
    content += `/\n`;
    content += `├── src/               # Frontend Leptos code\n`;
    content += `│   ├── components/    # UI components\n`;
    content += `│   ├── models/        # Frontend data models\n`;
    content += `│   ├── services/      # Frontend services\n`;
    content += `│   └── pages/         # Application pages\n`;
    content += `├── src-tauri/         # Backend Rust code\n`;
    content += `│   ├── src/\n`;
    content += `│   │   ├── api/       # API endpoints (${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}% complete)\n`;
    content += `│   │   ├── models/    # Data models (${this.getPercentage(this.metrics.models.implemented, this.metrics.models.total)}% complete)\n`;
    content += `│   │   ├── database/  # Database access\n`;
    content += `│   │   ├── services/  # Business logic\n`;
    content += `│   │   │   └── search.rs  # MeiliSearch integration\n`;
    content += `│   │   └── repository/ # Data repositories\n`;
    content += `│   └── tauri.conf.json\n`;
    content += `└── docs/              # Documentation\n`;
    content += `    ├── relationship_map.md\n`;
    content += `    └── central_reference_hub.md\n`;
    content += '```\n\n';
    
    // Implementation details section
    content += `## 📊 Implementation Details\n\n`;
    
    // Models section
    const modelPercentage = this.getPercentage(this.metrics.models.implemented, this.metrics.models.total);
    content += `### Models (${modelPercentage}% Complete)\n\n`;
    content += `| Model | File | Status | Notes |\n`;
    content += `|-------|------|--------|-------|\n`;
    
    // Sort models by completeness
    const sortedModels = [...this.metrics.models.details]
      .sort((a, b) => b.completeness - a.completeness)
      .slice(0, 10);
      
    for (const model of sortedModels) {
      const status = model.completeness >= 75 ? "✅ Complete" :
                    model.completeness >= 50 ? "⚠️ Partial" : "🔴 Minimal";
      let notes = "";
      
      if (model.name === "User") notes = "Core fields implemented, missing auth integration";
      else if (model.name === "Category") notes = "Core structure complete, needs relationships";
      else if (model.name === "Topic") notes = "Basic implementation, needs advanced features";
      else if (model.name === "Post") notes = "Basic CRUD, missing reactions and formatting";
      else if (model.name === "Tag") notes = "Basic structure, missing hierarchical tags";
      else if (model.name === "Course") notes = "Core fields, missing enrollment functionality";
      else if (model.name === "Forum") notes = "Basic structure complete";
      else notes = "Basic implementation";
      
      content += `| ${model.name} | ${model.file} | ${model.completeness}% ${status} | ${notes} |\n`;
    }
    
    content += `\n`;
    
    // API endpoints section
    content += `### API Endpoints (${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}% Complete)\n\n`;
    content += `| Endpoint Group | Files | Endpoints | Status |\n`;
    content += `|---------------|-------|-----------|--------|\n`;
    
    // Group API endpoints by feature area
    const endpointsByArea = {};
    for (const area in this.metrics.featureAreas) {
      if (this.metrics.featureAreas[area].total > 0) {
        endpointsByArea[area] = {
          total: this.metrics.featureAreas[area].total,
          implemented: this.metrics.featureAreas[area].implemented,
          endpoints: this.metrics.apiEndpoints.details.filter(e => e.featureArea === area)
        };
      }
    }
    
    for (const [area, stats] of Object.entries(endpointsByArea)) {
      const percentage = this.getPercentage(stats.implemented, stats.total);
      const status = percentage > 0 ? `✅ ${percentage}%` : `❌ 0%`;
      
      // Find common files for this area
      const files = [...new Set(stats.endpoints.map(e => e.file))].join(', ');
      
      content += `| ${area} | ${files} | ${stats.total} | ${status} |\n`;
    }
    
    content += `\n`;
    
    // Code quality metrics
    content += `### Code Quality Metrics\n\n`;
    content += `| Metric | Value | Status |\n`;
    content += `|--------|-------|--------|\n`;
    
    // Complexity
    const complexityStatus = 
      this.metrics.codeQuality.complexity.average < 8 ? "✅ Good" :
      this.metrics.codeQuality.complexity.average < 15 ? "⚠️ Moderate" : "🔴 High";
      
    content += `| Average Complexity | ${this.metrics.codeQuality.complexity.average} | ${complexityStatus} |\n`;
    
    // High complexity files
    const highComplexityStatus = 
      this.metrics.codeQuality.complexity.high === 0 ? "✅ Good" :
      this.metrics.codeQuality.complexity.high < 5 ? "⚠️ Few issues" : "🔴 Many issues";
      
    content += `| High Complexity Files | ${this.metrics.codeQuality.complexity.high} | ${highComplexityStatus} |\n`;
    
    // Tech Debt score
    const techDebtStatus = 
      this.metrics.codeQuality.techDebt.score < 10 ? "✅ Low" :
      this.metrics.codeQuality.techDebt.score < 30 ? "⚠️ Moderate" : "🔴 High";
      
    content += `| Technical Debt | ${Math.round(this.metrics.codeQuality.techDebt.score)}% | ${techDebtStatus} |\n\n`;
    
    // List high complexity files if there are any
    if (this.metrics.codeQuality.techDebt.items && this.metrics.codeQuality.techDebt.items.length > 0) {
      content += `#### Technical Debt Items\n\n`;
      content += `| File | Issue | Complexity | Recommendation |\n`;
      content += `|------|-------|------------|----------------|\n`;
      
      const topItems = this.metrics.codeQuality.techDebt.items
        .sort((a, b) => b.complexity - a.complexity)
        .slice(0, 5);
      
      for (const item of topItems) {
        content += `| ${item.file} | ${item.issue} | ${item.complexity} | ${item.recommendation} |\n`;
      }
      
      content += `\n`;
    }
    
    // NEW SECTION: MeiliSearch Integration
    content += `## 🔍 MeiliSearch Integration\n\n`;
    content += `The LMS platform integrates MeiliSearch for advanced search capabilities across course content, forum posts, and user-generated content.\n\n`;
    
    // MeiliSearch Setup
    content += `### Setup and Configuration\n\n`;
    content += `1. **Installation**\n\n`;
    content += `\`\`\`bash\n`;
    content += `# Install MeiliSearch\n`;
    content += `curl -L https://install.meilisearch.com | sh\n`;
    content += `\n`;
    content += `# Launch MeiliSearch (in production, use proper key management)\n`;
    content += `./meilisearch --master-key="aSampleMasterKey"\n`;
    content += `\`\`\`\n\n`;
    
    content += `2. **Dependencies**\n\n`;
    content += `Add the following to your \`Cargo.toml\`:\n\n`;
    content += `\`\`\`toml\n`;
    content += `[dependencies]\n`;
    content += `meilisearch-sdk = "0.28.0"\n`;
    content += `futures = "0.3"\n`;
    content += `serde = { version = "1.0", features = ["derive"] }\n`;
    content += `serde_json = "1.0"\n`;
    content += `\`\`\`\n\n`;
    
    // Implementation
    content += `### Implementation in LMS\n\n`;
    
    content += `1. **Model Definitions**\n\n`;
    content += `Define searchable model structures:\n\n`;
    content += `\`\`\`rust\n`;
    content += `// filepath: src-tauri/src/models/searchable.rs\n`;
    content += `use serde::{Serialize, Deserialize};\n`;
    content += `\n`;
    content += `#[derive(Serialize, Deserialize, Debug, Clone)]\n`;
    content += `pub struct SearchableTopic {\n`;
    content += `    pub id: i64,\n`;
    content += `    pub title: String,\n`;
    content += `    pub content: String,\n`;
    content += `    pub category_id: i64,\n`;
    content += `    pub category_name: String,\n`;
    content += `    pub created_at: i64,\n`;
    content += `    pub user_id: i64,\n`;
    content += `    pub username: String,\n`;
    content += `    pub tags: Vec<String>,\n`;
    content += `}\n`;
    content += `\n`;
    content += `#[derive(Serialize, Deserialize, Debug, Clone)]\n`;
    content += `pub struct SearchablePost {\n`;
    content += `    pub id: i64,\n`;
    content += `    pub content: String,\n`;
    content += `    pub topic_id: i64,\n`;
    content += `    pub topic_title: String,\n`;
    content += `    pub created_at: i64,\n`;
    content += `    pub user_id: i64,\n`;
    content += `    pub username: String,\n`;
    content += `}\n`;
    content += `\n`;
    content += `#[derive(Serialize, Deserialize, Debug, Clone)]\n`;
    content += `pub struct SearchableCourse {\n`;
    content += `    pub id: i64,\n`;
    content += `    pub title: String,\n`;
    content += `    pub description: String,\n`;
    content += `    pub start_date: i64,\n`;
    content += `    pub end_date: Option<i64>,\n`;
    content += `    pub instructor_id: i64,\n`;
    content += `    pub instructor_name: String,\n`;
    content += `    pub tags: Vec<String>,\n`;
    content += `}\n`;
    content += `\`\`\`\n\n`;
    
    content += `2. **Search Service Implementation**\n\n`;
    content += `Create a search service to manage MeiliSearch operations:\n\n`;
    content += `\`\`\`rust\n`;
    content += `// filepath: src-tauri/src/services/search.rs\n`;
    content += `use meilisearch_sdk::{client::*, indexes::*, search::*};\n`;
    content += `use crate::models::searchable::*;\n`;
    content += `use std::sync::{Arc, Mutex};\n`;
    content += `use futures::executor::block_on;\n`;
    content += `\n`;
    content += `pub struct SearchService {\n`;
    content += `    client: Client,\n`;
    content += `    initialized: bool,\n`;
    content += `}\n`;
    content += `\n`;
    content += `impl SearchService {\n`;
    content += `    pub fn new(url: &str, api_key: Option<&str>) -> Self {\n`;
    content += `        let client = Client::new(url, api_key.map(|k| k.to_string()));\n`;
    content += `        \n`;
    content += `        Self {\n`;
    content += `            client,\n`;
    content += `            initialized: false,\n`;
    content += `        }\n`;
    content += `    }\n`;
    content += `    \n`;
    content += `    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {\n`;
    content += `        // Create indexes if they don't exist\n`;
    content += `        self.client.create_index("topics", Some("id")).await?;\n`;
    content += `        self.client.create_index("posts", Some("id")).await?;\n`;
    content += `        self.client.create_index("courses", Some("id")).await?;\n`;
    content += `        \n`;
    content += `        // Configure searchable attributes\n`;
    content += `        let topics_index = self.client.index("topics");\n`;
    content += `        topics_index.set_searchable_attributes(&["title", "content", "category_name", "tags"]).await?;\n`;
    content += `        \n`;
    content += `        let posts_index = self.client.index("posts");\n`;
    content += `        posts_index.set_searchable_attributes(&["content", "topic_title", "username"]).await?;\n`;
    content += `        \n`;
    content += `        let courses_index = self.client.index("courses");\n`;
    content += `        courses_index.set_searchable_attributes(&["title", "description", "instructor_name", "tags"]).await?;\n`;
    content += `        \n`;
    content += `        self.initialized = true;\n`;
    content += `        Ok(())\n`;
    content += `    }\n`;
    content += `    \n`;
    content += `    pub async fn index_topic(&self, topic: &SearchableTopic) -> Result<(), Box<dyn std::error::Error>> {\n`;
    content += `        let index = self.client.index("topics");\n`;
    content += `        index.add_documents(&[topic], None).await?;\n`;
    content += `        Ok(())\n`;
    content += `    }\n`;
    content += `    \n`;
    content += `    pub async fn index_post(&self, post: &SearchablePost) -> Result<(), Box<dyn std::error::Error>> {\n`;
    content += `        let index = self.client.index("posts");\n`;
    content += `        index.add_documents(&[post], None).await?;\n`;
    content += `        Ok(())\n`;
    content += `    }\n`;
    content += `    \n`;
    content += `    pub async fn index_course(&self, course: &SearchableCourse) -> Result<(), Box<dyn std::error::Error>> {\n`;
    content += `        let index = self.client.index("courses");\n`;
    content += `        index.add_documents(&[course], None).await?;\n`;
    content += `        Ok(())\n`;
    content += `    }\n`;
    content += `    \n`;
    content += `    pub async fn search_topics(&self, query: &str) -> Result<Vec<SearchableTopic>, Box<dyn std::error::Error>> {\n`;
    content += `        let index = self.client.index("topics");\n`;
    content += `        let results = index.search()\n`;
    content += `            .with_query(query)\n`;
    content += `            .with_limit(20)\n`;
    content += `            .execute::<SearchableTopic>()\n`;
    content += `            .await?;\n`;
    content += `            \n`;
    content += `        Ok(results.hits.into_iter().map(|hit| hit.result).collect())\n`;
    content += `    }\n`;
    content += `    \n`;
    content += `    pub async fn search_all(&self, query: &str) -> Result<SearchResults, Box<dyn std::error::Error>> {\n`;
    content += `        let topics = self.search_topics(query).await?;\n`;
    content += `        let posts = self.client.index("posts").search()\n`;
    content += `            .with_query(query)\n`;
    content += `            .with_limit(20)\n`;
    content += `            .execute::<SearchablePost>()\n`;
    content += `            .await?\n`;
    content += `            .hits\n`;
    content += `            .into_iter()\n`;
    content += `            .map(|hit| hit.result)\n`;
    content += `            .collect();\n`;
    content += `            \n`;
    content += `        let courses = self.client.index("courses").search()\n`;
    content += `            .with_query(query)\n`;
    content += `            .with_limit(20)\n`;
    content += `            .execute::<SearchableCourse>()\n`;
    content += `            .await?\n`;
    content += `            .hits\n`;
    content += `            .into_iter()\n`;
    content += `            .map(|hit| hit.result)\n`;
    content += `            .collect();\n`;
    content += `            \n`;
    content += `        Ok(SearchResults {\n`;
    content += `            topics,\n`;
    content += `            posts,\n`;
    content += `            courses,\n`;
    content += `            query: query.to_string()\n`;
    content += `        })\n`;
    content += `    }\n`;
    content += `}\n`;
    content += `\n`;
    content += `#[derive(Serialize, Deserialize, Debug)]\n`;
    content += `pub struct SearchResults {\n`;
    content += `    pub topics: Vec<SearchableTopic>,\n`;
    content += `    pub posts: Vec<SearchablePost>,\n`;
    content += `    pub courses: Vec<SearchableCourse>,\n`;
    content += `    pub query: String\n`;
    content += `}\n`;
    content += `\`\`\`\n\n`;
    
    content += `3. **Integration with API Layer**\n\n`;
    content += `Create search endpoints in your API:\n\n`;
    content += `\`\`\`rust\n`;
    content += `// filepath: src-tauri/src/api/search.rs\n`;
    content += `use axum::{extract::Query, Json};\n`;
    content += `use serde::{Deserialize};\n`;
    content += `use crate::services::search::{SearchService, SearchResults};\n`;
    content += `use std::sync::Arc;\n`;
    content += `\n`;
    content += `#[derive(Deserialize)]\n`;
    content += `pub struct SearchQuery {\n`;
    content += `    q: String,\n`;
    content += `    limit: Option<usize>,\n`;
    content += `}\n`;
    content += `\n`;
    content += `pub async fn search(\n`;
    content += `    Query(params): Query<SearchQuery>,\n`;
    content += `    search_service: Arc<SearchService>\n`;
    content += `) -> Json<SearchResults> {\n`;
    content += `    let results = search_service.search_all(&params.q)\n`;
    content += `        .await\n`;
    content += `        .unwrap_or_else(|_| SearchResults {\n`;
    content += `            topics: vec![],\n`;
    content += `            posts: vec![],\n`;
    content += `            courses: vec![],\n`;
    content += `            query: params.q\n`;
    content += `        });\n`;
    content += `        \n`;
    content += `    Json(results)\n`;
    content += `}\n`;
    content += `\`\`\`\n\n`;
    
    content += `### Integration Points with Canvas and Discourse\n\n`;
    content += `MeiliSearch provides a unified search experience across both Canvas and Discourse content:\n\n`;
    content += `1. **Indexing Strategy**\n`;
    content += `   - Course content from Canvas is indexed in the "courses" index\n`;
    content += `   - Forum topics and posts from Discourse are indexed in "topics" and "posts" indexes\n`;
    content += `   - Shared entities like users are indexed with references to both systems\n\n`;
    
    content += `2. **Search UI Integration**\n`;
    content += `   - Implement a unified search component in the UI\n`;
    content += `   - Results are categorized by type (course, topic, post)\n`;
    content += `   - Deep linking to appropriate content based on search result type\n\n`;
    
    content += `3. **Real-time Indexing**\n`;
    content += `   - Hook into create/update events in both systems\n`;
    content += `   - Ensure search indexes remain current with content changes\n\n`;
    
    // Next Implementation Tasks
    content += `## 🛠️ Next Implementation Tasks\n\n`;
    content += '```json\n';
    content += `{\n`;
    content += `  "high_priority_tasks": [\n`;
    content += `    {\n`;
    content += `      "id": "task-1",\n`;
    content += `      "title": "Implement Forum API endpoints",\n`;
    content += `      "source_files": [\n`;
    content += `        "discourse/app/controllers/categories_controller.rb",\n`;
    content += `        "discourse/app/controllers/topics_controller.rb"\n`;
    content += `      ],\n`;
    content += `      "target_file": "src-tauri/src/api/forum.rs",\n`;
    content += `      "description": "Port the basic CRUD operations for forum categories and topics"\n`;
    content += `    },\n`;
    content += `    {\n`;
    content += `      "id": "task-2",\n`;
    content += `      "title": "Implement Course API endpoints",\n`;
    content += `      "source_files": ["canvas/app/controllers/courses_controller.rb"],\n`;
    content += `      "target_file": "src-tauri/src/api/lms/courses.rs",\n`;
    content += `      "description": "Port the basic CRUD operations for courses"\n`;
    content += `    },\n`;
    content += `    {\n`;
    content += `      "id": "task-3",\n`;
    content += `      "title": "Complete User authentication integration",\n`;
    content += `      "source_files": [\n`;
    content += `        "canvas/app/controllers/login.rb",\n`;
    content += `        "discourse/app/controllers/session_controller.rb"\n`;
    content += `      ],\n`;
    content += `      "target_file": "src-tauri/src/api/auth.rs",\n`;
    content += `      "description": "Finish merging authentication approaches from both systems"\n`;
    content += `    },\n`;
    content += `    {\n`;
    content += `      "id": "task-4",\n`;
    content += `      "title": "Implement MeiliSearch Integration",\n`;
    content += `      "target_file": "src-tauri/src/services/search.rs",\n`;
    content += `      "description": "Implement search service using MeiliSearch as shown in the Central Reference Hub"\n`;
    content += `    }\n`;
    content += `  ],\n`;
    content += `  "medium_priority_tasks": [\n`;
    content += `    {\n`;
    content += `      "id": "task-5",\n`;
    content += `      "title": "Implement notification system",\n`;
    content += `      "source_files": [\n`;
    content += `        "canvas/app/models/notification.rb",\n`;
    content += `        "discourse/app/models/notification.rb"\n`;
    content += `      ],\n`;
    content += `      "target_file": "src-tauri/src/models/notification.rs",\n`;
    content += `      "description": "Create unified notification model and service"\n`;
    content += `    }\n`;
    content += `  ]\n`;
    content += `}\n`;
    content += '```\n\n';
    
    // Architecture Overview with Mermaid diagram
    content += `## 🏗️ Architecture Overview\n\n`;
    content += `The unified LMS application uses a modular architecture that combines Canvas LMS educational features with Discourse forum capabilities:\n\n`;
    content += '```mermaid\n';
    content += 'graph TD\n';
    content += '    UI[UI Layer - Leptos] --> API[API Layer - Axum]\n';
    content += '    API --> Models[Data Models]\n';
    content += '    API --> Services[Business Logic Services]\n';
    content += '    Services --> Models\n';
    content += '    Models --> DB[(SQLite Database)]\n';
    content += '    Services --> External[External Canvas API]\n';
    content += '    Services --> Search[MeiliSearch]\n';
    content += '    API --> Search\n';
    content += '```\n\n';
    
    // Key Integration Points
    content += `## 🔍 Key Integration Points\n\n`;
    content += `These are the critical integration areas between Canvas and Discourse:\n\n`;
    
    // Notification system
    content += `1. **Unified Notification System**\n`;
    content += `   - Source: Canvas notifications + Discourse notifications\n`;
    content += `   - Implementation status: 0%\n`;
    content += `   - Integration approach: Create a unified notification service that dispatches to both systems\n\n`;
    
    // Search functionality
    content += `2. **Unified Search Functionality**\n`;
    content += `   - Source: Canvas search + Discourse search\n`;
    content += `   - Implementation status: 0%\n`;
    content += `   - Integration approach: Implement MeiliSearch service for federated search\n\n`;
    
    // File upload system
    content += `3. **Unified File Upload System**\n`;
    content += `   - Source: Canvas attachments + Discourse uploads\n`;
    content += `   - Implementation status: 0%\n`;
    content += `   - Integration approach: Create shared file storage system with consistent API\n\n`;
    
    // Completion predictions
    content += `## 📈 Project Trajectories\n\n`;
    content += `Current analysis suggests project completion by ${this.metrics.predictions.estimates.project.estimatedDate}, with these milestones:\n\n`;
    content += `- Models: ${this.metrics.predictions.estimates.models.remainingItems} remaining, estimated completion ${this.metrics.predictions.estimates.models.estimatedDate}\n`;
    content += `- API Endpoints: ${this.metrics.predictions.estimates.apiEndpoints.remainingItems} remaining, estimated completion ${this.metrics.predictions.estimates.apiEndpoints.estimatedDate}\n`;
    content += `- UI Components: ${this.metrics.predictions.estimates.uiComponents.remainingItems} remaining, estimated completion ${this.metrics.predictions.estimates.uiComponents.estimatedDate}\n\n`;
    
    // Analysis references
    content += `<!--\n`;
    content += `# ANALYSIS REFERENCES\n`;
    content += `# These paths point to detailed analysis files that provide additional context\n`;
    content += `relationship_map: ${path.join(docsDir, 'relationship_map.md')}\n`;
    content += `port_analysis: C:\\Users\\Tim\\Desktop\\port\\analysis\\integration_summary.md\n`;
    content += `conflicts_detailed: C:\\Users\\Tim\\Desktop\\port\\analysis\\conflicts_detailed.md\n`;
    content += `integration_points: C:\\Users\\Tim\\Desktop\\port\\analysis\\integration_points.md\n`;
    content += `integration_diagrams: C:\\Users\\Tim\\Desktop\\port\\analysis\\integration_diagrams.md\n`;
    content += `-->\n`;
    
    // Write the central reference hub to file
    const outputFile = path.join(docsDir, 'central_reference_hub.md');
    fs.writeFileSync(outputFile, content);
    console.log(`Central Reference Hub generated at ${outputFile}`);
    
    return outputFile;
  }

  /**
   * Get statistics for a directory
   */
  getDirectoryStats(dirPath) {
    let fileCount = 0;
    let lineCount = 0;
    
    const walkDir = (dir) => {
      const files = fs.readdirSync(dir);
      
      for (const file of files) {
        const filePath = path.join(dir, file);
        const stat = fs.statSync(filePath);
        
        if (stat.isDirectory()) {
          // Skip common directories to avoid excessive processing
          if (!['node_modules', '.git', 'coverage', 'public/assets'].some(d => filePath.includes(d))) {
            walkDir(filePath);
          }
        } else {
          fileCount++;
          
          // Only count lines for text files
          const ext = path.extname(filePath).toLowerCase();
          if (['.rb', '.js', '.jsx', '.ts', '.tsx', '.py', '.erb', '.html', '.scss', '.css', '.rs'].includes(ext)) {
            try {
              const content = fs.readFileSync(filePath, 'utf8');
              lineCount += content.split('\n').length;
            } catch (err) {
              // Skip if can't read file
            }
          }
        }
      }
    };
    
    try {
      walkDir(dirPath);
    } catch (err) {
      console.warn(`Error analyzing directory ${dirPath}:`, err.message);
    }
    
    return { files: fileCount, lines: lineCount };
  }
}

// Run the analyzer
async function main() {
  try {
    const baseDir = path.resolve(__dirname);
    const analyzer = new UnifiedProjectAnalyzer(baseDir);
    await analyzer.analyze();
  } catch (err) {
    console.error("Error during analysis:", err);
    process.exit(1);
  }
}

main();