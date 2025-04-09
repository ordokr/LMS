#!/usr/bin/env node

/**
 * This is a wrapper script for orchestrate-analysis.js that allows disabling
 * the Gemini AI analysis completely when using the --no-ai flag.
 */

const path = require('path');
const fs = require('fs');
const { spawnSync } = require('child_process');

// Parse command line arguments
const args = process.argv.slice(2);
const noAiFlag = args.includes('--no-ai');

console.log(`🔍 Starting analysis with AI ${noAiFlag ? 'DISABLED' : 'ENABLED'}...`);

if (noAiFlag) {
  // If --no-ai flag is present, temporarily modify the GeminiAnalyzer
  const geminiAnalyzerPath = path.join(__dirname, 'modules', 'gemini-analyzer.js');
  
  // Check if backup exists
  const backupPath = `${geminiAnalyzerPath}.bak`;
  let originalContent = '';
  
  if (!fs.existsSync(backupPath)) {
    // Create backup if it doesn't exist
    originalContent = fs.readFileSync(geminiAnalyzerPath, 'utf8');
    fs.writeFileSync(backupPath, originalContent, 'utf8');
    console.log('Created backup of gemini-analyzer.js');
  } else {
    // Read from existing backup
    originalContent = fs.readFileSync(backupPath, 'utf8');
  }
  
  // Create a modified version that skips all AI processing
  const modifiedContent = originalContent.replace(
    /async generateCodeInsights\([^)]*\)\s*{[^}]*}/s,
    `async generateCodeInsights() {
      console.log("✅ Gemini AI analysis SKIPPED (--no-ai flag detected)");
      return { path: "skipped", insights: "AI analysis disabled" };
    }`
  );
  
  // Write the modified file
  fs.writeFileSync(geminiAnalyzerPath, modifiedContent, 'utf8');
  console.log('Modified GeminiAnalyzer to skip AI processing');
  
  try {
    // Run the original analysis script
    const result = spawnSync('node', ['orchestrate-analysis.js', ...args], {
      stdio: 'inherit',
      encoding: 'utf8'
    });
    
    if (result.error) {
      console.error('Error running analysis:', result.error);
    }
  } finally {
    // Restore the original file
    fs.writeFileSync(geminiAnalyzerPath, originalContent, 'utf8');
    console.log('Restored original GeminiAnalyzer file');
  }
} else {
  // If no --no-ai flag, just run the original script
  const result = spawnSync('node', ['orchestrate-analysis.js', ...args], {
    stdio: 'inherit',
    encoding: 'utf8'
  });
  
  if (result.error) {
    console.error('Error running analysis:', result.error);
  }
}