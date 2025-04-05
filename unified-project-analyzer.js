const parser = require('@babel/parser');
const traverse = require('@babel/traverse').default;
const FileSystemUtils = require('./fileSystemUtils');
const AnalysisUtils = require('./analysisUtils'); // Import the new AnalysisUtils module
const path = require('path');
const fs = require('fs'); // Keep fs for generateCentralReferenceHub
const AstAnalyzer = require('./astAnalyzer'); // Import the new AstAnalyzer module
const ProjectPredictor = require('./projectPredictor'); // Import the new ProjectPredictor module

/**
 * Unified Project Analyzer
 * Consolidates all analyzer functionality into a single tool
 */
class UnifiedProjectAnalyzer {
  constructor(baseDir) {
    this.baseDir = baseDir;

    // Configuration
    this.config = {
      implementationThreshold: 35,
    };
    
    // Metrics tracking - INITIALIZE METRICS FIRST
    this.metrics = {
      models: { total: 0, implemented: 0, details: [] },
      apiEndpoints: { total: 0, implemented: 0, details: [] },
      uiComponents: { total: 0, implemented: 0, details: [] },
      tests: { total: 0, passing: 0, coverage: 0, details: [] },
      
      codeQuality: {
          complexity: {
              average: 0,
              high: 0,
              files: []
          },
          techDebt: {
              score: 0,
              items: []
          },
          solidViolations: {
              srp: [],
              ocp: [],
              lsp: [],
              isp: [],
              dip: []
          },
          designPatterns: {
              polymorphism: {
                  implementations: [],
                  violations: []  
              },
              dependencyInjection: {
                  implementations: [],
                  violations: []
              },
              ioc: {
                  implementations: [],
                  violations: []
              }
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
              models: 1.5,       // Models implemented per week
              apiEndpoints: 3,    // API endpoints implemented per week
              uiComponents: 5,    // UI components implemented per week
              tests: 2           // Tests added per week
          },
          estimates: {}
      }
    };

    // Initialize utility classes
    this.fsUtils = new FileSystemUtils(this.baseDir, this.getExcludePatterns());
    this.analysisUtils = new AnalysisUtils(this.baseDir, this.fsUtils, this.config, this.metrics);
    this.astAnalyzer = new AstAnalyzer();
    this.predictor = new ProjectPredictor(this.metrics);
  }

  /**
   * Defines the patterns for excluding directories and files during discovery.
   */
  getExcludePatterns() {
    // Keep exclude patterns logic here or move fully to fsUtils if preferred
    return [
      /node_modules/,
      /\.git/,
      /target\/(?!.*\.rs$)/, // Exclude target dir except .rs files
      /dist/,
      /build/,
      /\.cache/,
      /\.next/,
      /\.nuxt/,
      /\.DS_Store/,
      /coverage/,
      /\.vscode/,
      /\.idea/,
      /assets/, // Exclude assets dir
      /public/, // Exclude public dir
      /docs/, // Exclude docs dir
      /references/, // Exclude references dir
      /analysis_summary/, // Exclude analysis_summary dir
      /md_dashboard/, // Exclude md_dashboard dir
      /tools\/__pycache__/, // Exclude python cache
      /.*\.log$/, // Exclude log files
      /.*\.tmp$/, // Exclude temp files
      /.*\.bak.?$/, // Exclude backup files
      /.*\.swp$/, // Exclude vim swap files
      /LMS\.code-workspace/, // Exclude workspace file
      /package-lock\.json/, // Exclude lock file
      /yarn\.lock/, // Exclude lock file
      /unified-project-analyzer\.js/, // Exclude self
      /project-analyzer\.js.*/, // Exclude older analyzer versions
      /debug-analyzer\.js/,
      /status-analyzer\.js/,
      /advanced-api-analyzer\.js/,
      /fix-.*\.js/, // Exclude fix scripts
      /cleanup-docs\.js/,
      /run-full-analysis\.js/,
      /status-updater\.js/,
      /analyze_project\.pdb/, // Exclude pdb file
      /fileSystemUtils\.js/, // Exclude the new utils file
    ];
  }

  /**
   * Run the analysis
   */
  async analyze() {
    console.log(`Starting analysis of ${this.baseDir}...`);

    // Discover and read files using FileSystemUtils
    this.fsUtils.discoverFiles();
    this.fsUtils.readFileContents();

    // Use AnalysisUtils for analysis
    await this.analysisUtils.analyzeModels();
    await this.analysisUtils.analyzeApiEndpoints();
    await this.analysisUtils.analyzeUIComponents();
    await this.analysisUtils.analyzeTests();

    // Use AnalysisUtils for code quality analysis too
    await this.analysisUtils.analyzeCodeQuality(this.astAnalyzer);

    // Generate relationship maps with Mermaid diagrams
    await this.generateRelationshipMaps();

    // Make completion predictions (use only this one)
    this.predictor.predictCompletion();

    // Update project status
    this.updateProjectStatus();

    // Generate central reference hub (new)
    await this.generateCentralReferenceHub();

    this.printSummary();
    return this.metrics;
  }

  // File system related methods (discoverFiles, readFileContents, indexFileKeywords, findFilesByPatterns, getDirectoryStats) are removed.

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
      techDebt: `${this.metrics.codeQuality.techDebt.score}%` // Use calculated score
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
   * Generate detailed section for reports
   */
  generateDetailedSection() {
    let details = "## Implementation Details\n\n";

    // Models
    details += `### Models (${this.getPercentage(this.metrics.models.implemented, this.metrics.models.total)}% Complete)\n\n`;
    details += "| Model | File | Completeness |\n";
    details += "|-------|------|-------------|\n";
    this.metrics.models.details
        .sort((a, b) => a.name.localeCompare(b.name))
        .forEach(m => {
            details += `| ${m.name} | ${m.file.replace(/\\/g, '/')} | ${m.completeness}% ${m.completeness < 50 ? '⚠️ Low' : ''} |\n`;
        });
    details += "\n";

    // API Endpoints
    details += `### API Endpoints (${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}% Complete)\n\n`;
    details += "| Handler | File | Route | Completeness | Feature Area |\n";
    details += "|---------|------|-------|-------------|--------------|\n";
     this.metrics.apiEndpoints.details
        .sort((a, b) => (a.featureArea + a.name).localeCompare(b.featureArea + b.name))
        .forEach(e => {
            details += `| ${e.name} | ${e.file.replace(/\\/g, '/')} | ${e.routePath || '-'} | ${e.completeness}% ${e.completeness < 50 ? '⚠️ Low' : ''} | ${e.featureArea} |\n`;
        });
    details += "\n";

    // UI Components
    details += `### UI Components (${this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}% Complete)\n\n`;
    details += "| Component | File | Completeness |\n";
    details += "|-----------|------|-------------|\n";
    this.metrics.uiComponents.details
        .sort((a, b) => a.name.localeCompare(b.name))
        .forEach(c => {
            details += `| ${c.name} | ${c.file.replace(/\\/g, '/')} | ${c.completeness}% ${c.completeness < 50 ? '⚠️ Low' : ''} |\n`;
        });
    details += "\n";

     // Code Quality
     details += `### Code Quality Metrics\n\n`;
     details += `| Metric              | Value |\n`;
     details += `|---------------------|-------|\n`;
     details += `| Avg Complexity      | ${this.metrics.codeQuality.complexity.average.toFixed(1)} |\n`;
     details += `| High Complexity Files | ${this.metrics.codeQuality.complexity.high} |\n`;
     // details += `| Duplication Count   | ${this.metrics.codeQuality.duplications.count} |\n`; // Add if implemented
     // details += `| Duplicated Lines    | ${this.metrics.codeQuality.duplications.lines} |\n`; // Add if implemented
     details += `| Technical Debt Score| ${this.metrics.codeQuality.techDebt.score}% |\n`;
     details += "\n";

     if (this.metrics.codeQuality.techDebt.items.length > 0) {
         details += `#### Top Technical Debt Items\n\n`;
         details += `| File | Issue | Complexity/Score | Recommendation |\n`;
         details += `|------|-------|-----------------|----------------|\n`;
         this.metrics.codeQuality.techDebt.items
             .sort((a, b) => b.score - a.score) // Sort by score descending
             .slice(0, 10) // Show top 10
             .forEach(item => {
                 details += `| ${item.file.replace(/\\/g, '/')} | ${item.issue} | ${item.score} | ${item.recommendation} |\n`;
             });
         details += "\n";
     }


    return details;
  }

  /**
   * Get the feature area with the lowest implementation percentage
   */
  getLowestImplementedArea() {
    let lowestPercent = 101;
    let lowestArea = 'N/A';

    for (const area in this.metrics.featureAreas) {
      const { implemented, total } = this.metrics.featureAreas[area];
      const percent = this.getPercentage(implemented, total);
      if (total > 0 && percent < lowestPercent) {
        lowestPercent = percent;
        lowestArea = area;
      }
    }
    return lowestArea;
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
    
    // Detect code smells related to SOLID principles
    await this.detectCodeSmells();
    
    // Delegate the relationship detection to analysisUtils
    if (this.analysisUtils.findModelRelationships) {
      await this.analysisUtils.findModelRelationships();
    }
    
    // Rest of the method can stay as is since it's just visualization
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
     // Add styles for nodes (optional)
     nodes.forEach(node => {
         const model = this.metrics.models.details.find(m => m.name === node);
         const completeness = model ? model.completeness : 0;
         let style = 'fill:#eee,stroke:#333,stroke-width:1px';
         if (completeness >= 75) style = 'fill:#c8e6c9,stroke:#2e7d32,stroke-width:2px'; // Green
         else if (completeness >= 40) style = 'fill:#fff9c4,stroke:#fbc02d,stroke-width:1px'; // Yellow
         else if (completeness > 0) style = 'fill:#ffcdd2,stroke:#c62828,stroke-width:1px'; // Red
         mermaidDiagram += `  style ${node} ${style}\n`;
     });


    // Save to a file (e.g., docs/relationship_map.md)
    const mapContent = `# Model Relationship Map\n\n\`\`\`mermaid\n${mermaidDiagram}\n\`\`\`\n`;
    try {
        fs.writeFileSync(path.join(this.baseDir, 'docs', 'relationship_map.md'), mapContent);
        console.log("Relationship map saved to docs/relationship_map.md");
    } catch (err) {
        console.error("Error saving relationship map:", err.message);
    }
  }


  /**
   * Determine feature area based on file path or name
   */
  determineApiFeatureArea(name = '', filePath = '', routePath = '') {
    return this.analysisUtils.determineApiFeatureArea(name, filePath, routePath);
  }

  /**
   * Add an API endpoint to metrics
   */
  addApiEndpoint(name, filePath, completeness, featureArea = 'other', routePath = null) {
    return this.analysisUtils.addApiEndpoint(name, filePath, completeness, featureArea, routePath);
  }

  /**
   * Print a summary of the analysis results
   */
  printSummary() {
    console.log("\n--- Analysis Summary ---");
    console.log(`Models: ${this.metrics.models.implemented}/${this.metrics.models.total} (${this.getPercentage(this.metrics.models.implemented, this.metrics.models.total)}%)`);
    console.log(`API Endpoints: ${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total} (${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}%)`);
    console.log(`UI Components: ${this.metrics.uiComponents.implemented}/${this.metrics.uiComponents.total} (${this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}%)`);
    console.log(`Tests: ${this.metrics.tests.total} (Coverage: ${this.metrics.tests.coverage}%)`);
    console.log(`Code Quality: Avg Complexity=${this.metrics.codeQuality.complexity.average.toFixed(1)}, High Complexity Files=${this.metrics.codeQuality.complexity.high}, Tech Debt=${this.metrics.codeQuality.techDebt.score}%`);
    console.log(`Overall Phase: ${this.metrics.overallPhase}`);
    console.log("----------------------\n");
  }

  /**
   * Calculate percentage safely
   */
  getPercentage(value, total) {
    if (total === 0) return 0;
    return Math.round((value / total) * 100);
  }

  /**
   * Parse code content to AST, handling potential errors
   */
  parseToAst(content, filePath = 'unknown') {
    return this.astAnalyzer.parseToAst(content, filePath);
  }

  /**
   * Calculate Cyclomatic Complexity using AST
   */
  calculateComplexity(ast) {
    return this.astAnalyzer.calculateComplexity(ast);
  }

  /**
   * Analyze AST for component details (props, hooks, state, handlers)
   */
  analyzeComponentAst(content, filePath) {
    return this.astAnalyzer.analyzeComponentAst(content, filePath);
  }

  /**
   * Generate the Central Reference Hub Markdown file
   */
  async generateCentralReferenceHub() {
    console.log("Generating Central Reference Hub...");
    const hubPath = path.join(this.baseDir, 'docs', 'central_reference_hub.md');

    // --- Project Overview ---
    const overview = {
        overall_status: this.metrics.overallPhase,
        project_stats: {
            foundation_complete: this.metrics.overallPhase !== 'planning', // Basic check
            model_implementation: this.getPercentage(this.metrics.models.implemented, this.metrics.models.total) + '%',
            api_implementation: this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total) + '%',
            ui_implementation: this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total) + '%',
            test_coverage: this.metrics.tests.coverage + '%',
            technical_debt: this.metrics.codeQuality.techDebt.score + '%'
        },
        // Add source/target system info if available/needed
        target_system: {
            code_location: this.baseDir,
            // Add stack info if detectable or configured
        },
        completion_forecasts: {
            models: this.metrics.predictions.estimates.models.date,
            api_endpoints: this.metrics.predictions.estimates.apiEndpoints.date,
            ui_components: this.metrics.predictions.estimates.uiComponents.date,
            entire_project: this.metrics.predictions.estimates.project.date
        }
    };

    // --- Source-to-Target Mapping (Placeholder/Example) ---
    // This would ideally come from a configuration file or more advanced analysis
    const mappingTable = `| Component | Source System | Source Location | Target Location | Status | Priority |
|-----------|---------------|-----------------|-----------------|--------|----------|
| User Model | Both | \`canvas/.../user.rb\` + \`discourse/.../user.rb\` | \`src-tauri/src/models/user.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='User')?.completeness || 0}% | High |
| Forum Topics | Discourse | \`discourse/.../topic.rb\` | \`src-tauri/src/models/topic.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='Topic')?.completeness || 0}% | High |
| Forum Posts | Discourse | \`discourse/.../post.rb\` | \`src-tauri/src/models/post.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='Post')?.completeness || 0}% | High |
| Courses | Canvas | \`canvas/.../course.rb\` | \`src-tauri/src/models/course.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='Course')?.completeness || 0}% | High |
| Forum API | Discourse | \`discourse/.../topics_controller.rb\` | \`src-tauri/src/api/forum.rs\` | ❌ ${this.getPercentage(this.metrics.featureAreas.forum.implemented, this.metrics.featureAreas.forum.total)}% | High |
| Course API | Canvas | \`canvas/.../courses_controller.rb\` | \`src-tauri/src/api/lms/courses.rs\` | ❌ ${this.getPercentage(this.metrics.featureAreas.lms.implemented, this.metrics.featureAreas.lms.total)}% | High |
| UI Components | Both | Multiple files | \`src/components/\` | ✅ ${this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}% | High |
`; // Add more mappings as needed

    // --- Integration Conflicts (Placeholder) ---
    const conflicts = {
        model_conflicts: [ /* ... populate based on analysis ... */ ],
        route_conflicts: [ /* ... populate based on analysis ... */ ]
    };

    // --- Implementation Tasks (Placeholder) ---
    // Prioritize based on missing features or low completeness
    const tasks = [
        `1. **Complete API Endpoint Implementation** (${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total} completed)`,
        `   - High Priority: Focus on areas like '${this.getLowestImplementedArea()}'`,
        `2. **Complete UI Component Implementation** (${this.metrics.uiComponents.implemented}/${this.metrics.uiComponents.total} completed)`,
        `   - Implement components corresponding to new API endpoints`,
        `3. **Address Technical Debt** (Score: ${this.metrics.codeQuality.techDebt.score}%)`,
        `   - Refactor ${this.metrics.codeQuality.complexity.high} high complexity files`,
        `   - Improve test coverage (currently ${this.metrics.tests.coverage}%)`,
        `4. **Integrate Key Systems** (e.g., Search, Notifications - if applicable)`
    ];


    // --- Directory Structure ---
    // Use fsUtils project structure data
    let dirStructure = "```\n/\n";
    const sortedDirs = [...this.fsUtils.getProjectStructure().directories].sort();
    const topLevelDirs = sortedDirs.filter(d => !d.includes('/') && !d.includes('\\'));
    const structureMap = {};

     // Build nested structure
     sortedDirs.forEach(dir => {
         const parts = dir.split(/[\\\/]/);
         let currentLevel = structureMap;
         parts.forEach(part => {
             if (!currentLevel[part]) {
                 currentLevel[part] = {};
             }
             currentLevel = currentLevel[part];
         });
     });

     const buildStructureString = (level, indent = ' ') => {
         let str = '';
         Object.keys(level).sort().forEach(key => {
             const dirPath = indent.substring(1).replace(/ /g, '/').replace('├──', '').replace('└──', '').trim() + key; // Approximate path
             const category = this.getDirCategory(dirPath);
             const categoryLabel = category ? ` # ${category.charAt(0).toUpperCase() + category.slice(1)}` : '';
             str += `${indent}├── ${key}/${categoryLabel}\n`;
             str += buildStructureString(level[key], indent + '│  ');
         });
         return str.replace(/├──(?=[^├──]*$)/, '└──'); // Fix last item marker
     };

     dirStructure += buildStructureString(structureMap);
     dirStructure += "```\n";


    // --- Implementation Details Table ---
    const detailsSection = this.generateDetailedSection(); // Use existing method

    // --- SOLID Violations Table ---
    let solidViolationsSection = `## 📊 SOLID Principles Violations\n\n`;

    // SRP violations
    const srpViolations = this.metrics.codeQuality.solidViolations?.srp || [];
    solidViolationsSection += `### Single Responsibility Principle (${srpViolations.length} violations)\n\n`;

    if (srpViolations.length > 0) {
      solidViolationsSection += `| Component | File | Score | Details |\n`;
      solidViolationsSection += `|-----------|------|-------|--------|\n`;
      
      srpViolations
        .sort((a, b) => b.score - a.score)
        .slice(0, 5) // Show top 5 violations
        .forEach(v => {
          solidViolationsSection += `| ${v.name} | ${v.file.replace(/\\/g, '/')} | ${v.score} | ${v.details} |\n`;
        });
      
      if (srpViolations.length > 5) {
        solidViolationsSection += `\n_...and ${srpViolations.length - 5} more violations. See full report in docs/solid_code_smells.md_\n`;
      }
    } else {
      solidViolationsSection += `No SRP violations detected.\n`;
    }

    // Add other SOLID principles here as they're implemented

    // --- SOLID Violations Summary ---
    solidViolationsSection += `## 📊 SOLID Principles Violations\n\n`;

    const solidViolations = this.metrics.codeQuality.solidViolations;
    const totalViolations = 
      (solidViolations.srp?.length || 0) + 
      (solidViolations.ocp?.length || 0) + 
      (solidViolations.lsp?.length || 0) + 
      (solidViolations.isp?.length || 0) + 
      (solidViolations.dip?.length || 0);

    solidViolationsSection += `| Principle | Violations | Most Affected Component |\n`;
    solidViolationsSection += `|-----------|------------|------------------------|\n`;

    // Helper to get the most problematic component
    const getMostProblematicComponent = (violations) => {
      if (!violations || violations.length === 0) return '-';
      return violations.sort((a, b) => b.score - a.score)[0].name;
    };

    solidViolationsSection += `| Single Responsibility | ${solidViolations.srp?.length || 0} | ${getMostProblematicComponent(solidViolations.srp)} |\n`;
    solidViolationsSection += `| Open-Closed | ${solidViolations.ocp?.length || 0} | ${getMostProblematicComponent(solidViolations.ocp)} |\n`;
    solidViolationsSection += `| Liskov Substitution | ${solidViolations.lsp?.length || 0} | ${getMostProblematicComponent(solidViolations.lsp)} |\n`;
    solidViolationsSection += `| Interface Segregation | ${solidViolations.isp?.length || 0} | ${getMostProblematicComponent(solidViolations.isp)} |\n`;
    solidViolationsSection += `| Dependency Inversion | ${solidViolations.dip?.length || 0} | ${getMostProblematicComponent(solidViolations.dip)} |\n`;

    solidViolationsSection += `\n*For detailed analysis, see [SOLID Code Smells Report](docs/solid_code_smells.md)*\n\n`;


    // --- Assemble Hub Content ---
    const hubContent = `# LMS Integration Project - Central Reference Hub

_Last updated: ${new Date().toISOString().split('T')[0]}_

## 📊 Project Overview

\`\`\`json
${JSON.stringify(overview, null, 2)}
\`\`\`

## 🔄 Source-to-Target Mapping

${mappingTable}

## 🔍 Integration Conflicts (Placeholder)

\`\`\`json
${JSON.stringify(conflicts, null, 2)}
\`\`\`

## 📋 Implementation Tasks

${tasks.join('\n')}

## 📁 Project Directory Structure

${dirStructure}

${detailsSection}

${solidViolationsSection}

## 📈 Project Trajectories (Predictions)

\`\`\`json
${JSON.stringify(this.metrics.predictions.estimates, null, 2)}
\`\`\`
`;

    // --- Write to File ---
    try {
      fs.writeFileSync(hubPath, hubContent);
      console.log(`Central Reference Hub saved to ${hubPath}`);
    } catch (err) {
      console.error("Error saving Central Reference Hub:", err.message);
    }
  }

   /** Helper to get directory category */
   getDirCategory(dirPath) {
     return this.fsUtils.getDirCategory(dirPath);
   }

  /**
   * Add a model to metrics - delegates to analysisUtils
   */
  addModel(name, filePath, completeness) {
    return this.analysisUtils.addModel(name, filePath, completeness);
  }

  /**
   * Add an API endpoint to metrics - delegates to analysisUtils
   */
  addApiEndpoint(name, filePath, completeness, featureArea = 'other', routePath = null) {
    return this.analysisUtils.addApiEndpoint(name, filePath, completeness, featureArea, routePath);
  }

  /**
   * Add a UI component to metrics - delegates to analysisUtils
   */
  addUIComponent(name, filePath, completeness) {
    return this.analysisUtils.addUIComponent(name, filePath, completeness);
  }

  /**
   * Add a test to metrics - delegates to analysisUtils
   */
  addTest(name, filePath, passing = false) {
    return this.analysisUtils.addTest(name, filePath, passing);
  }

  /**
   * Detect code smells related to SOLID principles
   */
  async detectCodeSmells() {
    console.log("Analyzing code for SOLID principles and design patterns...");
    
    // Initialize code smells array in metrics if not exists
    if (!this.metrics.codeQuality.solidViolations) {
      this.metrics.codeQuality.solidViolations = {
        srp: [],
        ocp: [],
        lsp: [],
        isp: [],
        dip: []
      };
    }
    
    // Initialize design patterns tracking
    if (!this.metrics.codeQuality.designPatterns) {
      this.metrics.codeQuality.designPatterns = {
        polymorphism: { implementations: [], violations: [] },
        dependencyInjection: { implementations: [], violations: [] },
        ioc: { implementations: [], violations: [] }
      };
    }
    
    // Process all JavaScript/TypeScript files
    const jsFiles = this.fsUtils.filterFiles(/\.(js|jsx|ts|tsx)$/);
    for (const file of jsFiles) {
      const content = this.fsUtils.getFileContent(file);
      if (!content) continue;
      
      try {
        // Parse the file to an AST
        const ast = this.parseToAst(content, file);
        if (!ast) continue;
        
        // Detect violations for each SOLID principle
        this.detectSRPViolations(ast, file);
        this.detectOCPViolations(ast, file);
        this.detectLSPViolations(ast, file);
        this.detectISPViolations(ast, file);
        this.detectDIPViolations(ast, file);
        
        // Detect patterns and anti-patterns
        this.detectPolymorphism(ast, file);
        this.detectDependencyInjection(ast, file);
        this.detectIoC(ast, file);
      } catch (error) {
        console.error(`Error analyzing ${file} for code smells:`, error.message);
      }
    }
    
    // Process all Rust files
    const rustFiles = this.fsUtils.filterFiles(/\.rs$/);
    for (const file of rustFiles) {
      const content = this.fsUtils.getFileContent(file);
      if (!content) continue;
      
      // For Rust files, we can only detect some principles with regex
      this.detectRustSRPViolations(content, file);
      // Add more Rust-specific detectors if needed
    }
    
    // Generate a code smells report
    await this.generateCodeSmellsReport();
  }

  /**
   * Detect Single Responsibility Principle violations in JavaScript/TypeScript code
   */
  detectSRPViolations(ast, filePath) {
    const violations = [];
    
    // Function to calculate responsibility score based on various metrics
    const calculateResponsibilityScore = (node) => {
      // Metrics that indicate multiple responsibilities
      let distinctConcerns = 0;
      let mixedFunctionality = false;
      let highComplexity = false;
      let tooManyMethods = false;
      let tooManyDependencies = false;
      let methodsWithDifferentPrefixes = new Set();
      
      // Get class/function name
      let name = "anonymous";
      if (node.id && node.id.name) {
        name = node.id.name;
      }
      
      // Count methods if it's a class
      let methods = [];
      let dependenciesCount = 0;
      
      // For classes, check methods and their prefixes
      if (node.type === 'ClassDeclaration') {
        // Extract methods
        methods = node.body.body.filter(item => 
          item.type === 'ClassMethod' || item.type === 'MethodDefinition'
        );
        
        // Count dependencies (constructor parameters or class properties that look like services)
        const constructor = node.body.body.find(item => 
          (item.type === 'ClassMethod' || item.type === 'MethodDefinition') && 
          item.key.name === 'constructor'
        );
        
        if (constructor && constructor.params) {
          dependenciesCount = constructor.params.length;
        }
        
        // Check if methods have different prefixes (indicating different responsibilities)
        methods.forEach(method => {
          if (method.key && method.key.name) {
            const methodName = method.key.name;
            if (methodName !== 'constructor') {
              // Extract prefix (e.g., "get" from "getUserData")
              const prefix = methodName.match(/^([a-z]+)[A-Z]/);
              if (prefix && prefix[1]) {
                methodsWithDifferentPrefixes.add(prefix[1]);
              }
            }
          }
        });
        
        // Too many methods is a code smell
        tooManyMethods = methods.length > 10;
        
        // Too many dependencies might indicate too many responsibilities
        tooManyDependencies = dependenciesCount > 5;
        
        // Different method prefixes might indicate different responsibilities
        distinctConcerns = methodsWithDifferentPrefixes.size;
        
        // More than 3 different concerns is a code smell
        mixedFunctionality = distinctConcerns > 3;
      }
      
      // For functions, calculate complexity
      if (node.type === 'FunctionDeclaration') {
        const complexity = this.calculateComplexity({ program: { body: [node] } });
        highComplexity = complexity > 10;
      }
      
      // Calculate overall score (0-100, higher is worse)
      let score = 0;
      if (mixedFunctionality) score += 30;
      if (highComplexity) score += 25;
      if (tooManyMethods) score += 20;
      if (tooManyDependencies) score += 15;
      score += (distinctConcerns * 5);
      
      return {
        name,
        score,
        distinctConcerns,
        methods: methods.length,
        dependencies: dependenciesCount,
        recommendation: score > 40 ? 'Consider splitting this into multiple classes/functions with single responsibilities' : null
      };
    };
    
    // Visit classes and functions in the AST
    traverse(ast, {
      ClassDeclaration(path) {
        const result = calculateResponsibilityScore(path.node);
        if (result.score > 40) {
          violations.push({
            type: 'SRP',
            file: filePath,
            name: result.name,
            score: result.score,
            details: `Class has ${result.methods} methods with ${result.distinctConcerns} distinct concerns`,
            recommendation: result.recommendation
          });
        }
      },
      
      FunctionDeclaration(path) {
        const result = calculateResponsibilityScore(path.node);
        if (result.score > 40) {
          violations.push({
            type: 'SRP',
            file: filePath,
            name: result.name,
            score: result.score,
            details: 'Function has too many responsibilities or is too complex',
            recommendation: result.recommendation
          });
        }
      }
    });
    
    // Add violations to metrics
    if (violations.length > 0) {
      this.metrics.codeQuality.solidViolations.srp.push(...violations);
      
      // Also add to technical debt
      violations.forEach(v => {
        this.metrics.codeQuality.techDebt.items.push({
          file: v.file,
          issue: `SRP Violation: ${v.details}`,
          score: v.score,
          recommendation: v.recommendation
        });
      });
    }
  }

  /**
   * Detect Single Responsibility Principle violations in Rust code using regex
   */
  detectRustSRPViolations(content, filePath) {
    // Simple heuristics for Rust files
    const violations = [];
    
    // Get struct name from content
    const structMatch = content.match(/struct\s+(\w+)/);
    let name = structMatch ? structMatch[1] : "unknown";
    
    // Count impl blocks for the struct (might indicate multiple responsibilities)
    const implCount = (content.match(new RegExp(`impl\\s+${name}`, 'g')) || []).length;
    
    // Count functions in the impl blocks
    const functionMatches = content.match(/fn\s+\w+/g) || [];
    const functionCount = functionMatches.length;
    
    // Try to detect different method prefixes
    const methodPrefixes = new Set();
    functionMatches.forEach(fn => {
      const match = fn.match(/fn\s+([a-z]+)_/);
      if (match && match[1]) {
        methodPrefixes.add(match[1]);
      }
    });
    
    // Calculate a simple score
    let score = 0;
    if (implCount > 3) score += 20;
    if (functionCount > 10) score += 20;
    if (methodPrefixes.size > 3) score += 30;
    if (content.length > 500) score += Math.min(30, content.length / 100);
    
    if (score > 40) {
      violations.push({
        type: 'SRP',
        file: filePath,
        name: name,
        score: score,
        details: `Struct has ${functionCount} methods with ${methodPrefixes.size} distinct prefixes across ${implCount} impl blocks`,
        recommendation: 'Consider splitting this struct into multiple structs with single responsibilities'
      });
      
      // Add to metrics
      this.metrics.codeQuality.solidViolations.srp.push(...violations);
      
      // Also add to technical debt
      violations.forEach(v => {
        this.metrics.codeQuality.techDebt.items.push({
          file: v.file,
          issue: `SRP Violation: ${v.details}`,
          score: v.score,
          recommendation: v.recommendation
        });
      });
    }
  }

  /**
   * Detect Open-Closed Principle violations in JavaScript/TypeScript code
   */
  detectOCPViolations(ast, filePath) {
    const violations = [];
    
    // Visit classes and track inheritance/extension patterns
    traverse(ast, {
      ClassDeclaration(path) {
        const className = path.node.id?.name || 'Anonymous';
        
        // Check for large switch statements or if/else chains that could indicate OCP violations
        let largeConditionalBlocks = [];
        
        // Find methods with switch statements or long if-else chains
        path.traverse({
          SwitchStatement(switchPath) {
            // Count cases in switch statement
            const caseCount = switchPath.node.cases?.length || 0;
            if (caseCount > 3) {
              // Get the parent function or method name
              let methodName = 'unknown';
              let parentFunc = switchPath.findParent(p => p.isClassMethod() || p.isFunctionDeclaration());
              if (parentFunc && parentFunc.node.key) {
                methodName = parentFunc.node.key.name;
              }
              
              largeConditionalBlocks.push({
                type: 'switch',
                caseCount,
                methodName,
                loc: switchPath.node.loc
              });
            }
          },
          
          // Track long if-else chains
          IfStatement(ifPath) {
            let chainLength = 1;
            let current = ifPath;
            
            // Count consecutive else-if statements
            while (current.node.alternate && current.node.alternate.type === 'IfStatement') {
              chainLength++;
              current = current.get('alternate');
            }
            
            if (current.node.alternate) {
              chainLength++; // Count the final else
            }
            
            if (chainLength > 3) {
              // Get the parent function or method name
              let methodName = 'unknown';
              let parentFunc = ifPath.findParent(p => p.isClassMethod() || p.isFunctionDeclaration());
              if (parentFunc && parentFunc.node.key) {
                methodName = parentFunc.node.key.name;
              }
              
              largeConditionalBlocks.push({
                type: 'if-else',
                chainLength,
                methodName,
                loc: ifPath.node.loc
              });
            }
          }
        });
        
        // If we found OCP violations, add them
        if (largeConditionalBlocks.length > 0) {
          // Calculate a score based on the number and size of conditional blocks
          const score = Math.min(100, largeConditionalBlocks.reduce(
            (sum, block) => sum + (block.type === 'switch' ? block.caseCount * 5 : block.chainLength * 7), 
            20
          ));
          
          const details = largeConditionalBlocks.map(block => 
            `${block.type === 'switch' ? 'Switch with' : 'If-else chain with'} ${block.type === 'switch' ? block.caseCount : block.chainLength} cases in method ${block.methodName}`
          ).join('; ');
          
          violations.push({
            type: 'OCP',
            file: filePath,
            name: className,
            score,
            details: `Potential violation with conditional logic: ${details}`,
            recommendation: 'Consider using polymorphism or the Strategy pattern instead of conditional logic'
          });
        }
      }
    });
    
    // Add violations to metrics
    if (violations.length > 0) {
      this.metrics.codeQuality.solidViolations.ocp.push(...violations);
      
      // Also add to technical debt
      violations.forEach(v => {
        this.metrics.codeQuality.techDebt.items.push({
          file: v.file,
          issue: `OCP Violation: ${v.details}`,
          score: v.score,
          recommendation: v.recommendation
        });
      });
    }
  }

/**
 * Detect Liskov Substitution Principle violations in JavaScript/TypeScript code
 */
detectLSPViolations(ast, filePath) {
  const violations = [];
  const classHierarchy = new Map(); // Track class inheritance
  
  // First pass: build class hierarchy
  traverse(ast, {
    ClassDeclaration(path) {
      const className = path.node.id?.name || 'Anonymous';
      const superClassName = path.node.superClass?.name || null;
      
      // Store the class info
      classHierarchy.set(className, {
        superClass: superClassName,
        methods: new Map(), // Will store method signatures
        properties: new Set(), // Will store property names
      });
      
      // Extract methods and their parameters
      path.node.body.body.forEach(member => {
        if (member.type === 'ClassMethod' || member.type === 'MethodDefinition') {
          const methodName = member.key.name;
          const params = member.params.map(p => p.type);
          classHierarchy.get(className).methods.set(methodName, params);
        } 
        // Extract properties (for TypeScript classes with property declarations)
        else if (member.type === 'ClassProperty') {
          const propName = member.key.name;
          classHierarchy.get(className).properties.add(propName);
        }
      });
    }
  });
  
  // Second pass: check for LSP violations in subclasses
  for (const [className, classInfo] of classHierarchy.entries()) {
    if (!classInfo.superClass) continue; // Skip if not a subclass
    
    const superClassInfo = classHierarchy.get(classInfo.superClass);
    if (!superClassInfo) continue; // Super class not found in this file
    
    // Check for method signature changes
    for (const [methodName, superParams] of superClassInfo.methods.entries()) {
      // If subclass overrides the method
      if (classInfo.methods.has(methodName)) {
        const subParams = classInfo.methods.get(methodName);
        
        // Check if parameter count is different (potential LSP violation)
        if (subParams.length !== superParams.length) {
          violations.push({
            type: 'LSP',
            file: filePath,
            name: className,
            score: 60,
            details: `Method ${methodName} changes parameter count from parent class ${classInfo.superClass}`,
            recommendation: 'Ensure subclass methods maintain the same signature as parent class methods'
          });
        }
      }
    }
  }
  
  // Add violations to metrics
  if (violations.length > 0) {
    this.metrics.codeQuality.solidViolations.lsp.push(...violations);
    
    // Also add to technical debt
    violations.forEach(v => {
      this.metrics.codeQuality.techDebt.items.push({
        file: v.file,
        issue: `LSP Violation: ${v.details}`,
        score: v.score,
        recommendation: v.recommendation
      });
    });
  }
}

  /**
   * Detect Interface Segregation Principle violations in JavaScript/TypeScript code
   */
  detectISPViolations(ast, filePath) {
    const violations = [];
    
    // For JavaScript, we can look for objects with too many properties
    // or classes with many methods that aren't fully utilized by clients
    
    traverse(ast, {
      // Look for large interface-like objects or classes
      ObjectExpression(path) {
        const properties = path.node.properties.length;
        
        // If the object has too many properties, it might violate ISP
        if (properties > 10) {
          let objectName = "Anonymous";
          
          // Try to get the variable name if it's part of a variable declaration
          const varDecl = path.findParent(p => p.isVariableDeclarator());
          if (varDecl && varDecl.node.id) {
            objectName = varDecl.node.id.name;
          }
          
          violations.push({
            type: 'ISP',
            file: filePath,
            name: objectName,
            score: Math.min(80, properties * 3),
            details: `Large object with ${properties} properties might be violating ISP`,
            recommendation: 'Consider splitting this object into smaller, more focused interfaces'
          });
        }
      },
      
      // Check for classes with many methods that could be split
      ClassDeclaration(path) {
        const className = path.node.id?.name || 'Anonymous';
        
        // Count public methods (potential interface methods)
        const methods = path.node.body.body.filter(member => 
          (member.type === 'ClassMethod' || member.type === 'MethodDefinition') &&
          (!member.accessibility || member.accessibility === 'public')
        );
        
        // Group methods by prefix to detect potential interfaces
        const methodPrefixes = new Map();
        methods.forEach(method => {
          const methodName = method.key.name;
          if (methodName === 'constructor') return;
          
          // Extract prefix (e.g., "get" from "getUserData")
          const prefix = methodName.match(/^([a-z]+)[A-Z]/);
          if (prefix && prefix[1]) {
            if (!methodPrefixes.has(prefix[1])) {
              methodPrefixes.set(prefix[1], []);
            }
            methodPrefixes.get(prefix[1]).push(methodName);
          }
        });
        
        // If we have multiple method groups and many methods overall, suggest interface segregation
        if (methods.length > 8 && methodPrefixes.size > 2) {
          const details = Array.from(methodPrefixes.entries())
            .map(([prefix, methods]) => `${prefix}* methods (${methods.length})`)
            .join(', ');
            
          violations.push({
            type: 'ISP',
            file: filePath,
            name: className,
            score: Math.min(80, methods.length * 4),
            details: `Class with ${methods.length} methods contains multiple responsibilities: ${details}`,
            recommendation: 'Consider splitting this class into multiple interfaces based on method groups'
          });
        }
      }
    });
    
    // Add violations to metrics
    if (violations.length > 0) {
      this.metrics.codeQuality.solidViolations.isp.push(...violations);
      
      // Also add to technical debt
      violations.forEach(v => {
        this.metrics.codeQuality.techDebt.items.push({
          file: v.file,
          issue: `ISP Violation: ${v.details}`,
          score: v.score,
          recommendation: v.recommendation
        });
      });
    }
  }

  /**
   * Detect Dependency Inversion Principle violations in JavaScript/TypeScript code
   */
  detectDIPViolations(ast, filePath) {
    const violations = [];
    
    // Look for concrete class instantiations in constructors
    traverse(ast, {
      ClassDeclaration(path) {
        const className = path.node.id?.name || 'Anonymous';
        const concreteInstantiations = [];
        
        // Find the constructor
        const constructor = path.node.body.body.find(
          node => node.type === 'ClassMethod' && node.key.name === 'constructor'
        );
        
        if (!constructor) return;
        
        // Look for 'new' expressions in the constructor
        path.traverse({
          NewExpression(newExprPath) {
            // Only check news in the constructor
            const isInConstructor = newExprPath.findParent(
              p => p.isClassMethod() && p.node.key.name === 'constructor'
            );
            
            if (isInConstructor) {
              const concreteName = newExprPath.node.callee.name;
              if (concreteName) {
                concreteInstantiations.push(concreteName);
              }
            }
          }
        });
        
        // If we found concrete instantiations, report DIP violation
        if (concreteInstantiations.length > 0) {
          violations.push({
            type: 'DIP',
            file: filePath,
            name: className,
            score: 50 + (concreteInstantiations.length * 10),
            details: `Class directly instantiates concrete classes in constructor: ${concreteInstantiations.join(', ')}`,
            recommendation: 'Use dependency injection instead of direct instantiation'
          });
        }
      }
    });
    
    // Add violations to metrics
    if (violations.length > 0) {
      this.metrics.codeQuality.solidViolations.dip.push(...violations);
      
      // Also add to technical debt
      violations.forEach(v => {
        this.metrics.codeQuality.techDebt.items.push({
          file: v.file,
          issue: `DIP Violation: ${v.details}`,
          score: v.score,
          recommendation: v.recommendation
        });
      });
    }
  }

/**
 * Detect polymorphism usage and violations in code
 */
detectPolymorphism(ast, filePath) {
  const polymorphismImplementations = [];
  const polymorphismViolations = [];
  
  // Track class hierarchy for polymorphism analysis
  const classHierarchy = new Map();
  
  // First pass: build class hierarchy
  traverse(ast, {
    ClassDeclaration(path) {
      const className = path.node.id?.name || 'Anonymous';
      const superClassName = path.node.superClass?.name || null;
      
      if (!superClassName) return; // Not relevant for polymorphism if no parent
      
      // Store the class and its methods
      classHierarchy.set(className, {
        superClass: superClassName,
        methods: new Map(),
        overriddenMethods: new Set()
      });
      
      // Extract methods
      path.node.body.body.forEach(member => {
        if (member.type === 'ClassMethod' || member.type === 'MethodDefinition') {
          const methodName = member.key.name;
          classHierarchy.get(className).methods.set(methodName, member);
        }
      });
    }
  });
  
  // Second pass: analyze method overrides for polymorphism
  for (const [className, classInfo] of classHierarchy.entries()) {
    const superClassInfo = classHierarchy.get(classInfo.superClass);
    if (!superClassInfo) continue; // Super class not found in this file
    
    // Analyze methods that override parent class methods
    for (const [methodName, method] of classInfo.methods.entries()) {
      if (methodName === 'constructor') continue;
      
      // Check if this method exists in the parent class
      if (superClassInfo.methods.has(methodName)) {
        // This is an overridden method - good for polymorphism
        classInfo.overriddenMethods.add(methodName);
        
        polymorphismImplementations.push({
          file: filePath,
          name: `${className}.${methodName}`,
          details: `Method ${methodName} in class ${className} overrides parent class ${classInfo.superClass}`,
          type: 'method-override',
          score: 70 // Good score for proper polymorphism
        });
      }
    }
    
    // Check for typical polymorphism violations
    
    // 1. Check for instanceof/type checks (often a polymorphism violation)
    path.traverse({
      IfStatement(ifPath) {
        const test = ifPath.node.test;
        
        // Check for instanceof expressions
        if (test.type === 'BinaryExpression' && 
            (test.operator === 'instanceof' || 
             (test.operator === '===' && test.right.type === 'StringLiteral' && test.left.property?.name === 'name'))) {
          
          let methodName = 'unknown';
          const parentFunc = ifPath.findParent(p => p.isClassMethod() || p.isFunctionDeclaration());
          if (parentFunc?.node.key) {
            methodName = parentFunc.node.key.name;
          }
          
          polymorphismViolations.push({
            file: filePath,
            name: `${className}.${methodName}`,
            details: `Type checking with ${test.operator === 'instanceof' ? 'instanceof' : 'constructor.name'} instead of using polymorphism`,
            type: 'type-checking',
            score: 65,
            recommendation: 'Replace type checking with polymorphic method calls'
          });
        }
      }
    });
  }
  
  // Find usage of polymorphism (method calls on parent type variables)
  traverse(ast, {
    CallExpression(path) {
      // Look for obj.method() pattern where obj could be polymorphic
      if (path.node.callee.type === 'MemberExpression') {
        const methodName = path.node.callee.property.name;
        
        // Check if this methodName exists in multiple classes in our hierarchy
        let polymorphicMethodCount = 0;
        let implementingClasses = [];
        
        for (const [className, classInfo] of classHierarchy.entries()) {
          if (classInfo.overriddenMethods.has(methodName)) {
            polymorphicMethodCount++;
            implementingClasses.push(className);
          }
        }
        
        if (polymorphicMethodCount > 1) {
          polymorphismImplementations.push({
            file: filePath,
            name: methodName,
            details: `Method ${methodName} is potentially used polymorphically (implemented by ${implementingClasses.join(', ')})`,
            type: 'polymorphic-usage',
            score: 60
          });
        }
      }
    }
  });
  
  // Store results in metrics
  if (!this.metrics.codeQuality.designPatterns) {
    this.metrics.codeQuality.designPatterns = {
      polymorphism: { implementations: [], violations: [] },
      dependencyInjection: { implementations: [], violations: [] },
      ioc: { implementations: [], violations: [] }
    };
  }
  
  this.metrics.codeQuality.designPatterns.polymorphism.implementations.push(...polymorphismImplementations);
  this.metrics.codeQuality.designPatterns.polymorphism.violations.push(...polymorphismViolations);
  
  // Also add violations to technical debt
  polymorphismViolations.forEach(v => {
    this.metrics.codeQuality.techDebt.items.push({
      file: v.file,
      issue: `Polymorphism Violation: ${v.details}`,
      score: v.score,
      recommendation: v.recommendation || 'Use proper inheritance and method overriding'
    });
  });
}

/**
 * Detect dependency injection patterns and violations
 */
detectDependencyInjection(ast, filePath) {
  const diImplementations = [];
  const diViolations = [];
  
  // Look for constructor dependency injection pattern
  traverse(ast, {
    ClassDeclaration(path) {
      const className = path.node.id?.name || 'Anonymous';
      const constructor = path.node.body.body.find(
        node => node.type === 'ClassMethod' && node.key.name === 'constructor'
      );
      
      if (!constructor) return;
      
      // Check for dependencies passed to constructor
      const params = constructor.params || [];
      const dependencies = [];
      const savedDeps = new Set();
      
      // Look for dependencies saved to instance variables
      path.traverse({
        AssignmentExpression(assignPath) {
          // Check for this.something = param pattern
          if (assignPath.node.left.type === 'MemberExpression' && 
              assignPath.node.left.object.type === 'ThisExpression') {
            
            const varName = assignPath.node.left.property.name;
            
            // If right side is an identifier that matches a parameter
            if (assignPath.node.right.type === 'Identifier') {
              const paramName = assignPath.node.right.name;
              const paramIndex = params.findIndex(p => p.name === paramName);
              
              if (paramIndex >= 0) {
                savedDeps.add(paramName);
                dependencies.push({
                  param: paramName,
                  instanceVar: varName,
                  isService: varName.endsWith('Service') || 
                             varName.endsWith('Repository') || 
                             varName.endsWith('Manager') ||
                             varName.endsWith('Provider')
                });
              }
            }
          }
        }
      });
      
      // If we have dependencies, this might be DI
      if (dependencies.length > 0) {
        const serviceCount = dependencies.filter(d => d.isService).length;
        
        // Calculate a DI quality score
        const diScore = Math.min(100, 40 + (serviceCount * 10));
        
        diImplementations.push({
          file: filePath,
          name: className,
          details: `Class receives ${dependencies.length} dependencies via constructor (${serviceCount} likely services)`,
          type: 'constructor-injection',
          score: diScore,
          dependencies: dependencies.map(d => d.instanceVar)
        });
      }
      
      // Check for DI violations: instances created with 'new' inside class
      const newExpressions = [];
      path.traverse({
        NewExpression(newPath) {
          // Ignore 'new' for basic types (Date, Map, etc.)
          const basicTypes = ['Array', 'Object', 'Date', 'Map', 'Set', 'Promise', 'RegExp'];
          const className = newPath.node.callee.name;
          
          if (className && !basicTypes.includes(className)) {
            let methodName = 'unknown';
            const parentMethod = newPath.findParent(p => p.isClassMethod());
            if (parentMethod?.node.key) {
              methodName = parentMethod.node.key.name;
            }
            
            newExpressions.push({
              className,
              methodName
            });
          }
        }
      });
      
      // Report violations for service-looking classes created with 'new'
      newExpressions.forEach(expr => {
        if (expr.className.endsWith('Service') || 
            expr.className.endsWith('Repository') || 
            expr.className.endsWith('Manager') ||
            expr.className.endsWith('Factory') ||
            expr.className.endsWith('Provider')) {
          
          diViolations.push({
            file: filePath,
            name: `${className}.${expr.methodName}`,
            details: `Creates service '${expr.className}' with 'new' instead of using dependency injection`,
            type: 'new-service-instance',
            score: 75,
            recommendation: 'Inject this dependency through constructor instead of creating it directly'
          });
        }
      });
    }
  });
  
  // Store results in metrics
  this.metrics.codeQuality.designPatterns.dependencyInjection.implementations.push(...diImplementations);
  this.metrics.codeQuality.designPatterns.dependencyInjection.violations.push(...diViolations);
  
  // Also add violations to technical debt
  diViolations.forEach(v => {
    this.metrics.codeQuality.techDebt.items.push({
      file: v.file,
      issue: `DI Violation: ${v.details}`,
      score: v.score,
      recommendation: v.recommendation || 'Use proper dependency injection'
    });
  });
}

/**
 * Detect Inversion of Control patterns and violations
 */
detectIoC(ast, filePath) {
  const iocImplementations = [];
  const iocViolations = [];
  
  // Look for IoC container/registration patterns
  let hasIoCContainer = false;
  
  traverse(ast, {
    // Look for signs of an IoC container
    CallExpression(path) {
      const callee = path.node.callee;
      if (callee.type !== 'MemberExpression') return;
      
      const methodName = callee.property.name;
      const objectName = callee.object.name || 
                        (callee.object.type === 'MemberExpression' ? callee.object.property.name : '');
      
      // Common IoC container registration methods
      const iocRegisterMethods = [
        'register', 'registerSingleton', 'registerTransient', 'addSingleton', 
        'addTransient', 'bind', 'provide', 'service', 'factory'
      ];
      
      if (iocRegisterMethods.includes(methodName)) {
        hasIoCContainer = true;
        iocImplementations.push({
          file: filePath,
          name: objectName || 'IoC container',
          details: `IoC registration with '${methodName}'`,
          type: 'registration',
          score: 80
        });
      }
      
      // Common IoC container resolution methods
      const iocResolveMethods = [
        'resolve', 'get', 'getService', 'make', 'createInstance', 'inject'
      ];
      
      if (iocResolveMethods.includes(methodName)) {
        hasIoCContainer = true;
        iocImplementations.push({
          file: filePath,
          name: objectName || 'IoC container',
          details: `IoC service resolution with '${methodName}'`,
          type: 'resolution',
          score: 75
        });
      }
    },
    
    // IoC-related imports
    ImportDeclaration(path) {
      const source = path.node.source.value;
      
      // Common IoC libraries
      const iocLibraries = [
        'inversify', 'tsyringe', 'typedi', 'awilix', 'injection', 'di', 
        'container', 'service-locator', 'dependency-injection'
      ];
      
      if (iocLibraries.some(lib => source.includes(lib))) {
        hasIoCContainer = true;
        iocImplementations.push({
          file: filePath,
          name: source,
          details: `Using IoC library: ${source}`,
          type: 'library-usage',
          score: 90
        });
      }
    },
    
    // Look for decorators that might be IoC-related
    Decorator(path) {
      const expression = path.node.expression;
      let decoratorName;
      
      if (expression.type === 'Identifier') {
        decoratorName = expression.name;
      } else if (expression.type === 'CallExpression' && expression.callee.type === 'Identifier') {
        decoratorName = expression.callee.name;
      }
      
      // Common IoC decorators
      const iocDecorators = [
        'Injectable', 'Service', 'Inject', 'Singleton', 'Provides',
        'Component', 'Autowired', 'Dependency'
      ];
      
      if (decoratorName && iocDecorators.includes(decoratorName)) {
        hasIoCContainer = true;
        
        let targetName = 'unknown';
        const parent = path.parent;
        if (parent.type === 'ClassDeclaration' && parent.id) {
          targetName = parent.id.name;
        } else if (parent.type === 'ClassMethod' && parent.key) {
          targetName = parent.key.name;
        } else if (parent.type === 'ClassProperty' && parent.key) {
          targetName = parent.key.name;
        }
        
        iocImplementations.push({
          file: filePath,
          name: targetName,
          details: `IoC decorator: @${decoratorName}`,
          type: 'decorator',
          score: 90
        });
      }
    }
  });
  
  // Look for potential IoC violations
  // For example, explicit dependency instantiation in a file with IoC patterns
  if (hasIoCContainer) {
    traverse(ast, {
      NewExpression(path) {
        // Ignore 'new' for basic types (Date, Map, etc.)
        const basicTypes = ['Array', 'Object', 'Date', 'Map', 'Set', 'Promise', 'RegExp'];
        const className = path.node.callee.name;
        
        if (className && !basicTypes.includes(className) && 
           (className.endsWith('Service') || className.endsWith('Repository'))) {
          
          let methodName = 'unknown';
          const parentFunc = path.findParent(p => p.isFunction());
          if (parentFunc && parentFunc.node.id) {
            methodName = parentFunc.node.id.name;
          } else if (parentFunc && parentFunc.node.key) {
            methodName = parentFunc.node.key.name;
          }
          
          iocViolations.push({
            file: filePath,
            name: methodName,
            details: `Creates service '${className}' with 'new' while using IoC elsewhere`,
            type: 'inconsistent-instantiation',
            score: 70,
            recommendation: 'Resolve this dependency from the IoC container instead of creating it directly'
          });
        }
      }
    });
  }
  
  // Store results in metrics
  this.metrics.codeQuality.designPatterns.ioc.implementations.push(...iocImplementations);
  this.metrics.codeQuality.designPatterns.ioc.violations.push(...iocViolations);
  
  // Also add violations to technical debt
  iocViolations.forEach(v => {
    this.metrics.codeQuality.techDebt.items.push({
      file: v.file,
      issue: `IoC Violation: ${v.details}`,
      score: v.score,
      recommendation: v.recommendation || 'Use the IoC container consistently throughout the codebase'
    });
  });
}

  /**
   * Generate a report of code smells
   */
  async generateCodeSmellsReport() {
    console.log("Generating code smells report...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    // Get all violations
    const solidViolations = this.metrics.codeQuality.solidViolations;
    const designPatterns = this.metrics.codeQuality.designPatterns || {
      polymorphism: { implementations: [], violations: [] },
      dependencyInjection: { implementations: [], violations: [] },
      ioc: { implementations: [], violations: [] }
    };
    
    // Generate markdown report
    let reportContent = `# Code Quality Analysis Report\n\n`;
    
    // Function to create a section for SOLID principles
    const createPrincipleSection = (violations, title, description) => {
      let section = `## ${title}\n\n`;
      section += `${description}\n\n`;
      section += `Found **${violations.length}** potential violations.\n\n`;
      
      if (violations.length > 0) {
        section += `| File | Component | Score | Details | Recommendation |\n`;
        section += `|------|-----------|-------|---------|----------------|\n`;
        
        violations
          .sort((a, b) => b.score - a.score)
          .forEach(v => {
            section += `| ${v.file.replace(/\\/g, '/')} | ${v.name} | ${v.score} | ${v.details} | ${v.recommendation || '-'} |\n`;
          });
      }
      
      return section + "\n";
    };
    
    // Function to create a section for design patterns
    const createPatternSection = (pattern, title, description) => {
      const implementations = pattern.implementations || [];
      const violations = pattern.violations || [];
      
      let section = `## ${title}\n\n`;
      section += `${description}\n\n`;
      section += `Found **${implementations.length}** implementations and **${violations.length}** violations.\n\n`;
      
      if (implementations.length > 0) {
        section += `### Implementations\n\n`;
        section += `| File | Component | Type | Details |\n`;
        section += `|------|-----------|------|--------|\n`;
        
        implementations
          .sort((a, b) => b.score - a.score)
          .forEach(imp => {
            section += `| ${imp.file.replace(/\\/g, '/')} | ${imp.name} | ${imp.type} | ${imp.details} |\n`;
          });
        section += "\n";
      }
      
      if (violations.length > 0) {
        section += `### Violations\n\n`;
        section += `| File | Component | Details | Recommendation |\n`;
        section += `|------|-----------|---------|----------------|\n`;
        
        violations
          .sort((a, b) => b.score - a.score)
          .forEach(v => {
            section += `| ${v.file.replace(/\\/g, '/')} | ${v.name} | ${v.details} | ${v.recommendation || '-'} |\n`;
          });
      }
      
      return section + "\n";
    };
    
    // Add sections for each SOLID principle
    reportContent += `# SOLID Principles Analysis\n\n`;
    
    reportContent += createPrincipleSection(
      solidViolations.srp,
      "Single Responsibility Principle Violations",
      "A class should have only one reason to change."
    );
    
    reportContent += createPrincipleSection(
      solidViolations.ocp,
      "Open-Closed Principle Violations",
      "Software entities should be open for extension, but closed for modification."
    );
    
    reportContent += createPrincipleSection(
      solidViolations.lsp,
      "Liskov Substitution Principle Violations",
      "Subtypes must be substitutable for their base types."
    );
    
    reportContent += createPrincipleSection(
      solidViolations.isp,
      "Interface Segregation Principle Violations",
      "Clients should not be forced to depend on methods they do not use."
    );
    
    reportContent += createPrincipleSection(
      solidViolations.dip,
      "Dependency Inversion Principle Violations",
      "High-level modules should not depend on low-level modules. Both should depend on abstractions."
    );
    
    // Add sections for design patterns
    reportContent += `# Design Patterns Analysis\n\n`;
    
    reportContent += createPatternSection(
      designPatterns.polymorphism,
      "Polymorphism",
      "The ability to present the same interface for differing underlying forms (data types)."
    );
    
    reportContent += createPatternSection(
      designPatterns.dependencyInjection,
      "Dependency Injection",
      "A technique whereby one object supplies the dependencies of another object."
    );
    
    reportContent += createPatternSection(
      designPatterns.ioc,
      "Inversion of Control (IoC)",
      "A design principle in which control flow is inverted compared to traditional programming."
    );
    
    // Add a summary section
    const totalSolidViolations = solidViolations.srp.length + 
                            solidViolations.ocp.length + 
                            solidViolations.lsp.length +
                            solidViolations.isp.length +
                            solidViolations.dip.length;
                            
    const totalPatternViolations = designPatterns.polymorphism.violations.length +
                              designPatterns.dependencyInjection.violations.length +
                              designPatterns.ioc.violations.length;
                               
    const totalPatternImplementations = designPatterns.polymorphism.implementations.length +
                                   designPatterns.dependencyInjection.implementations.length +
                                   designPatterns.ioc.implementations.length;
    
    reportContent = `# Code Quality Analysis Report\n\n` +
                   `**Total SOLID Violations:** ${totalSolidViolations}\n` +
                   `**Total Pattern Implementations:** ${totalPatternImplementations}\n` +
                   `**Total Pattern Violations:** ${totalPatternViolations}\n\n` +
                   `## SOLID Principles Summary\n\n` +
                   `| Principle | Violations |\n` +
                   `|-----------|------------|\n` +
                   `| Single Responsibility | ${solidViolations.srp.length} |\n` +
                   `| Open-Closed | ${solidViolations.ocp.length} |\n` +
                   `| Liskov Substitution | ${solidViolations.lsp.length} |\n` +
                   `| Interface Segregation | ${solidViolations.isp.length} |\n` +
                   `| Dependency Inversion | ${solidViolations.dip.length} |\n\n` +
                   `## Design Patterns Summary\n\n` +
                   `| Pattern | Implementations | Violations |\n` +
                   `|---------|-----------------|------------|\n` +
                   `| Polymorphism | ${designPatterns.polymorphism.implementations.length} | ${designPatterns.polymorphism.violations.length} |\n` +
                   `| Dependency Injection | ${designPatterns.dependencyInjection.implementations.length} | ${designPatterns.dependencyInjection.violations.length} |\n` +
                   `| Inversion of Control | ${designPatterns.ioc.implementations.length} | ${designPatterns.ioc.violations.length} |\n\n` +
                   reportContent;
    
    // Save to a file
    try {
      fs.writeFileSync(path.join(this.baseDir, 'docs', 'code_quality_report.md'), reportContent);
      console.log("Code quality report saved to docs/code_quality_report.md");
    } catch (err) {
      console.error("Error saving code quality report:", err.message);
    }
  }

  // getDirectoryStats is now handled by FileSystemUtils
}


// Main execution block
async function main() {
  const analyzer = new UnifiedProjectAnalyzer(process.cwd()); // Use current working directory
  await analyzer.analyze();
}

if (require.main === module) {
  main().catch(err => {
    console.error("Analysis failed:", err);
    process.exit(1);
  });
}

module.exports = UnifiedProjectAnalyzer; // Export class for potential reuse