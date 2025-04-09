use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl std::fmt::Display for ModuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleStatus::Active => write!(f, "active"),
            ModuleStatus::Locked => write!(f, "locked"),
            ModuleStatus::Completed => write!(f, "completed"),
            ModuleStatus::Unpublished => write!(f, "unpublished"),
        }
    }
}

impl std::fmt::Display for ModuleItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleItemType::File => write!(f, "file"),
            ModuleItemType::Page => write!(f, "page"),
            ModuleItemType::Discussion => write!(f, "discussion"),
            ModuleItemType::Assignment => write!(f, "assignment"),
            ModuleItemType::Quiz => write!(f, "quiz"),
            ModuleItemType::SubHeader => write!(f, "subheader"),
            ModuleItemType::ExternalUrl => write!(f, "external_url"),
            ModuleItemType::ExternalTool => write!(f, "external_tool"),
        }
    }
}

// String conversion for types - useful for database operations
impl std::str::FromStr for ModuleStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(ModuleStatus::Active),
            "locked" => Ok(ModuleStatus::Locked),
            "completed" => Ok(ModuleStatus::Completed),
            "unpublished" => Ok(ModuleStatus::Unpublished),
            _ => Err(format!("Invalid module status: {}", s)),
        }
    }
}

impl std::str::FromStr for ModuleItemType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(ModuleItemType::File),
            "page" => Ok(ModuleItemType::Page),
            "discussion" => Ok(ModuleItemType::Discussion),
            "assignment" => Ok(ModuleItemType::Assignment),
            "quiz" => Ok(ModuleItemType::Quiz),
            "subheader" => Ok(ModuleItemType::SubHeader),
            "external_url" => Ok(ModuleItemType::ExternalUrl),
            "external_tool" => Ok(ModuleItemType::ExternalTool),
            _ => Err(format!("Invalid module item type: {}", s)),
        }
    }
}