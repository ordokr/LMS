/**
 * Technical Documentation Generator
 * Analyzes source code to generate technical implementation details
 */
const fs = require('fs');
const path = require('path');
const glob = require('glob');

class TechnicalDocsGenerator {
  /**
   * Create a new technical documentation generator
   * @param {Object} options - Configuration options
   * @param {string} options.baseDir - Base directory for the project
   * @param {string} options.outputDir - Output directory for documentation
   * @param {Array} options.sourcePatterns - Glob patterns for source files to analyze
   */
  constructor(options = {}) {
    this.options = Object.assign({
      baseDir: process.cwd(),
      outputDir: path.join(process.cwd(), 'rag_knowledge_base', 'integration'),
      sourcePatterns: [
        'services/integration/**/*.js',
        'services/integration/**/*.ts',
        'controllers/integration/**/*.js',
        'controllers/integration/**/*.ts',
        'models/integration/**/*.js', 
        'models/integration/**/*.ts',
        'plugins/discourse/**/*.rb',
        'plugins/discourse/**/*.js',
        'plugins/canvas/**/*.rb',
        'plugins/canvas/**/*.js'
      ]
    }, options);
  }

  /**
   * Generate technical documentation
   */
  async generate() {
    console.log("Generating technical implementation documentation...");
    
    try {
      // Ensure output directory exists
      if (!fs.existsSync(this.options.outputDir)) {
        fs.mkdirSync(this.options.outputDir, { recursive: true });
      }
      
      // Find all relevant source files
      const sourceFiles = await this.findSourceFiles();
      console.log(`Found ${sourceFiles.length} relevant source files`);
      
      // Extract implementation details from source files
      const implementations = await this.extractImplementationDetails(sourceFiles);
      
      // Generate documentation by component
      await this.generateComponentDocs(implementations);
      
      // Create main technical documentation file
      await this.generateMainDocument(implementations);
      
      console.log("Technical implementation documentation generated successfully");
      return path.join(this.options.outputDir, 'technical_implementation.md');
    } catch (error) {
      console.error("Failed to generate technical documentation:", error);
      throw error;
    }
  }
  
  /**
   * Find all source files matching the patterns
   * @returns {Array<string>} Array of file paths
   */
  async findSourceFiles() {
    const files = [];
    
    for (const pattern of this.options.sourcePatterns) {
      const matches = glob.sync(pattern, {
        cwd: this.options.baseDir,
        absolute: true
      });
      
      files.push(...matches);
    }
    
    return files;
  }
  
  /**
   * Extract implementation details from source files
   * @param {Array<string>} files - Array of file paths
   * @returns {Object} Implementation details grouped by component
   */
  async extractImplementationDetails(files) {
    const implementations = {
      authentication: {
        files: [],
        classes: [],
        functions: [],
        schema: []
      },
      modelMapping: {
        files: [],
        classes: [],
        functions: [],
        schema: []
      },
      apiIntegration: {
        files: [],
        classes: [],
        functions: [],
        schema: []
      },
      synchronization: {
        files: [],
        classes: [],
        functions: [],
        schema: []
      }
    };
    
    for (const file of files) {
      try {
        const content = fs.readFileSync(file, 'utf8');
        const component = this.classifyComponent(file, content);
        if (!component) continue;
        
        const fileInfo = {
          path: path.relative(this.options.baseDir, file),
          language: this.getFileLanguage(file),
          size: content.length,
          lastModified: fs.statSync(file).mtime
        };
        
        implementations[component].files.push(fileInfo);
        
        // Extract classes, functions, and schemas
        const classes = this.extractClasses(content, fileInfo.language);
        const functions = this.extractFunctions(content, fileInfo.language);
        const schema = this.extractSchema(content, fileInfo.language);
        
        implementations[component].classes.push(...classes);
        implementations[component].functions.push(...functions);
        implementations[component].schema.push(...schema);
      } catch (error) {
        console.warn(`Error processing ${file}:`, error.message);
      }
    }
    
    return implementations;
  }
  
  /**
   * Classify file into component based on path and content
   * @param {string} filePath - Path to the file
   * @param {string} content - File content
   * @returns {string|null} Component name or null if not relevant
   */
  classifyComponent(filePath, content) {
    const relativePath = path.relative(this.options.baseDir, filePath);
    const lowerContent = content.toLowerCase();
    
    // Check path-based classification
    if (/auth|sso|login|jwt|oauth/i.test(relativePath)) {
      return 'authentication';
    }
    
    if (/model|mapping|entity|schema/i.test(relativePath)) {
      return 'modelMapping';
    }
    
    if (/api|client|fetch|request|endpoint/i.test(relativePath)) {
      return 'apiIntegration';
    }
    
    if (/sync|job|queue|background|webhook/i.test(relativePath)) {
      return 'synchronization';
    }
    
    // Content-based classification as fallback
    if (/authentication|sso|jwt|token|login|sign[\s-]?on/i.test(lowerContent)) {
      return 'authentication';
    }
    
    if (/mapping|entity|relationship|course.+category|discussion.+topic/i.test(lowerContent)) {
      return 'modelMapping';
    }
    
    if (/api|client|fetch|axios|request\.get|request\.post/i.test(lowerContent)) {
      return 'apiIntegration';
    }
    
    if (/sync|job|queue|bull|sidekiq|background|webhook/i.test(lowerContent)) {
      return 'synchronization';
    }
    
    return null; // Not relevant to integration components
  }
  
  /**
   * Get file language based on extension
   * @param {string} filePath - Path to the file
   * @returns {string} Programming language
   */
  getFileLanguage(filePath) {
    const ext = path.extname(filePath).toLowerCase();
    
    switch (ext) {
      case '.js': return 'javascript';
      case '.jsx': return 'javascript';
      case '.ts': return 'typescript';
      case '.tsx': return 'typescript';
      case '.rb': return 'ruby';
      case '.py': return 'python';
      case '.java': return 'java';
      case '.php': return 'php';
      case '.sql': return 'sql';
      default: return 'unknown';
    }
  }
  
  /**
   * Extract classes from source code
   * @param {string} content - File content
   * @param {string} language - Programming language
   * @returns {Array} Extracted classes
   */
  extractClasses(content, language) {
    const classes = [];
    
    try {
      switch (language) {
        case 'javascript':
        case 'typescript':
          // Extract ES6 classes
          const classRegex = /class\s+([A-Za-z0-9_]+)(?:\s+extends\s+([A-Za-z0-9_]+))?\s*{([^}]*)}/g;
          let match;
          while ((match = classRegex.exec(content)) !== null) {
            classes.push({
              name: match[1],
              extends: match[2] || null,
              methods: this.extractMethodsFromClass(match[3]),
              source: match[0].substring(0, 500) + (match[0].length > 500 ? '...' : '')
            });
          }
          break;
          
        case 'ruby':
          // Extract Ruby classes
          const rubyClassRegex = /class\s+([A-Za-z0-9_:]+)(?:\s+<\s+([A-Za-z0-9_:]+))?\b(.*?)(?:end|^\s*class\s+[A-Za-z0-9_:]+)/gms;
          let rubyMatch;
          while ((rubyMatch = rubyClassRegex.exec(content)) !== null) {
            classes.push({
              name: rubyMatch[1],
              extends: rubyMatch[2] || null,
              methods: this.extractMethodsFromRubyClass(rubyMatch[3] || ''),
              source: rubyMatch[0].substring(0, 500) + (rubyMatch[0].length > 500 ? '...' : '')
            });
          }
          break;
      }
    } catch (error) {
      console.warn("Error extracting classes:", error.message);
    }
    
    return classes;
  }
  
  /**
   * Extract methods from a JavaScript/TypeScript class body
   * @param {string} classBody - Class body text
   * @returns {Array} Extracted methods
   */
  extractMethodsFromClass(classBody) {
    const methods = [];
    
    // Match both regular methods and async methods
    const methodRegex = /(async\s+)?([A-Za-z0-9_]+)\s*\(([^)]*)\)\s*{/g;
    let match;
    
    while ((match = methodRegex.exec(classBody)) !== null) {
      methods.push({
        name: match[2],
        isAsync: !!match[1],
        parameters: match[3].split(',').map(p => p.trim()).filter(p => p),
      });
    }
    
    return methods;
  }
  
  /**
   * Extract methods from a Ruby class body
   * @param {string} classBody - Class body text
   * @returns {Array} Extracted methods
   */
  extractMethodsFromRubyClass(classBody) {
    const methods = [];
    
    // Match Ruby methods
    const methodRegex = /def\s+([A-Za-z0-9_?!]+)(?:\((.*?)\))?/g;
    let match;
    
    while ((match = methodRegex.exec(classBody)) !== null) {
      methods.push({
        name: match[1],
        parameters: match[2] ? match[2].split(',').map(p => p.trim()).filter(p => p) : []
      });
    }
    
    return methods;
  }
  
  /**
   * Extract functions from source code
   * @param {string} content - File content
   * @param {string} language - Programming language
   * @returns {Array} Extracted functions
   */
  extractFunctions(content, language) {
    const functions = [];
    
    try {
      switch (language) {
        case 'javascript':
        case 'typescript':
          // Match function declarations
          const funcRegex = /(?:function\s+([A-Za-z0-9_]+)|(?:const|let|var)\s+([A-Za-z0-9_]+)\s*=\s*(?:async\s*)?\([^)]*\)\s*=>|(?:const|let|var)\s+([A-Za-z0-9_]+)\s*=\s*function(?:\s*|\s+[A-Za-z0-9_]+)?\s*\([^)]*\))\s*{/g;
          let match;
          while ((match = funcRegex.exec(content)) !== null) {
            const name = match[1] || match[2] || match[3];
            if (name) {
              // Extract the context (line before and after)
              const startPos = Math.max(0, match.index - 100);
              const endPos = Math.min(content.length, match.index + match[0].length + 200);
              const context = content.substring(startPos, endPos);
              
              // Check if it's integration-related
              if (this.isIntegrationRelated(name, context)) {
                functions.push({
                  name,
                  source: context,
                  isAsync: /async/.test(match[0])
                });
              }
            }
          }
          break;
          
        case 'ruby':
          // Match Ruby methods outside of classes
          const rubyFuncRegex = /def\s+([A-Za-z0-9_?!]+)(?:\((.*?)\))?(.*?)end/gms;
          let rubyMatch;
          while ((rubyMatch = rubyFuncRegex.exec(content)) !== null) {
            const name = rubyMatch[1];
            if (name && this.isIntegrationRelated(name, rubyMatch[0])) {
              functions.push({
                name,
                parameters: rubyMatch[2] ? rubyMatch[2].split(',').map(p => p.trim()).filter(p => p) : [],
                source: rubyMatch[0].substring(0, 500) + (rubyMatch[0].length > 500 ? '...' : '')
              });
            }
          }
          break;
      }
    } catch (error) {
      console.warn("Error extracting functions:", error.message);
    }
    
    return functions;
  }
  
  /**
   * Check if function/method is integration related
   * @param {string} name - Function name
   * @param {string} context - Function context
   * @returns {boolean} Is integration related
   */
  isIntegrationRelated(name, context) {
    const integrationKeywords = [
      'canvas', 'discourse', 'sync', 'integration',
      'sso', 'auth', 'token', 'jwt', 'oauth',
      'category', 'course', 'topic', 'discussion',
      'mapping', 'convert', 'transform'
    ];
    
    const nameLower = name.toLowerCase();
    const contextLower = context.toLowerCase();
    
    // Check if name contains integration keywords
    for (const keyword of integrationKeywords) {
      if (nameLower.includes(keyword)) return true;
    }
    
    // Check if context contains multiple integration keywords
    let keywordCount = 0;
    for (const keyword of integrationKeywords) {
      if (contextLower.includes(keyword)) keywordCount++;
      if (keywordCount >= 2) return true; // At least two keywords in context
    }
    
    return false;
  }
  
  /**
   * Extract schema definitions from source code
   * @param {string} content - File content
   * @param {string} language - Programming language
   * @returns {Array} Extracted schema definitions
   */
  extractSchema(content, language) {
    const schemas = [];
    
    try {
      // Schema from SQL statements
      const createTableRegex = /CREATE\s+TABLE\s+(\w+)\s*\(([\s\S]*?)\);/gi;
      let match;
      
      while ((match = createTableRegex.exec(content)) !== null) {
        schemas.push({
          type: 'table',
          name: match[1],
          sql: match[0]
        });
      }
      
      // Schema from JavaScript/TypeScript models
      if (language === 'javascript' || language === 'typescript') {
        // Mongoose schema
        const mongooseRegex = /new\s+mongoose\.Schema\s*\(\s*({[\s\S]*?})\s*\)/g;
        let mongooseMatch;
        
        while ((mongooseMatch = mongooseRegex.exec(content)) !== null) {
          schemas.push({
            type: 'mongoose',
            schema: mongooseMatch[1]
          });
        }
        
        // Sequelize model
        const sequelizeRegex = /([A-Za-z0-9_]+)\.define\s*\(\s*['"]([A-Za-z0-9_]+)['"]\s*,\s*({[\s\S]*?})\s*[,\)]/g;
        let sequelizeMatch;
        
        while ((sequelizeMatch = sequelizeRegex.exec(content)) !== null) {
          schemas.push({
            type: 'sequelize',
            name: sequelizeMatch[2],
            schema: sequelizeMatch[3]
          });
        }
      }
      
      // Schema from Ruby models (ActiveRecord)
      if (language === 'ruby') {
        const rubySchemaRegex = /create_table\s+:(\w+)(?:,.*?)?\s+do\s+\|t\|([\s\S]*?)end/g;
        let rubyMatch;
        
        while ((rubyMatch = rubySchemaRegex.exec(content)) !== null) {
          schemas.push({
            type: 'activerecord',
            name: rubyMatch[1],
            schema: rubyMatch[2]
          });
        }
      }
    } catch (error) {
      console.warn("Error extracting schema:", error.message);
    }
    
    return schemas;
  }
  
  /**
   * Generate documentation files for each component
   * @param {Object} implementations - Implementation details by component
   */
  async generateComponentDocs(implementations) {
    const componentMap = {
      authentication: 'Authentication',
      modelMapping: 'Model Mapping',
      apiIntegration: 'API Integration',
      synchronization: 'Synchronization'
    };
    
    for (const [key, label] of Object.entries(componentMap)) {
      const component = implementations[key];
      if (!component || component.files.length === 0) continue;
      
      let content = `# ${label} Technical Implementation\n\n`;
      content += `Generated on: ${new Date().toISOString().split('T')[0]}\n\n`;
      content += `## Overview\n\n`;
      content += `This document details the technical implementation of ${label.toLowerCase()} for the Canvas-Discourse integration.\n\n`;
      
      // Add files section
      content += `## Implementation Files\n\n`;
      for (const file of component.files) {
        content += `- \`${file.path}\` (${file.language}, last modified: ${file.lastModified.toISOString().split('T')[0]})\n`;
      }
      
      // Add classes section
      if (component.classes.length > 0) {
        content += `\n## Classes\n\n`;
        for (const cls of component.classes) {
          content += `### ${cls.name}\n\n`;
          if (cls.extends) {
            content += `Extends: \`${cls.extends}\`\n\n`;
          }
          
          if (cls.methods && cls.methods.length > 0) {
            content += `Methods:\n\n`;
            for (const method of cls.methods) {
              const params = method.parameters.join(', ');
              content += `- \`${method.isAsync ? 'async ' : ''}${method.name}(${params})\`\n`;
            }
            content += '\n';
          }
          
          content += `\`\`\`${cls.source.language}\n${cls.source}\n\`\`\`\n\n`;
        }
      }
      
      // Add functions section
      if (component.functions.length > 0) {
        content += `\n## Functions\n\n`;
        for (const func of component.functions) {
          content += `### ${func.name}\n\n`;
          if (func.isAsync) {
            content += `*Async function*\n\n`;
          }
          
          content += `\`\`\`${func.language || 'javascript'}\n${func.source}\n\`\`\`\n\n`;
        }
      }
      
      // Add schema section
      if (component.schema.length > 0) {
        content += `\n## Data Schema\n\n`;
        for (const schema of component.schema) {
          content += `### ${schema.name || 'Schema'}\n\n`;
          content += `Type: ${schema.type}\n\n`;
          content += `\`\`\`${schema.type === 'table' ? 'sql' : schema.type === 'activerecord' ? 'ruby' : 'javascript'}\n${schema.sql || schema.schema}\n\`\`\`\n\n`;
        }
      }
      
      // Write to file
      const outputPath = path.join(this.options.outputDir, `${key}_implementation.md`);
      fs.writeFileSync(outputPath, content);
    }
  }
  
  /**
   * Generate main technical documentation file
   * @param {Object} implementations - Implementation details by component
   */
  async generateMainDocument(implementations) {
    let content = `# Canvas-Discourse Technical Implementation Details\n\n`;
    content += `Generated on: ${new Date().toISOString().split('T')[0]}\n\n`;
    
    content += `## Overview\n\n`;
    content += `This document provides detailed technical specifications for implementing the Canvas-Discourse integration. It covers implementation patterns, data flow, and specific code approaches.\n\n`;
    
    // Summary of component status
    content += `## Implementation Status\n\n`;
    content += `| Component | Files | Classes | Functions | Schemas |\n`;
    content += `|-----------|-------|---------|-----------|--------|\n`;
    
    const componentMap = {
      authentication: 'Authentication',
      modelMapping: 'Model Mapping',
      apiIntegration: 'API Integration',
      synchronization: 'Synchronization'
    };
    
    for (const [key, label] of Object.entries(componentMap)) {
      const component = implementations[key];
      content += `| ${label} | ${component.files.length} | ${component.classes.length} | ${component.functions.length} | ${component.schema.length} |\n`;
    }
    content += '\n';
    
    // Authentication section
    content += `## Authentication Implementation\n\n`;
    if (implementations.authentication.functions.length > 0 || implementations.authentication.classes.length > 0) {
      // Include one example function or class
      const exampleItem = implementations.authentication.functions[0] || implementations.authentication.classes[0];
      if (exampleItem) {
        content += `### ${exampleItem.name || 'Authentication Implementation'}\n\n`;
        content += `\`\`\`${exampleItem.language || 'javascript'}\n${exampleItem.source}\n\`\`\`\n\n`;
      }
      
      content += `For complete authentication implementation details, see [Authentication Technical Implementation](authentication_implementation.md)\n\n`;
    } else {
      content += `Authentication implementation is still in planning phase. This section will be updated as code is developed.\n\n`;
      
      // Include placeholder for planned JWT authentication
      content += `### JWT-based Single Sign-On (Planned)\n\n`;
      content += `\`\`\`javascript\n// Sample code for Canvas-side JWT generation\nfunction generateDiscourseSSO(user) {\n  const payload = {\n    user_id: user.id,\n    email: user.email,\n    name: user.display_name,\n    external_id: \`canvas_\${user.id}\`,\n    admin: user.admin,\n    roles: user.roles.map(r => r.name).join(',')\n  };\n  \n  const token = jwt.sign(payload, DISCOURSE_SSO_SECRET, {\n    expiresIn: '1h',\n    audience: 'discourse',\n    issuer: 'canvas-lms'\n  });\n  \n  return token;\n}\n\`\`\`\n\n`;
    }
    
    // Model Mapping section
    content += `## Model Synchronization\n\n`;
    if (implementations.modelMapping.schema.length > 0) {
      const exampleSchema = implementations.modelMapping.schema[0];
      content += `### ${exampleSchema.name || 'Data Schema'}\n\n`;
      content += `\`\`\`${exampleSchema.type === 'table' ? 'sql' : 'javascript'}\n${exampleSchema.sql || exampleSchema.schema}\n\`\`\`\n\n`;
      content += `For complete model mapping details, see [Model Mapping Technical Implementation](modelMapping_implementation.md)\n\n`;
    } else {
      content += `Model mapping implementation is in progress. This section will be updated as code is developed.\n\n`;
      
      // Include placeholder
      content += `### Course to Category Mapping Schema (Planned)\n\n`;
      content += `\`\`\`sql\nCREATE TABLE course_category_mappings (\n  id SERIAL PRIMARY KEY,\n  canvas_course_id INTEGER NOT NULL,\n  discourse_category_id INTEGER NOT NULL,\n  sync_enabled BOOLEAN DEFAULT TRUE,\n  last_sync_at TIMESTAMP,\n  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,\n  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,\n  UNIQUE(canvas_course_id)\n);\n\`\`\`\n\n`;
    }
    
    // API Integration section
    content += `## API Integration\n\n`;
    if (implementations.apiIntegration.functions.length > 0) {
      const exampleFunc = implementations.apiIntegration.functions[0];
      content += `### ${exampleFunc.name || 'API Integration Example'}\n\n`;
      content += `\`\`\`${exampleFunc.language || 'javascript'}\n${exampleFunc.source}\n\`\`\`\n\n`;
      content += `For complete API integration details, see [API Integration Technical Implementation](apiIntegration_implementation.md)\n\n`;
    } else {
      content += `API integration implementation is in planning phase. This section will be updated as code is developed.\n\n`;
    }
    
    // Synchronization section
    content += `## Synchronization Implementation\n\n`;
    if (implementations.synchronization.functions.length > 0 || implementations.synchronization.classes.length > 0) {
      const exampleItem = implementations.synchronization.functions[0] || implementations.synchronization.classes[0];
      if (exampleItem) {
        content += `### ${exampleItem.name || 'Synchronization Implementation'}\n\n`;
        content += `\`\`\`${exampleItem.language || 'javascript'}\n${exampleItem.source}\n\`\`\`\n\n`;
      }
      content += `For complete synchronization implementation details, see [Synchronization Technical Implementation](synchronization_implementation.md)\n\n`;
    } else {
      content += `Synchronization implementation has not yet started. This section will be updated as code is developed.\n\n`;
    }
    
    // Error handling section
    content += `## Error Handling and Retry Mechanisms\n\n`;
    content += `\`\`\`javascript\nasync function reliableApiCall(apiFunction, ...args) {\n  const MAX_RETRIES = 3;\n  const RETRY_DELAY_MS = 1000;\n  \n  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {\n    try {\n      return await apiFunction(...args);\n    } catch (error) {\n      console.error(\`API call failed (attempt \${attempt}/\${MAX_RETRIES}):\`, error.message);\n      \n      // Don't retry if it's a 4xx error (except 429 - rate limiting)\n      if (error.status && error.status >= 400 && error.status < 500 && error.status !== 429) {\n        throw error;\n      }\n      \n      // Last attempt failed, propagate the error\n      if (attempt === MAX_RETRIES) {\n        throw error;\n      }\n      \n      // Wait before retrying\n      await new Promise(resolve => setTimeout(resolve, RETRY_DELAY_MS * attempt));\n    }\n  }\n}\n\`\`\`\n\n`;
    
    // Write to file
    const outputPath = path.join(this.options.outputDir, 'technical_implementation.md');
    fs.writeFileSync(outputPath, content);
    
    return outputPath;
  }
}

module.exports = TechnicalDocsGenerator;