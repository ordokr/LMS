// Responsible for analyzing Abstract Syntax Trees
pub mod ast_analyzer;
// Tests for AST analyzer
#[cfg(test)]
mod ast_analyzer_tests;
// Responsible for analyzing and detecting conflicts
pub mod conflict_analyzer;
// Responsible for analyzing and detecting technical debt
pub mod tech_debt_analyzer;
// Runner for tech debt analyzer
pub mod tech_debt_runner;
// Tests for tech debt analyzer
#[cfg(test)]
mod tech_debt_analyzer_tests;
// Enhanced tech debt analyzer with parallel processing and multi-factor severity scoring
pub mod enhanced_tech_debt_analyzer;
// Responsible for analyzing and extracting database schema information
pub mod db_schema_analyzer;
// Responsible for analyzing blockchain-related code
pub mod blockchain_analyzer;
// Responsible for analyzing Haskell code
pub mod haskell_analyzer;

// Responsible for analyzing Ruby on Rails code (Canvas & Discourse)
pub mod ruby_rails_analyzer;
// Enhanced Ruby model analyzer with detailed extraction capabilities
pub mod enhanced_ruby_model_analyzer;
#[cfg(test)]
mod enhanced_ruby_model_analyzer_tests;
// Enhanced Ruby controller analyzer with detailed extraction capabilities
pub mod enhanced_ruby_controller_analyzer;
#[cfg(test)]
mod enhanced_ruby_controller_analyzer_tests;
// Enhanced Ruby view analyzer with detailed extraction capabilities
pub mod enhanced_ruby_view_analyzer;
#[cfg(test)]
mod enhanced_ruby_view_analyzer_tests;
// Enhanced Ruby migration analyzer with detailed extraction capabilities
pub mod enhanced_ruby_migration_analyzer;
#[cfg(test)]
mod enhanced_ruby_migration_analyzer_tests;
// Enhanced React analyzer with detailed extraction capabilities
pub mod enhanced_react_analyzer;
#[cfg(test)]
mod enhanced_react_analyzer_tests;
// Enhanced Ember analyzer with detailed extraction capabilities
pub mod enhanced_ember_analyzer;
#[cfg(test)]
mod enhanced_ember_analyzer_tests;
// Enhanced Vue.js analyzer with detailed extraction capabilities
pub mod enhanced_vue_analyzer;
#[cfg(test)]
mod enhanced_vue_analyzer_tests;
// Enhanced Angular analyzer with detailed extraction capabilities
pub mod enhanced_angular_analyzer;
#[cfg(test)]
mod enhanced_angular_analyzer_tests;
// Responsible for analyzing React code (Canvas)
pub mod react_analyzer;
// Responsible for analyzing Ember.js code (Discourse)
pub mod ember_analyzer;
// Responsible for analyzing template files (ERB, HBS, HTML)
pub mod template_analyzer;
// Incremental template analyzer with caching for large codebases
pub mod incremental_template_analyzer;
// Responsible for analyzing and unifying route information
pub mod route_analyzer;
// Responsible for analyzing API endpoints and interactions
pub mod api_analyzer;
// Enhanced API analyzer with detailed pattern matching
pub mod enhanced_api_analyzer;
// Incremental API analyzer with caching for large codebases
pub mod incremental_api_analyzer;
// Responsible for analyzing project dependencies
pub mod dependency_analyzer;
// Incremental dependency analyzer with caching for large codebases
pub mod incremental_dependency_analyzer;
// Responsible for analyzing authentication and authorization flows
pub mod auth_flow_analyzer;
// Responsible for analyzing offline-first readiness of the application
pub mod offline_first_readiness_analyzer;
// Responsible for analyzing and extracting database schema
pub mod database_schema_analyzer;
pub mod improved_db_schema_analyzer;
// Fix for database schema analyzer to ensure accurate detection and reporting
pub mod db_schema_fix;
// Responsible for analyzing core business logic and workflows
pub mod business_logic_analyzer;
// Responsible for analyzing file structure and dependencies
pub mod file_structure_analyzer;
// Incremental file structure analyzer with caching for large codebases
pub mod incremental_file_structure_analyzer;
// Responsible for unifying the analysis from all modules
pub mod unified_analyzer;
	// Responsible for analyzing Canvas LMS specifically
	pub mod canvas_analyzer;
	// Responsible for analyzing Discourse specifically
	pub mod discourse_analyzer;

// Integration Advisor Modules
// Responsible for mapping entities between systems
pub mod entity_mapper;
// Responsible for detecting and mapping features between systems
pub mod feature_detector;
// Responsible for analyzing code quality and usefulness
pub mod code_quality_scorer;
// Responsible for detecting naming and semantic conflicts
pub mod conflict_checker;
// Responsible for tracking integration progress
pub mod integration_tracker;
// Responsible for generating development recommendations
pub mod recommendation_system;
// Improved recommendation system focused on migration strategies
pub mod recommendation_system_improved;
// Responsible for HelixDB integration planning
pub mod helix_db_integration;
// Source system implementations for various platforms
pub mod source_systems;
