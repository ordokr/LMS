use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReactComponent {
    pub name: String,
    pub file_path: String,
    pub props: Vec<String>,
    pub state_variables: Vec<String>,
    pub hooks: Vec<String>,
    pub lifecycle_methods: Vec<String>,
    pub jsx_elements: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReactHook {
    pub name: String,
    pub file_path: String,
    pub dependencies: Vec<String>,
    pub return_values: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReactRoute {
    pub path: String,
    pub component: String,
    pub exact: bool,
    pub auth_required: bool,
    pub file_path: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReactReduxStore {
    pub name: String,
    pub actions: Vec<String>,
    pub reducers: Vec<String>,
    pub selectors: Vec<String>,
    pub file_path: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReactAnalyzer {
    pub components: HashMap<String, ReactComponent>,
    pub hooks: HashMap<String, ReactHook>,
    pub routes: Vec<ReactRoute>,
    pub redux_stores: HashMap<String, ReactReduxStore>,
}

impl ReactAnalyzer {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            hooks: HashMap::new(),
            routes: Vec::new(),
            redux_stores: HashMap::new(),
        }
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = ReactAnalyzer::new();

        // Look for React files in the codebase
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "jsx" || ext == "tsx" || ext == "js" || ext == "ts" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Skip node_modules and other library directories
                                if path.to_string_lossy().contains("node_modules") ||
                                   path.to_string_lossy().contains("vendor") ||
                                   path.to_string_lossy().contains("dist") {
                                    continue;
                                }

                                // Only analyze React files
                                if content.contains("React") ||
                                   content.contains("react") ||
                                   content.contains("useState") ||
                                   content.contains("useEffect") ||
                                   content.contains("Component") {

                                    // Extract component information
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        // Extract components
                                        analyzer.extract_components(&content, &file_path);

                                        // Extract hooks
                                        analyzer.extract_hooks(&content, &file_path);

                                        // Extract routes
                                        analyzer.extract_routes(&content, &file_path);

                                        // Extract Redux stores
                                        analyzer.extract_redux_stores(&content, &file_path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize ReactAnalyzer: {}", e)),
        }
    }

    fn extract_components(&mut self, content: &str, file_path: &str) {
        lazy_static! {
            // Match functional components
            static ref FUNC_COMPONENT_REGEX: Regex = Regex::new(
                r"(?:export\s+)?(?:const|function)\s+([A-Z][\w]*)\s*(?:=\s*)?(?:\([^)]*\)|[^=]*=>)"
            ).unwrap();

            // Match class components
            static ref CLASS_COMPONENT_REGEX: Regex = Regex::new(
                r"class\s+([A-Z][\w]*)\s+extends\s+(?:React\.)?Component"
            ).unwrap();

            // Match props destructuring
            static ref PROPS_REGEX: Regex = Regex::new(
                r"(?:const|let)\s+\{([^}]*)\}\s*=\s*props"
            ).unwrap();

            // Match useState hooks
            static ref USE_STATE_REGEX: Regex = Regex::new(
                r"const\s+\[([\w]+),\s*set([\w]+)\]\s*=\s*useState"
            ).unwrap();

            // Match useEffect hooks
            static ref USE_EFFECT_REGEX: Regex = Regex::new(
                r"useEffect\(\s*\(\)\s*=>\s*\{[^}]*\},\s*\[([^\]]*)\]\s*\)"
            ).unwrap();

            // Match JSX elements
            static ref JSX_ELEMENT_REGEX: Regex = Regex::new(
                r"<([A-Z][\w]*)\s*[^>]*>"
            ).unwrap();

            // Match lifecycle methods in class components
            static ref LIFECYCLE_METHODS_REGEX: Regex = Regex::new(
                r"(componentDidMount|componentDidUpdate|componentWillUnmount|shouldComponentUpdate|getSnapshotBeforeUpdate|componentDidCatch)\s*\("
            ).unwrap();
        }

        // Extract functional components
        for cap in FUNC_COMPONENT_REGEX.captures_iter(content) {
            if let Some(component_name) = cap.get(1) {
                let name = component_name.as_str().to_string();

                let mut component = ReactComponent {
                    name: name.clone(),
                    file_path: file_path.to_string(),
                    props: Vec::new(),
                    state_variables: Vec::new(),
                    hooks: Vec::new(),
                    lifecycle_methods: Vec::new(),
                    jsx_elements: Vec::new(),
                };

                // Extract props
                for props_cap in PROPS_REGEX.captures_iter(content) {
                    if let Some(props_list) = props_cap.get(1) {
                        for prop in props_list.as_str().split(',') {
                            let prop_name = prop.trim();
                            if !prop_name.is_empty() {
                                component.props.push(prop_name.to_string());
                            }
                        }
                    }
                }

                // Extract state variables from useState
                for state_cap in USE_STATE_REGEX.captures_iter(content) {
                    if let Some(state_var) = state_cap.get(1) {
                        component.state_variables.push(state_var.as_str().to_string());
                    }
                }

                // Extract hooks
                if content.contains("useState") {
                    component.hooks.push("useState".to_string());
                }
                if content.contains("useEffect") {
                    component.hooks.push("useEffect".to_string());
                }
                if content.contains("useContext") {
                    component.hooks.push("useContext".to_string());
                }
                if content.contains("useReducer") {
                    component.hooks.push("useReducer".to_string());
                }
                if content.contains("useCallback") {
                    component.hooks.push("useCallback".to_string());
                }
                if content.contains("useMemo") {
                    component.hooks.push("useMemo".to_string());
                }
                if content.contains("useRef") {
                    component.hooks.push("useRef".to_string());
                }

                // Extract JSX elements
                for jsx_cap in JSX_ELEMENT_REGEX.captures_iter(content) {
                    if let Some(element) = jsx_cap.get(1) {
                        let element_name = element.as_str();
                        // Only include components (starting with uppercase)
                        if element_name.chars().next().map_or(false, |c| c.is_uppercase()) {
                            component.jsx_elements.push(element_name.to_string());
                        }
                    }
                }

                self.components.insert(name, component);
            }
        }

        // Extract class components
        for cap in CLASS_COMPONENT_REGEX.captures_iter(content) {
            if let Some(component_name) = cap.get(1) {
                let name = component_name.as_str().to_string();

                let mut component = ReactComponent {
                    name: name.clone(),
                    file_path: file_path.to_string(),
                    props: Vec::new(),
                    state_variables: Vec::new(),
                    hooks: Vec::new(),
                    lifecycle_methods: Vec::new(),
                    jsx_elements: Vec::new(),
                };

                // Extract lifecycle methods
                for method_cap in LIFECYCLE_METHODS_REGEX.captures_iter(content) {
                    if let Some(method) = method_cap.get(1) {
                        component.lifecycle_methods.push(method.as_str().to_string());
                    }
                }

                // Extract state variables from this.state
                let state_regex = Regex::new(r"this\.state\s*=\s*\{([^}]*)\}").unwrap();
                for state_cap in state_regex.captures_iter(content) {
                    if let Some(state_vars) = state_cap.get(1) {
                        for state_var in state_vars.as_str().split(',') {
                            if let Some(var_name) = state_var.split(':').next() {
                                let var_name = var_name.trim();
                                if !var_name.is_empty() {
                                    component.state_variables.push(var_name.to_string());
                                }
                            }
                        }
                    }
                }

                // Extract JSX elements
                for jsx_cap in JSX_ELEMENT_REGEX.captures_iter(content) {
                    if let Some(element) = jsx_cap.get(1) {
                        let element_name = element.as_str();
                        // Only include components (starting with uppercase)
                        if element_name.chars().next().map_or(false, |c| c.is_uppercase()) {
                            component.jsx_elements.push(element_name.to_string());
                        }
                    }
                }

                self.components.insert(name, component);
            }
        }
    }

    fn extract_hooks(&mut self, content: &str, file_path: &str) {
        lazy_static! {
            // Match custom hooks
            static ref CUSTOM_HOOK_REGEX: Regex = Regex::new(
                r"(?:export\s+)?(?:const|function)\s+(use[A-Z][\w]*)\s*(?:=\s*)?(?:\([^)]*\)|[^=]*=>)"
            ).unwrap();

            // Match hook dependencies
            static ref HOOK_DEPS_REGEX: Regex = Regex::new(
                r"useEffect\(\s*\(\)\s*=>\s*\{[^}]*\},\s*\[([^\]]*)\]\s*\)"
            ).unwrap();

            // Match return values
            static ref RETURN_REGEX: Regex = Regex::new(
                r"return\s+\{([^}]*)\}"
            ).unwrap();
        }

        // Extract custom hooks
        for cap in CUSTOM_HOOK_REGEX.captures_iter(content) {
            if let Some(hook_name) = cap.get(1) {
                let name = hook_name.as_str().to_string();

                let mut hook = ReactHook {
                    name: name.clone(),
                    file_path: file_path.to_string(),
                    dependencies: Vec::new(),
                    return_values: Vec::new(),
                };

                // Extract dependencies
                for deps_cap in HOOK_DEPS_REGEX.captures_iter(content) {
                    if let Some(deps_list) = deps_cap.get(1) {
                        for dep in deps_list.as_str().split(',') {
                            let dep_name = dep.trim();
                            if !dep_name.is_empty() {
                                hook.dependencies.push(dep_name.to_string());
                            }
                        }
                    }
                }

                // Extract return values
                for return_cap in RETURN_REGEX.captures_iter(content) {
                    if let Some(return_list) = return_cap.get(1) {
                        for ret in return_list.as_str().split(',') {
                            if let Some(ret_name) = ret.split(':').next() {
                                let ret_name = ret_name.trim();
                                if !ret_name.is_empty() {
                                    hook.return_values.push(ret_name.to_string());
                                }
                            }
                        }
                    }
                }

                self.hooks.insert(name, hook);
            }
        }
    }

    fn extract_routes(&mut self, content: &str, file_path: &str) {
        lazy_static! {
            // Match React Router routes
            static ref ROUTE_REGEX: Regex = Regex::new(
                r#"<Route\s+(?:[^>]*\s+)?path=["']([^"']+)["'](?:[^>]*\s+)?component=\{([^}]+)\}"#
            ).unwrap();

            // Match exact prop
            static ref EXACT_REGEX: Regex = Regex::new(
                r#"<Route\s+(?:[^>]*\s+)?exact(?:\s+|=\{true\}|[^>]*)"#
            ).unwrap();

            // Match private/protected routes
            static ref PROTECTED_ROUTE_REGEX: Regex = Regex::new(
                r#"<(?:Private|Protected|Auth)Route"#
            ).unwrap();
        }

        // Extract routes
        for cap in ROUTE_REGEX.captures_iter(content) {
            if let (Some(path), Some(component)) = (cap.get(1), cap.get(2)) {
                let path_str = path.as_str().to_string();
                let component_str = component.as_str().trim().to_string();

                // Check if route is exact
                let exact = EXACT_REGEX.is_match(cap.get(0).unwrap().as_str());

                // Check if route is protected
                let auth_required = PROTECTED_ROUTE_REGEX.is_match(content) ||
                                   content.contains("isAuthenticated") ||
                                   content.contains("requireAuth");

                self.routes.push(ReactRoute {
                    path: path_str,
                    component: component_str,
                    exact,
                    auth_required,
                    file_path: file_path.to_string(),
                });
            }
        }
    }

    fn extract_redux_stores(&mut self, content: &str, file_path: &str) {
        lazy_static! {
            // Match Redux action types
            static ref ACTION_TYPE_REGEX: Regex = Regex::new(
                r#"(?:export\s+)?const\s+([A-Z_]+)\s*=\s*["'](\w+)["']\s*;"#
            ).unwrap();

            // Match Redux action creators
            static ref ACTION_CREATOR_REGEX: Regex = Regex::new(
                r#"(?:export\s+)?(?:const|function)\s+(\w+Action)\s*=\s*\([^)]*\)\s*=>"#
            ).unwrap();

            // Match Redux reducers
            static ref REDUCER_REGEX: Regex = Regex::new(
                r#"(?:export\s+)?(?:const|function)\s+(\w+Reducer)\s*=\s*\(state\s*=\s*initialState,\s*action\)\s*=>"#
            ).unwrap();

            // Match Redux selectors
            static ref SELECTOR_REGEX: Regex = Regex::new(
                r#"(?:export\s+)?(?:const|function)\s+(\w+Selector|select\w+)\s*=\s*\(state\)\s*=>"#
            ).unwrap();
        }

        // Check if this is a Redux file
        if content.contains("createStore") ||
           content.contains("combineReducers") ||
           content.contains("useDispatch") ||
           content.contains("useSelector") ||
           content.contains("mapStateToProps") {

            // Try to determine store name from file path
            let file_name = PathBuf::from(file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            let store_name = if file_name.ends_with("Reducer") {
                file_name.replace("Reducer", "Store")
            } else if file_name.ends_with("Slice") {
                file_name.replace("Slice", "Store")
            } else {
                format!("{}{}", file_name, "Store")
            };

            let mut redux_store = ReactReduxStore {
                name: store_name.clone(),
                file_path: file_path.to_string(),
                actions: Vec::new(),
                reducers: Vec::new(),
                selectors: Vec::new(),
            };

            // Extract action types and creators
            for cap in ACTION_TYPE_REGEX.captures_iter(content) {
                if let Some(action_type) = cap.get(1) {
                    redux_store.actions.push(action_type.as_str().to_string());
                }
            }

            for cap in ACTION_CREATOR_REGEX.captures_iter(content) {
                if let Some(action_creator) = cap.get(1) {
                    redux_store.actions.push(action_creator.as_str().to_string());
                }
            }

            // Extract reducers
            for cap in REDUCER_REGEX.captures_iter(content) {
                if let Some(reducer) = cap.get(1) {
                    redux_store.reducers.push(reducer.as_str().to_string());
                }
            }

            // Extract selectors
            for cap in SELECTOR_REGEX.captures_iter(content) {
                if let Some(selector) = cap.get(1) {
                    redux_store.selectors.push(selector.as_str().to_string());
                }
            }

            if !redux_store.actions.is_empty() || !redux_store.reducers.is_empty() || !redux_store.selectors.is_empty() {
                self.redux_stores.insert(store_name, redux_store);
            }
        }
    }
}
