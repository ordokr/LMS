use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnifiedOutput {
    pub files: Vec<FileMetadata>,
    pub routes: Vec<RouteInfo>,
    pub components: Vec<ComponentInfo>,
    pub api_map: Vec<ApiEndpointInfo>,
    pub templates: Vec<TemplateInfo>,
    pub auth: AuthInfo,
    pub database: DatabaseInfo,
    pub business_logic: BusinessLogicInfo,
    pub offline_plan: OfflinePlanInfo,
    pub file_dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: String,
    pub file_type: String,
    pub size: u64,
    pub modified_time: String,
    pub directory: Option<String>,
    pub purpose: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub auth_required: bool,
    pub params: Vec<String>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub file_path: String,
    pub framework: String,
    pub props: Vec<String>,
    pub state: Option<Vec<String>>,
    pub lifecycle_hooks: Option<Vec<String>>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiEndpointInfo {
    pub path: String,
    pub method: String,
    pub controller: String,
    pub action: String,
    pub parameters: Vec<ApiParameterInfo>,
    pub response_format: Option<String>,
    pub auth_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiParameterInfo {
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub path: String,
    pub template_type: String,
    pub bindings: Vec<String>,
    pub partials: Vec<String>,
    pub loops: Vec<String>,
    pub conditionals: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub authentication_methods: Vec<String>,
    pub roles: HashMap<String, Vec<String>>,
    pub csrf_protection: bool,
    pub session_management: bool,
    pub password_policies: HashMap<String, String>,
    pub sso_integrations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub tables: Vec<TableInfo>,
    pub relationships: Vec<RelationshipInfo>,
    pub indexes: Vec<IndexInfo>,
    pub migrations: Vec<MigrationInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub primary_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub r#type: String,
    pub nullable: bool,
    pub default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipInfo {
    pub from_table: String,
    pub to_table: String,
    pub relationship_type: String,
    pub foreign_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexInfo {
    pub table: String,
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationInfo {
    pub version: String,
    pub name: String,
    pub operations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessLogicInfo {
    pub core_patterns: Vec<String>,
    pub domain_algorithms: HashMap<String, String>,
    pub workflows: Vec<WorkflowInfo>,
    pub edge_cases: Vec<String>,
    pub business_rules: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowInfo {
    pub name: String,
    pub steps: Vec<String>,
    pub actors: Vec<String>,
    pub triggers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfflinePlanInfo {
    pub data_access_patterns: Vec<DataAccessPatternInfo>,
    pub sync_strategies: Vec<String>,
    pub conflict_resolution: Vec<String>,
    pub storage_requirements: HashMap<String, String>,
    pub critical_online_features: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAccessPatternInfo {
    pub pattern_type: String,
    pub description: String,
    pub sync_feasibility: String,
}