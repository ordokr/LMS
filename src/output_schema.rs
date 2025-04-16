rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub file_type: String,
    pub size: u64,
    pub modified_time: String,
    pub dependencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteInfo {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub parameters: Vec<String>,
    pub authentication_required: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentInfo {
    pub name: String,
    pub file_path: String,
    pub component_type: String,
    pub props: Vec<String>,
    pub state: Vec<String>,
    pub lifecycle_methods: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiCallInfo {
    pub endpoint: String,
    pub method: String,
    pub controller: String,
    pub input_schema: HashMap<String, String>,
    pub output_schema: HashMap<String, String>,
    pub authentication_required: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateInfo {
    pub file_path: String,
    pub template_type: String,
    pub data_bindings: Vec<String>,
    pub loops: Vec<String>,
    pub partials: Vec<String>,
    pub inheritance: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthInfo {
    pub login_logic: String,
    pub session_management: String,
    pub roles: Vec<String>,
    pub permissions: HashMap<String, Vec<String>>,
    pub csrf_protection: bool,
    pub token_flow: String,
    pub cookie_usage: String,
    pub sso_integrations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseSchema {
    pub tables: HashMap<String, TableSchema>,
    pub relationships: Vec<Relationship>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableSchema {
    pub columns: HashMap<String, ColumnSchema>,
    pub indexes: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColumnSchema {
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub foreign_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
    pub from_table: String,
    pub to_table: String,
    pub on_delete: String,
    pub on_update: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BusinessLogicInfo {
    pub patterns: Vec<String>,
    pub algorithms: Vec<String>,
    pub workflows: Vec<String>,
    pub edge_cases: Vec<String>,
    pub error_handling: Vec<String>,
    pub rules: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfflinePlan {
    pub remote_data_access: Vec<String>,
    pub sync_feasibility: HashMap<String, String>,
    pub data_update_patterns: Vec<String>,
    pub conflict_resolution: Vec<String>,
    pub real_time_requirements: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnifiedOutput {
    pub files: Vec<FileMetadata>,
    pub routes: Vec<RouteInfo>,
    pub components: Vec<ComponentInfo>,
    pub api_map: Vec<ApiCallInfo>,
    pub templates: Vec<TemplateInfo>,
    pub auth: Option<AuthInfo>,
    pub database: Option<DatabaseSchema>,
    pub business_logic: Vec<BusinessLogicInfo>,
    pub offline_plan: OfflinePlan,
}