use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Module model - ported from Canvas
/// Represents a course module that contains items like assignments, quizzes, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    // Core fields
    pub id: Option<Uuid>,
    pub course_id: Uuid,
    pub name: String,
    pub position: i32,
    pub unlock_at: Option<DateTime<Utc>>,
    pub require_sequential_progress: bool,
    pub prerequisite_module_ids: Vec<Uuid>,
    pub items: Vec<ModuleItem>,
    pub published: bool,
    pub status: ModuleStatus,

    // Canvas-specific fields
    pub canvas_id: Option<String>,
    pub canvas_course_id: Option<String>,
    pub workflow_state: Option<String>,

    // Common fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Module item model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: Option<Uuid>,
    pub module_id: Option<Uuid>,
    pub title: String,
    pub position: i32,
    pub indent: i32,
    pub item_type: ModuleItemType,
    pub content_id: Option<Uuid>,
    pub content_type: Option<String>,
    pub url: Option<String>,
    pub page_url: Option<String>,
    pub external_url: Option<String>,
    pub new_tab: bool,
    pub completion_requirement: Option<CompletionRequirement>,
    pub published: bool,
    pub canvas_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Module status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleStatus {
    Locked,
    Unlocked,
    Started,
    Completed,
    InProgress,
}

/// Module item type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleItemType {
    Assignment,
    Quiz,
    File,
    Page,
    Discussion,
    ExternalUrl,
    ExternalTool,
    SubHeader,
    Text,
    Other(String),
}

/// Completion requirement model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequirement {
    pub requirement_type: RequirementType,
    pub min_score: Option<f64>,
    pub completed: bool,
}

/// Requirement type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequirementType {
    MustView,
    MustSubmit,
    MustContribute,
    MinScore,
    MustMarkDone,
}

impl Module {
    /// Create a new module
    pub fn new(course_id: Uuid, name: String, position: i32) -> Self {
        let now = Utc::now();
        Self {
            id: Some(Uuid::new_v4()),
            course_id,
            name,
            position,
            unlock_at: None,
            require_sequential_progress: false,
            prerequisite_module_ids: Vec::new(),
            items: Vec::new(),
            published: false,
            status: ModuleStatus::Locked,
            canvas_id: None,
            canvas_course_id: None,
            workflow_state: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a module from Canvas data
    pub fn from_canvas(
        course_id: Uuid,
        name: String,
        position: i32,
        canvas_id: String,
        canvas_course_id: String,
        workflow_state: Option<String>,
        unlock_at: Option<DateTime<Utc>>,
        require_sequential_progress: bool,
        published: bool,
    ) -> Self {
        let mut module = Self::new(course_id, name, position);
        module.canvas_id = Some(canvas_id);
        module.canvas_course_id = Some(canvas_course_id);
        module.workflow_state = workflow_state;
        module.unlock_at = unlock_at;
        module.require_sequential_progress = require_sequential_progress;
        module.published = published;

        // Set status based on workflow state
        if let Some(state) = &module.workflow_state {
            module.status = match state.as_str() {
                "active" => ModuleStatus::Unlocked,
                "locked" => ModuleStatus::Locked,
                "started" => ModuleStatus::Started,
                "completed" => ModuleStatus::Completed,
                _ => ModuleStatus::Locked,
            };
        }

        module
    }

    /// Add an item to the module
    pub fn add_item(
        &mut self,
        title: String,
        position: i32,
        item_type: ModuleItemType,
        content_id: Option<Uuid>,
        content_type: Option<String>,
        url: Option<String>,
    ) -> Uuid {
        let now = Utc::now();
        let item_id = Uuid::new_v4();

        self.items.push(ModuleItem {
            id: Some(item_id),
            module_id: self.id,
            title,
            position,
            indent: 0,
            item_type,
            content_id,
            content_type,
            url,
            page_url: None,
            external_url: None,
            new_tab: false,
            completion_requirement: None,
            published: false,
            canvas_id: None,
            created_at: now,
            updated_at: now,
        });

        self.updated_at = now;
        item_id
    }

    /// Add a completion requirement to a module item
    pub fn add_completion_requirement(
        &mut self,
        item_id: Uuid,
        requirement_type: RequirementType,
        min_score: Option<f64>,
    ) -> bool {
        let now = Utc::now();
        let mut found = false;

        for item in &mut self.items {
            if let Some(id) = item.id {
                if id == item_id {
                    item.completion_requirement = Some(CompletionRequirement {
                        requirement_type,
                        min_score,
                        completed: false,
                    });
                    item.updated_at = now;
                    found = true;
                    break;
                }
            }
        }

        if found {
            self.updated_at = now;
        }

        found
    }

    /// Add a prerequisite module
    pub fn add_prerequisite(&mut self, prerequisite_module_id: Uuid) {
        if !self.prerequisite_module_ids.contains(&prerequisite_module_id) {
            self.prerequisite_module_ids.push(prerequisite_module_id);
            self.updated_at = Utc::now();
        }
    }

    /// Publish the module
    pub fn publish(&mut self) {
        let now = Utc::now();
        self.published = true;
        self.workflow_state = Some("active".to_string());
        self.status = ModuleStatus::Unlocked;
        self.updated_at = now;
    }

    /// Unpublish the module
    pub fn unpublish(&mut self) {
        let now = Utc::now();
        self.published = false;
        self.workflow_state = Some("unpublished".to_string());
        self.updated_at = now;
    }

    /// Lock the module
    pub fn lock(&mut self) {
        let now = Utc::now();
        self.workflow_state = Some("locked".to_string());
        self.status = ModuleStatus::Locked;
        self.updated_at = now;
    }

    /// Unlock the module
    pub fn unlock(&mut self) {
        let now = Utc::now();
        self.workflow_state = Some("active".to_string());
        self.status = ModuleStatus::Unlocked;
        self.updated_at = now;
    }

    /// Mark the module as completed
    pub fn mark_completed(&mut self) {
        let now = Utc::now();
        self.workflow_state = Some("completed".to_string());
        self.status = ModuleStatus::Completed;
        self.updated_at = now;
    }

    /// Mark the module as in progress
    pub fn mark_in_progress(&mut self) {
        let now = Utc::now();
        self.workflow_state = Some("started".to_string());
        self.status = ModuleStatus::InProgress;
        self.updated_at = now;
    }
}
