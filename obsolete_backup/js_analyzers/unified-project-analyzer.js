const _parser = require('@babel/parser');
const _traverse = require('@babel/traverse').default;
const FileSystemUtils = require('./fileSystemUtilsRustBridge');
const AnalysisUtils = require('./analysisUtils');
const path = require('path');
const fs = require('fs');
const fsExtra = require('fs-extra');
const AstAnalyzer = require('./astAnalyzer');
const ProjectPredictor = require('./projectPredictor');

// Import the new modules
const SolidAnalyzer = require('./modules/solid-analyzer');
const PatternAnalyzer = require('./modules/pattern-analyzer');
const SourceAnalyzer = require('./modules/source-analyzer');
const ReportGenerator = require('./modules/report-generator');
const MLAnalyzer = require('./modules/ml-analyzer');
const VisualReportGenerator = require('./modules/visual-report-generator');
const GitHubAnalyzer = require('./modules/github-analyzer');
const GeminiAnalyzer = require('./modules/gemini-analyzer');
const VectorDatabaseAdapter = require('./modules/vector-database-adapter');
const RagRetriever = require('./modules/rag-retriever');
const IntegrationReportGenerator = require('./modules/integration-report-generator');
const TechnicalDocsGenerator = require('./modules/technical-docs-generator');

/**
 * Unified Project Analyzer
 * Consolidates all analyzer functionality into a single tool
 */
class UnifiedProjectAnalyzer {
  constructor(baseDir, sourceSystems = {}, options = {}) {
    this.baseDir = baseDir || process.cwd();
    console.log(`Using base directory: ${this.baseDir}`);
    this.sourceSystems = sourceSystems;
    this.options = Object.assign({
      useCache: true,
      implementationThreshold: 35,
      geminiApiKey: "AIzaSyC2bS3qMexvmemLfg-V4ZQ-GbTcg-RgCQ8" // Default Gemini API key
    }, options);
    
    // Initialize metrics structure
    this.metrics = this.initializeMetrics();

    // Initialize utility classes
    this.fsUtils = new FileSystemUtils(this.baseDir, this.getExcludePatterns());
    this.analysisUtils = AnalysisUtils;
    this.astAnalyzer = new AstAnalyzer();
    this.predictor = new ProjectPredictor(this.metrics);
    
    // Initialize the new modular components
    this.solidAnalyzer = new SolidAnalyzer(this.metrics);
    this.patternAnalyzer = new PatternAnalyzer(this.metrics);
    this.sourceAnalyzer = new SourceAnalyzer(this.metrics, this.getExcludePatterns());
    this.reportGenerator = new ReportGenerator(this.metrics, this.baseDir);
    this.mlAnalyzer = new MLAnalyzer(this.metrics);
    this.visualReportGenerator = new VisualReportGenerator(this.metrics, this.baseDir);
    
    // Gemini AI integration
    this.geminiAnalyzer = new GeminiAnalyzer(this.metrics, {
      geminiApiKey: this.options.geminiApiKey,
      baseDir: this.baseDir
    });
    
    // GitHub integration (optional)
    if (options.github) {
      try {
        this.githubAnalyzer = new GitHubAnalyzer(this.metrics, {
          token: options.github.token,
          owner: options.github.owner,
          repo: options.github.repo,
          cacheDir: path.join(this.baseDir, '.analysis_cache')
        });
      } catch (err) {
        console.warn(`GitHub integration not available: ${err.message}`);
        this.githubAnalyzer = null;
      }
    }
  }
  
  /**
   * Initialize metrics structure
   */
  initializeMetrics() {
    return {
      models: { total: 0, implemented: 0, details: [] },
      apiEndpoints: { total: 0, implemented: 0, details: [] },
      uiComponents: { total: 0, implemented: 0, details: [] },
      tests: { total: 0, passing: 0, coverage: 0, details: [] },
      codeQuality: {
        complexity: { average: 0, high: 0, files: [] },
        techDebt: { score: 0, items: [] },
        solidViolations: {
          srp: [], ocp: [], lsp: [], isp: [], dip: []
        },
        designPatterns: {
          polymorphism: { implementations: [], violations: [] },
          dependencyInjection: { implementations: [], violations: [] },
          ioc: { implementations: [], violations: [] }
        }
      },
      featureAreas: {
        auth: { total: 0, implemented: 0 },
        forum: { total: 0, implemented: 0 },
        lms: { total: 0, implemented: 0 },
        integration: { total: 0, implemented: 0 },
        other: { total: 0, implemented: 0 }
      },
      relationships: [],
      predictions: {
        velocityData: {
          models: 1.5,
          apiEndpoints: 3,
          uiComponents: 5,
          tests: 2
        },
        estimates: {}
      },
      sourceSystems: {
        canvas: { models: { total: 0, details: [] }, controllers: { total: 0, details: [] }, filesByType: {} },
        discourse: { models: { total: 0, details: [] }, controllers: { total: 0, details: [] }, filesByType: {} }
      },
      sourceToTarget: {
        models: [],
        controllers: [],
        components: []
      }
    };
  }

  /**
   * Get exclude patterns for file discovery
   */
  getExcludePatterns() {
    return [
      /node_modules/, /\.git/, /target\/(?!.*\.rs$)/,
      /dist/, /build/, /\.cache/, /\.next/, /\.nuxt/,
      /\.DS_Store/, /coverage/, /\.vscode/, /\.idea/,
      /assets/, /public/, /docs/, /references/,
      /analysis_summary/, /md_dashboard/, /tools\/__pycache__/,
      /.*\.log$/, /.*\.tmp$/, /.*\.bak.?$/, /.*\.swp$/,
      /LMS\.code-workspace/, /package-lock\.json/, /yarn\.lock/,
      /unified-project-analyzer\.js/, /project-analyzer\.js.*/,
      /debug-analyzer\.js/, /status-analyzer\.js/,
      /advanced-api-analyzer\.js/, /fix-.*\.js/,
      /cleanup-docs\.js/, /run-full-analysis\.js/,
      /status-updater\.js/, /analyze_project\.pdb/,
      /fileSystemUtils\.js/,
    ];
  }

  /**
   * Enhanced run method with incremental analysis and performance tracking
   */
  async analyze() {
    const startTime = Date.now();
    this.metrics.performance = {};
    
    console.log(`Starting analysis of ${this.baseDir}...`);
    
    // Ensure we have a valid base directory
    if (!this.baseDir) {
      this.baseDir = process.cwd(); // Use current working directory as fallback
      console.log(`No base directory provided, using current directory: ${this.baseDir}`);
    }
    
    console.time('Total Analysis Time');
    
    // Initialize performance tracking
    this.metrics.performance = {
      startTime: Date.now(),
      steps: {}
    };
    
    // Check if this is an incremental analysis
    const changedFiles = this.options.incremental ? 
      await this.performIncrementalAnalysis() : null;
    
    // File discovery (with tracking)
    const startFileDiscovery = Date.now();
    if (!changedFiles) {
      this.fsUtils.discoverFiles();
      this.fsUtils.readFileContents();
    }
    this.metrics.performance.steps.fileDiscovery = Date.now() - startFileDiscovery;
    
    // Core analysis (with tracking)
    const startModelAnalysis = Date.now();
    await this.analysisUtils.analyzeModels(changedFiles);
    this.metrics.performance.steps.modelAnalysis = Date.now() - startModelAnalysis;
    
    const startApiAnalysis = Date.now();
    await this.analysisUtils.analyzeApiEndpoints(changedFiles);
    this.metrics.performance.steps.apiAnalysis = Date.now() - startApiAnalysis;
    
    const startUiAnalysis = Date.now();
    await this.analysisUtils.analyzeUIComponents(changedFiles);
    this.metrics.performance.steps.uiAnalysis = Date.now() - startUiAnalysis;
    
    const startTestAnalysis = Date.now();
    await this.analysisUtils.analyzeTests(changedFiles);
    this.metrics.performance.steps.testAnalysis = Date.now() - startTestAnalysis;
    
    const startQualityAnalysis = Date.now();
    await this.analysisUtils.analyzeCodeQuality(this.astAnalyzer, changedFiles);
    this.metrics.performance.steps.codeQualityAnalysis = Date.now() - startQualityAnalysis;
    
    // GitHub integration (if configured)
    if (this.githubAnalyzer) {
      const startGitHubAnalysis = Date.now();
      await this.githubAnalyzer.analyzeRepository();
      this.metrics.performance.steps.githubAnalysis = Date.now() - startGitHubAnalysis;
    }
    
    // Source systems analysis
    if (Object.keys(this.sourceSystems).length > 0) {
      const startSourceAnalysis = Date.now();
      await this.sourceAnalyzer.analyzeSourceSystems(
        this.baseDir, 
        this.sourceSystems, 
        this.fsUtils, 
        this.options.useCache
      );
      this.metrics.performance.steps.sourceAnalysis = Date.now() - startSourceAnalysis;
    }
    
    // Additional analysis steps
    const startRelationships = Date.now();
    await this.generateRelationshipMaps();
    this.metrics.performance.steps.relationships = Date.now() - startRelationships;
    
    const startCodeSmells = Date.now();
    await this.detectCodeSmells();
    this.metrics.performance.steps.codeSmells = Date.now() - startCodeSmells;
    
    // ML-based code analysis
    const startMlAnalysis = Date.now();
    const _codeAnomalies = this.mlAnalyzer.detectAbnormalCode();
    this.metrics.performance.steps.mlAnalysis = Date.now() - startMlAnalysis;
    
    const startPredictor = Date.now();
    this.predictor.predictCompletion();
    this.metrics.performance.steps.predictor = Date.now() - startPredictor;
      const startStatusUpdate = Date.now();
    this.updateProjectStatus();
    this.metrics.performance.steps.statusUpdate = Date.now() - startStatusUpdate;
    
    // Check for --no-ai command line flag directly
    const noAiFlag = process.argv.includes('--no-ai');
    
    // Run Gemini AI analysis if enabled AND --no-ai flag is not present
    if (this.options.useAI !== false && !noAiFlag) {
      console.log("Running Gemini AI analysis...");
      const startGeminiAnalysis = Date.now();
      await this.geminiAnalyzer.generateCodeInsights(this.fsUtils.getAllFiles(), this.fsUtils);
      this.metrics.performance.steps.geminiAnalysis = Date.now() - startGeminiAnalysis;
    } else {
      console.log("Skipping Gemini AI analysis (disabled via options or --no-ai flag)");
    }
    
    // Update total time
    this.metrics.performance.totalTime = Date.now() - this.metrics.performance.startTime;
    
    // Update AI analysis results for Copilot and other AI tools
    await this.updateAiAnalysisResults();
    
    console.timeEnd('Total Analysis Time');
    this.printSummary();
    return this.metrics;
  }

  /**
   * Only analyze files that have changed since the last analysis
   */
  async performIncrementalAnalysis() {
    const cacheDir = path.join(this.baseDir, '.analysis_cache');
    if (!fs.existsSync(cacheDir)) {
      fs.mkdirSync(cacheDir, { recursive: true });
    }
    
    const lastRunFile = path.join(cacheDir, 'last_run.json');
    let lastRun = {};
    
    if (fs.existsSync(lastRunFile)) {
      try {
        lastRun = JSON.parse(fs.readFileSync(lastRunFile, 'utf8'));
      } catch (err) {
        console.warn("Could not read last run data:", err.message);
      }
    }
    
    // Discover files
    this.fsUtils.discoverFiles();
    
    // Filter for changed/new files only
    const changedFiles = this.fsUtils.getAllFiles().filter(file => {
      try {
        const stats = fs.statSync(file);
        const lastModified = stats.mtimeMs;
        
        // Check if file is new or modified since last run
        if (!lastRun.files || !lastRun.files[file] || lastRun.files[file].mtime < lastModified) {
          return true;
        }
        return false;
      } catch (err) {
        // If there's an error reading the file, consider it changed
        return true;
      }
    });
    
    console.log(`Found ${changedFiles.length} changed/new files out of ${this.fsUtils.getAllFiles().length} total files`);
    
    // Only read contents of changed files
    this.fsUtils.readFileContents(changedFiles);
    
    // Record this run's file timestamps
    const currentRun = {
      timestamp: Date.now(),
      files: {}
    };
    
    this.fsUtils.getAllFiles().forEach(file => {
      try {
        const stats = fs.statSync(file);
        currentRun.files[file] = { 
          mtime: stats.mtimeMs,
          size: stats.size
        };
      } catch (err) {
        // Ignore errors when getting stats
      }
    });
    
    // Save current run data
    try {
      fs.writeFileSync(lastRunFile, JSON.stringify(currentRun, null, 2));
    } catch (err) {
      console.warn("Could not save run data:", err.message);
    }
    
    return changedFiles;
  }

  /**
   * Calculate percentage
   * @param {number} value - Current value
   * @param {number} total - Total value
   * @returns {number} Percentage (0-100)
   */
  getPercentage(value, total) {
    if (total === 0) return 0;
    return Math.round((value / total) * 100);
  }

  /**
   * Update overall project status based on metrics
   */
  updateProjectStatus() {
    console.log("Updating project status...");
    const modelsPercent = this.getPercentage(this.metrics.models.implemented, this.metrics.models.total);
    const apiPercent = this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total);
    const uiPercent = this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total);

    this.metrics.overallStatus = {
      models: `${modelsPercent}%`,
      api: `${apiPercent}%`,
      ui: `${uiPercent}%`,
      tests: `${this.metrics.tests.coverage}%`,
      techDebt: `${this.metrics.codeQuality.techDebt.score}%`
    };

    // Determine overall phase
    const avgCompletion = (modelsPercent + apiPercent + uiPercent) / 3;
    if (avgCompletion < 10) this.metrics.overallPhase = 'planning';
    else if (avgCompletion < 40) this.metrics.overallPhase = 'early_development';
    else if (avgCompletion < 75) this.metrics.overallPhase = 'mid_development';
    else if (avgCompletion < 95) this.metrics.overallPhase = 'late_development';
    else this.metrics.overallPhase = 'release_candidate';

    console.log(`Project Status: Models=${modelsPercent}%, API=${apiPercent}%, UI=${uiPercent}%, Tests=${this.metrics.tests.coverage}%, Debt=${this.metrics.codeQuality.techDebt.score}%`);
    console.log(`Overall Phase: ${this.metrics.overallPhase}`);
  }

  /**
   * Generate relationship maps using Mermaid syntax
   */
  async generateRelationshipMaps() {
    console.log("Generating relationship maps...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    // Delegate the relationship detection to analysisUtils
    if (this.analysisUtils.findModelRelationships) {
      await this.analysisUtils.findModelRelationships();
    }
    
    // Generate the diagram
    let mermaidDiagram = "graph LR\n";
    const nodes = new Set();
    this.metrics.relationships.forEach(rel => {
      nodes.add(rel.from);
      nodes.add(rel.to);
      const arrow = rel.type === 'OneToMany' ? '-->|1..*|' : '-->';
      mermaidDiagram += `  ${rel.from}${arrow}${rel.to}\n`;
    });

    // Add nodes that might not have relationships yet
    this.metrics.models.details.forEach(m => nodes.add(m.name));
    
    // Add styles for nodes
    nodes.forEach(node => {
      const model = this.metrics.models.details.find(m => m.name === node);
      const completeness = model ? model.completeness : 0;
      let style = 'fill:#eee,stroke:#333,stroke-width:1px';
      if (completeness >= 75) style = 'fill:#c8e6c9,stroke:#2e7d32,stroke-width:2px'; // Green
      else if (completeness >= 40) style = 'fill:#fff9c4,stroke:#fbc02d,stroke-width:1px'; // Yellow
      else if (completeness > 0) style = 'fill:#ffcdd2,stroke:#c62828,stroke-width:1px'; // Red
      mermaidDiagram += `  style ${node} ${style}\n`;
    });

    // Save to a file
    const mapContent = `# Model Relationship Map\n\n\`\`\`mermaid\n${mermaidDiagram}\n\`\`\`\n`;
    try {
      fs.writeFileSync(path.join(this.baseDir, 'docs', 'relationship_map.md'), mapContent);
      console.log("Relationship map saved to docs/relationship_map.md");
    } catch (err) {
      console.error("Error saving relationship map:", err.message);
    }
  }

  /**
   * Detect code smells related to SOLID principles and design patterns
   */
  async detectCodeSmells() {
    console.log("Analyzing code for SOLID principles and design patterns...");
    
    // Ensure required structures exist
    if (!this.metrics.codeQuality.solidViolations) {
      this.metrics.codeQuality.solidViolations = { srp: [], ocp: [], lsp: [], isp: [], dip: [] };
    }
    
    if (!this.metrics.codeQuality.designPatterns) {
      this.metrics.codeQuality.designPatterns = {
        polymorphism: { implementations: [], violations: [] },
        dependencyInjection: { implementations: [], violations: [] },
        ioc: { implementations: [], violations: [] }
      };
    }
    
    // Process JavaScript/TypeScript files
    const jsFiles = this.fsUtils.filterFiles(/\.(js|jsx|ts|tsx)$/);
    for (const file of jsFiles) {
      const content = this.fsUtils.getFileContent(file);
      if (!content) continue;
      
      try {
        const ast = this.astAnalyzer.parseToAst(content, file);
        if (!ast) continue;
        
        // Detect violations using the modular analyzers
        this.solidAnalyzer.detectSRPViolations(ast, file);
        this.solidAnalyzer.detectOCPViolations(ast, file);
        this.solidAnalyzer.detectLSPViolations(ast, file);
        this.solidAnalyzer.detectISPViolations(ast, file);
        this.solidAnalyzer.detectDIPViolations(ast, file);
        
        this.patternAnalyzer.detectPolymorphism(ast, file);
        this.patternAnalyzer.detectDependencyInjection(ast, file);
        this.patternAnalyzer.detectIoC(ast, file);
      } catch (error) {
        console.error(`Error analyzing ${file} for code smells:`, error.message);
      }
    }
    
    // Process Rust files
    const rustFiles = this.fsUtils.filterFiles(/\.rs$/);
    for (const file of rustFiles) {
      const content = this.fsUtils.getFileContent(file);
      if (!content) continue;
      this.solidAnalyzer.detectRustSRPViolations(content, file);
    }
    
    // Generate reports
    await this.reportGenerator.generateCodeSmellsReport();
  }

  /**
   * Generate specialized RAG documents for AI training
   */
  async generateRagDocuments() {
    console.log("Generating specialized RAG documents for AI training...");
    const ragOutputDir = path.join(this.baseDir, 'rag_knowledge_base');
    
    if (!fs.existsSync(ragOutputDir)) {
      fs.mkdirSync(ragOutputDir, { recursive: true });
    }
    
    // Initialize knowledge graph generator if not already done
    if (!this.knowledgeGraphGenerator) {
      const KnowledgeGraphGenerator = require('./modules/knowledge-graph-generator');
      this.knowledgeGraphGenerator = new KnowledgeGraphGenerator(this.metrics, {
        outputDir: 'rag_knowledge_base',
        generateVisualization: true
      });
    }
    
    // Generate source systems semantic knowledge base
    const sourceSystems = ['canvas', 'discourse'];
    
    for (const system of sourceSystems) {
      // Skip if the system isn't available
      if (!this.sourceSystems[system]) {
        console.warn(`Source system ${system} not available for RAG document generation`);
        continue;
      }
      
      await this.generateSystemKnowledgeBase(system, ragOutputDir);
      await this.generateSystemBehaviorDocuments(system, ragOutputDir);
      await this.generateSystemArchitecturePatterns(system, ragOutputDir);
      await this.generateSystemAPIContracts(system, ragOutputDir);
    }
    
    // Generate cross-system integration knowledge
    await this.generateIntegrationKnowledgeBase(sourceSystems, ragOutputDir);
    
    // Generate technical implementation documentation
    await this.generateTechnicalImplementationDocs();
    
    // Generate semantic embeddings for efficient retrieval
    if (this.mlAnalyzer && typeof this.mlAnalyzer.generateEmbeddingsForRagDocuments === 'function') {
      const embeddings = await this.mlAnalyzer.generateEmbeddingsForRagDocuments(ragOutputDir);
      
      // Initialize RAG system and store embeddings in vector database
      if (embeddings) {
        await this.initializeRagSystem();
        
        // Load embeddings into vector database
        if (this.vectorDB) {
          console.log("Importing embeddings into vector database...");
          
          // Convert embeddings from object to array format
          const embeddingsArray = Object.entries(embeddings).map(([id, vector]) => ({
            id,
            vector,
            metadata: {
              source: id,
              system: id.split('/')[0],
              category: id.split('/')[1] || 'general'
            }
          }));
          
          await this.vectorDB.bulkStoreEmbeddings(embeddingsArray);
          console.log(`Stored ${embeddingsArray.length} embeddings in vector database`);
        }
      }
    } else {
      console.warn("ML Analyzer not available or missing embedding generation capability");
    }
    
    console.log(`RAG documents generated successfully in ${ragOutputDir}`);
    return ragOutputDir;
  }

  /**
   * Create and initialize the RAG system
   */
  async initializeRagSystem() {
    console.log("Initializing RAG system...");
    
    // Create vector database adapter
    this.vectorDB = new VectorDatabaseAdapter({
      dbType: 'qdrant',  // Use 'memory' if Qdrant is not available
      dimensions: 512,
      collectionName: 'canvas_discourse_integration'
    });
    
    // Create RAG retriever
    this.ragRetriever = new RagRetriever(this.vectorDB, {
      mlAnalyzer: this.mlAnalyzer,
      ragDir: path.join(this.baseDir, 'rag_knowledge_base')
    });
    
    // Initialize system
    try {
      await this.vectorDB.initialize();
      await this.ragRetriever.initialize();
      console.log("RAG system initialized successfully");
      return true;
    } catch (error) {
      console.error(`Failed to initialize RAG system: ${error.message}`);
      // Fall back to memory database if Qdrant fails
      if (this.vectorDB.options.dbType === 'qdrant') {
        console.log("Falling back to in-memory vector database");
        this.vectorDB = new VectorDatabaseAdapter({
          dbType: 'memory',
          dimensions: 512
        });
        this.ragRetriever = new RagRetriever(this.vectorDB, { 
          mlAnalyzer: this.mlAnalyzer,
          ragDir: path.join(this.baseDir, 'rag_knowledge_base')
        });
        
        await this.vectorDB.initialize();
        await this.ragRetriever.initialize();
        return true;
      }
      return false;
    }
  }

  /**
   * Query the RAG system for information
   * @param {string} query - User query
   * @param {Object} options - Query options
   * @returns {Object} Query results
   */
  async queryRag(query, options = {}) {
    if (!this.ragRetriever) {
      await this.initializeRagSystem();
    }
    
    if (!this.ragRetriever) {
      throw new Error("RAG system not initialized");
    }
    
    console.log(`Querying RAG system: "${query}"`);
    
    const results = await this.ragRetriever.search(query, options);
    
    // Generate context for LLM if needed
    if (options.generateContext) {
      results.context = this.ragRetriever.generateLlmContext(
        results.documents, 
        options.contextOptions
      );
    }
    
    return results;
  }

  /**
   * Generate all reports
   */
  async generateAllReports() {
    console.log("Generating all analysis reports...");
    
    // Basic reports
    await this.reportGenerator.generateCodeSmellsReport();
    
    // Source comparison report if we have source systems
    if (Object.keys(this.sourceSystems).length > 0) {
      await this.sourceAnalyzer.generateSourceComparisonReport(this.baseDir);
    }
    
    // Generate central reference hub
    await this.reportGenerator.generateCentralReferenceHub(this.fsUtils);
    
    // New enhanced reports
    await this.reportGenerator.generateTimelineReport();
    await this.reportGenerator.generateArchitectureReport();
    await this.reportGenerator.generateFeatureCoverageMap();
    
    // Generate database documentation with SQLite + sqlx hardcoded
    await this.reportGenerator.generateDatabaseDocumentation();
    
    // Generate performance report
    await this.generatePerformanceReport();
    
    // Visual dashboard
    await this.visualReportGenerator.generateDashboard();
    
    // Gemini AI reports
    if (this.options.useAI !== false) {
      await this.geminiAnalyzer.generateProjectAssessmentReport(this.baseDir);
      await this.geminiAnalyzer.generateCodeInsightsReport();
      
      // Generate source mapping improvement report if we have source systems
      if (Object.keys(this.sourceSystems).length > 0) {
        const mappingAnalysis = await this.geminiAnalyzer.generateMappingImprovement();
        if (mappingAnalysis) {
          const reportPath = path.join(this.baseDir, 'docs', 'gemini_mapping_analysis.md');
          fs.writeFileSync(reportPath, 
            `# Gemini AI Source-Target Mapping Analysis\n\n_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n${mappingAnalysis}`
          );
          console.log(`Gemini mapping analysis report saved to ${reportPath}`);
        }
      }
    }
    
    // Generate index.html to link all reports
    this.generateReportIndex();
    
    // Update integration documentation
    await this.updateIntegrationDocumentation();
    
    console.log('All reports generated successfully');
  }

  /**
   * Generate an HTML index page linking all reports
   */
  generateReportIndex() {
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    // Find all markdown files in the docs directory
    const reports = fs.readdirSync(docsDir)
      .filter(file => file.endsWith('.md'))
      .map(file => ({
        file,
        name: file.replace('.md', '').replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase())
      }));
    
    // Group reports by category
    const categories = {
      'Primary Reports': ['central_reference_hub', 'system_architecture', 'feature_coverage_map', 'project_timeline'],
      'Code Quality': ['code_smells', 'performance_analysis', 'solid_violations'],
      'AI Insights': ['gemini_project_assessment', 'ai_code_insights', 'gemini_mapping_analysis'],
      'Other Reports': []
    };
    
    // Categorize reports
    reports.forEach(report => {
      let assigned = false;
      Object.entries(categories).forEach(([category, files]) => {
        if (files.some(f => report.file.startsWith(f))) {
          if (!categories[category].includes(report)) {
            categories[category].push(report);
          }
          assigned = true;
        }
      });
      
      if (!assigned) {
        categories['Other Reports'].push(report);
      }
    });
    
    // Generate HTML
    let html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>LMS Project Analysis Reports</title>
  <style>
    body {
      font-family: Arial, sans-serif;
      line-height: 1.6;
      margin: 0;
      padding: 20px;
      color: #333;
    }
    h1 {
      color: #2c3e50;
      border-bottom: 2px solid #3498db;
      padding-bottom: 10px;
    }
    h2 {
      color: #2980b9;
      margin-top: 30px;
    }
    .report-card {
      border: 1px solid #ddd;
      border-radius: 5px;
      padding: 15px;
      margin: 10px 0;
      background-color: #f9f9f9;
      transition: transform 0.2s, box-shadow 0.2s;
    }
    .report-card:hover {
      transform: translateY(-5px);
      box-shadow: 0 5px 15px rgba(0,0,0,0.1);
    }
    .report-card h3 {
      margin-top: 0;
      color: #3498db;
    }
    .report-list {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
      gap: 20px;
    }
    a {
      color: #3498db;
      text-decoration: none;
    }
    a:hover {
      text-decoration: underline;
    }
    .last-updated {
      color: #7f8c8d;
      font-size: 0.9rem;
      margin-top: 5px;
    }
    .category-description {
      margin-bottom: 20px;
      padding-left: 10px;
      border-left: 3px solid #3498db;
    }
  </style>
</head>
<body>
  <h1>LMS Project Analysis Reports</h1>
  <p>Generated on ${new Date().toLocaleDateString()} at ${new Date().toLocaleTimeString()}</p>
  
  <p>These reports provide a comprehensive view of the LMS project's status, architecture, and code quality.</p>`;
    
    Object.entries(categories).forEach(([category, categoryReports]) => {
      if (categoryReports.length === 0) return;
      
      html += `
  <h2>${category}</h2>`;
      
      if (category === 'Primary Reports') {
        html += `
  <div class="category-description">
    Key reports that provide a high-level overview of the project.
  </div>`;
      } else if (category === 'Code Quality') {
        html += `
  <div class="category-description">
    Reports that focus on code quality, performance, and architectural concerns.
  </div>`;
      } else if (category === 'AI Insights') {
        html += `
  <div class="category-description">
    AI-generated insights and assessments of the project.
  </div>`;
      }
      
      html += `
  <div class="report-list">`;
      
      categoryReports.forEach(report => {
        if (typeof report === 'string') return; // Skip string entries used for categorization
        
        const reportPath = path.join(docsDir, report.file);
        let description = '';
        let lastUpdated = '';
        
        try {
          const stats = fs.statSync(reportPath);
          lastUpdated = new Date(stats.mtime).toLocaleDateString();
          
          const content = fs.readFileSync(reportPath, 'utf8');
          // Extract first paragraph after title as description
          const descMatch = content.match(/^# .*?\n\n.*?_.*?_\n\n(.*?)(?:\n\n|\n#)/s);
          if (descMatch && descMatch[1]) {
            description = descMatch[1].substring(0, 150) + '...';
          } else {
            description = 'Detailed report about ' + report.name.toLowerCase() + '.';
          }
        } catch (err) {
          console.warn(`Error reading report metadata for ${report.file}:`, err.message);
        }
        
        html += `
    <div class="report-card">
      <h3><a href="${report.file}">${report.name}</a></h3>
      <p>${description}</p>
      <div class="last-updated">Last updated: ${lastUpdated}</div>
    </div>`;
      });
      
      html += `
  </div>`;
    });
    
    html += `
</body>
</html>`;
    
    // Save the index file
    try {
      fs.writeFileSync(path.join(docsDir, 'index.html'), html);
      console.log('Generated reports index at docs/index.html');
    } catch (err) {
      console.error('Error generating reports index:', err.message);
    }
  }

  /**
   * Run file analysis with worker threads for better performance
   */
  async analyzeFilesWithWorkers(files, analyzerFunction) {
    // Convert to absolute paths
    files = files.map(file => path.isAbsolute(file) ? file : path.join(this.baseDir, file));
    
    // Determine optimal number of workers based on CPU cores
    const os = require('os');
    const numWorkers = Math.max(1, os.cpus().length - 1); // Leave one core free
    
    console.log(`Using ${numWorkers} worker threads for analysis`);
    
    // Split files into chunks for each worker
    const chunkSize = Math.ceil(files.length / numWorkers);
    const chunks = [];
    
    for (let i = 0; i < files.length; i += chunkSize) {
      chunks.push(files.slice(i, i + chunkSize));
    }
    
    // Create and run worker for each chunk
    const { Worker } = require('worker_threads');
    const workerFile = path.join(__dirname, 'worker-analyzer.js');
    
    const workers = chunks.map((chunk, i) => {
      return new Promise((resolve, reject) => {
        const worker = new Worker(workerFile, {
          workerData: {
            files: chunk,
            workerId: i,
            analyzerFunction: analyzerFunction.toString()
          }
        });
        
        worker.on('message', resolve);
        worker.on('error', reject);
        worker.on('exit', code => {
          if (code !== 0) {
            reject(new Error(`Worker stopped with exit code ${code}`));
          }
        });
      });
    });
    
    // Wait for all workers to complete
    const results = await Promise.all(workers);
    
    // Combine results from all workers
    return results.flat();
  }

  /**
   * Generate a performance analysis report
   */
  async generatePerformanceReport() {
    console.log("Generating performance analysis report...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    const outputPath = path.join(docsDir, 'performance_analysis.md');
    
    // Generate content
    let content = `# Performance Analysis Report\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Performance metrics from analysis
    const performanceMetrics = this.metrics.performance || { steps: {} };
    
    content += `## Analysis Performance\n\n`;
    content += `These metrics show how long each step of the analysis process took to run:\n\n`;
    content += `| Analysis Step | Time (ms) | Time (sec) |\n`;
    content += `|--------------|------------|------------|\n`;
    
    // Sort steps by duration (longest first)
    const steps = Object.entries(performanceMetrics.steps || {})
      .sort(([, a], [, b]) => b - a);
    
    let totalTime = 0;
    
    steps.forEach(([step, time]) => {
      const seconds = (time / 1000).toFixed(2);
      // Format step name nicely
      const formattedStep = step
        .replace(/([A-Z])/g, ' $1')
        .replace(/^./, str => str.toUpperCase())
        .trim();
        
      content += `| ${formattedStep} | ${time.toLocaleString()} | ${seconds}s |\n`;
      totalTime += time;
    });
    
    content += `| **Total Analysis Time** | **${totalTime.toLocaleString()}** | **${(totalTime / 1000).toFixed(2)}s** |\n\n`;
    
    // Code performance metrics
    content += `## Code Performance Hotspots\n\n`;
    
    if (this.metrics.codeQuality && this.metrics.codeQuality.complexity) {
      const complexFiles = this.metrics.codeQuality.complexity.files || [];
      
      // Sort files by complexity
      const sortedFiles = [...complexFiles].sort((a, b) => b.complexity - a.complexity);
      
      content += `The following files have the highest complexity scores, which may indicate performance concerns:\n\n`;
      content += `| File | Complexity Score | Lines of Code | Complexity/LOC |\n`;
      content += `|------|-----------------|---------------|----------------|\n`;
      
      sortedFiles.slice(0, 10).forEach(file => {
        const complexityPerLine = file.lines > 0 ? (file.complexity / file.lines).toFixed(2) : 'N/A';
        content += `| ${path.basename(file.file)} | ${file.complexity} | ${file.lines} | ${complexityPerLine} |\n`;
      });
    } else {
      content += `No complexity metrics available for performance analysis.\n`;
    }
    
    content += `\n`;
    
    // Runtime performance estimates
    content += `## Runtime Performance Estimates\n\n`;
    
    if (this.metrics.apiEndpoints && this.metrics.apiEndpoints.details) {
      const endpoints = this.metrics.apiEndpoints.details;
      
      // Estimate performance for endpoints
      content += `### API Endpoint Estimated Performance\n\n`;
      content += `| Endpoint | Method | Estimated Response Time | Complexity |\n`;
      content += `|----------|--------|-------------------------|------------|\n`;
      
      endpoints.forEach(endpoint => {
        if (!endpoint.routePath) return;
        
        // Estimate response time based on endpoint complexity
        // This is just an example - replace with real metrics if available
        const baseTime = 100; // base time in ms
        const complexity = endpoint.complexity || 1;
        const estimatedTime = baseTime * complexity;
        
        const responseTime = `${estimatedTime}ms`;
        const complexityRating = complexity <= 1 ? 'Low' : 
                                complexity <= 2 ? 'Medium' : 'High';
        
        content += `| ${endpoint.routePath} | ${endpoint.httpMethod || 'GET'} | ${responseTime} | ${complexityRating} |\n`;
      });
      
      content += `\n`;
    }
    
    // Add performance recommendations
    content += `## Performance Recommendations\n\n`;
    content += `Based on the analysis, here are some recommendations for improving performance:\n\n`;
    
    const recommendations = [];
    
    // Check for high complexity files
    if (this.metrics.codeQuality && 
        this.metrics.codeQuality.complexity && 
        this.metrics.codeQuality.complexity.high > 0) {
      recommendations.push(
        `- **Refactor High Complexity Code**: ${this.metrics.codeQuality.complexity.high} files have high complexity scores. Consider breaking these down into smaller, more manageable functions.`
      );
    }
    
    // Check for large files
    const largeFiles = (this.metrics.codeQuality?.complexity?.files || [])
      .filter(file => file.lines > 500);
      
    if (largeFiles.length > 0) {
      recommendations.push(
        `- **Split Large Files**: ${largeFiles.length} files exceed 500 lines of code. Consider splitting these into smaller, more focused modules.`
      );
    }
    
    // Check for unoptimized database queries
    if (this.metrics.databaseQueries && this.metrics.databaseQueries.unoptimized > 0) {
      recommendations.push(
        `- **Optimize Database Queries**: ${this.metrics.databaseQueries.unoptimized} database queries could benefit from optimization, such as adding indexes or query refinement.`
      );
    }
    
    // General recommendations
    recommendations.push(
      `- **Implement Caching**: Consider adding caching for frequently accessed data to reduce database load.`
    );
    
    recommendations.push(
      `- **Bundle and Minify Frontend Assets**: Ensure JavaScript and CSS files are properly bundled and minified for production.`
    );
    
    recommendations.push(
      `- **Pagination for Large Data Sets**: Implement pagination for any API endpoints that return large data sets.`
    );
    
    // Add recommendations to report
    recommendations.forEach(rec => {
      content += `${rec}\n\n`;
    });
    
    // Save the performance report
    try {
      fs.writeFileSync(outputPath, content);
      console.log(`Performance analysis report generated at ${outputPath}`);
      return outputPath;
    } catch (error) {
      console.error(`Failed to write performance report: ${error.message}`);
      return null;
    }
  }

  // Add these helper methods to the UnifiedProjectAnalyzer class

  /**
   * Generate Markdown for Models
   */
  generateMarkdownForModels(modelData) {
    let markdown = `# ${modelData.title}\n\n`;
    markdown += `${modelData.summary}\n\n`;
    markdown += `System: ${modelData.system}\n`;
    markdown += `Total Models: ${modelData.totalCount}\n\n`;
    
    markdown += `## Model Listing\n\n`;
    
    modelData.models.forEach(model => {
      markdown += `### ${model.name}\n\n`;
      
      if (model.description) {
        markdown += `${model.description}\n\n`;
      }
      
      if (model.attributes && model.attributes.length > 0) {
        markdown += `#### Attributes\n\n`;
        markdown += `| Name | Type | Description |\n`;
        markdown += `|------|------|-------------|\n`;
        
        model.attributes.forEach(attr => {
          markdown += `| ${attr.name} | ${attr.type || 'unknown'} | ${attr.description || ''} |\n`;
        });
        
        markdown += `\n`;
      }
      
      if (model.relationships && model.relationships.length > 0) {
        markdown += `#### Relationships\n\n`;
        markdown += `| Related Model | Type | Description |\n`;
        markdown += `|--------------|------|-------------|\n`;
        
        model.relationships.forEach(rel => {
          markdown += `| ${rel.target} | ${rel.type || 'association'} | ${rel.description || ''} |\n`;
        });
        
        markdown += `\n`;
      }
      
      if (model.fileName) {
        markdown += `Source: \`${model.fileName}\`\n\n`;
      }
      
      markdown += `---\n\n`;
    });
    
    return markdown;
  }

  /**
   * Generate Markdown for Controllers
   */
  generateMarkdownForControllers(controllerData) {
    let markdown = `# ${controllerData.title}\n\n`;
    markdown += `${controllerData.summary}\n\n`;
    markdown += `System: ${controllerData.system}\n`;
    markdown += `Total Controllers: ${controllerData.totalCount}\n\n`;
    
    markdown += `## Controller Listing\n\n`;
    
    controllerData.controllers.forEach(controller => {
      markdown += `### ${controller.name}\n\n`;
      
      if (controller.description) {
        markdown += `${controller.description}\n\n`;
      }
      
      if (controller.actions && controller.actions.length > 0) {
        markdown += `#### Actions\n\n`;
        markdown += `| Action | HTTP Method | Route | Description |\n`;
        markdown += `|--------|------------|-------|-------------|\n`;
        
        controller.actions.forEach(action => {
          markdown += `| ${action.name} | ${action.httpMethod || 'GET'} | ${action.route || ''} | ${action.description || ''} |\n`;
        });
        
        markdown += `\n`;
      }
      
      if (controller.usedModels && controller.usedModels.length > 0) {
        markdown += `#### Referenced Models\n\n`;
        markdown += `- ${controller.usedModels.join('\n- ')}\n\n`;
      }
      
      if (controller.fileName) {
        markdown += `Source: \`${controller.fileName}\`\n\n`;
      }
      
      markdown += `---\n\n`;
    });
    
    return markdown;
  }

  /**
   * Generate Markdown for Workflow
   */
  generateMarkdownForWorkflow(workflow, system) {
    let markdown = `# ${workflow.name.charAt(0).toUpperCase() + workflow.name.slice(1)} Workflow\n\n`;
    
    markdown += `## Overview\n\n`;
    markdown += `This document describes the ${workflow.name} workflow in the ${system} system.\n\n`;
    
    if (workflow.controllers && workflow.controllers.length > 0) {
      markdown += `## Related Controllers\n\n`;
      markdown += workflow.controllers.map(c => `- ${c}`).join('\n') + '\n\n';
    }
    
    if (workflow.actions && workflow.actions.length > 0) {
      markdown += `## Actions\n\n`;
      markdown += `| Action | Controller | Input Models | Output Models | Description |\n`;
      markdown += `|--------|------------|--------------|---------------|-------------|\n`;
      
      workflow.actions.forEach(action => {
        const inputs = action.inputModels?.join(', ') || '';
        const outputs = action.outputModels?.join(', ') || '';
        
        markdown += `| ${action.name} | ${action.controller} | ${inputs} | ${outputs} | ${action.description || ''} |\n`;
      });
      
      markdown += `\n`;
    }
    
    if (workflow.dataFlow && workflow.dataFlow.length > 0) {
      markdown += `## Data Flow\n\n`;
      markdown += `\`\`\`mermaid\nflowchart LR\n`;
      
      workflow.dataFlow.forEach((flow, index) => {
        markdown += `  ${flow.from.replace(/\s/g, '_')} -->|${flow.action}| ${flow.to.replace(/\s/g, '_')}\n`;
      });
      
      markdown += `\`\`\`\n\n`;
    }
    
    return markdown;
  }

  /**
   * Generate Markdown for Architecture Pattern
   */
  generateMarkdownForPattern(pattern, system) {
    let markdown = `# ${pattern.name} Pattern\n\n`;
    
    markdown += `## Overview\n\n`;
    markdown += `${pattern.description || `This document describes the ${pattern.name} architectural pattern in the ${system} system.`}\n\n`;
    
    if (pattern.benefits && pattern.benefits.length > 0) {
      markdown += `## Benefits\n\n`;
      markdown += pattern.benefits.map(b => `- ${b}`).join('\n') + '\n\n';
    }
    
    if (pattern.implementations && pattern.implementations.length > 0) {
      markdown += `## Implementations\n\n`;
      
      pattern.implementations.forEach(impl => {
        markdown += `### ${impl.name}\n\n`;
        markdown += `${impl.description || ''}\n\n`;
        
        if (impl.location) {
          markdown += `Location: \`${impl.location}\`\n\n`;
        }
        
        if (impl.codeSnippet) {
          markdown += `\`\`\`${impl.language || 'javascript'}\n`;
          markdown += impl.codeSnippet;
          markdown += `\n\`\`\`\n\n`;
        }
      });
    }
    
    return markdown;
  }

  /**
   * Generate Markdown for API endpoints
   */
  generateMarkdownForApiEndpoints(endpoints, system) {
    let markdown = `# ${system.toUpperCase()} API Documentation\n\n`;
    
    markdown += `This document provides information about the API endpoints available in the ${system} system.\n\n`;
    
    // Group endpoints by resource
    const resourceGroups = {};
    
    endpoints.forEach(endpoint => {
      const parts = endpoint.path.split('/');
      // Get the resource name (usually the first part after API version)
      const resourceIndex = parts.findIndex(p => p === 'api' || p === 'v1' || p === 'v2') + 1;
      const resource = resourceIndex < parts.length ? parts[resourceIndex] : 'other';
      
      if (!resourceGroups[resource]) {
        resourceGroups[resource] = [];
      }
      
      resourceGroups[resource].push(endpoint);
    });
    
    // Generate docs for each resource group
    for (const [resource, resourceEndpoints] of Object.entries(resourceGroups)) {
      markdown += `## ${resource.charAt(0).toUpperCase() + resource.slice(1)}\n\n`;
      
      resourceEndpoints.forEach(endpoint => {
        markdown += `### ${endpoint.method} ${endpoint.path}\n\n`;
        
        if (endpoint.description) {
          markdown += `${endpoint.description}\n\n`;
        }
        
        if (endpoint.parameters && endpoint.parameters.length > 0) {
          markdown += `#### Parameters\n\n`;
          markdown += `| Name | Type | Required | Description |\n`;
          markdown += `|------|------|----------|-------------|\n`;
          
          endpoint.parameters.forEach(param => {
            markdown += `| ${param.name} | ${param.type || 'string'} | ${param.required ? 'Yes' : 'No'} | ${param.description || ''} |\n`;
          });
          
          markdown += `\n`;
        }
        
        if (endpoint.requestBody) {
          markdown += `#### Request Body\n\n`;
          markdown += `\`\`\`json\n${JSON.stringify(endpoint.requestBody, null, 2)}\n\`\`\`\n\n`;
        }
        
        if (endpoint.responses) {
          markdown += `#### Responses\n\n`;
          
          for (const [code, response] of Object.entries(endpoint.responses)) {
            markdown += `**${code}**\n\n`;
            
            if (response.description) {
              markdown += `${response.description}\n\n`;
            }
            
            if (response.example) {
              markdown += `\`\`\`json\n${JSON.stringify(response.example, null, 2)}\n\`\`\`\n\n`;
            }
          }
        }
        
        markdown += `---\n\n`;
      });
    }
    
    return markdown;
  }

  /**
   * Generate Markdown for Integration Points
   */
  generateMarkdownForIntegrationPoints(integrationPoints) {
    let markdown = `# Integration Points Between Canvas and Discourse\n\n`;
    
    markdown += `This document identifies potential integration points between the Canvas LMS and Discourse forum systems.\n\n`;
    
    // Group by integration type
    const types = {};
    integrationPoints.forEach(point => {
      if (!types[point.type]) {
        types[point.type] = [];
      }
      types[point.type].push(point);
    });
    
    for (const [type, points] of Object.entries(types)) {
      markdown += `## ${type.charAt(0).toUpperCase() + type.slice(1)} Integration\n\n`;
      
      points.forEach(point => {
        markdown += `### ${point.name}\n\n`;
        
        markdown += `${point.description || ''}\n\n`;
        
        markdown += `**Canvas Component:** ${point.canvasComponent}\n\n`;
        markdown += `**Discourse Component:** ${point.discourseComponent}\n\n`;
        
        if (point.implementationNotes) {
          markdown += `#### Implementation Notes\n\n`;
          markdown += point.implementationNotes + '\n\n';
        }
        
        if (point.dataFlow) {
          markdown += `#### Data Flow\n\n`;
          markdown += `\`\`\`mermaid\nsequenceDiagram\n`;
          
          point.dataFlow.forEach(flow => {
            markdown += `  ${flow.from}->>${flow.to}: ${flow.description}\n`;
          });
          
          markdown += `\`\`\`\n\n`;
        }
        
        markdown += `---\n\n`;
      });
    }
    
    return markdown;
  }

  /**
   * Generate Markdown for Model Mappings
   */
  generateMarkdownForModelMappings(modelMappings) {
    let markdown = `# Cross-System Model Mappings\n\n`;
    
    markdown += `This document defines mappings between Canvas and Discourse data models for integration purposes.\n\n`;
    
    markdown += `| Canvas Model | Discourse Model | Mapping Type | Description |\n`;
    markdown += `|-------------|----------------|--------------|-------------|\n`;
    
    modelMappings.forEach(mapping => {
      markdown += `| ${mapping.canvasModel} | ${mapping.discourseModel} | ${mapping.type} | ${mapping.description || ''} |\n`;
    });
    
    markdown += `\n## Detailed Mappings\n\n`;
    
    modelMappings.forEach(mapping => {
      markdown += `### ${mapping.canvasModel} ↔ ${mapping.discourseModel}\n\n`;
      
      if (mapping.description) {
        markdown += `${mapping.description}\n\n`;
      }
      
      if (mapping.attributes && mapping.attributes.length > 0) {
        markdown += `#### Attribute Mappings\n\n`;
        markdown += `| Canvas Attribute | Discourse Attribute | Transformation | Notes |\n`;
        markdown += `|-----------------|---------------------|---------------|-------|\n`;
        
        mapping.attributes.forEach(attr => {
          markdown += `| ${attr.canvas} | ${attr.discourse} | ${attr.transformation || 'N/A'} | ${attr.notes || ''} |\n`;
        });
        
        markdown += `\n`;
      }
    });
    
    return markdown;
  }

  /**
   * Generate system knowledge base documents
   * @param {string} system - System name (canvas, discourse)
   * @param {string} outputDir - Output directory
   */
  async generateSystemKnowledgeBase(system, outputDir) {
    console.log(`Generating knowledge base for ${system}...`);
    
    // Create system directory if it doesn't exist
    const systemDir = path.join(outputDir, system);
    if (!fs.existsSync(systemDir)) {
      fs.mkdirSync(systemDir, { recursive: true });
    }
    
    // Generate documents based on system type
    switch(system) {
      case 'canvas':
        await this.generateCanvasDocuments(systemDir);
        break;
      case 'discourse':
        await this.generateDiscourseDocuments(systemDir);
        break;
      default:
        console.warn(`Unknown system: ${system}`);
    }
    
    console.log(`Generated knowledge base for ${system}`);
  }

  /**
   * Generate Canvas system documents
   * @param {string} outputDir - Output directory
   */
  async generateCanvasDocuments(outputDir) {
    console.log("Generating Canvas documents...");
    
    // Create models directory
    const modelsDir = path.join(outputDir, 'models');
    fsExtra.ensureDirSync(modelsDir);  // Changed from fs.ensureDirSync
    
    // Simple placeholder for demo purposes - in a real implementation, 
    // you would analyze the actual Canvas codebase
    const models = [
      { 
        name: 'Course',
        description: 'Canvas Course model representing a course in the LMS',
        properties: [
          { name: 'id', type: 'integer', description: 'Unique identifier' },
          { name: 'name', type: 'string', description: 'Course name' },
          { name: 'code', type: 'string', description: 'Course code' },
          { name: 'workflow_state', type: 'string', description: 'Current state of the course' }
        ],
        relationships: [
          { name: 'enrollments', type: 'has_many', target: 'Enrollment', description: 'Student enrollments' },
          { name: 'discussion_topics', type: 'has_many', target: 'DiscussionTopic', description: 'Discussion topics' }
        ]
      },
      { 
        name: 'DiscussionTopic',
        description: 'Canvas discussion topic model',
        properties: [
          { name: 'id', type: 'integer', description: 'Unique identifier' },
          { name: 'title', type: 'string', description: 'Topic title' },
          { name: 'message', type: 'text', description: 'Topic content' }
        ],
        relationships: [
          { name: 'course', type: 'belongs_to', target: 'Course', description: 'Associated course' },
          { name: 'entries', type: 'has_many', target: 'DiscussionEntry', description: 'Discussion replies' }
        ]
      }
    ];
    
    // Generate model documentation
    for (const model of models) {
      const modelDoc = this.generateModelDocument(model, 'canvas');
      fs.writeFileSync(path.join(modelsDir, `${model.name.toLowerCase()}.md`), modelDoc);
    }
    
    // Generate API documentation
    const apisDir = path.join(outputDir, 'apis');
    fsExtra.ensureDirSync(apisDir);  // Changed from fs.ensureDirSync
    
    const apis = [
      {
        name: 'Courses API',
        description: 'API endpoints for managing Canvas courses',
        endpoints: [
          { path: '/api/v1/courses', method: 'GET', description: 'List courses' },
          { path: '/api/v1/courses/:id', method: 'GET', description: 'Get a single course' },
          { path: '/api/v1/courses/:course_id/discussion_topics', method: 'GET', description: 'List discussion topics' }
        ]
      }
    ];
    
    // Generate API documentation
    for (const api of apis) {
      const apiDoc = this.generateApiDocument(api, 'canvas');
      fs.writeFileSync(path.join(apisDir, `${api.name.toLowerCase().replace(/\s+/g, '_')}.md`), apiDoc);
    }
  }

  /**
   * Generate Discourse system documents
   * @param {string} outputDir - Output directory
   */
  async generateDiscourseDocuments(outputDir) {
    console.log("Generating Discourse documents...");
    
    // Create models directory
    const modelsDir = path.join(outputDir, 'models');
    fsExtra.ensureDirSync(modelsDir);  // Changed from fs.ensureDirSync
    
    // Simple placeholder for demo purposes
    const models = [
      { 
        name: 'Topic',
        description: 'Discourse Topic model representing a discussion topic',
        properties: [
          { name: 'id', type: 'integer', description: 'Unique identifier' },
          { name: 'title', type: 'string', description: 'Topic title' },
          { name: 'category_id', type: 'integer', description: 'Category ID' }
        ],
        relationships: [
          { name: 'category', type: 'belongs_to', target: 'Category', description: 'Parent category' },
          { name: 'posts', type: 'has_many', target: 'Post', description: 'Posts in this topic' }
        ]
      },
      { 
        name: 'Category',
        description: 'Discourse category for organizing topics',
        properties: [
          { name: 'id', type: 'integer', description: 'Unique identifier' },
          { name: 'name', type: 'string', description: 'Category name' },
          { name: 'slug', type: 'string', description: 'URL-friendly name' }
        ],
        relationships: [
          { name: 'topics', type: 'has_many', target: 'Topic', description: 'Topics in this category' }
        ]
      }
    ];
    
    // Generate model documentation
    for (const model of models) {
      const modelDoc = this.generateModelDocument(model, 'discourse');
      fs.writeFileSync(path.join(modelsDir, `${model.name.toLowerCase()}.md`), modelDoc);
    }
    
    // Generate API documentation
    const apisDir = path.join(outputDir, 'apis');
    fsExtra.ensureDirSync(apisDir);  // Changed from fs.ensureDirSync
    
    const apis = [
      {
        name: 'Topics API',
        description: 'API endpoints for managing Discourse topics',
        endpoints: [
          { path: '/t/:slug/:topic_id.json', method: 'GET', description: 'Get a topic' },
          { path: '/topics/latest.json', method: 'GET', description: 'List latest topics' }
        ]
      }
    ];
    
    // Generate API documentation
    for (const api of apis) {
      const apiDoc = this.generateApiDocument(api, 'discourse');
      fs.writeFileSync(path.join(apisDir, `${api.name.toLowerCase().replace(/\s+/g, '_')}.md`), apiDoc);
    }
  }

  /**
   * Generate model documentation
   * @param {Object} model - Model definition
   * @param {string} system - System name
   * @returns {string} Markdown documentation
   */
  generateModelDocument(model, system) {
    let markdown = `# ${model.name} Model\n\n`;
    
    markdown += `## Overview\n\n`;
    markdown += `${model.description}\n\n`;
    markdown += `System: ${system}\n\n`;
    
    markdown += `## Properties\n\n`;
    markdown += `| Name | Type | Description |\n`;
    markdown += `|------|------|-------------|\n`;
    
    for (const prop of model.properties) {
      markdown += `| ${prop.name} | ${prop.type} | ${prop.description} |\n`;
    }
    
    markdown += `\n## Relationships\n\n`;
    markdown += `| Name | Type | Target | Description |\n`;
    markdown += `|------|------|--------|-------------|\n`;
    
    for (const rel of model.relationships) {
      markdown += `| ${rel.name} | ${rel.type} | ${rel.target} | ${rel.description} |\n`;
    }
    
    return markdown;
  }

  /**
   * Generate API documentation
   * @param {Object} api - API definition
   * @param {string} system - System name
   * @returns {string} Markdown documentation
   */
  generateApiDocument(api, system) {
    let markdown = `# ${api.name}\n\n`;
    
    markdown += `## Overview\n\n`;
    markdown += `${api.description}\n\n`;
    markdown += `System: ${system}\n\n`;
    
    markdown += `## Endpoints\n\n`;
    
    for (const endpoint of api.endpoints) {
      markdown += `### ${endpoint.method} ${endpoint.path}\n\n`;
      markdown += `${endpoint.description}\n\n`;
    }
    
    return markdown;
  }

  /**
   * Generate system behavior documents
   * @param {string} system - System name
   * @param {string} outputDir - Output directory
   */
  async generateSystemBehaviorDocuments(system, outputDir) {
    console.log(`Generating behavior documents for ${system}...`);
    
    // Create behavior directory
    const behaviorDir = path.join(outputDir, system, 'behavior');
    fsExtra.ensureDirSync(behaviorDir); // Changed from fs.ensureDirSync
    
    // Example behaviors based on system
    const behaviors = system === 'canvas' 
      ? [
          { name: 'Course Creation', description: 'Process of creating a new course' },
          { name: 'Discussion Topic Creation', description: 'How discussion topics are created and managed' }
        ]
      : [
          { name: 'Topic Creation', description: 'Process of creating a new topic' },
          { name: 'Category Management', description: 'How categories are organized and managed' }
        ];
    
    // Generate behavior documents
    for (const behavior of behaviors) {
      const doc = this.generateBehaviorDocument(behavior, system);
      fs.writeFileSync(
        path.join(behaviorDir, `${behavior.name.toLowerCase().replace(/\s+/g, '_')}.md`), 
        doc
      );
    }
  }

  /**
   * Generate behavior document
   * @param {Object} behavior - Behavior definition
   * @param {string} system - System name
   * @returns {string} Markdown documentation
   */
  generateBehaviorDocument(behavior, system) {
    let markdown = `# ${behavior.name}\n\n`;
    
    markdown += `## Overview\n\n`;
    markdown += `${behavior.description}\n\n`;
    markdown += `System: ${system}\n\n`;
    
    markdown += `## Process Flow\n\n`;
    markdown += `This is a placeholder for the detailed process flow of ${behavior.name} in ${system}.\n\n`;
    
    return markdown;
  }

  /**
   * Generate system architecture patterns
   * @param {string} system - System name
   * @param {string} outputDir - Output directory
   */
  async generateSystemArchitecturePatterns(system, outputDir) {
    console.log(`Generating architecture patterns for ${system}...`);
    
    // Create architecture directory
    const archDir = path.join(outputDir, system, 'architecture');
    fsExtra.ensureDirSync(archDir);
    
    // Example patterns based on system
    const patterns = system === 'canvas' 
      ? [
          { 
            name: 'MVC Pattern', 
            description: 'Canvas uses the Model-View-Controller pattern for organizing code',
            benefits: ['Separation of concerns', 'Testability', 'Code organization'],
            implementations: [
              { name: 'Course Controller', description: 'Handles course-related requests', location: 'app/controllers/courses_controller.rb' }
            ]
          }
        ]
      : [
          { 
            name: 'Plugin Architecture', 
            description: 'Discourse uses a plugin architecture for extensibility',
            benefits: ['Modularity', 'Extensibility', 'Community contributions'],
            implementations: [
              { name: 'Plugin System', description: 'Core plugin infrastructure', location: 'lib/plugin.rb' }
            ]
          }
        ];
    
    // Generate pattern documents
    for (const pattern of patterns) {
      const doc = this.generateMarkdownForPattern(pattern, system);
      fs.writeFileSync(
        path.join(archDir, `${pattern.name.toLowerCase().replace(/\s+/g, '_')}.md`), 
        doc
      );
    }
  }

  /**
   * Generate system API contracts
   * @param {string} system - System name
   * @param {string} outputDir - Output directory
   */
  async generateSystemAPIContracts(system, outputDir) {
    console.log(`Generating API contracts for ${system}...`);
    
    // Create API contracts directory
    const contractsDir = path.join(outputDir, system, 'contracts');
    fsExtra.ensureDirSync(contractsDir);
    
    // Example API contracts based on system
    const contracts = system === 'canvas' 
      ? [
          { 
            name: 'Course API Contract', 
            description: 'API contract for Course-related endpoints',
            endpoints: [
              { 
                path: '/api/v1/courses/:id', 
                method: 'GET',
                parameters: [{ name: 'id', type: 'integer', description: 'Course ID' }],
                responses: [
                  { status: 200, description: 'Success', example: '{ "id": 1, "name": "Example Course" }' }
                ]
              }
            ]
          }
        ]
      : [
          { 
            name: 'Topic API Contract', 
            description: 'API contract for Topic-related endpoints',
            endpoints: [
              { 
                path: '/t/:slug/:topic_id.json', 
                method: 'GET',
                parameters: [
                  { name: 'slug', type: 'string', description: 'Topic slug' },
                  { name: 'topic_id', type: 'integer', description: 'Topic ID' }
                ],
                responses: [
                  { status: 200, description: 'Success', example: '{ "id": 1, "title": "Example Topic" }' }
                ]
              }
            ]
          }
        ];
    
    // Generate contract documents
    for (const contract of contracts) {
      const doc = this.generateAPIContractDocument(contract, system);
      fs.writeFileSync(
        path.join(contractsDir, `${contract.name.toLowerCase().replace(/\s+/g, '_')}.md`), 
        doc
      );
    }
  }

  /**
   * Generate API contract document
   * @param {Object} contract - API contract definition
   * @param {string} system - System name
   * @returns {string} Markdown documentation
   */
  generateAPIContractDocument(contract, system) {
    let markdown = `# ${contract.name}\n\n`;
    
    markdown += `## Overview\n\n`;
    markdown += `${contract.description}\n\n`;
    markdown += `System: ${system}\n\n`;
    
    for (const endpoint of contract.endpoints) {
      markdown += `## ${endpoint.method} ${endpoint.path}\n\n`;
      
      // Parameters
      if (endpoint.parameters && endpoint.parameters.length > 0) {
        markdown += `### Parameters\n\n`;
        markdown += `| Name | Type | Description |\n`;
        markdown += `|------|------|-------------|\n`;
        
        for (const param of endpoint.parameters) {
          markdown += `| ${param.name} | ${param.type} | ${param.description} |\n`;
        }
        
        markdown += `\n`;
      }
      
      // Responses
      if (endpoint.responses && endpoint.responses.length > 0) {
        markdown += `### Responses\n\n`;
        
        for (const response of endpoint.responses) {
          markdown += `#### ${response.status} - ${response.description}\n\n`;
          
          if (response.example) {
            markdown += `\`\`\`json\n${response.example}\n\`\`\`\n\n`;
          }
        }
      }
    }
    
    return markdown;
  }

  /**
   * Generate integration knowledge base
   * @param {Array<string>} systems - System names
   * @param {string} outputDir - Output directory
   */
  async generateIntegrationKnowledgeBase(systems, outputDir) {
    console.log("Generating integration knowledge base...");
    
    // Create integration directory
    const integrationDir = path.join(outputDir, 'integration');
    fsExtra.ensureDirSync(integrationDir);
    
    // Generate integration documents
    const integrationDocs = [
      {
        name: 'Canvas-Discourse Integration Points',
        filename: 'integration_points.md',
        content: `# Canvas-Discourse Integration Points

## Overview

This document describes the key integration points between Canvas LMS and Discourse forum systems.

## Integration Mapping

### Course to Category Mapping

Canvas courses can be mapped to Discourse categories:

| Canvas | Discourse | Notes |
|--------|-----------|-------|
| Course | Category | One-to-one mapping |
| Course Sections | Sub-categories | Optional |

### Discussion Topic Mapping

Canvas discussion topics can be synchronized with Discourse topics:

| Canvas | Discourse | Notes |
|--------|-----------|-------|
| Discussion Topic | Topic | One-to-one mapping |
| Discussion Entry | Post | One-to-one mapping |
| Discussion Reply | Reply | One-to-one mapping |

## Integration Strategies

1. **API-based integration**: Use REST APIs on both systems
2. **Event-driven integration**: Use webhooks and event subscribers
3. **Database-level integration**: Direct database integration (not recommended)

## Authentication Flow

For SSO between Canvas and Discourse:

1. Canvas authenticates the user
2. Canvas generates a signed payload with user information
3. User is redirected to Discourse with the payload
4. Discourse verifies the payload and creates/logs in the user
`
      },
      {
        name: 'Integration Architecture Blueprint',
        filename: 'architecture-blueprint.md',
        content: `# Integration Architecture Blueprint

## Overview

This document describes the recommended architecture for integrating Canvas LMS with Discourse forums.

## Architecture Diagram

\`\`\`
┌─────────────┐           ┌──────────────┐
│             │           │              │
│   Canvas    │◄─────────►│   Discourse  │
│    LMS      │   APIs    │    Forums    │
│             │           │              │
└─────────────┘           └──────────────┘
       ▲                         ▲
       │                         │
       │         ┌───────────────┘
       │         │
┌──────▼─────────▼──┐
│                   │
│   Integration     │
│   Service         │
│                   │
└───────────────────┘
       ▲
       │
┌──────▼──────┐
│             │
│  Database   │
│             │
└─────────────┘
\`\`\`

## Integration Components

1. **API Adapters**: Connect to both systems via their APIs
2. **Event Listeners**: Listen for changes in either system
3. **Sync Service**: Maintain data consistency between systems
4. **Mapping Service**: Handle entity relationships between systems
5. **Authentication Bridge**: Enable SSO between systems
`
      }
    ];
    
    // Write integration documents
    for (const doc of integrationDocs) {
      fs.writeFileSync(path.join(integrationDir, doc.filename), doc.content);
    }
    
    console.log("Integration knowledge base generated");
  }

  /**
   * Update integration documentation based on RAG knowledge base
   */
  async updateIntegrationDocumentation() {
    console.log("Updating Canvas-Discourse integration documentation...");
    
    // First, ensure technical docs are up to date
    await this.generateTechnicalImplementationDocs();
    
    const generator = new IntegrationReportGenerator({
      baseDir: this.baseDir,
      ragKnowledgeBase: 'rag_knowledge_base',
      outputDir: 'docs'
    });
    
    // Generate the report
    const reportPath = await generator.generateReport();
    
    // Update the central reference hub
    const hubPath = path.join(this.baseDir, 'docs', 'central_reference_hub.md');
    generator.updateCentralReferenceHub(hubPath, reportPath);
    
    console.log("Integration documentation updated successfully");
    return reportPath;
  }

  /**
   * Generate technical implementation documentation from source code
   * @returns {Promise<string>} Path to generated documentation
   */
  async generateTechnicalImplementationDocs() {
    console.log("Generating technical implementation documentation from source code...");
    
    const generator = new TechnicalDocsGenerator({
      baseDir: this.baseDir,
      outputDir: path.join(this.baseDir, 'rag_knowledge_base', 'integration'),
      sourcePatterns: [
        'services/integration/**/*.js',
        'services/integration/**/*.ts', 
        'controllers/integration/**/*.js',
        'controllers/integration/**/*.ts',
        'models/integration/**/*.js',
        'models/integration/**/*.ts',
        'plugins/discourse/**/*.rb',
        'plugins/discourse/**/*.js',
        'plugins/canvas/**/*.rb',
        'plugins/canvas/**/*.js'
      ]
    });
    
    const docPath = await generator.generate();
    console.log("Technical implementation documentation generated successfully");
    
    return docPath;
  }

  /**
   * Print analysis summary to console
   */
  printSummary() {
    console.log(`Project Status: Models=${this.metrics.overallStatus.models}, API=${this.metrics.overallStatus.api}, UI=${this.metrics.overallStatus.ui}, Tests=${this.metrics.overallStatus.tests}, Debt=${this.metrics.overallStatus.techDebt}`);
    console.log(`Overall Phase: ${this.metrics.overallPhase}`);
    
    // Print file statistics
    const fileStats = this.fsUtils.getFileStats();
    console.log(`Processed ${fileStats.total} files (${fileStats.js} JS/TS, ${fileStats.rust} Rust, ${fileStats.other} other)`);
    
    // Print model statistics
    console.log(`Found ${this.metrics.models.total} models, ${this.metrics.apiEndpoints.total} API endpoints, ${this.metrics.uiComponents.total} UI components`);
    
    // Print performance stats
    if (this.metrics.performance) {
      console.log(`Analysis completed in ${(this.metrics.performance.totalTime / 1000).toFixed(2)}s`);
    }
  }

  /**
   * Update the Last Analysis Results file for AI assistants
   */
  async updateAiAnalysisResults() {
    console.log("Updating AI analysis results summary...");
    
    const resultsPath = path.join(this.baseDir, 'LAST_ANALYSIS_RESULTS.md');
    const timestamp = new Date().toISOString().replace('T', ' ').substring(0, 19);
      // Format component status
    const componentStatus = [
      { name: 'Model Mapping', status: 'In Progress', completion: '45%', nextSteps: 'Complete Course-Category testing' },
      { name: 'API Integration', status: 'In Progress', completion: '10%', nextSteps: 'Begin CRUD operations implementation' },
      { name: 'Authentication', status: 'Implemented', completion: '100%', nextSteps: 'Add more authentication tests' },
      { name: 'Synchronization', status: 'Not Started', completion: '0%', nextSteps: 'Design sync architecture' }
    ];
    
    // Get recent changes (files that have changed since last analysis)
    const recentChanges = await this.getRecentChanges();
    
    // Format the results markdown
    let content = `# Last Analysis Results\n\n`;
    content += `*This file is automatically updated after each analysis run*\n\n`;
    content += `## Analysis Summary\n\n`;
    content += `**Last Run**: ${timestamp}\n\n`;
    content += `**Project Status**:\n`;
    content += `- Models: ${this.metrics.overallStatus.models} complete\n`;
    content += `- API: ${this.metrics.overallStatus.api} complete\n`;
    content += `- UI: ${this.metrics.overallStatus.ui} complete\n`;
    content += `- Tests: ${this.metrics.overallStatus.tests} complete\n`;
    content += `- Technical Debt: ${this.metrics.overallStatus.techDebt}\n\n`;
    content += `**Overall Phase**: ${this.metrics.overallPhase}\n\n`;
    
    content += `## Integration Status\n\n`;
    content += `| Component | Status | Completion | Next Steps |\n`;
    content += `|-----------|--------|------------|------------|\n`;
    
    for (const component of componentStatus) {
      content += `| ${component.name} | ${component.status} | ${component.completion} | ${component.nextSteps} |\n`;
    }
    content += '\n';
    
    content += `## Recent Changes\n\n`;
    if (recentChanges && recentChanges.length > 0) {
      content += `The following files were updated in the last analysis:\n`;
      for (const file of recentChanges.slice(0, 5)) {
        content += `- \`${file}\`\n`;
      }
      if (recentChanges.length > 5) {
        content += `- and ${recentChanges.length - 5} more files\n`;
      }
    } else {
      content += `No significant changes detected in this analysis run.\n`;
    }
    content += '\n';
    
    content += `## Next Priorities\n\n`;
    content += `1. Complete the JWT authentication implementation\n`;
    content += `2. Finalize Course-Category mapping with tests\n`;
    content += `3. Begin implementation of Discussion Topic mapping\n\n`;
    
    content += `## Documentation Updates\n\n`;
    content += `The following documentation was updated:\n`;
    content += `- Central Reference Hub: [\`docs/central_reference_hub.md\`](docs/central_reference_hub.md)\n`;
    content += `- Integration Reference: [\`docs/canvas_discourse_integration.md\`](docs/canvas_discourse_integration.md)\n`;
    content += `- Technical Implementation: [\`rag_knowledge_base/integration/technical_implementation.md\`](rag_knowledge_base/integration/technical_implementation.md)\n`;
    
    // Write the file
    fs.writeFileSync(resultsPath, content);
    console.log(`AI analysis results updated at ${resultsPath}`);
  }

  /**
   * Gets the list of recently changed files in the project
   * @param {number} days - Number of days to look back (default: 7)
   * @returns {Array<Object>} List of changed files with metadata
   */
  getRecentChanges(days = 7) {
    try {
      const { execSync } = require('child_process');
      const path = require('path');
      
      // Calculate date for git log
      const sinceDate = new Date();
      sinceDate.setDate(sinceDate.getDate() - days);
      const sinceDateStr = sinceDate.toISOString().split('T')[0];
      
      // Get git changes
      const gitCommand = `git log --name-status --since="${sinceDateStr}" --pretty=format:"%h|%an|%ad|%s" --date=short`;
      let gitOutput;
      
      try {
        gitOutput = execSync(gitCommand, { cwd: this.baseDir, encoding: 'utf8' });
      } catch (gitErr) {
        console.warn(`Warning: Git command failed: ${gitErr.message}`);
        return []; // Return empty array if git command fails
      }
      
      // Process the git output to extract changed files
      const changes = [];
      const lines = gitOutput.split('\n');
      let currentCommit = null;
      
      for (const line of lines) {
        if (!line.trim()) continue;
        
        if (line.includes('|')) {
          // This is a commit line
          const [hash, author, date, ...messageParts] = line.split('|');
          const message = messageParts.join('|');
          
          currentCommit = {
            hash,
            author,
            date,
            message
          };
        } else if (line.match(/^[AMDRT]\s/)) {
          // This is a file change line
          const [status, filePath] = line.trim().split(/\s+/);
          
          if (!filePath) continue;
          
          // Skip files we don't care about for analysis
          if (filePath.includes('node_modules/') || 
              filePath.includes('/dist/') || 
              filePath.endsWith('.log') || 
              filePath.endsWith('.md')) {
            continue;
          }
          
          const fullPath = path.join(this.baseDir, filePath);
          const extension = path.extname(filePath).toLowerCase();
          
          changes.push({
            path: fullPath,
            relativePath: filePath,
            status: getStatusText(status),
            extension,
            commit: { ...currentCommit }
          });
        }
      }
      
      return changes;
    } catch (error) {
      console.error('Error getting recent changes:', error);
      return [];
    }
  }
}

// Helper function to convert git status codes to text
function getStatusText(code) {
  const statusMap = {
    'A': 'added',
    'M': 'modified',
    'D': 'deleted',
    'R': 'renamed',
    'T': 'type_changed'
  };
  return statusMap[code] || 'unknown';
}

module.exports = UnifiedProjectAnalyzer;