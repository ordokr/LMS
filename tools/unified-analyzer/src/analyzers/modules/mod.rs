// Responsible for analyzing Abstract Syntax Trees
pub mod ast_analyzer;
// Responsible for analyzing and detecting conflicts
pub mod conflict_analyzer;
// Responsible for analyzing and detecting technical debt
pub mod tech_debt_analyzer;
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
