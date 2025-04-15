use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use serde::{Serialize, Deserialize};

/// AI-driven technical debt analyzer for Rust codebases
pub struct TechDebtAnalyzer {
    base_dir: PathBuf,
    patterns: HashMap<&'static str, Regex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtItem {
    pub file: String,
    pub line: usize,
    pub category: String,
    pub description: String,
    pub severity: TechDebtSeverity,
    pub fix_suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechDebtSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl TechDebtAnalyzer {
    pub fn new(base_dir: PathBuf) -> Self {
        let mut patterns = HashMap::new();
        
        // Patterns for common code smells in Rust
        patterns.insert("todo", Regex::new(r"(?i)// *TODO").unwrap());
        patterns.insert("fixme", Regex::new(r"(?i)// *FIXME").unwrap());
        patterns.insert("hack", Regex::new(r"(?i)// *HACK").unwrap());
        patterns.insert("magic_number", Regex::new(r"(?<!\bconst\b.*=\s*)\b\d{4,}\b").unwrap());
        patterns.insert("large_enum", Regex::new(r"enum\s+\w+\s*\{[^}]{1000,}\}").unwrap());
        patterns.insert("nested_match", Regex::new(r"match\s+.*\{\s*.*match\s+").unwrap());
        patterns.insert("unsafe_block", Regex::new(r"unsafe\s*\{").unwrap());
        patterns.insert("panics", Regex::new(r"panic!\(|\.unwrap\(\)|\.expect\(").unwrap());
        patterns.insert("println_debug", Regex::new(r"println!\(").unwrap());
        patterns.insert("large_function", Regex::new(r"fn\s+\w+[^}]*\{[^}]{300,}\}").unwrap());
        patterns.insert("clone_clone", Regex::new(r"\.clone\(\)\.clone\(\)").unwrap());
        patterns.insert("mut_params", Regex::new(r"fn\s+\w+.*\(&mut\s+.*\)").unwrap());
        
        Self {
            base_dir,
            patterns,
        }
    }
    
    /// Analyze a Rust file for technical debt
    pub fn analyze_file(&self, file_path: &Path) -> Result<Vec<TechDebtItem>, String> {
        // Skip files in target dir
        if file_path.to_string_lossy().contains("target") {
            return Ok(Vec::new());
        }
        
        // Skip non-Rust files
        if file_path.extension().map_or(true, |ext| ext != "rs") {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
        
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .map_err(|_| "Failed to create relative path".to_string())?
            .to_string_lossy()
            .to_string();
        
        let mut debt_items = Vec::new();
        
        // Check for TODOs, FIXMEs, and HACKs
        for (i, line) in content.lines().enumerate() {
            // Check for TODO comments
            if self.patterns.get("todo").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "TODO".to_string(),
                    description,
                    severity: TechDebtSeverity::Low,
                    fix_suggestion: "Implement the TODO item".to_string(),
                });
            }
            
            // Check for FIXME comments
            if self.patterns.get("fixme").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "FIXME".to_string(),
                    description,
                    severity: TechDebtSeverity::Medium,
                    fix_suggestion: "Fix the noted issue".to_string(),
                });
            }
            
            // Check for HACK comments
            if self.patterns.get("hack").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "HACK".to_string(),
                    description,
                    severity: TechDebtSeverity::Medium,
                    fix_suggestion: "Refactor the hack with a proper solution".to_string(),
                });
            }
            
            // Check for magic numbers
            if self.patterns.get("magic_number").unwrap().is_match(line) {
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "Magic Number".to_string(),
                    description: format!("Magic number in code: {}", line.trim()),
                    severity: TechDebtSeverity::Low,
                    fix_suggestion: "Replace with a named constant".to_string(),
                });
            }
            
            // Check for panics and unwraps
            if self.patterns.get("panics").unwrap().is_match(line) {
                if line.contains(".unwrap(") {
                    debt_items.push(TechDebtItem {
                        file: relative_path.clone(),
                        line: i + 1,
                        category: "Unwrap Usage".to_string(),
                        description: "Using unwrap() might lead to panics in production".to_string(),
                        severity: TechDebtSeverity::Medium,
                        fix_suggestion: "Replace with proper error handling using match or if let".to_string(),
                    });
                } else if line.contains("panic!") {
                    debt_items.push(TechDebtItem {
                        file: relative_path.clone(),
                        line: i + 1,
                        category: "Panic Usage".to_string(),
                        description: "Using panic! in production code".to_string(),
                        severity: TechDebtSeverity::High,
                        fix_suggestion: "Replace with proper error handling and propagation".to_string(),
                    });
                }
            }
            
            // Check for println debugging
            if self.patterns.get("println_debug").unwrap().is_match(line) && !line.contains("// println") {
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "Debug Print".to_string(),
                    description: "Debug println in code".to_string(),
                    severity: TechDebtSeverity::Low,
                    fix_suggestion: "Replace with proper logging or remove".to_string(),
                });
            }
            
            // Check for double clone
            if self.patterns.get("clone_clone").unwrap().is_match(line) {
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "Inefficient Cloning".to_string(),
                    description: "Double clone() call detected".to_string(),
                    severity: TechDebtSeverity::Medium,
                    fix_suggestion: "Simplify to a single clone or refactor to avoid cloning".to_string(),
                });
            }
        }
        
        // Check for unsafe blocks
        if self.patterns.get("unsafe_block").unwrap().is_match(&content) {
            debt_items.push(TechDebtItem {
                file: relative_path.clone(),
                line: 0,
                category: "Unsafe Code".to_string(),
                description: "Unsafe block used in the file".to_string(),
                severity: TechDebtSeverity::High,
                fix_suggestion: "Reconsider if unsafe is necessary and document the safety invariants".to_string(),
            });
        }
        
        // Check for large functions
        if self.patterns.get("large_function").unwrap().is_match(&content) {
            debt_items.push(TechDebtItem {
                file: relative_path.clone(),
                line: 0,
                category: "Large Function".to_string(),
                description: "File contains very large functions (>300 lines)".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Refactor large functions into smaller, more focused functions".to_string(),
            });
        }
        
        // Check for large enums
        if self.patterns.get("large_enum").unwrap().is_match(&content) {
            debt_items.push(TechDebtItem {
                file: relative_path.clone(),
                line: 0,
                category: "Large Enum".to_string(),
                description: "File contains very large enum definitions".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Consider splitting the enum or using a different data structure".to_string(),
            });
        }
        
        // Check for nested match statements
        if self.patterns.get("nested_match").unwrap().is_match(&content) {
            debt_items.push(TechDebtItem {
                file: relative_path.clone(),
                line: 0,
                category: "Nested Match".to_string(),
                description: "Nested match statements detected".to_string(),
                severity: TechDebtSeverity::Low,
                fix_suggestion: "Extract inner match to a separate function or use if let".to_string(),
            });
        }
        
        // Check file complexity by size
        let loc = content.lines().count();
        if loc > 1000 {
            debt_items.push(TechDebtItem {
                file: relative_path,
                line: 0,
                category: "Large File".to_string(),
                description: format!("File is very large with {} lines of code", loc),
                severity: TechDebtSeverity::High,
                fix_suggestion: "Split into multiple smaller, focused modules".to_string(),
            });
        } else if loc > 500 {
            debt_items.push(TechDebtItem {
                file: relative_path,
                line: 0,
                category: "Large File".to_string(),
                description: format!("File is large with {} lines of code", loc),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Consider splitting into multiple modules".to_string(),
            });
        }
        
        Ok(debt_items)
    }
    
    /// Analyze entire codebase for technical debt
    pub fn analyze_codebase(&self) -> Result<Vec<TechDebtItem>, String> {
        let mut all_debt = Vec::new();
        
        // Walk the directory tree
        self.analyze_directory(&self.base_dir, &mut all_debt)?;
        
        // Sort by severity (higher severity first)
        all_debt.sort_by(|a, b| {
            let a_severity = match a.severity {
                TechDebtSeverity::Critical => 3,
                TechDebtSeverity::High => 2,
                TechDebtSeverity::Medium => 1,
                TechDebtSeverity::Low => 0,
            };
            
            let b_severity = match b.severity {
                TechDebtSeverity::Critical => 3,
                TechDebtSeverity::High => 2,
                TechDebtSeverity::Medium => 1,
                TechDebtSeverity::Low => 0,
            };
            
            b_severity.cmp(&a_severity)
        });
        
        Ok(all_debt)
    }
    
    /// Recursively analyze directories
    fn analyze_directory(&self, dir: &Path, results: &mut Vec<TechDebtItem>) -> Result<(), String> {
        // Skip certain directories
        let dir_name = dir.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
        if ["target", "node_modules", ".git", "build-output"].contains(&dir_name.as_ref()) {
            return Ok(());
        }
        
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        self.analyze_directory(&path, results)?;
                    } else if path.is_file() {
                        let debt_items = self.analyze_file(&path)?;
                        results.extend(debt_items);
                    }
                }
            },
            Err(e) => {
                return Err(format!("Failed to read directory {}: {}", dir.display(), e));
            }
        }
        
        Ok(())
    }
    
    /// Generate a tech debt report in Markdown format
    pub fn generate_report(&self) -> Result<String, String> {
        let debt_items = self.analyze_codebase()?;
        
        if debt_items.is_empty() {
            return Ok("# Technical Debt Report\n\nNo technical debt items found. Great job!".to_string());
        }
        
        let mut report = String::new();
        
        // Header
        report.push_str("# Technical Debt Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", chrono::Local::now().format("%Y-%m-%d")));
        
        // Summary
        report.push_str("## Summary\n\n");
        
        let critical = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Critical)).count();
        let high = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::High)).count();
        let medium = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Medium)).count();
        let low = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Low)).count();
        
        report.push_str("| Severity | Count |\n");
        report.push_str("|----------|-------|\n");
        report.push_str(&format!("| 🔴 Critical | {} |\n", critical));
        report.push_str(&format!("| 🟠 High | {} |\n", high));
        report.push_str(&format!("| 🟡 Medium | {} |\n", medium));
        report.push_str(&format!("| 🟢 Low | {} |\n", low));
        report.push_str(&format!("| **Total** | **{}** |\n", debt_items.len()));
        
        report.push_str("\n");
        
        // Items by category
        report.push_str("## Items by Category\n\n");
        
        let mut categories = HashMap::new();
        for item in &debt_items {
            categories.entry(item.category.clone()).or_insert_with(Vec::new).push(item);
        }
        
        let mut category_names: Vec<String> = categories.keys().cloned().collect();
        category_names.sort();
        
        for category in &category_names {
            let items = categories.get(category).unwrap();
            report.push_str(&format!("### {}\n\n", category));
            
            report.push_str("| File | Line | Severity | Description | Suggestion |\n");
            report.push_str("|------|------|----------|-------------|------------|\n");
            
            for item in items {
                let severity_icon = match item.severity {
                    TechDebtSeverity::Critical => "🔴",
                    TechDebtSeverity::High => "🟠",
                    TechDebtSeverity::Medium => "🟡",
                    TechDebtSeverity::Low => "🟢",
                };
                
                report.push_str(&format!("| `{}` | {} | {} | {} | {} |\n",
                    item.file,
                    if item.line == 0 { "N/A".to_string() } else { item.line.to_string() },
                    severity_icon,
                    item.description,
                    item.fix_suggestion
                ));
            }
            
            report.push_str("\n");
        }
        
        // Highest severity items
        report.push_str("## Critical and High Severity Items\n\n");
        
        let high_severity_items: Vec<&TechDebtItem> = debt_items.iter()
            .filter(|item| matches!(item.severity, TechDebtSeverity::Critical | TechDebtSeverity::High))
            .collect();
        
        if high_severity_items.is_empty() {
            report.push_str("No critical or high severity items found.\n\n");
        } else {
            report.push_str("| File | Line | Category | Description | Suggestion |\n");
            report.push_str("|------|------|----------|-------------|------------|\n");
            
            for item in high_severity_items {
                let severity_icon = match item.severity {
                    TechDebtSeverity::Critical => "🔴",
                    TechDebtSeverity::High => "🟠",
                    _ => "",
                };
                
                report.push_str(&format!("| `{}` | {} | {} {} | {} | {} |\n",
                    item.file,
                    if item.line == 0 { "N/A".to_string() } else { item.line.to_string() },
                    severity_icon,
                    item.category,
                    item.description,
                    item.fix_suggestion
                ));
            }
            
            report.push_str("\n");
        }
        
        // Hotspots (files with most tech debt)
        report.push_str("## Technical Debt Hotspots\n\n");
        
        let mut file_counts = HashMap::new();
        for item in &debt_items {
            *file_counts.entry(item.file.clone()).or_insert(0) += 1;
        }
        
        let mut files: Vec<(String, usize)> = file_counts.into_iter().collect();
        files.sort_by(|a, b| b.1.cmp(&a.1));
        
        report.push_str("| File | Debt Items |\n");
        report.push_str("|------|------------|\n");
        
        for (file, count) in files.iter().take(10) {
            report.push_str(&format!("| `{}` | {} |\n", file, count));
        }
        
        report.push_str("\n");
        
        // Recommendations
        report.push_str("## Recommendations\n\n");
        
        if critical > 0 {
            report.push_str("1. **Address Critical Issues First**: Focus on resolving the critical issues that could lead to severe bugs or security vulnerabilities.\n");
        }
        
        if high > 0 {
            report.push_str("2. **Tackle High Severity Items**: Address high severity items which can improve code quality significantly.\n");
        }
        
        let top_categories: Vec<&String> = category_names.iter()
            .filter(|c| {
                let items = categories.get(*c).unwrap();
                items.len() >= 5
            })
            .collect();
        
        if !top_categories.is_empty() {
            report.push_str("3. **Focus on Common Categories**: Address these common issues:\n");
            for category in top_categories {
                report.push_str(&format!("   - {}: {} instances\n", 
                    category, 
                    categories.get(category).unwrap().len()));
            }
        }
        
        Ok(report)
    }
}

/// Helper function to extract comment text
fn extract_comment(line: &str) -> String {
    if let Some(comment_start) = line.find("//") {
        let comment = line[comment_start + 2..].trim();
        
        // Remove TODO, FIXME, HACK markers
        let comment = comment.trim_start_matches(|c: char| !c.is_alphabetic());
        let comment = comment.trim_start_matches("TODO").trim_start_matches("FIXME").trim_start_matches("HACK");
        let comment = comment.trim_start_matches(':').trim_start_matches('-').trim_start_matches('(').trim();
        
        if comment.is_empty() {
            "No description provided".to_string()
        } else {
            comment.to_string()
        }
    } else {
        "No description provided".to_string()
    }
}
