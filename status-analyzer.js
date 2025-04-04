const fs = require('fs');
const path = require('path');

/**
 * Advanced Rust Codebase Status Analyzer
 * 
 * This script thoroughly examines Rust source files to determine implementation
 * status of models, API endpoints, and UI components.
 */
class RustCodebaseAnalyzer {
  constructor(baseDir) {
    this.baseDir = baseDir;
    this.metrics = {
      models: { total: 0, implemented: 0, details: [] },
      apiEndpoints: { total: 0, implemented: 0, details: [] },
      uiComponents: { total: 0, implemented: 0, details: [] },
      tests: { total: 0, coverage: 0, details: [] }
    };
  }

  /**
   * Run the complete analysis
   */
  analyze() {
    console.log("Starting comprehensive Rust codebase analysis...");
    
    // Find all Rust files
    const modelFiles = this.findFiles('src/models', '.rs');
    const apiFiles = this.findFiles('src/api', '.rs');
    const uiFiles = this.findFiles('src/ui', '.rs');
    const testFiles = this.findFiles('tests', '.rs');
    
    console.log(`Found ${modelFiles.length} model files, ${apiFiles.length} API files, ${uiFiles.length} UI files, ${testFiles.length} test files`);
    
    // Analyze models
    this.analyzeModels(modelFiles);
    
    // Analyze API endpoints
    this.analyzeApiEndpoints(apiFiles);
    
    // Analyze UI components
    this.analyzeUiComponents(uiFiles);
    
    // Analyze test coverage
    this.analyzeTestCoverage(testFiles, [...modelFiles, ...apiFiles, ...uiFiles]);
    
    // Calculate summary metrics
    this.calculateSummaryMetrics();
    
    // Update project status document
    this.updateProjectStatus();
    
    console.log("Analysis complete!");
    return this.metrics;
  }

  /**
   * Find all files in a directory with a specific extension
   */
  findFiles(dir, extension, results = []) {
    const fullPath = path.join(this.baseDir, dir);
    
    if (!fs.existsSync(fullPath)) {
      return results;
    }
    
    const items = fs.readdirSync(fullPath);
    
    for (const item of items) {
      const itemPath = path.join(fullPath, item);
      const stat = fs.statSync(itemPath);
      
      if (stat.isDirectory()) {
        this.findFiles(path.join(dir, item), extension, results);
      } else if (path.extname(item) === extension) {
        results.push({
          path: path.join(dir, item),
          fullPath: itemPath
        });
      }
    }
    
    return results;
  }

  /**
   * Analyze model files to identify structs and their implementation status
   */
  analyzeModels(modelFiles) {
    console.log("Analyzing models...");
    
    for (const file of modelFiles) {
      try {
        const content = fs.readFileSync(file.fullPath, 'utf8');
        const relativePath = file.path;
        
        // Extract struct definitions
        const structMatches = this.extractStructs(content);
        
        for (const struct of structMatches) {
          this.metrics.models.total++;
          
          // Check implementation completeness based on fields and impl blocks
          const fields = struct.fields.split(',').filter(f => f.trim()).length;
          
          // Find impl blocks for this struct
          const implBlocks = this.findImplBlocks(content, struct.name);
          const methodCount = implBlocks.reduce((count, impl) => count + impl.methods.length, 0);
          
          // Estimate completeness
          let completeness = 0;
          if (fields > 0) {
            // Base completeness on field count and method count
            completeness = Math.min(100, Math.round((fields + methodCount * 2) / (fields > 5 ? 10 : 5) * 100));
          }
          
          this.metrics.models.details.push({
            name: struct.name,
            file: relativePath,
            fieldCount: fields,
            methodCount,
            completeness,
            isEnum: false
          });
          
          if (completeness >= 50) {
            this.metrics.models.implemented++;
          }
        }
        
        // Extract enum definitions
        const enumMatches = this.extractEnums(content);
        
        for (const enumDef of enumMatches) {
          this.metrics.models.total++;
          
          // Count variants
          const variants = enumDef.variants.split(',').filter(v => v.trim()).length;
          
          // Find impl blocks for this enum
          const implBlocks = this.findImplBlocks(content, enumDef.name);
          const methodCount = implBlocks.reduce((count, impl) => count + impl.methods.length, 0);
          
          // Estimate completeness
          const completeness = Math.min(100, Math.round((variants + methodCount * 2) / 5 * 100));
          
          this.metrics.models.details.push({
            name: enumDef.name,
            file: relativePath,
            variantCount: variants,
            methodCount,
            completeness,
            isEnum: true
          });
          
          if (completeness >= 50) {
            this.metrics.models.implemented++;
          }
        }
      } catch (err) {
        console.error(`Error analyzing model file ${file.path}:`, err.message);
      }
    }
    
    console.log(`Found ${this.metrics.models.total} models, ${this.metrics.models.implemented} implemented.`);
  }

  /**
   * Extract struct definitions from Rust code
   */
  extractStructs(content) {
    const structs = [];
    const structRegex = /struct\s+(\w+)(?:<[^>]*>)?\s*{([^}]*)}/g;
    
    let match;
    while ((match = structRegex.exec(content)) !== null) {
      structs.push({
        name: match[1],
        fields: match[2]
      });
    }
    
    return structs;
  }

  /**
   * Extract enum definitions from Rust code
   */
  extractEnums(content) {
    const enums = [];
    const enumRegex = /enum\s+(\w+)(?:<[^>]*>)?\s*{([^}]*)}/g;
    
    let match;
    while ((match = enumRegex.exec(content)) !== null) {
      enums.push({
        name: match[1],
        variants: match[2]
      });
    }
    
    return enums;
  }

  /**
   * Find implementation blocks for a struct or enum
   */
  findImplBlocks(content, name) {
    const implBlocks = [];
    const implRegex = new RegExp(`impl(?:\\s*<[^>]*>)?\\s+${name}\\s*(?:<[^>]*>)?\\s*{([^}]*)}`, 'g');
    
    let match;
    while ((match = implRegex.exec(content)) !== null) {
      const blockContent = match[1];
      const methods = [];
      
      // Extract methods
      const methodRegex = /fn\s+(\w+)/g;
      let methodMatch;
      while ((methodMatch = methodRegex.exec(blockContent)) !== null) {
        methods.push(methodMatch[1]);
      }
      
      implBlocks.push({
        content: blockContent,
        methods
      });
    }
    
    return implBlocks;
  }

  /**
   * Analyze API endpoint handlers
   */
  analyzeApiEndpoints(apiFiles) {
    console.log("Analyzing API endpoints...");
    
    for (const file of apiFiles) {
      try {
        const content = fs.readFileSync(file.fullPath, 'utf8');
        const relativePath = file.path;
        
        // Look for route definitions using Axum route macros or Router::new() chains
        const routeHandlers = this.findRouteHandlers(content);
        
        for (const handler of routeHandlers) {
          this.metrics.apiEndpoints.total++;
          
          // Find function body
          const funcBody = this.findFunctionBody(content, handler.name);
          let completeness = 0;
          
          if (funcBody) {
            completeness = this.estimateImplementationCompleteness(funcBody);
          }
          
          this.metrics.apiEndpoints.details.push({
            name: handler.name,
            file: relativePath,
            route: handler.route || "unknown",
            method: handler.method || "unknown",
            completeness
          });
          
          if (completeness >= 50) {
            this.metrics.apiEndpoints.implemented++;
          }
        }
      } catch (err) {
        console.error(`Error analyzing API file ${file.path}:`, err.message);
      }
    }
    
    console.log(`Found ${this.metrics.apiEndpoints.total} API endpoints, ${this.metrics.apiEndpoints.implemented} implemented.`);
  }

  /**
   * Find route handlers in API code
   */
  findRouteHandlers(content) {
    const handlers = [];
    
    // Match #[route] macro style handlers
    const macroRegex = /#\[(?:get|post|put|delete|patch|head|options|trace|connect)\([^)]*\)\]\s*(?:pub\s+)?async\s+fn\s+(\w+)/g;
    
    let match;
    while ((match = macroRegex.exec(content)) !== null) {
      handlers.push({
        name: match[1],
        type: 'macro'
      });
    }
    
    // Match router.method() style handlers
    const routerRegex = /\.(?:get|post|put|delete|patch|head|options|trace|connect)\((?:"[^"]*"|'[^']*'),\s*(\w+)(?:::[\w:]+)?\)/g;
    
    while ((match = routerRegex.exec(content)) !== null) {
      handlers.push({
        name: match[1],
        type: 'method'
      });
    }
    
    return handlers;
  }

  /**
   * Find function body for a given function name
   */
  findFunctionBody(content, funcName) {
    const funcRegex = new RegExp(`(?:async\\s+)?fn\\s+${funcName}[^{]*{([^}]*(?:{[^}]*}[^}]*)*)}`);
    const match = content.match(funcRegex);
    
    return match ? match[1] : null;
  }

  /**
   * Analyze UI components in Leptos files
   */
  analyzeUiComponents(uiFiles) {
    console.log("Analyzing UI components...");
    
    for (const file of uiFiles) {
      try {
        const content = fs.readFileSync(file.fullPath, 'utf8');
        const relativePath = file.path;
        
        // Match component functions in Leptos style
        const components = this.findLeptosComponents(content);
        
        for (const component of components) {
          this.metrics.uiComponents.total++;
          
          const funcBody = this.findFunctionBody(content, component.name);
          let completeness = 0;
          
          if (funcBody) {
            completeness = this.estimateUIComponentCompleteness(funcBody);
          }
          
          this.metrics.uiComponents.details.push({
            name: component.name,
            file: relativePath,
            completeness
          });
          
          if (completeness >= 50) {
            this.metrics.uiComponents.implemented++;
          }
        }
      } catch (err) {
        console.error(`Error analyzing UI file ${file.path}:`, err.message);
      }
    }
    
    console.log(`Found ${this.metrics.uiComponents.total} UI components, ${this.metrics.uiComponents.implemented} implemented.`);
  }

  /**
   * Find Leptos component functions
   */
  findLeptosComponents(content) {
    const components = [];
    const componentRegex = /#\[(?:component|server|island)(?:\([^)]*\))?\]\s*(?:pub\s+)?fn\s+(\w+)(?:<[^>]*>)?/g;
    
    let match;
    while ((match = componentRegex.exec(content)) !== null) {
      components.push({
        name: match[1]
      });
    }
    
    return components;
  }

  /**
   * Analyze test coverage by examining test files
   */
  analyzeTestCoverage(testFiles, sourceFiles) {
    console.log("Analyzing test coverage...");
    
    // Count total test functions
    let testCount = 0;
    
    for (const file of testFiles) {
      try {
        const content = fs.readFileSync(file.fullPath, 'utf8');
        
        // Match test functions
        const testFunctions = this.findTestFunctions(content);
        testCount += testFunctions.length;
        
        for (const testFunc of testFunctions) {
          this.metrics.tests.details.push({
            name: testFunc.name,
            file: file.path
          });
        }
      } catch (err) {
        console.error(`Error analyzing test file ${file.path}:`, err.message);
      }
    }
    
    // Count total functions in source files for coverage estimate
    let totalFunctions = 0;
    
    for (const file of sourceFiles) {
      try {
        const content = fs.readFileSync(file.fullPath, 'utf8');
        const functions = this.countFunctions(content);
        totalFunctions += functions;
      } catch (err) {
        console.error(`Error counting functions in ${file.path}:`, err.message);
      }
    }
    
    this.metrics.tests.total = testCount;
    this.metrics.tests.coverage = totalFunctions > 0 ? Math.min(100, Math.round((testCount / totalFunctions) * 100)) : 0;
    
    console.log(`Found ${testCount} tests, estimated coverage: ${this.metrics.tests.coverage}%`);
  }

  /**
   * Find test functions in test files
   */
  findTestFunctions(content) {
    const tests = [];
    const testRegex = /#\[test\]\s*(?:pub\s+)?fn\s+(\w+)/g;
    
    let match;
    while ((match = testRegex.exec(content)) !== null) {
      tests.push({
        name: match[1]
      });
    }
    
    return tests;
  }

  /**
   * Count functions in a file
   */
  countFunctions(content) {
    const funcRegex = /fn\s+\w+/g;
    const matches = content.match(funcRegex);
    return matches ? matches.length : 0;
  }

  /**
   * Estimate implementation completeness of a function body
   */
  estimateImplementationCompleteness(body) {
    if (!body || body.trim() === "") return 0;
    
    // If it only has todo! or unimplemented! macros, it's a stub
    if (body.trim().match(/^\s*(?:todo|unimplemented)!\(\);?\s*$/)) {
      return 10; // Just a stub
    }
    
    // Count statements as an estimate of complexity
    const statements = body.split(';').length;
    
    // Check for common patterns that indicate completeness
    const hasErrorHandling = body.includes("Result<") || body.includes("match ") || body.includes("if let ");
    const hasDatabaseAccess = body.includes(".query") || body.includes(".execute") || body.includes("repository");
    const hasReturnValue = body.includes("return ");
    const hasLogic = statements > 3;
    
    // Calculate completeness score
    let score = 20; // Base score for non-empty function
    
    if (hasLogic) score += 20;
    if (hasErrorHandling) score += 20;
    if (hasDatabaseAccess) score += 20;
    if (hasReturnValue) score += 20;
    
    return Math.min(100, score);
  }
  
  /**
   * Estimate UI component completeness
   */
  estimateUIComponentCompleteness(body) {
    if (!body || body.trim() === "") return 0;
    
    // If it only has todo! or unimplemented! macros, it's a stub
    if (body.trim().match(/^\s*(?:todo|unimplemented)!\(\);?\s*$/)) {
      return 10; // Just a stub
    }
    
    // Leptos-specific patterns
    const hasSignals = body.includes("create_signal") || body.includes("create_rw_signal");
    const hasEffects = body.includes("create_effect");
    const hasResources = body.includes("create_resource");
    const hasComponents = (body.match(/<\w+/g) || []).length;
    const hasEvents = body.includes("on:click") || body.includes("on:input") || body.includes("on_click");
    const hasConditionals = body.includes("Show") || body.includes("if ");
    const hasLoops = body.includes("For") || body.includes("for ");
    const hasClasses = body.includes("class=");
    
    // Calculate completeness score
    let score = 20; // Base score for non-empty component
    
    if (hasSignals) score += 10;
    if (hasEffects) score += 10;
    if (hasResources) score += 10;
    if (hasEvents) score += 10;
    if (hasConditionals) score += 10;
    if (hasLoops) score += 10;
    if (hasClasses) score += 10;
    score += Math.min(20, hasComponents * 5); // Up to 20 points for components
    
    return Math.min(100, score);
  }

  /**
   * Calculate summary metrics
   */
  calculateSummaryMetrics() {
    // Calculate percentages safely
    const modelPercentage = this.metrics.models.total > 0 
      ? Math.round((this.metrics.models.implemented / this.metrics.models.total) * 100) 
      : 0;
      
    const apiPercentage = this.metrics.apiEndpoints.total > 0 
      ? Math.round((this.metrics.apiEndpoints.implemented / this.metrics.apiEndpoints.total) * 100) 
      : 0;
      
    const uiPercentage = this.metrics.uiComponents.total > 0 
      ? Math.round((this.metrics.uiComponents.implemented / this.metrics.uiComponents.total) * 100) 
      : 0;
    
    // Store calculated percentages
    this.metrics.summary = {
      modelImplementation: `${modelPercentage}%`,
      apiImplementation: `${apiPercentage}%`,
      uiImplementation: `${uiPercentage}%`,
      testCoverage: `${this.metrics.tests.coverage}%`
    };
    
    console.log("Summary metrics calculated:");
    console.log(`- Models: ${this.metrics.summary.modelImplementation}`);
    console.log(`- API: ${this.metrics.summary.apiImplementation}`);
    console.log(`- UI: ${this.metrics.summary.uiImplementation}`);
    console.log(`- Test coverage: ${this.metrics.summary.testCoverage}`);
  }

  /**
   * Update the project status document with new metrics
   */
  updateProjectStatus() {
    console.log("Updating project status document...");
    const statusFile = path.join(this.baseDir, 'project_status.md');
    
    if (!fs.existsSync(statusFile)) {
      console.error("Project status file not found!");
      return;
    }
    
    let statusContent = fs.readFileSync(statusFile, 'utf8');
    
    // Update the implementation percentages
    statusContent = statusContent.replace(
      /modelImplementation: "[^"]+"/,
      `modelImplementation: "${this.metrics.summary.modelImplementation}"`
    );
    
    statusContent = statusContent.replace(
      /apiImplementation: "[^"]+"/,
      `apiImplementation: "${this.metrics.summary.apiImplementation}"`
    );
    
    statusContent = statusContent.replace(
      /uiImplementation: "[^"]+"/,
      `uiImplementation: "${this.metrics.summary.uiImplementation}"`
    );
    
    statusContent = statusContent.replace(
      /testCoverage: "[^"]+"/,
      `testCoverage: "${this.metrics.summary.testCoverage}"`
    );
    
    // Update the last updated date
    const today = new Date().toISOString().split('T')[0];
    statusContent = statusContent.replace(
      /_Last updated: \*\*[^*]+\*\*/,
      `_Last updated: **${today}**`
    );
    
    // Write back the updated content
    fs.writeFileSync(statusFile, statusContent);
    
    console.log("Project status document updated successfully!");
    
    // Additionally, generate a detailed report
    this.generateDetailedReport();
  }
  
  /**
   * Generate a detailed report of implementation status
   */
  generateDetailedReport() {
    console.log("Generating detailed implementation report...");
    
    const reportFile = path.join(this.baseDir, 'implementation_details.md');
    let report = `# Detailed Implementation Report\n_Generated on ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Add models section
    report += `## Models (${this.metrics.summary.modelImplementation} Complete)\n\n`;
    report += "| Model | File | Type | Completeness | Fields/Methods |\n";
    report += "|-------|------|------|-------------|----------------|\n";
    
    this.metrics.models.details.forEach(model => {
      const type = model.isEnum ? "Enum" : "Struct";
      const fieldsOrVariants = model.isEnum 
        ? `${model.variantCount} variants` 
        : `${model.fieldCount} fields`;
        
      report += `| ${model.name} | ${model.file} | ${type} | ${model.completeness}% | ${fieldsOrVariants}, ${model.methodCount} methods |\n`;
    });
    
    report += "\n";
    
    // Add API endpoints section
    report += `## API Endpoints (${this.metrics.summary.apiImplementation} Complete)\n\n`;
    report += "| Handler | File | Completeness |\n";
    report += "|---------|------|-------------|\n";
    
    this.metrics.apiEndpoints.details.forEach(endpoint => {
      report += `| ${endpoint.name} | ${endpoint.file} | ${endpoint.completeness}% |\n`;
    });
    
    report += "\n";
    
    // Add UI components section
    report += `## UI Components (${this.metrics.summary.uiImplementation} Complete)\n\n`;
    report += "| Component | File | Completeness |\n";
    report += "|-----------|------|-------------|\n";
    
    this.metrics.uiComponents.details.forEach(component => {
      report += `| ${component.name} | ${component.file} | ${component.completeness}% |\n`;
    });
    
    report += "\n";
    
    // Add tests section
    report += `## Tests (${this.metrics.summary.testCoverage} Coverage)\n\n`;
    report += "| Test | File |\n";
    report += "|------|------|\n";
    
    this.metrics.tests.details.forEach(test => {
      report += `| ${test.name} | ${test.file} |\n`;
    });
    
    // Write the report
    fs.writeFileSync(reportFile, report);
    console.log(`Detailed report generated at ${reportFile}`);
  }
}

// Execute the analysis
function main() {
  const baseDir = path.resolve(__dirname);
  const analyzer = new RustCodebaseAnalyzer(baseDir);
  analyzer.analyze();
}

main();