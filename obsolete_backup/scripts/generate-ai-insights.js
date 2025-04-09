/**
 * Script to generate AI insights without running full analysis
 */
const UnifiedProjectAnalyzer = require('./unified-project-analyzer');
const FileSystemUtils = require('./fileSystemUtilsRustBridge');
const path = require('path');
const fs = require('fs');
const GeminiAnalyzer = require('./modules/gemini-analyzer');

async function generateAiInsights(options = {}) {
  const baseDir = options.baseDir || process.cwd();
  console.log(`Generating AI insights for ${baseDir}...`);
  
  // Create minimal metrics structure
  const metrics = {
    models: { details: [] },
    apiEndpoints: { details: [] },
    codeQuality: { complexity: { files: [] } }
  };
  
  // Load saved metrics if available
  const metricsPath = path.join(baseDir, '.analysis_cache', 'metrics.json');
  if (fs.existsSync(metricsPath)) {
    try {
      Object.assign(metrics, JSON.parse(fs.readFileSync(metricsPath, 'utf8')));
      console.log('Loaded existing metrics data');
    } catch (err) {
      console.warn('Could not load metrics:', err.message);
    }
  }
  
  // Initialize file system utils
  const fsUtils = new FileSystemUtils(baseDir, [
    /node_modules/, /\.git/, /dist/, /build/
  ]);
  
  // Discover files
  fsUtils.discoverFiles();
  
  // Initialize Gemini analyzer
  const geminiAnalyzer = new GeminiAnalyzer(metrics, {
    geminiApiKey: options.geminiApiKey || "AIzaSyC2bS3qMexvmemLfg-V4ZQ-GbTcg-RgCQ8",
    baseDir: baseDir
  });
  
  // Only read selected files for analysis
  const filesToAnalyze = options.file ? 
    [path.resolve(baseDir, options.file)] :
    fsUtils.getAllFiles().filter(file => {
      const ext = path.extname(file).toLowerCase();
      return ['.js', '.ts', '.jsx', '.tsx'].includes(ext);
    }).slice(0, options.limit || 3); // Limit files to avoid excessive API usage
  
  // Read content only for selected files
  fsUtils.readFileContents(filesToAnalyze);
  
  // Generate insights
  const insights = await geminiAnalyzer.generateCodeInsights(filesToAnalyze, fsUtils);
  
  // Generate report
  const overview = await geminiAnalyzer.generateProjectOverview();
  
  // Create output
  const docsDir = path.join(baseDir, 'docs');
  if (!fs.existsSync(docsDir)) {
    fs.mkdirSync(docsDir, { recursive: true });
  }
  
  // Write insights report
  const reportPath = path.join(docsDir, 'gemini_code_insights.md');
  let content = `# Gemini AI Code Insights\n\n`;
  content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
  
  // Add file insights
  Object.entries(insights).forEach(([file, insight]) => {
    content += `## ${path.basename(file)}\n\n`;
    content += `${insight.summary}\n\n`;
    content += `---\n\n`;
  });
  
  fs.writeFileSync(reportPath, content);
  console.log(`AI insights saved to ${reportPath}`);
  
  return { insights, overview };
}

// Run if called directly
if (require.main === module) {
  const args = process.argv.slice(2);
  const options = {
    baseDir: process.cwd(),
    file: args.find(arg => !arg.startsWith('--')),
    limit: args.includes('--all') ? 999 : 5
  };
  
  generateAiInsights(options).catch(err => {
    console.error('Failed to generate AI insights:', err);
    process.exit(1);
  });
} else {
  module.exports = generateAiInsights;
}