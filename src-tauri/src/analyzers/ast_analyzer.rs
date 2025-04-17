use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AstAnalysisResult {
    pub file_path: String,
    pub language: String,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
    pub functions: Vec<FunctionInfo>,
    pub classes: Vec<ClassInfo>,
    pub interfaces: Vec<InterfaceInfo>,
    pub variables: Vec<VariableInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportInfo {
    pub name: String,
    pub source: String,
    pub is_default: bool,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportInfo {
    pub name: String,
    pub is_default: bool,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub is_generator: bool,
    pub is_exported: bool,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub type_annotation: Option<String>,
    pub default_value: Option<String>,
    pub is_rest: bool,
    pub is_optional: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassInfo {
    pub name: String,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub methods: Vec<MethodInfo>,
    pub properties: Vec<PropertyInfo>,
    pub is_exported: bool,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodInfo {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub is_static: bool,
    pub is_async: bool,
    pub is_generator: bool,
    pub visibility: String, // "public", "private", "protected"
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyInfo {
    pub name: String,
    pub type_annotation: Option<String>,
    pub is_static: bool,
    pub visibility: String, // "public", "private", "protected"
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceInfo {
    pub name: String,
    pub extends: Vec<String>,
    pub properties: Vec<InterfacePropertyInfo>,
    pub methods: Vec<InterfaceMethodInfo>,
    pub is_exported: bool,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfacePropertyInfo {
    pub name: String,
    pub type_annotation: String,
    pub is_optional: bool,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceMethodInfo {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub is_optional: bool,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub type_annotation: Option<String>,
    pub is_const: bool,
    pub is_exported: bool,
    pub line: usize,
}

pub struct AstAnalyzer;

impl AstAnalyzer {
    pub fn analyze_file<P: AsRef<Path>>(file_path: P) -> Result<AstAnalysisResult, String> {
        let path = file_path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
            
        let language = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                "js" => "JavaScript",
                "ts" => "TypeScript",
                "jsx" => "React",
                "tsx" => "React TypeScript",
                "rs" => "Rust",
                "py" => "Python",
                "rb" => "Ruby",
                _ => "Unknown",
            })
            .unwrap_or("Unknown")
            .to_string();
            
        // This is a placeholder implementation
        // In a real implementation, we would use language-specific parsers
        // to analyze the AST of the file
        
        Ok(AstAnalysisResult {
            file_path: path.to_string_lossy().to_string(),
            language,
            imports: Vec::new(),
            exports: Vec::new(),
            functions: Vec::new(),
            classes: Vec::new(),
            interfaces: Vec::new(),
            variables: Vec::new(),
        })
    }
    
    pub fn analyze_directory<P: AsRef<Path>>(dir_path: P) -> Result<Vec<AstAnalysisResult>, String> {
        let path = dir_path.as_ref();
        let mut results = Vec::new();
        
        if !path.is_dir() {
            return Err(format!("Not a directory: {}", path.display()));
        }
        
        for entry in fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let entry_path = entry.path();
            
            if entry_path.is_file() {
                let ext = entry_path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");
                    
                // Only analyze source code files
                if ["js", "ts", "jsx", "tsx", "rs", "py", "rb"].contains(&ext) {
                    match Self::analyze_file(&entry_path) {
                        Ok(result) => results.push(result),
                        Err(e) => eprintln!("Failed to analyze {}: {}", entry_path.display(), e),
                    }
                }
            } else if entry_path.is_dir() {
                // Recursively analyze subdirectories
                match Self::analyze_directory(&entry_path) {
                    Ok(mut sub_results) => results.append(&mut sub_results),
                    Err(e) => eprintln!("Failed to analyze directory {}: {}", entry_path.display(), e),
                }
            }
        }
        
        Ok(results)
    }
}
