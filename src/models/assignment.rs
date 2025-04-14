use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub due_date: chrono::NaiveDateTime,
}

impl Assignment {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Assignment name cannot be empty.".to_string());
        }
        if self.description.is_empty() {
            return Err("Assignment description cannot be empty.".to_string());
        }
        if self.due_date < chrono::Local::now().naive_local() {
            return Err("Due date must be in the future.".to_string());
        }
        Ok(())
    }
}
