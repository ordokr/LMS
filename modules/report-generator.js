/**
 * Module for generating analysis reports
 */
const path = require('path');
const fs = require('fs');

class ReportGenerator {
  constructor(metrics, baseDir) {
    this.metrics = metrics;
    this.baseDir = baseDir;
    
    // Hardcode SQLite + sqlx as the database solution
    this.databaseSolution = {
      engine: "SQLite",
      driver: "sqlx",
      configuration: "Embedded, file-based",
      databasePath: "./src-tauri/educonnect.db",
      migrations: "sqlx built-in migrations"
    };
  }

  /**
   * Generate central reference hub for project documentation
   */
  async generateCentralReferenceHub(fsUtils) {
    console.log("Generating central reference hub...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    const outputPath = path.join(docsDir, 'central_reference_hub.md');
    
    // Create the content
    let content = `# LMS Project Central Reference Hub\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Project overview section
    content += `## Project Overview\n\n`;
    const modelsPercent = this.getPercentage(this.metrics.models.implemented, this.metrics.models.total);
    const apiPercent = this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total);
    const uiPercent = this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total);
    
    content += `| Component | Completion | Status |\n`;
    content += `|-----------|------------|--------|\n`;
    content += `| Models | ${modelsPercent}% | ${this.getStatusEmoji(modelsPercent)} |\n`;
    content += `| API Endpoints | ${apiPercent}% | ${this.getStatusEmoji(apiPercent)} |\n`;
    content += `| UI Components | ${uiPercent}% | ${this.getStatusEmoji(uiPercent)} |\n`;
    content += `| Test Coverage | ${this.metrics.tests.coverage}% | ${this.getStatusEmoji(this.metrics.tests.coverage)} |\n\n`;
    
    content += `**Overall Phase:** ${this.metrics.overallPhase}\n\n`;
    
    // Model reference
    content += `## Model Reference\n\n`;
    content += `| Model | Properties | Completeness | Implementation |\n`;
    content += `|-------|------------|-------------|----------------|\n`;
    
    // Sort models by name
    const sortedModels = [...this.metrics.models.details].sort((a, b) => 
      a.name.localeCompare(b.name)
    );
    
    sortedModels.forEach(model => {
      const propertyCount = model.properties ? model.properties.length : 0;
      const filePath = model.file ? this.getRelativePath(model.file) : 'N/A';
      content += `| ${model.name} | ${propertyCount} | ${model.completeness}% | [View Code](${filePath}) |\n`;
    });
    
    content += `\n`;
    
    // API endpoint reference
    content += `## API Endpoint Reference\n\n`;
    content += `| Endpoint | Route | HTTP Method | Completeness | Implementation |\n`;
    content += `|----------|-------|------------|-------------|----------------|\n`;
    
    // Sort endpoints by path
    const sortedEndpoints = [...this.metrics.apiEndpoints.details].sort((a, b) => {
      if (!a.routePath) return 1;
      if (!b.routePath) return -1;
      return a.routePath.localeCompare(b.routePath);
    });
    
    sortedEndpoints.forEach(endpoint => {
      const route = endpoint.routePath || 'N/A';
      const method = endpoint.httpMethod || 'GET';
      const filePath = endpoint.file ? this.getRelativePath(endpoint.file) : 'N/A';
      content += `| ${endpoint.name} | ${route} | ${method} | ${endpoint.completeness}% | [View Code](${filePath}) |\n`;
    });
    
    content += `\n`;
    
    // UI component reference
    content += `## UI Component Reference\n\n`;
    content += `| Component | Type | Completeness | Implementation |\n`;
    content += `|-----------|------|-------------|----------------|\n`;
    
    // Sort components by name
    const sortedComponents = [...this.metrics.uiComponents.details].sort((a, b) =>
      a.name.localeCompare(b.name)
    );
    
    sortedComponents.forEach(component => {
      const type = component.type || 'Component';
      const filePath = component.file ? this.getRelativePath(component.file) : 'N/A';
      content += `| ${component.name} | ${type} | ${component.completeness}% | [View Code](${filePath}) |\n`;
    });
    
    content += `\n`;
    
    // Quality metrics
    content += `## Code Quality Summary\n\n`;
    content += `- **Average Complexity:** ${this.metrics.codeQuality.complexity.average.toFixed(1)}\n`;
    content += `- **High Complexity Files:** ${this.metrics.codeQuality.complexity.high}\n`;
    content += `- **Technical Debt Score:** ${this.metrics.codeQuality.techDebt.score}%\n`;
    
    if (this.metrics.codeQuality.solidViolations) {
      const totalViolations = 
        this.metrics.codeQuality.solidViolations.srp.length +
        this.metrics.codeQuality.solidViolations.ocp.length +
        this.metrics.codeQuality.solidViolations.lsp.length +
        this.metrics.codeQuality.solidViolations.isp.length +
        this.metrics.codeQuality.solidViolations.dip.length;
      
      content += `- **SOLID Violations:** ${totalViolations}\n`;
    }
    
    content += `\n`;
    
    // Link to other reports
    content += `## Available Reports\n\n`;
    
    const reportFiles = fsUtils.filterFiles(/\.md$/)
      .filter(file => file.includes('docs/') && !file.includes('central_reference_hub.md'));
    
    if (reportFiles.length > 0) {
      reportFiles.forEach(reportFile => {
        const reportName = path.basename(reportFile);
        const reportPath = this.getRelativePath(reportFile);
        content += `- [${this.formatReportName(reportName)}](${reportPath})\n`;
      });
    } else {
      content += `No additional reports available yet.\n`;
    }
    
    // Save the hub file
    try {
      fs.writeFileSync(outputPath, content);
      console.log(`Central reference hub generated at ${outputPath}`);
      return outputPath;
    } catch (error) {
      console.error(`Failed to write central reference hub: ${error.message}`);
      return null;
    }
  }

  async generateCodeSmellsReport() {
    // Move code smells report generation here
  }
  
  generateDetailedSection() {
    // Move detailed section generation here
  }

  /**
   * Generate system architecture report
   */
  generateArchitectureReport() {
    console.log("Generating system architecture report...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    const outputPath = path.join(docsDir, 'system_architecture.md');
    
    // Generate content
    let content = `# System Architecture\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Add technology stack section
    content += `## Technology Stack\n\n`;
    
    // Detect technologies used based on file extensions and content
    const technologies = this.detectTechnologies();
    
    // Add dedicated database section for the hardcoded SQLite solution
    content += `### Database Solution (Hardcoded)\n\n`;
    content += `- **Engine**: ${this.databaseSolution.engine}\n`;
    content += `- **Driver**: ${this.databaseSolution.driver} (Rust crate)\n`;
    content += `- **Configuration**: ${this.databaseSolution.configuration}\n`;
    content += `- **Path**: ${this.databaseSolution.databasePath}\n`;
    content += `- **Migrations**: ${this.databaseSolution.migrations}\n\n`;
    content += `SQLite with sqlx is hardcoded as the database solution for this project. This combination provides:\n\n`;
    content += `- **Offline-First Architecture**: Local database enabling full offline functionality\n`;
    content += `- **Zero Configuration**: No separate database server setup required\n`;
    content += `- **Cross-Platform**: Works consistently on all supported platforms\n`;
    content += `- **Type Safety**: Through sqlx compile-time SQL checking\n\n`;
    
    if (Object.keys(technologies).length > 0) {
      // Group by category
      const categories = {
        frontend: ['react', 'angular', 'vue', 'svelte', 'typescript', 'javascript', 'html', 'css', 'sass', 'less'],
        backend: ['node', 'express', 'koa', 'fastify', 'nestjs', 'python', 'django', 'flask', 'ruby', 'rails', 'java', 'spring', 'php', 'laravel', 'rust', 'golang'],
        database: ['mongodb', 'postgresql', 'mysql', 'redis', 'elasticsearch', 'cassandra'], // sqlite removed as it's handled separately
        infrastructure: ['docker', 'kubernetes', 'aws', 'azure', 'gcp', 'terraform', 'nginx', 'apache'],
        testing: ['jest', 'mocha', 'chai', 'cypress', 'selenium', 'puppeteer', 'pytest']
      };
      
      Object.entries(categories).forEach(([category, techs]) => {
        // Skip database section as we've added a dedicated one
        if (category === 'database' && technologies.sqlite) {
          return;
        }
        
        content += `### ${category.charAt(0).toUpperCase() + category.slice(1)}\n\n`;
        const found = techs.filter(tech => technologies[tech]);
        
        if (found.length > 0) {
          found.forEach(tech => {
            content += `- **${tech.charAt(0).toUpperCase() + tech.slice(1)}**: ${technologies[tech]}\n`;
          });
        } else {
          content += `- No ${category} technologies detected\n`;
        }
        
        content += `\n`;
      });
    } else {
      content += `No specific technologies detected\n\n`;
    }
    
    // System components
    content += `## System Components\n\n`;
    
    // Model layer
    content += `### Model Layer\n\n`;
    content += `The system contains ${this.metrics.models.total} models with ${this.metrics.models.implemented} implemented (${this.getPercentage(this.metrics.models.implemented, this.metrics.models.total)}%).\n\n`;
    
    // List important models
    const importantModels = this.metrics.models.details
      .filter(m => m.completeness >= 50)
      .sort((a, b) => b.completeness - a.completeness)
      .slice(0, 5);
    
    if (importantModels.length > 0) {
      content += `#### Key Models\n\n`;
      importantModels.forEach(model => {
        content += `- **${model.name}** (${model.completeness}% complete)\n`;
        if (model.properties && model.properties.length > 0) {
          content += `  - Properties: ${model.properties.map(p => p.name).join(', ')}\n`;
        }
      });
      content += `\n`;
    }
    
    // API layer
    content += `### API Layer\n\n`;
    content += `The system exposes ${this.metrics.apiEndpoints.total} API endpoints with ${this.metrics.apiEndpoints.implemented} implemented (${this.getPercentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}%).\n\n`;
    
    // Group API endpoints by route prefix
    const apiGroups = {};
    this.metrics.apiEndpoints.details.forEach(endpoint => {
      if (!endpoint.routePath) return;
      
      const routePrefix = endpoint.routePath.split('/')[1] || 'other';
      if (!apiGroups[routePrefix]) {
        apiGroups[routePrefix] = [];
      }
      apiGroups[routePrefix].push(endpoint);
    });
    
    if (Object.keys(apiGroups).length > 0) {
      content += `#### API Endpoint Groups\n\n`;
      Object.entries(apiGroups).forEach(([group, endpoints]) => {
        content += `- **${group}**: ${endpoints.length} endpoints\n`;
      });
      content += `\n`;
    }
    
    // UI layer
    content += `### UI Layer\n\n`;
    content += `The system has ${this.metrics.uiComponents.total} UI components with ${this.metrics.uiComponents.implemented} implemented (${this.getPercentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}%).\n\n`;
    
    // Group UI components by type
    const uiGroups = {};
    this.metrics.uiComponents.details.forEach(component => {
      const type = component.type || 'Component';
      if (!uiGroups[type]) {
        uiGroups[type] = [];
      }
      uiGroups[type].push(component);
    });
    
    if (Object.keys(uiGroups).length > 0) {
      content += `#### UI Component Types\n\n`;
      Object.entries(uiGroups).forEach(([type, components]) => {
        content += `- **${type}**: ${components.length} components\n`;
      });
      content += `\n`;
    }
    
    // Architecture diagram
    content += `## Architecture Diagram\n\n`;
    content += `\`\`\`mermaid\n`;
    content += `flowchart TB\n`;
    
    // Create architecture diagram based on detected components
    content += `    User((User)) --> FE[Frontend Layer]\n`;
    content += `    FE --> API[API Layer]\n`;
    content += `    API --> DB[(SQLite/sqlx)]\n`;  // Changed to explicitly mention SQLite/sqlx
    
    // Add frontend components
    if (Object.keys(uiGroups).length > 0) {
      content += `    FE --> |includes| UI[UI Components]\n`;
      content += `    UI --> |contains| FE_COMP[${Object.keys(uiGroups).join(', ')}]\n`;
    }
    
    // Add API routes
    if (Object.keys(apiGroups).length > 0) {
      content += `    API --> |routes| ROUTES[API Routes]\n`;
      const routeList = Object.keys(apiGroups).slice(0, 5);
      if (routeList.length > 0) {
        content += `    ROUTES --> |includes| API_GROUPS[${routeList.join(', ')}]\n`;
      }
    }
    
    // Add models
    content += `    API --> |uses| MODELS[Models]\n`;
    const modelList = importantModels.map(m => m.name).slice(0, 5);
    if (modelList.length > 0) {
      content += `    MODELS --> |includes| MODEL_LIST[${modelList.join(', ')}]\n`;
    }
    
    // Add SQLite details
    content += `    DB --> |type-safe| SQL[SQL Queries]\n`;
    content += `    DB --> |managed by| MIGRATIONS[Migrations]\n`;
    
    content += `\`\`\`\n\n`;
    
    // Relationship diagram
    if (this.metrics.relationships && this.metrics.relationships.length > 0) {
      content += `## Entity Relationships\n\n`;
      content += `\`\`\`mermaid\n`;
      content += `erDiagram\n`;
      
      // Create unique relationships to avoid duplicates
      const uniqueRelationships = new Set();
      this.metrics.relationships.forEach(rel => {
        const relationString = `    ${rel.from} ${rel.type === 'OneToMany' ? '||--o{' : '||--||'} ${rel.to} : relates\n`;
        uniqueRelationships.add(relationString);
      });
      
      uniqueRelationships.forEach(rel => {
        content += rel;
      });
      
      content += `\`\`\`\n\n`;
    }
    
    // Save the architecture report
    try {
      fs.writeFileSync(outputPath, content);
      console.log(`System architecture report generated at ${outputPath}`);
      return outputPath;
    } catch (error) {
      console.error(`Failed to write architecture report: ${error.message}`);
      return null;
    }
  }

  /**
   * Detect technologies used in the project
   */
  detectTechnologies() {
    const technologies = {};
    
    // Hardcoded database solution
    technologies.sqlite = `Embedded database (sqlx driver)`;
    
    // Check package.json for dependencies
    const packagePath = path.join(this.baseDir, 'package.json');
    if (fs.existsSync(packagePath)) {
      try {
        const packageJson = JSON.parse(fs.readFileSync(packagePath, 'utf8'));
        const allDeps = {
          ...packageJson.dependencies,
          ...packageJson.devDependencies
        };
        
        // Check for common technologies
        if (allDeps.react) technologies.react = `v${allDeps.react}`;
        if (allDeps.vue) technologies.vue = `v${allDeps.vue}`;
        if (allDeps.angular) technologies.angular = `v${allDeps.angular}`;
        if (allDeps['@angular/core']) technologies.angular = `v${allDeps['@angular/core']}`;
        if (allDeps.svelte) technologies.svelte = `v${allDeps.svelte}`;
        if (allDeps.typescript) technologies.typescript = `v${allDeps.typescript}`;
        
        // Backend
        if (allDeps.express) technologies.express = `v${allDeps.express}`;
        if (allDeps.koa) technologies.koa = `v${allDeps.koa}`;
        if (allDeps.fastify) technologies.fastify = `v${allDeps.fastify}`;
        if (allDeps['@nestjs/core']) technologies.nestjs = `v${allDeps['@nestjs/core']}`;
        
        // Rust-related dependencies would be in Cargo.toml, not package.json
        technologies.rust = 'Tauri backend (with sqlx for database access)';
        
        // Database - SQLite is hardcoded with sqlx
        // Override any auto-detected databases
        technologies.sqlite = 'SQLite with sqlx (hardcoded)';
        if (allDeps.mongoose || allDeps.mongodb) delete technologies.mongodb;
        if (allDeps.pg || allDeps.postgres || allDeps.postgresql) delete technologies.postgresql;
        if (allDeps.mysql || allDeps.mysql2) delete technologies.mysql;
        if (allDeps.redis) technologies.redis = 'Detected in dependencies (secondary cache only)';
        
        // Testing
        if (allDeps.jest) technologies.jest = `v${allDeps.jest}`;
        if (allDeps.mocha) technologies.mocha = `v${allDeps.mocha}`;
        if (allDeps.chai) technologies.chai = `v${allDeps.chai}`;
        if (allDeps.cypress) technologies.cypress = `v${allDeps.cypress}`;
        if (allDeps.puppeteer) technologies.puppeteer = `v${allDeps.puppeteer}`;
      } catch (err) {
        console.warn("Could not parse package.json:", err.message);
      }
    }
    
    // Check for other technologies based on file extensions
    const fileExtensions = {};
    this.metrics.models.details.forEach(model => {
      if (!model.file) return;
      const ext = path.extname(model.file).toLowerCase();
      fileExtensions[ext] = (fileExtensions[ext] || 0) + 1;
    });
    
    this.metrics.apiEndpoints.details.forEach(endpoint => {
      if (!endpoint.file) return;
      const ext = path.extname(endpoint.file).toLowerCase();
      fileExtensions[ext] = (fileExtensions[ext] || 0) + 1;
    });
    
    // Add detected technologies based on file extensions
    if (fileExtensions['.py'] && !technologies.python) technologies.python = `${fileExtensions['.py']} Python files detected`;
    if (fileExtensions['.rb'] && !technologies.ruby) technologies.ruby = `${fileExtensions['.rb']} Ruby files detected`;
    if (fileExtensions['.java'] && !technologies.java) technologies.java = `${fileExtensions['.java']} Java files detected`;
    if (fileExtensions['.rs'] && !technologies.rust) technologies.rust = `${fileExtensions['.rs']} Rust files detected`;
    if (fileExtensions['.go'] && !technologies.golang) technologies.golang = `${fileExtensions['.go']} Go files detected`;
    if (fileExtensions['.js'] && !technologies.javascript) technologies.javascript = `${fileExtensions['.js']} JavaScript files detected`;
    if (fileExtensions['.ts'] && !technologies.typescript) technologies.typescript = `${fileExtensions['.ts']} TypeScript files detected`;
    
    // Node.js is assumed if we have JS files
    if (fileExtensions['.js'] && !technologies.node) {
      technologies.node = 'Detected based on JavaScript files';
    }
    
    return technologies;
  }

  /**
   * Get status emoji based on percentage
   */
  getStatusEmoji(percent) {
    if (percent >= 80) return '✅';
    if (percent >= 50) return '🟡';
    if (percent >= 20) return '🟠';
    return '❌';
  }

  /**
   * Get relative path for linking
   */
  getRelativePath(filePath) {
    if (!filePath) return '#';
    
    // For absolute paths, make them relative to baseDir
    if (path.isAbsolute(filePath)) {
      try {
        return path.relative(this.baseDir, filePath);
      } catch (err) {
        return filePath;
      }
    }
    
    return filePath;
  }

  /**
   * Format report name for display
   */
  formatReportName(filename) {
    return filename
      .replace(/\.md$/, '')
      .replace(/_/g, ' ')
      .replace(/\b\w/g, c => c.toUpperCase());
  }

  /**
   * Get percentage safely
   */
  getPercentage(value, total) {
    if (total === 0) return 0;
    return Math.round((value / total) * 100);
  }

  /**
   * Generate feature coverage map
   */
  generateFeatureCoverageMap() {
    console.log("Generating feature coverage map...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    const outputPath = path.join(docsDir, 'feature_coverage_map.md');
    
    // Generate content
    let content = `# Feature Coverage Map\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Group features by areas
    const featureAreas = this.metrics.featureAreas || {
      auth: { total: 0, implemented: 0 },
      forum: { total: 0, implemented: 0 },
      lms: { total: 0, implemented: 0 },
      integration: { total: 0, implemented: 0 },
      other: { total: 0, implemented: 0 }
    };
    
    // Feature implementation chart
    content += `## Feature Area Implementation\n\n`;
    content += `\`\`\`mermaid\n`;
    content += `pie title Feature Area Implementation\n`;
    
    // Add slices for each feature area
    Object.entries(featureAreas).forEach(([area, stats]) => {
      if (stats.total > 0) {
        const percentage = this.getPercentage(stats.implemented, stats.total);
        content += `    "${area} (${percentage}%)" : ${stats.implemented}\n`;
      }
    });
    
    content += `\`\`\`\n\n`;
    
    // Feature status table
    content += `## Feature Status\n\n`;
    content += `| Feature Area | Implemented | Total | Completion |\n`;
    content += `|--------------|-------------|-------|------------|\n`;
    
    Object.entries(featureAreas).forEach(([area, stats]) => {
      const percentage = this.getPercentage(stats.implemented, stats.total);
      content += `| ${area.charAt(0).toUpperCase() + area.slice(1)} | ${stats.implemented} | ${stats.total} | ${percentage}% |\n`;
    });
    
    content += `\n`;
    
    // Feature heatmap
    content += `## Feature Implementation Heatmap\n\n`;
    content += `\`\`\`mermaid\n`;
    content += `heatmap\n`;
    content += `  title Feature Implementation Status\n`;
    
    // X-axis
    content += `  x-axis [Authentication, User Management, Content Management, Discussions, Assessments, Analytics, Notifications, Integration]\n`;
    
    // Y-axis
    content += `  y-axis [Backend, Frontend, Testing]\n`;
    
    // Generate some data based on what we know
    // Higher values = more complete
    // Scale: 0 (not started) to 5 (fully implemented)
    content += `  0 0 ${this.getHeatmapValue('auth', 'backend')}\n`;
    content += `  0 1 ${this.getHeatmapValue('auth', 'frontend')}\n`;
    content += `  0 2 ${this.getHeatmapValue('auth', 'testing')}\n`;
    
    content += `  1 0 ${this.getHeatmapValue('user', 'backend')}\n`;
    content += `  1 1 ${this.getHeatmapValue('user', 'frontend')}\n`;
    content += `  1 2 ${this.getHeatmapValue('user', 'testing')}\n`;
    
    content += `  2 0 ${this.getHeatmapValue('content', 'backend')}\n`;
    content += `  2 1 ${this.getHeatmapValue('content', 'frontend')}\n`;
    content += `  2 2 ${this.getHeatmapValue('content', 'testing')}\n`;
    
    content += `  3 0 ${this.getHeatmapValue('forum', 'backend')}\n`;
    content += `  3 1 ${this.getHeatmapValue('forum', 'frontend')}\n`;
    content += `  3 2 ${this.getHeatmapValue('forum', 'testing')}\n`;
    
    content += `  4 0 ${this.getHeatmapValue('lms', 'backend')}\n`;
    content += `  4 1 ${this.getHeatmapValue('lms', 'frontend')}\n`;
    content += `  4 2 ${this.getHeatmapValue('lms', 'testing')}\n`;
    
    content += `  5 0 ${this.getHeatmapValue('analytics', 'backend')}\n`;
    content += `  5 1 ${this.getHeatmapValue('analytics', 'frontend')}\n`;
    content += `  5 2 ${this.getHeatmapValue('analytics', 'testing')}\n`;
    
    content += `  6 0 ${this.getHeatmapValue('notifications', 'backend')}\n`;
    content += `  6 1 ${this.getHeatmapValue('notifications', 'frontend')}\n`;
    content += `  6 2 ${this.getHeatmapValue('notifications', 'testing')}\n`;
    
    content += `  7 0 ${this.getHeatmapValue('integration', 'backend')}\n`;
    content += `  7 1 ${this.getHeatmapValue('integration', 'frontend')}\n`;
    content += `  7 2 ${this.getHeatmapValue('integration', 'testing')}\n`;
    
    content += `  color-scheme Blues\n`;
    content += `\`\`\`\n\n`;
    
    // Feature implementation roadmap
    content += `## Feature Implementation Roadmap\n\n`;
    content += `\`\`\`mermaid\n`;
    content += `gantt\n`;
    content += `  title Feature Implementation Roadmap\n`;
    content += `  dateFormat YYYY-MM-DD\n`;
    content += `  axisFormat %m-%d\n`;
    
    // Get current date
    const today = new Date().toISOString().split('T')[0];
    
    // Calculate dates based on predictions
    const predictions = this.metrics.predictions?.estimates || {};
    const completionDate = predictions.project?.date || 
                          new Date(new Date().setMonth(new Date().getMonth() + 3)).toISOString().split('T')[0];
    
    // Generate roadmap based on feature areas and their completion percentage
    Object.entries(featureAreas).forEach(([area, stats]) => {
      const percentage = this.getPercentage(stats.implemented, stats.total);
      const areaName = area.charAt(0).toUpperCase() + area.slice(1);
      
      // For completed features, show as done from past to today
      if (percentage >= 100) {
        content += `  section ${areaName}\n`;
        content += `  ${areaName} : done, ${today}, 7d\n`;
      } 
      // For in-progress features, show partial completion and remaining work
      else if (percentage > 0) {
        // Calculate mid-point date between now and completion
        const midDate = new Date();
        const completionObj = new Date(completionDate);
        midDate.setDate(midDate.getDate() + Math.floor((completionObj - midDate) / 2 / 86400000));
        
        content += `  section ${areaName}\n`;
        content += `  ${percentage}% Complete : done, ${today}, ${midDate.toISOString().split('T')[0]}\n`;
        content += `  Remaining : active, after ${midDate.toISOString().split('T')[0]}, ${completionDate}\n`;
      }
      // For not-started features, show as future task
      else if (stats.total > 0) {
        content += `  section ${areaName}\n`;
        content += `  ${areaName} : ${today}, ${completionDate}\n`;
      }
    });
    
    content += `\`\`\`\n\n`;
    
    content += `## Feature Breakdown\n\n`;
    
    // Features by implementation status
    const featureStatus = this.analyzeFeatureStatus();
    
    // Add a section for each feature area with feature details
    Object.entries(featureStatus).forEach(([area, features]) => {
      content += `### ${area.charAt(0).toUpperCase() + area.slice(1)}\n\n`;
      
      // Sort features by implementation percentage
      features.sort((a, b) => b.percentage - a.percentage);
      
      content += `| Feature | Status | Implementation |\n`;
      content += `|---------|--------|----------------|\n`;
      
      features.forEach(feature => {
        const status = feature.percentage >= 100 ? '✅ Complete' :
                      feature.percentage > 0 ? `⏳ ${feature.percentage}%` :
                      '❌ Not Started';
        
        content += `| ${feature.name} | ${status} | ${feature.details} |\n`;
      });
      
      content += `\n`;
    });
    
    // Save the feature coverage map
    try {
      fs.writeFileSync(outputPath, content);
      console.log(`Feature coverage map generated at ${outputPath}`);
      return outputPath;
    } catch (error) {
      console.error(`Failed to write feature coverage map: ${error.message}`);
      return null;
    }
  }

  /**
   * Get heatmap value for a feature area and layer
   */
  getHeatmapValue(featureArea, layer) {
    // Default implementation analyzes feature area completion
    // This could be enhanced with more detailed analysis
    
    // Get the feature area stats
    const featureAreas = this.metrics.featureAreas || {};
    let area;
    
    // Map feature area to our known categories
    if (featureArea === 'auth') area = featureAreas.auth;
    else if (featureArea === 'user') area = featureAreas.auth; // Similar to auth
    else if (featureArea === 'content') area = featureAreas.lms;
    else if (featureArea === 'forum') area = featureAreas.forum;
    else if (featureArea === 'lms') area = featureAreas.lms;
    else if (featureArea === 'analytics') area = featureAreas.other;
    else if (featureArea === 'notifications') area = featureAreas.other;
    else if (featureArea === 'integration') area = featureAreas.integration;
    else area = { total: 0, implemented: 0 };
    
    // Calculate base value from 0-5 based on implementation percentage
    const percentage = area.total > 0 ? 
      Math.round((area.implemented / area.total) * 5) : 0;
    
    // Adjust value based on layer
    if (layer === 'backend') {
      // Backend is typically implemented first
      return Math.min(5, Math.max(0, percentage + 1));
    } else if (layer === 'frontend') {
      // Frontend follows backend
      return Math.min(5, Math.max(0, percentage));
    } else if (layer === 'testing') {
      // Testing often lags behind implementation
      return Math.min(5, Math.max(0, percentage - 1));
    }
    
    return percentage;
  }

  /**
   * Analyze feature status in more detail
   */
  analyzeFeatureStatus() {
    const featureStatus = {
      auth: [],
      forum: [],
      lms: [],
      integration: [],
      other: []
    };
    
    // Authentication features
    featureStatus.auth.push({
      name: 'User Registration',
      percentage: this.estimateFeatureCompletion('auth', ['registration', 'signup', 'user']),
      details: this.getFeatureDetails('auth', ['registration', 'signup', 'user'])
    });
    
    featureStatus.auth.push({
      name: 'Login/Logout',
      percentage: this.estimateFeatureCompletion('auth', ['login', 'logout', 'authentication']),
      details: this.getFeatureDetails('auth', ['login', 'logout', 'authentication'])
    });
    
    featureStatus.auth.push({
      name: 'Password Reset',
      percentage: this.estimateFeatureCompletion('auth', ['password', 'reset', 'forgot']),
      details: this.getFeatureDetails('auth', ['password', 'reset', 'forgot'])
    });
    
    // Forum features
    featureStatus.forum.push({
      name: 'Discussion Threads',
      percentage: this.estimateFeatureCompletion('forum', ['thread', 'discussion', 'topic']),
      details: this.getFeatureDetails('forum', ['thread', 'discussion', 'topic'])
    });
    
    featureStatus.forum.push({
      name: 'Comments',
      percentage: this.estimateFeatureCompletion('forum', ['comment', 'reply']),
      details: this.getFeatureDetails('forum', ['comment', 'reply'])
    });
    
    featureStatus.forum.push({
      name: 'User Mentions',
      percentage: this.estimateFeatureCompletion('forum', ['mention', '@user']),
      details: this.getFeatureDetails('forum', ['mention', '@user'])
    });
    
    // LMS features
    featureStatus.lms.push({
      name: 'Courses',
      percentage: this.estimateFeatureCompletion('lms', ['course', 'class']),
      details: this.getFeatureDetails('lms', ['course', 'class'])
    });
    
    featureStatus.lms.push({
      name: 'Assignments',
      percentage: this.estimateFeatureCompletion('lms', ['assignment', 'homework', 'task']),
      details: this.getFeatureDetails('lms', ['assignment', 'homework', 'task'])
    });
    
    featureStatus.lms.push({
      name: 'Grading',
      percentage: this.estimateFeatureCompletion('lms', ['grade', 'score', 'assessment']),
      details: this.getFeatureDetails('lms', ['grade', 'score', 'assessment'])
    });
    
    // Integration features
    featureStatus.integration.push({
      name: 'Canvas Integration',
      percentage: this.estimateFeatureCompletion('integration', ['canvas', 'lms', 'import']),
      details: this.getFeatureDetails('integration', ['canvas', 'lms', 'import'])
    });
    
    featureStatus.integration.push({
      name: 'Discourse Integration',
      percentage: this.estimateFeatureCompletion('integration', ['discourse', 'forum', 'import']),
      details: this.getFeatureDetails('integration', ['discourse', 'forum', 'import'])
    });
    
    return featureStatus;
  }

  /**
   * Estimate completion percentage for a feature
   */
  estimateFeatureCompletion(area, keywords) {
    // Look for models related to this feature
    const modelMatches = this.metrics.models.details.filter(model => 
      keywords.some(kw => 
        model.name.toLowerCase().includes(kw.toLowerCase()) ||
        (model.file && model.file.toLowerCase().includes(kw.toLowerCase()))
      )
    );
    
    const modelPercent = modelMatches.length > 0 ?
      modelMatches.reduce((sum, model) => sum + model.completeness, 0) / modelMatches.length : 0;
    
    // Look for API endpoints related to this feature
    const apiMatches = this.metrics.apiEndpoints.details.filter(endpoint => 
      keywords.some(kw => 
        endpoint.name.toLowerCase().includes(kw.toLowerCase()) ||
        (endpoint.routePath && endpoint.routePath.toLowerCase().includes(kw.toLowerCase())) ||
        (endpoint.file && endpoint.file.toLowerCase().includes(kw.toLowerCase()))
      )
    );
    
    const apiPercent = apiMatches.length > 0 ?
      apiMatches.reduce((sum, endpoint) => sum + endpoint.completeness, 0) / apiMatches.length : 0;
    
    // Look for UI components related to this feature
    const uiMatches = this.metrics.uiComponents.details.filter(component => 
      keywords.some(kw => 
        component.name.toLowerCase().includes(kw.toLowerCase()) ||
        (component.file && component.file.toLowerCase().includes(kw.toLowerCase()))
      )
    );
    
    const uiPercent = uiMatches.length > 0 ?
      uiMatches.reduce((sum, component) => sum + component.completeness, 0) / uiMatches.length : 0;
    
    // Calculate overall percentage
    const counts = [
      modelMatches.length > 0 ? 1 : 0,
      apiMatches.length > 0 ? 1 : 0,
      uiMatches.length > 0 ? 1 : 0
    ];
    
    const total = counts.reduce((sum, count) => sum + count, 0);
    
    if (total === 0) return 0;
    
    return Math.round((modelPercent * counts[0] + apiPercent * counts[1] + uiPercent * counts[2]) / total);
  }

  /**
   * Get details string for a feature
   */
  getFeatureDetails(area, keywords) {
    // Find matching components
    const modelMatches = this.metrics.models.details.filter(model => 
      keywords.some(kw => 
        model.name.toLowerCase().includes(kw.toLowerCase()) ||
        (model.file && model.file.toLowerCase().includes(kw.toLowerCase()))
      )
    );
    
    const apiMatches = this.metrics.apiEndpoints.details.filter(endpoint => 
      keywords.some(kw => 
        endpoint.name.toLowerCase().includes(kw.toLowerCase()) ||
        (endpoint.routePath && endpoint.routePath.toLowerCase().includes(kw.toLowerCase())) ||
        (endpoint.file && endpoint.file.toLowerCase().includes(kw.toLowerCase()))
      )
    );
    
    const uiMatches = this.metrics.uiComponents.details.filter(component => 
      keywords.some(kw => 
        component.name.toLowerCase().includes(kw.toLowerCase()) ||
        (component.file && component.file.toLowerCase().includes(kw.toLowerCase()))
      )
    );
    
    // Create details string
    const parts = [];
    
    if (modelMatches.length > 0) {
      parts.push(`${modelMatches.length} model${modelMatches.length > 1 ? 's' : ''}`);
    }
    
    if (apiMatches.length > 0) {
      parts.push(`${apiMatches.length} endpoint${apiMatches.length > 1 ? 's' : ''}`);
    }
    
    if (uiMatches.length > 0) {
      parts.push(`${uiMatches.length} component${uiMatches.length > 1 ? 's' : ''}`);
    }
    
    if (parts.length === 0) {
      return 'Not implemented';
    }
    
    return parts.join(', ');
  }

  /**
   * Generate project timeline report showing progress over time
   */
  generateTimelineReport() {
    console.log("Generating project timeline report...");
    
    const docsDir = path.join(this.baseDir, 'docs');
    if (!fs.existsSync(docsDir)) {
      fs.mkdirSync(docsDir, { recursive: true });
    }
    
    const outputPath = path.join(docsDir, 'project_timeline.md');
    
    // Load historical data if available
    const historyPath = path.join(this.baseDir, '.analysis_cache', 'history.json');
    let history = [];
    
    if (fs.existsSync(historyPath)) {
      try {
        history = JSON.parse(fs.readFileSync(historyPath, 'utf8'));
      } catch (err) {
        console.warn("Could not load historical data:", err.message);
      }
    }
    
    // Add current snapshot
    const currentSnapshot = {
      date: new Date().toISOString(),
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
      techDebt: this.metrics.codeQuality?.techDebt?.score || 0,
      complexity: this.metrics.codeQuality?.complexity?.average || 0,
      phase: this.metrics.overallPhase
    };
    
    // Add to history (avoid duplicates on same day)
    const today = new Date().toISOString().split('T')[0];
    const existingTodayIndex = history.findIndex(item => 
      item.date.split('T')[0] === today
    );
    
    if (existingTodayIndex >= 0) {
      history[existingTodayIndex] = currentSnapshot;
    } else {
      history.push(currentSnapshot);
    }
    
    // Save updated history
    try {
      // Create cache directory if it doesn't exist
      const cacheDir = path.join(this.baseDir, '.analysis_cache');
      if (!fs.existsSync(cacheDir)) {
        fs.mkdirSync(cacheDir, { recursive: true });
      }
      
      fs.writeFileSync(historyPath, JSON.stringify(history, null, 2));
    } catch (err) {
      console.warn("Could not save historical data:", err.message);
    }
    
    // Generate report content
    let content = `# Project Timeline Report\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Only show last 30 days if history is long
    const displayHistory = history.length > 30 ? history.slice(-30) : history;
    
    // Generate progress chart
    content += `## Progress Over Time\n\n`;
    content += `\`\`\`mermaid\n`;
    content += `timeline\n`;
    
    displayHistory.forEach((snapshot, index) => {
      const date = snapshot.date.split('T')[0];
      content += `    title Project Progress Timeline\n`;
      content += `    section ${date}\n`;
      content += `    Models ${snapshot.models.percent}% : ${snapshot.models.implemented}/${snapshot.models.total}\n`;
      content += `    API ${snapshot.apiEndpoints.percent}% : ${snapshot.apiEndpoints.implemented}/${snapshot.apiEndpoints.total}\n`;
      content += `    UI ${snapshot.uiComponents.percent}% : ${snapshot.uiComponents.implemented}/${snapshot.uiComponents.total}\n`;
      content += `    Tests ${snapshot.tests.coverage}% : Coverage\n`;
    });
    
    content += `\`\`\`\n\n`;
    
    // Generate data table
    content += `## Detailed History\n\n`;
    content += `| Date | Models | API Endpoints | UI Components | Test Coverage | Tech Debt | Phase |\n`;
    content += `|------|--------|--------------|--------------|--------------|-----------|-------|\n`;
    
    displayHistory.forEach(snapshot => {
      const date = snapshot.date.split('T')[0];
      content += `| ${date} | ${snapshot.models.percent}% | ${snapshot.apiEndpoints.percent}% | ${snapshot.uiComponents.percent}% | ${snapshot.tests.coverage}% | ${snapshot.techDebt}% | ${snapshot.phase} |\n`;
    });
    
    // Add velocity chart if we have enough data points
    if (history.length > 2) {
      content += `\n## Velocity Chart\n\n`;
      content += `\`\`\`mermaid\n`;
      content += `gantt\n`;
      content += `    title Implementation Velocity\n`;
      content += `    dateFormat YYYY-MM-DD\n`;
      content += `    axisFormat %m-%d\n`;
      
      // Calculate dates for the chart
      const startDate = history[0].date.split('T')[0];
      const lastRecord = history[history.length - 1];
      const endDate = this.metrics.predictions?.estimates?.project?.date || 
                      new Date(new Date().setMonth(new Date().getMonth() + 3)).toISOString().split('T')[0];
      
      content += `    section Models\n`;
      content += `    ${lastRecord.models.percent}% Complete :done, ${startDate}, ${lastRecord.date.split('T')[0]}\n`;
      content += `    Remaining :active, ${lastRecord.date.split('T')[0]}, ${endDate}\n`;
      
      content += `    section API\n`;
      content += `    ${lastRecord.apiEndpoints.percent}% Complete :done, ${startDate}, ${lastRecord.date.split('T')[0]}\n`;
      content += `    Remaining :active, ${lastRecord.date.split('T')[0]}, ${endDate}\n`;
      
      content += `    section UI\n`;
      content += `    ${lastRecord.uiComponents.percent}% Complete :done, ${startDate}, ${lastRecord.date.split('T')[0]}\n`;
      content += `    Remaining :active, ${lastRecord.date.split('T')[0]}, ${endDate}\n`;
      
      content += `\`\`\`\n\n`;
      
      // Add trends
      content += `## Trends and Observations\n\n`;
      
      // Calculate average weekly progress for each area
      if (history.length > 7) {
        const weeklyModelProgress = this.calculateWeeklyProgress(history, 'models');
        const weeklyApiProgress = this.calculateWeeklyProgress(history, 'apiEndpoints');
        const weeklyUiProgress = this.calculateWeeklyProgress(history, 'uiComponents');
        
        content += `- **Models:** Average weekly progress is ${weeklyModelProgress.toFixed(1)}%\n`;
        content += `- **API Endpoints:** Average weekly progress is ${weeklyApiProgress.toFixed(1)}%\n`;
        content += `- **UI Components:** Average weekly progress is ${weeklyUiProgress.toFixed(1)}%\n`;
        
        // Tech debt trend
        const techDebtTrend = this.calculateTrend(history, 'techDebt');
        if (techDebtTrend < 0) {
          content += `- **Tech Debt:** Decreasing (Improving) 📉\n`;
        } else if (techDebtTrend > 0) {
          content += `- **Tech Debt:** Increasing (Worsening) 📈\n`;
        } else {
          content += `- **Tech Debt:** Stable ↔️\n`;
        }
      }
    }
    
    // Save the timeline report
    try {
      fs.writeFileSync(outputPath, content);
      console.log(`Project timeline report generated at ${outputPath}`);
      return outputPath;
    } catch (error) {
      console.error(`Failed to write timeline report: ${error.message}`);
      return null;
    }
  }

  /**
   * Calculate weekly progress from historical data
   */
  calculateWeeklyProgress(history, metric) {
    if (history.length < 2) return 0;
    
    // Get oldest and newest entries
    const oldest = history[0];
    const newest = history[history.length - 1];
    
    // Calculate difference in percentage
    const percentDiff = newest[metric].percent - oldest[metric].percent;
    
    // Calculate time difference in weeks
    const msInWeek = 7 * 24 * 60 * 60 * 1000;
    const timeDiff = (new Date(newest.date) - new Date(oldest.date)) / msInWeek;
    
    if (timeDiff < 0.1) return 0; // Avoid division by very small numbers
    
    return percentDiff / timeDiff;
  }

  /**
   * Calculate trend (positive or negative) for a metric
   */
  calculateTrend(history, metric) {
    if (history.length < 3) return 0;
    
    // Get last few entries
    const recent = history.slice(-3);
    
    // Simple linear regression
    let sumX = 0, sumY = 0, sumXY = 0, sumXX = 0;
    const n = recent.length;
    
    recent.forEach((item, i) => {
      sumX += i;
      sumY += item[metric] || 0;
      sumXY += i * (item[metric] || 0);
      sumXX += i * i;
    });
    
    const slope = (n * sumXY - sumX * sumY) / (n * sumXX - sumX * sumX);
    return slope;
  }

  /**
   * Generate database architecture documentation
   */
  generateDatabaseDocumentation() {
    console.log("Generating database documentation...");
    
    const ragDir = path.join(this.baseDir, 'rag_knowledge_base', 'integration');
    if (!fs.existsSync(ragDir)) {
      fs.mkdirSync(ragDir, { recursive: true });
    }
    
    const outputPath = path.join(ragDir, 'database_architecture.md');
    
    // Generate content
    let content = `# Database Architecture\n\n`;
    content += `_Generated on: ${new Date().toISOString().split('T')[0]}_\n\n`;
    
    // Database overview
    content += `## Database Solution\n\n`;
    content += `This project uses **SQLite** with the **sqlx** Rust crate as its database solution. This combination is hardcoded into the application architecture.\n\n`;
    
    // Justification
    content += `### Why SQLite + sqlx?\n\n`;
    content += `- **Offline-First Architecture**: SQLite provides local database capabilities essential for our offline-first approach\n`;
    content += `- **Zero-Configuration**: No separate database server installation required\n`;
    content += `- **Cross-Platform**: SQLite works consistently across all supported platforms\n`;
    content += `- **Performance**: Excellent performance for our expected workloads\n`;
    content += `- **Type Safety**: sqlx provides compile-time SQL query validation\n`;
    content += `- **Transactions**: Full ACID compliance with transaction support\n\n`;
    
    // Architecture details
    content += `## Implementation Details\n\n`;
    content += `### Database Connection\n\n`;
    content += `\`\`\`rust
// src-tauri/src/db/mod.rs
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool, Error};
use std::path::PathBuf;

pub async fn establish_connection() -> Result<SqlitePool, Error> {
    let db_path = app_local_data_dir()
        .map(|dir| dir.join("educonnect.db"))
        .ok_or_else(|| Error::Database("Failed to get app data directory".into()))?;
    
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
\`\`\`\n\n`;

    content += `### Dependency Injection\n\n`;
    content += `The SQLite connection pool is injected into services, ensuring consistent database access throughout the application.\n\n`;
    content += `\`\`\`rust
// Example service initialization
pub fn initialize_services(pool: SqlitePool) -> AppServices {
    let user_repository = UserRepository::new(pool.clone());
    let course_repository = CourseRepository::new(pool.clone());
    // ...other repositories
    
    let auth_service = AuthService::new(user_repository.clone());
    let course_service = CourseService::new(course_repository);
    // ...other services
    
    AppServices {
        auth_service,
        course_service,
        // ...other services
    }
}
\`\`\`\n\n`;

    // Add migration information
    content += `## Database Migration\n\n`;
    content += `The project uses sqlx's built-in migrations system for schema management. Migrations are SQL files stored in \`src-tauri/migrations/\` and run automatically when the application starts.\n\n`;
    content += `Example migration file structure:\n\n`;
    content += `\`\`\`
src-tauri/migrations/
├── 20230101000000_initial_schema.sql
├── 20230201000000_add_user_preferences.sql
└── 20230301000000_add_course_features.sql
\`\`\`\n\n`;
    
    // Add schema documentation if available
    if (this.metrics.models && this.metrics.models.details.length > 0) {
      content += `## Schema Documentation\n\n`;
      
      // Group models by category
      const categories = {
        users: ['user', 'profile', 'account', 'permission', 'role'],
        courses: ['course', 'lesson', 'assignment', 'module', 'material'],
        discussions: ['discussion', 'post', 'thread', 'comment', 'forum'],
        other: []
      };
      
      // Sort models into categories
      const categorizedModels = {};
      Object.keys(categories).forEach(category => { categorizedModels[category] = []; });
      
      this.metrics.models.details.forEach(model => {
        let assigned = false;
        
        Object.entries(categories).forEach(([category, keywords]) => {
          if (assigned) return;
          
          const lowerName = model.name.toLowerCase();
          if (keywords.some(keyword => lowerName.includes(keyword))) {
            categorizedModels[category].push(model);
            assigned = true;
          }
        });
        
        if (!assigned) {
          categorizedModels.other.push(model);
        }
      });
      
      // Add each category
      Object.entries(categorizedModels).forEach(([category, models]) => {
        if (models.length === 0) return;
        
        content += `### ${category.charAt(0).toUpperCase() + category.slice(1)} Schema\n\n`;
        
        models.forEach(model => {
          content += `#### ${model.name}\n\n`;
          
          if (model.properties && model.properties.length > 0) {
            content += `| Field | Type | Description |\n`;
            content += `|-------|------|-------------|\n`;
            
            model.properties.forEach(prop => {
              const type = prop.type || 'Unknown';
              const description = prop.description || '';
              content += `| ${prop.name} | ${type} | ${description} |\n`;
            });
            content += `\n`;
          } else {
            content += `Schema details not available\n\n`;
          }
        });
      });
    }
    
    // Save the database documentation
    try {
      fs.writeFileSync(outputPath, content);
      console.log(`Database documentation generated at ${outputPath}`);
      return outputPath;
    } catch (error) {
      console.error(`Failed to write database documentation: ${error.message}`);
      return null;
    }
  }

  /**
   * Generate AI context documents for GitHub Copilot
   */
  async generateAIContext() {
    console.log("Generating AI context documents...");
    
    const aiDocsDir = path.join(this.baseDir, 'docs', 'ai');
    if (!fs.existsSync(aiDocsDir)) {
      fs.mkdirSync(aiDocsDir, { recursive: true });
    }
    
    // Create the AI guidance document at project root
    const aiGuidancePath = path.join(this.baseDir, 'AI_GUIDANCE.md');
    
    // Generate content
    let content = `# AI Guidance for LMS Integration Project\n\n`;
    content += `<!-- AI_METADATA
version: 1.0
priority: highest
updated: ${new Date().toISOString().split('T')[0]}
role: guidance
-->\n\n`;
    
    // Reference the documentation hierarchy
    content += `## Documentation Hierarchy\n\n`;
    content += `1. **This Document**: High-level guidance for AI tooling\n`;
    content += `2. **[Central Reference Hub](docs/central_reference_hub.md)**: Project source of truth\n`;
    content += `3. **[Knowledge Base](rag_knowledge_base/)**: Technical documentation and specifications\n\n`;
    
    // Project metadata
    content += `## Project Metadata\n\n`;
    content += `- **Database**: SQLite with sqlx (embedded file database)\n`;
    content += `- **Frontend Framework**: Tauri with web frontend\n`;
    content += `- **Backend Language**: Rust\n`;
    content += `- **Architecture**: Offline-first with local-first data storage\n\n`;
    
    // Component implementation guidance
    content += `## Component Implementation Guidance\n\n`;
    content += `When implementing components:\n\n`;
    content += `1. Check the central reference hub for existing specifications\n`;
    content += `2. Maintain the documented completion percentage\n`;
    content += `3. Follow existing naming conventions and file structure\n`;
    content += `4. Ensure database models match documented schemas\n`;
    content += `5. Use SQLite with sqlx for all database operations\n\n`;
    
    // Save the file
    try {
      fs.writeFileSync(aiGuidancePath, content);
      console.log(`AI guidance document generated at ${aiGuidancePath}`);
    } catch (error) {
      console.error(`Failed to write AI guidance: ${error.message}`);
    }
    
    // Generate component cross-reference file
    this.generateComponentCrossReference(aiDocsDir);
    
    // Add AI metadata to central reference hub
    this.addAIMetadataToCentralHub();
    
    return aiGuidancePath;
  }

  /**
   * Generate component cross-reference for AI
   */
  generateComponentCrossReference(aiDocsDir) {
    const componentIndexPath = path.join(aiDocsDir, 'component_index.md');
    
    let content = `# Component Implementation Cross-Reference\n\n`;
    content += `<!-- AI_METADATA
version: 1.0
priority: medium
updated: ${new Date().toISOString().split('T')[0]}
role: component_reference
-->\n\n`;
    
    // Group components by implementation percentage
    const completionGroups = {
      high: [],
      medium: [],
      low: []
    };
    
    this.metrics.uiComponents.details.forEach(component => {
      if (component.completeness >= 60) {
        completionGroups.high.push(component);
      } else if (component.completeness >= 30) {
        completionGroups.medium.push(component);
      } else {
        completionGroups.low.push(component);
      }
    });
    
    // Add component tables by completion level
    content += `## High Completion Components (60%+)\n\n`;
    content += this.generateComponentTable(completionGroups.high);
    
    content += `## Medium Completion Components (30-59%)\n\n`;
    content += this.generateComponentTable(completionGroups.medium);
    
    content += `## Low Completion Components (<30%)\n\n`;
    content += this.generateComponentTable(completionGroups.low);
    
    // Save the file
    try {
      fs.writeFileSync(componentIndexPath, content);
      console.log(`Component index generated at ${componentIndexPath}`);
    } catch (error) {
      console.error(`Failed to write component index: ${error.message}`);
    }
  }

  /**
   * Generate a component table for the given components
   */
  generateComponentTable(components) {
    if (components.length === 0) {
      return "No components in this category.\n\n";
    }
    
    let table = `| Component | Type | Completeness | Implementation |\n`;
    table += `|-----------|------|-------------|----------------|\n`;
    
    components.forEach(component => {
      const type = component.type || 'Component';
      const filePath = component.file ? this.getRelativePath(component.file) : 'N/A';
      table += `| ${component.name} | ${type} | ${component.completeness}% | [View Code](${filePath}) |\n`;
    });
    
    return table + "\n";
  }

  /**
   * Add AI metadata to central reference hub
   */
  addAIMetadataToCentralHub() {
    const hubPath = path.join(this.baseDir, 'docs', 'central_reference_hub.md');
    
    try {
      let content = fs.readFileSync(hubPath, 'utf8');
      
      // Check if AI metadata already exists
      if (!content.includes('AI_METADATA')) {
        // Add AI metadata to the top
        content = `<!-- AI_METADATA
version: 1.0
priority: highest
updated: ${new Date().toISOString().split('T')[0]}
role: reference
status: authoritative
-->\n\n${content}`;
        
        fs.writeFileSync(hubPath, content);
        console.log(`Added AI metadata to central reference hub`);
      }
    } catch (error) {
      console.error(`Failed to update central reference hub: ${error.message}`);
    }
  }
}

module.exports = ReportGenerator;