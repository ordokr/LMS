/**
 * Use machine learning techniques to detect code smells
 */
const fs = require('fs');
const path = require('path');

class MLAnalyzer {
  constructor(metrics) {
    this.metrics = metrics;
    this.features = {};
  }
  
  /**
   * Extract code features for ML-based analysis
   */
  extractFeatures(filePath, content, ast) {
    // Basic metrics
    const lines = content.split('\n').length;
    const chars = content.length;
    const commentLines = (content.match(/\/\/.*/g) || []).length + 
                        ((content.match(/\/\*[\s\S]*?\*\//g) || [])
                          .join('\n').split('\n').length);
    
    // AST-based metrics
    let maxNestingLevel = 0;
    let currentNestingLevel = 0;
    let functionCount = 0;
    let longFunctionCount = 0;
    let complexConditionCount = 0;
    
    traverse(ast, {
      enter(node) {
        // Track nesting level
        if (node.type === 'BlockStatement' || 
            node.type === 'ObjectExpression' || 
            node.type === 'ArrayExpression') {
          currentNestingLevel++;
          maxNestingLevel = Math.max(maxNestingLevel, currentNestingLevel);
        }
        
        // Count functions
        if (node.type === 'FunctionDeclaration' || 
            node.type === 'FunctionExpression' || 
            node.type === 'ArrowFunctionExpression') {
          functionCount++;
          
          // Check for long functions
          if (node.body && node.body.type === 'BlockStatement' && 
              node.body.body && node.body.body.length > 15) {
            longFunctionCount++;
          }
        }
        
        // Check for complex conditions
        if ((node.type === 'LogicalExpression' || node.type === 'BinaryExpression') &&
            node.left && node.right &&
            (node.left.type === 'LogicalExpression' || node.left.type === 'BinaryExpression' ||
             node.right.type === 'LogicalExpression' || node.right.type === 'BinaryExpression')) {
          complexConditionCount++;
        }
      },
      exit(node) {
        if (node.type === 'BlockStatement' || 
            node.type === 'ObjectExpression' || 
            node.type === 'ArrayExpression') {
          currentNestingLevel--;
        }
      }
    });
    
    // Store features
    this.features[filePath] = {
      lines,
      chars,
      commentLines,
      commentRatio: commentLines / lines,
      maxNestingLevel,
      functionCount,
      longFunctionCount,
      longFunctionRatio: functionCount > 0 ? longFunctionCount / functionCount : 0,
      complexConditionCount,
      avgCharsPerLine: lines > 0 ? chars / lines : 0,
      timestamp: Date.now()
    };
    
    return this.features[filePath];
  }
  
  /**
   * Detect abnormal code using statistical methods
   */
  detectAbnormalCode() {
    const files = Object.keys(this.features);
    if (files.length < 5) return []; // Need enough samples
    
    // Calculate mean and standard deviation for each metric
    const metrics = ['maxNestingLevel', 'longFunctionRatio', 'complexConditionCount', 'commentRatio'];
    const stats = {};
    
    metrics.forEach(metric => {
      const values = files.map(file => this.features[file][metric]);
      const mean = values.reduce((sum, val) => sum + val, 0) / values.length;
      const variance = values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length;
      const stdDev = Math.sqrt(variance);
      
      stats[metric] = { mean, stdDev };
    });
    
    // Detect outliers (z-score > 2)
    const outliers = [];
    
    files.forEach(file => {
      const feature = this.features[file];
      const issues = [];
      
      metrics.forEach(metric => {
        const zScore = Math.abs((feature[metric] - stats[metric].mean) / stats[metric].stdDev);
        
        if (zScore > 2) {
          issues.push({
            metric,
            value: feature[metric],
            mean: stats[metric].mean,
            zScore
          });
        }
      });
      
      if (issues.length > 0) {
        outliers.push({
          file,
          issues,
          score: issues.reduce((sum, issue) => sum + issue.zScore, 0) / issues.length
        });
      }
    });
    
    // Sort by outlier score
    outliers.sort((a, b) => b.score - a.score);
    
    // Update metrics with ML findings
    this.metrics.codeQuality.mlAnalysis = {
      outliers,
      stats
    };
    
    return outliers;
  }

  // Add this method to the MLAnalyzer class

  /**
   * Generate embeddings for RAG documents
   */
  async generateEmbeddingsForRagDocuments(ragDir) {
    console.log("Generating embeddings for RAG documents...");
    
    // Check if embedding capability is available
    if (!this.tensorflowLoaded) {
      console.warn("TensorFlow not available for embedding generation");
      return;
    }
    
    const embeddingsDir = path.join(ragDir, 'embeddings');
    fs.ensureDirSync(embeddingsDir);
    
    // Get all markdown files recursively
    const files = this.getAllMarkdownFiles(ragDir);
    console.log(`Found ${files.length} documents for embedding generation`);
    
    // Generate embeddings for each document
    const embeddings = {};
    
    for (const file of files) {
      try {
        // Skip embedding files
        if (file.includes('embeddings/')) continue;
        
        // Read file content
        const content = fs.readFileSync(file, 'utf8');
        
        // Generate embedding
        const embedding = await this.generateDocumentEmbedding(content);
        
        // Store with relative path as key
        const relativePath = path.relative(ragDir, file);
        embeddings[relativePath] = embedding;
        
        console.log(`Generated embedding for ${relativePath}`);
      } catch (err) {
        console.error(`Error generating embedding for ${file}:`, err.message);
      }
    }
    
    // Save all embeddings
    fs.writeFileSync(
      path.join(embeddingsDir, 'document_embeddings.json'),
      JSON.stringify(embeddings, null, 2)
    );
    
    // Generate metadata file
    const metadata = {
      created: new Date().toISOString(),
      documentCount: Object.keys(embeddings).length,
      embeddingDimensions: Object.values(embeddings)[0]?.length || 0,
      model: 'universal-sentence-encoder'
    };
    
    fs.writeFileSync(
      path.join(embeddingsDir, 'metadata.json'),
      JSON.stringify(metadata, null, 2)
    );
    
    console.log(`Generated embeddings for ${Object.keys(embeddings).length} documents`);
    return embeddings;
  }

  /**
   * Generate embedding for a document
   */
  async generateDocumentEmbedding(text) {
    // This is a simplified version - you'd use actual embedding model here
    if (!this.use) {
      try {
        // Load Universal Sentence Encoder
        this.use = await require('@tensorflow-models/universal-sentence-encoder').load();
      } catch (err) {
        console.error("Error loading Universal Sentence Encoder:", err.message);
        return null;
      }
    }
    
    // Generate embedding
    try {
      const embeddings = await this.use.embed(text);
      const embeddingArray = await embeddings.array();
      return embeddingArray[0]; // First item in batch
    } catch (err) {
      console.error("Error generating document embedding:", err.message);
      return null;
    }
  }

  /**
   * Get all markdown files recursively
   */
  getAllMarkdownFiles(dir) {
    let results = [];
    const list = fs.readdirSync(dir);
    
    list.forEach(file => {
      const fullPath = path.join(dir, file);
      const stat = fs.statSync(fullPath);
      
      if (stat.isDirectory()) {
        results = results.concat(this.getAllMarkdownFiles(fullPath));
      } else if (file.endsWith('.md')) {
        results.push(fullPath);
      }
    });
    
    return results;
  }
}

module.exports = MLAnalyzer;