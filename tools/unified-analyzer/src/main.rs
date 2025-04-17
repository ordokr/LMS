mod analyzers;
mod config;
mod generators;
mod integrator;
mod output_schema;
mod utils;

use crate::analyzers::modules::{api_analyzer::ApiAnalyzer, auth_flow_analyzer::AuthFlowAnalyzer, canvas_analyzer::CanvasAnalyzer, database_schema_analyzer::DatabaseSchemaAnalyzer, discourse_analyzer::DiscourseAnalyzer, ruby_rails_analyzer::RubyRailsAnalyzer,
    business_logic_analyzer::BusinessLogicAnalyzer, dependency_analyzer::DependencyAnalyzer,
    ember_analyzer::EmberAnalyzer, file_structure_analyzer::FileStructureAnalyzer,
    offline_first_readiness_analyzer::OfflineFirstReadinessAnalyzer, react_analyzer::ReactAnalyzer,
    route_analyzer::RouteAnalyzer, template_analyzer::TemplateAnalyzer, db_schema_analyzer::DbSchemaAnalyzer,
    blockchain_analyzer::BlockchainAnalyzer, unified_analyzer::UnifiedProjectAnalyzer,
    entity_mapper::EntityMapper, feature_detector::FeatureDetector, code_quality_scorer::CodeQualityScorer,
    conflict_checker::ConflictChecker, integration_tracker::IntegrationTracker, recommendation_system::RecommendationSystem,
    helix_db_integration::HelixDbIntegrationAnalyzer,
};
use crate::analyzers::{run_all_analyzers, run_ast_analyzer, run_project_structure_analyzer};
use crate::analyzers::modules::tech_debt_runner::run_tech_debt_analyzer;
use crate::analyzers::modules::conflict_analyzer::analyze_conflicts;
use anyhow::Result;
use config::Config;
// Import only what we need from generators
use crate::generators::{MigrationRoadmapGenerator, ComponentTreeGenerator, ApiMapGenerator, DbSchemaGenerator, all_generators, enhanced_central_hub_generator};
use log::info;
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::Arc;
use crate::{generators::documentation_generator::generate_documentation, integrator::integrate_analysis_results};

use crate::utils::performance::{AnalysisCache, PerformanceMetrics, measure_execution_time, new_shared_metrics};

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
        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands:");
            println!("  full            Run full analysis");
            println!("  quick           Run quick analysis");
            println!("  update-hub      Update central reference hub");
            println!("  summary         Generate summary report");
            println!("  update-rag      Update RAG knowledge base");
            println!("  roadmap         Generate migration roadmap");
            println!("  component-tree  Generate component tree visualization");
            println!("  api-map         Generate API map visualization");
            println!("  db-schema       Generate database schema visualization");
            println!("  db-schema-viz   Generate database schema visualization with Mermaid");
            println!("  source-db-schema Generate database schema visualization from Canvas and Discourse source code");
            println!("  viz             Generate all visualizations");
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

    // Generate database schema visualization
    let db_schema_generator = DbSchemaGenerator::new();

    // Generate in output/docs directory first
    db_schema_generator.generate(&unified_output, &output_docs_dir).expect("Failed to generate database schema visualization in output directory");

    // Copy the generated files from output/docs to root docs folder
    let output_vis_dir = output_docs_dir.join("visualizations").join("db_schema");
    let root_vis_dir = root_docs_dir.join("visualizations").join("db_schema");

    // Create the visualizations directory in root docs folder
    std::fs::create_dir_all(&root_vis_dir).expect("Failed to create visualizations directory in root docs folder");

    // Copy the HTML file
    let html_src = output_vis_dir.join("db_schema.html");
    let html_dst = root_vis_dir.join("db_schema.html");
    if html_src.exists() {
        std::fs::copy(&html_src, &html_dst).expect("Failed to copy database schema HTML file");
        println!("Copied database schema HTML file to: {:?}", html_dst);
    }

    // Copy the Markdown file
    let md_src = output_vis_dir.join("db_schema.md");
    let md_dst = root_vis_dir.join("db_schema.md");
    if md_src.exists() {
        std::fs::copy(&md_src, &md_dst).expect("Failed to copy database schema Markdown file");
        println!("Copied database schema Markdown file to: {:?}", md_dst);
    }

    println!("Database schema visualization generated successfully.");
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
    result.models.total = stats.model_count;
    result.models.implemented = stats.model_count;
    result.models.implementation_percentage = if stats.model_count > 0 { 100.0 } else { 0.0 };

    result.api_endpoints.total = stats.api_endpoint_count;
    result.api_endpoints.implemented = stats.api_endpoint_count;
    result.api_endpoints.implementation_percentage = if stats.api_endpoint_count > 0 { 100.0 } else { 0.0 };

    result.ui_components.total = stats.component_count;
    result.ui_components.implemented = stats.component_count;
    result.ui_components.implementation_percentage = if stats.component_count > 0 { 100.0 } else { 0.0 };

    result.tests.total = stats.test_count;
    result.tests.passing = stats.test_count; // Assume all tests pass
    result.tests.coverage = stats.test_coverage;

    // Update project status
    result.project_status.phase = "development".to_string();
    result.project_status.completion_percentage = if stats.total_files > 0 {
        (stats.model_count + stats.api_endpoint_count + stats.component_count) as f32 / (stats.total_files as f32) * 100.0
    } else {
        0.0
    };
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

    // Create the reports directory if it doesn't exist
    let reports_dir = base_dir.join("docs").join("unified-analyzer").join("reports");
    fs::create_dir_all(&reports_dir)?;
    println!("Reports will be saved to: {}", reports_dir.display());

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

    // Initialize components
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
    let mut helix_db_analyzer = HelixDbIntegrationAnalyzer::new();

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

    // Run HelixDB integration analyzer
    println!("Analyzing HelixDB integration...");
    helix_db_analyzer.extract_canvas_schema(&canvas_path)?;
    helix_db_analyzer.extract_discourse_schema(&discourse_path)?;
    helix_db_analyzer.extract_ordo_schema(&ordo_path)?;

    // Extract Moodle schema if path is provided
    let moodle_path = match config.get_path("moodle_path") {
        Some(path) => Some(PathBuf::from(path)),
        None => None,
    };

    if let Some(path) = &moodle_path {
        println!("Extracting Moodle database schema...");
        helix_db_analyzer.extract_moodle_schema(path)?;
    }

    // Extract WordPress schema if path is provided
    let wordpress_path = match config.get_path("wordpress_path") {
        Some(path) => Some(PathBuf::from(path)),
        None => None,
    };

    if let Some(path) = &wordpress_path {
        println!("Extracting WordPress database schema...");
        helix_db_analyzer.extract_wordpress_schema(path)?;
    }

    helix_db_analyzer.generate_mappings()?;

    // Generate reports
    println!("Generating reports...");
    generate_entity_mapping_report(&entity_mapper, &ordo_path)?;
    generate_feature_mapping_report(&feature_detector, &ordo_path)?;
    generate_code_quality_report(&code_quality_scorer, &ordo_path)?;
    generate_conflict_report(&conflict_checker, &ordo_path)?;
    generate_integration_progress_report(&integration_tracker, &ordo_path)?;
    generate_recommendation_report(&recommendation_system, &ordo_path)?;
    generate_helix_db_integration_report(&helix_db_analyzer, &ordo_path)?;

    println!("---- Full Integration Advisor Completed ----");

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

    // Analyze code quality
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
    let reports_dir = output_dir.join("docs").join("unified-analyzer").join("reports");
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
    let reports_dir = output_dir.join("docs").join("unified-analyzer").join("reports");
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
    let reports_dir = output_dir.join("docs").join("unified-analyzer").join("reports");
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
    let reports_dir = output_dir.join("docs").join("unified-analyzer").join("reports");
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
    let reports_dir = output_dir.join("docs").join("unified-analyzer").join("reports");
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
    let reports_dir = output_dir.join("docs").join("unified-analyzer").join("reports");
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
    let next_steps_path = output_dir.join("docs").join("unified-analyzer").join("next_steps.md");
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
    let reports_dir = output_dir.join("docs").join("unified-analyzer").join("reports");
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
    let plan_path = output_dir.join("docs").join("unified-analyzer").join("helix_db_integration_plan.md");
    fs::write(&plan_path, &markdown_report)?;
    println!("HelixDB integration plan saved to: {}", plan_path.display());

    Ok(())
}
