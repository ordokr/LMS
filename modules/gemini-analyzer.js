/**
 * Module for Google Gemini AI integration
 */
const fs = require('fs');
const path = require('path');
const { GoogleGenerativeAI } = require('@google/generative-ai');

class GeminiAnalyzer {
  constructor(metrics, options = {}) {
    this.metrics = metrics;
    this.options = options;
    this.apiKey = options.geminiApiKey || "AIzaSyC2bS3qMexvmemLfg-V4ZQ-GbTcg-RgCQ8";
    
    // Initialize Google Generative AI
    this.genAI = new GoogleGenerativeAI(this.apiKey);
    this.model = this.genAI.getGenerativeModel({ 
      model: options.modelName || "gemini-1.5-pro" 
    });
    
    this.cacheDir = path.join(options.baseDir || process.cwd(), '.analysis_cache', 'gemini');
    if (!fs.existsSync(this.cacheDir)) {
      fs.mkdirSync(this.cacheDir, { recursive: true });
    }
  }

  /**
   * Generate AI code insights for specific files
   */
  async generateCodeInsights(files, fsUtils) {
    console.log('Generating Gemini AI insights for code files...');
    
    // Filter to only analyze important files
    const filteredFiles = files.filter(file => {
      // Only analyze source code files
      const ext = path.extname(file).toLowerCase();
      return ['.js', '.ts', '.jsx', '.tsx', '.rs', '.py', '.java', '.rb'].includes(ext);
    });
    
    // Select most important files (models, controllers, complex files)
    const priorityFiles = filteredFiles.filter(file => {
      // Check if it's a model
      const isModel = this.metrics.models.details.some(m => m.file === file);
      
      // Check if it's an API endpoint
      const isEndpoint = this.metrics.apiEndpoints.details.some(e => e.file === file);
      
      // Check if it has high complexity
      const isComplex = this.metrics.codeQuality.complexity.files.some(
        c => c.file === file && c.complexity > 10
      );
      
      return isModel || isEndpoint || isComplex;
    });
    
    // Limit to 10 most important files to prevent excessive API usage
    const filesToProcess = priorityFiles.slice(0, 10);
    console.log(`Selected ${filesToProcess.length} files for AI analysis`);
    
    const insights = {};
    
    // Process each file
    for (const file of filesToProcess) {
      const cacheFile = path.join(this.cacheDir, this.getCacheFileName(file));
      
      // Check if we have cached results
      if (fs.existsSync(cacheFile)) {
        try {
          const cachedData = JSON.parse(fs.readFileSync(cacheFile, 'utf8'));
          insights[file] = cachedData;
          console.log(`Using cached insights for ${file}`);
          continue;
        } catch (err) {
          console.warn(`Could not use cache for ${file}:`, err.message);
        }
      }
      
      try {
        console.log(`Analyzing ${file} with Gemini...`);
        const content = fsUtils.getFileContent(file);
        if (!content || content.length < 50) continue;
        
        // Truncate content if too large (Gemini has context limits)
        const truncatedContent = content.length > 30000 ? 
          content.substring(0, 30000) + '...[truncated]' : content;
        
        // Generate the prompt
        const prompt = `You are a senior software architect reviewing code.
Please analyze this code file and provide:
1. A brief summary of what this code does
2. Design patterns identified
3. Potential SOLID principle violations
4. Recommendations for improvements
5. Security concerns (if any)
6. Code quality assessment (1-10 scale)

File: ${path.basename(file)}
${truncatedContent}`;

        // Send to Gemini
        const result = await this.model.generateContent(prompt);
        const response = result.response;
        const insight = {
          summary: response.text(),
          file: path.basename(file),
          timestamp: Date.now()
        };
        
        // Cache the result
        fs.writeFileSync(cacheFile, JSON.stringify(insight, null, 2));
        
        insights[file] = insight;
        console.log(`Generated insights for ${file}`);
        
        // Wait a short time to avoid hitting rate limits
        await new Promise(resolve => setTimeout(resolve, 1000));
        
      } catch (error) {
        console.error(`Error analyzing ${file} with Gemini:`, error.message);
      }
    }
    
    this.metrics.aiInsights = insights;
    return insights;
  }
  
  /**
   * Generate a project overview using Gemini
   */
  async generateProjectOverview() {
    console.log('Generating project overview with Gemini AI...');
    
    try {
      // Create summary of project metrics for Gemini
      const projectSummary = {
        models: {
          total: this.metrics.models.total,
          implemented: this.metrics.models.implemented,
          percent: this.getPercentage(this.metrics.models.implemented, this.metrics.models.total)
        },
        apiEndpoints: {
          total: this.metrics.apiEndpoints.total,
          implemented: this.metrics.apiEndpoints.implemented,
          percent: this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)
        },
        uiComponents: {
          total: this.metrics.uiComponents.total,
          implemented: this.metrics.uiComponents.implemented,
          percent: this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)
        },
        tests: {
          total: this.metrics.tests.total,
          coverage: this.metrics.tests.coverage
        },
        codeQuality: {
          complexity: this.metrics.codeQuality.complexity.average.toFixed(1),
          highComplexityFiles: this.metrics.codeQuality.complexity.high,
          techDebt: this.metrics.codeQuality.techDebt.score
        },
        solidViolations: Object.entries(this.metrics.codeQuality.solidViolations || {})
          .map(([principle, violations]) => ({
            principle,
            count: violations.length
          }))
      };
      
      // Get top 5 models
      const topModels = this.metrics.models.details
        .sort((a, b) => b.completeness - a.completeness)
        .slice(0, 5)
        .map(m => ({
          name: m.name,
          completeness: m.completeness
        }));
      
      // Prepare prompt for Gemini
      const prompt = `You are a senior software architect analyzing a project. 
Based on the following metrics, provide a comprehensive assessment of the project:

PROJECT METRICS:
${JSON.stringify(projectSummary, null, 2)}

TOP MODELS:
${JSON.stringify(topModels, null, 2)}

Please include the following sections in your Markdown-formatted response:
1. ## Project Status Overview
2. ## Implementation Assessment
3. ## Code Quality Analysis
4. ## Recommendations
5. ## Next Steps

Include specific recommendations for improving code quality and implementation speed.`;

      const result = await this.model.generateContent(prompt);
      const text = result.response.text();
      
      // Cache the result
      const cacheFile = path.join(this.cacheDir, 'project_overview.json');
      fs.writeFileSync(cacheFile, JSON.stringify({ 
        overview: text,
        timestamp: Date.now()
      }, null, 2));
      
      return text;
    } catch (error) {
      console.error('Error generating project overview with Gemini:', error.message);
      return "Failed to generate project overview with Gemini AI.";
    }
  }
  
  /**
   * Generate a complete project assessment report
   */
  async generateProjectAssessmentReport(baseDir) {
    const overview = await this.generateProjectOverview();
    
    // Create the report file
    const docsDir = path.join(baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    const reportPath = path.join(docsDir, 'gemini_project_assessment.md');
    
    let content = `# Gemini AI Project Assessment\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    content += overview;
    
    // Add insights for specific files if available
    if (this.metrics.aiInsights) {
      content += `\n\n## File-Specific Insights\n\n`;
      
      Object.entries(this.metrics.aiInsights)
        .slice(0, 5) // Limit to top 5 files to keep the report concise
        .forEach(([file, insight]) => {
          content += `### ${path.basename(file)}\n\n`;
          content += `${insight.summary}\n\n`;
          content += `---\n\n`;
        });
    }
    
    fs.writeFileSync(reportPath, content);
    console.log(`Gemini project assessment report saved to ${reportPath}`);
    return reportPath;
  }
  
  /**
   * Generate a source-target mapping improvement suggestions
   */
  async generateMappingImprovement() {
    if (!this.metrics.sourceToTarget || !this.metrics.sourceToTarget.models) {
      return null;
    }
    
    try {
      // Create data for source-target analysis
      const mappingData = {
        models: this.metrics.sourceToTarget.models.slice(0, 10),
        controllers: this.metrics.sourceToTarget.controllers.slice(0, 10),
        missingModels: this.metrics.sourceToTarget.missingModels || [],
        missingControllers: this.metrics.sourceToTarget.missingControllers || []
      };
      
      const prompt = `You are a software architect analyzing the mapping between source systems and target implementation.
Based on this mapping data, provide recommendations for improving the integration:

MAPPING DATA:
${JSON.stringify(mappingData, null, 2)}

Please provide your recommendations in Markdown format with these sections:
1. ## Mapping Analysis
2. ## Integration Gaps
3. ## Implementation Priorities
4. ## Technical Approach Recommendations`;

      const result = await this.model.generateContent(prompt);
      const text = result.response.text();
      
      // Cache the result
      const cacheFile = path.join(this.cacheDir, 'mapping_improvement.json');
      fs.writeFileSync(cacheFile, JSON.stringify({ 
        analysis: text,
        timestamp: Date.now()
      }, null, 2));
      
      return text;
    } catch (error) {
      console.error('Error generating mapping improvement with Gemini:', error.message);
      return null;
    }
  }
  
  /**
   * Generate code insights report
   */
  async generateCodeInsightsReport() {
    if (!this.metrics.aiInsights) {
      console.log('No AI insights available to generate report');
      return null;
    }
    
    const docsDir = path.join(this.options.baseDir || process.cwd(), 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    const outputPath = path.join(docsDir, 'ai_code_insights.md');
    
    let content = `# AI-Powered Code Insights\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Add project overview if we have one
    try {
      const overview = await this.generateProjectOverview();
      content += `## Project Overview\n\n`;
      content += overview;
      content += `\n\n`;
    } catch (err) {
      console.warn("Could not generate project overview:", err.message);
    }
    
    // Add code insights
    content += `## Code Analysis\n\n`;
    
    // Group insights by category
    const insights = Object.entries(this.metrics.aiInsights).reduce((result, [file, insight]) => {
      // Try to determine file type/purpose
      let category = 'Other';
      
      if (file.includes('model') || file.includes('entity') || file.includes('schema')) {
        category = 'Models';
      } else if (file.includes('controller') || file.includes('route') || file.includes('api')) {
        category = 'APIs';
      } else if (file.includes('component') || file.includes('view') || file.includes('page')) {
        category = 'UI Components';
      } else if (file.includes('test') || file.includes('spec')) {
        category = 'Tests';
      } else if (file.includes('util') || file.includes('helper')) {
        category = 'Utilities';
      }
      
      if (!result[category]) {
        result[category] = [];
      }
      
      result[category].push({ file, insight });
      return result;
    }, {});
    
    // Add each category
    Object.entries(insights).forEach(([category, items]) => {
      content += `### ${category}\n\n`;
      
      items.forEach(({ file, insight }) => {
        content += `#### ${path.basename(file)}\n\n`;
        content += insight.summary + '\n\n';
        content += `---\n\n`;
      });
    });
    
    // Add patterns and anti-patterns section
    content += `## Identified Patterns & Anti-patterns\n\n`;
    
    try {
      const patternPrompt = `Based on the AI insights for these files, list common design patterns and anti-patterns found in the codebase:

${JSON.stringify(this.metrics.aiInsights, null, 2)}

Format your response in markdown with two sections:
1. Common Design Patterns
2. Anti-patterns & Issues`;

      const result = await this.model.generateContent(patternPrompt);
      content += result.response.text();
    } catch (err) {
      console.warn("Could not generate patterns section:", err.message);
      content += `No patterns analysis available.\n\n`;
    }
    
    // Save the report
    try {
      fs.writeFileSync(outputPath, content);
      console.log(`AI Code Insights report generated at ${outputPath}`);
      return outputPath;
    } catch (error) {
      console.error(`Failed to write AI Code Insights report: ${error.message}`);
      return null;
    }
  }
  
  /**
   * Get cache file name for a file
   */
  getCacheFileName(filePath) {
    // Create a safe filename from the path
    const basename = path.basename(filePath);
    const hash = require('crypto')
      .createHash('md5')
      .update(filePath)
      .digest('hex')
      .substring(0, 8);
    
    return `${basename.replace(/[^a-zA-Z0-9]/g, '_')}_${hash}.json`;
  }
  
  /**
   * Calculate percentage safely
   */
  getPercentage(value, total) {
    if (total === 0) return 0;
    return Math.round((value / total) * 100);
  }
}

module.exports = GeminiAnalyzer;