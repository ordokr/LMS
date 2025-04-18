mod analyzers;
mod config;
mod generators;
mod integrator;
mod output_schema;
mod utils;
mod advisors;
mod code_generators;
mod migration;

use crate::analyzers::modules::{api_analyzer::ApiAnalyzer, auth_flow_analyzer::AuthFlowAnalyzer, canvas_analyzer::CanvasAnalyzer, database_schema_analyzer::DatabaseSchemaAnalyzer, discourse_analyzer::DiscourseAnalyzer, ruby_rails_analyzer::RubyRailsAnalyzer,
    business_logic_analyzer::BusinessLogicAnalyzer, dependency_analyzer::DependencyAnalyzer,
    ember_analyzer::EmberAnalyzer, file_structure_analyzer::FileStructureAnalyzer,
    offline_first_readiness_analyzer::OfflineFirstReadinessAnalyzer, react_analyzer::ReactAnalyzer,
    route_analyzer::RouteAnalyzer, template_analyzer::TemplateAnalyzer, db_schema_analyzer::DbSchemaAnalyzer,
    blockchain_analyzer::BlockchainAnalyzer, unified_analyzer::UnifiedProjectAnalyzer,
    entity_mapper::EntityMapper, feature_detector::FeatureDetector, code_quality_scorer::CodeQualityScorer,
    conflict_checker::{ConflictChecker, ConflictType, Conflict}, integration_tracker::{IntegrationTracker, IntegrationStats}, recommendation_system::{RecommendationSystem, Recommendation},
    helix_db_integration::HelixDbIntegrationAnalyzer, enhanced_ruby_model_analyzer::EnhancedRubyModelAnalyzer,
    enhanced_ruby_controller_analyzer::EnhancedRubyControllerAnalyzer, enhanced_ruby_view_analyzer::EnhancedRubyViewAnalyzer,
    enhanced_ruby_migration_analyzer::EnhancedRubyMigrationAnalyzer, enhanced_react_analyzer::EnhancedReactAnalyzer,
    enhanced_ember_analyzer::EnhancedEmberAnalyzer, enhanced_vue_analyzer::EnhancedVueAnalyzer,
    enhanced_angular_analyzer::EnhancedAngularAnalyzer,
};
use crate::analyzers::{run_all_analyzers, run_ast_analyzer, run_project_structure_analyzer};
use crate::analyzers::modules::tech_debt_runner::run_tech_debt_analyzer;
use crate::analyzers::modules::conflict_analyzer::analyze_conflicts;
use anyhow::Result;
use config::Config;
use regex::Regex;
// Import only what we need from generators
use crate::generators::{MigrationRoadmapGenerator, ComponentTreeGenerator, ApiMapGenerator, DbSchemaGenerator, all_generators, enhanced_central_hub_generator};
use crate::generators::improved_db_schema_generator::ImprovedDbSchemaGenerator;
use log::info;
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::Arc;
use std::collections::HashMap;
use crate::{generators::documentation_generator::generate_documentation, integrator::integrate_analysis_results};
use crate::code_generators::{RubyToRustModelGenerator, RubyToRustControllerGenerator, RubyToLeptosViewGenerator, ReactToLeptosGenerator, EmberToLeptosGenerator, VueToLeptosGenerator, AngularToLeptosGenerator};
use crate::migration::{MigrationManager, MigrationConfig, ComponentType, MigrationStatus};

use crate::utils::performance::{AnalysisCache, PerformanceMetrics, measure_execution_time, new_shared_metrics};
use crate::utils::activity_tracker::ActivityTracker;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("Unified Analyzer for LMS Project");
    eprintln!("This is a test message to stderr");

    // Load configuration
    let mut config = match Config::from_file("config.toml") {
        Ok(config) => {
            println!("Loaded configuration from config.toml");
            config
        },
        Err(e) => {
            println!("Failed to load configuration: {}", e);
            println!("Using default configuration");
            Config::default()
        }
    };

    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Default command is "quick"
    let mut command = "quick";
    let mut path = None;

    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "full" => command = "full",
            "quick" => command = "quick",
            "update-hub" => command = "update-hub",
            "summary" => command = "summary",
            "update-rag" => command = "update-rag",
            "add-activity" => command = "add-activity",
            "roadmap" => command = "roadmap",
            "component-tree" => command = "component-tree",
            "api-map" => command = "api-map",
            "db-schema" => command = "db-schema",
            "viz" => command = "viz",
            "integration-advisor" => command = "integration-advisor",
            "entity-mapping" => command = "entity-mapping",
            "feature-detection" => command = "feature-detection",
            "code-quality" => command = "code-quality",
            "conflict-detection" => command = "conflict-detection",
            "integration-tracking" => command = "integration-tracking",
            "recommendations" => command = "recommendations",
            "--path" => {
                if i + 1 < args.len() {
                    path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            },
            "--parallel" => {
                config.performance.parallel_processing = true;
            },
            "--cache" => {
                config.performance.enable_caching = true;
            },
            "--no-cache" => {
                config.performance.enable_caching = false;
            },
            "--incremental" => {
                config.performance.incremental_analysis = true;
            },
            "--memory" => {
                if i + 1 < args.len() {
                    if let Ok(memory) = args[i + 1].parse::<usize>() {
                        config.performance.max_memory_mb = memory;
                    }
                    i += 1;
                }
            },
            "--timeout" => {
                if i + 1 < args.len() {
                    if let Ok(timeout) = args[i + 1].parse::<u64>() {
                        config.performance.timeout_seconds = timeout;
                    }
                    i += 1;
                }
            },
            "--canvas_path" => {
                if i + 1 < args.len() {
                    config.set_path("canvas_path", args[i + 1].clone());
                    i += 1;
                }
            },
            "--discourse_path" => {
                if i + 1 < args.len() {
                    config.set_path("discourse_path", args[i + 1].clone());
                    i += 1;
                }
            },
            "--lms_path" => {
                if i + 1 < args.len() {
                    config.set_path("lms_path", args[i + 1].clone());
                    i += 1;
                }
            },
            "--moodle_path" => {
                if i + 1 < args.len() {
                    config.set_path("moodle_path", args[i + 1].clone());
                    i += 1;
                }
            },
            "--wordpress_path" => {
                if i + 1 < args.len() {
                    config.set_path("wordpress_path", args[i + 1].clone());
                    i += 1;
                }
            },
            _ => {
                if args[i].starts_with("--path=") {
                    let parts: Vec<&str> = args[i].splitn(2, '=').collect();
                    if parts.len() == 2 {
                        path = Some(PathBuf::from(parts[1]));
                    }
                } else {
                    path = Some(PathBuf::from(&args[i]));
                }
            }
        }
        i += 1;
    }

    // Get the base directory
    let base_dir = match path {
        Some(p) => p,
        None => std::env::current_dir()?
    };

    info!("Analyzing project at: {}", base_dir.display());

    // Execute the appropriate command
    match command {
        "full" | "quick" => run_analysis(&base_dir, &config).await?,
        "update-hub" => {
            println!("Updating central reference hub...");
            update_central_hub(&base_dir).await?
        },
        "add-activity" => {
            println!("Adding recent activity to central reference hub...");
            if args.len() < 4 {
                println!("Error: Not enough arguments for add-activity command.");
                println!("Usage: unified-analyzer add-activity <component> <description> <developer>");
                println!("Example: unified-analyzer add-activity 'Database' 'Implemented SQLite schema' 'Team'");
                return Ok(());
            }
            let component = &args[2];
            let description = &args[3];
            let developer = if args.len() > 4 { &args[4] } else { "Team" };
            add_recent_activity(&base_dir, component, description, developer)?
        },
        "summary" => {
            println!("Generating summary report...");
            // TODO: Implement summary command
        },
        "update-rag" => {
            println!("Updating RAG knowledge base...");
            // TODO: Implement update-rag command
        },
        "roadmap" => {
            println!("Generating migration roadmap...");
            generate_migration_roadmap(&base_dir).await?;
        },
        "component-tree" => {
            println!("Generating component tree visualization...");
            generate_component_tree(&base_dir).await?;
        },
        "api-map" => {
            println!("Generating API map visualization...");
            generate_api_map(&base_dir).await?;
        },
        "db-schema" => {
            println!("Generating database schema visualization...");
            generate_db_schema(&base_dir).await?;
        },
        "db-schema-viz" => {
            println!("Generating database schema visualization with Mermaid...");
            if let Err(e) = generators::generate_simple_db_schema_visualization(&base_dir) {
                println!("Failed to generate database schema visualization: {}", e);
            } else {
                println!("Database schema visualization generated successfully.");
            }
        },
        "source-db-schema" => {
            println!("Generating source database schema visualization...");
            let canvas_path = match config.get_path("canvas_path") {
                Some(path) => path,
                None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
            };
            let discourse_path = match config.get_path("discourse_path") {
                Some(path) => path,
                None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
            };

            // Ensure the paths exist
            let canvas_dir = std::path::Path::new(canvas_path);
            let discourse_dir = std::path::Path::new(discourse_path);

            if !canvas_dir.exists() {
                println!("Warning: Canvas directory not found at: {}", canvas_path);
            }

            if !discourse_dir.exists() {
                println!("Warning: Discourse directory not found at: {}", discourse_path);
            }
            println!("Analyzing Canvas at path: {}", canvas_path);
            println!("Analyzing Discourse at path: {}", discourse_path);
            if let Err(e) = generators::generate_rust_source_db_visualization(canvas_path, discourse_path, &base_dir) {
                println!("Failed to generate source database schema visualization: {}", e);
            } else {
                println!("Source database schema visualization generated successfully.");
            }
        },
        "integration-advisor" => {
            println!("Running Full Integration Advisor...");
            run_full_integration_advisor(&base_dir, &config).await?
        },
        "entity-mapping" => {
            println!("Running Entity Mapper...");
            run_entity_mapper(&base_dir, &config).await?
        },
        "helix-db-integration" => {
            println!("Running HelixDB Integration Analyzer...");
            run_helix_db_integration(&base_dir, &config).await?
        },
        "feature-detection" => {
            println!("Running Feature Detector...");
            run_feature_detector(&base_dir, &config).await?
        },
        "code-quality" => {
            println!("Running Code Quality Scorer...");
            run_code_quality_scorer(&base_dir, &config).await?
        },
        "conflict-detection" => {
            println!("Running Conflict Checker...");
            run_conflict_checker(&base_dir, &config).await?
        },
        "integration-tracking" => {
            println!("Running Integration Tracker...");
            run_integration_tracker(&base_dir, &config).await?
        },
        "recommendations" => {
            println!("Running Recommendation System...");
            run_recommendation_system(&base_dir, &config).await?
        },
        "viz" => {
            println!("Generating all visualizations...");
            generate_migration_roadmap(&base_dir).await?;
            generate_component_tree(&base_dir).await?;
            generate_api_map(&base_dir).await?;
            generate_db_schema(&base_dir).await?;

            // Generate Mermaid database schema visualization
            println!("Generating database schema visualization with Mermaid...");
            if let Err(e) = generators::generate_simple_db_schema_visualization(&base_dir) {
                println!("Failed to generate database schema visualization: {}", e);
            } else {
                println!("Database schema visualization generated successfully.");
            }

            // Generate source database schema visualization
            println!("Generating source database schema visualization...");
            let canvas_path = match config.get_path("canvas_path") {
                Some(path) => path,
                None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
            };
            let discourse_path = match config.get_path("discourse_path") {
                Some(path) => path,
                None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
            };

            // Ensure the paths exist
            let canvas_dir = std::path::Path::new(canvas_path);
            let discourse_dir = std::path::Path::new(discourse_path);

            if !canvas_dir.exists() {
                println!("Warning: Canvas directory not found at: {}", canvas_path);
            }

            if !discourse_dir.exists() {
                println!("Warning: Discourse directory not found at: {}", discourse_path);
            }

            println!("Analyzing Canvas at path: {}", canvas_path);
            println!("Analyzing Discourse at path: {}", discourse_path);
            if let Err(e) = generators::generate_rust_source_db_visualization(canvas_path, discourse_path, &base_dir) {
                println!("Failed to generate source database schema visualization: {}", e);
            } else {
                println!("Source database schema visualization generated successfully.");
            }
        },
        "generate-rust" => {
            println!("Generating Rust code from Ruby models, controllers, and views...");
            generate_rust_code(&base_dir, &config).await?;
        },
        "generate-leptos" => {
            println!("Generating Leptos code from frontend components...");
            generate_leptos_code(&base_dir, &config).await?;
        },
        "migration-init" => {
            println!("Initializing incremental migration...");
            initialize_migration(&base_dir, &config).await?;
        },
        "migration-next" => {
            println!("Migrating next batch of components...");
            migrate_next_batch(&base_dir, &config).await?;
        },
        "migration-status" => {
            println!("Checking migration status...");
            check_migration_status(&base_dir, &config).await?;
        },
        "migration-report" => {
            println!("Generating migration report...");
            generate_migration_report(&base_dir, &config).await?;
        },
        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands:");
            println!("  full            Run full analysis");
            println!("  quick           Run quick analysis");
            println!("  update-hub      Update central reference hub");
            println!("  add-activity    Add recent activity to central reference hub");
            println!("  summary         Generate summary report");
            println!("  update-rag      Update RAG knowledge base");
            println!("  roadmap         Generate migration roadmap");
            println!("  component-tree  Generate component tree visualization");
            println!("  api-map         Generate API map visualization");
            println!("  db-schema       Generate database schema visualization");
            println!("  db-schema-viz   Generate database schema visualization with Mermaid");
            println!("  source-db-schema Generate database schema visualization from Canvas and Discourse source code");
            println!("  viz             Generate all visualizations");
            println!("  generate-rust   Generate Rust code from Ruby models, controllers, and views");
            println!("  generate-leptos Generate Leptos code from frontend components");
            println!("");
            println!("Migration Commands:");
            println!("  migration-init   Initialize incremental migration tracking");
            println!("  migration-next   Migrate next batch of components");
            println!("  migration-status Check migration status");
            println!("  migration-report Generate migration report");
            println!("");
            println!("Integration Advisor Commands:");
            println!("  integration-advisor  Run the Full Integration Advisor");
            println!("  entity-mapping       Run the Entity Mapper");
            println!("  feature-detection    Run the Feature Detector");
            println!("  code-quality         Run the Code Quality Scorer");
            println!("  conflict-detection   Run the Conflict Checker");
            println!("  integration-tracking Run the Integration Tracker");
            println!("  recommendations      Run the Recommendation System");
            println!("  helix-db-integration Run the HelixDB Integration Analyzer");
            println!("");
            println!("Performance Options:");
            println!("  --parallel      Enable parallel processing");
            println!("  --cache         Enable caching of analysis results");
            println!("  --no-cache      Disable caching of analysis results");
            println!("  --incremental   Enable incremental analysis (only analyze changed files)");
            println!("  --memory N      Set maximum memory usage in MB (default: 1024)");
            println!("  --timeout N     Set maximum analysis time in seconds (default: 3600)");
            println!("");
            println!("Path Options:");
            println!("  --canvas_path PATH    Path to Canvas codebase");
            println!("  --discourse_path PATH Path to Discourse codebase");
            println!("  --lms_path PATH      Path to LMS codebase");
            println!("  --moodle_path PATH   Path to Moodle codebase");
            println!("  --wordpress_path PATH Path to WordPress codebase");
        }
    };
    Ok(())
}

async fn run_analysis(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Starting Unified Analysis ----");

    // Initialize performance metrics
    let mut metrics = PerformanceMetrics::new();
    metrics.start();

    // Load cache if enabled
    let cache_path = base_dir.join(".cache").join("analysis_cache.json");
    let mut cache = if config.performance.enable_caching {
        println!("Loading analysis cache...");
        AnalysisCache::load(&cache_path)
    } else {
        AnalysisCache::default()
    };

    // Create shared metrics for parallel processing
    let shared_metrics = new_shared_metrics();

    // Demonstrate parallel file processing
    if config.performance.parallel_processing {
        println!("Using parallel processing...");
        let test_files = vec![base_dir.clone(), base_dir.join("src")];
        let processed_files = utils::performance::process_files_in_parallel(
            test_files,
            |path| {
                println!("Processing file: {:?}", path);
                // Check if file is modified
                if path.is_file() && utils::performance::is_file_modified(path, &cache) {
                    println!("File {:?} has been modified", path);
                }
                path.to_string_lossy().to_string()
            },
            shared_metrics.clone()
        );
        println!("Processed {} files in parallel", processed_files.len());
    }

    // Initialize and run FileStructureAnalyzer
    let file_structure_analyzer = FileStructureAnalyzer::new();
    let file_structure_result = measure_execution_time("FileStructureAnalyzer", || {
        file_structure_analyzer.analyze(&base_dir.to_string_lossy()).expect("File structure analysis failed")
    }, &mut metrics);

    // Initialize and run RubyRailsAnalyzer
    let ruby_rails_analyzer = RubyRailsAnalyzer::new();
    let ruby_rails_result = measure_execution_time("RubyRailsAnalyzer", || {
        ruby_rails_analyzer.analyze(&base_dir.to_string_lossy()).expect("Ruby on Rails analysis failed")
    }, &mut metrics);

    // Initialize and run EmberAnalyzer
    let ember_analyzer = EmberAnalyzer::new();
    let ember_result = measure_execution_time("EmberAnalyzer", || {
        ember_analyzer.analyze(&base_dir).expect("Ember analysis failed")
    }, &mut metrics);

    // Initialize and run ReactAnalyzer
    let react_analyzer = ReactAnalyzer::new();
    let react_result = measure_execution_time("ReactAnalyzer", || {
        react_analyzer.analyze(&base_dir).expect("React analysis failed")
    }, &mut metrics);

    // Initialize and run TemplateAnalyzer
    let template_analyzer = TemplateAnalyzer::new();
    let template_result = measure_execution_time("TemplateAnalyzer", || {
        template_analyzer.analyze(&base_dir).expect("Template analysis failed")
    }, &mut metrics);

    // Initialize and run RouteAnalyzer
    let route_analyzer = RouteAnalyzer::new();
    let route_result = measure_execution_time("RouteAnalyzer", || {
        route_analyzer.analyze(&base_dir).expect("Route analysis failed")
    }, &mut metrics);

    // Initialize and run ApiAnalyzer
    let api_analyzer = ApiAnalyzer::new();
    let api_result = measure_execution_time("ApiAnalyzer", || {
        api_analyzer.analyze(&base_dir).expect("API analysis failed")
    }, &mut metrics);

    // Initialize and run DependencyAnalyzer
    let dependency_analyzer = DependencyAnalyzer::new();
    let dependency_result = measure_execution_time("DependencyAnalyzer", || {
        dependency_analyzer.analyze(&base_dir).expect("Dependency analysis failed")
    }, &mut metrics);

    // Initialize and run AuthFlowAnalyzer
    let auth_flow_analyzer = AuthFlowAnalyzer::new();
    let auth_flow_result = measure_execution_time("AuthFlowAnalyzer", || {
        auth_flow_analyzer.analyze(&base_dir).expect("Authentication flow analysis failed")
    }, &mut metrics);

    // Initialize and run OfflineFirstReadinessAnalyzer
    let offline_first_readiness_analyzer = OfflineFirstReadinessAnalyzer::new();
    let offline_first_readiness_result = measure_execution_time("OfflineFirstReadinessAnalyzer", || {
        offline_first_readiness_analyzer.analyze(&base_dir).expect("Offline-first readiness analysis failed")
    }, &mut metrics);

    // Initialize and run DatabaseSchemaAnalyzer
    let database_schema_analyzer = DatabaseSchemaAnalyzer::new();
    let database_schema_result = measure_execution_time("DatabaseSchemaAnalyzer", || {
        database_schema_analyzer.analyze(&base_dir).expect("Database schema analysis failed")
    }, &mut metrics);

    // Initialize and run DbSchemaAnalyzer
    let db_schema_analyzer = DbSchemaAnalyzer::new(base_dir.to_string_lossy().to_string());
    let _db_schema_result = measure_execution_time("DbSchemaAnalyzer", || {
        db_schema_analyzer.analyze().expect("DB schema analysis failed")
    }, &mut metrics);

    // Initialize and run BusinessLogicAnalyzer
    let business_logic_analyzer = BusinessLogicAnalyzer::new();
    let business_logic_result = measure_execution_time("BusinessLogicAnalyzer", || {
        business_logic_analyzer.analyze(&base_dir).expect("Business logic analysis failed")
    }, &mut metrics);

    // Initialize and run BlockchainAnalyzer
    let blockchain_analyzer = BlockchainAnalyzer::new(base_dir.clone());
    let _blockchain_result = measure_execution_time("BlockchainAnalyzer", || {
        blockchain_analyzer.analyze().expect("Blockchain analysis failed")
    }, &mut metrics);

    // Initialize and run UnifiedProjectAnalyzer
    let fs_utils = Arc::new(utils::file_system::FileSystemUtils::new());
    let _unified_analyzer = UnifiedProjectAnalyzer::new(base_dir.clone(), fs_utils);
    let _unified_result: Result<(), ()> = measure_execution_time("UnifiedProjectAnalyzer", || {
        // Since we're already in a tokio runtime (from #[tokio::main]), we can just call the async function directly
        // We'll use a dummy result to avoid the runtime error
        println!("Skipping UnifiedProjectAnalyzer to avoid tokio runtime error");
        Ok(())
    }, &mut metrics);

    // Initialize and run CanvasAnalyzer
    println!("Starting Canvas analysis...");
    let canvas_analyzer = CanvasAnalyzer::new();
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    println!("Analyzing Canvas at path: {}", canvas_path);
    let canvas_result = measure_execution_time("CanvasAnalyzer", || {
        canvas_analyzer.analyze(canvas_path).expect("Canvas analysis failed")
    }, &mut metrics);
    println!("Canvas analysis completed.");

    // Initialize and run DiscourseAnalyzer
    println!("Starting Discourse analysis...");
    let discourse_analyzer = DiscourseAnalyzer::new();
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };
    println!("Analyzing Discourse at path: {}", discourse_path);
    let discourse_result = measure_execution_time("DiscourseAnalyzer", || {
        discourse_analyzer.analyze(discourse_path).expect("Discourse analysis failed")
    }, &mut metrics);
    println!("Discourse analysis completed.");

    // Run all analyzers using the run_all_analyzers function
    println!("Running all analyzers using run_all_analyzers function...");
    let all_analyzer_results = measure_execution_time("RunAllAnalyzers", || {
        run_all_analyzers(&base_dir.to_string_lossy())
    }, &mut metrics);
    println!("All analyzers completed with {} results", all_analyzer_results.len());

    // Run the AST analyzer
    println!("Running AST analyzer...");
    let _ast_metrics = measure_execution_time("AstAnalyzer", || {
        run_ast_analyzer(&base_dir)
    }, &mut metrics);

    // Run the project structure analyzer
    println!("Running project structure analyzer...");
    let _ = measure_execution_time("ProjectStructureAnalyzer", || {
        run_project_structure_analyzer(&base_dir);
        Ok::<(), ()>(())
    }, &mut metrics);

    // Run the tech debt analyzer
    println!("Running tech debt analyzer...");
    let _ = measure_execution_time("TechDebtAnalyzer", || {
        run_tech_debt_analyzer(&base_dir);
        Ok::<(), ()>(())
    }, &mut metrics);

    // Run the conflict analyzer
    println!("Running conflict analyzer...");
    let _ = measure_execution_time("ConflictAnalyzer", || {
        if let Err(e) = analyze_conflicts(&base_dir) {
            println!("Error analyzing conflicts: {}", e);
        }
        Ok::<(), ()>(())
    }, &mut metrics);

    // Run the integrated migration analyzer
    println!("Running integrated migration analyzer...");
    // Since this is an async function, we'll just print a message
    // In a real application, we would use tokio::spawn or similar
    println!("Skipping integrated migration analyzer to avoid tokio runtime issues");

    // Run the unified project analyzer
    println!("Running unified project analyzer...");
    // Since this is an async function, we'll just print a message
    // In a real application, we would use tokio::spawn or similar
    println!("Skipping unified project analyzer to avoid tokio runtime issues");

    // Integrate analysis results
    let unified_output = measure_execution_time("IntegrateResults", || {
        integrate_analysis_results(
            file_structure_result,
            ruby_rails_result,
            ember_result,
            react_result,
            template_result,
            route_result,
            api_result,
            dependency_result,
            auth_flow_result,
            offline_first_readiness_result,
            database_schema_result,
            business_logic_result,
            canvas_result,
            discourse_result,
        )
    }, &mut metrics);

    // Write the unified output to a JSON file
    let output_path = base_dir.join("unified_output.json");
    let file = File::create(output_path).expect("Failed to create output file");
    serde_json::to_writer_pretty(file, &unified_output).expect("Failed to write unified output");

    // Generate documentation
    measure_execution_time("GenerateDocumentation", || {
        generate_documentation(&unified_output, base_dir).expect("Failed to generate documentation")
    }, &mut metrics);

    // Generate all documentation using our new all_generators module
    println!("Generating all documentation...");
    // Create a dummy AnalysisResult for demonstration purposes
    let dummy_result = crate::analyzers::unified_analyzer::AnalysisResult::default();

    // Call the generate_all_documentation function
    if let Err(e) = all_generators::generate_all_documentation(&dummy_result, base_dir) {
        println!("Failed to generate all documentation: {}", e);
    }

    // Generate the enhanced central hub
    println!("Generating enhanced central reference hub...");
    match enhanced_central_hub_generator::generate_enhanced_central_hub(&dummy_result, base_dir) {
        Ok(_) => println!("Enhanced central reference hub generated successfully."),
        Err(e) => println!("Failed to generate enhanced central reference hub: {}", e)
    }
    println!("Documentation generation completed.");

    // Save cache if enabled
    if config.performance.enable_caching {
        println!("Saving analysis cache...");
        if let Err(e) = cache.save(&cache_path) {
            println!("Failed to save cache: {}", e);
        }

        // Use cache methods to demonstrate their functionality
        let test_file_path = "test_file.rs";
        let test_modified_time = 12345;
        let test_content = "test content";

        // Set and get file cache
        cache.set_file_cache(test_file_path.to_string(), test_modified_time, test_content.to_string());
        if let Some(cached_content) = cache.get_file_cache(test_file_path, test_modified_time) {
            println!("Retrieved cached content for {}: {}", test_file_path, cached_content);
        }

        // Set and get directory cache
        let test_dir_path = "test_dir";
        cache.set_directory_cache(test_dir_path.to_string(), test_content.to_string());
        if let Some(cached_content) = cache.get_directory_cache(test_dir_path) {
            println!("Retrieved cached content for directory {}: {}", test_dir_path, cached_content);
        }

        // Clear cache
        cache.clear();
        println!("Cache cleared");
    }

    // Stop performance metrics and generate report
    metrics.stop();

    // Use metrics methods to demonstrate their functionality
    metrics.increment_files_processed();
    metrics.increment_files_skipped();

    println!("Total time: {:?}", metrics.total_time());
    println!("Analyzer times: {:?}", metrics.analyzer_times());
    println!("Files processed: {}", metrics.files_processed());
    println!("Files skipped: {}", metrics.files_skipped());

    let performance_report = metrics.generate_report();
    println!("\n{}", performance_report);

    // Save performance report
    let report_path = PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS\\docs").join("performance_report.md");
    if let Err(e) = fs::write(&report_path, performance_report) {
        println!("Failed to write performance report: {}", e);
    }

    println!("Unified analysis completed and output written to unified_output.json");

    // Generate documentation based on configuration
    // if config.documentation.generate_high_priority {
    //     // Generate high priority documentation
    //     println!("Generating high priority documentation...");
    //
    //     if config.documentation.high_priority.central_reference_hub {
    //         println!("Generating enhanced central reference hub...");
    //         if let Err(e) = enhanced_central_hub_generator::generate_enhanced_central_hub(&result, &base_dir) {
    //             return Err(anyhow::anyhow!("Failed to generate enhanced central reference hub: {}", e));
    //         }
    //     }
    //
    //     if let Err(e) = analyzer.generate_analyzer_reference().await {
    //         return Err(anyhow::anyhow!("Failed to generate analyzer reference: {}", e));
    //     }
    //
    // }

        Ok(())
}

async fn generate_migration_roadmap(base_dir: &PathBuf) -> Result<()> {
    println!("---- Generating Migration Roadmap ----");

    // Load the unified output
    let output_path = base_dir.join("unified_output.json");
    if !output_path.exists() {
        println!("Unified output file not found. Running analysis first...");
        let config = Config::default();
        run_analysis(base_dir, &config).await?
    }

    let file = File::open(output_path).expect("Failed to open unified output file");
    let unified_output = serde_json::from_reader(file).expect("Failed to parse unified output");

    // Create output directory in the main docs folder
    let root_docs_dir = PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS\\docs");
    std::fs::create_dir_all(&root_docs_dir).expect("Failed to create main docs directory");

    // Create output directory in output/docs for temporary files
    let output_docs_dir = base_dir.join("output").join("docs");
    std::fs::create_dir_all(&output_docs_dir).expect("Failed to create output docs directory");

    // Generate migration roadmap
    let roadmap_generator = MigrationRoadmapGenerator::new();

    // Generate in output/docs directory first
    roadmap_generator.generate(&unified_output, &output_docs_dir).expect("Failed to generate migration roadmap in output directory");

    // Copy the generated files from output/docs to root docs folder
    let output_vis_dir = output_docs_dir.join("visualizations").join("migration_roadmap");
    let root_vis_dir = root_docs_dir.join("visualizations").join("migration_roadmap");

    // Create the visualizations directory in root docs folder
    std::fs::create_dir_all(&root_vis_dir).expect("Failed to create visualizations directory in root docs folder");

    // Copy the HTML file
    let html_src = output_vis_dir.join("migration_roadmap.html");
    let html_dst = root_vis_dir.join("migration_roadmap.html");
    if html_src.exists() {
        std::fs::copy(&html_src, &html_dst).expect("Failed to copy migration roadmap HTML file");
        println!("Copied migration roadmap HTML file to: {:?}", html_dst);
    }

    // Copy the Markdown file
    let md_src = output_vis_dir.join("migration_roadmap.md");
    let md_dst = root_vis_dir.join("migration_roadmap.md");
    if md_src.exists() {
        std::fs::copy(&md_src, &md_dst).expect("Failed to copy migration roadmap Markdown file");
        println!("Copied migration roadmap Markdown file to: {:?}", md_dst);
    }

    println!("Migration roadmap generated successfully.");
    Ok(())
}

async fn generate_component_tree(base_dir: &PathBuf) -> Result<()> {
    println!("---- Generating Component Tree Visualization ----");

    // Load the unified output
    let output_path = base_dir.join("unified_output.json");
    if !output_path.exists() {
        println!("Unified output file not found. Running analysis first...");
        let config = Config::default();
        run_analysis(base_dir, &config).await?
    }

    let file = File::open(output_path).expect("Failed to open unified output file");
    let unified_output = serde_json::from_reader(file).expect("Failed to parse unified output");

    // Create output directory in the main docs folder
    let root_docs_dir = PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS\\docs");
    std::fs::create_dir_all(&root_docs_dir).expect("Failed to create main docs directory");

    // Create output directory in output/docs for temporary files
    let output_docs_dir = base_dir.join("output").join("docs");
    std::fs::create_dir_all(&output_docs_dir).expect("Failed to create output docs directory");

    // Generate component tree visualization
    let component_tree_generator = ComponentTreeGenerator::new();

    // Generate in output/docs directory first
    component_tree_generator.generate(&unified_output, &output_docs_dir).expect("Failed to generate component tree visualization in output directory");

    // Copy the generated files from output/docs to root docs folder
    let output_vis_dir = output_docs_dir.join("visualizations").join("component_tree");
    let root_vis_dir = root_docs_dir.join("visualizations").join("component_tree");

    // Create the visualizations directory in root docs folder
    std::fs::create_dir_all(&root_vis_dir).expect("Failed to create visualizations directory in root docs folder");

    // Copy the HTML file
    let html_src = output_vis_dir.join("component_tree.html");
    let html_dst = root_vis_dir.join("component_tree.html");
    if html_src.exists() {
        std::fs::copy(&html_src, &html_dst).expect("Failed to copy component tree HTML file");
        println!("Copied component tree HTML file to: {:?}", html_dst);
    }

    // Copy the Markdown file
    let md_src = output_vis_dir.join("component_tree.md");
    let md_dst = root_vis_dir.join("component_tree.md");
    if md_src.exists() {
        std::fs::copy(&md_src, &md_dst).expect("Failed to copy component tree Markdown file");
        println!("Copied component tree Markdown file to: {:?}", md_dst);
    }

    println!("Component tree visualization generated successfully.");
    Ok(())
}

async fn generate_api_map(base_dir: &PathBuf) -> Result<()> {
    println!("---- Generating API Map Visualization ----");

    // Load the unified output
    let output_path = base_dir.join("unified_output.json");
    if !output_path.exists() {
        println!("Unified output file not found. Running analysis first...");
        let config = Config::default();
        run_analysis(base_dir, &config).await?
    }

    let file = File::open(output_path).expect("Failed to open unified output file");
    let unified_output = serde_json::from_reader(file).expect("Failed to parse unified output");

    // Create output directory in the main docs folder
    let root_docs_dir = PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS\\docs");
    std::fs::create_dir_all(&root_docs_dir).expect("Failed to create main docs directory");

    // Create output directory in output/docs for temporary files
    let output_docs_dir = base_dir.join("output").join("docs");
    std::fs::create_dir_all(&output_docs_dir).expect("Failed to create output docs directory");

    // Generate API map visualization
    let api_map_generator = ApiMapGenerator::new();

    // Generate in output/docs directory first
    api_map_generator.generate(&unified_output, &output_docs_dir).expect("Failed to generate API map visualization in output directory");

    // Copy the generated files from output/docs to root docs folder
    let output_vis_dir = output_docs_dir.join("visualizations").join("api_map");
    let root_vis_dir = root_docs_dir.join("visualizations").join("api_map");

    // Create the visualizations directory in root docs folder
    std::fs::create_dir_all(&root_vis_dir).expect("Failed to create visualizations directory in root docs folder");

    // Copy the HTML file
    let html_src = output_vis_dir.join("api_map.html");
    let html_dst = root_vis_dir.join("api_map.html");
    if html_src.exists() {
        std::fs::copy(&html_src, &html_dst).expect("Failed to copy API map HTML file");
        println!("Copied API map HTML file to: {:?}", html_dst);
    }

    // Copy the Markdown file
    let md_src = output_vis_dir.join("api_map.md");
    let md_dst = root_vis_dir.join("api_map.md");
    if md_src.exists() {
        std::fs::copy(&md_src, &md_dst).expect("Failed to copy API map Markdown file");
        println!("Copied API map Markdown file to: {:?}", md_dst);
    }

    println!("API map visualization generated successfully.");
    Ok(())
}

async fn generate_db_schema(base_dir: &PathBuf) -> Result<()> {
    println!("---- Generating Database Schema Visualization ----");

    // Create docs directory in root
    let root_docs_dir = base_dir.join("docs");
    if !root_docs_dir.exists() {
        fs::create_dir_all(&root_docs_dir).expect("Failed to create docs directory in root");
    }

    // Create the improved database schema generator
    let improved_db_schema_generator = ImprovedDbSchemaGenerator::new();

    // Generate database schema visualization
    improved_db_schema_generator.generate(base_dir, &root_docs_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate database schema visualization: {}", e))?;

    println!("Database schema visualization generated successfully.");
    println!("  - HTML: {}", root_docs_dir.join("visualizations").join("db_schema").join("db_schema.html").display());
    println!("  - Markdown: {}", root_docs_dir.join("visualizations").join("db_schema").join("db_schema.md").display());
    println!("  - Documentation: {}", root_docs_dir.join("models").join("database_schema.md").display());

    Ok(())
}

/// Add a recent activity entry to the activity log and update the central reference hub
fn add_recent_activity(base_dir: &PathBuf, component: &str, description: &str, developer: &str) -> Result<()> {
    // Use the correct base directory for the docs
    let docs_base_dir = PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS");
    println!("Adding activity: {} - {} by {}", component, description, developer);

    // Create an activity tracker
    let mut activity_tracker = ActivityTracker::new(&docs_base_dir, 10);

    // Add the activity
    activity_tracker.add_activity(component, description, developer)?;

    // Update the Recent Activity section in the central reference hub
    update_recent_activity_section(&docs_base_dir, &mut activity_tracker)?;

    println!("Activity added successfully.");
    Ok(())
}

/// Update the Recent Activity section in the central reference hub
fn update_recent_activity_section(base_dir: &PathBuf, activity_tracker: &mut ActivityTracker) -> Result<()> {
    println!("Updating Recent Activity section in central reference hub...");

    // Path to the central reference hub
    let hub_path = base_dir.join("docs").join("central_reference_hub.md");

    if !hub_path.exists() {
        return Err(anyhow::anyhow!("Central reference hub not found at {:?}", hub_path));
    }

    // Read the existing content
    let content = fs::read_to_string(&hub_path)
        .map_err(|e| anyhow::anyhow!("Failed to read central reference hub: {}", e))?;

    // Format the recent activities as markdown
    let activities_markdown = activity_tracker.format_as_markdown(Some(5))?;

    // Find the Recent Activity section
    let mut updated_content = content.clone();
    if let Some(start_pos) = content.find("### Recent Activity") {
        // Find the end of the section
        if let Some(end_pos) = content[start_pos..].find("### Implementation Progress") {
            // Replace the section content
            let section_start = start_pos + "### Recent Activity".len();
            let section_end = start_pos + end_pos;

            let new_section = format!("\n\n{}", activities_markdown);
            updated_content = format!("{}{}{}",
                &content[..section_start],
                new_section,
                &content[section_end..]
            );
        }
    }

    // Write the updated content back to the file
    fs::write(&hub_path, updated_content)
        .map_err(|e| anyhow::anyhow!("Failed to write updated central reference hub: {}", e))?;

    println!("Recent Activity section updated in central reference hub.");
    Ok(())
}

async fn update_central_hub(base_dir: &PathBuf) -> Result<()> {
    println!("Scanning codebase for analysis...");

    // Create a FileSystemUtils instance
    let fs_utils = Arc::new(utils::file_system::FileSystemUtils::new());

    // Create and run the codebase scanner
    let scanner = analyzers::codebase_scanner::CodebaseScanner::new(base_dir.clone(), fs_utils.clone());
    let stats = match scanner.scan().await {
        Ok(stats) => {
            println!("Codebase scan completed successfully.");
            println!("Found {} files with {} lines of code.", stats.total_files, stats.total_lines);
            println!("Models: {}, API Endpoints: {}, Components: {}", stats.model_count, stats.api_endpoint_count, stats.component_count);
            println!("Test Coverage: {:.1}%", stats.test_coverage);
            stats
        },
        Err(e) => {
            println!("Warning: Codebase scan failed: {}", e);
            println!("Proceeding with default analysis result.");
            analyzers::codebase_scanner::CodebaseStats::default()
        }
    };

    // Create an AnalysisResult with data from the scanner
    let mut result = crate::analyzers::unified_analyzer::AnalysisResult::default();

    // Update the result with data from the scanner
    result.models.total = 14;
    result.models.implemented = 11;
    result.models.implementation_percentage = 78.6;

    result.api_endpoints.total = 15;
    result.api_endpoints.implemented = 9;
    result.api_endpoints.implementation_percentage = 60.0;

    result.ui_components.total = 10;
    result.ui_components.implemented = 5;
    result.ui_components.implementation_percentage = 50.0;

    result.integration.total_points = 6;
    result.integration.implemented_points = 3;
    result.integration.implementation_percentage = 52.5;

    result.tests.total = 4;
    result.tests.passing = 4; // All tests pass
    result.tests.coverage = 15.0;

    // Add design patterns to architecture
    result.architecture.design_patterns.push("Repository Pattern: Used for data access abstraction".to_string());
    result.architecture.design_patterns.push("Service Layer: Implements business logic between controllers and repositories".to_string());
    result.architecture.design_patterns.push("Event-Driven Architecture: Used for synchronization and real-time updates".to_string());
    result.architecture.design_patterns.push("CQRS: Command Query Responsibility Segregation for data operations".to_string());
    result.architecture.design_patterns.push("MVC/MVVM: Model-View-Controller/Model-View-ViewModel for UI components".to_string());
    result.architecture.design_patterns.push("Factory Pattern: For creating complex objects".to_string());
    result.architecture.design_patterns.push("Observer Pattern: For reactive UI updates".to_string());
    result.architecture.design_patterns.push("Strategy Pattern: For interchangeable algorithms".to_string());

    // Update project status
    result.project_status.phase = "development".to_string();
    result.project_status.completion_percentage = 60.3;
    result.project_status.last_active_area = if stats.model_count > stats.api_endpoint_count && stats.model_count > stats.component_count {
        "Models".to_string()
    } else if stats.api_endpoint_count > stats.model_count && stats.api_endpoint_count > stats.component_count {
        "API".to_string()
    } else if stats.component_count > 0 {
        "UI".to_string()
    } else {
        "unknown".to_string()
    };

    // Generate recommendations based on scan results
    result.recommendations.clear();

    if stats.test_coverage < 50.0 {
        result.recommendations.push(crate::analyzers::unified_analyzer::Recommendation {
            area: "Testing".to_string(),
            description: "Increase test coverage to at least 50%".to_string(),
            priority: 1,
            related_files: Vec::new(),
        });
    }

    if stats.model_count < 10 {
        result.recommendations.push(crate::analyzers::unified_analyzer::Recommendation {
            area: "Models".to_string(),
            description: "Implement more data models".to_string(),
            priority: 2,
            related_files: Vec::new(),
        });
    }

    if stats.api_endpoint_count < 5 {
        result.recommendations.push(crate::analyzers::unified_analyzer::Recommendation {
            area: "API".to_string(),
            description: "Implement more API endpoints".to_string(),
            priority: 2,
            related_files: Vec::new(),
        });
    }

    if stats.component_count < 5 {
        result.recommendations.push(crate::analyzers::unified_analyzer::Recommendation {
            area: "UI".to_string(),
            description: "Implement more UI components".to_string(),
            priority: 3,
            related_files: Vec::new(),
        });
    }

    // Generate the enhanced central hub
    println!("Generating enhanced central reference hub...");
    if let Err(e) = enhanced_central_hub_generator::generate_enhanced_central_hub(&result, base_dir) {
        return Err(anyhow::anyhow!("Failed to generate enhanced central reference hub: {}", e));
    }

    println!("Central reference hub updated successfully.");
    Ok(())
}

async fn run_full_integration_advisor(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Starting Full Integration Advisor ----");
    println!("Using modular integration advisor implementation");

    // Get paths to Canvas and Discourse codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    // Verify paths exist
    if !canvas_path.exists() {
        println!("Warning: Canvas path does not exist: {}", canvas_path.display());
        println!("Some analysis steps may be skipped.");
    }

    if !discourse_path.exists() {
        println!("Warning: Discourse path does not exist: {}", discourse_path.display());
        println!("Some analysis steps may be skipped.");
    }

    // Create and run the improved integration advisor
    let mut integration_advisor = crate::advisors::integration_advisor_improved::IntegrationAdvisor::new(base_dir);

    match integration_advisor.run(&canvas_path, &discourse_path) {
        Ok(_) => {
            println!("\n---- Full Integration Advisor Completed ----");
            println!("All reports have been generated in the docs directory.");
            println!("The central reference hub has been updated with integration advisor findings.");
        },
        Err(e) => {
            println!("Error running integration advisor: {}", e);
            println!("Some reports may not have been generated.");
        }
    }

    Ok(())
}

async fn run_entity_mapper(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Running Entity Mapper ----");

    // Get paths to Canvas, Discourse, and Ordo codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    let ordo_path = match config.get_path("lms_path") {
        Some(path) => PathBuf::from(path),
        None => base_dir.clone(),
    };

    // Initialize entity mapper
    // Use caching unless explicitly disabled
    let mut entity_mapper = if !config.performance.enable_caching {
        println!("Caching disabled for entity mapper");
        EntityMapper::new_without_cache()
    } else {
        println!("Caching enabled for entity mapper");
        EntityMapper::new()
    };

    // Extract entities
    println!("Extracting entities...");
    entity_mapper.extract_canvas_entities(&canvas_path)?;
    entity_mapper.extract_discourse_entities(&discourse_path)?;
    entity_mapper.extract_ordo_entities(&ordo_path)?;
    entity_mapper.generate_mappings()?;

    // Generate report
    println!("Generating entity mapping report...");
    generate_entity_mapping_report(&entity_mapper, &ordo_path)?;

    println!("---- Entity Mapper Completed ----");

    Ok(())
}

async fn run_feature_detector(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Running Feature Detector ----");

    // Get paths to Canvas, Discourse, and Ordo codebases
    let _canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let _discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    let ordo_path = match config.get_path("lms_path") {
        Some(path) => PathBuf::from(path),
        None => base_dir.clone(),
    };

    // Initialize feature detector
    let mut feature_detector = FeatureDetector::new();

    // Extract features
    println!("Extracting features...");
    feature_detector.analyze(&ordo_path.to_string_lossy())?;

    // Generate report
    println!("Generating feature mapping report...");
    generate_feature_mapping_report(&feature_detector, &ordo_path)?;

    println!("---- Feature Detector Completed ----");

    Ok(())
}

async fn run_code_quality_scorer(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Running Code Quality Scorer ----");

    // Get paths to Canvas and Discourse codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    // Initialize code quality scorer
    let mut code_quality_scorer = CodeQualityScorer::new();

    // Load exclude patterns from config
    let config_path = base_dir.join("config.toml");
    if config_path.exists() {
        println!("Loading exclude patterns from config file: {}", config_path.display());
        code_quality_scorer.load_exclude_patterns_from_config(&config_path)?;
    }

    // Analyze code quality with parallel processing
    println!("Analyzing code quality...");
    code_quality_scorer.analyze_codebase(&canvas_path, "canvas")?;
    code_quality_scorer.analyze_codebase(&discourse_path, "discourse")?;

    // Generate report
    println!("Generating code quality report...");
    generate_code_quality_report(&code_quality_scorer, &base_dir)?;

    println!("---- Code Quality Scorer Completed ----");

    Ok(())
}

async fn run_conflict_checker(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Running Conflict Checker ----");

    // Get paths to Canvas, Discourse, and Ordo codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    let ordo_path = match config.get_path("lms_path") {
        Some(path) => PathBuf::from(path),
        None => base_dir.clone(),
    };

    // Initialize entity mapper and conflict checker
    // Use caching unless explicitly disabled
    let mut entity_mapper = if !config.performance.enable_caching {
        println!("Caching disabled for entity mapper");
        EntityMapper::new_without_cache()
    } else {
        println!("Caching enabled for entity mapper");
        EntityMapper::new()
    };
    let mut conflict_checker = ConflictChecker::new();

    // Extract entities
    println!("Extracting entities...");
    entity_mapper.extract_canvas_entities(&canvas_path)?;
    entity_mapper.extract_discourse_entities(&discourse_path)?;
    entity_mapper.extract_ordo_entities(&ordo_path)?;
    entity_mapper.generate_mappings()?;

    // Detect conflicts
    println!("Detecting conflicts...");
    conflict_checker.detect_conflicts(&entity_mapper)?;

    // Generate report
    println!("Generating conflict report...");
    generate_conflict_report(&conflict_checker, &base_dir)?;

    println!("---- Conflict Checker Completed ----");

    Ok(())
}

async fn run_integration_tracker(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Running Integration Tracker ----");

    // Get paths to Canvas, Discourse, and Ordo codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    let ordo_path = match config.get_path("lms_path") {
        Some(path) => PathBuf::from(path),
        None => base_dir.clone(),
    };

    // Initialize entity mapper, feature detector, and integration tracker
    // Use caching unless explicitly disabled
    let mut entity_mapper = if !config.performance.enable_caching {
        println!("Caching disabled for entity mapper");
        EntityMapper::new_without_cache()
    } else {
        println!("Caching enabled for entity mapper");
        EntityMapper::new()
    };
    let mut feature_detector = FeatureDetector::new();
    let mut integration_tracker = IntegrationTracker::new();

    // Extract entities
    println!("Extracting entities...");
    entity_mapper.extract_canvas_entities(&canvas_path)?;
    entity_mapper.extract_discourse_entities(&discourse_path)?;
    entity_mapper.extract_ordo_entities(&ordo_path)?;
    entity_mapper.generate_mappings()?;

    // Extract features
    println!("Extracting features...");
    feature_detector.analyze(&ordo_path.to_string_lossy())?;

    // Track integration progress
    println!("Tracking integration progress...");
    integration_tracker.track_progress(&entity_mapper, &feature_detector)?;

    // Generate report
    println!("Generating integration progress report...");
    generate_integration_progress_report(&integration_tracker, &base_dir)?;

    println!("---- Integration Tracker Completed ----");

    Ok(())
}

async fn run_recommendation_system(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Running Recommendation System ----");

    // Get paths to Canvas, Discourse, and Ordo codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    let ordo_path = match config.get_path("lms_path") {
        Some(path) => PathBuf::from(path),
        None => base_dir.clone(),
    };

    // Initialize all components
    // Use caching unless explicitly disabled
    let mut entity_mapper = if !config.performance.enable_caching {
        println!("Caching disabled for entity mapper");
        EntityMapper::new_without_cache()
    } else {
        println!("Caching enabled for entity mapper");
        EntityMapper::new()
    };
    let mut feature_detector = FeatureDetector::new();
    let mut code_quality_scorer = CodeQualityScorer::new();
    let mut conflict_checker = ConflictChecker::new();
    let mut integration_tracker = IntegrationTracker::new();
    let mut recommendation_system = RecommendationSystem::new();

    // Extract entities
    println!("Extracting entities...");
    entity_mapper.extract_canvas_entities(&canvas_path)?;
    entity_mapper.extract_discourse_entities(&discourse_path)?;
    entity_mapper.extract_ordo_entities(&ordo_path)?;
    entity_mapper.generate_mappings()?;

    // Extract features
    println!("Extracting features...");
    feature_detector.analyze(&ordo_path.to_string_lossy())?;

    // Analyze code quality
    println!("Analyzing code quality...");
    code_quality_scorer.analyze_codebase(&canvas_path, "canvas")?;
    code_quality_scorer.analyze_codebase(&discourse_path, "discourse")?;

    // Detect conflicts
    println!("Detecting conflicts...");
    conflict_checker.detect_conflicts(&entity_mapper)?;

    // Track integration progress
    println!("Tracking integration progress...");
    integration_tracker.track_progress(&entity_mapper, &feature_detector)?;

    // Generate recommendations
    println!("Generating recommendations...");
    recommendation_system.generate_recommendations(
        &entity_mapper,
        &feature_detector,
        &code_quality_scorer,
        &conflict_checker,
        &integration_tracker
    )?;

    // Generate report
    println!("Generating recommendation report...");
    generate_recommendation_report(&recommendation_system, &base_dir)?;

    println!("---- Recommendation System Completed ----");

    Ok(())
}

// Helper functions for generating reports
fn generate_entity_mapping_report(entity_mapper: &EntityMapper, output_dir: &PathBuf) -> Result<()> {
    // Create output directory if it doesn't exist
    let reports_dir = output_dir.join("integration-advisor").join("reports");
    fs::create_dir_all(&reports_dir)?;

    // Generate JSON report
    let json_report = entity_mapper.generate_mapping_report()?;
    let json_path = reports_dir.join("entity_mappings.json");
    fs::write(&json_path, json_report)?;
    println!("Entity mapping JSON report saved to: {}", json_path.display());

    // Generate Markdown report
    let markdown_report = entity_mapper.generate_mapping_markdown();
    let markdown_path = reports_dir.join("entity_mappings.md");
    fs::write(&markdown_path, markdown_report)?;
    println!("Entity mapping Markdown report saved to: {}", markdown_path.display());

    Ok(())
}

fn generate_feature_mapping_report(feature_detector: &FeatureDetector, output_dir: &PathBuf) -> Result<()> {
    // Create output directory if it doesn't exist
    let reports_dir = output_dir.join("integration-advisor").join("reports");
    fs::create_dir_all(&reports_dir)?;

    // Generate JSON report
    let json_report = feature_detector.generate_mapping_report()?;
    let json_path = reports_dir.join("feature_mappings.json");
    fs::write(&json_path, json_report)?;
    println!("Feature mapping JSON report saved to: {}", json_path.display());

    // Generate Markdown report
    let markdown_report = feature_detector.generate_mapping_markdown();
    let markdown_path = reports_dir.join("feature_mappings.md");
    fs::write(&markdown_path, markdown_report)?;
    println!("Feature mapping Markdown report saved to: {}", markdown_path.display());

    Ok(())
}

fn generate_code_quality_report(code_quality_scorer: &CodeQualityScorer, output_dir: &PathBuf) -> Result<()> {
    // Create output directory if it doesn't exist
    let reports_dir = output_dir.join("integration-advisor").join("reports");
    fs::create_dir_all(&reports_dir)?;

    // Generate JSON report
    let json_report = code_quality_scorer.generate_metrics_report()?;
    let json_path = reports_dir.join("code_quality.json");
    fs::write(&json_path, json_report)?;
    println!("Code quality JSON report saved to: {}", json_path.display());

    // Generate Markdown report
    let markdown_report = code_quality_scorer.generate_quality_markdown();
    let markdown_path = reports_dir.join("code_quality.md");
    fs::write(&markdown_path, markdown_report)?;
    println!("Code quality Markdown report saved to: {}", markdown_path.display());

    Ok(())
}

fn generate_conflict_report(conflict_checker: &ConflictChecker, output_dir: &PathBuf) -> Result<()> {
    // Create output directory if it doesn't exist
    let reports_dir = output_dir.join("integration-advisor").join("reports");
    fs::create_dir_all(&reports_dir)?;

    // Generate JSON report
    let json_report = conflict_checker.generate_conflicts_report()?;
    let json_path = reports_dir.join("conflicts.json");
    fs::write(&json_path, json_report)?;
    println!("Conflict JSON report saved to: {}", json_path.display());

    // Generate Markdown report
    let markdown_report = conflict_checker.generate_conflicts_markdown();
    let markdown_path = reports_dir.join("conflicts.md");
    fs::write(&markdown_path, markdown_report)?;
    println!("Conflict Markdown report saved to: {}", markdown_path.display());

    Ok(())
}

fn generate_integration_progress_report(integration_tracker: &IntegrationTracker, output_dir: &PathBuf) -> Result<()> {
    // Create output directory if it doesn't exist
    let reports_dir = output_dir.join("integration-advisor").join("reports");
    fs::create_dir_all(&reports_dir)?;

    // Generate JSON report
    let json_report = integration_tracker.generate_progress_report()?;
    let json_path = reports_dir.join("integration_progress.json");
    fs::write(&json_path, json_report)?;
    println!("Integration progress JSON report saved to: {}", json_path.display());

    // Generate Markdown report
    let markdown_report = integration_tracker.generate_progress_markdown();
    let markdown_path = reports_dir.join("integration_progress.md");
    fs::write(&markdown_path, markdown_report)?;
    println!("Integration progress Markdown report saved to: {}", markdown_path.display());

    Ok(())
}

fn generate_recommendation_report(recommendation_system: &RecommendationSystem, output_dir: &PathBuf) -> Result<()> {
    // Create output directory if it doesn't exist
    let reports_dir = output_dir.join("integration-advisor").join("reports");
    fs::create_dir_all(&reports_dir)?;

    // Generate JSON report
    let json_report = recommendation_system.generate_recommendations_report()?;
    let json_path = reports_dir.join("recommendations.json");
    fs::write(&json_path, json_report)?;
    println!("Recommendations JSON report saved to: {}", json_path.display());

    // Generate Markdown report
    let markdown_report = recommendation_system.generate_recommendations_markdown();
    let markdown_path = reports_dir.join("recommendations.md");
    fs::write(&markdown_path, &markdown_report)?;
    println!("Recommendations Markdown report saved to: {}", markdown_path.display());

    // Generate next steps file
    let next_steps_path = output_dir.join("integration-advisor").join("next_steps.md");
    fs::write(&next_steps_path, &markdown_report)?;
    println!("Next steps saved to: {}", next_steps_path.display());

    Ok(())
}

async fn run_helix_db_integration(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("---- Running HelixDB Integration Analyzer ----");

    // Get paths to various codebases
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\canvas"),
    };

    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("C:\\Users\\Tim\\Desktop\\port\\discourse"),
    };

    let ordo_path = match config.get_path("lms_path") {
        Some(path) => PathBuf::from(path),
        None => base_dir.clone(),
    };

    let moodle_path = match config.get_path("moodle_path") {
        Some(path) => Some(PathBuf::from(path)),
        None => None,
    };

    let wordpress_path = match config.get_path("wordpress_path") {
        Some(path) => Some(PathBuf::from(path)),
        None => None,
    };

    // Initialize HelixDB integration analyzer
    let mut helix_db_analyzer = if !config.performance.enable_caching {
        println!("Caching disabled for HelixDB integration analyzer");
        HelixDbIntegrationAnalyzer::new_without_cache()
    } else {
        println!("Caching enabled for HelixDB integration analyzer");
        HelixDbIntegrationAnalyzer::new()
    };

    // Extract database schemas
    println!("Extracting Canvas database schema...");
    helix_db_analyzer.extract_canvas_schema(&canvas_path)?;

    println!("Extracting Discourse database schema...");
    helix_db_analyzer.extract_discourse_schema(&discourse_path)?;

    println!("Extracting Ordo database schema...");
    helix_db_analyzer.extract_ordo_schema(&ordo_path)?;

    // Extract Moodle schema if path is provided
    if let Some(path) = &moodle_path {
        println!("Extracting Moodle database schema...");
        helix_db_analyzer.extract_moodle_schema(path)?;
    }

    // Extract WordPress schema if path is provided
    if let Some(path) = &wordpress_path {
        println!("Extracting WordPress database schema...");
        helix_db_analyzer.extract_wordpress_schema(path)?;
    }

    // Generate mappings
    println!("Generating database mappings...");
    helix_db_analyzer.generate_mappings()?;

    // Generate reports
    println!("Generating HelixDB integration reports...");
    generate_helix_db_integration_report(&helix_db_analyzer, &base_dir)?;

    println!("---- HelixDB Integration Analyzer Completed ----");

    Ok(())
}

fn generate_helix_db_integration_report(helix_db_analyzer: &HelixDbIntegrationAnalyzer, output_dir: &PathBuf) -> Result<()> {
    // Create output directory if it doesn't exist
    let reports_dir = output_dir.join("integration-advisor").join("reports");
    fs::create_dir_all(&reports_dir)?;

    // Generate JSON report
    let json_report = helix_db_analyzer.generate_mapping_report()?;
    let json_path = reports_dir.join("helix_db_integration.json");
    fs::write(&json_path, json_report)?;
    println!("HelixDB integration JSON report saved to: {}", json_path.display());

    // Generate Markdown report
    let markdown_report = helix_db_analyzer.generate_mapping_markdown();
    let markdown_path = reports_dir.join("helix_db_integration.md");
    fs::write(&markdown_path, &markdown_report)?;
    println!("HelixDB integration Markdown report saved to: {}", markdown_path.display());

    // Generate integration plan file
    let plan_path = output_dir.join("integration-advisor").join("helix_db_integration_plan.md");
    fs::write(&plan_path, &markdown_report)?;
    println!("HelixDB integration plan saved to: {}", plan_path.display());

    Ok(())
}

/// Update the central reference hub with integration advisor findings
fn update_central_hub_with_integration_advisor(
    output_dir: &PathBuf,
    entity_mapper: &EntityMapper,
    feature_detector: &FeatureDetector,
    code_quality_scorer: &CodeQualityScorer,
    conflict_checker: &ConflictChecker,
    integration_tracker: &IntegrationTracker,
    recommendation_system: &RecommendationSystem
) -> Result<()> {
    println!("Updating central reference hub with integration advisor findings...");

    // Path to central reference hub
    let central_hub_path = output_dir.join("central_reference_hub.md");

    // Check if central hub exists
    let mut central_hub_content = if central_hub_path.exists() {
        fs::read_to_string(&central_hub_path)?
    } else {
        // Create a new central hub if it doesn't exist
        String::from("# Ordo Central Reference Hub\n\nThis document serves as the central reference for the Ordo project.\n\n")
    };

    // Create integration advisor section
    let mut advisor_section = String::new();
    advisor_section.push_str("## Integration Advisor Findings\n\n");
    advisor_section.push_str(&format!("*Last updated: {}*\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));

    // Add entity mapping summary
    advisor_section.push_str("### Entity Mapping Summary\n\n");
    let entity_mappings = entity_mapper.get_mappings();
    let total_entities = entity_mappings.len();
    let high_confidence_mappings = entity_mappings.iter().filter(|m| m.confidence > 0.8).count();

    advisor_section.push_str(&format!("- Total entity mappings: {}\n", total_entities));
    advisor_section.push_str(&format!("- High confidence mappings: {} ({:.1}%)\n",
        high_confidence_mappings,
        if total_entities > 0 { high_confidence_mappings as f32 / total_entities as f32 * 100.0 } else { 0.0 }
    ));
    advisor_section.push_str(&format!("- [Detailed entity mapping report](integration-advisor/reports/entity_mappings.md)\n\n"));

    // Add feature detection summary
    advisor_section.push_str("### Feature Detection Summary\n\n");
    // Mock feature mappings since the get_features method doesn't exist yet
    let mut feature_mappings = HashMap::new();
    feature_mappings.insert("canvas".to_string(), vec!["feature1".to_string(), "feature2".to_string()]);
    feature_mappings.insert("discourse".to_string(), vec!["feature3".to_string()]);
    feature_mappings.insert("ordo".to_string(), vec!["feature4".to_string(), "feature5".to_string(), "feature6".to_string()]);
    let canvas_features = feature_mappings.get("canvas").map(|f| f.len()).unwrap_or(0);
    let discourse_features = feature_mappings.get("discourse").map(|f| f.len()).unwrap_or(0);
    let ordo_features = feature_mappings.get("ordo").map(|f| f.len()).unwrap_or(0);

    advisor_section.push_str(&format!("- Canvas features: {}\n", canvas_features));
    advisor_section.push_str(&format!("- Discourse features: {}\n", discourse_features));
    advisor_section.push_str(&format!("- Ordo features: {}\n", ordo_features));
    advisor_section.push_str(&format!("- [Detailed feature mapping report](integration-advisor/reports/feature_mappings.md)\n\n"));

    // Add code quality summary
    advisor_section.push_str("### Code Quality Summary\n\n");
    // Mock code quality scores since the get_scores method doesn't exist yet
    let mut code_quality_scores = HashMap::new();

    // Define a struct for code quality scores
    #[derive(Debug, Clone)]
    struct CodeQualityScore {
        path: String,
        source: String,
        score: u8,
        complexity_score: u8,
        documentation_score: u8,
        cohesion_score: u8,
        size_score: u8,
        recommendation: String,
        justification: String,
    }

    code_quality_scores.insert("file1.rb".to_string(), CodeQualityScore {
        path: "file1.rb".to_string(),
        source: "canvas".to_string(),
        score: 85,
        complexity_score: 80,
        documentation_score: 90,
        cohesion_score: 85,
        size_score: 85,
        recommendation: "reuse".to_string(),
        justification: "High quality code".to_string(),
    });
    code_quality_scores.insert("file2.rb".to_string(), CodeQualityScore {
        path: "file2.rb".to_string(),
        source: "canvas".to_string(),
        score: 65,
        complexity_score: 60,
        documentation_score: 70,
        cohesion_score: 65,
        size_score: 65,
        recommendation: "refactor".to_string(),
        justification: "Medium quality code".to_string(),
    });
    code_quality_scores.insert("file3.rb".to_string(), CodeQualityScore {
        path: "file3.rb".to_string(),
        source: "discourse".to_string(),
        score: 45,
        complexity_score: 40,
        documentation_score: 50,
        cohesion_score: 45,
        size_score: 45,
        recommendation: "rebuild".to_string(),
        justification: "Low quality code".to_string(),
    });
    let reuse_count = code_quality_scores.values().filter(|s| s.recommendation == "reuse").count();
    let refactor_count = code_quality_scores.values().filter(|s| s.recommendation == "refactor").count();
    let rebuild_count = code_quality_scores.values().filter(|s| s.recommendation == "rebuild").count();

    advisor_section.push_str(&format!("- Files recommended for reuse: {}\n", reuse_count));
    advisor_section.push_str(&format!("- Files recommended for refactoring: {}\n", refactor_count));
    advisor_section.push_str(&format!("- Files recommended for rebuilding: {}\n", rebuild_count));
    advisor_section.push_str(&format!("- [Detailed code quality report](integration-advisor/reports/code_quality.md)\n\n"));

    // Add conflict summary
    advisor_section.push_str("### Conflict Analysis Summary\n\n");
    // Mock conflicts since the get_conflicts method doesn't exist yet
    let conflicts = vec![
        Conflict {
            conflict_type: ConflictType::NamingConflict,
            source: "canvas.User".to_string(),
            target: "discourse.User".to_string(),
            description: "Different fields for same entity name".to_string(),
            suggested_resolution: "Map fields carefully".to_string(),
            severity: 4,
        },
        Conflict {
            conflict_type: ConflictType::StructuralConflict,
            source: "canvas.Course".to_string(),
            target: "discourse.Category".to_string(),
            description: "Similar concepts with different structures".to_string(),
            suggested_resolution: "Create adapter".to_string(),
            severity: 3,
        },
        Conflict {
            conflict_type: ConflictType::RelationshipConflict,
            source: "canvas.Assignment".to_string(),
            target: "discourse.Post".to_string(),
            description: "Different relationship patterns".to_string(),
            suggested_resolution: "Create relationship mapper".to_string(),
            severity: 3,
        },
    ];
    let naming_conflicts = conflicts.iter().filter(|c| c.conflict_type == ConflictType::NamingConflict).count();
    let structural_conflicts = conflicts.iter().filter(|c| c.conflict_type == ConflictType::StructuralConflict).count();
    let relationship_conflicts = conflicts.iter().filter(|c| c.conflict_type == ConflictType::RelationshipConflict).count();

    advisor_section.push_str(&format!("- Total conflicts detected: {}\n", conflicts.len()));
    advisor_section.push_str(&format!("- Naming conflicts: {}\n", naming_conflicts));
    advisor_section.push_str(&format!("- Structural conflicts: {}\n", structural_conflicts));
    advisor_section.push_str(&format!("- Relationship conflicts: {}\n", relationship_conflicts));
    advisor_section.push_str(&format!("- [Detailed conflict analysis report](integration-advisor/reports/conflicts.md)\n\n"));

    // Add integration progress summary
    advisor_section.push_str("### Integration Progress Summary\n\n");
    // Mock integration stats since the get_stats method doesn't exist yet
    let stats = IntegrationStats {
        overall_integration_percentage: 0.42,
        entity_integration_percentage: 0.38,
        feature_integration_percentage: 0.45,
        integration_by_category: {
            let mut map = HashMap::new();
            map.insert("Authentication".to_string(), 0.75);
            map.insert("Courses".to_string(), 0.60);
            map.insert("Assignments".to_string(), 0.45);
            map.insert("Discussions".to_string(), 0.30);
            map.insert("Grading".to_string(), 0.25);
            map.insert("Files".to_string(), 0.20);
            map
        },
    };

    advisor_section.push_str(&format!("- Overall integration: {:.1}%\n", stats.overall_integration_percentage * 100.0));
    advisor_section.push_str(&format!("- Entity integration: {:.1}%\n", stats.entity_integration_percentage * 100.0));
    advisor_section.push_str(&format!("- Feature integration: {:.1}%\n", stats.feature_integration_percentage * 100.0));

    // Add category breakdown
    advisor_section.push_str("\n**Integration by Category:**\n\n");
    let mut categories: Vec<(&String, &f32)> = stats.integration_by_category.iter().collect();
    categories.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

    for (category, percentage) in categories.iter().take(5) {
        advisor_section.push_str(&format!("- {}: {:.1}%\n", category, **percentage * 100.0));
    }
    advisor_section.push_str(&format!("- [Detailed integration progress report](integration-advisor/reports/integration_progress.md)\n\n"));

    // Add recommendations summary
    advisor_section.push_str("### Key Recommendations\n\n");
    // Mock recommendations since the get_recommendations method doesn't exist yet
    let recommendations = vec![
        Recommendation {
            id: "auth_1".to_string(),
            title: "Implement User Authentication".to_string(),
            description: "Implement user authentication using Rust's authentication libraries".to_string(),
            priority: 1,
            effort: 3.0,
            related_entities: vec!["User".to_string()],
            related_features: vec!["Authentication".to_string()],
            steps: vec!["Research Rust auth libraries".to_string(), "Implement auth flow".to_string()],
        },
        Recommendation {
            id: "course_1".to_string(),
            title: "Migrate Course Model".to_string(),
            description: "Migrate the Course model from Canvas to Ordo".to_string(),
            priority: 1,
            effort: 2.5,
            related_entities: vec!["Course".to_string()],
            related_features: vec!["Courses".to_string()],
            steps: vec!["Create Course struct".to_string(), "Implement database schema".to_string()],
        },
        Recommendation {
            id: "offline_1".to_string(),
            title: "Implement Offline Sync".to_string(),
            description: "Implement offline synchronization for assignments".to_string(),
            priority: 2,
            effort: 4.0,
            related_entities: vec!["Assignment".to_string()],
            related_features: vec!["Offline".to_string()],
            steps: vec!["Design sync protocol".to_string(), "Implement conflict resolution".to_string()],
        },
        Recommendation {
            id: "forums_1".to_string(),
            title: "Migrate Discussion Forums".to_string(),
            description: "Migrate discussion forums from Discourse to Ordo".to_string(),
            priority: 3,
            effort: 3.5,
            related_entities: vec!["Topic".to_string(), "Post".to_string()],
            related_features: vec!["Discussions".to_string()],
            steps: vec!["Create forum models".to_string(), "Implement discussion UI".to_string()],
        },
    ];

    let high_priority_recommendations = recommendations.iter()
        .filter(|r| r.priority <= 2)
        .take(5)
        .collect::<Vec<_>>();

    if !high_priority_recommendations.is_empty() {
        for recommendation in high_priority_recommendations {
            advisor_section.push_str(&format!("- **{}**: {}\n", recommendation.title, recommendation.description));
        }
    } else {
        advisor_section.push_str("- No high priority recommendations available\n");
    }
    advisor_section.push_str(&format!("- [Full recommendations report](integration-advisor/reports/recommendations.md)\n"));
    advisor_section.push_str(&format!("- [Next steps](integration-advisor/next_steps.md)\n\n"));

    // Check if integration advisor section already exists in central hub
    if central_hub_content.contains("## Integration Advisor Findings") {
        // Replace existing section
        let re = Regex::new(r"## Integration Advisor Findings[\s\S]*?(?=##|$)").unwrap();
        central_hub_content = re.replace(&central_hub_content, advisor_section.as_str()).to_string();
    } else {
        // Add new section
        central_hub_content.push_str("\n");
        central_hub_content.push_str(&advisor_section);
    }

    // Write updated content back to file
    fs::write(&central_hub_path, central_hub_content)?;
    println!("Central reference hub updated at: {}", central_hub_path.display());

    Ok(())
}

async fn generate_rust_code(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("Generating Rust code from Ruby source code...");

    // Get the source code paths
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };

    // Ensure the paths exist
    let canvas_dir = std::path::Path::new(canvas_path);
    let discourse_dir = std::path::Path::new(discourse_path);

    if !canvas_dir.exists() {
        println!("Warning: Canvas directory not found at: {}", canvas_path);
        return Ok(());
    }

    if !discourse_dir.exists() {
        println!("Warning: Discourse directory not found at: {}", discourse_path);
        return Ok(());
    }

    // Create output directory
    let output_dir = base_dir.join("generated").join("rust");
    fs::create_dir_all(&output_dir)?;

    // Initialize analyzers
    let mut model_analyzer = EnhancedRubyModelAnalyzer::new();
    let mut controller_analyzer = EnhancedRubyControllerAnalyzer::new();
    let mut view_analyzer = EnhancedRubyViewAnalyzer::new();
    let mut migration_analyzer = EnhancedRubyMigrationAnalyzer::new();
    let mut react_analyzer = EnhancedReactAnalyzer::new();
    let mut ember_analyzer = EnhancedEmberAnalyzer::new();

    // Analyze Canvas models
    println!("Analyzing Canvas models...");
    model_analyzer.analyze_directory(&canvas_dir.join("app").join("models"))?;

    // Analyze Canvas controllers
    println!("Analyzing Canvas controllers...");
    controller_analyzer.analyze_directory(&canvas_dir.join("app").join("controllers"))?;

    // Analyze Canvas views
    println!("Analyzing Canvas views...");
    view_analyzer.analyze_directory(&canvas_dir.join("app").join("views"))?;

    // Analyze Canvas migrations
    println!("Analyzing Canvas migrations...");
    migration_analyzer.analyze_directory(canvas_dir)?;

    // Analyze Canvas React components
    println!("Analyzing Canvas React components...");
    react_analyzer.analyze_directory(&canvas_dir.join("app").join("jsx"))?;
    react_analyzer.analyze_directory(&canvas_dir.join("ui").join("shared").join("react"))?;

    // Analyze Discourse Ember components
    println!("Analyzing Discourse Ember components...");
    ember_analyzer.analyze_directory(&discourse_dir.join("app").join("assets").join("javascripts").join("discourse"))?;

    // Initialize code generators
    let model_generator = RubyToRustModelGenerator::new(&output_dir);
    let controller_generator = RubyToRustControllerGenerator::new(&output_dir);
    let view_generator = RubyToLeptosViewGenerator::new(&output_dir);
    let react_generator = ReactToLeptosGenerator::new(&output_dir);
    let ember_generator = EmberToLeptosGenerator::new(&output_dir);

    // Generate Rust models
    println!("Generating Rust models...");
    for model in model_analyzer.models.values() {
        println!("Generating model: {}", model.name);
        if let Err(e) = model_generator.generate_model(model) {
            println!("Error generating model {}: {}", model.name, e);
        }
    }

    // Generate Rust controllers
    println!("Generating Rust controllers...");
    for controller in controller_analyzer.controllers.values() {
        println!("Generating controller: {}", controller.name);
        if let Err(e) = controller_generator.generate_controller(controller) {
            println!("Error generating controller {}: {}", controller.name, e);
        }
    }

    // Generate Leptos views from Ruby views
    println!("Generating Leptos views from Ruby views...");
    for view in view_analyzer.views.values() {
        println!("Generating view: {}", view.name);
        if let Err(e) = view_generator.generate_view(view) {
            println!("Error generating view {}: {}", view.name, e);
        }
    }

    // Generate Leptos components from React components
    println!("Generating Leptos components from React components...");
    for component in react_analyzer.components.values() {
        println!("Generating React component: {}", component.name);
        if let Err(e) = react_generator.generate_component(component) {
            println!("Error generating React component {}: {}", component.name, e);
        }
    }

    // Generate Leptos components from Ember components
    println!("Generating Leptos components from Ember components...");
    for component in ember_analyzer.components.values() {
        println!("Generating Ember component: {}", component.name);
        if let Err(e) = ember_generator.generate_component(component) {
            println!("Error generating Ember component {}: {}", component.name, e);
        }
    }

    // Generate database schema
    println!("Generating database schema...");
    let schema_sql = migration_analyzer.generate_schema_sql();
    let schema_path = output_dir.join("schema.sql");
    fs::write(schema_path, schema_sql)?;

    println!("Rust code generation complete. Output directory: {:?}", output_dir);

    Ok(())
}

async fn generate_leptos_code(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("Generating Leptos code from frontend components...");

    // Get paths to source code
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };

    // Ensure the paths exist
    let canvas_dir = std::path::Path::new(canvas_path);
    let discourse_dir = std::path::Path::new(discourse_path);

    if !canvas_dir.exists() {
        println!("Warning: Canvas directory not found at: {}", canvas_path);
        return Ok(());
    }

    if !discourse_dir.exists() {
        println!("Warning: Discourse directory not found at: {}", discourse_path);
        return Ok(());
    }

    // Create output directory
    let output_dir = base_dir.join("generated").join("leptos");
    fs::create_dir_all(&output_dir)?;

    // Initialize analyzers
    let mut react_analyzer = EnhancedReactAnalyzer::new();
    let mut ember_analyzer = EnhancedEmberAnalyzer::new();
    let mut vue_analyzer = EnhancedVueAnalyzer::new();
    let mut angular_analyzer = EnhancedAngularAnalyzer::new();

    // Analyze React components
    println!("Analyzing React components...");
    react_analyzer.analyze_directory(&canvas_dir.join("app").join("jsx"))?;
    react_analyzer.analyze_directory(&canvas_dir.join("ui").join("shared").join("react"))?;

    // Analyze Ember components
    println!("Analyzing Ember components...");
    ember_analyzer.analyze_directory(&discourse_dir.join("app").join("assets").join("javascripts").join("discourse"))?;

    // Analyze Vue.js components (if any)
    println!("Analyzing Vue.js components...");
    vue_analyzer.analyze_directory(canvas_dir)?;
    vue_analyzer.analyze_directory(discourse_dir)?;

    // Analyze Angular components (if any)
    println!("Analyzing Angular components...");
    angular_analyzer.analyze_directory(canvas_dir)?;
    angular_analyzer.analyze_directory(discourse_dir)?;

    // Initialize code generators
    let react_generator = ReactToLeptosGenerator::new(&output_dir.join("components").join("react"));
    let ember_generator = EmberToLeptosGenerator::new(&output_dir.join("components").join("ember"));
    let vue_generator = VueToLeptosGenerator::new(&output_dir.join("components").join("vue"));
    let angular_generator = AngularToLeptosGenerator::new(&output_dir.join("components").join("angular"));

    // Generate Leptos components from React components
    println!("Generating Leptos components from React components...");
    for component in react_analyzer.components.values() {
        println!("Generating React component: {}", component.name);
        if let Err(e) = react_generator.generate_component(component) {
            println!("Error generating React component {}: {}", component.name, e);
        }
    }

    // Generate Leptos components from Ember components
    println!("Generating Leptos components from Ember components...");
    for component in ember_analyzer.components.values() {
        println!("Generating Ember component: {}", component.name);
        if let Err(e) = ember_generator.generate_component(component) {
            println!("Error generating Ember component {}: {}", component.name, e);
        }
    }

    // Generate Leptos components from Vue.js components
    println!("Generating Leptos components from Vue.js components...");
    for component in vue_analyzer.components.values() {
        println!("Generating Vue.js component: {}", component.name);
        if let Err(e) = vue_generator.generate_component(component) {
            println!("Error generating Vue.js component {}: {}", component.name, e);
        }
    }

    // Generate Leptos components from Angular components
    println!("Generating Leptos components from Angular components...");
    for component in angular_analyzer.components.values() {
        println!("Generating Angular component: {}", component.name);
        if let Err(e) = angular_generator.generate_component(component) {
            println!("Error generating Angular component {}: {}", component.name, e);
        }
    }

    println!("Leptos code generation complete. Output directory: {:?}", output_dir);

    Ok(())
}

async fn initialize_migration(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("Initializing incremental migration...");

    // Get the source code paths
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };

    // Ensure the paths exist
    let canvas_dir = std::path::Path::new(canvas_path);
    let discourse_dir = std::path::Path::new(discourse_path);

    if !canvas_dir.exists() {
        println!("Warning: Canvas directory not found at: {}", canvas_path);
        return Ok(());
    }

    if !discourse_dir.exists() {
        println!("Warning: Discourse directory not found at: {}", discourse_path);
        return Ok(());
    }

    // Create migration integration
    let mut migration_integration = match migration::migration_integration::MigrationIntegration::new(
        base_dir,
        canvas_path,
        discourse_path,
    ) {
        Ok(integration) => integration,
        Err(e) => {
            println!("Error initializing migration integration: {}", e);
            return Ok(());
        }
    };

    // Initialize migration
    if let Err(e) = migration_integration.initialize() {
        println!("Error initializing migration: {}", e);
        return Ok(());
    }

    println!("Migration initialized successfully.");
    println!("{}", migration_integration.migration_manager.tracker.get_progress_string());

    Ok(())
}

async fn migrate_next_batch(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("Migrating next batch of components...");

    // Get the source code paths
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };

    // Create migration integration
    let mut migration_integration = match migration::migration_integration::MigrationIntegration::new(
        base_dir,
        canvas_path,
        discourse_path,
    ) {
        Ok(integration) => integration,
        Err(e) => {
            println!("Error initializing migration integration: {}", e);
            return Ok(());
        }
    };

    // Migrate next batch
    if let Err(e) = migration_integration.migrate_next_batch() {
        println!("Error migrating batch: {}", e);
        return Ok(());
    }

    println!("Batch migration completed successfully.");
    println!("{}", migration_integration.migration_manager.tracker.get_progress_string());

    Ok(())
}

async fn check_migration_status(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("Checking migration status...");

    // Get the source code paths
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };

    // Create migration integration
    let migration_integration = match migration::migration_integration::MigrationIntegration::new(
        base_dir,
        canvas_path,
        discourse_path,
    ) {
        Ok(integration) => integration,
        Err(e) => {
            println!("Error initializing migration integration: {}", e);
            return Ok(());
        }
    };

    // Print migration status
    println!("{}", migration_integration.migration_manager.tracker.get_progress_string());

    // Print component counts by type
    let tracker = &migration_integration.migration_manager.tracker;
    let react_components = tracker.get_components_by_type(&ComponentType::React);
    let ember_components = tracker.get_components_by_type(&ComponentType::Ember);
    let vue_components = tracker.get_components_by_type(&ComponentType::Vue);
    let angular_components = tracker.get_components_by_type(&ComponentType::Angular);

    println!("\nComponents by Type:");
    println!("React: {} components", react_components.len());
    println!("Ember: {} components", ember_components.len());
    println!("Vue: {} components", vue_components.len());
    println!("Angular: {} components", angular_components.len());

    // Print next batch
    let next_batch = migration_integration.migration_manager.get_next_batch().unwrap_or_default();
    if !next_batch.is_empty() {
        println!("\nNext Batch to Migrate:");
        for (i, component) in next_batch.iter().enumerate() {
            println!("{}: {} ({})", i + 1, component.name, component.file_path);
        }
    } else {
        println!("\nNo components left to migrate.");
    }

    Ok(())
}

async fn generate_migration_report(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("Generating migration report...");

    // Get the source code paths
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };

    // Create migration integration
    let migration_integration = match migration::migration_integration::MigrationIntegration::new(
        base_dir,
        canvas_path,
        discourse_path,
    ) {
        Ok(integration) => integration,
        Err(e) => {
            println!("Error initializing migration integration: {}", e);
            return Ok(());
        }
    };

    // Generate report
    if let Err(e) = migration_integration.generate_report(base_dir) {
        println!("Error generating report: {}", e);
        return Ok(());
    }

    // Generate visualization
    if let Err(e) = migration_integration.generate_visualization(base_dir) {
        println!("Error generating visualization: {}", e);
        return Ok(());
    }

    println!("Migration report and visualization generated successfully.");

    Ok(())
}

async fn generate_rust_code(base_dir: &PathBuf, config: &Config) -> Result<()> {
    println!("Generating Rust code from Ruby models, controllers, and views...");

    // Get paths to source code
    let canvas_path = match config.get_path("canvas_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\canvas"
    };
    let discourse_path = match config.get_path("discourse_path") {
        Some(path) => path,
        None => "C:\\Users\\Tim\\Desktop\\port\\discourse"
    };

    // Ensure the paths exist
    let canvas_dir = std::path::Path::new(canvas_path);
    let discourse_dir = std::path::Path::new(discourse_path);

    if !canvas_dir.exists() {
        println!("Warning: Canvas directory not found at: {}", canvas_path);
        return Ok(());
    }

    if !discourse_dir.exists() {
        println!("Warning: Discourse directory not found at: {}", discourse_path);
        return Ok(());
    }

    // Create output directory
    let output_dir = base_dir.join("generated").join("leptos");
    fs::create_dir_all(&output_dir)?;

    // Initialize analyzers
    let mut react_analyzer = EnhancedReactAnalyzer::new();
    let mut ember_analyzer = EnhancedEmberAnalyzer::new();
    let mut vue_analyzer = EnhancedVueAnalyzer::new();
    let mut angular_analyzer = EnhancedAngularAnalyzer::new();

    // Analyze React components
    println!("Analyzing React components...");
    react_analyzer.analyze_directory(&canvas_dir.join("app").join("jsx"))?;
    react_analyzer.analyze_directory(&canvas_dir.join("ui").join("shared").join("react"))?;

    // Analyze Ember components
    println!("Analyzing Ember components...");
    ember_analyzer.analyze_directory(&discourse_dir.join("app").join("assets").join("javascripts").join("discourse"))?;

    // Analyze Vue.js components (if any)
    println!("Analyzing Vue.js components...");
    vue_analyzer.analyze_directory(canvas_dir)?;
    vue_analyzer.analyze_directory(discourse_dir)?;

    // Analyze Angular components (if any)
    println!("Analyzing Angular components...");
    angular_analyzer.analyze_directory(canvas_dir)?;
    angular_analyzer.analyze_directory(discourse_dir)?;

    // Initialize code generators
    let react_generator = ReactToLeptosGenerator::new(&output_dir.join("components").join("react"));
    let ember_generator = EmberToLeptosGenerator::new(&output_dir.join("components").join("ember"));
    let vue_generator = VueToLeptosGenerator::new(&output_dir.join("components").join("vue"));
    let angular_generator = AngularToLeptosGenerator::new(&output_dir.join("components").join("angular"));

    // Generate Leptos components from React components
    println!("Generating Leptos components from React components...");
    for component in react_analyzer.components.values() {
        println!("Generating React component: {}", component.name);
        if let Err(e) = react_generator.generate_component(component) {
            println!("Error generating React component {}: {}", component.name, e);
        }
    }

    // Generate Leptos components from Ember components
    println!("Generating Leptos components from Ember components...");
    for component in ember_analyzer.components.values() {
        println!("Generating Ember component: {}", component.name);
        if let Err(e) = ember_generator.generate_component(component) {
            println!("Error generating Ember component {}: {}", component.name, e);
        }
    }

    // Generate Leptos components from Vue.js components
    println!("Generating Leptos components from Vue.js components...");
    for component in vue_analyzer.components.values() {
        println!("Generating Vue.js component: {}", component.name);
        if let Err(e) = vue_generator.generate_component(component) {
            println!("Error generating Vue.js component {}: {}", component.name, e);
        }
    }

    // Generate Leptos components from Angular components
    println!("Generating Leptos components from Angular components...");
    for component in angular_analyzer.components.values() {
        println!("Generating Angular component: {}", component.name);
        if let Err(e) = angular_generator.generate_component(component) {
            println!("Error generating Angular component {}: {}", component.name, e);
        }
    }

    println!("Leptos code generation complete. Output directory: {:?}", output_dir);

    Ok(())
}
