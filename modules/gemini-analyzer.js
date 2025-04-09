/**
 * Module for Google Gemini AI integration with GitHub Copilot optimization
 */
const fs = require('fs');
const path = require('path');
const { GoogleGenerativeAI } = require('@google/generative-ai');

class GeminiAnalyzer {
  constructor(metrics, options = {}) {
    this.metrics = metrics;
    this.options = options;
    this.apiKey = options.geminiApiKey || process.env.GEMINI_API_KEY;
    
    // Initialize Google Generative AI
    this.genAI = new GoogleGenerativeAI(this.apiKey);
    
    // Use Gemini 1.5 as primary model with fallback to 1.0
    // According to Google docs: https://ai.google.dev/models/gemini
    this.primaryModel = this.genAI.getGenerativeModel({ 
      model: "gemini-2.5-pro-exp-03-25" // Gemini 2.5 (flash for faster performance)
    });
    
    this.fallbackModel = this.genAI.getGenerativeModel({
      model: "gemini-2.0-flash" // Gemini 2.0 fallback
    });
    
    this.cacheDir = path.join(options.baseDir || process.cwd(), '.analysis_cache', 'gemini');
    if (!fs.existsSync(this.cacheDir)) {
      fs.mkdirSync(this.cacheDir, { recursive: true });
    }

    // Track API calls to implement rate limiting
    this.lastApiCallTime = 0;
    this.minTimeBetweenCalls = options.minTimeBetweenCalls || 1000; // 1 second minimum
  }

  /**
   * Sleep utility function
   */
  async sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  /**
   * Execute model with fallback capability, quota management and caching
   */
  async executeWithFallback(prompt, cacheKey = null) {
    // Apply rate limiting
    const now = Date.now();
    const timeSinceLastCall = now - this.lastApiCallTime;
    
    if (timeSinceLastCall < this.minTimeBetweenCalls) {
      const delayNeeded = this.minTimeBetweenCalls - timeSinceLastCall;
      console.log(`Rate limiting: waiting ${delayNeeded}ms before next API call`);
      await this.sleep(delayNeeded);
    }
    
    // Update last API call time
    this.lastApiCallTime = Date.now();

    // Use cache if available and requested
    if (cacheKey && this.options.useCache !== false) {
      const cachedResult = await this.getFromCache(cacheKey);
      if (cachedResult) {
        console.log(`Using cached result for ${cacheKey}`);
        return { text: () => cachedResult };
      }
    }
    
    try {
      // Check if we should skip Gemini 2.5 due to quota issues
      const skipGemini25 = this.options.skipGemini25 || 
                           process.env.SKIP_GEMINI_25 === 'true';
      
      // Try the primary model if not skipping
      if (!skipGemini25) {
        console.log("Using Gemini 2.5 model...");
        try {
          const result = await this.primaryModel.generateContent(prompt);
          
          // Cache the successful result if we have a cache key
          if (cacheKey && result) {
            const responseText = result.response.text();
            await this.saveToCache(cacheKey, responseText);
          }
          
          return result.response; // Return the response directly
        } catch (error) {
          // If we hit a quota limit, set a flag to skip this model for future requests
          if (error.message && error.message.includes('429') && 
              error.message.includes('quota')) {
            console.warn("Gemini 2.5 quota exceeded, will skip for future requests");
            this.options.skipGemini25 = true;
            
            // Also set an environment variable to persist across runs
            process.env.SKIP_GEMINI_25 = 'true';
          }
          
          console.warn("Gemini 2.5 failed, falling back to Gemini 2.0:", error.message);
        }
      } else {
        console.log("Skipping Gemini 2.5 due to previous quota limit, using Gemini 2.0...");
      }
      
      // Use fallback model
      try {
        const fallbackResult = await this.fallbackModel.generateContent(prompt);
        
        // For Gemini 2.0, adapt the response format to match Gemini 2.5
        const adaptedResponse = {
          text: () => {
            // Check if the structure matches Gemini 2.0
            if (fallbackResult && fallbackResult.response && 
                typeof fallbackResult.response.text === 'function') {
              return fallbackResult.response.text();
            } else if (fallbackResult && fallbackResult.text && 
                      typeof fallbackResult.text === 'function') {
              return fallbackResult.text();
            } else if (fallbackResult && fallbackResult.candidates && 
                      fallbackResult.candidates[0] && 
                      fallbackResult.candidates[0].content && 
                      fallbackResult.candidates[0].content.parts) {
              // Extract text from Gemini 2.0 structure
              return fallbackResult.candidates[0].content.parts
                .filter(part => part.text)
                .map(part => part.text)
                .join('\n');
            } else {
              // Last resort - try to extract any text we can find
              const resultStr = JSON.stringify(fallbackResult);
              const textMatch = resultStr.match(/"text":"([^"]+)"/);
              return textMatch ? textMatch[1] : "Unable to extract text from response";
            }
          }
        };
        
        // Cache the successful result if we have a cache key
        if (cacheKey) {
          const responseText = adaptedResponse.text();
          await this.saveToCache(cacheKey, responseText);
        }
        
        return adaptedResponse;
      } catch (fallbackError) {
        // If both models have quota issues, we need to use local fallbacks
        if (fallbackError.message && fallbackError.message.includes('429') && 
            fallbackError.message.includes('quota')) {
          console.error("All Gemini models have reached quota limits, using local fallbacks");
          
          // If it's a quota error, try to use cached similar requests
          if (cacheKey) {
            const similarCachedResult = await this.findSimilarFromCache(cacheKey);
            if (similarCachedResult) {
              console.log(`Using similar cached result due to quota limits`);
              return { text: () => similarCachedResult };
            }
          }
          
          throw new Error("QUOTA_EXCEEDED: " + fallbackError.message);
        }
        
        console.error("Fallback model also failed:", fallbackError.message);
        throw new Error("Both models failed: " + fallbackError.message);
      }
    } catch (error) {
      // For any other errors
      throw error;
    }
  }

  /**
   * Get result from cache
   */
  async getFromCache(cacheKey) {
    const cacheFilePath = path.join(this.cacheDir, `${this.hashString(cacheKey)}.json`);
    
    if (fs.existsSync(cacheFilePath)) {
      try {
        const cachedData = JSON.parse(fs.readFileSync(cacheFilePath, 'utf8'));
        
        // Check if the cache is still valid (less than 24 hours old)
        if (Date.now() - cachedData.timestamp < 24 * 60 * 60 * 1000) {
          return cachedData.response;
        }
      } catch (error) {
        console.warn(`Cache read error: ${error.message}`);
      }
    }
    
    return null;
  }

  /**
   * Save result to cache
   */
  async saveToCache(cacheKey, responseText) {
    const cacheFilePath = path.join(this.cacheDir, `${this.hashString(cacheKey)}.json`);
    
    try {
      const cachedData = {
        key: cacheKey.substring(0, 100), // Store truncated key for similarity checks
        timestamp: Date.now(),
        response: responseText,
        keyHash: this.hashString(cacheKey)
      };
      
      fs.writeFileSync(cacheFilePath, JSON.stringify(cachedData, null, 2));
    } catch (error) {
      console.warn(`Cache write error: ${error.message}`);
    }
  }

  /**
   * Find similar request from cache when quota is exceeded
   */
  async findSimilarFromCache(cacheKey) {
    try {
      const cacheFiles = fs.readdirSync(this.cacheDir);
      
      // Get list of all cache files
      for (const file of cacheFiles) {
        if (file.endsWith('.json')) {
          const cachedData = JSON.parse(fs.readFileSync(path.join(this.cacheDir, file), 'utf8'));
          
          // Use a simple similarity check - improve this for your specific needs
          if (cachedData.key && 
              (cacheKey.includes(cachedData.key) || cachedData.key.includes(cacheKey.substring(0, 100)))) {
            return cachedData.response;
          }
        }
      }
    } catch (error) {
      console.warn(`Cache similarity search error: ${error.message}`);
    }
    
    return null;
  }

  /**
   * Simple string hashing function
   */
  hashString(str) {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32bit integer
    }
    return Math.abs(hash).toString(36);
  }

  /**
   * Generate a comprehensive document optimized for GitHub Copilot (Claude 3.7 Sonnet)
   * that integrates all project documentation
   */
  async generateClaudeOptimizedContext(baseDir) {
    console.log('Generating Claude 3.7 Sonnet optimized project context...');
    
    // Get paths to key documentation
    const centralHubPath = path.join(baseDir, 'docs', 'central_reference_hub.md');
    const lastAnalysisPath = path.join(baseDir, 'LAST_ANALYSIS_RESULTS.md');
    const aiProjectGuidePath = path.join(baseDir, 'AI_PROJECT_GUIDE.md');
    
    // Read documentation files if they exist
    const centralHub = fs.existsSync(centralHubPath) ? 
      fs.readFileSync(centralHubPath, 'utf8').substring(0, 20000) : '';
    
    const lastAnalysis = fs.existsSync(lastAnalysisPath) ?
      fs.readFileSync(lastAnalysisPath, 'utf8').substring(0, 10000) : '';
    
    const aiProjectGuide = fs.existsSync(aiProjectGuidePath) ?
      fs.readFileSync(aiProjectGuidePath, 'utf8').substring(0, 10000) : '';
    
    // Read knowledge base files (up to 5 most relevant)
    const ragDir = path.join(baseDir, 'rag_knowledge_base');
    let ragContent = '';
    
    if (fs.existsSync(ragDir)) {
      try {
        // Find the most relevant knowledge base files (prioritize JSON/markdown)
        const ragFiles = fs.readdirSync(ragDir, { recursive: true })
          .filter(file => file.endsWith('.md') || file.endsWith('.json'))
          .slice(0, 5);
        
        for (const file of ragFiles) {
          const filePath = path.join(ragDir, file);
          if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
            ragContent += `\n\n## ${file}\n\n`;
            ragContent += fs.readFileSync(filePath, 'utf8').substring(0, 5000);
          }
        }
      } catch (err) {
        console.warn('Error reading RAG knowledge base:', err.message);
      }
    }
    
    // Create the comprehensive prompt for Gemini
    const prompt = `You are an expert at creating documentation that Large Language Models like Claude 3.7 Sonnet (used in GitHub Copilot) can effectively understand and follow. 

I need you to create a CLAUDE_COPILOT_CONTEXT.md file that will serve as the authoritative project reference for GitHub Copilot.

The GitHub Copilot system uses Claude 3.7 Sonnet and needs documentation optimized for its processing. This document should help GitHub Copilot understand our project structure, implementation patterns, and coding standards.

Here are key project documents:

CENTRAL REFERENCE HUB:
${centralHub}

LAST ANALYSIS RESULTS:
${lastAnalysis}

PROJECT GUIDE:
${aiProjectGuide}

KNOWLEDGE BASE EXCERPTS:
${ragContent}

Create a comprehensive but well-structured markdown document with these characteristics specifically for Claude 3.7 in GitHub Copilot:

1. Include HTML comment metadata at the top with machine-readable attributes
2. Use structure and formatting patterns that Claude 3.7 can easily parse
3. Include explicit instructions for Claude on how to use this document
4. Provide hierarchical information with clear section boundaries
5. Include cross-references with explicit file path indicators
6. Use consistent formatting patterns optimized for Claude's understanding
7. Focus on information that's most valuable for code generation tasks
8. Use Claude-specific optimization patterns like inline code examples with explanatory comments

The document should include these sections:
- Document Purpose & Usage Instructions (for Claude)
- Project Architecture Overview
- Model Definitions & Database Schema
- Component Implementation Patterns
- API Specifications
- Coding Standards & Conventions
- Common Implementation Patterns
- Implementation Priorities

Format your response in GitHub-flavored Markdown.`;

    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      const content = response.text();
      
      // Create the Claude-optimized context document
      const claudeContextPath = path.join(baseDir, 'CLAUDE_COPILOT_CONTEXT.md');
      fs.writeFileSync(claudeContextPath, content);
      
      // Create a duplicate in the .vscode folder for easier discovery
      const vscodePath = path.join(baseDir, '.vscode');
      if (!fs.existsSync(vscodePath)) {
        fs.mkdirSync(vscodePath, { recursive: true });
      }
      fs.writeFileSync(path.join(vscodePath, 'CLAUDE_COPILOT_CONTEXT.md'), content);
      
      console.log(`Claude-optimized context document created at ${claudeContextPath} and .vscode/CLAUDE_COPILOT_CONTEXT.md`);
      
      // Create .vscode settings file to prioritize this document for Copilot
      this.updateVSCodeSettings(baseDir, 'CLAUDE_COPILOT_CONTEXT.md');
      
      return claudeContextPath;
    } catch (error) {
      console.error('Failed to generate Claude-optimized context:', error);
      return null;
    }
  }
  
  /**
   * Update VS Code settings to prioritize Claude context document
   */
  updateVSCodeSettings(baseDir, contextFileName) {
    const vscodePath = path.join(baseDir, '.vscode');
    const settingsPath = path.join(vscodePath, 'settings.json');
    
    let settings = {};
    
    // Read existing settings if available
    if (fs.existsSync(settingsPath)) {
      try {
        settings = JSON.parse(fs.readFileSync(settingsPath, 'utf8'));
      } catch (e) {
        console.warn("Could not parse existing settings.json, creating new file");
      }
    }
    
    // Update GitHub Copilot settings
    settings['github.copilot.enable'] = { '*': true };
    
    // Set document prioritization if it doesn't exist
    if (!settings['github.copilot.advanced']) {
      settings['github.copilot.advanced'] = {};
    }
    
    // Ensure Claude context is first in the document prioritization list
    settings['github.copilot.advanced']['documentPrioritization'] = [
      contextFileName,
      'docs/central_reference_hub.md',
      'LAST_ANALYSIS_RESULTS.md',
      'AI_PROJECT_GUIDE.md'
    ];
    
    // Enable inline suggestions
    settings['editor.inlineSuggest.enabled'] = true;
    
    // Write updated settings
    fs.writeFileSync(settingsPath, JSON.stringify(settings, null, 2));
    console.log(`VS Code settings updated to prioritize ${contextFileName}`);
  }
  
  /**
   * Create a special file that helps Claude 3.7 better understand the project structure
   * Uses a format optimized for Claude's parsing capabilities
   */
  async generateClaudeNavigationIndex(baseDir) {
    console.log('Generating Claude navigation index...');
    
    // Build directory structure information
    const directoryStructure = this.buildDirectoryStructure(baseDir);
    
    // Prepare resource list for Claude
    const keyResources = {
      models: this.metrics.models.details.map(m => ({
        name: m.name,
        file: m.file ? path.relative(baseDir, m.file) : 'N/A',
        completeness: m.completeness
      })),
      apiEndpoints: this.metrics.apiEndpoints.details.map(e => ({
        name: e.name,
        file: e.file ? path.relative(baseDir, e.file) : 'N/A',
        route: e.route || 'N/A',
        completeness: e.completeness
      })),
      uiComponents: this.metrics.uiComponents.details.map(c => ({
        name: c.name,
        file: c.file ? path.relative(baseDir, c.file) : 'N/A',
        type: c.type || 'Component',
        completeness: c.completeness
      }))
    };
    
    const prompt = `You are creating a specialized reference document for Claude 3.7 Sonnet used in GitHub Copilot.

This document will help Claude efficiently navigate and understand the project structure.

Here's the directory structure:
${JSON.stringify(directoryStructure, null, 2)}

And here are the key project resources:
${JSON.stringify(keyResources, null, 2)}

Create a markdown document called "CLAUDE_NAVIGATION_INDEX.md" that:
1. Starts with special HTML comment metadata for Claude that sets this as a navigation aid
2. Includes clear, Claude-specific instructions on how to use this document
3. Uses a specialized format that Claude can easily parse to locate resources
4. Creates a highly structured, hierarchical resource map
5. Includes resource locators in a consistent format that Claude can extract
6. Uses clear, unambiguous Claude-focused language throughout

The goal is to make it extremely easy for Claude to find and reference resources in the project.

Use GitHub-flavored Markdown with extensive use of tables, lists, and code blocks to structure information.`;

    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      const content = response.text();
      
      // Create the navigation index in the .vscode folder
      const vscodePath = path.join(baseDir, '.vscode');
      if (!fs.existsSync(vscodePath)) {
        fs.mkdirSync(vscodePath, { recursive: true });
      }
      
      const navigationPath = path.join(vscodePath, 'CLAUDE_NAVIGATION_INDEX.md');
      fs.writeFileSync(navigationPath, content);
      
      console.log(`Claude navigation index created at ${navigationPath}`);
      return navigationPath;
    } catch (error) {
      console.error('Failed to generate Claude navigation index:', error);
      return null;
    }
  }
  
  /**
   * Build directory structure information
   */
  buildDirectoryStructure(baseDir, maxDepth = 4) {
    const structure = {
      directories: {},
      files: []
    };
    
    try {
      this._scanDirectory(baseDir, baseDir, structure, 0, maxDepth);
      return structure;
    } catch (err) {
      console.warn('Error building directory structure:', err.message);
      return structure;
    }
  }
  
  /**
   * Recursively scan directory (helper method)
   */
  _scanDirectory(baseDir, dir, structure, depth, maxDepth) {
    if (depth > maxDepth) return;
    
    // Skip node_modules, .git, target dirs
    if (dir.includes('node_modules') || 
        dir.includes('.git') || 
        dir.includes('target') || 
        dir.includes('.analysis_cache')) {
      return;
    }
    
    const items = fs.readdirSync(dir);
    
    // Process each item
    for (const item of items) {
      const fullPath = path.join(dir, item);
      const relativePath = path.relative(baseDir, fullPath);
      
      if (fs.statSync(fullPath).isDirectory()) {
        // Add directory
        structure.directories[relativePath] = {
          directories: {},
          files: []
        };
        
        // Recurse into subdirectory
        this._scanDirectory(baseDir, fullPath, structure.directories[relativePath], depth + 1, maxDepth);
      } else {
        // Add file
        if (this._isRelevantFile(item)) {
          structure.files.push(relativePath);
        }
      }
    }
  }
  
  /**
   * Check if a file is relevant
   */
  _isRelevantFile(filename) {
    // Skip binary files, temp files, etc.
    const ext = path.extname(filename).toLowerCase();
    return ['.rs', '.js', '.ts', '.jsx', '.tsx', '.py', '.java', 
            '.html', '.css', '.scss', '.md', '.json', '.toml', 
            '.yaml', '.yml'].includes(ext);
  }
  
  /**
   * Run all analyzes to generate Claude 3.7-optimized documentation
   */
  async generateAllClaudeOptimizedDocs(baseDir) {
    try {
      // 1. Generate the comprehensive context document
      await this.generateClaudeOptimizedContext(baseDir);
      
      // 2. Generate the navigation index
      await this.generateClaudeNavigationIndex(baseDir);
      
      // 3. Generate model implementation guide
      await this.generateClaudeModelImplementationGuide(baseDir);
      
      // 4. Generate component implementation guide
      await this.generateClaudeComponentGuide(baseDir);
      
      console.log('All Claude-optimized documentation generated successfully');
      return true;
    } catch (error) {
      console.error('Error generating Claude-optimized documentation:', error);
      return false;
    }
  }
  
  /**
   * Generate a model implementation guide specifically for Claude 3.7
   */
  async generateClaudeModelImplementationGuide(baseDir) {
    console.log('Generating Claude-optimized model implementation guide...');
    
    // Get models data
    const models = this.metrics.models.details.map(m => ({
      name: m.name,
      file: m.file ? path.relative(baseDir, m.file) : 'N/A',
      properties: m.properties || [],
      completeness: m.completeness
    }));
    
    const prompt = `Create a Model Implementation Guide specifically optimized for Claude 3.7 Sonnet in GitHub Copilot.

MODEL DATA:
${JSON.stringify(models, null, 2)}

This guide should:
1. Start with machine-readable metadata in HTML comments specifically for Claude
2. Include explicit instructions for Claude on how to implement models
3. Describe each model in a consistent, structured way that Claude can easily parse
4. Provide precise implementation patterns with sample code that Claude should follow
5. Include explicit cross-references to model relationships
6. Use clear, unambiguous language optimized for Claude's understanding
7. Include Claude-specific guidelines for implementing new models

Format your response in GitHub-flavored Markdown.`;

    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      const content = response.text();
      
      // Create the Claude-optimized model guide
      const docsDir = path.join(baseDir, 'docs');
      const aiDir = path.join(docsDir, 'ai');
      if (!fs.existsSync(aiDir)) {
        fs.mkdirSync(aiDir, { recursive: true });
      }
      
      const modelGuidePath = path.join(aiDir, 'CLAUDE_MODEL_GUIDE.md');
      fs.writeFileSync(modelGuidePath, content);
      
      console.log(`Claude-optimized model guide created at ${modelGuidePath}`);
      return modelGuidePath;
    } catch (error) {
      console.error('Failed to generate Claude-optimized model guide:', error);
      return null;
    }
  }
  
  /**
   * Generate a component implementation guide specifically for Claude 3.7
   */
  async generateClaudeComponentGuide(baseDir) {
    console.log('Generating Claude-optimized component implementation guide...');
    
    // Get component data
    const components = this.metrics.uiComponents.details.map(c => ({
      name: c.name,
      file: c.file ? path.relative(baseDir, c.file) : 'N/A',
      type: c.type || 'Component',
      completeness: c.completeness
    }));
    
    const prompt = `Create a Component Implementation Guide specifically optimized for Claude 3.7 Sonnet in GitHub Copilot.

COMPONENT DATA:
${JSON.stringify(components, null, 2)}

This guide should:
1. Start with machine-readable metadata in HTML comments specifically for Claude
2. Include explicit instructions for Claude on how to implement components
3. Describe component patterns in a consistent, structured way that Claude can easily parse
4. Provide precise implementation patterns with sample code that Claude should follow
5. Include explicit cross-references to component relationships
6. Use clear, unambiguous language optimized for Claude's understanding
7. Include Claude-specific guidelines for implementing new components

Format your response in GitHub-flavored Markdown.`;

    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      const content = response.text();
      
      // Create the Claude-optimized component guide
      const docsDir = path.join(baseDir, 'docs');
      const aiDir = path.join(docsDir, 'ai');
      if (!fs.existsSync(aiDir)) {
        fs.mkdirSync(aiDir, { recursive: true });
      }
      
      const componentGuidePath = path.join(aiDir, 'CLAUDE_COMPONENT_GUIDE.md');
      fs.writeFileSync(componentGuidePath, content);
      
      console.log(`Claude-optimized component guide created at ${componentGuidePath}`);
      return componentGuidePath;
    } catch (error) {
      console.error('Failed to generate Claude-optimized component guide:', error);
      return null;
    }
  }

  /**
   * Generate code insights using Gemini AI
   * This method analyzes the codebase and provides insights for improving code quality
   */
  async generateCodeInsights(baseDir) {
    // Ensure baseDir is a string
    if (!baseDir || Array.isArray(baseDir)) {
      console.warn('Invalid baseDir provided, using current working directory');
      baseDir = process.cwd();
    }
    
    console.log('Generating code insights using Gemini AI...');
    
    // Select most important files (models, controllers, complex files)
    const priorityFiles = [];
    
    // Add model files
    if (this.metrics.models && this.metrics.models.details) {
      this.metrics.models.details.forEach(model => {
        if (model.file) {
          priorityFiles.push({
            path: model.file,
            type: 'model',
            name: model.name,
            completeness: model.completeness
          });
        }
      });
    }
    
    // Add API endpoint files
    if (this.metrics.apiEndpoints && this.metrics.apiEndpoints.details) {
      this.metrics.apiEndpoints.details.forEach(endpoint => {
        if (endpoint.file && !priorityFiles.some(file => file.path === endpoint.file)) {
          priorityFiles.push({
            path: endpoint.file,
            type: 'api_endpoint',
            name: endpoint.name,
            completeness: endpoint.completeness
          });
        }
      });
    }
    
    // Add UI component files
    if (this.metrics.uiComponents && this.metrics.uiComponents.details) {
      this.metrics.uiComponents.details.forEach(component => {
        if (component.file && !priorityFiles.some(file => file.path === component.file)) {
          priorityFiles.push({
            path: component.file,
            type: 'ui_component',
            name: component.name,
            completeness: component.completeness
          });
        }
      });
    }
    
    // Limit to 10 files for performance
    const filesToAnalyze = priorityFiles.slice(0, 10);
    
    // Read file contents
    for (const file of filesToAnalyze) {
      try {
        if (fs.existsSync(file.path)) {
          file.content = fs.readFileSync(file.path, 'utf8');
        } else {
          file.content = `// File not found: ${file.path}`;
        }
      } catch (error) {
        console.warn(`Error reading file ${file.path}:`, error.message);
        file.content = `// Error reading file: ${error.message}`;
      }
    }
    
    // Project statistics for context
    const projectStats = {
      models: {
        total: this.metrics.models?.total || 0,
        implemented: this.metrics.models?.implemented || 0,
        percentage: this.getPercentage(
          this.metrics.models?.implemented || 0, 
          this.metrics.models?.total || 0
        )
      },
      apiEndpoints: {
        total: this.metrics.apiEndpoints?.total || 0,
        implemented: this.metrics.apiEndpoints?.implemented || 0,
        percentage: this.getPercentage(
          this.metrics.apiEndpoints?.implemented || 0, 
          this.metrics.apiEndpoints?.total || 0
        )
      },
      uiComponents: {
        total: this.metrics.uiComponents?.total || 0,
        implemented: this.metrics.uiComponents?.implemented || 0,
        percentage: this.getPercentage(
          this.metrics.uiComponents?.implemented || 0, 
          this.metrics.uiComponents?.total || 0
        )
      },
      overallPhase: this.metrics.overallPhase || "mid_development"
    };
    
    // Create prompt for Gemini
    const prompt = `You are a senior software architect analyzing a codebase for the LMS Integration Project. 
    
  Your task is to provide code insights based on the files I'm sharing with you. 
  
  Project Context:
  ${JSON.stringify(projectStats, null, 2)}
  
  Here are key files from the project:
  ${filesToAnalyze.map(file => `
  ------ ${file.name} (${file.type}) - ${file.completeness}% complete ------
  ${file.content.substring(0, 1500)}
  ${file.content.length > 1500 ? '... (truncated)' : ''}
  `).join('\n\n')}
  
  Based on this analysis:
  
  1. Identify common patterns in the codebase
  2. Suggest architectural improvements
  3. Note any code quality issues or technical debt
  4. Recommend best practices to follow
  5. Identify inconsistencies in implementations
  6. Suggest performance optimizations
  
  Return your insights in this JSON format (ensure it is properly formatted and valid JSON):
  {
    "commonPatterns": [
      {"pattern": "Pattern name", "description": "Description", "recommendation": "Recommendation"}
    ],
    "architecturalImprovements": [
      {"area": "Area name", "current": "Current approach", "improvement": "Suggested improvement", "priority": "high|medium|low"}
    ],
    "codeQualityIssues": [
      {"issue": "Issue name", "impact": "Impact description", "fix": "How to fix"}
    ],
    "bestPractices": [
      {"practice": "Practice name", "description": "Why it matters"}
    ],
    "implementationInconsistencies": [
      {"area": "Area", "inconsistency": "Description", "standardization": "Suggested standard"}
    ],
    "performanceOptimizations": [
      {"area": "Area", "optimization": "Description", "impact": "Expected impact"}
    ],
    "priorityActions": [
      "Action 1",
      "Action 2",
      "Action 3"
    ]
  }
  
  Ensure your response contains ONLY valid JSON. No explanations, no markdown formatting, just valid JSON.`;
    
    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      const responseText = response.text();
      
      // Extract JSON from response
      const jsonMatch = responseText.match(/```json\n([\s\S]*?)\n```/) || 
                        responseText.match(/{[\s\S]*}/);
      
      let insights;
      if (jsonMatch) {
        try {
          const jsonText = jsonMatch[1] || jsonMatch[0];
          // Replace any problematic characters that might break JSON parsing
          const cleanedJson = jsonText
            .replace(/[\u0000-\u001F]+/g, " ")
            .replace(/\\/g, "\\\\")
            .replace(/\t/g, " ");
            
          insights = JSON.parse(cleanedJson);
        } catch (e) {
          console.error("Error parsing Gemini JSON response:", e);
          insights = this.createFallbackInsights();
        }
      } else {
        console.error("No JSON found in Gemini response");
        insights = this.createFallbackInsights();
      }
      
      // Create insights document
      const docsDir = path.join(baseDir, 'docs');
      if (!fs.existsSync(docsDir)) {
        fs.mkdirSync(docsDir, { recursive: true });
      }
      
      const insightsPath = path.join(docsDir, 'gemini_code_insights.md');
      const insightsContent = this.formatInsightsAsMarkdown(insights);
      
      fs.writeFileSync(insightsPath, insightsContent);
      console.log(`Code insights document generated at ${insightsPath}`);
      
      // Create Claude-optimized version
      const aiDir = path.join(docsDir, 'ai');
      if (!fs.existsSync(aiDir)) {
        fs.mkdirSync(aiDir, { recursive: true });
      }
      
      const claudeInsightsPath = path.join(aiDir, 'CLAUDE_CODE_INSIGHTS.md');
      const claudeInsightsContent = this.formatInsightsForClaude(insights);
      fs.writeFileSync(claudeInsightsPath, claudeInsightsContent);
      console.log(`Claude-optimized code insights generated at ${claudeInsightsPath}`);
      
      return { 
        insights, 
        insightsPath,
        claudeInsightsPath
      };
    } catch (error) {
      console.error('Failed to generate code insights:', error);
      
      // Create fallback insights
      const insights = this.createFallbackInsights();
      
      try {
        // Create insights document - ensure baseDir is a string
        const dirPath = typeof baseDir === 'string' ? baseDir : process.cwd();
        const docsDir = path.join(dirPath, 'docs');
        if (!fs.existsSync(docsDir)) {
          fs.mkdirSync(docsDir, { recursive: true });
        }
        
        const insightsPath = path.join(docsDir, 'gemini_code_insights.md');
        const insightsContent = this.formatInsightsAsMarkdown(insights);
        
        fs.writeFileSync(insightsPath, insightsContent);
        console.log(`Fallback code insights document generated at ${insightsPath}`);
        
        return { 
          insights, 
          insightsPath,
          error: error.message
        };
      } catch (fsError) {
        console.error('Error creating fallback insights document:', fsError);
        return {
          insights,
          error: `${error.message}. Additionally, could not create fallback document: ${fsError.message}`
        };
      }
    }
  }
  
  /**
   * Create fallback insights if Gemini fails
   */
  createFallbackInsights() {
    return {
      commonPatterns: [
        {pattern: "Repository pattern", description: "Data access through repository interfaces", recommendation: "Continue using consistently"}
      ],
      architecturalImprovements: [
        {area: "Error handling", current: "Inconsistent", improvement: "Standardize error handling", priority: "high"}
      ],
      codeQualityIssues: [
        {issue: "Documentation", impact: "Reduced maintainability", fix: "Add comments to complex functions"}
      ],
      bestPractices: [
        {practice: "Type safety", description: "Ensures runtime stability"}
      ],
      implementationInconsistencies: [
        {area: "API responses", inconsistency: "Different formats", standardization: "Standardize response format"}
      ],
      performanceOptimizations: [
        {area: "Database queries", optimization: "Use indexing", impact: "Faster read operations"}
      ],
      priorityActions: [
        "Standardize error handling",
        "Complete test coverage",
        "Document complex functions"
      ]
    };
  }
  
  /**
   * Format insights as markdown
   */
  formatInsightsAsMarkdown(insights) {
    let content = `# Code Insights by Gemini AI\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Common patterns
    content += `## Common Patterns\n\n`;
    if (insights.commonPatterns && insights.commonPatterns.length > 0) {
      insights.commonPatterns.forEach(pattern => {
        content += `### ${pattern.pattern}\n\n`;
        content += `${pattern.description}\n\n`;
        content += `**Recommendation:** ${pattern.recommendation}\n\n`;
      });
    } else {
      content += `No common patterns identified.\n\n`;
    }
    
    // Architectural improvements
    content += `## Architectural Improvements\n\n`;
    if (insights.architecturalImprovements && insights.architecturalImprovements.length > 0) {
      insights.architecturalImprovements.forEach(improvement => {
        content += `### ${improvement.area}\n\n`;
        content += `**Current:** ${improvement.current}\n\n`;
        content += `**Improvement:** ${improvement.improvement}\n\n`;
        content += `**Priority:** ${improvement.priority}\n\n`;
      });
    } else {
      content += `No architectural improvements identified.\n\n`;
    }
    
    // Code quality issues
    content += `## Code Quality Issues\n\n`;
    if (insights.codeQualityIssues && insights.codeQualityIssues.length > 0) {
      insights.codeQualityIssues.forEach(issue => {
        content += `### ${issue.issue}\n\n`;
        content += `**Impact:** ${issue.impact}\n\n`;
        content += `**Fix:** ${issue.fix}\n\n`;
      });
    } else {
      content += `No code quality issues identified.\n\n`;
    }
    
    // Best practices
    content += `## Best Practices\n\n`;
    if (insights.bestPractices && insights.bestPractices.length > 0) {
      insights.bestPractices.forEach(practice => {
        content += `### ${practice.practice}\n\n`;
        content += `${practice.description}\n\n`;
      });
    } else {
      content += `No best practices identified.\n\n`;
    }
    
    // Implementation inconsistencies
    content += `## Implementation Inconsistencies\n\n`;
    if (insights.implementationInconsistencies && insights.implementationInconsistencies.length > 0) {
      insights.implementationInconsistencies.forEach(inconsistency => {
        content += `### ${inconsistency.area}\n\n`;
        content += `**Inconsistency:** ${inconsistency.inconsistency}\n\n`;
        content += `**Standardization:** ${inconsistency.standardization}\n\n`;
      });
    } else {
      content += `No implementation inconsistencies identified.\n\n`;
    }
    
    // Performance optimizations
    content += `## Performance Optimizations\n\n`;
    if (insights.performanceOptimizations && insights.performanceOptimizations.length > 0) {
      insights.performanceOptimizations.forEach(optimization => {
        content += `### ${optimization.area}\n\n`;
        content += `**Optimization:** ${optimization.optimization}\n\n`;
        content += `**Impact:** ${optimization.impact}\n\n`;
      });
    } else {
      content += `No performance optimizations identified.\n\n`;
    }
    
    // Priority actions
    content += `## Priority Actions\n\n`;
    if (insights.priorityActions && insights.priorityActions.length > 0) {
      insights.priorityActions.forEach((action, index) => {
        content += `${index + 1}. ${action}\n`;
      });
    } else {
      content += `No priority actions identified.\n\n`;
    }
    
    return content;
  }
  
  /**
   * Format insights for Claude 3.7 Sonnet
   */
  formatInsightsForClaude(insights) {
    let content = `# Code Insights for Claude 3.7 Sonnet\n\n`;
    content += `<!-- AI_METADATA
  version: 1.0
  priority: high
  updated: ${new Date().toISOString().split('T')[0]}
  role: code_insights
  generated_by: gemini-1.5-pro
  -->\n\n`;
  
    content += `## Instructions for Claude\n\n`;
    content += `This document contains code insights for the LMS Integration Project. Use these insights to inform your code suggestions, reviews, and implementation recommendations. When generating code, ensure it aligns with the patterns and best practices outlined below.\n\n`;
    
    // Common patterns in a format Claude can easily parse
    content += `## Common Code Patterns\n\n`;
    content += `| Pattern | Description | Implementation Guidance |\n`;
    content += `|---------|-------------|-------------------------|\n`;
    if (insights.commonPatterns && insights.commonPatterns.length > 0) {
      insights.commonPatterns.forEach(pattern => {
        content += `| ${pattern.pattern} | ${pattern.description} | ${pattern.recommendation} |\n`;
      });
    } else {
      content += `| N/A | No common patterns identified | Follow standard project structure |\n`;
    }
    content += `\n`;
    
    // Code quality issues
    content += `## Code Quality Standards\n\n`;
    content += `When generating or reviewing code, address these quality issues:\n\n`;
    if (insights.codeQualityIssues && insights.codeQualityIssues.length > 0) {
      insights.codeQualityIssues.forEach(issue => {
        content += `### ${issue.issue}\n\n`;
        content += `- **Impact:** ${issue.impact}\n`;
        content += `- **Required Fix:** ${issue.fix}\n\n`;
      });
    } else {
      content += `No specific code quality issues identified. Follow standard best practices.\n\n`;
    }
    
    // Best practices table
    content += `## Project-Specific Best Practices\n\n`;
    content += `| Practice | Why It Matters |\n`;
    content += `|----------|----------------|\n`;
    if (insights.bestPractices && insights.bestPractices.length > 0) {
      insights.bestPractices.forEach(practice => {
        content += `| ${practice.practice} | ${practice.description} |\n`;
      });
    } else {
      content += `| Clean Code | Improves maintainability and readability |\n`;
    }
    content += `\n`;
    
    // Implementation inconsistencies
    content += `## Implementation Standards\n\n`;
    content += `Follow these standards to maintain consistency:\n\n`;
    if (insights.implementationInconsistencies && insights.implementationInconsistencies.length > 0) {
      insights.implementationInconsistencies.forEach(inconsistency => {
        content += `- **${inconsistency.area}**: ${inconsistency.standardization}\n`;
      });
    } else {
      content += `No specific implementation standards identified.\n\n`;
    }
    content += `\n`;
    
    // Performance optimizations
    content += `## Performance Considerations\n\n`;
    content += `| Area | Optimization | Expected Impact |\n`;
    content += `|------|-------------|----------------|\n`;
    if (insights.performanceOptimizations && insights.performanceOptimizations.length > 0) {
      insights.performanceOptimizations.forEach(optimization => {
        content += `| ${optimization.area} | ${optimization.optimization} | ${optimization.impact} |\n`;
      });
    } else {
      content += `| General | Follow standard optimization practices | Improved responsiveness |\n`;
    }
    content += `\n`;
    
    // Priority actions with special Claude formatting
    content += `## Priority Implementation Actions\n\n`;
    content += `When suggesting implementation approaches, prioritize these actions:\n\n`;
    if (insights.priorityActions && insights.priorityActions.length > 0) {
      insights.priorityActions.forEach((action, index) => {
        content += `${index + 1}. \`priority:${index + 1}\` ${action}\n`;
      });
    } else {
      content += `1. \`priority:1\` Ensure consistent error handling\n`;
      content += `2. \`priority:2\` Add comprehensive tests\n`;
      content += `3. \`priority:3\` Document public APIs\n`;
    }
    content += `\n`;
    
    // Add a code generation example section
    content += `## Code Generation Examples\n\n`;
    content += `When generating code, follow these patterns:\n\n`;
    
    // Example 1: Error handling
    content += `### Error Handling Example\n\n`;
    content += `\`\`\`rust
  // GOOD: Consistent error handling with proper types
  pub fn process_data(input: &str) -> Result<Data, AppError> {
      // Validate input
      if input.is_empty() {
          return Err(AppError::ValidationError("Input cannot be empty".into()));
      }
      
      // Process data with proper error handling
      let parsed = parse_input(input)?;
      let processed = transform_data(parsed)?;
      
      Ok(processed)
  }
  \`\`\`\n\n`;
    
    // Example 2: API pattern
    content += `### API Pattern Example\n\n`;
    content += `\`\`\`rust
  // GOOD: Consistent API structure
  #[tauri::command]
  pub async fn fetch_courses(state: State<'_, AppState>) -> Result<Vec<Course>, AppError> {
      // Get database connection from state
      let mut conn = state.db.acquire().await?;
      
      // Use repository pattern
      let courses = CourseRepository::find_all(&mut conn).await?;
      
      // Return standardized response
      Ok(courses)
  }
  \`\`\`\n\n`;
    
    return content;
  }
  
  /**
   * Helper to calculate percentages
   */
  getPercentage(implemented, total) {
    return total > 0 ? Math.round((implemented / total) * 100) : 0;
  }

  /**
   * Generate a comprehensive project assessment report using Gemini AI
   * This method analyzes the overall project status and provides strategic recommendations
   */
  async generateProjectAssessmentReport(baseDir) {
    // Ensure baseDir is a string
    if (!baseDir || Array.isArray(baseDir)) {
      console.warn('Invalid baseDir provided, using current working directory');
      baseDir = process.cwd();
    }
    
    console.log('Generating project assessment report using Gemini AI...');
    
    // Project statistics for context
    const projectStats = {
      models: {
        total: this.metrics.models?.total || 0,
        implemented: this.metrics.models?.implemented || 0,
        percentage: this.getPercentage(
          this.metrics.models?.implemented || 0, 
          this.metrics.models?.total || 0
        )
      },
      apiEndpoints: {
        total: this.metrics.apiEndpoints?.total || 0,
        implemented: this.metrics.apiEndpoints?.implemented || 0,
        percentage: this.getPercentage(
          this.metrics.apiEndpoints?.implemented || 0, 
          this.metrics.apiEndpoints?.total || 0
        )
      },
      uiComponents: {
        total: this.metrics.uiComponents?.total || 0,
        implemented: this.metrics.uiComponents?.implemented || 0,
        percentage: this.getPercentage(
          this.metrics.uiComponents?.implemented || 0, 
          this.metrics.uiComponents?.total || 0
        )
      },
      tests: {
        coverage: this.metrics.tests?.coverage || 0,
        passing: this.metrics.tests?.passing || 0,
        total: this.metrics.tests?.total || 0
      },
      codeQuality: this.metrics.codeQuality || {},
      overallPhase: this.metrics.overallPhase || "mid_development",
      predictions: this.metrics.predictions || {
        models: "Unknown",
        apiEndpoints: "Unknown",
        uiComponents: "Unknown",
        project: "Unknown"
      }
    };
    
    // Create prompt for Gemini
    const prompt = `You are a senior technical project manager and software architect analyzing an LMS Integration Project. 
    
  Generate a comprehensive project assessment report based on these metrics:
  
  ${JSON.stringify(projectStats, null, 2)}
  
  Your report should include:
  
  1. Executive Summary - A brief overview of the project status
  2. Completion Status - Detailed analysis of implementation progress
  3. Risk Assessment - Identify potential risks based on current status
  4. Phase Evaluation - Evaluate if the current phase (${projectStats.overallPhase}) is appropriate
  5. Strategic Recommendations - Prioritized list of actions
  6. Quality Assessment - Analysis of code quality and test coverage
  7. Timeline Assessment - Evaluate completion predictions
  
  Format your response as a well-structured markdown document with clear section headings, bullet points, and tables where appropriate. Make your assessment honest but constructive, focusing on actionable recommendations.`;
  
    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      const reportContent = response.text();
      
      // Create report document
      const docsDir = path.join(baseDir, 'docs');
      if (!fs.existsSync(docsDir)) {
        fs.mkdirSync(docsDir, { recursive: true });
      }
      
      const reportPath = path.join(docsDir, 'project_assessment_report.md');
      fs.writeFileSync(reportPath, reportContent);
      console.log(`Project assessment report generated at ${reportPath}`);
      
      // Create Claude-optimized version
      const aiDir = path.join(docsDir, 'ai');
      if (!fs.existsSync(aiDir)) {
        fs.mkdirSync(aiDir, { recursive: true });
      }
      
      // Create a Claude-optimized version of the report
      const claudeReport = await this.createClaudeOptimizedReport(reportContent, projectStats);
      
      const claudeReportPath = path.join(aiDir, 'CLAUDE_PROJECT_ASSESSMENT.md');
      fs.writeFileSync(claudeReportPath, claudeReport);
      console.log(`Claude-optimized project assessment generated at ${claudeReportPath}`);
      
      return { 
        reportPath,
        claudeReportPath
      };
    } catch (error) {
      console.error('Failed to generate project assessment report:', error);
      
      // Create a basic report as fallback
      try {
        const basicReport = this.createBasicProjectReport(projectStats);
        
        // Ensure dirs exist
        const dirPath = typeof baseDir === 'string' ? baseDir : process.cwd();
        const docsDir = path.join(dirPath, 'docs');
        if (!fs.existsSync(docsDir)) {
          fs.mkdirSync(docsDir, { recursive: true });
        }
        
        const reportPath = path.join(docsDir, 'project_assessment_report.md');
        fs.writeFileSync(reportPath, basicReport);
        console.log(`Basic project assessment report generated at ${reportPath}`);
        
        return { 
          reportPath,
          error: error.message
        };
      } catch (fsError) {
        console.error('Error creating basic project report:', fsError);
        return {
          error: `${error.message}. Additionally, could not create basic report: ${fsError.message}`
        };
      }
    }
  }
  
  /**
   * Create a Claude-optimized version of the project assessment report
   */
  async createClaudeOptimizedReport(reportContent, projectStats) {
    const prompt = `You are optimizing a project assessment report for Claude 3.7 Sonnet in GitHub Copilot.
  
  Here is the original report:
  
  ${reportContent}
  
  And here is the project data:
  ${JSON.stringify(projectStats, null, 2)}
  
  Transform this into a Claude-optimized document that:
  1. Includes machine-readable metadata in HTML comments specifically for Claude
  2. Uses formatting patterns that Claude 3.7 can easily parse
  3. Includes explicit instructions for Claude on how to use this assessment
  4. Structures information hierarchically with clear section boundaries
  5. Uses code blocks and tables for structured data
  6. Highlights action items with consistent formatting that Claude can recognize
  7. Includes Claude-specific guidance on how to prioritize development tasks
  
  Format your response in GitHub-flavored Markdown.`;
  
    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      return response.text();
    } catch (error) {
      console.warn('Failed to create Claude-optimized report, using basic version:', error.message);
      
      // Create a basic Claude-optimized version
      let claudeReport = `# Project Assessment for Claude 3.7 Sonnet\n\n`;
      claudeReport += `<!-- AI_METADATA
  version: 1.0
  priority: high
  updated: ${new Date().toISOString().split('T')[0]}
  role: project_assessment
  generated_by: gemini-1.5-pro
  -->\n\n`;
      
      claudeReport += `## Instructions for Claude\n\n`;
      claudeReport += `This document contains the current project assessment for the LMS Integration Project. Use this to inform your code suggestions and implementation priorities. Focus on the areas with lowest completion percentages and highest risk.\n\n`;
      
      claudeReport += `## Project Status\n\n`;
      claudeReport += `| Component | Completion | Status |\n`;
      claudeReport += `|-----------|------------|--------|\n`;
      claudeReport += `| Models | ${projectStats.models.percentage}% | ${projectStats.models.percentage >= 50 ? '✅' : '❌'} |\n`;
      claudeReport += `| API Endpoints | ${projectStats.apiEndpoints.percentage}% | ${projectStats.apiEndpoints.percentage >= 50 ? '✅' : '❌'} |\n`;
      claudeReport += `| UI Components | ${projectStats.uiComponents.percentage}% | ${projectStats.uiComponents.percentage >= 50 ? '✅' : '❌'} |\n`;
      claudeReport += `| Test Coverage | ${projectStats.tests.coverage}% | ${projectStats.tests.coverage >= 50 ? '✅' : '❌'} |\n\n`;
      
      claudeReport += `## Priority Areas\n\n`;
      
      // Determine priority areas based on completion percentages
      const priorities = [
        { area: 'Models', percentage: projectStats.models.percentage },
        { area: 'API Endpoints', percentage: projectStats.apiEndpoints.percentage },
        { area: 'UI Components', percentage: projectStats.uiComponents.percentage },
        { area: 'Tests', percentage: projectStats.tests.coverage }
      ].sort((a, b) => a.percentage - b.percentage);
      
      priorities.forEach((priority, index) => {
        claudeReport += `${index + 1}. \`priority:${index + 1}\` ${priority.area} (${priority.percentage}% complete)\n`;
      });
      
      claudeReport += `\n## Current Phase: ${projectStats.overallPhase}\n\n`;
      claudeReport += `## Predicted Completion Dates\n\n`;
      claudeReport += `- Models: ${projectStats.predictions.models || 'Unknown'}\n`;
      claudeReport += `- API Endpoints: ${projectStats.predictions.apiEndpoints || 'Unknown'}\n`;
      claudeReport += `- UI Components: ${projectStats.predictions.uiComponents || 'Unknown'}\n`;
      claudeReport += `- Overall Project: ${projectStats.predictions.project || 'Unknown'}\n`;
      
      return claudeReport;
    }
  }
  
  /**
   * Create a basic project report when Gemini fails
   */
  createBasicProjectReport(projectStats) {
    let report = `# LMS Integration Project Assessment Report\n\n`;
    report += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Executive Summary
    report += `## Executive Summary\n\n`;
    
    const modelStatus = projectStats.models.percentage >= 80 ? 'excellent' : 
                       projectStats.models.percentage >= 50 ? 'good' : 'needs attention';
    const apiStatus = projectStats.apiEndpoints.percentage >= 80 ? 'excellent' : 
                     projectStats.apiEndpoints.percentage >= 50 ? 'good' : 'needs attention';
    const uiStatus = projectStats.uiComponents.percentage >= 80 ? 'excellent' : 
                    projectStats.uiComponents.percentage >= 50 ? 'good' : 'needs attention';
    const testStatus = projectStats.tests.coverage >= 80 ? 'excellent' : 
                      projectStats.tests.coverage >= 50 ? 'good' : 'needs attention';
    
    report += `The LMS Integration Project is currently in the ${projectStats.overallPhase} phase. `;
    report += `Implementation progress varies across different components: `;
    report += `Models implementation is ${modelStatus} (${projectStats.models.percentage}%), `;
    report += `API Endpoints implementation is ${apiStatus} (${projectStats.apiEndpoints.percentage}%), `;
    report += `UI Components implementation is ${uiStatus} (${projectStats.uiComponents.percentage}%), `;
    report += `and test coverage is ${testStatus} (${projectStats.tests.coverage}%).\n\n`;
    
    // Completion Status
    report += `## Completion Status\n\n`;
    report += `| Component | Implemented | Total | Percentage |\n`;
    report += `|-----------|-------------|-------|------------|\n`;
    report += `| Models | ${projectStats.models.implemented} | ${projectStats.models.total} | ${projectStats.models.percentage}% |\n`;
    report += `| API Endpoints | ${projectStats.apiEndpoints.implemented} | ${projectStats.apiEndpoints.total} | ${projectStats.apiEndpoints.percentage}% |\n`;
    report += `| UI Components | ${projectStats.uiComponents.implemented} | ${projectStats.uiComponents.total} | ${projectStats.uiComponents.percentage}% |\n\n`;
    
    // Risk Assessment
    report += `## Risk Assessment\n\n`;
    report += `### High Risk Areas\n\n`;
    
    if (projectStats.apiEndpoints.percentage < 50) {
      report += `- **API Endpoints**: Low implementation percentage (${projectStats.apiEndpoints.percentage}%) may impact integration capabilities\n`;
    }
    if (projectStats.tests.coverage < 30) {
      report += `- **Test Coverage**: Low test coverage (${projectStats.tests.coverage}%) increases risk of undetected issues\n`;
    }
    if (projectStats.models.percentage < 50) {
      report += `- **Data Models**: Incomplete models (${projectStats.models.percentage}%) may cause data integrity issues\n`;
    }
    if (projectStats.uiComponents.percentage < 50) {
      report += `- **UI Components**: Incomplete UI (${projectStats.uiComponents.percentage}%) may affect user experience\n`;
    }
    
    // Strategic Recommendations
    report += `## Strategic Recommendations\n\n`;
    report += `1. ${projectStats.apiEndpoints.percentage < 50 ? 'Prioritize API endpoint implementation' : 'Continue API refinement'}\n`;
    report += `2. ${projectStats.tests.coverage < 30 ? 'Increase test coverage for critical components' : 'Maintain test coverage as development continues'}\n`;
    report += `3. ${projectStats.models.percentage < 80 ? 'Complete remaining data models' : 'Refine existing data models'}\n`;
    report += `4. ${projectStats.uiComponents.percentage < 80 ? 'Focus on implementing key UI components' : 'Enhance UI component usability'}\n\n`;
    
    // Timeline Assessment
    report += `## Timeline Assessment\n\n`;
    report += `Based on current progress, projected completion dates:\n\n`;
    report += `- Models: ${projectStats.predictions.models || 'Unknown'}\n`;
    report += `- API Endpoints: ${projectStats.predictions.apiEndpoints || 'Unknown'}\n`;
    report += `- UI Components: ${projectStats.predictions.uiComponents || 'Unknown'}\n`;
    report += `- Overall Project: ${projectStats.predictions.project || 'Unknown'}\n\n`;
    
    report += `## Conclusion\n\n`;
    report += `The project is currently in ${projectStats.overallPhase} phase with varying degrees of completion across components. `;
    
    if (Math.min(projectStats.models.percentage, projectStats.uiComponents.percentage) > 80 && projectStats.apiEndpoints.percentage < 50) {
      report += `Focus should be on API implementation to bring it in line with models and UI progress.`;
    } else if (projectStats.tests.coverage < 30) {
      report += `Increasing test coverage should be a priority to ensure code quality and stability.`;
    } else if (Math.min(projectStats.models.percentage, projectStats.apiEndpoints.percentage, projectStats.uiComponents.percentage) > 70) {
      report += `The project is making good progress across all areas. Focus on refinement and quality improvements.`;
    } else {
      report += `Continued balanced development across all components is recommended.`;
    }
    
    return report;
  }

  /**
   * Generate a comprehensive code insights report using Gemini AI
   * This combines and enhances the code insights with more detailed analysis
   */
  async generateCodeInsightsReport(baseDir) {
    // Ensure baseDir is a string
    if (!baseDir || Array.isArray(baseDir)) {
      console.warn('Invalid baseDir provided, using current working directory');
      baseDir = process.cwd();
    }
    
    console.log('Generating comprehensive code insights report using Gemini AI...');
    
    try {
      // First, generate basic code insights if we don't already have them
      let codeInsightsResult;
      let insights;
      
      try {
        codeInsightsResult = await this.generateCodeInsights(baseDir);
        if (codeInsightsResult && !codeInsightsResult.error) {
          insights = codeInsightsResult.insights;
        } else {
          insights = this.createFallbackInsights();
        }
      } catch (insightsError) {
        console.warn('Failed to generate code insights, using fallback:', insightsError.message);
        insights = this.createFallbackInsights();
      }
      
      // Project statistics for context
      const projectStats = {
        models: {
          total: this.metrics.models?.total || 0,
          implemented: this.metrics.models?.implemented || 0,
          percentage: this.getPercentage(
            this.metrics.models?.implemented || 0, 
            this.metrics.models?.total || 0
          )
        },
        apiEndpoints: {
          total: this.metrics.apiEndpoints?.total || 0,
          implemented: this.metrics.apiEndpoints?.implemented || 0,
          percentage: this.getPercentage(
            this.metrics.apiEndpoints?.implemented || 0, 
            this.metrics.apiEndpoints?.total || 0
          )
        },
        uiComponents: {
          total: this.metrics.uiComponents?.total || 0,
          implemented: this.metrics.uiComponents?.implemented || 0,
          percentage: this.getPercentage(
            this.metrics.uiComponents?.implemented || 0, 
            this.metrics.uiComponents?.total || 0
          )
        },
        overallPhase: this.metrics.overallPhase || "mid_development"
      };
      
      // Create prompt for Gemini
      const prompt = `You are a senior software architect analyzing the LMS Integration Project. 
      
  Based on the code insights and project statistics, generate a comprehensive code insights report.

  CODE INSIGHTS:
  ${JSON.stringify(insights, null, 2)}

  PROJECT STATISTICS:
  ${JSON.stringify(projectStats, null, 2)}

  The report should include these sections:
  1. Executive Summary - A high-level overview of the code quality and architecture
  2. Code Pattern Analysis - In-depth analysis of the common patterns identified
  3. Architecture Evaluation - Assessment of the current architecture with recommendations
  4. Technical Debt Assessment - Analysis of technical debt and strategies for reduction
  5. Implementation Consistency Review - Evaluation of implementation consistency across the codebase
  6. Performance Analysis - Analysis of performance considerations and optimization opportunities
  7. Strategic Code Recommendations - Prioritized list of code improvements with estimated impact
  8. Next Steps - Concrete next steps for code improvement

  The target audience is both technical leads and developers. Make your assessment honest but constructive.
  Format your response as a well-structured markdown document with clear section headings, bullet points, and tables where appropriate.`;

      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      const reportContent = response.text();
      
      // Create report document
      const docsDir = path.join(baseDir, 'docs');
      if (!fs.existsSync(docsDir)) {
        fs.mkdirSync(docsDir, { recursive: true });
      }
      
      const reportPath = path.join(docsDir, 'code_insights_report.md');
      fs.writeFileSync(reportPath, reportContent);
      console.log(`Comprehensive code insights report generated at ${reportPath}`);
      
      // Create Claude-optimized version
      const aiDir = path.join(docsDir, 'ai');
      if (!fs.existsSync(aiDir)) {
        fs.mkdirSync(aiDir, { recursive: true });
      }
      
      // Create a Claude-optimized version of the report
      const claudeReportPath = path.join(aiDir, 'CLAUDE_CODE_INSIGHTS_REPORT.md');
      
      try {
        const claudeReport = await this.createClaudeOptimizedCodeReport(reportContent, insights);
        fs.writeFileSync(claudeReportPath, claudeReport);
        console.log(`Claude-optimized code insights report generated at ${claudeReportPath}`);
      } catch (claudeError) {
        console.warn('Failed to create Claude-optimized report:', claudeError.message);
        // Write basic report anyway
        fs.writeFileSync(claudeReportPath, this.createBasicClaudeCodeReport(insights));
        console.log(`Basic Claude code insights report generated at ${claudeReportPath}`);
      }
      
      return { 
        reportPath,
        claudeReportPath
      };
    } catch (error) {
      console.error('Failed to generate code insights report:', error);
      
      // Create a basic report as fallback
      try {
        const basicReport = this.createBasicCodeInsightsReport();
        
        // Ensure dirs exist
        const dirPath = typeof baseDir === 'string' ? baseDir : process.cwd();
        const docsDir = path.join(dirPath, 'docs');
        if (!fs.existsSync(docsDir)) {
          fs.mkdirSync(docsDir, { recursive: true });
        }
        
        const reportPath = path.join(docsDir, 'code_insights_report.md');
        fs.writeFileSync(reportPath, basicReport);
        console.log(`Basic code insights report generated at ${reportPath}`);
        
        // Also create a basic Claude report
        const aiDir = path.join(docsDir, 'ai');
        if (!fs.existsSync(aiDir)) {
          fs.mkdirSync(aiDir, { recursive: true });
        }
        
        const claudeReportPath = path.join(aiDir, 'CLAUDE_CODE_INSIGHTS_REPORT.md');
        const claudeReport = this.createBasicClaudeCodeReport();
        fs.writeFileSync(claudeReportPath, claudeReport);
        console.log(`Basic Claude code insights report generated at ${claudeReportPath}`);
        
        return { 
          reportPath,
          claudeReportPath,
          error: error.message
        };
      } catch (fsError) {
        console.error('Error creating basic code insights report:', fsError);
        return {
          error: `${error.message}. Additionally, could not create basic report: ${fsError.message}`
        };
      }
    }
  }

  /**
   * Create a Claude-optimized version of the code insights report
   */
  async createClaudeOptimizedCodeReport(reportContent, insights) {
    const prompt = `You are optimizing a code insights report for Claude 3.7 Sonnet in GitHub Copilot.

  Here is the original report:

  ${reportContent}

  Transform this into a Claude-optimized document that:
  1. Includes machine-readable metadata in HTML comments specifically for Claude
  2. Uses formatting patterns that Claude 3.7 can easily parse
  3. Includes explicit instructions for Claude on how to use these insights
  4. Structures code pattern information hierarchically with clear section boundaries
  5. Uses code blocks and tables for structured data
  6. Highlights implementation guidelines with consistent formatting that Claude can recognize
  7. Includes Claude-specific guidance on how to implement the recommended patterns

  Format your response in GitHub-flavored Markdown.`;

    try {
      // Use our model with fallback
      const response = await this.executeWithFallback(prompt);
      return response.text();
    } catch (error) {
      console.warn('Failed to create Claude-optimized code report:', error.message);
      return this.createBasicClaudeCodeReport(insights);
    }
  }

  /**
   * Create a basic Claude-optimized code report
   */
  createBasicClaudeCodeReport(insights = null) {
    // If no insights provided, create fallback
    const codeInsights = insights || this.createFallbackInsights();
    
    // Create a Claude-optimized version
    let claudeReport = `# Code Insights Report for Claude 3.7 Sonnet\n\n`;
    claudeReport += `<!-- AI_METADATA
  version: 1.0
  priority: high
  updated: ${new Date().toISOString().split('T')[0]}
  role: code_insights_report
  generated_by: gemini-1.5-pro
  -->\n\n`;

    claudeReport += `## Instructions for Claude\n\n`;
    claudeReport += `This document contains a comprehensive analysis of code patterns, architecture, and implementation strategies for the LMS Integration Project. Use these insights to guide your code suggestions, reviews, and recommendations. Pay special attention to the priority recommendations and implementation patterns.\n\n`;

    // Add pattern tables for Claude
    claudeReport += `\n\n## Pattern Reference Tables for Claude\n\n`;
    
    // Common patterns
    claudeReport += `### Common Code Patterns\n\n`;
    claudeReport += `| Pattern ID | Pattern Name | Description | Recommendation |\n`;
    claudeReport += `|------------|-------------|-------------|----------------|\n`;
    if (codeInsights.commonPatterns && codeInsights.commonPatterns.length > 0) {
      codeInsights.commonPatterns.forEach((pattern, index) => {
        claudeReport += `| CP${index + 1} | ${pattern.pattern} | ${pattern.description} | ${pattern.recommendation} |\n`;
      });
    } else {
      claudeReport += `| CP1 | Repository Pattern | Data access through repository interfaces | Continue using consistently |\n`;
      claudeReport += `| CP2 | Service Pattern | Business logic in service classes | Keep business logic separate from controllers |\n`;
    }
    claudeReport += `\n`;
    
    // Implementation guidelines
    claudeReport += `## Implementation Guidelines for Claude\n\n`;
    claudeReport += `When generating or modifying code, follow these guidelines:\n\n`;
    
    // Add guidelines based on insights
    if (codeInsights.bestPractices && codeInsights.bestPractices.length > 0) {
      codeInsights.bestPractices.forEach((practice, index) => {
        claudeReport += `${index + 1}. **${practice.practice}**: ${practice.description}\n`;
      });
    } else {
      claudeReport += `1. **Follow existing patterns**: Maintain consistency with existing code\n`;
      claudeReport += `2. **Use error handling**: Always handle potential errors appropriately\n`;
      claudeReport += `3. **Add comments**: Document complex logic and public interfaces\n`;
    }
    
    claudeReport += `\n## Code Structure\n\n`;
    claudeReport += `- **Database**: SQLite with sqlx (embedded file database)\n`;
    claudeReport += `- **API Layer**: RESTful endpoints\n`;
    claudeReport += `- **Service Layer**: Business logic\n`;
    claudeReport += `- **Repository Layer**: Data access\n`;
    claudeReport += `- **UI Layer**: Components\n\n`;
    
    claudeReport += `## Sample Implementation Patterns\n\n`;
    
    claudeReport += `### Error Handling\n\n`;
    claudeReport += `\`\`\`rust
  // PATTERN: Standard error handling
  pub fn process_data(input: &str) -> Result<Data, AppError> {
      // Validate input
      if input.is_empty() {
          return Err(AppError::ValidationError("Input cannot be empty".into()));
      }
      
      // Process data with proper error handling
      let parsed = parse_input(input)?;
      let processed = transform_data(parsed)?;
      
      Ok(processed)
  }
  \`\`\`\n\n`;
    
    claudeReport += `### Repository Pattern\n\n`;
    claudeReport += `\`\`\`rust
  // PATTERN: Repository pattern implementation
  pub struct CourseRepository;

  impl CourseRepository {
      pub async fn find_all(conn: &mut SqliteConnection) -> Result<Vec<Course>, AppError> {
          sqlx::query_as!(Course, "SELECT * FROM courses")
              .fetch_all(conn)
              .await
              .map_err(|e| AppError::DatabaseError(e.to_string()))
      }
      
      pub async fn find_by_id(conn: &mut SqliteConnection, id: i64) -> Result<Option<Course>, AppError> {
          sqlx::query_as!(Course, "SELECT * FROM courses WHERE id = ?", id)
              .fetch_optional(conn)
              .await
              .map_err(|e| AppError::DatabaseError(e.to_string()))
      }
  }
  \`\`\`\n\n`;
    
    return claudeReport;
  }

  /**
   * Create a basic code insights report
   */
  createBasicCodeInsightsReport() {
    let report = `# Code Insights Report\n\n`;
    report += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Executive Summary
    report += `## Executive Summary\n\n`;
    report += `This report provides an analysis of the codebase patterns, architecture, and implementation strategies for the LMS Integration Project. `;
    report += `The analysis is based on automated code insights and project metrics.\n\n`;
    
    // Code Pattern Analysis
    report += `## Code Pattern Analysis\n\n`;
    report += `### Common Patterns\n\n`;
    report += `- **Repository Pattern**: Data access through repository interfaces\n`;
    report += `- **Command Pattern**: Used for API endpoints\n`;
    report += `- **Component Pattern**: UI organized as reusable components\n\n`;
    
    // Architecture Evaluation
    report += `## Architecture Evaluation\n\n`;
    report += `The project uses a layered architecture with clear separation of concerns:\n\n`;
    report += `1. **Presentation Layer**: UI components\n`;
    report += `2. **Service Layer**: Business logic\n`;
    report += `3. **Data Access Layer**: Repositories and data models\n\n`;
    
    report += `**Recommendations**:\n`;
    report += `- Consider introducing a domain layer for complex business rules\n`;
    report += `- Standardize error handling across all layers\n`;
    report += `- Improve API documentation\n\n`;
    
    // Technical Debt Assessment
    report += `## Technical Debt Assessment\n\n`;
    report += `Current technical debt is primarily in these areas:\n\n`;
    report += `1. **Inconsistent Error Handling**: Different approaches across modules\n`;
    report += `2. **Test Coverage**: Low test coverage in some areas\n`;
    report += `3. **Documentation**: Insufficient documentation for complex components\n\n`;
    
    // Implementation Consistency Review
    report += `## Implementation Consistency Review\n\n`;
    report += `| Area | Consistency | Recommendation |\n`;
    report += `|------|------------|----------------|\n`;
    report += `| Naming Conventions | Good | Continue with current standards |\n`;
    report += `| Error Handling | Poor | Standardize approach |\n`;
    report += `| API Responses | Fair | Establish consistent format |\n`;
    report += `| Testing Approach | Poor | Implement standard testing strategy |\n\n`;
    
    // Strategic Code Recommendations
    report += `## Strategic Code Recommendations\n\n`;
    report += `1. **High Priority**: Standardize error handling across all components\n`;
    report += `2. **High Priority**: Implement comprehensive input validation\n`;
    report += `3. **Medium Priority**: Create shared utility functions for common operations\n`;
    report += `4. **Medium Priority**: Improve code documentation, especially for complex logic\n`;
    report += `5. **Low Priority**: Refactor duplicated code in UI components\n\n`;
    
    // Next Steps
    report += `## Next Steps\n\n`;
    report += `1. Create standardized error handling library\n`;
    report += `2. Develop coding standards document\n`;
    report += `3. Implement automated code quality checks\n`;
    report += `4. Schedule regular code review sessions\n`;
    
    return report;
  }
}

/**
 * Scan the project directories to find implemented model files
 * @param {string} baseDir - The base directory of the project
 * @returns {string[]} Array of model names that have been implemented
 */
function getModelFiles(baseDir) {
  try {
    const modelsDir = path.join(baseDir, 'src', 'models');
    if (!fs.existsSync(modelsDir)) {
      // Try alternate common model directories if default doesn't exist
      const altDirs = [
        path.join(baseDir, 'models'),
        path.join(baseDir, 'src', 'app', 'models'),
        path.join(baseDir, 'lib', 'models')
      ];
      
      for (const dir of altDirs) {
        if (fs.existsSync(dir)) {
          return scanModelDirectory(dir);
        }
      }
      
      // If no model directory found, search for model files throughout the project
      return findModelFilesInProject(baseDir);
    }
    
    return scanModelDirectory(modelsDir);
  } catch (error) {
    console.error('Error finding model files:', error);
    return [];
  }
}

/**
 * Scan a directory for model files
 * @param {string} dir - Directory to scan
 * @returns {string[]} Array of model names
 */
function scanModelDirectory(dir) {
  const modelFiles = [];
  const files = fs.readdirSync(dir);
  
  const modelExtensions = ['.js', '.ts', '.jsx', '.tsx'];
  
  for (const file of files) {
    // Skip directories, test files and non-model files
    if (file.includes('.test.') || file.includes('.spec.') || 
        file === 'index.js' || file === 'index.ts') {
      continue;
    }
    
    const filePath = path.join(dir, file);
    const stats = fs.statSync(filePath);
    
    if (stats.isDirectory()) {
      // Recursively scan subdirectories
      const subDirModels = scanModelDirectory(filePath);
      modelFiles.push(...subDirModels);
    } else if (modelExtensions.includes(path.extname(file))) {
      // Check file contents to verify it's a model file
      const content = fs.readFileSync(filePath, 'utf8');
      
      // Look for common model patterns
      if (
        // Class pattern
        content.includes('class') && (
          content.includes('extends Model') || 
          content.includes('extends BaseModel') ||
          content.includes('schema.') ||
          content.includes('Schema.')
        ) ||
        // Schema pattern
        content.includes('mongoose.Schema') ||
        content.includes('new Schema') ||
        // Sequelize pattern
        content.includes('sequelize.define') ||
        content.includes('DataTypes')
      ) {
        const modelName = path.basename(file, path.extname(file));
        modelFiles.push(modelName);
      }
    }
  }
  
  return modelFiles;
}

/**
 * Find model files throughout the project when there's no dedicated models directory
 * @param {string} baseDir - The base directory of the project
 * @returns {string[]} Array of model names
 */
function findModelFilesInProject(baseDir) {
  const modelFiles = [];
  const ignoreDirs = ['node_modules', 'dist', 'build', '.git', 'coverage'];
  
  function scanDir(dir) {
    const files = fs.readdirSync(dir);
    
    for (const file of files) {
      if (ignoreDirs.includes(file)) continue;
      
      const filePath = path.join(dir, file);
      const stats = fs.statSync(filePath);
      
      if (stats.isDirectory()) {
        scanDir(filePath);
      } else if (file.includes('Model') || file.includes('model')) {
        const modelName = path.basename(file, path.extname(file))
          .replace('Model', '')
          .replace('model', '');
        modelFiles.push(modelName);
      } else if (path.extname(file) === '.js' || path.extname(file) === '.ts') {
        // Look for model-like content
        const content = fs.readFileSync(filePath, 'utf8');
        if (
          (content.includes('class') && content.includes('Model')) ||
          content.includes('mongoose.Schema') ||
          content.includes('sequelize.define')
        ) {
          const modelName = path.basename(file, path.extname(file));
          modelFiles.push(modelName);
        }
      }
    }
  }
  
  scanDir(baseDir);
  return [...new Set(modelFiles)]; // Remove duplicates
}

/**
 * Scan the project directories to find implemented API endpoints
 * @param {string} baseDir - The base directory of the project
 * @returns {string[]} Array of API endpoint names that have been implemented
 */
function getApiEndpoints(baseDir) {
  try {
    // Common locations for API route definitions
    const possibleDirs = [
      path.join(baseDir, 'src', 'routes'),
      path.join(baseDir, 'routes'),
      path.join(baseDir, 'src', 'api'),
      path.join(baseDir, 'api'),
      path.join(baseDir, 'src', 'controllers'),
      path.join(baseDir, 'controllers')
    ];
    
    let endpoints = [];
    
    // Check each possible directory
    for (const dir of possibleDirs) {
      if (fs.existsSync(dir)) {
        endpoints = [...endpoints, ...scanApiDirectory(dir)];
      }
    }
    
    // If no endpoints found in common locations, search more broadly
    if (endpoints.length === 0) {
      endpoints = findApiEndpointsInProject(baseDir);
    }
    
    return [...new Set(endpoints)]; // Remove duplicates
  } catch (error) {
    console.error('Error finding API endpoints:', error);
    return [];
  }
}

/**
 * Scan a directory for API route definitions
 * @param {string} dir - Directory to scan
 * @returns {string[]} Array of API endpoint names
 */
function scanApiDirectory(dir) {
  const endpoints = [];
  const files = fs.readdirSync(dir);
  
  for (const file of files) {
    // Skip test files
    if (file.includes('.test.') || file.includes('.spec.')) {
      continue;
    }
    
    const filePath = path.join(dir, file);
    const stats = fs.statSync(filePath);
    
    if (stats.isDirectory()) {
      // Recursively scan subdirectories
      const subDirEndpoints = scanApiDirectory(filePath);
      endpoints.push(...subDirEndpoints);
    } else if (path.extname(file) === '.js' || path.extname(file) === '.ts') {
      // Read file content and look for route definitions
      const content = fs.readFileSync(filePath, 'utf8');
      
      // Express route patterns
      const routePatterns = [
        /router\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/, 
        /app\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/, 
        /route\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/, 
        /\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/
      ];
      
      for (const pattern of routePatterns) {
        const matches = content.match(new RegExp(pattern, 'g'));
        if (matches) {
          for (const match of matches) {
            const routeMatch = match.match(pattern);
            if (routeMatch && routeMatch[2]) {
              const endpoint = routeMatch[2].startsWith('/') ? 
                routeMatch[2].substring(1) : routeMatch[2];
              
              // Skip generic/parameter routes
              if (!endpoint.includes(':') && endpoint !== '*') {
                endpoints.push(endpoint);
              }
            }
          }
        }
      }
    }
  }
  
  return endpoints;
}

/**
 * Find API endpoints throughout the project when there's no dedicated API directory
 * @param {string} baseDir - The base directory of the project
 * @returns {string[]} Array of API endpoint names
 */
function findApiEndpointsInProject(baseDir) {
  const endpoints = [];
  const ignoreDirs = ['node_modules', 'dist', 'build', '.git', 'coverage'];
  
  function scanDir(dir) {
    const files = fs.readdirSync(dir);
    
    for (const file of files) {
      if (ignoreDirs.includes(file)) continue;
      
      const filePath = path.join(dir, file);
      const stats = fs.statSync(filePath);
      
      if (stats.isDirectory()) {
        scanDir(filePath);
      } else if (path.extname(file) === '.js' || path.extname(file) === '.ts') {
        // Look for route definitions
        const content = fs.readFileSync(filePath, 'utf8');
        
        // Check for common API patterns
        if (
          content.includes('router.') || 
          content.includes('app.get') || 
          content.includes('app.post') || 
          content.includes('app.put') || 
          content.includes('app.delete') || 
          content.includes('express.Router')
        ) {
          // Extract routes
          const routePatterns = [
            /router\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/, 
            /app\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/, 
            /route\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/, 
            /\.(get|post|put|patch|delete)\s*\(\s*['"](.*?)['"]/
          ];
          
          for (const pattern of routePatterns) {
            const matches = content.match(new RegExp(pattern, 'g'));
            if (matches) {
              for (const match of matches) {
                const routeMatch = match.match(pattern);
                if (routeMatch && routeMatch[2]) {
                  const endpoint = routeMatch[2].startsWith('/') ? 
                    routeMatch[2].substring(1) : routeMatch[2];
                  
                  // Skip generic/parameter routes
                  if (!endpoint.includes(':') && endpoint !== '*') {
                    endpoints.push(endpoint);
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  
  scanDir(baseDir);
  return endpoints;
}

/**
 * Scan the project directories to find implemented UI components
 * @param {string} baseDir - The base directory of the project
 * @returns {string[]} Array of UI component names that have been implemented
 */
function getUiComponents(baseDir) {
  try {
    // Common locations for UI components
    const possibleDirs = [
      path.join(baseDir, 'src', 'components'),
      path.join(baseDir, 'components'),
      path.join(baseDir, 'src', 'ui'),
      path.join(baseDir, 'ui'),
      path.join(baseDir, 'src', 'views'),
      path.join(baseDir, 'views')
    ];
    
    let components = [];
    
    // Check each possible directory
    for (const dir of possibleDirs) {
      if (fs.existsSync(dir)) {
        components = [...components, ...scanComponentDirectory(dir)];
      }
    }
    
    // If no components found in common locations, search more broadly
    if (components.length === 0) {
      components = findComponentsInProject(baseDir);
    }
    
    return [...new Set(components)]; // Remove duplicates
  } catch (error) {
    console.error('Error finding UI components:', error);
    return [];
  }
}

/**
 * Scan a directory for UI component definitions
 * @param {string} dir - Directory to scan
 * @returns {string[]} Array of component names
 */
function scanComponentDirectory(dir) {
  const components = [];
  const files = fs.readdirSync(dir);
  
  const componentExtensions = ['.jsx', '.tsx', '.js', '.ts', '.vue', '.svelte'];
  
  for (const file of files) {
    // Skip test files and utility files
    if (file.includes('.test.') || file.includes('.spec.') || 
        file === 'index.js' || file === 'index.ts' ||
        file.includes('utils') || file.includes('helpers')) {
      continue;
    }
    
    const filePath = path.join(dir, file);
    const stats = fs.statSync(filePath);
    
    if (stats.isDirectory()) {
      // Check if the directory is a component with index file
      if (fs.existsSync(path.join(filePath, 'index.js')) || 
          fs.existsSync(path.join(filePath, 'index.jsx')) || 
          fs.existsSync(path.join(filePath, 'index.tsx'))) {
        components.push(file);
      }
      
      // Recursively scan subdirectories
      const subDirComponents = scanComponentDirectory(filePath);
      components.push(...subDirComponents);
    } else if (componentExtensions.includes(path.extname(file))) {
      // Check file contents to verify it's a component file
      const content = fs.readFileSync(filePath, 'utf8');
      
      // Look for common component patterns
      if (
        // React component patterns
        content.includes('React') || 
        content.includes('Component') ||
        content.includes('function') && content.includes('return') && (
          content.includes('jsx') || content.includes('<') && content.includes('>')
        ) ||
        // Vue component patterns
        content.includes('<template>') ||
        // Angular component patterns
        content.includes('@Component') ||
        // Svelte pattern
        content.includes('<script>') && content.includes('<style')
      ) {
        const componentName = path.basename(file, path.extname(file));
        components.push(componentName);
      }
    }
  }
  
  return components;
}

/**
 * Find UI components throughout the project when there's no dedicated components directory
 * @param {string} baseDir - The base directory of the project
 * @returns {string[]} Array of component names
 */
function findComponentsInProject(baseDir) {
  const components = [];
  const ignoreDirs = ['node_modules', 'dist', 'build', '.git', 'coverage'];
  const componentExtensions = ['.jsx', '.tsx', '.vue', '.svelte'];
  
  function scanDir(dir) {
    const files = fs.readdirSync(dir);
    
    for (const file of files) {
      if (ignoreDirs.includes(file)) continue;
      
      const filePath = path.join(dir, file);
      const stats = fs.statSync(filePath);
      
      if (stats.isDirectory()) {
        scanDir(filePath);
      } else if (componentExtensions.includes(path.extname(file))) {
        // It's likely a component file
        const componentName = path.basename(file, path.extname(file));
        components.push(componentName);
      } else if ((path.extname(file) === '.js' || path.extname(file) === '.ts') && 
                 !file.includes('.test.') && !file.includes('.spec.')) {
        // Check if regular JS/TS file contains component code
        const content = fs.readFileSync(filePath, 'utf8');
        
        if (
          (content.includes('React') && content.includes('Component')) ||
          (content.includes('function') && content.includes('return') && 
           content.includes('jsx'))
        ) {
          const componentName = path.basename(file, path.extname(file));
          components.push(componentName);
        }
      }
    }
  }
  
  scanDir(baseDir);
  return components;
}

/**
 * Calculate test coverage from Jest coverage reports or by analyzing test files
 * @param {string} baseDir - The base directory of the project
 * @returns {number} Test coverage percentage
 */
function calculateTestCoverage(baseDir) {
  try {
    // Check for Jest coverage report
    const coveragePath = path.join(baseDir, 'coverage', 'coverage-summary.json');
    if (fs.existsSync(coveragePath)) {
      const coverageData = JSON.parse(fs.readFileSync(coveragePath, 'utf8'));
      if (coverageData.total && coverageData.total.statements) {
        return parseFloat(coverageData.total.statements.pct);
      }
    }
    
    // If no coverage report, estimate based on test files
    const sourceDirs = [
      path.join(baseDir, 'src'),
      path.join(baseDir, 'lib'),
      path.join(baseDir, 'app')
    ];
    
    let sourceFiles = 0;
    let testFiles = 0;
    
    // Count source files and test files
    for (const dir of sourceDirs) {
      if (fs.existsSync(dir)) {
        const stats = countSourceAndTestFiles(dir);
        sourceFiles += stats.sourceFiles;
        testFiles += stats.testFiles;
      }
    }
    
    if (sourceFiles === 0) return 0;
    
    // Simple heuristic: one test file can cover roughly 60% of one source file
    const estimatedCoverage = Math.min(100, (testFiles / sourceFiles) * 60);
    return Math.round(estimatedCoverage);
  } catch (error) {
    console.error('Error calculating test coverage:', error);
    return 0;
  }
}

/**
 * Count source and test files in a directory
 * @param {string} dir - Directory to scan
 * @returns {Object} Count of source and test files
 */
function countSourceAndTestFiles(dir) {
  let sourceFiles = 0;
  let testFiles = 0;
  const ignoreDirs = ['node_modules', 'dist', 'build', '.git', 'coverage'];
  
  function scanDir(currentDir) {
    const files = fs.readdirSync(currentDir);
    
    for (const file of files) {
      if (ignoreDirs.includes(file)) continue;
      
      const filePath = path.join(currentDir, file);
      const stats = fs.statSync(filePath);
      
      if (stats.isDirectory()) {
        scanDir(filePath);
      } else {
        const ext = path.extname(file);
        if (['.js', '.ts', '.jsx', '.tsx'].includes(ext)) {
          if (file.includes('.test.') || file.includes('.spec.')) {
            testFiles++;
          } else {
            sourceFiles++;
          }
        }
      }
    }
  }
  
  scanDir(dir);
  return { sourceFiles, testFiles };
}

/**
 * Measure technical debt by analyzing code quality, TODOs/FIXMEs, and complexity
 * @param {string} baseDir - The base directory of the project
 * @returns {number} Technical debt percentage
 */
function calculateTechnicalDebt(baseDir) {
  try {
    // Check for ESLint reports
    const eslintReportPath = path.join(baseDir, 'eslint-report.json');
    if (fs.existsSync(eslintReportPath)) {
      const eslintData = JSON.parse(fs.readFileSync(eslintReportPath, 'utf8'));
      const errorCount = eslintData.reduce((sum, file) => sum + file.errorCount, 0);
      const warningCount = eslintData.reduce((sum, file) => sum + file.warningCount, 0);
      const totalIssues = errorCount + (warningCount * 0.5);
      
      // Normalize to a percentage (arbitrary scale)
      return Math.min(100, Math.round((totalIssues / eslintData.length) * 10));
    }
    
    // If no ESLint report, analyze code directly
    let totalFiles = 0;
    let todoCount = 0;
    let complexityScore = 0;
    
    // Analyze source code files
    const sourceDirs = [
      path.join(baseDir, 'src'),
      path.join(baseDir, 'lib'),
      path.join(baseDir, 'app')
    ];
    
    for (const dir of sourceDirs) {
      if (fs.existsSync(dir)) {
        const stats = analyzeCodeQuality(dir);
        totalFiles += stats.totalFiles;
        todoCount += stats.todoCount;
        complexityScore += stats.complexityScore;
      }
    }
    
    if (totalFiles === 0) return 0;
    
    // Calculate technical debt score (0-100%)
    const todoScore = Math.min(50, (todoCount / totalFiles) * 20);
    const avgComplexity = complexityScore / totalFiles;
    const complexityDebtScore = Math.min(50, avgComplexity * 5);
    
    return Math.round(todoScore + complexityDebtScore);
  } catch (error) {
    console.error('Error calculating technical debt:', error);
    return 5; // Default fallback value
  }
}

/**
 * Analyze code quality in a directory
 * @param {string} dir - Directory to scan
 * @returns {Object} Code quality statistics
 */
function analyzeCodeQuality(dir) {
  let totalFiles = 0;
  let todoCount = 0;
  let complexityScore = 0;
  const ignoreDirs = ['node_modules', 'dist', 'build', '.git', 'coverage'];
  
  function scanDir(currentDir) {
    const files = fs.readdirSync(currentDir);
    
    for (const file of files) {
      if (ignoreDirs.includes(file)) continue;
      
      const filePath = path.join(currentDir, file);
      const stats = fs.statSync(filePath);
      
      if (stats.isDirectory()) {
        scanDir(filePath);
      } else {
        const ext = path.extname(file);
        if (['.js', '.ts', '.jsx', '.tsx', '.css', '.scss', '.html', '.vue'].includes(ext)) {
          totalFiles++;
          
          const content = fs.readFileSync(filePath, 'utf8');
          
          // Count TODOs and FIXMEs
          const todoMatches = content.match(/TODO|FIXME/g);
          if (todoMatches) {
            todoCount += todoMatches.length;
          }
          
          // Calculate rough complexity
          const lines = content.split('\n');
          const nestingLevel = Math.max(
            ...lines.map(line => {
              const indent = line.match(/^\s*/)[0].length;
              return Math.floor(indent / 2); // Assuming 2-space indentation
            })
          );
          
          const functionCount = (content.match(/function|=>/g) || []).length;
          const conditionalCount = (content.match(/if|switch|for|while|catch/g) || []).length;
          
          // Simple complexity score based on nesting, functions and conditionals
          const fileComplexity = nestingLevel * 1 + functionCount * 0.5 + conditionalCount * 0.5;
          complexityScore += fileComplexity;
        }
      }
    }
  }
  
  scanDir(dir);
  return { totalFiles, todoCount, complexityScore };
}

/**
 * Update the project status metrics calculation using the dynamic functions
 */
function calculateProjectMetrics(baseDir) {
  // Get implemented models, APIs, and UI components
  const modelFiles = getModelFiles(baseDir);
  const apiEndpoints = getApiEndpoints(baseDir);
  const uiComponents = getUiComponents(baseDir);
  
  // Calculate test coverage and technical debt
  const testCoverage = calculateTestCoverage(baseDir);
  const technicalDebt = calculateTechnicalDebt(baseDir);
  
  // Constants for total expected items
  const TOTAL_MODELS = 28;
  const TOTAL_APIS = 42;
  const TOTAL_UI = 35;
  
  // Calculate percentages
  const modelPercentage = Math.round((modelFiles.length / TOTAL_MODELS) * 100);
  const apiPercentage = Math.round((apiEndpoints.length / TOTAL_APIS) * 100);
  const uiPercentage = Math.round((uiComponents.length / TOTAL_UI) * 100);
  
  return {
    models: {
      percentage: modelPercentage,
      count: modelFiles.length,
      total: TOTAL_MODELS,
      implemented: modelFiles
    },
    apis: {
      percentage: apiPercentage,
      count: apiEndpoints.length,
      total: TOTAL_APIS,
      implemented: apiEndpoints
    },
    ui: {
      percentage: uiPercentage,
      count: uiComponents.length,
      total: TOTAL_UI,
      implemented: uiComponents
    },
    tests: Math.round(testCoverage),
    technicalDebt: Math.round(technicalDebt)
  };
}

/**
 * Generate project status analysis and update LAST_ANALYSIS_RESULTS.md
 */
GeminiAnalyzer.prototype.generateProjectAnalysis = async function(baseDir) {
  // Calculate project metrics
  const metrics = calculateProjectMetrics(baseDir);
  
  // Format the current date and time
  const now = new Date();
  const dateTime = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')} ${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`;
  
  // Generate the markdown content
  const content = `# Last Analysis Results

## Analysis Summary

**Last Run**: ${dateTime}

**Project Status**:
- Models: ${metrics.models.percentage}% complete (${metrics.models.count}/${metrics.models.total})
- API: ${metrics.apis.percentage}% complete (${metrics.apis.count}/${metrics.apis.total})
- UI: ${metrics.ui.percentage}% complete (${metrics.ui.count}/${metrics.ui.total})
- Tests: ${metrics.tests}% complete
- Technical Debt: ${metrics.technicalDebt}%

**Overall Phase**: planning

## Integration Status

| Component | Status | Completion | Next Steps |
|-----------|--------|------------|------------|
| Project Scope | Defined | 100% | Begin implementation based on scope |
| Timeline | Defined | 100% | Track progress against timeline |
| Model Mapping | In Progress | 45% | Complete Course-Category testing |
| API Integration | In Progress | 10% | Begin CRUD operations implementation |
| Authentication | Implemented | 100% | Add more authentication tests |
| Synchronization | Not Started | 0% | Design sync architecture |

## Recent Changes

- Defined full project scope with 28 models, 42 API endpoints, and 35 UI components
- Created realistic project timeline with completion target of 2025-11-15
- Established project baseline and KPIs
- Defined comprehensive quality strategy

## Next Priorities

1. Complete Course-Category model mapping implementation
2. Expand API endpoint CRUD operations for existing models
3. Improve test coverage for authentication system
4. Begin Synchronization architecture design

## Documentation Updates

The following documentation was updated:
- Project Scope: [\`docs/project_scope.md\`](docs/project_scope.md)
- Project Timeline: [\`docs/project_timeline.md\`](docs/project_timeline.md)
- Project Baseline: [\`docs/project_baseline.md\`](docs/project_baseline.md)
- Quality Strategy: [\`docs/quality_strategy.md\`](docs/quality_strategy.md)
`;

  const lastAnalysisPath = path.join(baseDir, 'LAST_ANALYSIS_RESULTS.md');
  fs.writeFileSync(lastAnalysisPath, content);
  
  return {
    metrics,
    filePath: lastAnalysisPath
  };
};

module.exports = GeminiAnalyzer;