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
// Responsible for analyzing and extracting database schema information
pub mod db_schema_analyzer;
// Responsible for analyzing blockchain-related code
pub mod blockchain_analyzer;

// Responsible for analyzing Ruby on Rails code (Canvas & Discourse)
pub mod ruby_rails_analyzer;
// Responsible for analyzing React code (Canvas)
pub mod react_analyzer;
// Responsible for analyzing Ember.js code (Discourse)
pub mod ember_analyzer;
// Responsible for analyzing template files (ERB, HBS, HTML)
pub mod template_analyzer;
// Responsible for analyzing and unifying route information
pub mod route_analyzer;
// Responsible for analyzing API endpoints and interactions
pub mod api_analyzer;
// Enhanced API analyzer with detailed pattern matching
pub mod enhanced_api_analyzer;
// Responsible for analyzing project dependencies
pub mod dependency_analyzer;
// Responsible for analyzing authentication and authorization flows
pub mod auth_flow_analyzer;
// Responsible for analyzing offline-first readiness of the application
pub mod offline_first_readiness_analyzer;
// Responsible for analyzing and extracting database schema
pub mod database_schema_analyzer;
// Responsible for analyzing core business logic and workflows
pub mod business_logic_analyzer;
// Responsible for analyzing file structure and dependencies
pub mod file_structure_analyzer;
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
// Responsible for HelixDB integration planning
pub mod helix_db_integration;
// Source system implementations for various platforms
pub mod source_systems;
