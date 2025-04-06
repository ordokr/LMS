const parser = require('@babel/parser');
const traverse = require('@babel/traverse').default;
const FileSystemUtils = require('./fileSystemUtils');
const AnalysisUtils = require('./analysisUtils'); // Import the new AnalysisUtils module
const path = require('path');
const fs = require('fs'); // Keep fs for generateCentralReferenceHub
const AstAnalyzer = require('./astAnalyzer'); // Import the new AstAnalyzer module
const ProjectPredictor = require('./projectPredictor'); // Import the new ProjectPredictor module
use serde::{Deserialize, Serialize};
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::State,
};
use std::sync::Arc;
use crate::AppState;
use crate::database::repositories::{
    ForumCategoryRepository, 
    ForumTopicRepository, 
    ForumPostRepository
};
use crate::shared::models::{Category, Topic, Post, Tag, ForumStats};
/** chrono::{DateTime, Utc};
 * Unified Project Analyzer
 * Consolidates all analyzer functionality into a single tool
 */
class UnifiedProjectAnalyzer {ror};
  constructor(baseDir, sourceSystems = {}) {Post, Tag, ForumStats};
    this.baseDir = baseDir;er;
    this.sourceSystems = sourceSystems; // {canvas: '/path/to/canvas', discourse: '/path/to/discourse'}
    ForumCategoryRepository, ForumTopicRepository, ForumPostRepository,
    // ConfigurationForumTagRepository
    this.config = {
      implementationThreshold: 35,er;
    };
    mod categories {
    // Metrics tracking - INITIALIZE METRICS FIRST
    this.metrics = {
      models: { total: 0, implemented: 0, details: [] },
      apiEndpoints: { total: 0, implemented: 0, details: [] },
      uiComponents: { total: 0, implemented: 0, details: [] },
      tests: { total: 0, passing: 0, coverage: 0, details: [] },
      
      codeQuality: {
          complexity: {lemented later
              average: 0,
              high: 0,
              files: []lize)]
          },aginationParams {
          techDebt: { "default_page")]
              score: 0,
              items: []default_per_page")]
          },page: usize,
          solidViolations: {
              srp: [],
              ocp: [],size { 1 }
              lsp: [],-> usize { 20 }
              isp: [],
              dip: []ialize)]
          },reateCategoryRequest {
          designPatterns: {
              polymorphism: {
                  implementations: [],
                  violations: []  
              },d: Option<i64>,
              dependencyInjection: {
                  implementations: [],
                  violations: []
              },
              ioc: {rialize)]
                  implementations: [],
                  violations: []
              }y_id: i64,
          }tent: String, // Initial post content
      },tags: Option<Vec<String>>,
      
      featureAreas: {
          auth: { total: 0, implemented: 0 },
          forum: { total: 0, implemented: 0 },
          lms: { total: 0, implemented: 0 },
          integration: { total: 0, implemented: 0 },
          other: { total: 0, implemented: 0 }
      },
      ve(Debug, Deserialize)]
      relationships: [],uest {
      b content: String,
      predictions: {
          velocityData: {
              models: 1.5,       // Models implemented per week
              apiEndpoints: 3,    // API endpoints implemented per week
              uiComponents: 5,    // UI components implemented per week
              tests: 2           // Tests added per week
          },s: Vec<Post>,
          estimates: {}
      },
      ine SearchQuery struct near other request structs
      // Add source system metrics
      sourceSystems: { {
        canvas: {, // The search query string
          models: { total: 0, details: [] },
          controllers: { total: 0, details: [] },
          filesByType: {}64>,
        },e(default = "default_page")]
        discourse: {
          models: { total: 0, details: [] },
          controllers: { total: 0, details: [] },
          filesByType: {}
        }
      },e SyncQuery struct for update endpoints
      ve(Debug, Deserialize)]
      // Add source-to-target mappings
      sourceToTarget: {default_since")]
        models: [], // {sourceSystem, sourceName, sourceFile, targetName, targetFile, completeness}
        controllers: [], // {sourceSystem, sourceName, sourceFile, targetHandler, targetFile, completeness}
        components: [] // {sourceSystem, sourceName, sourceFile, targetName, targetFile, completeness}
      }ult_since() -> i64 { 0 }
    };
// TODO: Define SearchResult enum/struct if not already defined elsewhere
    // Initialize utility classes
    this.fsUtils = new FileSystemUtils(this.baseDir, this.getExcludePatterns());
    this.analysisUtils = new AnalysisUtils(this.baseDir, this.fsUtils, this.config, this.metrics);
    this.astAnalyzer = new AstAnalyzer();
    this.predictor = new ProjectPredictor(this.metrics);
  }}

  /**n forum_routes() -> Router<Arc<AppState>> {
   * Defines the patterns for excluding directories and files during discovery.
   */   // Category routes
  getExcludePatterns() {ies", get(get_categories))
    // Keep exclude patterns logic here or move fully to fsUtils if preferred
    return [te("/categories/:id", get(get_category))
      /node_modules/,gories/:id", put(update_category))
      /\.git/,("/categories/:id", delete(delete_category))
      /target\/(?!.*\.rs$)/, // Exclude target dir except .rs files))
      /dist/,e("/courses/:id/categories", get(get_categories_by_course))
      /build/,
      /\.cache/, routes
      /\.next/,"/topics", get(get_topics))
      /\.nuxt/,"/topics", post(create_topic))
      /\.DS_Store/,pics/:id", get(get_topic))
      /coverage/,topics/:id", put(update_topic))
      /\.vscode/,topics/:id", delete(delete_topic))
      /\.idea/,"/topics/:id/posts", get(get_posts_by_topic))
      /assets/, // Exclude assets dirost(create_post))
      /public/, // Exclude public dirget_recent_topics))
      /docs/, // Exclude docs dir
      /references/, // Exclude references dir
      /analysis_summary/, // Exclude analysis_summary dir
      /md_dashboard/, // Exclude md_dashboard dir
      /tools\/__pycache__/, // Exclude python cache
      /.*\.log$/, // Exclude log files(like_post))
      /.*\.tmp$/, // Exclude temp files
      /.*\.bak.?$/, // Exclude backup files
      /.*\.swp$/, // Exclude vim swap files
      /LMS\.code-workspace/, // Exclude workspace file
      /package-lock\.json/, // Exclude lock file
      /yarn\.lock/, // Exclude lock file
      /unified-project-analyzer\.js/, // Exclude self
      /project-analyzer\.js.*/, // Exclude older analyzer versions
      /debug-analyzer\.js/,
      /status-analyzer\.js/,t(search_forum))
      /advanced-api-analyzer\.js/,
      /fix-.*\.js/, // Exclude fix scripts
      /cleanup-docs\.js/,categories", get(get_updated_categories))
      /run-full-analysis\.js/,s", get(get_updated_topics))
      /status-updater\.js/,sts", get(get_updated_posts))
      /analyze_project\.pdb/, // Exclude pdb file
      /fileSystemUtils\.js/, // Exclude the new utils file
    ];egory handlers
  }nc fn get_categories(
    State(state): State<Arc<AppState>>,
  /**Result<Json<Vec<Category>>, AppError> {
   * Run the analysisategoryRepository::new(state.pool.clone());
   */et categories = repo.get_all().await?;
  async analyze() {ies))
    console.log(`Starting analysis of ${this.baseDir}...`);

    // Discover and read files using FileSystemUtils
    this.fsUtils.discoverFiles();ate>>,
    this.fsUtils.readFileContents();
    Json(payload): Json<CreateCategoryRequest>,
    // Use AnalysisUtils for analysis {
    await this.analysisUtils.analyzeModels();ermission
    await this.analysisUtils.analyzeApiEndpoints();uthentication required".to_string()))?;
    await this.analysisUtils.analyzeUIComponents();
    await this.analysisUtils.analyzeTests();n privileges required".to_string()));
    }
    // Use AnalysisUtils for code quality analysis too
    await this.analysisUtils.analyzeCodeQuality(this.astAnalyzer);
    
    // Analyze source systems if any are defined
    if (Object.keys(this.sourceSystems).length > 0) {
      await this.analyzeSourceSystems();
    }   None => slugify(&payload.name),
    };
    // Generate relationship maps with Mermaid diagrams
    await this.generateRelationshipMaps();
        &payload.name,
    // Make completion predictions (use only this one)
    this.predictor.predictCompletion();
        payload.parent_id,
    // Update project status
    this.updateProjectStatus();),
        payload.text_color.as_deref(),
    // Generate central reference hub (new)
    await this.generateCentralReferenceHub();
    Ok(Json(category))
    // Generate source system comparison report (new)
    await this.generateSourceComparisonReport();
nc fn get_category(
    this.printSummary();    State(state): State<Arc<AppState>>,
    return this.metrics;
  }) -> Result<Json<Category>, AppError> {
et repo = ForumCategoryRepository::new(state.pool.clone());
  // File system related methods (discoverFiles, readFileContents, indexFileKeywords, findFilesByPatterns, getDirectoryStats) are removed.
   .ok_or(AppError::NotFound("Category not found".to_string()))?;
  /**
   * Update overall project status based on metrics
   */
  updateProjectStatus() {
    console.log("Updating project status...");
    const modelsPercent = calculate_percentage(this.metrics.models.implemented, this.metrics.models.total);    State(state): State<Arc<AppState>>,
    const apiPercent = calculate_percentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total);
    const uiPercent = calculate_percentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total);
teCategoryRequest>,
    this.metrics.overallStatus = {, AppError> {
      models: `${modelsPercent}%`,mission
      api: `${apiPercent}%`,)))?;
      ui: `${uiPercent}%`, !user.is_admin {
      tests: `${this.metrics.tests.coverage}%`,        return Err(AppError::Forbidden("Admin privileges required".to_string()));
      techDebt: `${this.metrics.codeQuality.techDebt.score}%` // Use calculated score
    };

    // Determine overall phase
    const avgCompletion = (modelsPercent + apiPercent + uiPercent) / 3;
    if (avgCompletion < 10) this.metrics.overallPhase = 'planning';
    else if (avgCompletion < 40) this.metrics.overallPhase = 'early_development';_string()))?;
    else if (avgCompletion < 75) this.metrics.overallPhase = 'mid_development';    
    else if (avgCompletion < 95) this.metrics.overallPhase = 'late_development';
    else this.metrics.overallPhase = 'release_candidate';
     Some(slug) => slug.clone(),
    console.log(`Project Status: Models=${modelsPercent}%, API=${apiPercent}%, UI=${uiPercent}%, Tests=${this.metrics.tests.coverage}%, Debt=${this.metrics.codeQuality.techDebt.score}%`);        None => slugify(&payload.name),
    console.log(`Overall Phase: ${this.metrics.overallPhase}`);;
  }
et updated = repo.update(
  /**
   * Generate detailed section for reports
   */        &slug,
  generateDetailedSection() {ad.description.as_deref(),
    let details = "## Implementation Details\n\n";

    // Models
    details += `### Models (${calculate_percentage(this.metrics.models.implemented, this.metrics.models.total)}% Complete)\n\n`;eref(),
    details += "| Model | File | Completeness |\n";
    details += "|-------|------|-------------|\n";
    this.metrics.models.details
        .sort((a, b) => a.name.localeCompare(b.name))
        .forEach(m => {
            details += `| ${m.name} | ${m.file.replace(/\\/g, '/')} | ${m.completeness}% ${m.completeness < 50 ? '⚠️ Low' : ''} |\n`;async fn delete_category(
        });ate<Arc<AppState>>,
    details += "\n";

    // API Endpoints
    details += `### API Endpoints (${calculate_percentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}% Complete)\n\n`;d has permission
    details += "| Handler | File | Route | Completeness | Feature Area |\n";)?;
    details += "|---------|------|-------|-------------|--------------|\n";
     this.metrics.apiEndpoints.details
        .sort((a, b) => (a.featureArea + a.name).localeCompare(b.featureArea + b.name))
        .forEach(e => {
            details += `| ${e.name} | ${e.file.replace(/\\/g, '/')} | ${e.routePath || '-'} | ${e.completeness}% ${e.completeness < 50 ? '⚠️ Low' : ''} | ${e.featureArea} |\n`;    let repo = ForumCategoryRepository::new(state.pool.clone());
        });
    details += "\n";

    // UI Components.to_string()))?;
    details += `### UI Components (${calculate_percentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}% Complete)\n\n`;
    details += "| Component | File | Completeness |\n";
    details += "|-----------|------|-------------|\n";
    this.metrics.uiComponents.details
        .sort((a, b) => a.name.localeCompare(b.name))
        .forEach(c => {
            details += `| ${c.name} | ${c.file.replace(/\\/g, '/')} | ${c.completeness}% ${c.completeness < 50 ? '⚠️ Low' : ''} |\n`;async fn get_categories_by_course(
        });ate<Arc<AppState>>,
    details += "\n";

     // Code Qualityol.clone());
     details += `### Code Quality Metrics\n\n`;
     details += `| Metric              | Value |\n`;
     details += `|---------------------|-------|\n`;
     details += `| Avg Complexity      | ${this.metrics.codeQuality.complexity.average.toFixed(1)} |\n`;
     details += `| High Complexity Files | ${this.metrics.codeQuality.complexity.high} |\n`;
     // details += `| Duplication Count   | ${this.metrics.codeQuality.duplications.count} |\n`; // Add if implemented
     // details += `| Duplicated Lines    | ${this.metrics.codeQuality.duplications.lines} |\n`; // Add if implementedasync fn get_topics(
     details += `| Technical Debt Score| ${this.metrics.codeQuality.techDebt.score}% |\n`;
     details += "\n";

     if (this.metrics.codeQuality.techDebt.items.length > 0) {
         details += `#### Top Technical Debt Items\n\n`;s.per_page).await?;
         details += `| File | Issue | Complexity/Score | Recommendation |\n`;
         details += `|------|-------|-----------------|----------------|\n`;
         this.metrics.codeQuality.techDebt.items
             .sort((a, b) => b.score - a.score) // Sort by score descending
             .slice(0, 10) // Show top 10topic(
             .forEach(item => {rc<AppState>>,
                 details += `| ${item.file.replace(/\\/g, '/')} | ${item.issue} | ${item.score} | ${item.recommendation} |\n`;er: Option<User>,
             });    Json(payload): Json<CreateTopicRequest>,
         details += "\n";) -> Result<Json<Topic>, AppError> {
     }is authenticated
 let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
    
    return details;et topic_repo = ForumTopicRepository::new(state.pool.clone());
  }
et tag_repo = ForumTagRepository::new(state.pool.clone());
  /**
   * Get the feature area with the lowest implementation percentagests
   */umCategoryRepository::new(state.pool.clone());
  getLowestImplementedArea() {    let _ = category_repo.get_by_id(payload.category_id).await?
    let lowestPercent = 101;nd".to_string()))?;
    let lowestArea = 'N/A';

    for (const area in this.metrics.featureAreas) {
      const { implemented, total } = this.metrics.featureAreas[area];
      const percent = calculate_percentage(implemented, total);
      if (total > 0 && percent < lowestPercent) { mut tx = state.pool.begin().await.map_err(DbError::from)?;
        lowestPercent = percent;
        lowestArea = area;c
      } let topic = topic_repo.create_with_tx(
    }        &mut tx,
    return lowestArea;   &payload.title,
  }
   payload.category_id,
  /**
   * Generate relationship maps using Mermaid syntax
   */    false, // locked
  async generateRelationshipMaps() {
    console.log("Generating relationship maps...");
    
    const docsDir = path.join(this.baseDir, 'docs');ost_repo.create_with_tx(
    if (!fs.existsSync(docsDir)) {    &mut tx,
      fs.mkdirSync(docsDir, { recursive: true });
    }
        &payload.content,
    // Detect code smells related to SOLID principles
    await this.detectCodeSmells();
    
    // Delegate the relationship detection to analysisUtils/ Add tags if provided
    if (this.analysisUtils.findModelRelationships) {if let Some(tags) = payload.tags {
      await this.analysisUtils.findModelRelationships();
    }with_tx(&mut tx, &tag_name, topic.id).await?;
    
    // Rest of the method can stay as is since it's just visualization
    let mermaidDiagram = "graph LR\n";
    const nodes = new Set();ion
    this.metrics.relationships.forEach(rel => {
        nodes.add(rel.from);
        nodes.add(rel.to);Get the complete topic with all relations
        const arrow = rel.type === 'OneToMany' ? '-->|1..*|' : '-->';    let topic = topic_repo.get_by_id(topic.id).await?
        mermaidDiagram += `  ${rel.from}${arrow}${rel.to}\n`;r creation".to_string()))?;
    });

     // Add nodes that might not have relationships yet
     this.metrics.models.details.forEach(m => nodes.add(m.name));
     // Add styles for nodes (optional)
     nodes.forEach(node => {
         const model = this.metrics.models.details.find(m => m.name === node);
         const completeness = model ? model.completeness : 0;
         let style = 'fill:#eee,stroke:#333,stroke-width:1px';
         if (completeness >= 75) style = 'fill:#c8e6c9,stroke:#2e7d32,stroke-width:2px'; // Green
         else if (completeness >= 40) style = 'fill:#fff9c4,stroke:#fbc02d,stroke-width:1px'; // Yellow.ok_or(AppError::NotFound("Topic not found".to_string()))?;
         else if (completeness > 0) style = 'fill:#ffcdd2,stroke:#c62828,stroke-width:1px'; // Red    
         mermaidDiagram += `  style ${node} ${style}\n`;    // Increment view count
     });

on(topic))
    // Save to a file (e.g., docs/relationship_map.md)
    const mapContent = `# Model Relationship Map\n\n\`\`\`mermaid\n${mermaidDiagram}\n\`\`\`\n`;
    try {ic(
        fs.writeFileSync(path.join(this.baseDir, 'docs', 'relationship_map.md'), mapContent);
        console.log("Relationship map saved to docs/relationship_map.md");ser: Option<User>,
    } catch (err) { Path(id): Path<i64>,
        console.error("Error saving relationship map:", err.message);    Json(payload): Json<CreateTopicRequest>,
    }) -> Result<Json<Topic>, AppError> {
  }/ Ensure user is authenticated
entication required".to_string()))?;

  /**
   * Determine feature area based on file path or name
   */ // Check if topic exists and user has permission
  determineApiFeatureArea(name = '', filePath = '', routePath = '') {    let topic = repo.get_by_id(id).await?
    return this.analysisUtils.determineApiFeatureArea(name, filePath, routePath);   .ok_or(AppError::NotFound("Topic not found".to_string()))?;
  }
f topic.user_id != user.id && !user.is_admin {
  /**o_string()));
   * Add an API endpoint to metrics
   */ 
  addApiEndpoint(name, filePath, completeness, featureArea = 'other', routePath = null) {    // Generate slug if title changed
    return this.analysisUtils.addApiEndpoint(name, filePath, completeness, featureArea, routePath);et slug = slugify(&payload.title);
  }
et updated = repo.update(
  /**
   * Print a summary of the analysis results
   */
  printSummary() {
    console.log("\n--- Analysis Summary ---");
    console.log(`Models: ${this.metrics.models.implemented}/${this.metrics.models.total} (${calculate_percentage(this.metrics.models.implemented, this.metrics.models.total)}%)`);
    console.log(`API Endpoints: ${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total} (${calculate_percentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total)}%)`);
    console.log(`UI Components: ${this.metrics.uiComponents.implemented}/${this.metrics.uiComponents.total} (${calculate_percentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}%)`);
    console.log(`Tests: ${this.metrics.tests.total} (Coverage: ${this.metrics.tests.coverage}%)`);
    console.log(`Code Quality: Avg Complexity=${this.metrics.codeQuality.complexity.average.toFixed(1)}, High Complexity Files=${this.metrics.codeQuality.complexity.high}, Tech Debt=${this.metrics.codeQuality.techDebt.score}%`);nc fn delete_topic(
    console.log(`Overall Phase: ${this.metrics.overallPhase}`);    State(state): State<Arc<AppState>>,
    console.log("----------------------\n");ser: Option<User>,
  }
Result<StatusCode, AppError> {
/**
 * Calculate percentage safely
 */
fn calculate_percentage(value: usize, total: usize) -> u32 {
    if total == 0 {
        0
    } else {
        ((value as f64 / total as f64) * 100.0).round() as u32
    }
}
   .ok_or(AppError::NotFound("Topic not found".to_string()))?;
  /**
   * Parse code content to AST, handling potential errors
   */     return Err(AppError::Forbidden("You don't have permission to delete this topic".to_string()));
  parseToAst(content, filePath = 'unknown') {    }
    return this.astAnalyzer.parseToAst(content, filePath);
  }

  /**T)
   * Calculate Cyclomatic Complexity using AST
   */
  calculateComplexity(ast) {async fn get_topics_by_category(
    return this.astAnalyzer.calculateComplexity(ast);tate(state): State<Arc<AppState>>,
  }
uery(params): Query<PaginationParams>,
  /**
   * Analyze AST for component details (props, hooks, state, handlers)
   */ let topics = repo.get_by_category_id(category_id, params.page, params.per_page).await?;
  analyzeComponentAst(content, filePath) {    
    return this.astAnalyzer.analyzeComponentAst(content, filePath);k(Json(topics))
  }

  /**
   * Generate the Central Reference Hub Markdown file
   */
  async generateCentralReferenceHub() {) -> Result<Json<Vec<Topic>>, AppError> {
    console.log("Generating Central Reference Hub...");tory::new(state.pool.clone());
    const hubPath = path.join(this.baseDir, 'docs', 'central_reference_hub.md');get_recent(params.page, params.per_page).await?;

    // --- Project Overview ---
    const overview = {
        overall_status: this.metrics.overallPhase,
        project_stats: {
            foundation_complete: this.metrics.overallPhase !== 'planning', // Basic check
            model_implementation: calculate_percentage(this.metrics.models.implemented, this.metrics.models.total) + '%',
            api_implementation: calculate_percentage(this.metrics.apiEndpoints.implemented, this.metrics.apiEndpoints.total) + '%',
            ui_implementation: calculate_percentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total) + '%',t<Json<Post>, AppError> {
            test_coverage: this.metrics.tests.coverage + '%',
            technical_debt: this.metrics.codeQuality.techDebt.score + '%'by_id(id).await?
        },not found".to_string()))?;
        // Add source/target system info if available/needed
        target_system: {n(post))
            code_location: this.baseDir,
            // Add stack info if detectable or configured
        },
        completion_forecasts: {
            models: this.metrics.predictions.estimates.models.date,
            api_endpoints: this.metrics.predictions.estimates.apiEndpoints.date,(params): Query<PaginationParams>,
            ui_components: this.metrics.predictions.estimates.uiComponents.date,esult<Json<Vec<Post>>, AppError> {
            entire_project: this.metrics.predictions.estimates.project.date    let repo = ForumPostRepository::new(state.pool.clone());
        }arams.per_page).await?;
    };

    // --- Source-to-Target Mapping (Placeholder/Example) ---
    // This would ideally come from a configuration file or more advanced analysis
    const mappingTable = `| Component | Source System | Source Location | Target Location | Status | Priority |
|-----------|---------------|-----------------|-----------------|--------|----------|
| User Model | Both | \`canvas/.../user.rb\` + \`discourse/.../user.rb\` | \`src-tauri/src/models/user.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='User')?.completeness || 0}% | High |
| Forum Topics | Discourse | \`discourse/.../topic.rb\` | \`src-tauri/src/models/topic.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='Topic')?.completeness || 0}% | High |
| Forum Posts | Discourse | \`discourse/.../post.rb\` | \`src-tauri/src/models/post.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='Post')?.completeness || 0}% | High |
| Courses | Canvas | \`canvas/.../course.rb\` | \`src-tauri/src/models/course.rs\` | ✅ ${this.metrics.models.details.find(m=>m.name==='Course')?.completeness || 0}% | High |
| Forum API | Discourse | \`discourse/.../topics_controller.rb\` | \`src-tauri/src/api/forum.rs\` | ❌ ${calculate_percentage(this.metrics.featureAreas.forum.implemented, this.metrics.featureAreas.forum.total)}% | High |ed
| Course API | Canvas | \`canvas/.../courses_controller.rb\` | \`src-tauri/src/api/lms/courses.rs\` | ❌ ${calculate_percentage(this.metrics.featureAreas.lms.implemented, this.metrics.featureAreas.lms.total)}% | High |    let user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
| UI Components | Both | Multiple files | \`src/components/\` | ✅ ${calculate_percentage(this.metrics.uiComponents.implemented, this.metrics.uiComponents.total)}% | High |
`; // Add more mappings as neededumPostRepository::new(state.pool.clone());

    // --- Integration Conflicts (Placeholder) ---
    const conflicts = { Check if topic exists
        model_conflicts: [ /* ... populate based on analysis ... */ ],    let _ = topic_repo.get_by_id(topic_id).await?
        route_conflicts: [ /* ... populate based on analysis ... */ ]d".to_string()))?;
    };
ost
    // --- Implementation Tasks (Placeholder) ---
    // Prioritize based on missing features or low completeness
    const tasks = [
        `1. **Complete API Endpoint Implementation** (${this.metrics.apiEndpoints.implemented}/${this.metrics.apiEndpoints.total} completed)`,
        `   - High Priority: Focus on areas like '${this.getLowestImplementedArea()}'`,
        `2. **Complete UI Component Implementation** (${this.metrics.uiComponents.implemented}/${this.metrics.uiComponents.total} completed)`,
        `   - Implement components corresponding to new API endpoints`,
        `3. **Address Technical Debt** (Score: ${this.metrics.codeQuality.techDebt.score}%)`,
        `   - Refactor ${this.metrics.codeQuality.complexity.high} high complexity files`,pic_repo.update_last_post(topic_id, post.id, user.id, Utc::now()).await?;
        `   - Improve test coverage (currently ${this.metrics.tests.coverage}%)`,    
        `4. **Integrate Key Systems** (e.g., Search, Notifications - if applicable)`    Ok(Json(post))
    ];


    // --- Directory Structure ---
    // Use fsUtils project structure data
    let dirStructure = "```\n/\n";
    const sortedDirs = [...this.fsUtils.getProjectStructure().directories].sort();    Json(payload): Json<UpdatePostRequest>,
    const topLevelDirs = sortedDirs.filter(d => !d.includes('/') && !d.includes('\\'));or> {
    const structureMap = {};ted
rized("Authentication required".to_string()))?;
     // Build nested structure
     sortedDirs.forEach(dir => {ry::new(state.pool.clone());
         const parts = dir.split(/[\\\/]/);
         let currentLevel = structureMap;permission
         parts.forEach(part => { repo.get_by_id(id).await?
             if (!currentLevel[part]) {nd".to_string()))?;
                 currentLevel[part] = {};
             }ost.user_id != user.id && !user.is_admin {
             currentLevel = currentLevel[part];        return Err(AppError::Forbidden("You don't have permission to update this post".to_string()));
         });
     });
.await?;
     const buildStructureString = (level, indent = ' ') => {
         let str = '';
         Object.keys(level).sort().forEach(key => {
             const dirPath = indent.substring(1).replace(/ /g, '/').replace('├──', '').replace('└──', '').trim() + key; // Approximate path
             const category = this.getDirCategory(dirPath);
             const categoryLabel = category ? ` # ${category.charAt(0).toUpperCase() + category.slice(1)}` : '';ate): State<Arc<AppState>>,
             str += `${indent}├── ${key}/${categoryLabel}\n`;
             str += buildStructureString(level[key], indent + '│  ');h(id): Path<i64>,
         });) -> Result<StatusCode, AppError> {
         return str.replace(/├──(?=[^├──]*$)/, '└──'); // Fix last item marker
     };Error::Unauthorized("Authentication required".to_string()))?;
    
     dirStructure += buildStructureString(structureMap);    let repo = ForumPostRepository::new(state.pool.clone());
     dirStructure += "```\n";

    let post = repo.get_by_id(id).await?
    // --- Implementation Details Table ---st not found".to_string()))?;
    const detailsSection = this.generateDetailedSection(); // Use existing method
    if post.user_id != user.id && !user.is_admin {
    // --- SOLID Violations Table ---pError::Forbidden("You don't have permission to delete this post".to_string()));
    let solidViolationsSection = `## 📊 SOLID Principles Violations\n\n`;

    // SRP violations    repo.delete(id).await?;
    const srpViolations = this.metrics.codeQuality.solidViolations?.srp || [];
    solidViolationsSection += `### Single Responsibility Principle (${srpViolations.length} violations)\n\n`;

    if (srpViolations.length > 0) {
      solidViolationsSection += `| Component | File | Score | Details |\n`;
      solidViolationsSection += `|-----------|------|-------|--------|\n`;
      
      srpViolations,
        .sort((a, b) => b.score - a.score)
        .slice(0, 5) // Show top 5 violationsre user is authenticated
        .forEach(v => {t user = user.ok_or(AppError::Unauthorized("Authentication required".to_string()))?;
          solidViolationsSection += `| ${v.name} | ${v.file.replace(/\\/g, '/')} | ${v.score} | ${v.details} |\n`;
        });
      
      if (srpViolations.length > 5) { if post exists
        solidViolationsSection += `\n_...and ${srpViolations.length - 5} more violations. See full report in docs/solid_code_smells.md_\n`;
      }   .ok_or(AppError::NotFound("Post not found".to_string()))?;
    } else {    
      solidViolationsSection += `No SRP violations detected.\n`;
    }    let updated = repo.toggle_like(id, user.id).await?;

    // Add other SOLID principles here as they're implemented
}
    // --- SOLID Violations Summary ---
    solidViolationsSection += `## 📊 SOLID Principles Violations\n\n`;

    const solidViolations = this.metrics.codeQuality.solidViolations;
    const totalViolations = 
      (solidViolations.srp?.length || 0) + e.pool.clone());
      (solidViolations.ocp?.length || 0) + 
      (solidViolations.lsp?.length || 0) +     
      (solidViolations.isp?.length || 0) + 
      (solidViolations.dip?.length || 0);

    solidViolationsSection += `| Principle | Violations | Most Affected Component |\n`;
    solidViolationsSection += `|-----------|------------|------------------------|\n`;

    // Helper to get the most problematic component
    const getMostProblematicComponent = (violations) => {esult<Json<Vec<Topic>>, AppError> {
      if (!violations || violations.length === 0) return '-';    let repo = ForumTopicRepository::new(state.pool.clone());
      return violations.sort((a, b) => b.score - a.score)[0].name;
    };

    solidViolationsSection += `| Single Responsibility | ${solidViolations.srp?.length || 0} | ${getMostProblematicComponent(solidViolations.srp)} |\n`;
    solidViolationsSection += `| Open-Closed | ${solidViolations.ocp?.length || 0} | ${getMostProblematicComponent(solidViolations.ocp)} |\n`;// Stats handler
    solidViolationsSection += `| Liskov Substitution | ${solidViolations.lsp?.length || 0} | ${getMostProblematicComponent(solidViolations.lsp)} |\n`;    State(state): State<Arc<AppState>>,
    solidViolationsSection += `| Interface Segregation | ${solidViolations.isp?.length || 0} | ${getMostProblematicComponent(solidViolations.isp)} |\n`;
    solidViolationsSection += `| Dependency Inversion | ${solidViolations.dip?.length || 0} | ${getMostProblematicComponent(solidViolations.dip)} |\n`;    State(state): State<Arc<AppState>>,
) -> Result<Json<ForumStats>, AppError> {
    solidViolationsSection += `\n*For detailed analysis, see [SOLID Code Smells Report](docs/solid_code_smells.md)*\n\n`;sitory::new(state.pool.clone());

    let user_repo = UserRepository::new(state.pool.clone());
    // --- Assemble Hub Content ---
    const hubContent = `# LMS Integration Project - Central Reference Hub    let total_topics = topic_repo.count().await?;
post_repo.count().await?;
_Last updated: ${new Date().toISOString().split('T')[0]}_    let total_users = user_repo.count().await?;

## 📊 Project Overviewt_today().await?;
t active_users_today = user_repo.count_active_today().await?;
\`\`\`json    
${JSON.stringify(overview, null, 2)}
\`\`\`        total_posts,
opics,
## 🔄 Source-to-Target Mapping        total_users,

${mappingTable}        active_users_today,

## 🔍 Integration Conflicts (Placeholder)
(Json(stats))
\`\`\`json}
${JSON.stringify(conflicts, null, 2)}
\`\`\`// Search handler
um(
## 📋 Implementation Tasks    State(state): State<Arc<AppState>>,
String, String>>,
${tasks.join('\n')}) -> Result<Json<Vec<Topic>>, AppError> {
plement proper search logic using the query parameters
## 📁 Project Directory Structure    // This likely involves calling a more specific search function in the repository layer
et's keep the existing call but acknowledge it needs refinement
${dirStructure}    let topic_repo = ForumTopicRepository::new(state.pool.clone());
ce with actual search implementation using query parameters
${detailsSection}    // The repository function `search_all` likely needs to be updated or replaced
 `query` (category_id, tags, user_id).
${solidViolationsSection}    let results = topic_repo.search_all(&query.q, query.page, query.per_page).await?;

## 📈 Project Trajectories (Predictions)ary
 let search_results = results.into_iter().map(|res| { /* map to SearchResult */ }).collect();
\`\`\`json
${JSON.stringify(this.metrics.predictions.estimates, null, 2)}ts))
\`\`\`    Ok(Json(results)) // Returning raw Topic results for now

## Source System Comparison

### File Count Comparison

| File Type | Target | Canvas | Discourse |
|-----------|--------|--------|----------|
| javascript | ${targetFilesByType.javascript || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.javascript || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.javascript || 0} |
| typescript | ${targetFilesByType.typescript || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.typescript || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.typescript || 0} |
| rust | ${targetFilesByType.rust || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.rust || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.rust || 0} |
| ruby | ${targetFilesByType.ruby || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.ruby || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.ruby || 0} |
| react | ${targetFilesByType.react || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.react || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.react || 0} |
| css | ${targetFilesByType.css || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.css || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.css || 0} |}
| scss | ${targetFilesByType.scss || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.scss || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.scss || 0} |
| html | ${targetFilesByType.html || 0} | ${this.metrics.sourceSystems.canvas?.filesByType?.html || 0} | ${this.metrics.sourceSystems.discourse?.filesByType?.html || 0} |async fn get_updated_topics(

### Model Implementation Status

| Source System | Source Model | Target Model | Completeness | File Location |
|--------------|-------------|-------------|-------------|-------------|
${this.metrics.sourceToTarget.models.length > 0 ? this.metrics.sourceToTarget.modelsquery.since).await?;
  .sort((a, b) => a.sourceSystem.localeCompare(b.sourceSystem) || a.sourceName.localeCompare(b.sourceName))    
  .map(mapping => `| ${mapping.sourceSystem} | ${mapping.sourceName} | ${mapping.targetName} | ${mapping.completeness}% | ${mapping.targetFile.replace(/\\/g, '/')} |`)    let slug: String = text
  .join('\n') : '| - | - | - | - | - |'}}

### API Implementation Status

| Source System | Source Controller#Action | Target API | Route | Completeness |
|--------------|-------------------------|-----------|-------|-------------|
${this.metrics.sourceToTarget.controllers.length > 0 ? this.metrics.sourceToTarget.controllersstate.pool.clone());
  .sort((a, b) => a.sourceSystem.localeCompare(b.sourceSystem) || a.sourceName.localeCompare(b.sourceName))    // Use query.since directly
  .map(mapping => `| ${mapping.sourceSystem} | ${mapping.sourceName} | ${mapping.targetHandler} | ${mapping.targetRoute || '-'} | ${mapping.completeness}% |`)pdated_since(query.since).await?;
  .join('\n') : '| - | - | - | - | - |'}    

### Missing Model Mappings}

The following source models don't have corresponding target implementations:ugs

| Source System | Source Model | Source File |
|--------------|-------------|-------------|
${Object.keys(this.sourceSystems).map(system => this.metrics.sourceSystems[system]?.models.details
  .filter(sourceModel => !this.metrics.sourceToTarget.models.some(m => m.sourceSystem === system && m.sourceName === sourceModel.name))  // Replace non-alphanumeric characters with hyphens
  .map(sourceModel => `| ${system} | ${sourceModel.name} | ${sourceModel.file.replace(/\\/g, '/')} |`)    let slug: String = text
  .join('\n')).join('\n') || '| - | - | - |'}
`;map(|c| {

    // --- Write to File ---
    try {if c.is_whitespace() {
      fs.writeFileSync(hubPath, hubContent);
      console.log(`Central Reference Hub saved to ${hubPath}`);       } else {
    } catch (err) {             '-'
      console.error("Error saving Central Reference Hub:", err.message);            }
    }
  }

   /** Helper to get directory category */// Replace multiple consecutive hyphens with a single one
   getDirCategory(dirPath) {    let slug = slug.replace("--", "-");
     return this.fsUtils.getDirCategory(dirPath);    let slug = slug.trim_matches('-').to_string();
   }
  /**
   * Add a model to metrics - delegates to analysisUtils
   */
  addModel(name, filePath, completeness) {
    return this.analysisUtils.addModel(name, filePath, completeness);ror handling
  }
num AppError {
  /**
   * Add an API endpoint to metrics - delegates to analysisUtils
   */ BadRequest(String),
  addApiEndpoint(name, filePath, completeness, featureArea = 'other', routePath = null) {    Unauthorized(String),
    return this.analysisUtils.addApiEndpoint(name, filePath, completeness, featureArea, routePath);orbidden(String),
  }

  /**
   * Add a UI component to metrics - delegates to analysisUtils
   */     let (status, message) = match self {
  addUIComponent(name, filePath, completeness) {            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)),
    return this.analysisUtils.addUIComponent(name, filePath, completeness);       AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
  }AD_REQUEST, msg),
       AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
  /**tusCode::FORBIDDEN, msg),
   * Add a test to metrics - delegates to analysisUtils
   */     
  addTest(name, filePath, passing = false) {        let body = Json(serde_json::json!({
    return this.analysisUtils.addTest(name, filePath, passing);       "error": message
  }
   
  /**response()
   * Detect code smells related to SOLID principles
   */
  async detectCodeSmells() {
    console.log("Analyzing code for SOLID principles and design patterns...");
    
    // Initialize code smells array in metrics if not exists::Database(err)
    if (!this.metrics.codeQuality.solidViolations) {
      this.metrics.codeQuality.solidViolations = {
        srp: [],
        ocp: [],
        lsp: [],ing::{get, post, put, delete},
        isp: [],outer,
        dip: []
      };
    }
    
    // Initialize design patterns tracking
    if (!this.metrics.codeQuality.designPatterns) {
      this.metrics.codeQuality.designPatterns = {
        polymorphism: { implementations: [], violations: [] },outes() -> Router<AppState> {
        dependencyInjection: { implementations: [], violations: [] },outer::new()
        ioc: { implementations: [], violations: [] }    // Category routes
      };ries::get_categories))
    }y))
     get(forum_categories::get_category))
    // Process all JavaScript/TypeScript filesupdate_category))
    const jsFiles = this.fsUtils.filterFiles(/\.(js|jsx|ts|tsx)$/);id", delete(forum_categories::delete_category))
    for (const file of jsFiles) {  
      const content = this.fsUtils.getFileContent(file);Topic routes
      if (!content) continue;_topics::get_topics))
      _topic))
      try {", get(forum_topics::get_topic))
        // Parse the file to an AST.route("/topics/:id", put(forum_topics::update_topic))
        const ast = this.parseToAst(content, file);lete_topic))
        if (!ast) continue;opics::get_recent_topics))
        (forum_topics::get_topics_by_category))
        // Detect violations for each SOLID principle
        this.detectSRPViolations(ast, file);
        this.detectOCPViolations(ast, file);m_posts::get_posts_for_topic))
        this.detectLSPViolations(ast, file);.route("/topics/:id/posts", post(forum_posts::create_post))
        this.detectISPViolations(ast, file);::get_post))
        this.detectDIPViolations(ast, file);s::update_post))
        elete_post))
        // Detect patterns and anti-patternsn", post(forum_posts::mark_as_solution))
        this.detectPolymorphism(ast, file);:id/like", post(forum_posts::like_post))
        this.detectDependencyInjection(ast, file);
        this.detectIoC(ast, file); .route("/posts/recent", get(forum_posts::get_recent_posts))
      } catch (error) {
        console.error(`Error analyzing ${file} for code smells:`, error.message);
      }
    }
    
    // Process all Rust files this function exists for testing
    const rustFiles = this.fsUtils.filterFiles(/\.rs$/);:Category;
    for (const file of rustFiles) {e axum::extract::State;
      const content = this.fsUtils.getFileContent(file);
      if (!content) continue;
      
      // For Rust files, we can only detect some principles with regexsync fn setup_test_env() -> Arc<AppState> {
      this.detectRustSRPViolations(content, file);    let pool = create_in_memory_db_pool().await.expect("Failed to create in-memory db pool");
      // Add more Rust-specific detectors if neededecessary
    }
     }
    // Generate a code smells report
    await this.generateCodeSmellsReport();[tokio::test]
  }
   let state = setup_test_env().await;
  /**oryRepository::new(state.pool.clone());
   * Detect Single Responsibility Principle violations in JavaScript/TypeScript code
   */    // Arrange: Insert some test categories
  detectSRPViolations(ast, filePath) {None, None, None, None).await.unwrap();
    const violations = [];, "test-cat-2", None, None, None, None, None).await.unwrap();
    
    // Function to calculate responsibility score based on various metricsr
    const calculateResponsibilityScore = (node) => {tate(state.clone())).await;
      // Metrics that indicate multiple responsibilities
      let distinctConcerns = 0;esult is Ok and contains the expected categories
      let mixedFunctionality = false;
      let highComplexity = false;
      let tooManyMethods = false;  assert_eq!(categories.len(), 2);
      let tooManyDependencies = false;).any(|c| c.id == cat1.id && c.name == "Test Cat 1"));
      let methodsWithDifferentPrefixes = new Set();er().any(|c| c.id == cat2.id && c.name == "Test Cat 2"));
      
      // Get class/function name
      let name = "anonymous";TODO: Add more tests for other handlers (create_category, get_category, etc.)
      if (node.id && node.id.name) { TODO: Add tests for error cases (e.g., database errors, not found)
        name = node.id.name;ion/authorization checks where applicable
      }            // Count methods if it's a class      let methods = [];      let dependenciesCount = 0;            // For classes, check methods and their prefixes      if (node.type === 'ClassDeclaration') {        // Extract methods        methods = node.body.body.filter(item =>           item.type === 'ClassMethod' || item.type === 'MethodDefinition'        );                // Count dependencies (constructor parameters or class properties that look like services)        const constructor = node.body.body.find(item =>           (item.type === 'ClassMethod' || item.type === 'MethodDefinition') &&           item.key.name === 'constructor'        );                if (constructor && constructor.params) {          dependenciesCount = constructor.params.length;        }                // Check if methods have different prefixes (indicating different responsibilities)        methods.forEach(method => {          if (method.key && method.key.name) {            const methodName = method.key.name;            if (methodName !== 'constructor') {              // Extract prefix (e.g., "get" from "getUserData")              const prefix = methodName.match(/^([a-z]+)[A-Z]/);              if (prefix && prefix[1]) {                methodsWithDifferentPrefixes.add(prefix[1]);              }            }          }        });                // Too many methods is a code smell        tooManyMethods = methods.length > 10;                // Too many dependencies might indicate too many responsibilities        tooManyDependencies = dependenciesCount > 5;                // Different method prefixes might indicate different responsibilities        distinctConcerns = methodsWithDifferentPrefixes.size;                // More than 3 different concerns is a code smell        mixedFunctionality = distinctConcerns > 3;      }            // For functions, calculate complexity      if (node.type === 'FunctionDeclaration') {        const complexity = this.calculateComplexity({ program: { body: [node] } });        highComplexity = complexity > 10;      }            // Calculate overall score (0-100, higher is worse)      let score = 0;      if (mixedFunctionality) score += 30;      if (highComplexity) score += 25;      if (tooManyMethods) score += 20;      if (tooManyDependencies) score += 15;      score += (distinctConcerns * 5);            return {        name,        score,        distinctConcerns,        methods: methods.length,        dependencies: dependenciesCount,        recommendation: score > 40 ? 'Consider splitting this into multiple classes/functions with single responsibilities' : null      };    };        // Visit classes and functions in the AST    traverse(ast, {      ClassDeclaration(path) {        const result = calculateResponsibilityScore(path.node);        if (result.score > 40) {          violations.push({            type: 'SRP',            file: filePath,            name: result.name,            score: result.score,            details: `Class has ${result.methods} methods with ${result.distinctConcerns} distinct concerns`,            recommendation: result.recommendation          });        }      },            FunctionDeclaration(path) {        const result = calculateResponsibilityScore(path.node);        if (result.score > 40) {          violations.push({            type: 'SRP',            file: filePath,            name: result.name,            score: result.score,            details: 'Function has too many responsibilities or is too complex',            recommendation: result.recommendation          });        }      }    });        // Add violations to metrics    if (violations.length > 0) {      this.metrics.codeQuality.solidViolations.srp.push(...violations);            // Also add to technical debt      violations.forEach(v => {        this.metrics.codeQuality.techDebt.items.push({          file: v.file,          issue: `SRP Violation: ${v.details}`,          score: v.score,          recommendation: v.recommendation        });      });    }  }  /**   * Detect Single Responsibility Principle violations in Rust code using regex   */  detectRustSRPViolations(content, filePath) {    // Simple heuristics for Rust files    const violations = [];        // Get struct name from content    const structMatch = content.match(/struct\s+(\w+)/);    let name = structMatch ? structMatch[1] : "unknown";        // Count impl blocks for the struct (might indicate multiple responsibilities)    const implCount = (content.match(new RegExp(`impl\\s+${name}`, 'g')) || []).length;        // Count functions in the impl blocks    const functionMatches = content.match(/fn\s+\w+/g) || [];    const functionCount = functionMatches.length;        // Try to detect different method prefixes    const methodPrefixes = new Set();    functionMatches.forEach(fn => {      const match = fn.match(/fn\s+([a-z]+)_/);      if (match && match[1]) {        methodPrefixes.add(match[1]);      }    });        // Calculate a simple score    let score = 0;    if (implCount > 3) score += 20;    if (functionCount > 10) score += 20;    if (methodPrefixes.size > 3) score += 30;    if (content.length > 500) score += Math.min(30, content.length / 100);        if (score > 40) {      violations.push({        type: 'SRP',        file: filePath,        name: name,        score: score,        details: `Struct has ${functionCount} methods with ${methodPrefixes.size} distinct prefixes across ${implCount} impl blocks`,        recommendation: 'Consider splitting this struct into multiple structs with single responsibilities'      });            // Add to metrics      this.metrics.codeQuality.solidViolations.srp.push(...violations);            // Also add to technical debt      violations.forEach(v => {        this.metrics.codeQuality.techDebt.items.push({          file: v.file,          issue: `SRP Violation: ${v.details}`,          score: v.score,          recommendation: v.recommendation        });      });    }  }  /**   * Detect Open-Closed Principle violations in JavaScript/TypeScript code   */  detectOCPViolations(ast, filePath) {    const violations = [];        // Visit classes and track inheritance/extension patterns    traverse(ast, {      ClassDeclaration(path) {        const className = path.node.id?.name || 'Anonymous';                // Check for large switch statements or if/else chains that could indicate OCP violations        let largeConditionalBlocks = [];                // Find methods with switch statements or long if-else chains        path.traverse({          SwitchStatement(switchPath) {            // Count cases in switch statement            const caseCount = switchPath.node.cases?.length || 0;            if (caseCount > 3) {              // Get the parent function or method name              let methodName = 'unknown';              let parentFunc = switchPath.findParent(p => p.isClassMethod() || p.isFunctionDeclaration());              if (parentFunc && parentFunc.node.key) {                methodName = parentFunc.node.key.name;              }                            largeConditionalBlocks.push({                type: 'switch',                caseCount,                methodName,                loc: switchPath.node.loc              });            }          },                    // Track long if-else chains          IfStatement(ifPath) {            let chainLength = 1;            let current = ifPath;                        // Count consecutive else-if statements            while (current.node.alternate && current.node.alternate.type === 'IfStatement') {              chainLength++;              current = current.get('alternate');            }                        if (current.node.alternate) {              chainLength++; // Count the final else            }                        if (chainLength > 3) {              // Get the parent function or method name              let methodName = 'unknown';              let parentFunc = ifPath.findParent(p => p.isClassMethod() || p.isFunctionDeclaration());              if (parentFunc && parentFunc.node.key) {                methodName = parentFunc.node.key.name;              }                            largeConditionalBlocks.push({                type: 'if-else',                chainLength,                methodName,                loc: ifPath.node.loc              });            }          }        });                // If we found OCP violations, add them        if (largeConditionalBlocks.length > 0) {          // Calculate a score based on the number and size of conditional blocks          const score = Math.min(100, largeConditionalBlocks.reduce(            (sum, block) => sum + (block.type === 'switch' ? block.caseCount * 5 : block.chainLength * 7),             20          ));                    const details = largeConditionalBlocks.map(block =>             `${block.type === 'switch' ? 'Switch with' : 'If-else chain with'} ${block.type === 'switch' ? block.caseCount : block.chainLength} cases in method ${block.methodName}`          ).join('; ');                    violations.push({            type: 'OCP',            file: filePath,            name: className,            score,            details: `Potential violation with conditional logic: ${details}`,            recommendation: 'Consider using polymorphism or the Strategy pattern instead of conditional logic'          });        }      }    });        // Add violations to metrics    if (violations.length > 0) {      this.metrics.codeQuality.solidViolations.ocp.push(...violations);            // Also add to technical debt      violations.forEach(v => {        this.metrics.codeQuality.techDebt.items.push({          file: v.file,          issue: `OCP Violation: ${v.details}`,          score: v.score,          recommendation: v.recommendation        });      });    }  }/** * Detect Liskov Substitution Principle violations in JavaScript/TypeScript code */detectLSPViolations(ast, filePath) {  const violations = [];  const classHierarchy = new Map(); // Track class inheritance    // First pass: build class hierarchy  traverse(ast, {    ClassDeclaration(path) {      const className = path.node.id?.name || 'Anonymous';      const superClassName = path.node.superClass?.name || null;            // Store the class info      classHierarchy.set(className, {        superClass: superClassName,        methods: new Map(), // Will store method signatures        properties: new Set(), // Will store property names      });            // Extract methods and their parameters      path.node.body.body.forEach(member => {        if (member.type === 'ClassMethod' || member.type === 'MethodDefinition') {          const methodName = member.key.name;          const params = member.params.map(p => p.type);          classHierarchy.get(className).methods.set(methodName, params);        }         // Extract properties (for TypeScript classes with property declarations)        else if (member.type === 'ClassProperty') {          const propName = member.key.name;          classHierarchy.get(className).properties.add(propName);        }      });    }  });    // Second pass: check for LSP violations in subclasses  for (const [className, classInfo] of classHierarchy.entries()) {    if (!classInfo.superClass) continue; // Skip if not a subclass        const superClassInfo = classHierarchy.get(classInfo.superClass);    if (!superClassInfo) continue; // Super class not found in this file        // Check for method signature changes    for (const [methodName, superParams] of superClassInfo.methods.entries()) {      // If subclass overrides the method      if (classInfo.methods.has(methodName)) {        const subParams = classInfo.methods.get(methodName);                // Check if parameter count is different (potential LSP violation)        if (subParams.length !== superParams.length) {          violations.push({            type: 'LSP',            file: filePath,            name: className,            score: 60,            details: `Method ${methodName} changes parameter count from parent class ${classInfo.superClass}`,            recommendation: 'Ensure subclass methods maintain the same signature as parent class methods'          });        }      }    }  }    // Add violations to metrics  if (violations.length > 0) {    this.metrics.codeQuality.solidViolations.lsp.push(...violations);        // Also add to technical debt    violations.forEach(v => {      this.metrics.codeQuality.techDebt.items.push({        file: v.file,        issue: `LSP Violation: ${v.details}`,        score: v.score,        recommendation: v.recommendation      });    });  }}  /**   * Detect Interface Segregation Principle violations in JavaScript/TypeScript code   */  detectISPViolations(ast, filePath) {    const violations = [];        // For JavaScript, we can look for objects with too many properties    // or classes with many methods that aren't fully utilized by clients        traverse(ast, {      // Look for large interface-like objects or classes      ObjectExpression(path) {        const properties = path.node.properties.length;                // If the object has too many properties, it might violate ISP        if (properties > 10) {          let objectName = "Anonymous";                    // Try to get the variable name if it's part of a variable declaration          const varDecl = path.findParent(p => p.isVariableDeclarator());          if (varDecl && varDecl.node.id) {            objectName = varDecl.node.id.name;          }                    violations.push({            type: 'ISP',            file: filePath,            name: objectName,            score: Math.min(80, properties * 3),            details: `Large object with ${properties} properties might be violating ISP`,            recommendation: 'Consider splitting this object into smaller, more focused interfaces'          });        }      },            // Check for classes with many methods that could be split      ClassDeclaration(path) {        const className = path.node.id?.name || 'Anonymous';                // Count public methods (potential interface methods)        const methods = path.node.body.body.filter(member =>           (member.type === 'ClassMethod' || member.type === 'MethodDefinition') &&          (!member.accessibility || member.accessibility === 'public')        );                // Group methods by prefix to detect potential interfaces        const methodPrefixes = new Map();        methods.forEach(method => {          const methodName = method.key.name;          if (methodName === 'constructor') return;                    // Extract prefix (e.g., "get" from "getUserData")          const prefix = methodName.match(/^([a-z]+)[A-Z]/);          if (prefix && prefix[1]) {            if (!methodPrefixes.has(prefix[1])) {              methodPrefixes.set(prefix[1], []);            }            methodPrefixes.get(prefix[1]).push(methodName);          }        });                // If we have multiple method groups and many methods overall, suggest interface segregation        if (methods.length > 8 && methodPrefixes.size > 2) {          const details = Array.from(methodPrefixes.entries())            .map(([prefix, methods]) => `${prefix}* methods (${methods.length})`)            .join(', ');                      violations.push({            type: 'ISP',            file: filePath,            name: className,            score: Math.min(80, methods.length * 4),            details: `Class with ${methods.length} methods contains multiple responsibilities: ${details}`,            recommendation: 'Consider splitting this class into multiple interfaces based on method groups'          });        }      }    });        // Add violations to metrics    if (violations.length > 0) {      this.metrics.codeQuality.solidViolations.isp.push(...violations);            // Also add to technical debt      violations.forEach(v => {        this.metrics.codeQuality.techDebt.items.push({          file: v.file,          issue: `ISP Violation: ${v.details}`,          score: v.score,          recommendation: v.recommendation        });      });    }  }  /**   * Detect Dependency Inversion Principle violations in JavaScript/TypeScript code   */  detectDIPViolations(ast, filePath) {    const violations = [];        // Look for concrete class instantiations in constructors    traverse(ast, {      ClassDeclaration(path) {        const className = path.node.id?.name || 'Anonymous';        const concreteInstantiations = [];                // Find the constructor        const constructor = path.node.body.body.find(          node => node.type === 'ClassMethod' && node.key.name === 'constructor'        );                if (!constructor) return;                // Look for 'new' expressions in the constructor        path.traverse({          NewExpression(newExprPath) {            // Only check news in the constructor            const isInConstructor = newExprPath.findParent(              p => p.isClassMethod() && p.node.key.name === 'constructor'            );                        if (isInConstructor) {              const concreteName = newExprPath.node.callee.name;              if (concreteName) {                concreteInstantiations.push(concreteName);              }            }          }        });                // If we found concrete instantiations, report DIP violation        if (concreteInstantiations.length > 0) {          violations.push({            type: 'DIP',            file: filePath,            name: className,            score: 50 + (concreteInstantiations.length * 10),            details: `Class directly instantiates concrete classes in constructor: ${concreteInstantiations.join(', ')}`,            recommendation: 'Use dependency injection instead of direct instantiation'          });        }      }    });        // Add violations to metrics    if (violations.length > 0) {      this.metrics.codeQuality.solidViolations.dip.push(...violations);            // Also add to technical debt      violations.forEach(v => {        this.metrics.codeQuality.techDebt.items.push({          file: v.file,          issue: `DIP Violation: ${v.details}`,          score: v.score,          recommendation: v.recommendation        });      });    }  }/** * Detect polymorphism usage and violations in code */detectPolymorphism(ast, filePath) {  const polymorphismImplementations = [];  const polymorphismViolations = [];    // Track class hierarchy for polymorphism analysis  const classHierarchy = new Map();    // First pass: build class hierarchy  traverse(ast, {    ClassDeclaration(path) {      const className = path.node.id?.name || 'Anonymous';      const superClassName = path.node.superClass?.name || null;            if (!superClassName) return; // Not relevant for polymorphism if no parent            // Store the class and its methods      classHierarchy.set(className, {        superClass: superClassName,        methods: new Map(),        overriddenMethods: new Set()      });            // Extract methods      path.node.body.body.forEach(member => {        if (member.type === 'ClassMethod' || member.type === 'MethodDefinition') {          const methodName = member.key.name;          classHierarchy.get(className).methods.set(methodName, member);        }      });    }  });    // Second pass: analyze method overrides for polymorphism  for (const [className, classInfo] of classHierarchy.entries()) {    const superClassInfo = classHierarchy.get(classInfo.superClass);    if (!superClassInfo) continue; // Super class not found in this file        // Analyze methods that override parent class methods    for (const [methodName, method] of classInfo.methods.entries()) {      if (methodName === 'constructor') continue;            // Check if this method exists in the parent class      if (superClassInfo.methods.has(methodName)) {        // This is an overridden method - good for polymorphism        classInfo.overriddenMethods.add(methodName);                polymorphismImplementations.push({          file: filePath,          name: `${className}.${methodName}`,          details: `Method ${methodName} in class ${className} overrides parent class ${classInfo.superClass}`,          type: 'method-override',          score: 70 // Good score for proper polymorphism        });      }    }        // Check for typical polymorphism violations        // 1. Check for instanceof/type checks (often a polymorphism violation)    path.traverse({      IfStatement(ifPath) {        const test = ifPath.node.test;                // Check for instanceof expressions        if (test.type === 'BinaryExpression' &&             (test.operator === 'instanceof' ||              (test.operator === '===' && test.right.type === 'StringLiteral' && test.left.property?.name === 'name'))) {                    let methodName = 'unknown';          const parentFunc = ifPath.findParent(p => p.isClassMethod() || p.isFunctionDeclaration());          if (parentFunc?.node.key) {            methodName = parentFunc.node.key.name;          }                    polymorphismViolations.push({            file: filePath,            name: `${className}.${methodName}`,            details: `Type checking with ${test.operator === 'instanceof' ? 'instanceof' : 'constructor.name'} instead of using polymorphism`,            type: 'type-checking',            score: 65,            recommendation: 'Replace type checking with polymorphic method calls'          });        }      }    });  }    // Find usage of polymorphism (method calls on parent type variables)  traverse(ast, {    CallExpression(path) {      // Look for obj.method() pattern where obj could be polymorphic      if (path.node.callee.type === 'MemberExpression') {        const methodName = path.node.callee.property.name;                // Check if this methodName exists in multiple classes in our hierarchy        let polymorphicMethodCount = 0;        let implementingClasses = [];                for (const [className, classInfo] of classHierarchy.entries()) {          if (classInfo.overriddenMethods.has(methodName)) {            polymorphicMethodCount++;            implementingClasses.push(className);          }        }                if (polymorphicMethodCount > 1) {          polymorphismImplementations.push({            file: filePath,            name: methodName,            details: `Method ${methodName} is potentially used polymorphically (implemented by ${implementingClasses.join(', ')})`,            type: 'polymorphic-usage',            score: 60          });        }      }    }  });    // Store results in metrics  if (!this.metrics.codeQuality.designPatterns) {    this.metrics.codeQuality.designPatterns = {      polymorphism: { implementations: [], violations: [] },      dependencyInjection: { implementations: [], violations: [] },      ioc: { implementations: [], violations: [] }    };  }    this.metrics.codeQuality.designPatterns.polymorphism.implementations.push(...polymorphismImplementations);  this.metrics.codeQuality.designPatterns.polymorphism.violations.push(...polymorphismViolations);    // Also add violations to technical debt  polymorphismViolations.forEach(v => {    this.metrics.codeQuality.techDebt.items.push({      file: v.file,      issue: `Polymorphism Violation: ${v.details}`,      score: v.score,      recommendation: v.recommendation || 'Use proper inheritance and method overriding'    });  });}/** * Detect dependency injection patterns and violations */detectDependencyInjection(ast, filePath) {  const diImplementations = [];  const diViolations = [];    // Look for constructor dependency injection pattern  traverse(ast, {    ClassDeclaration(path) {      const className = path.node.id?.name || 'Anonymous';      const constructor = path.node.body.body.find(        node => node.type === 'ClassMethod' && node.key.name === 'constructor'      );            if (!constructor) return;            // Check for dependencies passed to constructor      const params = constructor.params || [];      const dependencies = [];      const savedDeps = new Set();            // Look for dependencies saved to instance variables      path.traverse({        AssignmentExpression(assignPath) {          // Check for this.something = param pattern          if (assignPath.node.left.type === 'MemberExpression' &&               assignPath.node.left.object.type === 'ThisExpression') {                        const varName = assignPath.node.left.property.name;                        // If right side is an identifier that matches a parameter            if (assignPath.node.right.type === 'Identifier') {              const paramName = assignPath.node.right.name;              const paramIndex = params.findIndex(p => p.name === paramName);                            if (paramIndex >= 0) {                savedDeps.add(paramName);                dependencies.push({                  param: paramName,                  instanceVar: varName,                  isService: varName.endsWith('Service') ||                              varName.endsWith('Repository') ||                              varName.endsWith('Manager') ||                             varName.endsWith('Provider')                });              }            }          }        }      });            // If we have dependencies, this might be DI      if (dependencies.length > 0) {        const serviceCount = dependencies.filter(d => d.isService).length;                // Calculate a DI quality score        const diScore = Math.min(100, 40 + (serviceCount * 10));                diImplementations.push({          file: filePath,          name: className,          details: `Class receives ${dependencies.length} dependencies via constructor (${serviceCount} likely services)`,          type: 'constructor-injection',          score: diScore,          dependencies: dependencies.map(d => d.instanceVar)        });      }            // Check for DI violations: instances created with 'new' inside class      const newExpressions = [];      path.traverse({        NewExpression(newPath) {          // Ignore 'new' for basic types (Date, Map, etc.)          const basicTypes = ['Array', 'Object', 'Date', 'Map', 'Set', 'Promise', 'RegExp'];          const className = newPath.node.callee.name;                    if (className && !basicTypes.includes(className)) {            let methodName = 'unknown';            const parentMethod = newPath.findParent(p => p.isClassMethod());            if (parentMethod?.node.key) {              methodName = parentMethod.node.key.name;            }                        newExpressions.push({              className,              methodName            });          }        }      });            // Report violations for service-looking classes created with 'new'      newExpressions.forEach(expr => {        if (expr.className.endsWith('Service') ||             expr.className.endsWith('Repository') ||             expr.className.endsWith('Manager') ||            expr.className.endsWith('Factory') ||            expr.className.endsWith('Provider')) {                    diViolations.push({            file: filePath,            name: `${className}.${expr.methodName}`,            details: `Creates service '${expr.className}' with 'new' instead of using dependency injection`,            type: 'new-service-instance',            score: 75,            recommendation: 'Inject this dependency through constructor instead of creating it directly'          });        }      });    }  });    // Store results in metrics  this.metrics.codeQuality.designPatterns.dependencyInjection.implementations.push(...diImplementations);  this.metrics.codeQuality.designPatterns.dependencyInjection.violations.push(...diViolations);    // Also add violations to technical debt  diViolations.forEach(v => {    this.metrics.codeQuality.techDebt.items.push({      file: v.file,      issue: `DI Violation: ${v.details}`,      score: v.score,      recommendation: v.recommendation || 'Use proper dependency injection'    });  });}/** * Detect Inversion of Control patterns and violations */detectIoC(ast, filePath) {  const iocImplementations = [];  const iocViolations = [];    // Look for IoC container/registration patterns  let hasIoCContainer = false;    traverse(ast, {    // Look for signs of an IoC container    CallExpression(path) {      const callee = path.node.callee;      if (callee.type !== 'MemberExpression') return;            const methodName = callee.property.name;      const objectName = callee.object.name ||                         (callee.object.type === 'MemberExpression' ? callee.object.property.name : '');            // Common IoC container registration methods      const iocRegisterMethods = [        'register', 'registerSingleton', 'registerTransient', 'addSingleton',         'addTransient', 'bind', 'provide', 'service', 'factory'      ];            if (iocRegisterMethods.includes(methodName)) {        hasIoCContainer = true;        iocImplementations.push({          file: filePath,          name: objectName || 'IoC container',          details: `IoC registration with '${methodName}'`,          type: 'registration',          score: 80        });      }            // Common IoC container resolution methods      const iocResolveMethods = [        'resolve', 'get', 'getService', 'make', 'createInstance', 'inject'      ];            if (iocResolveMethods.includes(methodName)) {        hasIoCContainer = true;        iocImplementations.push({          file: filePath,          name: objectName || 'IoC container',          details: `IoC service resolution with '${methodName}'`,          type: 'resolution',          score: 75        });      }    },        // IoC-related imports    ImportDeclaration(path) {      const source = path.node.source.value;            // Common IoC libraries      const iocLibraries = [        'inversify', 'tsyringe', 'typedi', 'awilix', 'injection', 'di',         'container', 'service-locator', 'dependency-injection'      ];            if (iocLibraries.some(lib => source.includes(lib))) {        hasIoCContainer = true;        iocImplementations.push({          file: filePath,          name: source,          details: `Using IoC library: ${source}`,          type: 'library-usage',          score: 90        });      }    },        // Look for decorators that might be IoC-related    Decorator(path) {      const expression = path.node.expression;      let decoratorName;            if (expression.type === 'Identifier') {        decoratorName = expression.name;      } else if (expression.type === 'CallExpression' && expression.callee.type === 'Identifier') {        decoratorName = expression.callee.name;      }            // Common IoC decorators      const iocDecorators = [        'Injectable', 'Service', 'Inject', 'Singleton', 'Provides',        'Component', 'Autowired', 'Dependency'      ];            if (decoratorName && iocDecorators.includes(decoratorName)) {        hasIoCContainer = true;                let targetName = 'unknown';        const parent = path.parent;        if (parent.type === 'ClassDeclaration' && parent.id) {          targetName = parent.id.name;        } else if (parent.type === 'ClassMethod' && parent.key) {          targetName = parent.key.name;        } else if (parent.type === 'ClassProperty' && parent.key) {          targetName = parent.key.name;        }                iocImplementations.push({          file: filePath,          name: targetName,          details: `IoC decorator: @${decoratorName}`,          type: 'decorator',          score: 90        });      }    }  });    // Look for potential IoC violations  // For example, explicit dependency instantiation in a file with IoC patterns  if (hasIoCContainer) {    traverse(ast, {      NewExpression(path) {        // Ignore 'new' for basic types (Date, Map, etc.)        const basicTypes = ['Array', 'Object', 'Date', 'Map', 'Set', 'Promise', 'RegExp'];        const className = path.node.callee.name;                if (className && !basicTypes.includes(className) &&            (className.endsWith('Service') || className.endsWith('Repository'))) {                    let methodName = 'unknown';          const parentFunc = path.findParent(p => p.isFunction());          if (parentFunc && parentFunc.node.id) {            methodName = parentFunc.node.id.name;          } else if (parentFunc && parentFunc.node.key) {            methodName = parentFunc.node.key.name;          }                    iocViolations.push({            file: filePath,            name: methodName,            details: `Creates service '${className}' with 'new' while using IoC elsewhere`,            type: 'inconsistent-instantiation',            score: 70,            recommendation: 'Resolve this dependency from the IoC container instead of creating it directly'          });        }      }    });  }    // Store results in metrics  this.metrics.codeQuality.designPatterns.ioc.implementations.push(...iocImplementations);  this.metrics.codeQuality.designPatterns.ioc.violations.push(...iocViolations);    // Also add violations to technical debt  iocViolations.forEach(v => {    this.metrics.codeQuality.techDebt.items.push({      file: v.file,      issue: `IoC Violation: ${v.details}`,      score: v.score,      recommendation: v.recommendation || 'Use the IoC container consistently throughout the codebase'    });  });}  /**   * Generate a report of code smells   */  async generateCodeSmellsReport() {    console.log("Generating code smells report...");        const docsDir = path.join(this.baseDir, 'docs');    if (!fs.existsSync(docsDir)) {      fs.mkdirSync(docsDir, { recursive: true });    }        // Get all violations    const solidViolations = this.metrics.codeQuality.solidViolations;    const designPatterns = this.metrics.codeQuality.designPatterns || {      polymorphism: { implementations: [], violations: [] },      dependencyInjection: { implementations: [], violations: [] },      ioc: { implementations: [], violations: [] }    };        // Generate markdown report    let reportContent = `# Code Quality Analysis Report\n\n`;        // Function to create a section for SOLID principles    const createPrincipleSection = (violations, title, description) => {      let section = `## ${title}\n\n`;      section += `${description}\n\n`;      section += `Found **${violations.length}** potential violations.\n\n`;            if (violations.length > 0) {        section += `| File | Component | Score | Details | Recommendation |\n`;        section += `|------|-----------|-------|---------|----------------|\n`;                violations          .sort((a, b) => b.score - a.score)          .forEach(v => {            section += `| ${v.file.replace(/\\/g, '/')} | ${v.name} | ${v.score} | ${v.details} | ${v.recommendation || '-'} |\n`;          });      }            return section + "\n";    };        // Function to create a section for design patterns    const createPatternSection = (pattern, title, description) => {      const implementations = pattern.implementations || [];      const violations = pattern.violations || [];            let section = `## ${title}\n\n`;      section += `${description}\n\n`;      section += `Found **${implementations.length}** implementations and **${violations.length}** violations.\n\n`;            if (implementations.length > 0) {        section += `### Implementations\n\n`;        section += `| File | Component | Type | Details |\n`;        section += `|------|-----------|------|--------|\n`;                implementations          .sort((a, b) => b.score - a.score)          .forEach(imp => {            section += `| ${imp.file.replace(/\\/g, '/')} | ${imp.name} | ${imp.type} | ${imp.details} |\n`;          });        section += "\n";      }            if (violations.length > 0) {        section += `### Violations\n\n`;        section += `| File | Component | Details | Recommendation |\n`;        section += `|------|-----------|---------|----------------|\n`;                violations          .sort((a, b) => b.score - a.score)          .forEach(v => {            section += `| ${v.file.replace(/\\/g, '/')} | ${v.name} | ${v.details} | ${v.recommendation || '-'} |\n`;          });      }            return section + "\n";    };        // Add sections for each SOLID principle    reportContent += `# SOLID Principles Analysis\n\n`;        reportContent += createPrincipleSection(      solidViolations.srp,      "Single Responsibility Principle Violations",      "A class should have only one reason to change."    );        reportContent += createPrincipleSection(      solidViolations.ocp,      "Open-Closed Principle Violations",      "Software entities should be open for extension, but closed for modification."    );        reportContent += createPrincipleSection(      solidViolations.lsp,      "Liskov Substitution Principle Violations",      "Subtypes must be substitutable for their base types."    );        reportContent += createPrincipleSection(      solidViolations.isp,      "Interface Segregation Principle Violations",      "Clients should not be forced to depend on methods they do not use."    );        reportContent += createPrincipleSection(      solidViolations.dip,      "Dependency Inversion Principle Violations",      "High-level modules should not depend on low-level modules. Both should depend on abstractions."    );        // Add sections for design patterns    reportContent += `# Design Patterns Analysis\n\n`;        reportContent += createPatternSection(      designPatterns.polymorphism,      "Polymorphism",      "The ability to present the same interface for differing underlying forms (data types)."    );        reportContent += createPatternSection(      designPatterns.dependencyInjection,      "Dependency Injection",      "A technique whereby one object supplies the dependencies of another object."    );        reportContent += createPatternSection(      designPatterns.ioc,      "Inversion of Control (IoC)",      "A design principle in which control flow is inverted compared to traditional programming."    );        // Add a summary section    const totalSolidViolations = solidViolations.srp.length +                             solidViolations.ocp.length +                             solidViolations.lsp.length +                            solidViolations.isp.length +                            solidViolations.dip.length;                                const totalPatternViolations = designPatterns.polymorphism.violations.length +                              designPatterns.dependencyInjection.violations.length +                              designPatterns.ioc.violations.length;                                   const totalPatternImplementations = designPatterns.polymorphism.implementations.length +                                   designPatterns.dependencyInjection.implementations.length +                                   designPatterns.ioc.implementations.length;        reportContent = `# Code Quality Analysis Report\n\n` +                   `**Total SOLID Violations:** ${totalSolidViolations}\n` +                   `**Total Pattern Implementations:** ${totalPatternImplementations}\n` +                   `**Total Pattern Violations:** ${totalPatternViolations}\n\n` +                   `## SOLID Principles Summary\n\n` +                   `| Principle | Violations |\n` +                   `|-----------|------------|\n` +                   `| Single Responsibility | ${solidViolations.srp.length} |\n` +                   `| Open-Closed | ${solidViolations.ocp.length} |\n` +                   `| Liskov Substitution | ${solidViolations.lsp.length} |\n` +                   `| Interface Segregation | ${solidViolations.isp.length} |\n` +                   `| Dependency Inversion | ${solidViolations.dip.length} |\n\n` +                   `## Design Patterns Summary\n\n` +                   `| Pattern | Implementations | Violations |\n` +                   `|---------|-----------------|------------|\n` +                   `| Polymorphism | ${designPatterns.polymorphism.implementations.length} | ${designPatterns.polymorphism.violations.length} |\n` +                   `| Dependency Injection | ${designPatterns.dependencyInjection.implementations.length} | ${designPatterns.dependencyInjection.violations.length} |\n` +                   `| Inversion of Control | ${designPatterns.ioc.implementations.length} | ${designPatterns.ioc.violations.length} |\n\n` +                   reportContent;        // Save to a file    try {      fs.writeFileSync(path.join(this.baseDir, 'docs', 'code_quality_report.md'), reportContent);      console.log("Code quality report saved to docs/code_quality_report.md");    } catch (err) {      console.error("Error saving code quality report:", err.message);    }  }  /**   * Analyze source systems (Canvas, Discourse)   */  async analyzeSourceSystems() {    console.log("Analyzing source systems...");        for (const [system, path] of Object.entries(this.sourceSystems)) {      if (!path || !fs.existsSync(path)) {        console.log(`Source system '${system}' path not found: ${path}`);        continue;      }            console.log(`Analyzing source system: ${system} at ${path}`);            // Create temporary FileSystemUtils for this source system      const sourceFS = new FileSystemUtils(path, this.getExcludePatterns());      sourceFS.discoverFiles();      sourceFS.readFileContents();            // Count files by type      const filesByType = this.countSourceFilesByType(system, sourceFS);      this.metrics.sourceSystems[system].filesByType = filesByType;            // Find and analyze models      await this.analyzeSourceModels(system, sourceFS);            // Find and analyze controllers      await this.analyzeSourceControllers(system, sourceFS);    }        // After analyzing source systems, map source to target    this.mapSourceToTarget();  }  /**   * Count files by type in source system   */  countSourceFilesByType(system, sourceFS) {    const result = {};        // Define file type patterns    const patterns = {      ruby: /\.rb$/,      javascript: /\.js$/,      typescript: /\.ts$/,      jsx: /\.jsx$/,      tsx: /\.tsx$/,      css: /\.css$/,      scss: /\.scss$/,      html: /\.html$/,      erb: /\.erb$/,      haml: /\.haml$/,      yaml: /\.ya?ml$/,      json: /\.json$/,      markdown: /\.md$/,      // Add more patterns as needed    };        // Count files for each pattern    for (const [type, pattern] of Object.entries(patterns)) {      const files = sourceFS.filterFiles(pattern);      result[type] = files.length;    }        return result;  }  /**   * Analyze models in source system   */  async analyzeSourceModels(system, sourceFS) {    console.log(`Analyzing ${system} models...`);        // Define patterns based on the system    let modelFiles = [];        if (system === 'canvas') {      modelFiles = sourceFS.filterFiles(/app\/models\/.*\.rb$/);    } else if (system === 'discourse') {      modelFiles = sourceFS.filterFiles(/app\/models\/.*\.rb$/);    }        for (const file of modelFiles) {      const content = sourceFS.getFileContent(file);      if (!content) continue;            // Extract the model name from file path      const modelName = path.basename(file, '.rb');      const modelNameCamelCase = modelName        .split('_')        .map((word, index) => index === 0 ? word : word.charAt(0).toUpperCase() + word.slice(1))        .join('');            this.metrics.sourceSystems[system].models.total++;      this.metrics.sourceSystems[system].models.details.push({        name: modelNameCamelCase, // Convert to similar naming convention as target        file: file,        properties: this.extractRubyModelProperties(content)      });    }        console.log(`Found ${this.metrics.sourceSystems[system].models.total} models in ${system}`);  }  /**   * Extract model properties from Ruby code   */  extractRubyModelProperties(content) {    const properties = [];        // Look for ActiveRecord-style property definitions    const attributeRegex = /attribute\s+:(\w+)/g;    let match;    while (match = attributeRegex.exec(content)) {      properties.push({name: match[1], type: 'attribute'});    }        // Look for database columns from schema    const columnRegex = /t\.(\w+)\s+["'](\w+)["']/g;    while (match = columnRegex.exec(content)) {      properties.push({name: match[2], type: match[1]});    }        // Look for relations    const hasOneRegex = /has_one\s+:(\w+)/g;    while (match = hasOneRegex.exec(content)) {      properties.push({name: match[1], type: 'relation', relationType: 'hasOne'});    }        const hasManyRegex = /has_many\s+:(\w+)/g;    while (match = hasManyRegex.exec(content)) {      properties.push({name: match[1], type: 'relation', relationType: 'hasMany'});    }        const belongsToRegex = /belongs_to\s+:(\w+)/g;    while (match = belongsToRegex.exec(content)) {      properties.push({name: match[1], type: 'relation', relationType: 'belongsTo'});    }        return properties;  }  /**   * Analyze controllers in source system   */  async analyzeSourceControllers(system, sourceFS) {    console.log(`Analyzing ${system} controllers...`);        // Define patterns based on the system    let controllerFiles = [];        if (system === 'canvas') {      controllerFiles = sourceFS.filterFiles(/app\/controllers\/.*\.rb$/);    } else if (system === 'discourse') {      controllerFiles = sourceFS.filterFiles(/app\/controllers\/.*\.rb$/);    }        for (const file of controllerFiles) {      const content = sourceFS.getFileContent(file);      if (!content) continue;            // Extract controller name and actions      const controllerName = path.basename(file, '_controller.rb');      const actions = this.extractRubyControllerActions(content);            this.metrics.sourceSystems[system].controllers.total++;      this.metrics.sourceSystems[system].controllers.details.push({        name: controllerName,        file: file,        actions: actions      });    }        console.log(`Found ${this.metrics.sourceSystems[system].controllers.total} controllers in ${system}`);  }  /**   * Extract controller actions from Ruby code   */  extractRubyControllerActions(content) {    const actions = [];        // Look for method definitions that are likely controller actions    const actionRegex = /def\s+(\w+)[^]*?end/g;    let match;        while (match = actionRegex.exec(content)) {      const actionName = match[1];      const actionBody = match[0];            // Skip if this is likely a private method or not an action      if (actionName.startsWith('_') ||           ['initialize', 'require_login', 'before_action'].includes(actionName)) {        continue;      }            // Determine HTTP method based on content      let httpMethod = 'GET';      if (actionBody.includes('create') ||           actionBody.includes('new_record') ||           actionBody.includes('save')) {        httpMethod = 'POST';      } else if (actionBody.includes('update') ||                 actionBody.includes('update_attributes')) {        httpMethod = 'PUT';      } else if (actionBody.includes('destroy') ||                 actionBody.includes('delete')) {        httpMethod = 'DELETE';      }            actions.push({        name: actionName,        httpMethod: httpMethod,        // Try to determine route pattern based on controller/action naming        routePattern: `/${actionName}`      });    }        return actions;  }  /**   * Map source models/controllers to target implementation   */  mapSourceToTarget() {    console.log("Mapping source system entities to target implementation...");        // Map models: Try to find corresponding models in target by name    for (const system of Object.keys(this.sourceSystems)) {      if (!this.metrics.sourceSystems[system]) continue;            // Map models      for (const sourceModel of this.metrics.sourceSystems[system].models.details) {        // Look for a target model with similar name        const targetModel = this.metrics.models.details.find(m =>           m.name.toLowerCase() === sourceModel.name.toLowerCase() ||          m.name.toLowerCase() === this.singularize(sourceModel.name.toLowerCase())        );                if (targetModel) {          this.metrics.sourceToTarget.models.push({            sourceSystem: system,            sourceName: sourceModel.name,            sourceFile: sourceModel.file,            targetName: targetModel.name,            targetFile: targetModel.file,            completeness: targetModel.completeness          });        }      }            // Map controllers to API handlers      for (const sourceController of this.metrics.sourceSystems[system].controllers.details) {        // For each controller action, try to find corresponding API handler        for (const action of sourceController.actions) {          const apiEndpoint = this.metrics.apiEndpoints.details.find(api =>             api.name.toLowerCase().includes(sourceController.name.toLowerCase()) ||             api.name.toLowerCase().includes(action.name.toLowerCase()) ||            (api.routePath && api.routePath.toLowerCase().includes(sourceController.name.toLowerCase()))          );                    if (apiEndpoint) {            this.metrics.sourceToTarget.controllers.push({              sourceSystem: system,              sourceName: `${sourceController.name}#${action.name}`,              sourceFile: sourceController.file,              targetHandler: apiEndpoint.name,              targetFile: apiEndpoint.file,              targetRoute: apiEndpoint.routePath,              completeness: apiEndpoint.completeness            });          }        }      }    }  }  /**   * Helper to convert plural to singular form (basic implementation)   */  singularize(word) {    // Very basic implementation - just handles common cases    if (word.endsWith('ies')) {      return word.slice(0, -3) + 'y';    } else if (word.endsWith('s')) {      return word.slice(0, -1);    }    return word;  }  /**   * Generate a source system comparison report   */  async generateSourceComparisonReport() {    console.log("Generating source system comparison report...");        const docsDir = path.join(this.baseDir, 'docs');    if (!fs.existsSync(docsDir)) {      fs.mkdirSync(docsDir, { recursive: true });    }        let reportContent = `# Source System to Target Implementation Comparison\n\n`;    reportContent += `_Last updated: ${new Date().toISOString().split('T')[0]}_\n\n`;        // Add overview of source systems
    reportContent += `## Overview\n\n`;
    reportContent += `| Metric | Canvas | Discourse | Target Implementation |\n`;
    reportContent += `|--------|--------|-----------|----------------------|\n`;
    reportContent += `| Models | ${this.metrics.sourceSystems.canvas?.models?.total || 0} | ${this.metrics.sourceSystems.discourse?.models?.total || 0} | ${this.metrics.models.total} |\n`;
    reportContent += `| Controllers/API | ${this.metrics.sourceSystems.canvas?.controllers?.total || 0} | ${this.metrics.sourceSystems.discourse?.controllers?.total || 0} | ${this.metrics.apiEndpoints.total} |\n`;
    reportContent += `| UI Components | N/A | N/A | ${this.metrics.uiComponents.total} |\n`;
    reportContent += `| Total Files | ${this.sumFilesByType(this.metrics.sourceSystems.canvas?.filesByType)} | ${this.sumFilesByType(this.metrics.sourceSystems.discourse?.filesByType)} | ${this.fsUtils.getAllFiles().length} |\n\n`;
    
    // Add model mapping details
    reportContent += `## Model Mappings\n\n`;
    reportContent += `### Canvas Models\n\n`;
    reportContent += this.generateSourceModelTable('canvas');
    
    reportContent += `\n### Discourse Models\n\n`;
    reportContent += this.generateSourceModelTable('discourse');
    
    // Add controller mapping details
    reportContent += `\n## API Endpoint Mappings\n\n`;
    reportContent += `### Canvas Controllers\n\n`;
    reportContent += this.generateSourceControllerTable('canvas');
    
    reportContent += `\n### Discourse Controllers\n\n`;
    reportContent += this.generateSourceControllerTable('discourse');
    
    // Add implementation gap analysis
    reportContent += `\n## Implementation Gap Analysis\n\n`;
    reportContent += `### Missing Models\n\n`;
    reportContent += this.generateMissingModelsTable();
    
    reportContent += `\n### Missing API Endpoints\n\n`;
    reportContent += this.generateMissingControllersTable();
    
    // Add implementation recommendation
    reportContent += `\n## Implementation Recommendations\n\n`;
    reportContent += this.generateImplementationRecommendations();
    
    // Save to file
    try {
      fs.writeFileSync(path.join(this.baseDir, 'docs', 'source_system_comparison.md'), reportContent);
      console.log("Source system comparison report saved to docs/source_system_comparison.md");
    } catch (err) {
      console.error("Error saving source system comparison report:", err.message);
    }
  }

  /**
   * Helper to sum files by type
   */
  sumFilesByType(filesByType) {
    if (!filesByType) return 0;
    return Object.values(filesByType).reduce((sum, count) => sum + count, 0);
  }

  /**
   * Generate table for source system models
   */
  generateSourceModelTable(system) {
    let table = `| Source Model | Properties | Target Model | Completeness |\n`;
    table += `|--------------|------------|-------------|-------------|\n`;
    
    const sourceModels = this.metrics.sourceSystems[system]?.models?.details || [];
    
    if (sourceModels.length === 0) {
      return `No models found in ${system}.\n`;
    }
    
    for (const model of sourceModels) {
      // Find mapping to target
      const mapping = this.metrics.sourceToTarget.models.find(m => 
        m.sourceSystem === system && m.sourceName === model.name);
      
      const targetModel = mapping ? mapping.targetName : '-';
      const completeness = mapping ? `${mapping.completeness}%` : 'Not implemented';
      const propertyCount = model.properties ? model.properties.length : 0;
      
      table += `| ${model.name} | ${propertyCount} | ${targetModel} | ${completeness} |\n`;
    }
    
    return table;
  }

  /**
   * Generate table for source system controllers
   */
  generateSourceControllerTable(system) {
    let table = `| Source Controller | Action | HTTP Method | Target API | Route | Completeness |\n`;
    table += `|-------------------|--------|------------|-----------|-------|-------------|\n`;
    
    const sourceControllers = this.metrics.sourceSystems[system]?.controllers?.details || [];
    
    if (sourceControllers.length === 0) {
      return `No controllers found in ${system}.\n`;
    }
    
    for (const controller of sourceControllers) {
      for (const action of controller.actions) {
        // Find mapping to target
        const mapping = this.metrics.sourceToTarget.controllers.find(m => 
          m.sourceSystem === system && m.sourceName === `${controller.name}#${action.name}`);
        
        const targetAPI = mapping ? mapping.targetHandler : '-';
        const route = mapping ? mapping.targetRoute || '-' : '-';
        const completeness = mapping ? `${mapping.completeness}%` : 'Not implemented';
        
        table += `| ${controller.name} | ${action.name} | ${action.httpMethod} | ${targetAPI} | ${route} | ${completeness} |\n`;
      }
    }
    
    return table;
  }

  /**
   * Generate table for missing models
   */
  generateMissingModelsTable() {
    let table = `| Source System | Model | Priority |\n`;
    table += `|--------------|-------|----------|\n`;
    
    let hasMissingModels = false;
    
    // For each source system
    for (const system of Object.keys(this.sourceSystems)) {
      if (!this.metrics.sourceSystems[system]) continue;
      
      // For each source model
      for (const sourceModel of this.metrics.sourceSystems[system]?.models?.details || []) {
        // Check if it has a mapping
        const hasMapping = this.metrics.sourceToTarget.models.some(m => 
          m.sourceSystem === system && m.sourceName === sourceModel.name);
        
        if (!hasMapping) {
          hasMissingModels = true;
          
          // Determine priority based on property count or other heuristics
          const propertyCount = sourceModel.properties?.length || 0;
          let priority = 'Low';
          if (propertyCount > 10) priority = 'High';
          else if (propertyCount > 5) priority = 'Medium';
          
          table += `| ${system} | ${sourceModel.name} | ${priority} |\n`;
        }
      }
    }
    
    if (!hasMissingModels) {
      table += `| - | - | - |\n`;
    }
    
    return table;
  }

  /**
   * Generate table for missing controllers/API endpoints
   */
  generateMissingControllersTable() {
    let table = `| Source System | Controller | Action | HTTP Method | Priority |\n`;
    table += `|--------------|-----------|--------|------------|----------|\n`;
    
    let hasMissingControllers = false;
    
    // For each source system
    for (const system of Object.keys(this.sourceSystems)) {
      if (!this.metrics.sourceSystems[system]) continue;
      
      // For each source controller
      for (const controller of this.metrics.sourceSystems[system]?.controllers?.details || []) {
        for (const action of controller.actions) {
          // Check if it has a mapping
          const hasMapping = this.metrics.sourceToTarget.controllers.some(m => 
            m.sourceSystem === system && m.sourceName === `${controller.name}#${action.name}`);
          
          if (!hasMapping) {
            hasMissingControllers = true;
            
            // Determine priority based on controller/action name
            let priority = 'Low';
            const name = controller.name.toLowerCase();
            
            // Critical controllers get high priority
            if (name.includes('user') || 
                name.includes('auth') || 
                name.includes('course') ||
                name.includes('grade') ||
                name.includes('topic')) {
              priority = 'High';
            } else if (name.includes('settings') || 
                      name.includes('notification') ||
                      name.includes('file') ||
                      name.includes('message')) {
              priority = 'Medium';
            }
            
            table += `| ${system} | ${controller.name} | ${action.name} | ${action.httpMethod} | ${priority} |\n`;
          }
        }
      }
    }
    
    if (!hasMissingControllers) {
      table += `| - | - | - | - | - |\n`;
    }
    
    return table;
  }

  /**
   * Generate implementation recommendations based on source system analysis
   */
  generateImplementationRecommendations() {
    let recommendations = '';
    
    // Count missing high priority items
    let missingHighPriorityModels = 0;
    let missingHighPriorityControllers = 0;
    
    // For each source system
    for (const system of Object.keys(this.sourceSystems)) {
      if (!this.metrics.sourceSystems[system]) continue;
      
      // Count missing high priority models
      for (const sourceModel of this.metrics.sourceSystems[system]?.models?.details || []) {
        const hasMapping = this.metrics.sourceToTarget.models.some(m => 
          m.sourceSystem === system && m.sourceName === sourceModel.name);
        
        if (!hasMapping) {
          const propertyCount = sourceModel.properties?.length || 0;
          if (propertyCount > 10) missingHighPriorityModels++;
        }
      }
      
      // Count missing high priority controllers
      for (const controller of this.metrics.sourceSystems[system]?.controllers?.details || []) {
        for (const action of controller.actions) {
          const hasMapping = this.metrics.sourceToTarget.controllers.some(m => 
            m.sourceSystem === system && m.sourceName === `${controller.name}#${action.name}`);
          
          if (!hasMapping) {
            const name = controller.name.toLowerCase();
            if (name.includes('user') || 
                name.includes('auth') || 
                name.includes('course') ||
                name.includes('grade') ||
                name.includes('topic')) {
              missingHighPriorityControllers++;
            }
          }
        }
      }
    }
    
    // Generate recommendations
    recommendations += `### Implementation Priorities\n\n`;
    recommendations += `Based on the source system analysis, here are the key implementation priorities:\n\n`;
    
    if (missingHighPriorityModels > 0) {
      recommendations += `1. **Implement Missing High-Priority Models** (${missingHighPriorityModels} missing)\n`;
      recommendations += `   - These models have many properties and relationships that are critical to the system\n`;
    }
    
    if (missingHighPriorityControllers > 0) {
      recommendations += `2. **Implement Missing High-Priority API Endpoints** (${missingHighPriorityControllers} missing)\n`;
      recommendations += `   - Focus on user, authentication, and core functionality endpoints\n`;
    }
    
    recommendations += `3. **Improve Existing Model Implementations**\n`;
    recommendations += `   - Enhance models that have low completeness scores\n`;
    recommendations += `   - Ensure all critical properties from source systems are represented\n\n`;
    
    recommendations += `4. **Address Integration Challenges**\n`;
    recommendations += `   - Reconcile differences between Canvas and Discourse data models\n`;
    recommendations += `   - Implement adapter patterns where necessary to bridge differences\n\n`;
    
    return recommendations;
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

// API DATA STRUCTURES

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicRequest {
    title: String,
    content: String,
    category_id: i64,
    tags: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostRequest {
    content: String,
    parent_id: Option<i64>
}

// ROUTE HANDLERS

// Category handlers
async fn get_categories(
    State(state): State<Arc<AppState>>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumCategoryRepository::new(&state.db);
    let categories = repo.get_all().await?;
    Ok(axum::Json(categories))
}

async fn get_categories_by_course(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(course_id): axum::extract::Path<i64>
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumCategoryRepository::new(&state.db);
    let categories = repo.get_by_course_id(course_id).await?;
    Ok(axum::Json(categories))
}

// Topic handlers
async fn get_topics(
    State(state): State<Arc<AppState>>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumTopicRepository::new(&state.db);
    let topics = repo.get_all().await?;
    Ok(axum::Json(topics))
}

async fn create_topic(
    State(state): State<Arc<AppState>>,
    axum::Json(topic_req): axum::Json<TopicRequest>
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumTopicRepository::new(&state.db);
    let topic = repo.create(topic_req.title, topic_req.content, topic_req.category_id, topic_req.tags).await?;
    Ok(axum::Json(topic))
}

async fn get_topic(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumTopicRepository::new(&state.db);
    let topic = repo.get_by_id(id).await?;
    Ok(axum::Json(topic))
}

async fn update_topic(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    axum::Json(topic_req): axum::Json<TopicRequest>
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumTopicRepository::new(&state.db);
    let topic = repo.update(id, topic_req.title, topic_req.content).await?;
    Ok(axum::Json(topic))
}

async fn delete_topic(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumTopicRepository::new(&state.db);
    repo.delete(id).await?;
    Ok(axum::Json(()))
}

// Post handlers
async fn get_posts_by_topic(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(topic_id): axum::extract::Path<i64>
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumPostRepository::new(&state.db);
    let posts = repo.get_by_topic_id(topic_id).await?;
    Ok(axum::Json(posts))
}

async fn create_post(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(topic_id): axum::extract::Path<i64>,
    axum::Json(post_req): axum::Json<PostRequest>
) -> Result<impl axum::response::IntoResponse, AppError> {
    let repo = ForumPostRepository::new(&state.db);
    let post = repo.create(post_req.content, topic_id, post_req.parent_id).await?;
    Ok(axum::Json(post))
}

// Error handling
#[derive(Debug, Serialize)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    InternalError(String)
}

// Configure forum API routes
pub fn forum_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Category routes
        .route("/categories", get(get_categories))
        .route("/courses/:id/categories", get(get_categories_by_course))
        
        // Topic routes
        .route("/topics", get(get_topics))
        .route("/topics", post(create_topic))
        .route("/topics/:id", get(get_topic))
        .route("/topics/:id", put(update_topic))
        .route("/topics/:id", delete(delete_topic))
        .route("/topics/:id/posts", get(get_posts_by_topic))
        .route("/topics/:id/posts", post(create_post))
}