use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForumConfig {
    #[serde(default = "default_categories")]
    pub categories: Hierarchy<Category>,
    pub trust_levels: TrustSystem,
    #[serde(default)]
    pub plugins: Vec<PluginConfig>,
}

impl Default for ForumConfig {
    fn default() -> Self {
        Self {
            categories: default_categories(),
            trust_levels: TrustSystem::new(),
            plugins: vec![],
        }
    }
}

pub fn default_categories() -> Hierarchy<Category> {
    let mut hierarchy = Hierarchy::new();
    hierarchy.push_root(Category::new("Uncategorized"));
    hierarchy
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrustSystem {}

impl TrustSystem {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginConfig {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hierarchy<T> {
    // TODO: Implement hierarchy
}

impl<T> Hierarchy<T> {
    pub fn new() -> Self {
        Self {}
    }

    pub fn push_root(&mut self, _root: T) {
        // TODO: Implement push_root
    }
}