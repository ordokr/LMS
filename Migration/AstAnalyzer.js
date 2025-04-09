// New file: c:\Users\Tim\Desktop\LMS\astAnalyzer.js
const parser = require('@babel/parser');
const traverse = require('@babel/traverse').default;

class AstAnalyzer {
  constructor() {
    this.parseOptions = {
      sourceType: 'module',
      plugins: ['jsx', 'typescript', 'classProperties', 'decorators-legacy']
    };
  }
  
  parseToAst(content, filePath = 'unknown') {
    try {
      return parser.parse(content, this.parseOptions);
    } catch (error) {
      return null; // Return null on parsing error
    }
  }
  
  calculateComplexity(ast) {
    if (!ast) return 1;
    
    try {
      let complexity = 1;
      
      // Rest of method...
      
      console.log(`Debug: Calculated complexity: ${complexity} for AST`);
      return complexity;
    } catch (error) {
      console.warn(`Error calculating complexity: ${error.message}`);
      return 1;
    }
  }
  
  analyzeComponentAst(content, filePath) {
    // Move existing analyzeComponentAst method here
  }

  /**
   * Estimate complexity for Rust code (basic version)
   */
  estimateRustComplexity(content) {
    let complexity = 1;
    // Count decision points: if, else if, for, while, loop, match arms, ? operator
    complexity += (content.match(/if\s|else\s+if|for\s|while\s|loop\s*\{|match\s|=>|Ok\s*\(/g) || []).length;
    // Add complexity for closures
    complexity += (content.match(/\|[^|]*\|/g) || []).length;
    // Add complexity for nested functions (basic check)
    complexity += (content.match(/fn\s+\w+\s*\(/g) || []).length - 1; // Subtract 1 for the main function if present

    return Math.max(1, complexity); // Ensure complexity is at least 1
  }
}

module.exports = AstAnalyzer;