use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnifiedAnalysisOutput {
    pub files: Vec<FileMetadata>,
    pub routes: Vec<RouteInfo>,
    pub components: HashMap<String, ComponentInfo>,
    pub api: ApiInfo,
    pub templates: Vec<TemplateInfo>,
    pub auth: AuthInfo,
    pub database: DatabaseInfo,
    pub business_logic: BusinessLogicInfo,
    pub offline_readiness: OfflineReadinessInfo,
    pub file_dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileMetadata {
    pub path: String,
    pub file_type: String,
    pub size: u64,
    pub modified_time: String,
    pub directory: Option<String>,
    pub purpose: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteInfo {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub auth_required: bool,
    pub params: Vec<String>,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComponentInfo {
    pub name: String,
    pub file_path: String,
    pub framework: String,
    pub props: Vec<String>,
    pub state: Option<Vec<String>>,
    pub lifecycle_hooks: Option<Vec<String>>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiInfo {
    pub endpoints: Vec<ApiEndpointInfo>,
    pub base_url: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiEndpointInfo {
    pub path: String,
    pub http_method: String,
    pub controller: Option<String>,
    pub action: Option<String>,
    pub description: Option<String>,
    pub request_params: Vec<String>,
    pub response_format: Option<String>,
    pub auth_required: bool,
    pub rate_limited: bool,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiParameterInfo {
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TemplateInfo {
    pub path: String,
    pub template_type: String,
    pub bindings: Vec<String>,
    pub partials: Vec<String>,
    pub loops: Vec<String>,
    pub conditionals: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthInfo {
    pub authentication_methods: Vec<String>,
    pub roles: HashMap<String, Vec<String>>,
    pub csrf_protection: bool,
    pub session_management: bool,
    pub password_policies: HashMap<String, String>,
    pub sso_integrations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatabaseInfo {
    pub tables: Vec<DatabaseTableInfo>,
    pub relationships: Vec<RelationshipInfo>,
    pub db_type: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatabaseTableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<IndexInfo>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub foreign_key: bool,
    pub references: Option<String>,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RelationshipInfo {
    pub source_table: String,
    pub target_table: String,
    pub relationship_type: String,
    pub source_column: String,
    pub target_column: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MigrationInfo {
    pub version: String,
    pub name: String,
    pub operations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BusinessLogicInfo {
    pub core_patterns: Vec<String>,
    pub domain_algorithms: HashMap<String, String>,
    pub workflows: Vec<WorkflowInfo>,
    pub edge_cases: Vec<String>,
    pub business_rules: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowInfo {
    pub name: String,
    pub steps: Vec<String>,
    pub actors: Vec<String>,
    pub triggers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OfflineReadinessInfo {
    pub offline_capable: bool,
    pub data_storage_mechanism: Option<String>,
    pub sync_mechanism: Option<String>,
    pub conflict_resolution_strategy: Option<String>,
    pub network_detection: bool,
    pub offline_features: Vec<String>,
    pub online_only_features: Vec<String>,
}