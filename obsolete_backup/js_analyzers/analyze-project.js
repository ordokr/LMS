#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const GeminiAnalyzer = require('./modules/gemini-analyzer');

// Simplified version without requiring google-generative-ai
async function main() {
  console.log("🔍 Starting project analysis...");
  
  // Get base directory
  const baseDir = path.resolve(__dirname);
  
  try {
    // Create a GeminiAnalyzer instance with minimal metrics
    const metrics = {
      models: { total: 28, implemented: [], details: [] },
      apiEndpoints: { total: 42, implemented: [], details: [] },
      uiComponents: { total: 35, implemented: [], details: [] },
      tests: { coverage: 0, passing: 0, total: 0 },
      overallPhase: "planning"
    };
    
    // Initialize the analyzer with metrics and options
    const analyzer = new GeminiAnalyzer(metrics, {
      baseDir: baseDir,
      useCache: true
    });
    
    // Call the generateProjectAnalysis method on the analyzer instance
    const analysis = await analyzer.generateProjectAnalysis(baseDir);
    
    console.log("✅ Analysis complete!");
    console.log(`📊 Results written to ${path.relative(baseDir, analysis.filePath)}`);
    
    // Skip the AI documentation generation entirely
    console.log('⚠️ Skipping AI documentation generation - module not installed');
    
    console.log("✨ Analysis completed successfully!");
    console.log("📝 See LAST_ANALYSIS_RESULTS.md for a summary.");
  } catch (error) {
    console.error("❌ Error during analysis:", error);
    process.exit(1);
  }
}

// Run the main function
main().catch(console.error);