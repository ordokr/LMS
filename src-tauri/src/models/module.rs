use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub position: i32,
    pub items_count: i32,
    pub publish_final_grade: bool,
    pub published: bool,
    pub status: ModuleStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCreate {
    pub course_id: String,
    pub title: String,
    pub position: Option<i32>,
    pub publish_final_grade: Option<bool>,
    pub published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleUpdate {
    pub title: Option<String>,
    pub position: Option<i32>,
    pub publish_final_grade: Option<bool>,
    pub published: Option<bool>,
    pub status: Option<ModuleStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleStatus {
    Active,
    Locked,
    Completed,
    Unpublished,
}

impl ToString for ModuleStatus {
    fn to_string(&self) -> String {
        match self {
            ModuleStatus::Active => "active".to_string(),
            ModuleStatus::Locked => "locked".to_string(),
            ModuleStatus::Completed => "completed".to_string(),
            ModuleStatus::Unpublished => "unpublished".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: String,
    pub module_id: String,
    pub title: String,
    pub position: i32,
    pub item_type: ModuleItemType,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub url: Option<String>,
    pub page_url: Option<String>,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItemCreate {
    pub module_id: String,
    pub title: String,
    pub position: Option<i32>,
    pub item_type: ModuleItemType,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub url: Option<String>,
    pub page_url: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItemUpdate {
    pub title: Option<String>,
    pub position: Option<i32>,
    pub published: Option<bool>,
    pub url: Option<String>,
    pub page_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleItemType {
    File,
    Page,
    Discussion,
    Assignment,
    Quiz,
    SubHeader,
    ExternalUrl,
    ExternalTool,
}

impl ToString for ModuleItemType {
    fn to_string(&self) -> String {
        match self {
            ModuleItemType::File => "file".to_string(),
            ModuleItemType::Page => "page".to_string(),
            ModuleItemType::Discussion => "discussion".to_string(),
            ModuleItemType::Assignment => "assignment".to_string(),
            ModuleItemType::Quiz => "quiz".to_string(),
            ModuleItemType::SubHeader => "sub_header".to_string(),
            ModuleItemType::ExternalUrl => "external_url".to_string(),
            ModuleItemType::ExternalTool => "external_tool".to_string(),
        }
    }
}