use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: Option<i64>,
    pub course_id: i64,
    pub name: String,
    pub position: Option<i32>,
    pub unlock_at: Option<String>,
    pub require_sequential_progress: bool,
    pub published: bool,
    pub items_count: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: Option<i64>,
    pub module_id: i64,
    pub title: String,
    pub position: Option<i32>,
    pub indent: i32,
    pub item_type: ModuleItemType,
    pub content_id: Option<i64>,
    pub page_url: Option<String>,
    pub external_url: Option<String>,
    pub completion_requirement: Option<CompletionRequirement>,
    pub published: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModuleItemType {
    Assignment,
    Quiz,
    File,
    Page,
    Discussion,
    ExternalUrl,
    ExternalTool,
    Header,
    SubHeader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequirement {
    pub requirement_type: CompletionRequirementType,
    pub min_score: Option<f32>,
    pub completed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompletionRequirementType {
    MustView,
    MustSubmit,
    MustContribute,
    MinScore,
    MarkDone,
}