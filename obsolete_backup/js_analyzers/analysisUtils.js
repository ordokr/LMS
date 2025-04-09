/**
 * Utility functions for code analysis
 */
const fs = require('fs');
const path = require('path');

// Create a simple object with analysis methods rather than a class
const AnalysisUtils = {
  /**
   * Analyzes code quality of a project
   * @param {Map|Object} fileContentsMap - Map or object containing file contents
   * @param {Object} options - Analysis options
   * @returns {Object} Code quality analysis results
   */
  analyzeCodeQuality(fileContentsMap, options = {}) {
    console.log("Analyzing code quality...");
    
    // Ensure we have a Map to work with
    let contentsMap;
    
    if (fileContentsMap instanceof Map) {
      contentsMap = fileContentsMap;
    } else if (fileContentsMap && typeof fileContentsMap === 'object') {
      // Convert object to Map
      contentsMap = new Map();
      for (const [key, value] of Object.entries(fileContentsMap)) {
        contentsMap.set(key, value);
      }
    } else {
      // Handle case where input is neither Map nor Object
      console.log("No valid file contents provided for code quality analysis");
      contentsMap = new Map();
    }
    
    if (contentsMap.size === 0) {
      console.log("No file contents available for code quality analysis");
      return {
        codeSmells: [],
        complexity: {
          average: 0,
          byFile: {}
        },
        qualityScore: 0
      };
    }
    
    const codeSmells = [];
    const complexity = {
      average: 0,
      total: 0,
      count: 0,
      byFile: {}
    };
    
    // Analyze each file
    for (const [filePath, content] of contentsMap.entries()) {
      if (!content || typeof content !== 'string') {
        console.log(`Skipping analysis for ${filePath} - invalid content`);
        continue;
      }
      
      const ext = path.extname(filePath).toLowerCase();
      
      // Skip non-code files
      if (!['.js', '.jsx', '.ts', '.tsx', '.rs', '.go', '.py', '.java', '.c', '.cpp', '.cs'].includes(ext)) {
        continue;
      }
      
      // Calculate file complexity
      const fileComplexity = this._calculateComplexity(content, ext);
      complexity.byFile[filePath] = fileComplexity;
      complexity.total += fileComplexity;
      complexity.count++;
      
      // Find code smells
      const fileCodeSmells = this._findCodeSmells(filePath, content, ext);
      codeSmells.push(...fileCodeSmells);
    }
    
    // Calculate average complexity
    complexity.average = complexity.count > 0 ? complexity.total / complexity.count : 0;
    
    // Calculate overall quality score (0-100)
    const qualityScore = this._calculateQualityScore(codeSmells, complexity);
    
    return {
      codeSmells,
      complexity,
      qualityScore
    };
  },
  
  /**
   * Calculates code complexity
   * @private
   */
  _calculateComplexity(content, fileExtension) {
    // Simple complexity calculation based on:
    // 1. Number of conditional statements (if/else/switch)
    // 2. Number of loops (for/while)
    // 3. Function nesting depth
    
    const lines = content.split('\n');
    let complexity = 0;
    
    // Count conditional statements and loops
    const conditionalMatch = content.match(/if\s*\(|else|switch\s*\(|case\s+/g);
    const loopMatch = content.match(/for\s*\(|while\s*\(|do\s*{/g);
    
    complexity += (conditionalMatch ? conditionalMatch.length : 0) * 1.5;
    complexity += (loopMatch ? loopMatch.length : 0) * 2;
    
    // Add complexity based on file length
    complexity += Math.floor(lines.length / 100);
    
    return Math.round(complexity);
  },
  
  /**
   * Finds code smells in the given content
   * @private
   */
  _findCodeSmells(filePath, content, fileExtension) {
    const codeSmells = [];
    const lines = content.split('\n');
    
    // Check for long lines
    lines.forEach((line, index) => {
      if (line.length > 100) {
        codeSmells.push({
          file: filePath,
          line: index + 1,
          type: 'Long Line',
          description: `Line exceeds 100 characters (${line.length})`,
          severity: 'low'
        });
      }
    });
    
    // Check for TODOs and FIXMEs
    const todoMatches = content.match(/\/\/\s*TODO|\/\*\s*TODO|#\s*TODO/g);
    if (todoMatches) {
      codeSmells.push({
        file: filePath,
        line: null,
        type: 'TODO Comment',
        description: `File contains ${todoMatches.length} TODO comment(s)`,
        severity: 'info'
      });
    }
    
    const fixmeMatches = content.match(/\/\/\s*FIXME|\/\*\s*FIXME|#\s*FIXME/g);
    if (fixmeMatches) {
      codeSmells.push({
        file: filePath,
        line: null,
        type: 'FIXME Comment',
        description: `File contains ${fixmeMatches.length} FIXME comment(s)`,
        severity: 'medium'
      });
    }
    
    // Check for large functions (basic heuristic)
    const functionMatches = content.match(/function\s+\w+\s*\([^)]*\)\s*{/g);
    if (functionMatches && functionMatches.length > 0) {
      // Very simple heuristic: if file has many functions and is long, some functions are likely too large
      if (functionMatches.length > 5 && lines.length > 300) {
        codeSmells.push({
          file: filePath,
          line: null,
          type: 'Large File with Many Functions',
          description: `File has ${functionMatches.length} functions and ${lines.length} lines`,
          severity: 'medium'
        });
      }
    }
    
    return codeSmells;
  },
  
  /**
   * Calculates overall code quality score
   * @private
   */
  _calculateQualityScore(codeSmells, complexity) {
    // Start with a perfect score
    let score = 100;
    
    // Deduct points for complexity
    if (complexity.average > 30) {
      score -= 20;
    } else if (complexity.average > 20) {
      score -= 15;
    } else if (complexity.average > 10) {
      score -= 10;
    } else if (complexity.average > 5) {
      score -= 5;
    }
    
    // Deduct points for code smells by severity
    const severeSmells = codeSmells.filter(smell => smell.severity === 'high').length;
    const mediumSmells = codeSmells.filter(smell => smell.severity === 'medium').length;
    const lowSmells = codeSmells.filter(smell => smell.severity === 'low').length;
    
    score -= severeSmells * 5;
    score -= mediumSmells * 2;
    score -= lowSmells * 0.5;
    
    // Ensure the score is between 0 and 100
    return Math.max(0, Math.min(100, Math.round(score)));
  },
  
  // Analysis methods
  analyzeAPIEndpoints(files, options = {}) {
    console.log("Analyzing API endpoints...");
    // Placeholder for API endpoint analysis
    return {
      endpoints: [],
      implementationStatus: {}
    };
  },
  
  analyzeApiEndpoints(files, options = {}) {
    // This is the method name expected by unified-project-analyzer.js
    console.log("Analyzing API endpoints...");
    // Call our existing implementation with capitalized API
    return this.analyzeAPIEndpoints(files, options);
  },

  analyzeModels(files, options = {}) {
    console.log("Analyzing models...");
    // Placeholder for model analysis
    return {
      models: [],
      implementationStatus: {}
    };
  },
  
  analyzeUIComponents(files, options = {}) {
    console.log("Analyzing UI components (Leptos)...");
    // Placeholder for UI component analysis
    return {
      components: [],
      implementationStatus: {}
    };
  },
  
  analyzeTests(files, options = {}) {
    console.log("Analyzing tests...");
    // Placeholder for test analysis
    return {
      tests: [],
      coverage: {}
    };
  }
};

// Export the object directly
module.exports = AnalysisUtils;