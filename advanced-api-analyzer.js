const fs = require('fs');
const path = require('path');

/**
 * Advanced API Analyzer
 * - Fixes regex patterns for route method handlers
 * - Enhances completeness estimation
 * - Adds feature area classification
 */
class AdvancedApiAnalyzer {
  constructor(baseDir) {
    this.baseDir = baseDir;
    this.metrics = {
      apiEndpoints: { total: 0, implemented: 0, details: [] },
      featureAreas: {
        auth: { total: 0, implemented: 0 },
        forum: { total: 0, implemented: 0 },
        lms: { total: 0, implemented: 0 },
        integration: { total: 0, implemented: 0 },
        other: { total: 0, implemented: 0 }
      }
    };
    
    // Function definitions cache to avoid duplicate analysis
    this.functionDefs = new Map();
    
    // Handler signatures - patterns to identify different API handler types
    this.handlerPatterns = [
      { type: 'auth', pattern: /(?:login|register|auth|token|user|me|password)/i },
      { type: 'forum', pattern: /(?:forum|category|topic|post|tag|reply|discussion)/i },
      { type: 'lms', pattern: /(?:course|module|assignment|submission|grade|enrollment)/i },
      { type: 'integration', pattern: /(?:integration|canvas|sync|lms.*forum|forum.*lms)/i }
    ];
  }
  
  async analyze() {
    console.log(`Starting advanced API analysis of ${this.baseDir}...`);
    
    // Check for API files in src-tauri/src/api
    const tauriApiPath = path.join(this.baseDir, 'src-tauri', 'src', 'api');
    if (fs.existsSync(tauriApiPath)) {
      console.log(`Analyzing API files in ${tauriApiPath}`);
      await this.analyzeApiFiles(tauriApiPath);
    }
    
    // Check for API files in src/api
    const srcApiPath = path.join(this.baseDir, 'src', 'api');
    if (fs.existsSync(srcApiPath)) {
      console.log(`Analyzing API files in ${srcApiPath}`);
      await this.analyzeApiFiles(srcApiPath);
    }
    
    // Categorize endpoints by feature area
    this.categorizeEndpoints();
    
    // Update project status document
    this.updateProjectStatus();
    
    console.log('Analysis complete!');
    console.log(`API Endpoints: ${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total} implemented (${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}%)`);
    console.log('\nAPI Endpoints by Feature Area:');
    for (const [area, stats] of Object.entries(this.metrics.featureAreas)) {
      if (stats.total > 0) {
        console.log(`  ${area}: ${stats.implemented}/${stats.total} (${this.getPercentage(stats.implemented, stats.total)}%)`);
      }
    }
    
    return this.metrics;
  }
  
  async analyzeApiFiles(apiPath) {
    if (!fs.existsSync(apiPath)) return;
    
    try {
      const items = fs.readdirSync(apiPath);
      
      for (const item of items) {
        const fullPath = path.join(apiPath, item);
        const stat = fs.statSync(fullPath);
        
        if (stat.isDirectory()) {
          // Recursively analyze subdirectories
          await this.analyzeApiFiles(fullPath);
        } else if (item.endsWith('.rs')) {
          // Analyze Rust file for API endpoints
          await this.analyzeApiFile(fullPath);
        }
      }
    } catch (err) {
      console.error(`Error reading directory ${apiPath}:`, err.message);
    }
  }
  
  async analyzeApiFile(filePath) {
    try {
      const relativePath = path.relative(this.baseDir, filePath);
      console.log(`Analyzing API file: ${relativePath}`);
      
      const content = fs.readFileSync(filePath, 'utf8');
      
      // Find all function definitions first and cache them
      await this.extractAllFunctionDefinitions(content, relativePath);
      
      // Find router setup
      const hasRouter = content.includes('Router::new') || content.includes('Router::with_state');
      if (hasRouter) {
        console.log(`  Found router definition in ${relativePath}`);
      }
      
      // Look for route handler functions with macros
      await this.findRouteHandlersWithMacros(content, relativePath);
      
      // Look for handler functions
      await this.findHandlerFunctions(content, relativePath);
      
      // Look for route method calls (fixed regex)
      await this.findRouteMethods(content, relativePath);
      
    } catch (err) {
      console.error(`Error analyzing file ${filePath}:`, err.message);
    }
  }
  
  async extractAllFunctionDefinitions(content, filePath) {
    // A simpler regex approach to find function definitions more reliably
    const funcDefs = [];
    const lines = content.split('\n');
    
    for (let i = 0; i < lines.length; i++) {
      // Look for function declaration
      if (lines[i].match(/(?:pub\s+)?(?:async\s+)?fn\s+(\w+).*\{/)) {
        const match = lines[i].match(/(?:pub\s+)?(?:async\s+)?fn\s+(\w+)/);
        if (match) {
          const funcName = match[1];
          
          // Collect the function body
          let braceCount = (lines[i].match(/{/g) || []).length - (lines[i].match(/}/g) || []).length;
          let body = lines[i];
          let j = i + 1;
          
          while (braceCount > 0 && j < lines.length) {
            body += '\n' + lines[j];
            braceCount += (lines[j].match(/{/g) || []).length;
            braceCount -= (lines[j].match(/}/g) || []).length;
            j++;
          }
          
          // Extract just the body part (between the outermost braces)
          const bodyMatch = body.match(/{([\s\S]*)}/);
          if (bodyMatch) {
            console.log(`  Found function definition: ${funcName}`);
            this.functionDefs.set(funcName, {
              name: funcName,
              body: bodyMatch[1],
              file: filePath
            });
          }
        }
      }
    }
  }
  
  async findRouteHandlersWithMacros(content, filePath) {
    // Match handler functions with route macros
    const routeHandlerRegex = /#\[(?:get|post|put|delete|patch)\(.*?\)\]\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)/g;
    let match;
    
    while ((match = routeHandlerRegex.exec(content)) !== null) {
      const handlerName = match[1];
      console.log(`  Found route handler with macro: ${handlerName}`);
      
      // Get function definition from cache
      const funcDef = this.functionDefs.get(handlerName);
      
      if (funcDef) {
        // Estimate implementation completeness
        const completeness = this.estimateHandlerCompleteness(funcDef.body, handlerName);
        
        this.addApiEndpoint(handlerName, filePath, completeness, 'macro');
      } else {
        // Fall back to a basic completeness estimate if we can't find the function definition
        this.addApiEndpoint(handlerName, filePath, 30, 'macro');
      }
    }
  }
  
  async findHandlerFunctions(content, filePath) {
    // Match regular functions that look like handlers
    const funcRegex = /(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\([^)]*(?:Request|Response|Json|Path|Query|State)/g;
    let match;
    
    while ((match = funcRegex.exec(content)) !== null) {
      const handlerName = match[1];
      
      // Skip if already counted
      if (this.metrics.apiEndpoints.details.some(h => h.name === handlerName)) {
        continue;
      }
      
      console.log(`  Found handler function: ${handlerName}`);
      
      // Get function definition from cache
      const funcDef = this.functionDefs.get(handlerName);
      
      if (funcDef) {
        // Estimate implementation completeness
        const completeness = this.estimateHandlerCompleteness(funcDef.body, handlerName);
        
        this.addApiEndpoint(handlerName, filePath, completeness, 'function');
      } else {
        // Fall back to a basic completeness estimate
        this.addApiEndpoint(handlerName, filePath, 30, 'function');
      }
    }
  }
  
  async findRouteMethods(content, filePath) {
    // Fixed regex to properly handle route methods
    // This pattern looks for .route(), .get(), etc. calls without trying to include the handler name
    const routeMethodLines = content.split('\n').filter(line => 
      line.includes('.route(') || 
      line.includes('.nest(') || 
      line.match(/\.(get|post|put|delete|patch)\(/)
    );
    
    for (const line of routeMethodLines) {
      try {
        // Extract method and path
        const methodMatch = line.match(/\.(route|get|post|put|delete|patch|nest)\s*\([^,]*/);
        if (!methodMatch) continue;
        
        const method = methodMatch[1];
        
        // Extract handler reference separately with a new regex
        const handlerRegex = /,\s*([\w:]+|\|[^|]*\||"[^"]*"|'[^']*')/;
        const handlerMatch = line.match(handlerRegex);
        
        if (!handlerMatch) continue;
        
        let handlerRef = handlerMatch[1].trim();
        
        // Clean up handler reference (remove quotes, etc.)
        handlerRef = handlerRef.replace(/[,"']/g, '').trim();
        
        // Skip router references
        if (handlerRef.includes('Router::')) {
          continue;
        }
        
        // Skip if it's clearly a parameter and not a handler
        if (['user', 'course_id', 'module_id', 'assignment_id', 'page', '-'].includes(handlerRef)) {
          continue;
        }
        
        console.log(`  Found route method (${method}) using handler: ${handlerRef}`);
        
        // Handle different handler reference types
        if (handlerRef.includes('::')) {
          // It's a module::function reference
          const parts = handlerRef.split('::');
          const functionName = parts[parts.length - 1];
          
          // Try to find the function definition
          for (const [name, def] of this.functionDefs.entries()) {
            if (name === functionName) {
              const completeness = this.estimateHandlerCompleteness(def.body, functionName);
              this.addApiEndpoint(`${method}:${handlerRef}`, filePath, completeness, 'module_reference');
              break;
            }
          }
        } else if (handlerRef.startsWith('|')) {
          // It's a closure
          const completeness = this.estimateClosureCompleteness(handlerRef);
          this.addApiEndpoint(`${method}:inline_closure`, filePath, completeness, 'closure');
        } else {
          // It's a direct function reference
          // See if we have its definition
          let funcDef = this.functionDefs.get(handlerRef);
          
          if (funcDef) {
            const completeness = this.estimateHandlerCompleteness(funcDef.body, handlerRef);
            this.addApiEndpoint(`${method}:${handlerRef}`, filePath, completeness, 'reference');
          } else {
            // Fall back to simple handler name
            this.addApiEndpoint(`${method}:${handlerRef}`, filePath, 30, 'unknown_reference');
          }
        }
      } catch (err) {
        console.error(`Error analyzing route method line: ${line.trim()}`, err.message);
      }
    }
  }
  
  estimateClosureCompleteness(closure) {
    // Estimate completeness of a closure
    if (!closure) return 20;
    
    // Count statements as an estimate of complexity
    const statementCount = (closure.match(/;/g) || []).length;
    const braceCount = (closure.match(/{/g) || []).length;
    
    // Check for common patterns
    const hasLogic = statementCount > 1 || braceCount > 1;
    const hasReturnValue = closure.includes("Ok(") || closure.includes("Err(") || closure.includes("Json(");
    const hasErrorHandling = closure.includes("?") || closure.includes("Result") || closure.includes("match");
    
    let score = 20; // Base score
    
    if (hasLogic) score += 20;
    if (hasReturnValue) score += 20;
    if (hasErrorHandling) score += 20;
    score += Math.min(20, statementCount * 5); // Up to 20 points for complexity
    
    return Math.min(100, score);
  }
  
  estimateHandlerCompleteness(body, handlerName) {
    try {
      if (!body || body.trim() === "") return 0;
      
      // If it only has todo! or unimplemented! macros, it's a stub
      if (body.trim().match(/^\s*(?:todo|unimplemented)!\(\);?\s*$/)) {
        return 10; // Just a stub
      }
      
      // Calculate complexity metrics
      const statements = body.split(';').length;
      const branches = (body.match(/if\s+|else\s+|match\s+/g) || []).length;
      const questionOps = (body.match(/\?/g) || []).length; // Error propagation
      const fnCalls = (body.match(/\w+\(/g) || []).length;
      
      // Extract pattern matches that indicate functionality
      // Database operations
      const hasDbOps = body.includes(".query") || 
                       body.includes(".execute") || 
                       body.includes("repository") ||
                       body.includes("db.") || 
                       body.includes("pool.");
      
      // Error handling
      const hasErrorHandling = body.includes("Result<") || 
                              body.includes("match ") || 
                              body.includes("if let ") || 
                              body.includes("try_") || 
                              questionOps > 0;
      
      // Return values
      const hasReturnValue = body.includes("return ") || 
                            body.includes("Ok(") || 
                            body.includes("Err(") ||
                            body.includes("Some(") || 
                            body.includes("None");
      
      // Service calls
      const hasServiceCall = body.includes("::new") || 
                            body.includes("service") || 
                            body.includes(".get") || 
                            body.includes(".find");
      
      // Response building
      const hasResponseBuilding = body.includes("Json(") || 
                                 body.includes("StatusCode") || 
                                 body.includes("response") ||
                                 body.includes("Reply") ||
                                 body.includes("IntoResponse");
      
      // JSON operations
      const hasJSONOps = body.includes("serde_json") || 
                        body.includes("from_json") || 
                        body.includes("to_json") ||
                        body.includes("deserialize") ||
                        body.includes("serialize");
      
      // Parameter handling
      const hasParamHandling = body.includes("Query<") || 
                              body.includes("Path<") || 
                              body.includes("param") ||
                              body.includes("extract") ||
                              body.includes("Json<");
      
      // Validation
      const hasValidation = body.includes("validate") || 
                           body.includes("is_valid") || 
                           body.includes("check_");
      
      // Auth checks
      const hasAuthChecks = body.includes("authenticate") || 
                           body.includes("authorize") || 
                           body.includes("permission") ||
                           body.includes("token") ||
                           body.includes("claims");
      
      // Calculate completeness score with more granular weights
      let score = 20; // Base score for non-empty function
      
      if (statements > 3) score += 5;
      if (statements > 10) score += 5;
      if (branches > 0) score += 5;
      if (branches > 3) score += 5;
      if (fnCalls > 3) score += 5;
      
      if (hasDbOps) score += 15;
      if (hasErrorHandling) score += 10;
      if (hasReturnValue) score += 10;
      if (hasServiceCall) score += 10;
      if (hasResponseBuilding) score += 10;
      if (hasJSONOps) score += 5;
      if (hasParamHandling) score += 5;
      if (hasValidation) score += 5;
      if (hasAuthChecks) score += 5;
      
      if (score >= 35) {
        console.log(`  Handler ${handlerName} scored ${score}% (implemented)`);
      }
      
      return Math.min(100, score);
    } catch (err) {
      console.error(`Error estimating completeness for ${handlerName}:`, err.message);
      return 20; // Default score on error
    }
  }
  
  addApiEndpoint(name, filePath, completeness, handlerType) {
    // Check if this endpoint already exists
    const existingIndex = this.metrics.apiEndpoints.details.findIndex(
      endpoint => endpoint.name === name && endpoint.file === filePath
    );
    
    if (existingIndex !== -1) {
      // Update existing endpoint if the new completeness is higher
      if (completeness > this.metrics.apiEndpoints.details[existingIndex].completeness) {
        this.metrics.apiEndpoints.details[existingIndex].completeness = completeness;
        
        // Update implemented count if needed
        if (completeness >= 35 && this.metrics.apiEndpoints.details[existingIndex].completeness < 35) {
          this.metrics.apiEndpoints.implemented++;
        } else if (completeness < 35 && this.metrics.apiEndpoints.details[existingIndex].completeness >= 35) {
          this.metrics.apiEndpoints.implemented--;
        }
      }
      return;
    }
    
    // Add new endpoint
    this.metrics.apiEndpoints.total++;
    
    this.metrics.apiEndpoints.details.push({
      name,
      file: filePath,
      completeness,
      handlerType,
      featureArea: this.determineFeatureArea(name, filePath)
    });
    
    // Lower the threshold to 35% to match previous analysis
    if (completeness >= 35) {
      this.metrics.apiEndpoints.implemented++;
    }
  }
  
  determineFeatureArea(name, filePath) {
    // Determine feature area based on name and file path
    const lowerName = name.toLowerCase();
    const lowerPath = filePath.toLowerCase();
    
    // Check file path first
    if (lowerPath.includes('/auth/') || lowerPath.includes('\\auth\\')) {
      return 'auth';
    }
    if (lowerPath.includes('/forum/') || lowerPath.includes('\\forum\\') || lowerPath.includes('forum_')) {
      return 'forum';
    }
    if (lowerPath.includes('/lms/') || lowerPath.includes('\\lms\\') || lowerPath.includes('canvas')) {
      return 'lms';
    }
    if (lowerPath.includes('/integration/') || lowerPath.includes('\\integration\\') || lowerPath.includes('sync')) {
      return 'integration';
    }
    
    // Then check name patterns
    for (const { type, pattern } of this.handlerPatterns) {
      if (pattern.test(lowerName)) {
        return type;
      }
    }
    
    return 'other';
  }
  
  categorizeEndpoints() {
    // Reset counts
    for (const area of Object.keys(this.metrics.featureAreas)) {
      this.metrics.featureAreas[area].total = 0;
      this.metrics.featureAreas[area].implemented = 0;
    }
    
    // Count endpoints by feature area
    for (const endpoint of this.metrics.apiEndpoints.details) {
      const area = endpoint.featureArea || 'other';
      
      this.metrics.featureAreas[area].total++;
      
      // Use the same 35% threshold as the main counter
      if (endpoint.completeness >= 35) {
        this.metrics.featureAreas[area].implemented++;
      }
    }
  }
  
  updateProjectStatus() {
    console.log("Updating project status document...");
    const statusFile = path.join(this.baseDir, 'project_status.md');
    
    if (!fs.existsSync(statusFile)) {
      console.error("Project status file not found!");
      return;
    }
    
    // Read file
    let statusContent = fs.readFileSync(statusFile, 'utf8');
    
    // Calculate API implementation percentage
    const apiPercentage = this.getPercentage(
      this.metrics.apiEndpoints.implemented, 
      this.metrics.apiEndpoints.total
    );
    
    // Simple search and replace with string methods instead of regex
    if (statusContent.includes('apiImplementation:')) {
      const startIdx = statusContent.indexOf('apiImplementation:');
      const endIdx = statusContent.indexOf(',', startIdx);
      if (endIdx > startIdx) {
        const beforeStr = statusContent.substring(0, startIdx);
        const afterStr = statusContent.substring(endIdx);
        statusContent = beforeStr + `apiImplementation: "${apiPercentage}%"` + afterStr;
      }
    }
    
    // Update last updated date
    const today = new Date().toISOString().split('T')[0];
    if (statusContent.includes('_Last updated:')) {
      const startIdx = statusContent.indexOf('_Last updated:');
      const endIdx = statusContent.indexOf('_', startIdx + 1);
      if (endIdx > startIdx) {
        const beforeStr = statusContent.substring(0, startIdx);
        const afterStr = statusContent.substring(endIdx);
        statusContent = beforeStr + `_Last updated: **${today}**` + afterStr;
      }
    }
    
    // Write updated content
    fs.writeFileSync(statusFile, statusContent);
    console.log("Project status document updated successfully!");
    
    // Generate enhanced API details report
    this.generateEnhancedApiReport();
  }
  
  generateEnhancedApiReport() {
    // Ensure docs directory exists
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir);
    }
    
    const reportFile = path.join(docsDir, 'api_implementation.md');
    
    let content = `# API Implementation Details\n_Generated on ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // API implementation summary
    const apiPercentage = this.getPercentage(
      this.metrics.apiEndpoints.implemented, 
      this.metrics.apiEndpoints.total
    );
    
    content += `## API Implementation: ${apiPercentage}%\n\n`;
    content += `- **Total API Endpoints**: ${this.metrics.apiEndpoints.total}\n`;
    content += `- **Implemented Endpoints**: ${this.metrics.apiEndpoints.implemented}\n`;
    content += `- **Implementation Rate**: ${apiPercentage}%\n\n`;
    
    // Add feature area breakdown
    content += `## Implementation by Feature Area\n\n`;
    content += "| Feature Area | Endpoints | Implemented | Percentage |\n";
    content += "|-------------|-----------|-------------|------------|\n";
    
    for (const [area, stats] of Object.entries(this.metrics.featureAreas)) {
      if (stats.total > 0) {
        const areaPercentage = this.getPercentage(stats.implemented, stats.total);
        content += `| ${area.charAt(0).toUpperCase() + area.slice(1)} | ${stats.total} | ${stats.implemented} | ${areaPercentage}% |\n`;
      }
    }
    
    content += "\n";
    
    // API endpoints by feature area
    for (const area of Object.keys(this.metrics.featureAreas)) {
      const areaEndpoints = this.metrics.apiEndpoints.details.filter(e => e.featureArea === area);
      
      if (areaEndpoints.length > 0) {
        const areaName = area.charAt(0).toUpperCase() + area.slice(1);
        const areaPercentage = this.getPercentage(
          this.metrics.featureAreas[area].implemented,
          this.metrics.featureAreas[area].total
        );
        
        content += `## ${areaName} API (${areaPercentage}%)\n\n`;
        content += "| Endpoint | File | Implementation | Status |\n";
        content += "|---------|------|----------------|--------|\n";
        
        // Sort by completeness (descending)
        const sortedEndpoints = [...areaEndpoints].sort((a, b) => b.completeness - a.completeness);
        
        for (const endpoint of sortedEndpoints) {
          // Lower thresholds to match the implementation criteria
          const status = endpoint.completeness >= 60 ? "✓ Complete" :
                        endpoint.completeness >= 35 ? "⚠ Partial" : "✗ Minimal";
          content += `| ${endpoint.name} | ${endpoint.file} | ${endpoint.completeness}% | ${status} |\n`;
        }
        
        content += "\n";
      }
    }
    
    // Implementation recommendations
    content += `## Implementation Recommendations\n\n`;
    
    // Find endpoints that are close to being considered implemented
    const almostImplemented = this.metrics.apiEndpoints.details
      .filter(e => e.completeness >= 25 && e.completeness < 35)
      .sort((a, b) => b.completeness - a.completeness)
      .slice(0, 5);
    
    if (almostImplemented.length > 0) {
      content += "### Priority Endpoints to Complete\n\n";
      content += "These endpoints are close to being considered implemented and should be prioritized:\n\n";
      
      for (const endpoint of almostImplemented) {
        content += `- **${endpoint.name}** (${endpoint.completeness}%) in \`${endpoint.file}\`\n`;
      }
      
      content += "\n";
    }
    
    // Feature area recommendations
    const areaPercentages = Object.entries(this.metrics.featureAreas)
      .map(([area, stats]) => ({
        area,
        percentage: stats.total > 0 ? (stats.implemented / stats.total) * 100 : 0
      }))
      .filter(a => a.percentage > 0)
      .sort((a, b) => a.percentage - b.percentage);
    
    if (areaPercentages.length > 0) {
      content += "### Feature Area Focus\n\n";
      content += "Based on current implementation rates, focus on these areas:\n\n";
      
      for (const { area, percentage } of areaPercentages.slice(0, 3)) {
        if (percentage < 50) {
          content += `- **${area.charAt(0).toUpperCase() + area.slice(1)}** API (${Math.round(percentage)}%)\n`;
        }
      }
    }
    
    // Write report
    fs.writeFileSync(reportFile, content);
    console.log(`Enhanced API implementation details saved to ${reportFile}`);
    
    // Update project_status.md to reference this file
    this.updateProjectStatusReference();
  }
  
  updateProjectStatusReference() {
    const statusFile = path.join(this.baseDir, 'project_status.md');
    
    if (fs.existsSync(statusFile)) {
      let statusContent = fs.readFileSync(statusFile, 'utf8');
      
      // Add reference to API implementation details if not already there
      if (!statusContent.includes('API Implementation Details')) {
        // Check if we need to add the Detailed Implementation section first
        if (!statusContent.includes('## 📊 Detailed Implementation')) {
          statusContent += '\n\n---\n\n## 📊 Detailed Implementation\n\n';
          statusContent += '### API Implementation\n\n';
          statusContent += `For detailed API implementation status, see [API Implementation Details](./docs/api_implementation.md)\n\n`;
          
          // Add feature area summary
          statusContent += "#### API by Feature Area\n\n";
          statusContent += "```\n";
          
          for (const [area, stats] of Object.entries(this.metrics.featureAreas)) {
            if (stats.total > 0) {
              const areaPercentage = this.getPercentage(stats.implemented, stats.total);
              statusContent += `${area.padEnd(12)}: ${stats.implemented.toString().padStart(2)}/${stats.total.toString().padStart(3)} (${areaPercentage.toString().padStart(2)}%)\n`;
            }
          }
          
          statusContent += "```\n";
        } else {
          // Find the section and position after it
          const sectionIndex = statusContent.indexOf('## 📊 Detailed Implementation');
          const nextSectionIndex = statusContent.indexOf('##', sectionIndex + 10);
          
          if (nextSectionIndex > sectionIndex) {
            // Insert before the next section
            const beforeSection = statusContent.substring(0, nextSectionIndex);
            const afterSection = statusContent.substring(nextSectionIndex);
            statusContent = beforeSection + 
                           '\n### API Implementation\n\n' +
                           `For detailed API implementation status, see [API Implementation Details](./docs/api_implementation.md)\n\n` +
                           "#### API by Feature Area\n\n" +
                           "```\n" +
                           Object.entries(this.metrics.featureAreas)
                             .filter(([_, stats]) => stats.total > 0)
                             .map(([area, stats]) => {
                               const areaPercentage = this.getPercentage(stats.implemented, stats.total);
                               return `${area.padEnd(12)}: ${stats.implemented.toString().padStart(2)}/${stats.total.toString().padStart(3)} (${areaPercentage.toString().padStart(2)}%)`;
                             })
                             .join('\n') +
                           "\n```\n\n" +
                           afterSection;
          } else {
            // Just append at the end of the section
            statusContent += '\n\n### API Implementation\n\n';
            statusContent += `For detailed API implementation status, see [API Implementation Details](./docs/api_implementation.md)\n\n`;
            
            // Add feature area summary
            statusContent += "#### API by Feature Area\n\n";
            statusContent += "```\n";
            
            for (const [area, stats] of Object.entries(this.metrics.featureAreas)) {
              if (stats.total > 0) {
                const areaPercentage = this.getPercentage(stats.implemented, stats.total);
                statusContent += `${area.padEnd(12)}: ${stats.implemented.toString().padStart(2)}/${stats.total.toString().padStart(3)} (${areaPercentage.toString().padStart(2)}%)\n`;
              }
            }
            
            statusContent += "```\n";
          }
        }
        
        // Write back updated content
        fs.writeFileSync(statusFile, statusContent);
        console.log("Added API implementation reference to project_status.md");
      }
    }
  }
  
  getPercentage(implemented, total) {
    if (total === 0) return 0;
    return Math.round((implemented / total) * 100);
  }
}

// Run the analyzer
async function main() {
  try {
    const baseDir = path.resolve(__dirname);
    const analyzer = new AdvancedApiAnalyzer(baseDir);
    await analyzer.analyze();
  } catch (err) {
    console.error("Error during analysis:", err);
    process.exit(1);
  }
}

main();