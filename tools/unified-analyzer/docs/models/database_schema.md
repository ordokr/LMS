# Database Schema Documentation

This document provides a comprehensive overview of the database schema, including tables, relationships, and migrations.

## Tables

### data_update_patterns

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| pattern_type | String | NOT NULL |  |
| description | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |

### dependency_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| ruby_dependencies | HashMap<String | NOT NULL |  |
| js_dependencies | HashMap<String | NOT NULL |  |
| python_dependencies | HashMap<String | NOT NULL |  |
| system_dependencies | HashMap<String | NOT NULL |  |
| dependency_graph | DependencyGraph | NOT NULL |  |

### modules

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE |  |
| name | String | NOT NULL |  |
| course_id | String | NOT NULL, FOREIGN KEY |  |
| items | Vec<ModuleItem> | NOT NULL |  |

### naming_conflicts

Represents a naming or semantic conflict between entities

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| entity1 | String | NOT NULL | First entity |
| entity2 | String | NOT NULL | Second entity |
| conflict_type | String | NOT NULL | Conflict type (name, field, semantic) |
| description | String | NOT NULL | Conflict description |
| suggested_resolution | String | NOT NULL | Suggested resolution |

### topics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE |  |
| title | String | NOT NULL |  |
| author | String | NOT NULL |  |
| content | String | NOT NULL |  |
| tags | Vec<String> | NOT NULL |  |
| category | Option<String> |  |  |

### code_quality_metrics

Code quality metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| complexity | ComplexityMetrics | NOT NULL |  |
| tech_debt | TechDebtMetrics | NOT NULL |  |
| solid_violations | SolidViolations | NOT NULL |  |
| design_patterns | DesignPatternUsage | NOT NULL |  |
| metrics | HashMap<String | NOT NULL |  |

### database_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| tables | Vec<DatabaseTableInfo> | NOT NULL |  |
| relationships | Vec<RelationshipInfo> | NOT NULL |  |
| db_type | Option<String> |  |  |
| version | Option<String> |  |  |

### feature_mappings

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source_feature | String | NOT NULL |  |
| target_feature | String | NOT NULL |  |
| confidence | f32 | NOT NULL |  |
| status | String | NOT NULL |  |
| priority | u8 | NOT NULL |  |

### sync_system_infos

Sync system information

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| implementation_status | String | NOT NULL |  |
| offline_capability | bool | NOT NULL |  |
| conflict_resolution | String | NOT NULL |  |

### discourse_analysis

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| users | Vec<DiscourseUser> | NOT NULL |  |
| topics | Vec<DiscourseTopic> | NOT NULL |  |
| posts | Vec<DiscoursePost> | NOT NULL |  |

### relationship_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source_table | String | NOT NULL |  |
| target_table | String | NOT NULL |  |
| relationship_type | String | NOT NULL |  |
| source_column | String | NOT NULL |  |
| target_column | String | NOT NULL |  |

### ember_routes

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| path | String | NOT NULL |  |
| model | Option<String> |  |  |

### integration_conflicts

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| description | String | NOT NULL |  |
| affected_components | Vec<String> | NOT NULL |  |
| resolution_status | String | NOT NULL |  |

### test_metrics

Test metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| total | usize | NOT NULL |  |
| passing | usize | NOT NULL |  |
| coverage | f32 | NOT NULL |  |
| details | Vec<TestInfo> | NOT NULL |  |

### template_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| templates | HashMap<String | NOT NULL |  |

### integration_metrics

Integration metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| canvas_integrations | Vec<IntegrationPoint> | NOT NULL |  |
| discourse_integrations | Vec<IntegrationPoint> | NOT NULL |  |
| conflicts | Vec<IntegrationConflict> | NOT NULL |  |
| total_points | usize | NOT NULL |  |
| implemented_points | usize | NOT NULL |  |
| implementation_percentage | f32 | NOT NULL |  |

### business_rules

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| description | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |

### api_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| endpoints | Vec<ApiEndpointInfo> | NOT NULL |  |
| base_url | Option<String> |  |  |
| version | Option<String> |  |  |

### conflict_resolution_strategys

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| description | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |

### template_loops

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| iterator | String | NOT NULL |  |
| collection | String | NOT NULL |  |

### canvas_elements

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| module | String | NOT NULL |  |
| due_date | Option<String> |  |  |

### ember_services

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| methods | Vec<String> | NOT NULL |  |

### ruby_rails_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| models | HashMap<String | NOT NULL |  |
| controllers | HashMap<String | NOT NULL |  |
| routes | Vec<Route> | NOT NULL |  |
| callbacks | Vec<Callback> | NOT NULL |  |
| hooks | Vec<Hook> | NOT NULL |  |
| database_schemas | HashMap<String | NOT NULL |  |

### business_logic_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| patterns | Vec<BusinessLogicPattern> | NOT NULL |  |
| algorithms | Vec<DomainAlgorithm> | NOT NULL |  |
| workflows | Vec<Workflow> | NOT NULL |  |
| edge_cases | Vec<EdgeCase> | NOT NULL |  |
| business_rules | Vec<BusinessRule> | NOT NULL |  |

### business_logic_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| core_patterns | Vec<String> | NOT NULL |  |
| domain_algorithms | HashMap<String | NOT NULL |  |
| workflows | Vec<WorkflowInfo> | NOT NULL |  |
| edge_cases | Vec<String> | NOT NULL |  |
| business_rules | HashMap<String | NOT NULL |  |

### solid_violations

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| srp | Vec<CodeViolation> | NOT NULL |  |
| ocp | Vec<CodeViolation> | NOT NULL |  |
| lsp | Vec<CodeViolation> | NOT NULL |  |
| isp | Vec<CodeViolation> | NOT NULL |  |
| dip | Vec<CodeViolation> | NOT NULL |  |

### api_parameter_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| required | bool | NOT NULL |  |
| description | Option<String> |  |  |

### auth_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| authentication_methods | Vec<String> | NOT NULL |  |
| roles | HashMap<String | NOT NULL |  |
| csrf_protection | bool | NOT NULL |  |
| session_management | bool | NOT NULL |  |
| password_policies | HashMap<String | NOT NULL |  |
| sso_integrations | Vec<String> | NOT NULL |  |

### index_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| columns | Vec<String> | NOT NULL |  |
| unique | bool | NOT NULL, UNIQUE |  |

### analysis_configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| analyzers | Vec<String> | NOT NULL |  |
| exclude_patterns | Vec<String> | NOT NULL |  |
| include_patterns | Vec<String> | NOT NULL |  |
| max_file_size_mb | usize | NOT NULL |  |

### discourse_topics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| title | String | NOT NULL |  |
| author | String | NOT NULL |  |

### template_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| path | String | NOT NULL |  |
| template_type | String | NOT NULL |  |
| bindings | Vec<String> | NOT NULL |  |
| partials | Vec<String> | NOT NULL |  |
| loops | Vec<String> | NOT NULL |  |
| conditionals | Vec<String> | NOT NULL |  |

### offline_first_readiness_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| data_access_patterns | Vec<DataAccessPattern> | NOT NULL |  |
| data_update_patterns | Vec<DataUpdatePattern> | NOT NULL |  |
| conflict_resolution_strategies | Vec<ConflictResolutionStrategy> | NOT NULL |  |
| real_time_update_requirements | Vec<RealTimeUpdateRequirement> | NOT NULL |  |
| offline_readiness_score | u8 | NOT NULL |  |
| recommendations | Vec<String> | NOT NULL |  |

### posts

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE |  |
| topic_id | String | NOT NULL, FOREIGN KEY |  |
| author | String | NOT NULL |  |
| content | String | NOT NULL |  |
| created_at | String | NOT NULL |  |

### columns

Represents a database column with its properties

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| column_type | String | NOT NULL |  |
| nullable | bool | NOT NULL |  |
| default | Option<String> |  |  |
| data_type | String | NOT NULL |  |
| primary_key | bool | NOT NULL, UNIQUE |  |
| foreign_key | Option<ForeignKey> | UNIQUE |  |
| unique | bool | NOT NULL, UNIQUE |  |
| default_value | Option<String> |  |  |
| description | Option<String> |  |  |
| options | HashMap<String | NOT NULL |  |

### workflow_steps

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| description | String | NOT NULL |  |
| actor | Option<String> |  |  |
| triggers | Vec<String> | NOT NULL |  |

### helix_db_indexs

Represents an index in a HelixDB table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL | Index name |
| fields | Vec<String> | NOT NULL | Fields included in the index |
| unique | bool | NOT NULL, UNIQUE | Whether the index is unique |

### ember_models

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| attributes | Vec<String> | NOT NULL |  |
| relationships | Vec<String> | NOT NULL |  |

### common_entitys

Common entity between Canvas and Discourse

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| canvas_path | String | NOT NULL |  |
| discourse_path | String | NOT NULL |  |
| mapping_complexity | String | NOT NULL |  |

### routes

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| path | String | NOT NULL |  |
| http_method | Option<String> |  |  |
| controller | Option<String> |  |  |
| action | Option<String> |  |  |
| name | Option<String> |  |  |
| parameters | Vec<RouteParameter> | NOT NULL |  |
| authentication_required | bool | NOT NULL |  |
| source_file | String | NOT NULL |  |
| framework | String | NOT NULL |  |
| verb | String | NOT NULL |  |

### test_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| file_path | PathBuf | NOT NULL |  |
| status | TestStatus | NOT NULL |  |

### architecture_infos

Architecture information

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| frameworks | Vec<String> | NOT NULL |  |
| design_patterns | Vec<String> | NOT NULL |  |
| technologies | HashMap<String | NOT NULL |  |

### medium_priority_doc_configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| sync_architecture | bool | NOT NULL |  |
| database_architecture | bool | NOT NULL |  |

### ui_component_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| file_path | PathBuf | NOT NULL |  |
| completeness | f32 | NOT NULL |  |
| props | Vec<String> | NOT NULL |  |
| states | Vec<String> | NOT NULL |  |

### route_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| path | String | NOT NULL |  |
| method | String | NOT NULL |  |
| handler | String | NOT NULL |  |
| auth_required | bool | NOT NULL |  |
| params | Vec<String> | NOT NULL |  |
| source | String | NOT NULL |  |

### module_items

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE |  |
| title | String | NOT NULL |  |
| item_type | String | NOT NULL |  |
| content_id | Option<String> |  |  |

### template_conditionals

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| condition | String | NOT NULL |  |

### design_pattern_implementations

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| pattern | String | NOT NULL |  |
| file_path | PathBuf | NOT NULL |  |
| description | String | NOT NULL |  |

### component_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| file_path | String | NOT NULL |  |
| framework | String | NOT NULL |  |
| props | Vec<String> | NOT NULL |  |
| state | Option<Vec<String>> |  |  |
| lifecycle_hooks | Option<Vec<String>> |  |  |
| dependencies | Vec<String> | NOT NULL |  |

### tech_debt_metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| score | f32 | NOT NULL |  |
| items | Vec<TechDebtItem> | NOT NULL |  |

### rails_models

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| associations | Vec<String> | NOT NULL |  |
| validations | Vec<String> | NOT NULL |  |

### model_metrics

Model implementation metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| total | usize | NOT NULL |  |
| implemented | usize | NOT NULL |  |
| details | Vec<ModelInfo> | NOT NULL |  |
| implementation_percentage | f32 | NOT NULL |  |

### api_clients

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| endpoint | String | NOT NULL |  |
| method | String | NOT NULL |  |
| client_type | String | NOT NULL |  |
| source_file | String | NOT NULL |  |

### callbacks

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| model | String | NOT NULL |  |
| method | String | NOT NULL |  |

### ember_initializers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| initialize | String | NOT NULL |  |

### migration_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| version | String | NOT NULL |  |
| name | String | NOT NULL |  |
| operations | Vec<String> | NOT NULL |  |

### blockchain_infos

Blockchain information

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| storage_size_kb | usize | NOT NULL |  |
| batch_efficiency | f64 | NOT NULL |  |
| transaction_count | usize | NOT NULL |  |
| block_count | usize | NOT NULL |  |
| metrics | HashMap<String | NOT NULL |  |
| implementation_status | String | NOT NULL |  |
| features | Vec<String> | NOT NULL |  |

### features

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source | String | NOT NULL |  |
| name | String | NOT NULL |  |
| category | String | NOT NULL |  |
| source_files | Vec<String> | NOT NULL |  |
| related_entities | Vec<String> | NOT NULL |  |
| metadata | HashMap<String | NOT NULL |  |

### rails_controllers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| actions | Vec<String> | NOT NULL |  |

### design_pattern_usages

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| patterns_used | Vec<String> | NOT NULL |  |
| pattern_implementations | HashMap<String | NOT NULL |  |

### performance_configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| parallel_processing | bool | NOT NULL |  |
| enable_caching | bool | NOT NULL |  |
| incremental_analysis | bool | NOT NULL |  |
| cache_dir | String | NOT NULL |  |
| max_memory_mb | usize | NOT NULL |  |
| timeout_seconds | u64 | NOT NULL |  |

### auth_roles

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| permissions | Vec<String> | NOT NULL |  |
| source_file | String | NOT NULL |  |

### api_endpoint_metrics

API endpoint metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| total | usize | NOT NULL |  |
| implemented | usize | NOT NULL |  |
| details | Vec<ApiEndpointInfo> | NOT NULL |  |
| implementation_percentage | f32 | NOT NULL |  |

### code_metrics

Represents code metrics for a file

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| file_path | String | NOT NULL | File path |
| loc | usize | NOT NULL | Lines of code |
| complexity | u32 | NOT NULL | Cyclomatic complexity |
| comment_coverage | f32 | NOT NULL | Comment coverage (percentage) |
| cohesion | f32 | NOT NULL | Cohesion score (0.0 to 1.0) |
| usefulness_score | u8 | NOT NULL | Overall usefulness score (0 to 100) |
| recommendation | String | NOT NULL | Recommendation (reuse, partial, rebuild) |

### database_schemas

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| columns | Vec<Column> | NOT NULL |  |

### configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| general | GeneralConfig | NOT NULL |  |
| analysis | AnalysisConfig | NOT NULL |  |
| documentation | DocumentationConfig | NOT NULL |  |
| performance | PerformanceConfig | NOT NULL |  |
| paths | PathsConfig | NOT NULL |  |

### migration_phases

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| description | String | NOT NULL |  |
| components | Vec<String> | NOT NULL |  |
| apis | Vec<String> | NOT NULL |  |
| routes | Vec<String> | NOT NULL |  |
| database_tables | Vec<String> | NOT NULL |  |
| estimated_effort | String | NOT NULL |  |
| dependencies | Vec<String> | NOT NULL |  |

### component_metrics

UI component metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| total | usize | NOT NULL |  |
| implemented | usize | NOT NULL |  |
| details | Vec<UiComponentInfo> | NOT NULL |  |
| implementation_percentage | f32 | NOT NULL |  |

### tables

Represents a database table with its columns, indexes, and relationships

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| columns | Vec<Column> | NOT NULL |  |
| primary_key | Option<String> | UNIQUE |  |
| indexes | Vec<Index> | NOT NULL |  |
| foreign_keys | Vec<ForeignKey> | NOT NULL, UNIQUE |  |
| source_file | String | NOT NULL |  |
| description | Option<String> |  |  |

### ember_helpers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| functions | Vec<String> | NOT NULL |  |

### migration_roadmaps

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| project_name | String | NOT NULL |  |
| generated_date | String | NOT NULL |  |
| phases | Vec<MigrationPhase> | NOT NULL |  |
| total_estimated_effort | String | NOT NULL |  |
| critical_path_items | Vec<String> | NOT NULL |  |
| risks | Vec<String> | NOT NULL |  |
| recommendations | Vec<String> | NOT NULL |  |

### canvas_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| courses | HashMap<String | NOT NULL |  |
| assignments | HashMap<String | NOT NULL |  |
| modules | HashMap<String | NOT NULL |  |

### workflow_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| steps | Vec<String> | NOT NULL |  |
| actors | Vec<String> | NOT NULL |  |
| triggers | Vec<String> | NOT NULL |  |

### foreign_keys

Represents a foreign key relationship

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| from_column | String | NOT NULL |  |
| to_table | String | NOT NULL |  |
| to_column | String | NOT NULL |  |
| references_table | String | NOT NULL |  |
| references_column | String | NOT NULL |  |

### react_routes

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| path | String | NOT NULL |  |
| component | String | NOT NULL |  |
| exact | bool | NOT NULL |  |
| auth_required | bool | NOT NULL |  |
| file_path | String | NOT NULL |  |

### database_table_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| columns | Vec<ColumnInfo> | NOT NULL |  |
| indexes | Vec<IndexInfo> | NOT NULL |  |
| description | Option<String> |  |  |

### workflows

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| description | String | NOT NULL |  |
| steps | Vec<WorkflowStep> | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |

### react_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| components | HashMap<String | NOT NULL |  |
| hooks | HashMap<String | NOT NULL |  |
| routes | Vec<ReactRoute> | NOT NULL |  |
| redux_stores | HashMap<String | NOT NULL |  |

### helix_db_tables

Represents a database table in HelixDB

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL | Table name |
| fields | Vec<HelixDbField> | NOT NULL | Table fields |
| indexes | Vec<HelixDbIndex> | NOT NULL | Table indexes |
| relationships | Vec<HelixDbRelationship> | NOT NULL | Table relationships |
| source | String | NOT NULL | Source system (canvas, discourse, ordo) |

### indexs

Represents an index on a table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| columns | Vec<String> | NOT NULL |  |
| unique | bool | NOT NULL, UNIQUE |  |

### assignments

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE |  |
| name | String | NOT NULL |  |
| description | Option<String> |  |  |
| course_id | String | NOT NULL, FOREIGN KEY |  |
| due_date | Option<String> |  |  |
| points_possible | Option<f64> |  |  |

### project_status

Overall project status

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| phase | String | NOT NULL |  |
| completion_percentage | f32 | NOT NULL |  |
| last_active_area | String | NOT NULL |  |
| estimated_completion_date | Option<DateTime<Utc>> |  |  |

### ember_controllers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| actions | Vec<String> | NOT NULL |  |

### template_partials

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| path | String | NOT NULL |  |

### business_logic_patterns

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| description | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |
| code_snippets | Vec<String> | NOT NULL |  |

### file_metadatas

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| parent_directory | Option<PathBuf> |  |  |
| relative_path | PathBuf | NOT NULL |  |
| absolute_path | PathBuf | NOT NULL |  |
| file_type | String | NOT NULL |  |
| size | u64 | NOT NULL |  |
| modified_time | String | NOT NULL |  |
| content | Cow<'static | NOT NULL |  |
| dependencies | Vec<PathBuf> | NOT NULL |  |
| path | String | NOT NULL |  |
| directory | Option<String> |  |  |
| purpose | Option<String> |  |  |

### migration_paths

Migration path between LMS and forum entities

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source_entity | String | NOT NULL |  |
| target_entity | String | NOT NULL |  |
| complexity | String | NOT NULL |  |
| mapping_strategy | String | NOT NULL |  |
| entity_name | String | NOT NULL |  |

### general_configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| project_name | String | NOT NULL |  |
| output_dir | String | NOT NULL |  |
| log_level | String | NOT NULL |  |

### high_priority_doc_configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| central_reference_hub | bool | NOT NULL |  |
| api_documentation | bool | NOT NULL |  |
| implementation_details | bool | NOT NULL |  |
| testing_documentation | bool | NOT NULL |  |
| tech_debt_report | bool | NOT NULL |  |
| summary_report | bool | NOT NULL |  |

### helix_db_relationships

Represents a relationship between HelixDB tables

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| relationship_type | String | NOT NULL | Relationship type (one-to-one, one-to-many, many-to-many) |
| foreign_table | String | NOT NULL | Foreign table name |
| local_field | String | NOT NULL | Local field name |
| foreign_field | String | NOT NULL | Foreign field name |

### courses

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE |  |
| name | String | NOT NULL |  |
| description | Option<String> |  |  |
| modules | Vec<String> | NOT NULL |  |
| account_id | i64 | NOT NULL |  |
| created_at | DateTime<Utc> | NOT NULL |  |
| updated_at | DateTime<Utc> | NOT NULL |  |

### integration_stats

Integration statistics for reporting

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| overall_integration_percentage | f32 | NOT NULL | Overall integration percentage (0.0-1.0) |
| entity_integration_percentage | f32 | NOT NULL | Entity integration percentage (0.0-1.0) |
| feature_integration_percentage | f32 | NOT NULL | Feature integration percentage (0.0-1.0) |
| integration_by_category | HashMap<String | NOT NULL | Integration percentage by category |

### api_endpoints

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| path | String | NOT NULL |  |
| method | String | NOT NULL |  |
| controller | Option<String> |  |  |
| action | Option<String> |  |  |
| authentication_required | bool | NOT NULL |  |
| parameters | Vec<String> | NOT NULL |  |
| response_format | Option<String> |  |  |
| source_file | String | NOT NULL |  |
| description | Option<String> |  |  |
| rate_limited | bool | NOT NULL |  |
| required_permissions | Vec<String> | NOT NULL |  |
| request_body_params | Vec<String> | NOT NULL |  |
| response_fields | Vec<String> | NOT NULL |  |

### real_time_update_requirements

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| feature | String | NOT NULL |  |
| description | String | NOT NULL |  |
| criticality | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |

### data_access_patterns

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| pattern_type | String | NOT NULL |  |
| description | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |
| sync_feasibility | SyncFeasibility | NOT NULL |  |

### recommendations

Represents a development recommendation

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE | Recommendation ID |
| title | String | NOT NULL | Recommendation title |
| description | String | NOT NULL | Recommendation description |
| priority | u8 | NOT NULL | Priority (1-5, with 5 being highest) |
| effort | f32 | NOT NULL | Estimated effort (days) |
| related_entities | Vec<String> | NOT NULL | Related entities |
| related_features | Vec<String> | NOT NULL | Related features |
| steps | Vec<String> | NOT NULL | Implementation steps |
| area | String | NOT NULL |  |
| related_files | Vec<PathBuf> | NOT NULL |  |

### route_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| routes | Vec<Route> | NOT NULL |  |
| route_patterns | HashMap<String | NOT NULL |  |
| auth_protected_routes | Vec<String> | NOT NULL |  |

### tech_debt_items

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| file | String | NOT NULL |  |
| line | usize | NOT NULL |  |
| category | String | NOT NULL |  |
| description | String | NOT NULL |  |
| severity | TechDebtSeverity | NOT NULL |  |
| fix_suggestion | String | NOT NULL |  |
| file_path | Option<PathBuf> |  |  |

### entity_mappings

Represents a mapping between entities from different systems

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source_entity | String | NOT NULL | Source entity (e.g., "canvas.Course") |
| target_entity | String | NOT NULL | Target entity (e.g., "ordo.Course") |
| confidence | f32 | NOT NULL | Mapping confidence (0.0 to 1.0) |
| field_mappings | HashMap<String | NOT NULL | Field mappings (source_field -> target_field) |
| unmapped_source_fields | Vec<String> | NOT NULL | Source fields not mapped to target |
| unmapped_target_fields | Vec<String> | NOT NULL | Target fields not mapped from source |

### model_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| file_path | PathBuf | NOT NULL |  |
| completeness | f32 | NOT NULL |  |
| source_system | Option<String> |  |  |
| source_file | Option<String> |  |  |
| relationships | Vec<ModelRelationship> | NOT NULL |  |

### api_endpoint_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| path | String | NOT NULL |  |
| method | String | NOT NULL |  |
| handler | String | NOT NULL |  |
| file_path | PathBuf | NOT NULL |  |
| completeness | f32 | NOT NULL |  |
| feature_area | String | NOT NULL |  |
| http_method | String | NOT NULL |  |
| controller | Option<String> |  |  |
| action | Option<String> |  |  |
| description | Option<String> |  |  |
| request_params | Vec<String> | NOT NULL |  |
| response_format | Option<String> |  |  |
| auth_required | bool | NOT NULL |  |
| rate_limited | bool | NOT NULL |  |
| category | Option<String> |  |  |

### edge_cases

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| scenario | String | NOT NULL |  |
| handling | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |

### model_relationships

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| from | String | NOT NULL |  |
| to | String | NOT NULL |  |
| relationship_type | RelationshipType | NOT NULL |  |

### documentation_configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| generate_high_priority | bool | NOT NULL |  |
| generate_medium_priority | bool | NOT NULL |  |
| generate_low_priority | bool | NOT NULL |  |
| high_priority | HighPriorityDocConfig | NOT NULL |  |
| medium_priority | MediumPriorityDocConfig | NOT NULL |  |

### ember_components

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| properties | Vec<String> | NOT NULL |  |
| actions | Vec<String> | NOT NULL |  |

### offline_readiness_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| offline_capable | bool | NOT NULL |  |
| data_storage_mechanism | Option<String> |  |  |
| sync_mechanism | Option<String> |  |  |
| conflict_resolution_strategy | Option<String> |  |  |
| network_detection | bool | NOT NULL |  |
| offline_features | Vec<String> | NOT NULL |  |
| online_only_features | Vec<String> | NOT NULL |  |

### auth_flows

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| steps | Vec<String> | NOT NULL |  |
| source_file | String | NOT NULL |  |

### hooks

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| target | String | NOT NULL |  |
| method | String | NOT NULL |  |

### react_redux_stores

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| actions | Vec<String> | NOT NULL |  |
| reducers | Vec<String> | NOT NULL |  |
| selectors | Vec<String> | NOT NULL |  |
| file_path | String | NOT NULL |  |

### auth_flow_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| auth_methods | HashMap<String | NOT NULL |  |
| auth_roles | HashMap<String | NOT NULL |  |
| auth_flows | Vec<AuthFlow> | NOT NULL |  |
| protected_routes | HashSet<String> | NOT NULL |  |

### dependency_graphs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| nodes | Vec<String> | NOT NULL |  |
| edges | Vec<(String | NOT NULL |  |

### route_parameters

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| constraint | Option<String> |  |  |

### directory_metadatas

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| purpose | DirectoryPurpose | NOT NULL |  |

### react_hooks

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| file_path | String | NOT NULL |  |
| dependencies | Vec<String> | NOT NULL |  |
| return_values | Vec<String> | NOT NULL |  |

### template_bindings

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| binding_type | String | NOT NULL |  |
| source | String | NOT NULL |  |

### categorys

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | String | PRIMARY KEY, NOT NULL, UNIQUE |  |
| name | String | NOT NULL |  |
| description | Option<String> |  |  |
| parent_category_id | Option<String> |  |  |

### feature_area_metrics

Feature area metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| total | usize | NOT NULL |  |
| implemented | usize | NOT NULL |  |
| priority | String | NOT NULL |  |

### integration_points

Points of integration between systems

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| canvas_component | String | NOT NULL |  |
| discourse_component | String | NOT NULL |  |
| data_flow | String | NOT NULL |  |
| sync_pattern | String | NOT NULL |  |
| entity_name | String | NOT NULL |  |
| source_feature | String | NOT NULL |  |
| target_implementation | String | NOT NULL |  |
| status | String | NOT NULL |  |
| details | String | NOT NULL |  |

### unified_analysis_outputs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| files | Vec<FileMetadata> | NOT NULL |  |
| routes | Vec<RouteInfo> | NOT NULL |  |
| components | HashMap<String | NOT NULL |  |
| api | ApiInfo | NOT NULL |  |
| templates | Vec<TemplateInfo> | NOT NULL |  |
| auth | AuthInfo | NOT NULL |  |
| database | DatabaseInfo | NOT NULL |  |
| business_logic | BusinessLogicInfo | NOT NULL |  |
| offline_readiness | OfflineReadinessInfo | NOT NULL |  |
| file_dependencies | HashMap<String | NOT NULL |  |

### analysis_caches

Cache for storing analysis results

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|

### paths_configs

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| canvas_path | Option<String> |  |  |
| discourse_path | Option<String> |  |  |
| lms_path | Option<String> |  |  |

### auth_methods

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| method_type | String | NOT NULL |  |
| source_file | String | NOT NULL |  |
| description | String | NOT NULL |  |

### discourse_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| topics | HashMap<String | NOT NULL |  |
| posts | HashMap<String | NOT NULL |  |
| categories | HashMap<String | NOT NULL |  |

### helix_db_table_mappings

Represents a mapping between tables from different systems

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source_table | String | NOT NULL | Source table |
| source_system | String | NOT NULL | Source system |
| target_table | String | NOT NULL | Target table |
| target_system | String | NOT NULL | Target system |
| field_mappings | Vec<HelixDbFieldMapping> | NOT NULL | Field mappings |
| confidence | u8 | NOT NULL | Mapping confidence (0-100) |

### helix_db_field_mappings

Represents a mapping between fields from different tables

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source_field | String | NOT NULL | Source field |
| target_field | String | NOT NULL | Target field |
| confidence | u8 | NOT NULL | Mapping confidence (0-100) |

### api_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| endpoints | HashMap<String | NOT NULL |  |
| clients | Vec<ApiClient> | NOT NULL |  |
| route_patterns | HashMap<String | NOT NULL |  |
| auth_protected_routes | Vec<String> | NOT NULL |  |

### integration_progress

Represents integration progress for entities and features

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| entity_progress | HashMap<String | NOT NULL | Entity integration progress |
| feature_progress | HashMap<String | NOT NULL | Feature integration progress |
| overall_progress | f32 | NOT NULL | Overall integration progress |
| category_progress | HashMap<String | NOT NULL | Integration status by category |

### migrations

Represents a database migration

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| version | String | NOT NULL |  |
| name | String | NOT NULL |  |
| operations | Vec<String> | NOT NULL |  |
| file_path | String | NOT NULL |  |

### template_analysis

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| path | String | NOT NULL |  |
| bindings | Vec<TemplateBinding> | NOT NULL |  |
| partials | Vec<TemplatePartial> | NOT NULL |  |
| loops | Vec<TemplateLoop> | NOT NULL |  |
| conditionals | Vec<TemplateConditional> | NOT NULL |  |

### conflicts

Represents a conflict between entities or features

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| conflict_type | ConflictType | NOT NULL | Conflict type |
| source | String | NOT NULL | Source entity or feature |
| target | String | NOT NULL | Target entity or feature |
| description | String | NOT NULL | Conflict description |
| suggested_resolution | String | NOT NULL | Suggested resolution |
| severity | u8 | NOT NULL | Severity (1-5, with 5 being highest) |

### ember_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| models | HashMap<String | NOT NULL |  |
| controllers | HashMap<String | NOT NULL |  |
| components | HashMap<String | NOT NULL |  |
| routes | HashMap<String | NOT NULL |  |
| services | HashMap<String | NOT NULL |  |
| helpers | HashMap<String | NOT NULL |  |
| initializers | HashMap<String | NOT NULL |  |

### database_schema_analyzers

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| tables | HashMap<String | NOT NULL |  |
| migrations | Vec<Migration> | NOT NULL |  |

### complexity_metrics

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| average | f32 | NOT NULL |  |
| high | usize | NOT NULL |  |
| file_details | HashMap<PathBuf | NOT NULL |  |

### analysis_results

Analysis result for the entire codebase

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| timestamp | DateTime<Utc> | NOT NULL | When the analysis was performed |
| project_status | ProjectStatus | NOT NULL | Overall project metrics |
| models | ModelMetrics | NOT NULL | Model implementations |
| api_endpoints | ApiEndpointMetrics | NOT NULL | API endpoints |
| ui_components | ComponentMetrics | NOT NULL | UI components |
| code_quality | CodeQualityMetrics | NOT NULL | Code quality metrics |
| tests | TestMetrics | NOT NULL | Test coverage |
| integration | IntegrationMetrics | NOT NULL | Integration points |
| architecture | ArchitectureInfo | NOT NULL | Detected architecture |
| sync_system | SyncSystemInfo | NOT NULL | Synchronization system |
| blockchain | BlockchainInfo | NOT NULL | Blockchain implementation |
| feature_areas | HashMap<String | NOT NULL | Feature area implementation percentages |
| recommendations | Vec<Recommendation> | NOT NULL | Next step recommendations |

### react_components

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| file_path | String | NOT NULL |  |
| props | Vec<String> | NOT NULL |  |
| state_variables | Vec<String> | NOT NULL |  |
| hooks | Vec<String> | NOT NULL |  |
| lifecycle_methods | Vec<String> | NOT NULL |  |
| jsx_elements | Vec<String> | NOT NULL |  |

### helix_db_fields

Represents a field in a HelixDB table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL | Field name |
| field_type | String | NOT NULL | Field type |
| nullable | bool | NOT NULL | Whether the field is nullable |
| default | Option<String> |  | Default value |
| primary_key | bool | NOT NULL, UNIQUE | Whether the field is a primary key |
| unique | bool | NOT NULL, UNIQUE | Whether the field is unique |

### discourse_users

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| username | String | NOT NULL |  |

### normalized_entitys

Represents a normalized entity extracted from source code

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| source | String | NOT NULL | Source system (canvas, discourse, ordo) |
| entity | String | NOT NULL | Entity name |
| category | String | NOT NULL | Entity category |
| fields | HashMap<String | NOT NULL | Fields and their types |
| source_file | String | NOT NULL | Source file path |
| metadata | HashMap<String | NOT NULL | Additional metadata |

### integrated_migration_results

Integrated migration analysis result

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| canvas_models | Vec<String> | NOT NULL |  |
| discourse_models | Vec<String> | NOT NULL |  |
| common_entities | std::collections::HashMap<String | NOT NULL |  |
| migration_paths | Vec<MigrationPath> | NOT NULL |  |
| integration_points | Vec<IntegrationPoint> | NOT NULL |  |

### dependencys

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| version | String | NOT NULL |  |
| dependency_type | String | NOT NULL |  |
| source_file | String | NOT NULL |  |

### code_violations

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| description | String | NOT NULL |  |
| file_path | PathBuf | NOT NULL |  |
| line_number | Option<usize> |  |  |

### column_infos

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| data_type | String | NOT NULL |  |
| nullable | bool | NOT NULL |  |
| primary_key | bool | NOT NULL, UNIQUE |  |
| foreign_key | bool | NOT NULL, UNIQUE |  |
| references | Option<String> |  |  |
| default_value | Option<String> |  |  |
| description | Option<String> |  |  |

### domain_algorithms

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| name | String | NOT NULL |  |
| description | String | NOT NULL |  |
| files | Vec<String> | NOT NULL |  |
| complexity | String | NOT NULL |  |

### improved_db_schema_analyzers

Improved database schema analyzer that extracts schema from multiple sources

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| tables | HashMap<String | NOT NULL |  |
| migrations | Vec<Migration> | NOT NULL |  |

### canvas_analysis

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| modules | Vec<CanvasElement> | NOT NULL |  |
| assignments | Vec<CanvasElement> | NOT NULL |  |
| quizzes | Vec<CanvasElement> | NOT NULL |  |
| pages | Vec<CanvasElement> | NOT NULL |  |

### discourse_posts

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| content | String | NOT NULL |  |
| topic | String | NOT NULL |  |
| author | String | NOT NULL |  |

## Relationships

| Parent Table | Child Table | Relationship Type | Foreign Key |
|-------------|------------|-------------------|-------------|
| courses | modules | one-to-many | course_id |
| topics | posts | one-to-many | topic_id |
| courses | assignments | one-to-many | course_id |

