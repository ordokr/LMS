use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

// Haskell module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaskellModule {
    pub name: String,
    pub path: String,
    pub functions: Vec<HaskellFunction>,
    pub data_types: Vec<HaskellDataType>,
    pub type_classes: Vec<HaskellTypeClass>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

// Haskell function information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaskellFunction {
    pub name: String,
    pub type_signature: String,
    pub is_exported: bool,
    pub complexity: u32,
    pub line_count: u32,
}

// Haskell data type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaskellDataType {
    pub name: String,
    pub constructors: Vec<String>,
    pub is_exported: bool,
    pub is_record: bool,
    pub fields: Vec<HaskellField>,
}

// Haskell field information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaskellField {
    pub name: String,
    pub type_name: String,
}

// Haskell type class information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaskellTypeClass {
    pub name: String,
    pub methods: Vec<HaskellFunction>,
    pub is_exported: bool,
}

// Haskell analysis result
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HaskellAnalysisResult {
    pub modules: Vec<HaskellModule>,
    pub total_functions: usize,
    pub total_data_types: usize,
    pub total_type_classes: usize,
    pub total_lines: usize,
    pub business_logic_modules: Vec<String>,
    pub blockchain_modules: Vec<String>,
    pub sync_modules: Vec<String>,
    pub parser_modules: Vec<String>,
    pub metrics: HashMap<String, String>,
}

// Haskell analyzer
#[derive(Debug, Default)]
pub struct HaskellAnalyzer;

impl HaskellAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        println!("Analyzing Haskell code at {:?}...", base_dir);

        let mut result = HaskellAnalysisResult::default();

        // Look for Haskell files in the project
        self.find_and_analyze_haskell_files(base_dir, &mut result)?;

        // Categorize modules based on their purpose
        self.categorize_modules(&mut result);

        // Calculate metrics
        self.calculate_metrics(&mut result);

        // Return the result as JSON
        match serde_json::to_string_pretty(&result) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize HaskellAnalysisResult: {}", e)),
        }
    }

    fn find_and_analyze_haskell_files(&self, base_dir: &PathBuf, result: &mut HaskellAnalysisResult) -> Result<(), String> {
        // Check for haskell-integration directory
        let haskell_integration_dir = base_dir.join("haskell-integration");
        if haskell_integration_dir.exists() {
            println!("Found haskell-integration directory at {:?}", haskell_integration_dir);
            self.analyze_haskell_directory(&haskell_integration_dir, result)?;
        }

        // Also check for any Haskell files in the project
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map_or(false, |ext| ext == "hs" || ext == "lhs")
            })
        {
            self.analyze_haskell_file(entry.path(), result)?;
        }

        Ok(())
    }

    fn analyze_haskell_directory(&self, dir: &PathBuf, result: &mut HaskellAnalysisResult) -> Result<(), String> {
        // Check for src directory
        let src_dir = dir.join("src");
        if src_dir.exists() {
            for entry in WalkDir::new(&src_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().map_or(false, |ext| ext == "hs" || ext == "lhs")
                })
            {
                self.analyze_haskell_file(entry.path(), result)?;
            }
        }

        // Check for app directory
        let app_dir = dir.join("app");
        if app_dir.exists() {
            for entry in WalkDir::new(&app_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().map_or(false, |ext| ext == "hs" || ext == "lhs")
                })
            {
                self.analyze_haskell_file(entry.path(), result)?;
            }
        }

        // Check for test directory
        let test_dir = dir.join("test");
        if test_dir.exists() {
            for entry in WalkDir::new(&test_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().map_or(false, |ext| ext == "hs" || ext == "lhs")
                })
            {
                self.analyze_haskell_file(entry.path(), result)?;
            }
        }

        // Check for package.yaml or *.cabal files
        let package_yaml = dir.join("package.yaml");
        if package_yaml.exists() {
            self.analyze_package_yaml(&package_yaml, result)?;
        }

        // Look for .cabal files
        for entry in fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "cabal") {
                self.analyze_cabal_file(&path, result)?;
            }
        }

        Ok(())
    }

    fn analyze_haskell_file(&self, path: &Path, result: &mut HaskellAnalysisResult) -> Result<(), String> {
        println!("Analyzing Haskell file: {:?}", path);

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

        // Extract module name
        let module_name = self.extract_module_name(&content)
            .unwrap_or_else(|| path.file_stem().unwrap_or_default().to_string_lossy().to_string());

        // Extract imports
        let imports = self.extract_imports(&content);

        // Extract exports
        let exports = self.extract_exports(&content);

        // Extract functions
        let functions = self.extract_functions(&content);

        // Extract data types
        let data_types = self.extract_data_types(&content);

        // Extract type classes
        let type_classes = self.extract_type_classes(&content);

        // Create module
        let module = HaskellModule {
            name: module_name,
            path: path.to_string_lossy().to_string(),
            functions,
            data_types,
            type_classes,
            imports,
            exports,
        };

        // Calculate metrics before pushing
        let functions_count = module.functions.len();
        let data_types_count = module.data_types.len();
        let type_classes_count = module.type_classes.len();
        let lines_count = content.lines().count();

        // Add module to result
        result.modules.push(module);
        result.total_functions += functions_count;
        result.total_data_types += data_types_count;
        result.total_type_classes += type_classes_count;
        result.total_lines += lines_count;

        Ok(())
    }

    fn extract_module_name(&self, content: &str) -> Option<String> {
        // Simple regex-like extraction for module name
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("module ") && line.contains(" where") {
                return Some(line.strip_prefix("module ")?.split_whitespace().next()?.to_string());
            }
        }
        None
    }

    fn extract_imports(&self, content: &str) -> Vec<String> {
        let mut imports = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("import ") {
                if let Some(import_name) = line.strip_prefix("import ").and_then(|s| s.split_whitespace().next()) {
                    imports.push(import_name.to_string());
                }
            }
        }
        imports
    }

    fn extract_exports(&self, content: &str) -> Vec<String> {
        let mut exports = Vec::new();
        // This is a simplified implementation
        // In a real implementation, we would parse the export list more carefully
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("module ") && line.contains("(") && line.contains(")") {
                if let Some(export_list) = line.split('(').nth(1).and_then(|s| s.split(')').next()) {
                    for export in export_list.split(',') {
                        exports.push(export.trim().to_string());
                    }
                }
            }
        }
        exports
    }

    fn extract_functions(&self, content: &str) -> Vec<HaskellFunction> {
        let mut functions = Vec::new();
        let mut current_function = None;
        let mut line_count = 0;

        for line in content.lines() {
            let line = line.trim();

            // Look for type signatures
            if line.contains("::") && !line.starts_with("--") {
                let parts: Vec<&str> = line.split("::").collect();
                if parts.len() >= 2 {
                    let name = parts[0].trim().to_string();
                    let type_sig = parts[1].trim().to_string();

                    current_function = Some(HaskellFunction {
                        name,
                        type_signature: type_sig,
                        is_exported: false, // We'll set this later
                        complexity: 1,      // Simple default
                        line_count: 0,
                    });
                    line_count = 0;
                }
            }
            // Look for function implementations
            else if let Some(ref mut func) = current_function {
                if line.starts_with(&func.name) || line.is_empty() || line.starts_with("  ") {
                    line_count += 1;
                } else {
                    // End of function
                    func.line_count = line_count;
                    functions.push(func.clone());
                    current_function = None;
                }
            }
        }

        // Don't forget the last function
        if let Some(mut func) = current_function {
            func.line_count = line_count;
            functions.push(func);
        }

        functions
    }

    fn extract_data_types(&self, content: &str) -> Vec<HaskellDataType> {
        let mut data_types = Vec::new();

        // This is a simplified implementation
        // In a real implementation, we would parse data types more carefully
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("data ") || line.starts_with("newtype ") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() >= 2 {
                    let type_decl = parts[0].trim();
                    let type_name = type_decl.split_whitespace().nth(1).unwrap_or("").to_string();

                    let is_record = parts[1].contains("{");

                    let constructors = if is_record {
                        vec![type_name.clone()]
                    } else {
                        parts[1].split('|')
                            .map(|s| s.trim().split_whitespace().next().unwrap_or("").to_string())
                            .collect()
                    };

                    let fields = if is_record {
                        // Extract record fields
                        let fields_str = parts[1].split('{').nth(1).unwrap_or("").split('}').next().unwrap_or("");
                        fields_str.split(',')
                            .map(|field| {
                                let field_parts: Vec<&str> = field.split("::").collect();
                                if field_parts.len() >= 2 {
                                    HaskellField {
                                        name: field_parts[0].trim().to_string(),
                                        type_name: field_parts[1].trim().to_string(),
                                    }
                                } else {
                                    HaskellField {
                                        name: field.trim().to_string(),
                                        type_name: "Unknown".to_string(),
                                    }
                                }
                            })
                            .collect()
                    } else {
                        Vec::new()
                    };

                    data_types.push(HaskellDataType {
                        name: type_name,
                        constructors,
                        is_exported: false, // We'll set this later
                        is_record,
                        fields,
                    });
                }
            }
        }

        data_types
    }

    fn extract_type_classes(&self, content: &str) -> Vec<HaskellTypeClass> {
        let mut type_classes = Vec::new();

        // This is a simplified implementation
        // In a real implementation, we would parse type classes more carefully
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("class ") {
                let parts: Vec<&str> = line.split(" where").collect();
                if parts.len() >= 1 {
                    let class_decl = parts[0].trim();
                    let class_name = class_decl.split_whitespace().nth(1).unwrap_or("").to_string();

                    type_classes.push(HaskellTypeClass {
                        name: class_name,
                        methods: Vec::new(), // We'll extract methods in a real implementation
                        is_exported: false,  // We'll set this later
                    });
                }
            }
        }

        type_classes
    }

    fn analyze_package_yaml(&self, path: &Path, result: &mut HaskellAnalysisResult) -> Result<(), String> {
        println!("Analyzing package.yaml: {:?}", path);

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

        // Extract exposed modules
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("exposed-modules:") {
                // In a real implementation, we would parse the YAML structure properly
                // This is a simplified approach
                result.metrics.insert("has_package_yaml".to_string(), "true".to_string());
                break;
            }
        }

        Ok(())
    }

    fn analyze_cabal_file(&self, path: &Path, result: &mut HaskellAnalysisResult) -> Result<(), String> {
        println!("Analyzing cabal file: {:?}", path);

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

        // Extract library information
        let mut in_library_section = false;
        for line in content.lines() {
            let line = line.trim();

            if line == "library" {
                in_library_section = true;
                result.metrics.insert("has_cabal_file".to_string(), "true".to_string());
            } else if in_library_section && line.starts_with("exposed-modules:") {
                // In a real implementation, we would parse the cabal file structure properly
                // This is a simplified approach
                break;
            }
        }

        Ok(())
    }

    fn categorize_modules(&self, result: &mut HaskellAnalysisResult) {
        for module in &result.modules {
            // Categorize based on module name and imports
            if module.name.contains("Blockchain") || module.imports.iter().any(|i| i.contains("Blockchain")) {
                result.blockchain_modules.push(module.name.clone());
            }

            if module.name.contains("Sync") || module.name.contains("CRDT") || module.imports.iter().any(|i| i.contains("Sync") || i.contains("CRDT")) {
                result.sync_modules.push(module.name.clone());
            }

            if module.name.contains("Parser") || module.imports.iter().any(|i| i.contains("Parser")) {
                result.parser_modules.push(module.name.clone());
            }

            // Business logic is a catch-all for modules that don't fit other categories
            if !module.name.contains("Test") &&
               !result.blockchain_modules.contains(&module.name) &&
               !result.sync_modules.contains(&module.name) &&
               !result.parser_modules.contains(&module.name) {
                result.business_logic_modules.push(module.name.clone());
            }
        }
    }

    fn calculate_metrics(&self, result: &mut HaskellAnalysisResult) {
        // Calculate basic metrics
        result.metrics.insert("total_modules".to_string(), result.modules.len().to_string());
        result.metrics.insert("total_functions".to_string(), result.total_functions.to_string());
        result.metrics.insert("total_data_types".to_string(), result.total_data_types.to_string());
        result.metrics.insert("total_type_classes".to_string(), result.total_type_classes.to_string());
        result.metrics.insert("total_lines".to_string(), result.total_lines.to_string());

        // Calculate category metrics
        result.metrics.insert("business_logic_modules".to_string(), result.business_logic_modules.len().to_string());
        result.metrics.insert("blockchain_modules".to_string(), result.blockchain_modules.len().to_string());
        result.metrics.insert("sync_modules".to_string(), result.sync_modules.len().to_string());
        result.metrics.insert("parser_modules".to_string(), result.parser_modules.len().to_string());

        // Calculate average function complexity
        let total_complexity: u32 = result.modules.iter()
            .flat_map(|m| &m.functions)
            .map(|f| f.complexity)
            .sum();

        if result.total_functions > 0 {
            let avg_complexity = total_complexity as f64 / result.total_functions as f64;
            result.metrics.insert("average_function_complexity".to_string(), format!("{:.2}", avg_complexity));
        }

        // Calculate average function size
        let total_lines: u32 = result.modules.iter()
            .flat_map(|m| &m.functions)
            .map(|f| f.line_count)
            .sum();

        if result.total_functions > 0 {
            let avg_lines = total_lines as f64 / result.total_functions as f64;
            result.metrics.insert("average_function_lines".to_string(), format!("{:.2}", avg_lines));
        }
    }
}
