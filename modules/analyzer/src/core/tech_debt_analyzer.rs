use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;

use crate::core::analysis_result::{TechDebtItem, TechDebtSeverity};

/// Analyzer for technical debt in the codebase
pub struct TechDebtAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    
    /// Regex patterns for detecting technical debt
    patterns: HashMap<String, Regex>,
    
    /// Directories to exclude from analysis
    exclude_dirs: Vec<String>,
}

impl TechDebtAnalyzer {
    /// Create a new technical debt analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        let mut patterns = HashMap::new();
        
        // Add regex patterns for detecting technical debt
        patterns.insert("todo".to_string(), Regex::new(r"(?i)//\s*TODO").unwrap());
        patterns.insert("fixme".to_string(), Regex::new(r"(?i)//\s*FIXME").unwrap());
        patterns.insert("hack".to_string(), Regex::new(r"(?i)//\s*HACK").unwrap());
        patterns.insert("bug".to_string(), Regex::new(r"(?i)//\s*BUG").unwrap());
        patterns.insert("workaround".to_string(), Regex::new(r"(?i)//\s*WORKAROUND").unwrap());
        patterns.insert("magic_number".to_string(), Regex::new(r"(?i)//\s*MAGIC NUMBER").unwrap());
        patterns.insert("hardcoded".to_string(), Regex::new(r"(?i)//\s*HARDCODED").unwrap());
        patterns.insert("refactor".to_string(), Regex::new(r"(?i)//\s*REFACTOR").unwrap());
        patterns.insert("optimize".to_string(), Regex::new(r"(?i)//\s*OPTIMIZE").unwrap());
        patterns.insert("security".to_string(), Regex::new(r"(?i)//\s*SECURITY").unwrap());
        
        // Add directories to exclude
        let exclude_dirs = vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "build-output".to_string(),
        ];
        
        Self {
            base_dir,
            patterns,
            exclude_dirs,
        }
    }
    
    /// Analyze a file for technical debt
    pub fn analyze_file(&self, file_path: &Path) -> Result<Vec<TechDebtItem>, String> {
        // Skip files in excluded directories
        for exclude_dir in &self.exclude_dirs {
            if file_path.to_string_lossy().contains(exclude_dir) {
                return Ok(Vec::new());
            }
        }
        
        // Skip non-Rust and non-Haskell files
        let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        if extension != "rs" && extension != "hs" {
            return Ok(Vec::new());
        }
        
        // Read file content
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
        
        // Create relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .map_err(|_| "Failed to create relative path".to_string())?
            .to_string_lossy()
            .to_string();
        
        let mut debt_items = Vec::new();
        
        // Check each line for technical debt patterns
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
                    severity: TechDebtSeverity::High,
                    fix_suggestion: "Refactor the hack with a proper solution".to_string(),
                });
            }
            
            // Check for BUG comments
            if self.patterns.get("bug").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "BUG".to_string(),
                    description,
                    severity: TechDebtSeverity::Critical,
                    fix_suggestion: "Fix the bug".to_string(),
                });
            }
            
            // Check for WORKAROUND comments
            if self.patterns.get("workaround").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "WORKAROUND".to_string(),
                    description,
                    severity: TechDebtSeverity::Medium,
                    fix_suggestion: "Replace the workaround with a proper solution".to_string(),
                });
            }
            
            // Check for MAGIC NUMBER comments
            if self.patterns.get("magic_number").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "MAGIC NUMBER".to_string(),
                    description,
                    severity: TechDebtSeverity::Low,
                    fix_suggestion: "Replace magic number with a named constant".to_string(),
                });
            }
            
            // Check for HARDCODED comments
            if self.patterns.get("hardcoded").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "HARDCODED".to_string(),
                    description,
                    severity: TechDebtSeverity::Medium,
                    fix_suggestion: "Replace hardcoded value with a configuration option".to_string(),
                });
            }
            
            // Check for REFACTOR comments
            if self.patterns.get("refactor").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "REFACTOR".to_string(),
                    description,
                    severity: TechDebtSeverity::Medium,
                    fix_suggestion: "Refactor the code as noted".to_string(),
                });
            }
            
            // Check for OPTIMIZE comments
            if self.patterns.get("optimize").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "OPTIMIZE".to_string(),
                    description,
                    severity: TechDebtSeverity::Low,
                    fix_suggestion: "Optimize the code as noted".to_string(),
                });
            }
            
            // Check for SECURITY comments
            if self.patterns.get("security").unwrap().is_match(line) {
                let description = extract_comment(line);
                debt_items.push(TechDebtItem {
                    file: relative_path.clone(),
                    line: i + 1,
                    category: "SECURITY".to_string(),
                    description,
                    severity: TechDebtSeverity::Critical,
                    fix_suggestion: "Address the security issue".to_string(),
                });
            }
        }
        
        Ok(debt_items)
    }
    
    /// Analyze a directory for technical debt
    pub fn analyze_directory(&self, dir_path: &Path, debt_items: &mut Vec<TechDebtItem>) -> Result<(), String> {
        // Skip excluded directories
        for exclude_dir in &self.exclude_dirs {
            if dir_path.to_string_lossy().contains(exclude_dir) {
                return Ok(());
            }
        }
        
        // Walk through the directory
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            
            // Analyze the file
            let mut file_debt = self.analyze_file(path)?;
            debt_items.append(&mut file_debt);
        }
        
        Ok(())
    }
    
    /// Analyze the entire codebase for technical debt
    pub fn analyze_codebase(&self) -> Result<Vec<TechDebtItem>, String> {
        let mut all_debt = Vec::new();
        
        // Analyze the base directory
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
    
    /// Generate a technical debt report
    pub fn generate_report(&self) -> Result<String, String> {
        // Analyze the codebase
        let debt_items = self.analyze_codebase()?;
        
        // Count items by severity
        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;
        
        for item in &debt_items {
            match item.severity {
                TechDebtSeverity::Critical => critical_count += 1,
                TechDebtSeverity::High => high_count += 1,
                TechDebtSeverity::Medium => medium_count += 1,
                TechDebtSeverity::Low => low_count += 1,
            }
        }
        
        // Generate the report
        let mut report = String::new();
        
        // Header
        report.push_str("# Technical Debt Report\n\n");
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("**Total Issues: {}**\n\n", debt_items.len()));
        report.push_str("| Severity | Count |\n");
        report.push_str("|----------|-------|\n");
        report.push_str(&format!("| Critical | {} |\n", critical_count));
        report.push_str(&format!("| High | {} |\n", high_count));
        report.push_str(&format!("| Medium | {} |\n", medium_count));
        report.push_str(&format!("| Low | {} |\n\n", low_count));
        
        // Critical Issues
        report.push_str("## Critical Issues\n\n");
        if critical_count > 0 {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for item in &debt_items {
                if item.severity == TechDebtSeverity::Critical {
                    report.push_str(&format!("| {} | {} | {} | {} |\n",
                        item.file,
                        item.line,
                        item.description,
                        item.fix_suggestion));
                }
            }
        } else {
            report.push_str("No critical issues found.\n");
        }
        report.push_str("\n");
        
        // High Issues
        report.push_str("## High Issues\n\n");
        if high_count > 0 {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for item in &debt_items {
                if item.severity == TechDebtSeverity::High {
                    report.push_str(&format!("| {} | {} | {} | {} |\n",
                        item.file,
                        item.line,
                        item.description,
                        item.fix_suggestion));
                }
            }
        } else {
            report.push_str("No high issues found.\n");
        }
        report.push_str("\n");
        
        // Medium Issues
        report.push_str("## Medium Issues\n\n");
        if medium_count > 0 {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for item in &debt_items {
                if item.severity == TechDebtSeverity::Medium {
                    report.push_str(&format!("| {} | {} | {} | {} |\n",
                        item.file,
                        item.line,
                        item.description,
                        item.fix_suggestion));
                }
            }
        } else {
            report.push_str("No medium issues found.\n");
        }
        report.push_str("\n");
        
        // Low Issues
        report.push_str("## Low Issues\n\n");
        if low_count > 0 {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for item in &debt_items {
                if item.severity == TechDebtSeverity::Low {
                    report.push_str(&format!("| {} | {} | {} | {} |\n",
                        item.file,
                        item.line,
                        item.description,
                        item.fix_suggestion));
                }
            }
        } else {
            report.push_str("No low issues found.\n");
        }
        
        Ok(report)
    }
}

/// Extract comment text from a line
fn extract_comment(line: &str) -> String {
    if let Some(comment_start) = line.find("//") {
        let comment = line[comment_start + 2..].trim();
        
        // Remove marker prefix
        if comment.starts_with("TODO:") {
            comment[5..].trim().to_string()
        } else if comment.starts_with("FIXME:") {
            comment[6..].trim().to_string()
        } else if comment.starts_with("HACK:") {
            comment[5..].trim().to_string()
        } else if comment.starts_with("BUG:") {
            comment[4..].trim().to_string()
        } else if comment.starts_with("WORKAROUND:") {
            comment[11..].trim().to_string()
        } else if comment.starts_with("MAGIC NUMBER:") {
            comment[13..].trim().to_string()
        } else if comment.starts_with("HARDCODED:") {
            comment[10..].trim().to_string()
        } else if comment.starts_with("REFACTOR:") {
            comment[9..].trim().to_string()
        } else if comment.starts_with("OPTIMIZE:") {
            comment[9..].trim().to_string()
        } else if comment.starts_with("SECURITY:") {
            comment[9..].trim().to_string()
        } else if comment.starts_with("TODO") {
            comment[4..].trim().to_string()
        } else if comment.starts_with("FIXME") {
            comment[5..].trim().to_string()
        } else if comment.starts_with("HACK") {
            comment[4..].trim().to_string()
        } else if comment.starts_with("BUG") {
            comment[3..].trim().to_string()
        } else if comment.starts_with("WORKAROUND") {
            comment[10..].trim().to_string()
        } else if comment.starts_with("MAGIC NUMBER") {
            comment[12..].trim().to_string()
        } else if comment.starts_with("HARDCODED") {
            comment[9..].trim().to_string()
        } else if comment.starts_with("REFACTOR") {
            comment[8..].trim().to_string()
        } else if comment.starts_with("OPTIMIZE") {
            comment[8..].trim().to_string()
        } else if comment.starts_with("SECURITY") {
            comment[8..].trim().to_string()
        } else {
            comment.to_string()
        }
    } else {
        String::new()
    }
}
