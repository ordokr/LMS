const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

/**
 * Comprehensive Analysis Runner
 * Runs analysis on source systems (Canvas & Discourse) and target Rust LMS application
 */
async function runFullAnalysis() {
  console.log('===================================');
  console.log('🔍 STARTING COMPREHENSIVE ANALYSIS 🔍');
  console.log('===================================\n');

  const startTime = Date.now();
  
  // Base directories
  const lmsDir = path.resolve(__dirname);
  const portDir = path.resolve('C:\\Users\\Tim\\Desktop\\port');
  const canvasDir = path.join(portDir, 'canvas');
  const discourseDir = path.join(portDir, 'port');
  
  // Create summary directory if it doesn't exist
  const summaryDir = path.join(lmsDir, 'analysis_summary');
  if (!fs.existsSync(summaryDir)) {
    fs.mkdirSync(summaryDir);
  }
  
  // Step 1: Analyze source systems (Canvas)
  console.log('1️⃣ Analyzing Canvas (Source System)...');
  if (fs.existsSync(canvasDir)) {
    try {
      const canvasOutput = runSourceAnalysis(canvasDir, 'Canvas');
      fs.writeFileSync(
        path.join(summaryDir, 'canvas_analysis.md'), 
        canvasOutput
      );
    } catch (error) {
      console.error(`❌ Error analyzing Canvas: ${error.message}`);
    }
  } else {
    console.warn(`⚠️ Canvas source directory not found at ${canvasDir}`);
  }
  
  // Step 2: Analyze source systems (Discourse)
  console.log('\n2️⃣ Analyzing Discourse (Source System)...');
  if (fs.existsSync(discourseDir)) {
    try {
      const discourseOutput = runSourceAnalysis(discourseDir, 'Discourse');
      fs.writeFileSync(
        path.join(summaryDir, 'discourse_analysis.md'), 
        discourseOutput
      );
    } catch (error) {
      console.error(`❌ Error analyzing Discourse: ${error.message}`);
    }
  } else {
    console.warn(`⚠️ Discourse source directory not found at ${discourseDir}`);
  }
  
  // Step 3: Run unified project analyzer on LMS
  console.log('\n3️⃣ Running Unified Project Analyzer on target LMS app...');
  try {
    execSync('node unified-project-analyzer.js', { 
      stdio: 'inherit',
      cwd: lmsDir
    });
  } catch (error) {
    console.error(`❌ Error running unified project analyzer: ${error.message}`);
  }
  
  // Step 4: Consolidate results into a master report
  console.log('\n4️⃣ Generating Consolidated Report...');
  await generateConsolidatedReport(summaryDir, lmsDir, canvasDir, discourseDir);
  
  const duration = ((Date.now() - startTime) / 1000).toFixed(2);
  console.log(`\n✅ Comprehensive analysis completed in ${duration}s`);
  console.log(`📄 Master report generated at: ${path.join(summaryDir, 'master_report.md')}`);
  console.log(`📊 Central Reference Hub: ${path.join(lmsDir, 'docs', 'central_reference_hub.md')}`);
}

/**
 * Run analysis on source systems (Canvas/Discourse)
 */
function runSourceAnalysis(sourceDir, systemName) {
  console.log(`Analyzing ${systemName} codebase at ${sourceDir}...`);
  
  // Count files by type
  const fileStats = countFilesByType(sourceDir);
  
  // Count lines of code
  const locStats = countLinesOfCode(sourceDir);
  
  // Analyze models
  const models = findModels(sourceDir, systemName.toLowerCase());
  
  // Analyze controllers
  const controllers = findControllers(sourceDir, systemName.toLowerCase());
  
  // Format the output as markdown
  let output = `# ${systemName} Source Code Analysis\n\n`;
  output += `_Analysis performed on ${new Date().toISOString().split('T')[0]}_\n\n`;
  
  // High level stats
  output += `## Overview\n\n`;
  output += `- **Total Files**: ${fileStats.total}\n`;
  output += `- **Lines of Code**: ${locStats.total.toLocaleString()}\n`;
  output += `- **Models**: ${models.length}\n`;
  output += `- **Controllers**: ${controllers.length}\n\n`;
  
  // Files by type
  output += `## File Types\n\n`;
  output += `| Extension | Count | Lines of Code |\n`;
  output += `|-----------|-------|---------------|\n`;
  
  for (const [ext, count] of Object.entries(fileStats.byExtension)) {
    const loc = locStats.byExtension[ext] || 0;
    output += `| ${ext || '(no extension)'} | ${count} | ${loc.toLocaleString()} |\n`;
  }
  output += `\n`;
  
  // Models
  output += `## Models\n\n`;
  if (models.length > 0) {
    output += `| Model | File | Fields | Associations |\n`;
    output += `|-------|------|--------|-------------|\n`;
    
    for (const model of models) {
      output += `| ${model.name} | ${model.file} | ${model.fieldCount} | ${model.associations.join(', ') || 'none'} |\n`;
    }
  } else {
    output += `No models found in the analyzed codebase.\n`;
  }
  output += `\n`;
  
  // Controllers
  output += `## Controllers\n\n`;
  if (controllers.length > 0) {
    output += `| Controller | File | Actions | Routes |\n`;
    output += `|------------|------|---------|--------|\n`;
    
    for (const controller of controllers) {
      output += `| ${controller.name} | ${controller.file} | ${controller.actions.length} | ${controller.routes.join(', ') || 'none'} |\n`;
    }
  } else {
    output += `No controllers found in the analyzed codebase.\n`;
  }
  
  return output;
}

/**
 * Count files by type
 */
function countFilesByType(dir) {
  const stats = { total: 0, byExtension: {} };
  const excludeDirs = ['node_modules', '.git', 'tmp', 'log', 'public/assets', 'coverage'];
  
  function walkDir(currentDir) {
    const items = fs.readdirSync(currentDir);
    
    for (const item of items) {
      const itemPath = path.join(currentDir, item);
      const isExcluded = excludeDirs.some(excluded => 
        itemPath.includes(path.sep + excluded + path.sep) || 
        itemPath.endsWith(path.sep + excluded)
      );
      
      if (isExcluded) continue;
      
      const stat = fs.statSync(itemPath);
      
      if (stat.isDirectory()) {
        walkDir(itemPath);
      } else {
        stats.total++;
        const ext = path.extname(item);
        stats.byExtension[ext] = (stats.byExtension[ext] || 0) + 1;
      }
    }
  }
  
  walkDir(dir);
  return stats;
}

/**
 * Count lines of code
 */
function countLinesOfCode(dir) {
  const stats = { total: 0, byExtension: {} };
  const excludeDirs = ['node_modules', '.git', 'tmp', 'log', 'public/assets', 'coverage'];
  const textExtensions = ['.rb', '.js', '.jsx', '.ts', '.tsx', '.py', '.erb', '.html', '.scss', '.css', '.rs'];
  
  function walkDir(currentDir) {
    const items = fs.readdirSync(currentDir);
    
    for (const item of items) {
      const itemPath = path.join(currentDir, item);
      const isExcluded = excludeDirs.some(excluded => 
        itemPath.includes(path.sep + excluded + path.sep) || 
        itemPath.endsWith(path.sep + excluded)
      );
      
      if (isExcluded) continue;
      
      const stat = fs.statSync(itemPath);
      
      if (stat.isDirectory()) {
        walkDir(itemPath);
      } else {
        const ext = path.extname(item);
        if (textExtensions.includes(ext)) {
          try {
            const content = fs.readFileSync(itemPath, 'utf8');
            const lines = content.split('\n').length;
            stats.total += lines;
            stats.byExtension[ext] = (stats.byExtension[ext] || 0) + lines;
          } catch (error) {
            // Skip files that can't be read
          }
        }
      }
    }
  }
  
  walkDir(dir);
  return stats;
}

/**
 * Find models in the source code
 */
function findModels(dir, system) {
  const models = [];
  let modelDirs = [];
  
  // System specific model patterns
  if (system === 'canvas') {
    modelDirs = ['app/models'];
  } else if (system === 'discourse') {
    modelDirs = ['app/models'];
  }
  
  for (const modelDir of modelDirs) {
    const fullModelDir = path.join(dir, modelDir);
    if (!fs.existsSync(fullModelDir)) continue;
    
    try {
      const files = fs.readdirSync(fullModelDir);
      
      for (const file of files) {
        if (path.extname(file) === '.rb') {
          const filePath = path.join(fullModelDir, file);
          try {
            const content = fs.readFileSync(filePath, 'utf8');
            // Simple model name extraction
            const modelName = file.replace('.rb', '').replace(/_/g, ' ')
              .replace(/\b\w/g, c => c.toUpperCase()).replace(/\s/g, '');
            
            // Field count estimation - count attr_ usage
            const fieldMatches = content.match(/attr_[a-z]+\s+:([a-z_]+)/g) || [];
            const fieldCount = fieldMatches.length;
            
            // Association estimation
            const associationMatches = content.match(/(?:has_many|has_one|belongs_to)\s+:([a-z_]+)/g) || [];
            const associations = associationMatches.map(match => {
              const parts = match.split(':');
              return parts.length > 1 ? parts[1].trim() : '';
            }).filter(Boolean);
            
            models.push({
              name: modelName,
              file: path.relative(dir, filePath),
              fieldCount,
              associations
            });
          } catch (error) {
            // Skip files that can't be read
          }
        }
      }
    } catch (error) {
      // Skip dirs that can't be read
    }
  }
  
  return models;
}

/**
 * Find controllers in the source code
 */
function findControllers(dir, system) {
  const controllers = [];
  let controllerDirs = [];
  
  // System specific controller patterns
  if (system === 'canvas') {
    controllerDirs = ['app/controllers'];
  } else if (system === 'discourse') {
    controllerDirs = ['app/controllers'];
  }
  
  for (const controllerDir of controllerDirs) {
    const fullControllerDir = path.join(dir, controllerDir);
    if (!fs.existsSync(fullControllerDir)) continue;
    
    try {
      const walkControllerDir = (currentDir) => {
        const files = fs.readdirSync(currentDir);
        
        for (const file of files) {
          const filePath = path.join(currentDir, file);
          const stat = fs.statSync(filePath);
          
          if (stat.isDirectory()) {
            walkControllerDir(filePath);
          } else if (path.extname(file) === '.rb') {
            try {
              const content = fs.readFileSync(filePath, 'utf8');
              
              // Simple controller name extraction
              const controllerName = file.replace('_controller.rb', '').replace(/_/g, ' ')
                .replace(/\b\w/g, c => c.toUpperCase()).replace(/\s/g, '') + 'Controller';
              
              // Action extraction - find def methods
              const actionMatches = content.match(/def\s+([a-z_]+)/g) || [];
              const actions = actionMatches.map(match => {
                return match.replace('def', '').trim();
              });
              
              // Route estimation - simple guess based on controller name and actions
              const basePath = '/' + controllerName.replace('Controller', '').toLowerCase();
              const routes = actions.map(action => {
                if (['index', 'new', 'create', 'show', 'edit', 'update', 'destroy'].includes(action)) {
                  return `${basePath}/${action}`;
                }
                return null;
              }).filter(Boolean);
              
              controllers.push({
                name: controllerName,
                file: path.relative(dir, filePath),
                actions,
                routes
              });
            } catch (error) {
              // Skip files that can't be read
            }
          }
        }
      };
      
      walkControllerDir(fullControllerDir);
    } catch (error) {
      // Skip dirs that can't be read
    }
  }
  
  return controllers;
}

/**
 * Generate a consolidated master report
 */
async function generateConsolidatedReport(summaryDir, lmsDir, canvasDir, discourseDir) {
  const centralHubPath = path.join(lmsDir, 'docs', 'central_reference_hub.md');
  const canvasAnalysisPath = path.join(summaryDir, 'canvas_analysis.md');
  const discourseAnalysisPath = path.join(summaryDir, 'discourse_analysis.md');
  
  let centralHubContent = '';
  let canvasAnalysis = '';
  let discourseAnalysis = '';
  
  // Read central hub if it exists
  if (fs.existsSync(centralHubPath)) {
    centralHubContent = fs.readFileSync(centralHubPath, 'utf8');
  }
  
  // Read canvas analysis if it exists
  if (fs.existsSync(canvasAnalysisPath)) {
    canvasAnalysis = fs.readFileSync(canvasAnalysisPath, 'utf8');
  }
  
  // Read discourse analysis if it exists
  if (fs.existsSync(discourseAnalysisPath)) {
    discourseAnalysis = fs.readFileSync(discourseAnalysisPath, 'utf8');
  }
  
  // Create master report
  let masterReport = `# Full Project Analysis Report\n\n`;
  masterReport += `_Generated on ${new Date().toISOString().split('T')[0]}_\n\n`;
  
  // Overview of all systems
  masterReport += `## Systems Overview\n\n`;
  masterReport += `| System | Directory | Status |\n`;
  masterReport += `|--------|-----------|--------|\n`;
  masterReport += `| Target LMS | ${lmsDir} | ✅ Analyzed |\n`;
  masterReport += `| Canvas Source | ${canvasDir} | ${canvasAnalysis ? '✅ Analyzed' : '❌ Not Found'} |\n`;
  masterReport += `| Discourse Source | ${discourseDir} | ${discourseAnalysis ? '✅ Analyzed' : '❌ Not Found'} |\n\n`;
  
  // Add section pointing to Central Reference Hub
  if (centralHubContent) {
    masterReport += `## 🔍 Central Reference Hub\n\n`;
    masterReport += `The Central Reference Hub contains comprehensive information about the integration project.\n`;
    masterReport += `It is available at: \`${centralHubPath}\`\n\n`;
    
    // Add key metrics from the hub - extract JSON metrics
    const metricsMatch = centralHubContent.match(/```json\n([\s\S]*?)\n```/);
    if (metricsMatch) {
      try {
        const metricsText = metricsMatch[1];
        // Just extract the project_stats section for display
        const statsMatch = metricsText.match(/"project_stats": (\{[^}]+\})/);
        if (statsMatch) {
          masterReport += `### Key Metrics\n\n`;
          masterReport += `\`\`\`json\n${statsMatch[0]}\n\`\`\`\n\n`;
        }
      } catch (error) {
        // Skip metrics extraction if it fails
      }
    }
    
    // Add link to view full hub
    masterReport += `For complete integration details and project status, please refer to the Central Reference Hub.\n\n`;
  }
  
  // Add source system analysis summaries
  if (canvasAnalysis) {
    masterReport += `## 📊 Canvas Source System\n\n`;
    const overviewMatch = canvasAnalysis.match(/## Overview\n\n([\s\S]*?)\n\n/);
    if (overviewMatch) {
      masterReport += overviewMatch[1] + '\n\n';
    }
    masterReport += `[View full Canvas analysis](${path.relative(summaryDir, canvasAnalysisPath)})\n\n`;
  }
  
  if (discourseAnalysis) {
    masterReport += `## 📊 Discourse Source System\n\n`;
    const overviewMatch = discourseAnalysis.match(/## Overview\n\n([\s\S]*?)\n\n/);
    if (overviewMatch) {
      masterReport += overviewMatch[1] + '\n\n';
    }
    masterReport += `[View full Discourse analysis](${path.relative(summaryDir, discourseAnalysisPath)})\n\n`;
  }
  
  // Add completion timeline
  masterReport += `## 📈 Completion Timeline\n\n`;
  // Extract timeline from central hub
  const timelineMatch = centralHubContent.match(/## 📈 Project Trajectories\n\n([\s\S]*?)\n\n/);
  if (timelineMatch) {
    masterReport += timelineMatch[1] + '\n\n';
  } else {
    masterReport += `Timeline information not available. Please refer to the Central Reference Hub.\n\n`;
  }
  
  // Next steps
  masterReport += `## Next Steps\n\n`;
  masterReport += `1. Review the Central Reference Hub for detailed implementation status\n`;
  masterReport += `2. Check source system analyses for legacy code patterns that need migration\n`;
  masterReport += `3. Follow the implementation tasks outlined in the Central Reference Hub\n`;
  masterReport += `4. Run this comprehensive analysis regularly to track progress\n\n`;
  
  // Write master report
  fs.writeFileSync(path.join(summaryDir, 'master_report.md'), masterReport);
}

// Run the full analysis
runFullAnalysis().catch(err => {
  console.error('Error running full analysis:', err);
  process.exit(1);
});