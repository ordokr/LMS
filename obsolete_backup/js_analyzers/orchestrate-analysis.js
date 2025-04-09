#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const UnifiedProjectAnalyzer = require('./unified-project-analyzer');
const TechnicalDocsGenerator = require('./modules/technical-docs-generator');

/**
 * Format a date into a readable string
 * @param {Date|string|number} date - Date to format
 * @returns {string} Formatted date string
 */
function formatDate(date) {
  if (!(date instanceof Date)) {
    date = new Date(date);
  }
  return date.toISOString().split('T')[0] + ' ' + 
         date.toTimeString().split(' ')[0];
}

/**
 * Parse command line arguments
 * @returns {Object} Options object
 */
function parseArgs() {
  const args = process.argv.slice(2);
  const options = {
    useAI: true, // Default to true for backward compatibility    useCache: false,
    verbose: true,
    includeTests: true,
    generateDocs: true
  };

  // Check for --no-ai flag (IMPORTANT: this must set useAI to false)
  if (args.includes('--no-ai')) {
    options.useAI = false;
    console.log("AI analysis disabled via command line flag");
  }
  
  // Check for --use-ai flag (explicitly set)
  if (args.includes('--use-ai')) {
    options.useAI = true;
    console.log("AI analysis explicitly enabled via command line flag");
  }
  
  return options;
}

/**
 * Main analysis function
 */
async function main() {
  console.log("🔍 Starting comprehensive project analysis...");
  
  // Get base directory
  const baseDir = path.resolve(__dirname);
    // Parse command line arguments
  const options = parseArgs();
  
  // Log the actual AI setting to debug
  console.log(`AI analysis setting: ${options.useAI === false ? 'DISABLED' : 'ENABLED'}`);
  
  try {
    // Explicitly create options object with useAI forced to a real boolean
    const analyzerOptions = {
      useCache: options.useCache,
      verbose: options.verbose,
      includeTests: options.includeTests,
      generateDocs: options.generateDocs,
      // Force to false if --no-ai was used
      useAI: Boolean(options.useAI)
    };
    
    // Double check the useAI setting - if --no-ai was provided, ensure it's false
    if (process.argv.includes('--no-ai')) {
      analyzerOptions.useAI = false;
      console.log("Forcing AI analysis OFF due to --no-ai flag");
    }
    
    console.log(`Creating analyzer with options: ${JSON.stringify(analyzerOptions)}`);
    
    // Create analyzer with explicit options
    const analyzer = new UnifiedProjectAnalyzer(baseDir, {}, analyzerOptions);
    
    console.log("📊 Analyzing project structure and codebase...");
    const analysis = await analyzer.analyze();
    
    // Generate technical documentation
    console.log("📝 Generating technical documentation...");
    try {
      const docsGenerator = new TechnicalDocsGenerator({
        baseDir: baseDir,
        outputDir: path.join(baseDir, 'docs'),
        sourcePatterns: [
          'src/**/*.js',
          'src/**/*.ts',
          'services/**/*.js',
          'models/**/*.js',
          'controllers/**/*.js'
        ]
      });
      
      // Check if TechnicalDocsGenerator has a generateDocs method
      if (typeof docsGenerator.generateDocs === 'function') {
        await docsGenerator.generateDocs(analysis);
      } else {
        // Fallback if generateDocs is not available
        console.log("No generateDocs method found in TechnicalDocsGenerator. Using fallback method.");
        await generateDocsFallback(baseDir, analysis);
      }
    } catch (docError) {
      console.warn("⚠️ Warning: Documentation generation failed, but analysis will continue:", docError.message);
    }
    
    // Update the LAST_ANALYSIS_RESULTS.md file
    console.log("💾 Saving analysis results...");
    updateAnalysisResults(analysis);
    
    console.log("✅ Analysis complete! Check LAST_ANALYSIS_RESULTS.md for details.");
  } catch (error) {
    console.error("❌ Error during analysis:", error);
    process.exit(1);
  }
}

/**
 * Update the LAST_ANALYSIS_RESULTS.md file with the latest analysis results
 * @param {Object} analysis - Analysis results object
 */
function updateAnalysisResults(analysis) {
  const resultsPath = path.join(__dirname, 'LAST_ANALYSIS_RESULTS.md');
  
  // Format recent changes properly
  const recentChanges = analysis.recentChanges && Array.isArray(analysis.recentChanges) 
    ? analysis.recentChanges.map(file => `- \`${typeof file === 'object' ? file.path || 'Unknown' : file}\``) 
    : ['No recent changes detected'];
  
  // Limit to show only first 5 changes
  const displayChanges = recentChanges.slice(0, 5);
  const remainingCount = Math.max(0, recentChanges.length - 5);
  
  const content = `# Last Analysis Results

*This file is automatically updated after each analysis run*

## Analysis Summary

**Last Run**: ${formatDate(new Date())}

**Project Status**:
- Models: ${analysis.metrics?.models?.completion || 0}% complete
- API: ${analysis.metrics?.apiEndpoints?.completion || 0}% complete
- UI: ${analysis.metrics?.uiComponents?.completion || 0}% complete
- Tests: ${analysis.metrics?.tests?.coverage || 0}% complete
- Technical Debt: ${analysis.metrics?.technicalDebt || 0}%

**Overall Phase**: ${analysis.metrics?.overallPhase || 'planning'}

## Integration Status

| Component | Status | Completion | Next Steps |
|-----------|--------|------------|------------|
| Model Mapping | ${analysis.components?.modelMapping?.status || 'Not Started'} | ${analysis.components?.modelMapping?.completion || 0}% | ${analysis.components?.modelMapping?.nextSteps || 'N/A'} |
| API Integration | ${analysis.components?.apiIntegration?.status || 'Not Started'} | ${analysis.components?.apiIntegration?.completion || 0}% | ${analysis.components?.apiIntegration?.nextSteps || 'N/A'} |
| Authentication | ${analysis.components?.authentication?.status || 'Not Started'} | ${analysis.components?.authentication?.completion || 0}% | ${analysis.components?.authentication?.nextSteps || 'N/A'} |
| Synchronization | ${analysis.components?.synchronization?.status || 'Not Started'} | ${analysis.components?.synchronization?.completion || 0}% | ${analysis.components?.synchronization?.nextSteps || 'N/A'} |

## Recent Changes

The following files were updated in the last analysis:
${displayChanges.join('\n')}
${remainingCount > 0 ? `\nand ${remainingCount} more files` : ''}

## Next Priorities

${analysis.priorities && Array.isArray(analysis.priorities) 
  ? analysis.priorities.map((priority, index) => `${index + 1}. ${priority}`).join('\n')
  : '1. No priorities identified during analysis'}

## Documentation Updates

The following documentation was updated:
${analysis.updatedDocs && Array.isArray(analysis.updatedDocs)
  ? analysis.updatedDocs.map(doc => `- ${doc.title || 'Untitled'}: [\`${doc.path || 'Unknown'}\`](${doc.path || '#'})`)
  : '- No documentation updates detected'
}`;

  fs.writeFileSync(resultsPath, content, 'utf8');
}

/**
 * Fallback function for generating documentation when TechnicalDocsGenerator doesn't have generateDocs
 * @param {string} baseDir - Project base directory
 * @param {Object} analysis - Analysis results
 */
async function generateDocsFallback(baseDir, analysis) {
  const docsDir = path.join(baseDir, 'docs');
  if (!fs.existsSync(docsDir)) {
    fs.mkdirSync(docsDir, { recursive: true });
  }
  
  // Create a basic analysis summary document
  const analysisDoc = path.join(docsDir, 'analysis_summary.md');
  
  const content = `# Project Analysis Summary
  
## Overview

This documentation was generated automatically from the latest project analysis.

## Component Status

- Models: ${analysis.metrics?.models?.completion || 0}% complete
- API Endpoints: ${analysis.metrics?.apiEndpoints?.completion || 0}% complete
- UI Components: ${analysis.metrics?.uiComponents?.completion || 0}% complete
- Test Coverage: ${analysis.metrics?.tests?.coverage || 0}%

## Implementation Details

${analysis.components ? Object.entries(analysis.components).map(([name, component]) => 
`### ${name}
- Status: ${component.status || 'Not started'}
- Completion: ${component.completion || 0}%
- Next Steps: ${component.nextSteps || 'N/A'}
`).join('\n') : 'No component details available.'}

## Technical Architecture

${analysis.architecture || 'Technical architecture documentation not available.'}

## Integration Points

${analysis.integrationPoints ? analysis.integrationPoints.map(point => 
`- ${point.name || 'Unnamed'}: ${point.status || 'Unknown status'}`
).join('\n') : 'No integration points documented.'}
`;

  fs.writeFileSync(analysisDoc, content, 'utf8');
  console.log(`Generated fallback documentation at ${analysisDoc}`);
  
  return { path: analysisDoc, title: 'Analysis Summary' };
}

// Run the main function
main().catch(console.error);
