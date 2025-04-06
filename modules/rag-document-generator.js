const path = require('path');
const fs = require('fs-extra');
const { encode } = require('gpt-3-encoder');

/**
 * RAG Document Generator
 * Creates specialized knowledge documents from source systems for AI retrieval
 */
class RagDocumentGenerator {
  constructor(metrics, options = {}) {
    this.metrics = metrics;
    this.options = Object.assign({
      outputDir: 'knowledge_base',
      chunkSize: 1500,
      chunkOverlap: 200,
      includeMetadata: true
    }, options);
    
    // Initialize tracking for generated documents
    this.metrics.rag = {
      documents: { total: 0, bySystem: {} },
      chunks: { total: 0, bySystem: {} },
      embeddings: { total: 0, bySystem: {} },
      coverage: { percentage: 0, bySystem: {} }
    };
  }

  /**
   * Generate RAG documents for all source systems
   */
  async generateRagDocuments(baseDir, sourceSystems, fsUtils) {
    console.log("Generating RAG knowledge documents...");
    
    const outputDir = path.join(baseDir, this.options.outputDir);
    await fs.ensureDir(outputDir);
    
    // Track start time for performance metrics
    const startTime = Date.now();
    
    // Process each source system
    for (const systemName of Object.keys(sourceSystems)) {
      const systemPath = sourceSystems[systemName];
      const systemOutputDir = path.join(outputDir, systemName);
      await fs.ensureDir(systemOutputDir);
      
      // Initialize system tracking
      this.metrics.rag.documents.bySystem[systemName] = 0;
      this.metrics.rag.chunks.bySystem[systemName] = 0;
      this.metrics.rag.embeddings.bySystem[systemName] = 0;
      
      // Generate system overview document
      await this.generateSystemOverview(systemName, systemPath, systemOutputDir);
      
      // Extract code knowledge
      await this.extractCodeKnowledge(systemName, systemPath, systemOutputDir, fsUtils);
      
      // Generate architectural knowledge documents
      await this.generateArchitecturalKnowledge(systemName, systemPath, systemOutputDir, fsUtils);
      
      // Generate relationship knowledge documents
      await this.generateRelationshipKnowledge(systemName, systemPath, systemOutputDir);
    }
    
    // Generate cross-system integration documents
    await this.generateIntegrationKnowledge(outputDir, sourceSystems);
    
    // Update metrics
    this.metrics.rag.performance = {
      generationTime: Date.now() - startTime,
      averageChunkSize: 0,
      compressionRatio: 0
    };
    
    if (this.metrics.rag.chunks.total > 0) {
      this.metrics.rag.performance.averageChunkSize = 
        this.metrics.rag.totalTokens / this.metrics.rag.chunks.total;
    }
    
    // Generate knowledge base index
    await this.generateKnowledgeIndex(outputDir);
    
    console.log(`RAG document generation complete. Generated ${this.metrics.rag.documents.total} documents with ${this.metrics.rag.chunks.total} chunks.`);
    return this.metrics.rag;
  }
  
  /**
   * Generate system overview document
   */
  async generateSystemOverview(systemName, systemPath, outputDir) {
    console.log(`Generating system overview for ${systemName}...`);
    
    let overview = `# ${systemName.toUpperCase()} System Overview\n\n`;
    overview += `Path: ${systemPath}\n\n`;
    
    // Add system description based on name
    if (systemName.toLowerCase() === 'canvas') {
      overview += `Canvas LMS is an open-source learning management system by Instructure. ` +
                 `It's built with Ruby on Rails and React, featuring course management, ` +
                 `assignments, grading, discussions, and extensive API integrations.\n\n`;
    } else if (systemName.toLowerCase() === 'discourse') {
      overview += `Discourse is an open-source discussion platform built with Ruby on Rails and Ember.js. ` +
                 `It provides forum functionality with modern features like infinite scrolling, ` +
                 `real-time updates, and a plugin system for extensibility.\n\n`;
    }
    
    // Add system statistics if available
    if (this.metrics.sourceSystems && this.metrics.sourceSystems[systemName]) {
      const stats = this.metrics.sourceSystems[systemName];
      overview += `## System Statistics\n\n`;
      
      if (stats.models) {
        overview += `- **Models**: ${stats.models.total} models identified\n`;
      }
      
      if (stats.controllers) {
        overview += `- **Controllers**: ${stats.controllers.total} controllers identified\n`;
      }
      
      if (stats.filesByType) {
        overview += `- **Files By Type**:\n`;
        for (const [type, count] of Object.entries(stats.filesByType)) {
          overview += `  - ${type}: ${count}\n`;
        }
      }
      
      overview += `\n`;
    }
    
    // Write overview file
    const overviewPath = path.join(outputDir, '00-system-overview.md');
    await fs.writeFile(overviewPath, overview);
    
    // Update metrics
    this.metrics.rag.documents.total++;
    this.metrics.rag.documents.bySystem[systemName]++;
    
    return overviewPath;
  }
  
  /**
   * Extract code knowledge from system files
   */
  async extractCodeKnowledge(systemName, systemPath, outputDir, fsUtils) {
    console.log(`Extracting code knowledge from ${systemName}...`);
    
    // Define important file categories based on system
    const filePatterns = {
      canvas: {
        models: /app\/models\/.*\.rb$/,
        controllers: /app\/controllers\/.*\.rb$/,
        api: /app\/controllers\/api\/.*\.rb$/,
        services: /app\/services\/.*\.rb$/,
        javascriptModules: /app\/javascript\/.*\.(js|jsx|ts)$/
      },
      discourse: {
        models: /app\/models\/.*\.rb$/,
        controllers: /app\/controllers\/.*\.rb$/,
        api: /app\/controllers\/api\/.*\.rb$/,
        services: /app\/services\/.*\.rb$/,
        javascriptModules: /app\/assets\/javascripts\/.*\.js$/,
        plugins: /plugins\/.*\/plugin\.rb$/
      }
    };
    
    const patterns = filePatterns[systemName.toLowerCase()] || {
      models: /models?\/.*\.(rb|js|ts)$/i,
      controllers: /controllers?\/.*\.(rb|js|ts)$/i
    };
    
    // Process each file category
    for (const [category, pattern] of Object.entries(patterns)) {
      await this.processFileCategory(systemName, systemPath, category, pattern, outputDir, fsUtils);
    }
  }
  
  /**
   * Process files in a category
   */
  async processFileCategory(systemName, systemPath, category, pattern, outputDir, fsUtils) {
    // Find all matching files
    const files = fsUtils.findFilesInDir(systemPath, pattern);
    
    if (files.length === 0) {
      console.log(`No ${category} files found for ${systemName}`);
      return;
    }
    
    console.log(`Processing ${files.length} ${category} files for ${systemName}`);
    
    // Group files by subcategory if many files exist
    const subcategories = {};
    
    files.forEach(file => {
      const relativePath = path.relative(systemPath, file);
      const parts = relativePath.split(path.sep);
      
      // Skip the first two parts (app/models or similar)
      let subcategory = 'general';
      if (parts.length > 2) {
        subcategory = parts[2]; // Use the directory after app/models as subcategory
        
        // Handle nested directories by using the first non-standard directory
        if (subcategory === 'concerns' && parts.length > 3) {
          subcategory = parts[3];
        }
      }
      
      if (!subcategories[subcategory]) {
        subcategories[subcategory] = [];
      }
      subcategories[subcategory].push(file);
    });
    
    // Create category directory
    const categoryDir = path.join(outputDir, category);
    await fs.ensureDir(categoryDir);
    
    // Process each subcategory
    for (const [subcategory, subcatFiles] of Object.entries(subcategories)) {
      // Create knowledge document for subcategory
      const docTitle = `${systemName}-${category}-${subcategory}`;
      const docPath = path.join(categoryDir, `${subcategory}.md`);
      
      let content = `# ${docTitle}\n\n`;
      content += `This document contains extracted knowledge from ${subcatFiles.length} ${category} files in the ${subcategory} subcategory.\n\n`;
      
      // Process each file in the subcategory
      for (const file of subcatFiles) {
        try {
          const fileContent = await fs.readFile(file, 'utf8');
          const relativePath = path.relative(systemPath, file);
          
          content += `## ${path.basename(file)}\n\n`;
          content += `Path: ${relativePath}\n\n`;
          
          // Extract and add documentation comments
          const docComments = this.extractDocComments(fileContent);
          if (docComments.length > 0) {
            content += `### Documentation\n\n${docComments.join('\n\n')}\n\n`;
          }
          
          // Extract key structures based on file type
          const ext = path.extname(file);
          if (ext === '.rb') {
            content += this.extractRubyStructures(fileContent, relativePath);
          } else if (['.js', '.jsx', '.ts', '.tsx'].includes(ext)) {
            content += this.extractJsStructures(fileContent, relativePath);
          }
          
          content += `\n\n---\n\n`;
        } catch (error) {
          console.error(`Error processing file ${file}: ${error.message}`);
        }
      }
      
      // Write the document
      await fs.writeFile(docPath, content);
      
      // Update metrics
      this.metrics.rag.documents.total++;
      this.metrics.rag.documents.bySystem[systemName]++;
      
      // Chunk the document for embedding
      await this.chunkDocument(docPath, `${systemName}/${category}/${subcategory}`, {
        system: systemName,
        category: category,
        subcategory: subcategory
      });
    }
  }
  
  /**
   * Extract documentation comments from code
   */
  extractDocComments(content) {
    const comments = [];
    
    // Ruby comments (# or =begin/=end)
    const rubyCommentRegex = /^[\s]*#\s*(.*$)/gm;
    const rubyBlockCommentRegex = /=begin\s*([\s\S]*?)\s*=end/gm;
    
    // JS/TS comments (// or /* */)
    const jsCommentRegex = /^[\s]*\/\/\s*(.*$)/gm;
    const jsBlockCommentRegex = /\/\*\*?([\s\S]*?)\*\//gm;
    
    let match;
    
    // Extract Ruby inline comments
    while ((match = rubyCommentRegex.exec(content)) !== null) {
      if (match[1] && !match[1].startsWith('TODO') && match[1].length > 10) {
        comments.push(match[1]);
      }
    }
    
    // Extract Ruby block comments
    while ((match = rubyBlockCommentRegex.exec(content)) !== null) {
      if (match[1]) {
        comments.push(match[1].trim());
      }
    }
    
    // Extract JS inline comments
    while ((match = jsCommentRegex.exec(content)) !== null) {
      if (match[1] && !match[1].startsWith('TODO') && match[1].length > 10) {
        comments.push(match[1]);
      }
    }
    
    // Extract JS block comments
    while ((match = jsBlockCommentRegex.exec(content)) !== null) {
      if (match[1]) {
        comments.push(match[1].trim().replace(/\n[\s]*\*/g, '\n'));
      }
    }
    
    return comments;
  }
  
  /**
   * Extract key structures from Ruby code
   */
  extractRubyStructures(content, filePath) {
    let result = '';
    
    // Extract class/module definitions
    const classRegex = /class\s+(\w+)(?:\s+<\s+(\w+))?\b/g;
    const moduleRegex = /module\s+(\w+)\b/g;
    
    // Extract method definitions
    const methodRegex = /def\s+(\w+)(?:\(([^)]*)\))?/g;
    
    // Extract ActiveRecord associations and validations
    const associationRegex = /(has_many|has_one|belongs_to|has_and_belongs_to_many)\s+:([\w_]+)(?:,\s*(.*))?/g;
    const validationRegex = /validates(?:_\w+)?\s+:([\w_]+)(?:,\s*(.*))?/g;
    
    // Extract class/module inheritance
    let classMatch;
    while ((classMatch = classRegex.exec(content)) !== null) {
      const className = classMatch[1];
      const parentClass = classMatch[2] || 'None';
      result += `**Class:** ${className} < ${parentClass}\n\n`;
    }
    
    let moduleMatch;
    while ((moduleMatch = moduleRegex.exec(content)) !== null) {
      result += `**Module:** ${moduleMatch[1]}\n\n`;
    }
    
    // Extract ActiveRecord associations
    let associations = [];
    let associationMatch;
    while ((associationMatch = associationRegex.exec(content)) !== null) {
      const relationType = associationMatch[1];
      const relationName = associationMatch[2];
      const options = associationMatch[3] || '';
      associations.push(`- ${relationType} :${relationName} ${options}`);
    }
    
    if (associations.length > 0) {
      result += `**Associations:**\n${associations.join('\n')}\n\n`;
    }
    
    // Extract validations
    let validations = [];
    let validationMatch;
    while ((validationMatch = validationRegex.exec(content)) !== null) {
      const fieldName = validationMatch[1];
      const options = validationMatch[2] || '';
      validations.push(`- validates :${fieldName} ${options}`);
    }
    
    if (validations.length > 0) {
      result += `**Validations:**\n${validations.join('\n')}\n\n`;
    }
    
    // Extract methods
    let methods = [];
    let methodMatch;
    while ((methodMatch = methodRegex.exec(content)) !== null) {
      const methodName = methodMatch[1];
      const params = methodMatch[2] || '';
      
      // Skip Rails callback methods if too many methods
      if (methods.length > 15 && 
          ['before_save', 'after_save', 'before_create', 'after_create'].includes(methodName)) {
        continue;  
      }
      
      methods.push(`- ${methodName}(${params})`);
    }
    
    if (methods.length > 0) {
      result += `**Methods:**\n${methods.join('\n')}\n\n`;
    }
    
    return result;
  }
  
  /**
   * Extract key structures from JavaScript/TypeScript code
   */
  extractJsStructures(content, filePath) {
    let result = '';
    
    // Extract class definitions
    const classRegex = /class\s+(\w+)(?:\s+extends\s+(\w+))?\s*{/g;
    let classMatch;
    while ((classMatch = classRegex.exec(content)) !== null) {
      const className = classMatch[1];
      const parentClass = classMatch[2] || 'None';
      result += `**Class:** ${className}${parentClass !== 'None' ? ` extends ${parentClass}` : ''}\n\n`;
    }
    
    // Extract function/method definitions
    const functionRegex = /(?:function|async function)\s+(\w+)\s*\(([^)]*)\)/g;
    const classMethodRegex = /(?:async\s+)?(\w+)\s*\(([^)]*)\)\s*{/g;
    
    let functions = [];
    let functionMatch;
    while ((functionMatch = functionRegex.exec(content)) !== null) {
      const functionName = functionMatch[1];
      const params = functionMatch[2] || '';
      functions.push(`- ${functionName}(${params})`);
    }
    
    let classMethods = [];
    let methodMatch;
    while ((methodMatch = classMethodRegex.exec(content)) !== null) {
      const methodName = methodMatch[1];
      
      // Skip constructor and common React lifecycle methods if too many methods
      if (classMethods.length > 15 && 
          ['constructor', 'componentDidMount', 'render'].includes(methodName)) {
        continue;
      }
      
      const params = methodMatch[2] || '';
      classMethods.push(`- ${methodName}(${params})`);
    }
    
    // Extract React component properties (props)
    const propsRegex = /props\.([\w]+)/g;
    const propTypes = /static\s+propTypes\s*=\s*{([^}]*)}/;
    
    let props = new Set();
    let propsMatch;
    while ((propsMatch = propsRegex.exec(content)) !== null) {
      props.add(propsMatch[1]);
    }
    
    if (props.size > 0) {
      result += `**Props:**\n${Array.from(props).map(p => `- ${p}`).join('\n')}\n\n`;
    }
    
    // Extract imports/exports for module dependencies
    const importRegex = /import\s+(?:{([^}]*)})?\s*(?:from\s+['"]([^'"]+)['"])?/g;
    const exportRegex = /export\s+(?:default\s+)?(?:class|function|const)?\s*(\w+)/g;
    
    let imports = [];
    let importMatch;
    while ((importMatch = importRegex.exec(content)) !== null) {
      if (importMatch[1] || importMatch[2]) {
        const modules = importMatch[1] ? importMatch[1] : 'default';
        const source = importMatch[2] || '';
        imports.push(`- import ${modules} from ${source}`);
      }
    }
    
    if (imports.length > 0) {
      result += `**Imports:**\n${imports.join('\n')}\n\n`;
    }
    
    let exports = [];
    let exportMatch;
    while ((exportMatch = exportRegex.exec(content)) !== null) {
      exports.push(`- ${exportMatch[1]}`);
    }
    
    if (exports.length > 0) {
      result += `**Exports:**\n${exports.join('\n')}\n\n`;
    }
    
    if (functions.length > 0) {
      result += `**Functions:**\n${functions.join('\n')}\n\n`;
    }
    
    if (classMethods.length > 0) {
      result += `**Methods:**\n${classMethods.join('\n')}\n\n`;
    }
    
    return result;
  }
  
  /**
   * Generate architectural knowledge documents
   */
  async generateArchitecturalKnowledge(systemName, systemPath, outputDir, fsUtils) {
    console.log(`Generating architectural knowledge for ${systemName}...`);
    
    const architectureDir = path.join(outputDir, 'architecture');
    await fs.ensureDir(architectureDir);
    
    // Generate system architecture overview
    const archOverviewPath = path.join(architectureDir, 'overview.md');
    let archContent = `# ${systemName.toUpperCase()} Architectural Overview\n\n`;
    
    // Add system-specific architectural descriptions
    if (systemName.toLowerCase() === 'canvas') {
      archContent += `
Canvas LMS follows a classic Ruby on Rails MVC architecture with a React frontend. Key architectural components include:

1. **Core Models**: Hierarchical structure with Course as a central entity, connected to Users, Assignments, and other educational resources.

2. **Plugin System**: Canvas supports plugins through a gems-based extension mechanism, allowing for custom features.

3. **API Layer**: RESTful API with versioning for integration with external systems and the frontend.

4. **Authentication**: Multiple authentication providers through the platform_web gem.

5. **Frontend Architecture**: Combination of server-rendered ERB templates and React components, with a gradual migration to React.

6. **Database Structure**: Uses PostgreSQL with complex relationships between educational entities.
`;
    } else if (systemName.toLowerCase() === 'discourse') {
      archContent += `
Discourse follows a modern web application architecture with several distinctive aspects:

1. **Plugin Architecture**: Highly extensible through Ruby gems and JavaScript modules.

2. **Frontend Framework**: Ember.js-based frontend with client-side rendering and a RESTful API backend.

3. **Real-time Updates**: Uses MessageBus for pushing live updates to clients.

4. **Authentication**: Supports multiple auth providers and SSO.

5. **Storage Services**: Supports various file storage providers for uploads.

6. **Database Structure**: PostgreSQL with extensive use of JSON columns for flexible data storage.

7. **Caching Layer**: Redis for caching and background job processing.
`;
    }
    
    // Look for architecture diagrams
    const diagramExtensions = ['.png', '.jpg', '.svg', '.drawio'];
    const diagramPatterns = diagramExtensions.map(ext => new RegExp(`arch.*\\${ext}$|diagram.*\\${ext}$|structure.*\\${ext}$`, 'i'));
    
    for (const pattern of diagramPatterns) {
      const diagrams = fsUtils.findFilesInDir(systemPath, pattern);
      if (diagrams.length > 0) {
        archContent += `\n## Architecture Diagrams\n\n`;
        archContent += `The following architecture diagrams were found in the codebase:\n\n`;
        
        for (const diagram of diagrams) {
          const relativePath = path.relative(systemPath, diagram);
          archContent += `- ${relativePath}\n`;
        }
      }
    }
    
    // Write architecture overview
    await fs.writeFile(archOverviewPath, archContent);
    
    // Update metrics
    this.metrics.rag.documents.total++;
    this.metrics.rag.documents.bySystem[systemName]++;
    
    // Chunk the document for embedding
    await this.chunkDocument(archOverviewPath, `${systemName}/architecture/overview`, {
      system: systemName,
      category: 'architecture'
    });
    
    // Extract and document core patterns
    await this.extractArchitecturalPatterns(systemName, systemPath, architectureDir, fsUtils);
  }
  
  /**
   * Extract architectural patterns from the codebase
   */
  async extractArchitecturalPatterns(systemName, systemPath, outputDir, fsUtils) {
    console.log(`Extracting architectural patterns for ${systemName}...`);
    
    // Define patterns to look for based on the system
    const patternDefinitions = {
      canvas: [
        { name: 'MVC', filePatterns: [/app\/models\/.*\.rb$/, /app\/controllers\/.*\.rb$/, /app\/views\/.*\.erb$/] },
        { name: 'Service Objects', filePatterns: [/app\/services\/.*\.rb$/, /app\/lib\/.*service.*\.rb$/] },
        { name: 'Observers', filePatterns: [/app\/observers\/.*\.rb$/] },
        { name: 'APIs', filePatterns: [/app\/controllers\/api\/v\d+\/.*\.rb$/] }
      ],
      discourse: [
        { name: 'MVC', filePatterns: [/app\/models\/.*\.rb$/, /app\/controllers\/.*\.rb$/, /app\/views\/.*\.erb$/] },
        { name: 'Services', filePatterns: [/app\/services\/.*\.rb$/, /app\/lib\/.*service.*\.rb$/] },
        { name: 'Plugins', filePatterns: [/plugins\/.*\/plugin\.rb$/] },
        { name: 'Jobs', filePatterns: [/app\/jobs\/.*\.rb$/] }
      ]
    };
    
    const systemPatterns = patternDefinitions[systemName.toLowerCase()] || patternDefinitions.canvas;
    
    let patternsContent = `# ${systemName.toUpperCase()} Architectural Patterns\n\n`;
    patternsContent += `This document describes key architectural patterns identified in the ${systemName} codebase.\n\n`;
    
    // Analyze each pattern
    for (const pattern of systemPatterns) {
      patternsContent += `## ${pattern.name} Pattern\n\n`;
      
      let examples = [];
      for (const filePattern of pattern.filePatterns) {
        const matchingFiles = fsUtils.findFilesInDir(systemPath, filePattern);
        if (matchingFiles.length > 0) {
          // Take a sample of matching files (up to 5)
          const sampleFiles = matchingFiles.slice(0, 5);
          examples = examples.concat(sampleFiles.map(f => path.relative(systemPath, f)));
        }
      }
      
      if (examples.length > 0) {
        patternsContent += `Found ${examples.length}+ examples.\n\n`;
        patternsContent += `**Example implementations:**\n`;
        examples.forEach(ex => {
          patternsContent += `- \`${ex}\`\n`;
        });
        patternsContent += `\n`;
      } else {
        patternsContent += `No clear examples of this pattern were found.\n\n`;
      }
    }
    
    // Write patterns document
    const patternsPath = path.join(outputDir, 'patterns.md');
    await fs.writeFile(patternsPath, patternsContent);
    
    // Update metrics
    this.metrics.rag.documents.total++;
    this.metrics.rag.documents.bySystem[systemName]++;
    
    // Chunk the document for embedding
    await this.chunkDocument(patternsPath, `${systemName}/architecture/patterns`, {
      system: systemName,
      category: 'architecture',
      subcategory: 'patterns'
    });
  }
  
  /**
   * Generate relationship knowledge documents
   */
  async generateRelationshipKnowledge(systemName, systemPath, outputDir) {
    console.log(`Generating relationship knowledge for ${systemName}...`);
    
    const relationshipsDir = path.join(outputDir, 'relationships');
    await fs.ensureDir(relationshipsDir);
    
    // Create relationship document based on metrics
    const relPath = path.join(relationshipsDir, 'model-relationships.md');
    let relContent = `# ${systemName.toUpperCase()} Model Relationships\n\n`;
    
    // Filter relationships for this system
    const systemModels = this.metrics.sourceSystems?.[systemName]?.models?.details || [];
    if (systemModels.length > 0) {
      relContent += `## Core Models\n\n`;
      relContent += `This system contains ${systemModels.length} identified models:\n\n`;
      
      // Group models by category if possible
      const modelsByCategory = {};
      
      systemModels.forEach(model => {
        let category = 'Other';
        
        // Try to categorize models
        if (systemName.toLowerCase() === 'canvas') {
          if (/user|account|profile|person/i.test(model.name)) category = 'Users';
          else if (/course|module|lesson/i.test(model.name)) category = 'Courses';
          else if (/assign|submiss|grade|quiz/i.test(model.name)) category = 'Assignments';
        } else if (systemName.toLowerCase() === 'discourse') {
          if (/user|profile/i.test(model.name)) category = 'Users';
          if (/topic|post|reply/i.test(model.name)) category = 'Posts';
          if (/categor|tag/i.test(model.name)) category = 'Categories';
        }
        
        if (!modelsByCategory[category]) {
          modelsByCategory[category] = [];
        }
        modelsByCategory[category].push(model);
      });
      
      // List models by category
      for (const [category, models] of Object.entries(modelsByCategory)) {
        relContent += `### ${category}\n\n`;
        models.forEach(model => {
          relContent += `- **${model.name}**\n`;
        });
        relContent += `\n`;
      }
    }
    
    // Add relationship information from metrics
    if (this.metrics.relationships && this.metrics.relationships.length > 0) {
      relContent += `## Model Relationships\n\n`;
      
      // Filter relationships for this system
      const systemRelationships = this.metrics.relationships.filter(rel => {
        return systemModels.some(m => m.name === rel.from || m.name === rel.to);
      });
      
      if (systemRelationships.length > 0) {
        relContent += `The following relationships were identified between models:\n\n`;
        relContent += `| From | Relationship | To | Notes |\n`;
        relContent += `|------|--------------|----|---------|\n`;
        
        systemRelationships.forEach(rel => {
          relContent += `| ${rel.from} | ${rel.type} | ${rel.to} | ${rel.notes || ''} |\n`;
        });
      } else {
        relContent += `No explicit relationships were identified between models.\n`;
      }
      
      // Add Mermaid diagram if there are relationships
      if (systemRelationships.length > 0) {
        relContent += `\n## Relationship Diagram\n\n`;
        relContent += "```mermaid\ngraph LR\n";
        
        systemRelationships.forEach(rel => {
          const arrow = rel.type === 'OneToMany' ? '-->|1:*|' : 
                       rel.type === 'ManyToMany' ? '<-->|*:*|' :
                       '-->';
          relContent += `  ${rel.from}${arrow}${rel.to}\n`;
        });
        
        relContent += "```\n";
      }
    }
    
    // Write relationship document
    await fs.writeFile(relPath, relContent);
    
    // Update metrics
    this.metrics.rag.documents.total++;
    this.metrics.rag.documents.bySystem[systemName]++;
    
    // Chunk the document for embedding
    await this.chunkDocument(relPath, `${systemName}/relationships/models`, {
      system: systemName,
      category: 'relationships'
    });
  }
  
  /**
   * Generate cross-system integration knowledge
   */
  async generateIntegrationKnowledge(outputDir, sourceSystems) {
    console.log("Generating integration knowledge between systems...");
    
    const integrationDir = path.join(outputDir, 'integration');
    await fs.ensureDir(integrationDir);
    
    // Create integration possibilities document
    const integrationPath = path.join(integrationDir, 'integration-points.md');
    
    let content = `# Cross-System Integration Points\n\n`;
    content += `This document identifies potential integration points between Canvas and Discourse systems.\n\n`;
    
    // Only proceed if both systems are available
    if (!this.metrics.sourceSystems?.canvas || !this.metrics.sourceSystems?.discourse) {
      content += `**Note:** Complete source system information is not available. Integration analysis is limited.\n\n`;
    } else {
      // Get model lists
      const canvasModels = this.metrics.sourceSystems.canvas.models.details || [];
      const discourseModels = this.metrics.sourceSystems.discourse.models.details || [];
      
      content += `## Model Integration\n\n`;
      content += `### Canvas Models (${canvasModels.length})\n\n`;
      canvasModels.slice(0, 10).forEach(model => {
        content += `- ${model.name}\n`;
      });
      if (canvasModels.length > 10) content += `- ...(${canvasModels.length - 10} more)\n`;
      
      content += `\n### Discourse Models (${discourseModels.length})\n\n`;
      discourseModels.slice(0, 10).forEach(model => {
        content += `- ${model.name}\n`;
      });
      if (discourseModels.length > 10) content += `- ...(${discourseModels.length - 10} more)\n`;
      
      // Identify core integration points
      content += `\n## Core Integration Points\n\n`;
      content += `The following integration points have been identified as critical for system blending:\n\n`;
      
      // User integration
      content += `### 1. User Identity & Authentication\n\n`;
      content += `Canvas and Discourse both have user systems that need to be integrated:\n\n`;
      content += `- **User Identity Mapping**: Map Canvas users to Discourse users\n`;
      content += `- **SSO Integration**: Use Canvas as the identity provider for Discourse\n`;
      content += `- **Profile Synchronization**: Keep user profiles in sync between systems\n\n`;
      
      // Content integration
      content += `### 2. Content Integration\n\n`;
      content += `Course content in Canvas needs to be linked with discussions in Discourse:\n\n`;
      content += `- **Course-Forum Mapping**: Each Canvas course maps to a Discourse category\n`;
      content += `- **Assignment Discussions**: Canvas assignments can be linked to Discourse topics\n`;
      content += `- **Content Embedding**: Discourse posts can be embedded in Canvas pages\n\n`;
      
      // Notification integration
      content += `### 3. Notifications & Activities\n\n`;
      content += `Users need a unified view of activities across both systems:\n\n`;
      content += `- **Notification Aggregation**: Combine notifications from both systems\n`;
      content += `- **Activity Streams**: Integrate Discourse activities into Canvas activity feeds\n`;
      content += `- **Email Preferences**: Unified email notification preferences\n\n`;
      
      // Data flow
      content += `## Data Flow Patterns\n\n`;
      content += `The following patterns should be considered for system integration:\n\n`;
      content += `1. **Event-Based Integration**: Use webhooks and event subscriptions to sync changes\n`;
      content += `2. **Shared Database Access**: Limited shared tables for critical mapping data\n`;
      content += `3. **API-Based Integration**: REST APIs for cross-system communication\n`;
      content += `4. **SSO & Authentication Flow**: Unified login and session management\n\n`;
      
      // API Integration
      content += `## API Integration Points\n\n`;
      content += `### Canvas APIs\n\n`;
      content += `- Users API - For user identity management\n`;
      content += `- Courses API - For course structure and membership\n`;
      content += `- Assignments API - For linking assignments to discussions\n`;
      content += `- Pages API - For embedding discussion content\n\n`;
      
      content += `### Discourse APIs\n\n`;
      content += `- Users API - For user creation and management\n`;
      content += `- Categories API - For creating and mapping to courses\n`;
      content += `- Topics API - For creating/reading discussions\n`;
      content += `- Posts API - For reading/writing discussion content\n`;
    }
    
    // Write integration document
    await fs.writeFile(integrationPath, content);
    
    // Update metrics
    this.metrics.rag.documents.total++;
    
    // Chunk the document for embedding
    await this.chunkDocument(integrationPath, `integration/integration-points`, {
      category: 'integration',
      purpose: 'system-integration'
    });
    
    // Create integration architecture blueprint
    await this.generateIntegrationBlueprint(integrationDir);
  }
  
  /**
   * Generate integration architecture blueprint
   */
  async generateIntegrationBlueprint(integrationDir) {
    console.log("Generating integration architecture blueprint...");
    
    const blueprintPath = path.join(integrationDir, 'architecture-blueprint.md');
    
    let content = `# Canvas-Discourse Integration Architecture Blueprint\n\n`;
    content += `This document provides an architectural blueprint for integrating Canvas LMS with Discourse.\n\n`;
    
    // Integration architecture
    content += `## High-Level Architecture\n\n`;
    
    content += `\`\`\`mermaid
graph TB
    subgraph "Canvas LMS"
        C_Users[Users]
        C_Courses[Courses]
        C_Assignments[Assignments]
        C_Pages[Pages]
        C_API[APIs]
    end
    
    subgraph "Integration Layer"
        I_SSO[SSO Service]
        I_Sync[Data Sync]
        I_Events[Event Bus]
        I_Embed[Content Embedding]
    end
    
    subgraph "Discourse"
        D_Users[Users]
        D_Categories[Categories]
        D_Topics[Topics]
        D_Posts[Posts]
        D_API[APIs]
    end
    
    C_Users --> I_SSO
    I_SSO --> D_Users
    
    C_Courses --> I_Sync
    I_Sync --> D_Categories
    
    C_API --> I_Events
    I_Events --> D_API
    
    C_Pages --> I_Embed
    I_Embed --> D_Posts
    
    C_Assignments --> D_Topics
\`\`\`\n\n`;
    
    // Integration principles
    content += `## Integration Principles\n\n`;
    content += `1. **Loose Coupling**: Systems should remain independently deployable\n`;
    content += `2. **Data Consistency**: Ensure data remains consistent across systems\n`;
    content += `3. **Fault Tolerance**: Integration should handle temporary failures gracefully\n`;
    content += `4. **Performance**: Integration should not significantly impact system performance\n`;
    content += `5. **Security**: Maintain security controls across system boundaries\n\n`;
    
    // Component details
    content += `## Integration Components\n\n`;
    
    content += `### SSO Integration Service\n\n`;
    content += `- **Purpose**: Provide single sign-on between Canvas and Discourse\n`;
    content += `- **Features**:\n`;
    content += `  - User identity mapping\n`;
    content += `  - Authentication flow\n`;
    content += `  - Session management\n`;
    content += `- **Implementation**: Use OAuth 2.0 for authentication flow\n\n`;
    
    content += `### Data Synchronization Service\n\n`;
    content += `- **Purpose**: Keep relevant data in sync between systems\n`;
    content += `- **Features**:\n`;
    content += `  - Course-to-Category mapping\n`;
    content += `  - User profile synchronization\n`;
    content += `  - Role and permission mapping\n`;
    content += `- **Implementation**: Scheduled jobs and event-based triggers\n\n`;
    
    content += `### Event Bus\n\n`;
    content += `- **Purpose**: Propagate events between systems for real-time updates\n`;
    content += `- **Features**:\n`;
    content += `  - Event publication from both systems\n`;
    content += `  - Event subscription and handlers\n`;
    content += `  - Delivery guarantees and retry logic\n`;
    content += `- **Implementation**: Message queue (RabbitMQ or similar)\n\n`;
    
    content += `### Content Embedding Service\n\n`;
    content += `- **Purpose**: Enable embedding of discourse content in Canvas\n`;
    content += `- **Features**:\n`;
    content += `  - oEmbed support for Discourse posts\n`;
    content += `  - Live content updates\n`;
    content += `  - Contextual toolbar for content creation\n`;
    content += `- **Implementation**: JavaScript widget and iframe embedding\n\n`;
    
    content += `## Data Models & Mapping\n\n`;
    content += `### Core Entity Mappings\n\n`;
    content += `| Canvas Entity | Discourse Entity | Mapping Logic |\n`;
    content += `|--------------|------------------|---------------|\n`;
    content += `| User         | User             | 1:1 mapping based on email/ID |\n`;
    content += `| Course       | Category         | 1:1 mapping with metadata |\n`;
    content += `| Assignment   | Topic            | 1:1 for discussion assignments |\n`;
    content += `| Group        | Group            | 1:1 for collaborative groups |\n`;
    content += `| Section      | Tags             | Sections mapped to topic tags |\n\n`;
    
    content += `## Development Roadmap\n\n`;
    content += `1. **Phase 1**: SSO Integration & User Mapping\n`;
    content += `   - Single sign-on between systems\n`;
    content += `   - Basic user profile synchronization\n\n`;
    content += `2. **Phase 2**: Course-Forum Integration\n`;
    content += `   - Course to category mapping\n`;
    content += `   - Basic content embedding\n\n`;
    content += `3. **Phase 3**: Deep Feature Integration\n`;
    content += `   - Assignment-Topic integration\n`;
    content += `   - Unified notifications\n`;
    content += `   - Gradebook integration for discussions\n\n`;
    content += `4. **Phase 4**: Advanced Features\n`;
    content += `   - Analytics integration\n`;
    content += `   - Mobile experience\n`;
    content += `   - Rich content embedding\n\n`;
    
    // Write blueprint document
    await fs.writeFile(blueprintPath, content);
    
    // Update metrics
    this.metrics.rag.documents.total++;
    
    // Chunk the document for embedding
    await this.chunkDocument(blueprintPath, `integration/architecture-blueprint`, {
      category: 'integration',
      subcategory: 'architecture',
      purpose: 'system-integration-blueprint'
    });
  }
  
  /**
   * Generate knowledge base index
   */
  async generateKnowledgeIndex(outputDir) {
    console.log("Generating knowledge base index...");
    
    // Get all markdown files
    const allFiles = await this.findAllMarkdownFiles(outputDir);
    
    // Create index content
    let indexContent = `# RAG Knowledge Base Index\n\n`;
    indexContent += `This index provides links to all knowledge documents in the RAG knowledge base.\n\n`;
    indexContent += `Generated on: ${new Date().toISOString().split('T')[0]}\n\n`;
    indexContent += `Total documents: ${allFiles.length}\n\n`;
    
    // Group files by system and category
    const filesBySystem = {};
    
    for (const file of allFiles) {
      // Extract system and category from path
      const relativePath = path.relative(outputDir, file);
      const parts = relativePath.split(path.sep);
      
      let system = parts[0] || 'shared';
      let category = parts.length > 1 ? parts[1] : 'general';
      
      if (!filesBySystem[system]) {
        filesBySystem[system] = {};
      }
      
      if (!filesBySystem[system][category]) {
        filesBySystem[system][category] = [];
      }
      
      filesBySystem[system][category].push({
        path: relativePath,
        name: path.basename(file, '.md')
      });
    }
    
    // Generate index by system and category
    for (const [system, categories] of Object.entries(filesBySystem)) {
      indexContent += `## ${system.charAt(0).toUpperCase() + system.slice(1)}\n\n`;
      
      for (const [category, files] of Object.entries(categories)) {
        indexContent += `### ${category.charAt(0).toUpperCase() + category.slice(1)}\n\n`;
        
        files.forEach(file => {
          const displayName = file.name
            .replace(/[-_]/g, ' ')
            .replace(/\b\w/g, c => c.toUpperCase());
          
          indexContent += `- [${displayName}](./${file.path})\n`;
        });
        
        indexContent += `\n`;
      }
    }
    
    // Write index file
    const indexPath = path.join(outputDir, 'index.md');
    await fs.writeFile(indexPath, indexContent);
    
    console.log(`Generated knowledge base index at ${indexPath}`);
    return indexPath;
  }
  
  /**
   * Find all markdown files in the output directory
   */
  async findAllMarkdownFiles(dir) {
    const items = await fs.readdir(dir, { withFileTypes: true });
    let files = [];
    
    for (const item of items) {
      const res = path.resolve(dir, item.name);
      if (item.isDirectory()) {
        files = files.concat(await this.findAllMarkdownFiles(res));
      } else if (item.isFile() && item.name.endsWith('.md') && item.name !== 'index.md') {
        files.push(res);
      }
    }
    
    return files;
  }
  
  /**
   * Chunk document for embedding
   */
  async chunkDocument(filePath, documentId, metadata = {}) {
    try {
      const content = await fs.readFile(filePath, 'utf8');
      
      // Calculate total tokens in the document
      const totalTokens = this.countTokens(content);
      
      // Create chunks based on token size
      const chunks = this.createChunks(content, this.options.chunkSize, this.options.chunkOverlap);
      
      // Output directory for chunks
      const chunksDir = path.join(path.dirname(filePath), '.chunks');
      await fs.ensureDir(chunksDir);
      
      // Write each chunk to a file
      const chunkFiles = [];
      for (let i = 0; i < chunks.length; i++) {
        const chunkFile = path.join(chunksDir, `${path.basename(filePath, '.md')}_chunk${i+1}.json`);
        
        // Add metadata to each chunk
        const chunkData = {
          documentId,
          chunkId: `${documentId}_chunk${i+1}`,
          content: chunks[i],
          metadata: {
            ...metadata,
            source: filePath,
            chunkIndex: i,
            totalChunks: chunks.length
          }
        };
        
        await fs.writeFile(chunkFile, JSON.stringify(chunkData, null, 2));
        chunkFiles.push(chunkFile);
      }
      
      // Update metrics
      if (metadata.system) {
        this.metrics.rag.chunks.bySystem[metadata.system] = 
          (this.metrics.rag.chunks.bySystem[metadata.system] || 0) + chunks.length;
      }
      this.metrics.rag.chunks.total += chunks.length;
      this.metrics.rag.totalTokens = (this.metrics.rag.totalTokens || 0) + totalTokens;
      
      return chunkFiles;
    } catch (error) {
      console.error(`Error chunking document ${filePath}:`, error.message);
      return [];
    }
  }
  
  /**
   * Create chunks from content based on token size
   */
  createChunks(content, chunkSize, chunkOverlap) {
    // Split content by headers to maintain context
    const headerSplits = content.split(/(?=#{1,3} )/);
    
    const chunks = [];
    let currentChunk = '';
    let currentTokens = 0;
    
    for (const split of headerSplits) {
      const splitTokens = this.countTokens(split);
      
      // If this section is already larger than a chunk, split it further
      if (splitTokens > chunkSize) {
        // Further split by paragraphs
        const paragraphs = split.split(/\n\n/);
        
        for (const paragraph of paragraphs) {
          const paraTokens = this.countTokens(paragraph);
          
          // If adding this paragraph would exceed chunk size, start a new chunk
          if (currentTokens + paraTokens > chunkSize && currentChunk !== '') {
            chunks.push(currentChunk);
            // Start new chunk with overlap
            if (chunkOverlap > 0 && currentChunk.length > chunkOverlap) {
              const overlapText = this.getOverlapText(currentChunk, chunkOverlap);
              currentChunk = overlapText;
              currentTokens = this.countTokens(overlapText);
            } else {
              currentChunk = '';
              currentTokens = 0;
            }
          }
          
          // Add paragraph to current chunk
          currentChunk += (currentChunk ? '\n\n' : '') + paragraph;
          currentTokens += paraTokens;
        }
      } 
      // If adding this section would exceed chunk size, start a new chunk
      else if (currentTokens + splitTokens > chunkSize && currentChunk !== '') {
        chunks.push(currentChunk);
        // Start new chunk with overlap
        if (chunkOverlap > 0 && currentChunk.length > chunkOverlap) {
          const overlapText = this.getOverlapText(currentChunk, chunkOverlap);
          currentChunk = overlapText;
          currentTokens = this.countTokens(overlapText);
        } else {
          currentChunk = '';
          currentTokens = 0;
        }
        
        currentChunk += split;
        currentTokens += splitTokens;
      }
      // Otherwise, add to current chunk
      else {
        currentChunk += (currentChunk ? '\n\n' : '') + split;
        currentTokens += splitTokens;
      }
    }
    
    // Add final chunk if not empty
    if (currentChunk) {
      chunks.push(currentChunk);
    }
    
    return chunks;
  }
  
  /**
   * Get overlap text from the end of a chunk
   */
  getOverlapText(text, overlapTokens) {
    // Simple implementation - get roughly the last X tokens
    // A more sophisticated implementation would consider paragraph boundaries
    const words = text.split(/\s+/);
    const approxWordsPerToken = 0.75; // Rough estimate
    const wordsToKeep = Math.ceil(overlapTokens / approxWordsPerToken);
    
    if (words.length <= wordsToKeep) {
      return text;
    }
    
    // Try to find a paragraph break near desired position
    const startIndex = Math.max(0, words.length - wordsToKeep);
    const overlapText = words.slice(startIndex).join(' ');
    
    // Look for the nearest header
    const headerMatch = overlapText.match(/(?=#{1,3} )/);
    if (headerMatch && headerMatch.index > 0) {
      return overlapText.substring(headerMatch.index);
    }
    
    return overlapText;
  }
  
  /**
   * Count tokens in text using GPT-3-Encoder
   */
  countTokens(text) {
    try {
      // Use GPT-3-Encoder to estimate token count
      return encode(text).length;
    } catch (error) {
      // Fallback to rough word-based estimation
      return Math.ceil(text.split(/\s+/).length * 1.3);
    }
  }
}

module.exports = RagDocumentGenerator;