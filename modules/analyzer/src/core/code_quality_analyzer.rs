use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;

/// Analyzer for code quality in the codebase
pub struct CodeQualityAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    
    /// Regex patterns for detecting SOLID violations
    solid_patterns: HashMap<String, Regex>,
    
    /// Regex patterns for detecting design pattern implementations
    pattern_impl_patterns: HashMap<String, Regex>,
    
    /// Regex patterns for detecting design pattern violations
    pattern_violation_patterns: HashMap<String, Regex>,
    
    /// Directories to exclude from analysis
    exclude_dirs: Vec<String>,
}

/// SOLID principle violation
#[derive(Debug, Clone)]
pub struct SolidViolation {
    /// File path
    pub file: String,
    
    /// Line number
    pub line: usize,
    
    /// SOLID principle that was violated
    pub principle: String,
    
    /// Description of the violation
    pub description: String,
    
    /// Suggestion for fixing the violation
    pub fix_suggestion: String,
}

/// Design pattern implementation
#[derive(Debug, Clone)]
pub struct PatternImplementation {
    /// File path
    pub file: String,
    
    /// Line number
    pub line: usize,
    
    /// Design pattern that was implemented
    pub pattern: String,
    
    /// Description of the implementation
    pub description: String,
}

/// Design pattern violation
#[derive(Debug, Clone)]
pub struct PatternViolation {
    /// File path
    pub file: String,
    
    /// Line number
    pub line: usize,
    
    /// Design pattern that was violated
    pub pattern: String,
    
    /// Description of the violation
    pub description: String,
    
    /// Suggestion for fixing the violation
    pub fix_suggestion: String,
}

/// Code quality metrics
#[derive(Debug, Clone)]
pub struct CodeQualityMetrics {
    /// SOLID violations
    pub solid_violations: Vec<SolidViolation>,
    
    /// Design pattern implementations
    pub pattern_implementations: Vec<PatternImplementation>,
    
    /// Design pattern violations
    pub pattern_violations: Vec<PatternViolation>,
}

impl CodeQualityAnalyzer {
    /// Create a new code quality analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        let mut solid_patterns = HashMap::new();
        let mut pattern_impl_patterns = HashMap::new();
        let mut pattern_violation_patterns = HashMap::new();
        
        // Add regex patterns for detecting SOLID violations
        solid_patterns.insert(
            "single_responsibility".to_string(),
            Regex::new(r"(?i)//\s*SRP violation").unwrap()
        );
        solid_patterns.insert(
            "open_closed".to_string(),
            Regex::new(r"(?i)//\s*OCP violation").unwrap()
        );
        solid_patterns.insert(
            "liskov_substitution".to_string(),
            Regex::new(r"(?i)//\s*LSP violation").unwrap()
        );
        solid_patterns.insert(
            "interface_segregation".to_string(),
            Regex::new(r"(?i)//\s*ISP violation").unwrap()
        );
        solid_patterns.insert(
            "dependency_inversion".to_string(),
            Regex::new(r"(?i)//\s*DIP violation").unwrap()
        );
        
        // Add regex patterns for detecting design pattern implementations
        pattern_impl_patterns.insert(
            "factory".to_string(),
            Regex::new(r"(?i)//\s*Factory pattern").unwrap()
        );
        pattern_impl_patterns.insert(
            "singleton".to_string(),
            Regex::new(r"(?i)//\s*Singleton pattern").unwrap()
        );
        pattern_impl_patterns.insert(
            "observer".to_string(),
            Regex::new(r"(?i)//\s*Observer pattern").unwrap()
        );
        pattern_impl_patterns.insert(
            "strategy".to_string(),
            Regex::new(r"(?i)//\s*Strategy pattern").unwrap()
        );
        pattern_impl_patterns.insert(
            "command".to_string(),
            Regex::new(r"(?i)//\s*Command pattern").unwrap()
        );
        pattern_impl_patterns.insert(
            "repository".to_string(),
            Regex::new(r"(?i)//\s*Repository pattern").unwrap()
        );
        pattern_impl_patterns.insert(
            "dependency_injection".to_string(),
            Regex::new(r"(?i)//\s*Dependency injection").unwrap()
        );
        
        // Add regex patterns for detecting design pattern violations
        pattern_violation_patterns.insert(
            "factory".to_string(),
            Regex::new(r"(?i)//\s*Factory pattern violation").unwrap()
        );
        pattern_violation_patterns.insert(
            "singleton".to_string(),
            Regex::new(r"(?i)//\s*Singleton pattern violation").unwrap()
        );
        pattern_violation_patterns.insert(
            "observer".to_string(),
            Regex::new(r"(?i)//\s*Observer pattern violation").unwrap()
        );
        pattern_violation_patterns.insert(
            "strategy".to_string(),
            Regex::new(r"(?i)//\s*Strategy pattern violation").unwrap()
        );
        pattern_violation_patterns.insert(
            "command".to_string(),
            Regex::new(r"(?i)//\s*Command pattern violation").unwrap()
        );
        pattern_violation_patterns.insert(
            "repository".to_string(),
            Regex::new(r"(?i)//\s*Repository pattern violation").unwrap()
        );
        pattern_violation_patterns.insert(
            "dependency_injection".to_string(),
            Regex::new(r"(?i)//\s*Dependency injection violation").unwrap()
        );
        
        // Add directories to exclude
        let exclude_dirs = vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "build-output".to_string(),
        ];
        
        Self {
            base_dir,
            solid_patterns,
            pattern_impl_patterns,
            pattern_violation_patterns,
            exclude_dirs,
        }
    }
    
    /// Analyze a file for code quality
    pub fn analyze_file(&self, file_path: &Path) -> Result<(Vec<SolidViolation>, Vec<PatternImplementation>, Vec<PatternViolation>), String> {
        // Skip files in excluded directories
        for exclude_dir in &self.exclude_dirs {
            if file_path.to_string_lossy().contains(exclude_dir) {
                return Ok((Vec::new(), Vec::new(), Vec::new()));
            }
        }
        
        // Skip non-Rust and non-Haskell files
        let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        if extension != "rs" && extension != "hs" {
            return Ok((Vec::new(), Vec::new(), Vec::new()));
        }
        
        // Read file content
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
        
        // Create relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .map_err(|_| "Failed to create relative path".to_string())?
            .to_string_lossy()
            .to_string();
        
        let mut solid_violations = Vec::new();
        let mut pattern_implementations = Vec::new();
        let mut pattern_violations = Vec::new();
        
        // Check each line for code quality patterns
        for (i, line) in content.lines().enumerate() {
            // Check for SOLID violations
            for (principle, pattern) in &self.solid_patterns {
                if pattern.is_match(line) {
                    let description = extract_comment(line);
                    let fix_suggestion = get_solid_fix_suggestion(principle);
                    
                    solid_violations.push(SolidViolation {
                        file: relative_path.clone(),
                        line: i + 1,
                        principle: get_solid_principle_name(principle),
                        description,
                        fix_suggestion,
                    });
                }
            }
            
            // Check for design pattern implementations
            for (pattern_name, pattern) in &self.pattern_impl_patterns {
                if pattern.is_match(line) {
                    let description = extract_comment(line);
                    
                    pattern_implementations.push(PatternImplementation {
                        file: relative_path.clone(),
                        line: i + 1,
                        pattern: get_pattern_name(pattern_name),
                        description,
                    });
                }
            }
            
            // Check for design pattern violations
            for (pattern_name, pattern) in &self.pattern_violation_patterns {
                if pattern.is_match(line) {
                    let description = extract_comment(line);
                    let fix_suggestion = get_pattern_fix_suggestion(pattern_name);
                    
                    pattern_violations.push(PatternViolation {
                        file: relative_path.clone(),
                        line: i + 1,
                        pattern: get_pattern_name(pattern_name),
                        description,
                        fix_suggestion,
                    });
                }
            }
        }
        
        Ok((solid_violations, pattern_implementations, pattern_violations))
    }
    
    /// Analyze a directory for code quality
    pub fn analyze_directory(
        &self,
        dir_path: &Path,
        solid_violations: &mut Vec<SolidViolation>,
        pattern_implementations: &mut Vec<PatternImplementation>,
        pattern_violations: &mut Vec<PatternViolation>
    ) -> Result<(), String> {
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
            let (mut file_solid_violations, mut file_pattern_implementations, mut file_pattern_violations) = self.analyze_file(path)?;
            
            solid_violations.append(&mut file_solid_violations);
            pattern_implementations.append(&mut file_pattern_implementations);
            pattern_violations.append(&mut file_pattern_violations);
        }
        
        Ok(())
    }
    
    /// Analyze the entire codebase for code quality
    pub fn analyze_codebase(&self) -> Result<CodeQualityMetrics, String> {
        let mut solid_violations = Vec::new();
        let mut pattern_implementations = Vec::new();
        let mut pattern_violations = Vec::new();
        
        // Analyze the base directory
        self.analyze_directory(
            &self.base_dir,
            &mut solid_violations,
            &mut pattern_implementations,
            &mut pattern_violations
        )?;
        
        Ok(CodeQualityMetrics {
            solid_violations,
            pattern_implementations,
            pattern_violations,
        })
    }
    
    /// Generate a code quality report
    pub fn generate_report(&self) -> Result<String, String> {
        // Analyze the codebase
        let metrics = self.analyze_codebase()?;
        
        // Generate the report
        let mut report = String::new();
        
        // Header
        report.push_str("# Code Quality Analysis Report\n\n");
        
        // Summary
        report.push_str(&format!("**Total SOLID Violations:** {}\n", metrics.solid_violations.len()));
        report.push_str(&format!("**Total Pattern Implementations:** {}\n", metrics.pattern_implementations.len()));
        report.push_str(&format!("**Total Pattern Violations:** {}\n\n", metrics.pattern_violations.len()));
        
        // SOLID Principles Summary
        report.push_str("## SOLID Principles Summary\n\n");
        report.push_str("| Principle | Violations |\n");
        report.push_str("|-----------|------------|\n");
        
        let mut solid_counts = HashMap::new();
        for violation in &metrics.solid_violations {
            *solid_counts.entry(violation.principle.clone()).or_insert(0) += 1;
        }
        
        for principle in &["Single Responsibility", "Open-Closed", "Liskov Substitution", "Interface Segregation", "Dependency Inversion"] {
            let count = solid_counts.get(*principle).unwrap_or(&0);
            report.push_str(&format!("| {} | {} |\n", principle, count));
        }
        report.push_str("\n");
        
        // Design Patterns Summary
        report.push_str("## Design Patterns Summary\n\n");
        report.push_str("| Pattern | Implementations | Violations |\n");
        report.push_str("|---------|-----------------|------------|\n");
        
        let mut pattern_impl_counts = HashMap::new();
        for implementation in &metrics.pattern_implementations {
            *pattern_impl_counts.entry(implementation.pattern.clone()).or_insert(0) += 1;
        }
        
        let mut pattern_violation_counts = HashMap::new();
        for violation in &metrics.pattern_violations {
            *pattern_violation_counts.entry(violation.pattern.clone()).or_insert(0) += 1;
        }
        
        let all_patterns: HashSet<_> = pattern_impl_counts.keys().chain(pattern_violation_counts.keys()).cloned().collect();
        for pattern in all_patterns {
            let impl_count = pattern_impl_counts.get(&pattern).unwrap_or(&0);
            let violation_count = pattern_violation_counts.get(&pattern).unwrap_or(&0);
            report.push_str(&format!("| {} | {} | {} |\n", pattern, impl_count, violation_count));
        }
        report.push_str("\n");
        
        // SOLID Violations
        report.push_str("# SOLID Principles Analysis\n\n");
        
        // Single Responsibility Principle
        report.push_str("## Single Responsibility Principle Violations\n\n");
        report.push_str("A class should have only one reason to change.\n\n");
        
        let srp_violations: Vec<_> = metrics.solid_violations.iter()
            .filter(|v| v.principle == "Single Responsibility")
            .collect();
        
        if !srp_violations.is_empty() {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for violation in srp_violations {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    violation.file,
                    violation.line,
                    violation.description,
                    violation.fix_suggestion));
            }
        } else {
            report.push_str("Found **0** potential violations.\n");
        }
        report.push_str("\n");
        
        // Open-Closed Principle
        report.push_str("## Open-Closed Principle Violations\n\n");
        report.push_str("Software entities should be open for extension, but closed for modification.\n\n");
        
        let ocp_violations: Vec<_> = metrics.solid_violations.iter()
            .filter(|v| v.principle == "Open-Closed")
            .collect();
        
        if !ocp_violations.is_empty() {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for violation in ocp_violations {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    violation.file,
                    violation.line,
                    violation.description,
                    violation.fix_suggestion));
            }
        } else {
            report.push_str("Found **0** potential violations.\n");
        }
        report.push_str("\n");
        
        // Liskov Substitution Principle
        report.push_str("## Liskov Substitution Principle Violations\n\n");
        report.push_str("Subtypes must be substitutable for their base types.\n\n");
        
        let lsp_violations: Vec<_> = metrics.solid_violations.iter()
            .filter(|v| v.principle == "Liskov Substitution")
            .collect();
        
        if !lsp_violations.is_empty() {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for violation in lsp_violations {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    violation.file,
                    violation.line,
                    violation.description,
                    violation.fix_suggestion));
            }
        } else {
            report.push_str("Found **0** potential violations.\n");
        }
        report.push_str("\n");
        
        // Interface Segregation Principle
        report.push_str("## Interface Segregation Principle Violations\n\n");
        report.push_str("Clients should not be forced to depend on methods they do not use.\n\n");
        
        let isp_violations: Vec<_> = metrics.solid_violations.iter()
            .filter(|v| v.principle == "Interface Segregation")
            .collect();
        
        if !isp_violations.is_empty() {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for violation in isp_violations {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    violation.file,
                    violation.line,
                    violation.description,
                    violation.fix_suggestion));
            }
        } else {
            report.push_str("Found **0** potential violations.\n");
        }
        report.push_str("\n");
        
        // Dependency Inversion Principle
        report.push_str("## Dependency Inversion Principle Violations\n\n");
        report.push_str("High-level modules should not depend on low-level modules. Both should depend on abstractions.\n\n");
        
        let dip_violations: Vec<_> = metrics.solid_violations.iter()
            .filter(|v| v.principle == "Dependency Inversion")
            .collect();
        
        if !dip_violations.is_empty() {
            report.push_str("| File | Line | Description | Fix Suggestion |\n");
            report.push_str("|------|------|-------------|---------------|\n");
            
            for violation in dip_violations {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    violation.file,
                    violation.line,
                    violation.description,
                    violation.fix_suggestion));
            }
        } else {
            report.push_str("Found **0** potential violations.\n");
        }
        report.push_str("\n");
        
        // Design Pattern Implementations
        report.push_str("# Design Pattern Implementations\n\n");
        
        if !metrics.pattern_implementations.is_empty() {
            report.push_str("| Pattern | File | Line | Description |\n");
            report.push_str("|---------|------|------|-------------|\n");
            
            for implementation in &metrics.pattern_implementations {
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    implementation.pattern,
                    implementation.file,
                    implementation.line,
                    implementation.description));
            }
        } else {
            report.push_str("No design pattern implementations found.\n");
        }
        report.push_str("\n");
        
        // Design Pattern Violations
        report.push_str("# Design Pattern Violations\n\n");
        
        if !metrics.pattern_violations.is_empty() {
            report.push_str("| Pattern | File | Line | Description | Fix Suggestion |\n");
            report.push_str("|---------|------|------|-------------|---------------|\n");
            
            for violation in &metrics.pattern_violations {
                report.push_str(&format!("| {} | {} | {} | {} | {} |\n",
                    violation.pattern,
                    violation.file,
                    violation.line,
                    violation.description,
                    violation.fix_suggestion));
            }
        } else {
            report.push_str("No design pattern violations found.\n");
        }
        
        Ok(report)
    }
}

/// Extract comment text from a line
fn extract_comment(line: &str) -> String {
    if let Some(comment_start) = line.find("//") {
        let comment = line[comment_start + 2..].trim();
        
        // Remove marker prefix
        if comment.starts_with("SRP violation:") {
            comment[13..].trim().to_string()
        } else if comment.starts_with("OCP violation:") {
            comment[13..].trim().to_string()
        } else if comment.starts_with("LSP violation:") {
            comment[13..].trim().to_string()
        } else if comment.starts_with("ISP violation:") {
            comment[13..].trim().to_string()
        } else if comment.starts_with("DIP violation:") {
            comment[13..].trim().to_string()
        } else if comment.starts_with("SRP violation") {
            comment[12..].trim().to_string()
        } else if comment.starts_with("OCP violation") {
            comment[12..].trim().to_string()
        } else if comment.starts_with("LSP violation") {
            comment[12..].trim().to_string()
        } else if comment.starts_with("ISP violation") {
            comment[12..].trim().to_string()
        } else if comment.starts_with("DIP violation") {
            comment[12..].trim().to_string()
        } else if comment.starts_with("Factory pattern:") {
            comment[16..].trim().to_string()
        } else if comment.starts_with("Singleton pattern:") {
            comment[18..].trim().to_string()
        } else if comment.starts_with("Observer pattern:") {
            comment[17..].trim().to_string()
        } else if comment.starts_with("Strategy pattern:") {
            comment[17..].trim().to_string()
        } else if comment.starts_with("Command pattern:") {
            comment[16..].trim().to_string()
        } else if comment.starts_with("Repository pattern:") {
            comment[19..].trim().to_string()
        } else if comment.starts_with("Dependency injection:") {
            comment[21..].trim().to_string()
        } else if comment.starts_with("Factory pattern") {
            comment[15..].trim().to_string()
        } else if comment.starts_with("Singleton pattern") {
            comment[17..].trim().to_string()
        } else if comment.starts_with("Observer pattern") {
            comment[16..].trim().to_string()
        } else if comment.starts_with("Strategy pattern") {
            comment[16..].trim().to_string()
        } else if comment.starts_with("Command pattern") {
            comment[15..].trim().to_string()
        } else if comment.starts_with("Repository pattern") {
            comment[18..].trim().to_string()
        } else if comment.starts_with("Dependency injection") {
            comment[20..].trim().to_string()
        } else if comment.starts_with("Factory pattern violation:") {
            comment[26..].trim().to_string()
        } else if comment.starts_with("Singleton pattern violation:") {
            comment[28..].trim().to_string()
        } else if comment.starts_with("Observer pattern violation:") {
            comment[27..].trim().to_string()
        } else if comment.starts_with("Strategy pattern violation:") {
            comment[27..].trim().to_string()
        } else if comment.starts_with("Command pattern violation:") {
            comment[26..].trim().to_string()
        } else if comment.starts_with("Repository pattern violation:") {
            comment[29..].trim().to_string()
        } else if comment.starts_with("Dependency injection violation:") {
            comment[31..].trim().to_string()
        } else if comment.starts_with("Factory pattern violation") {
            comment[25..].trim().to_string()
        } else if comment.starts_with("Singleton pattern violation") {
            comment[27..].trim().to_string()
        } else if comment.starts_with("Observer pattern violation") {
            comment[26..].trim().to_string()
        } else if comment.starts_with("Strategy pattern violation") {
            comment[26..].trim().to_string()
        } else if comment.starts_with("Command pattern violation") {
            comment[25..].trim().to_string()
        } else if comment.starts_with("Repository pattern violation") {
            comment[28..].trim().to_string()
        } else if comment.starts_with("Dependency injection violation") {
            comment[30..].trim().to_string()
        } else {
            comment.to_string()
        }
    } else {
        String::new()
    }
}

/// Get the full name of a SOLID principle
fn get_solid_principle_name(principle: &str) -> String {
    match principle {
        "single_responsibility" => "Single Responsibility".to_string(),
        "open_closed" => "Open-Closed".to_string(),
        "liskov_substitution" => "Liskov Substitution".to_string(),
        "interface_segregation" => "Interface Segregation".to_string(),
        "dependency_inversion" => "Dependency Inversion".to_string(),
        _ => principle.to_string(),
    }
}

/// Get a fix suggestion for a SOLID principle violation
fn get_solid_fix_suggestion(principle: &str) -> String {
    match principle {
        "single_responsibility" => "Split the class into multiple classes, each with a single responsibility".to_string(),
        "open_closed" => "Use interfaces or abstract classes to allow for extension without modification".to_string(),
        "liskov_substitution" => "Ensure that derived classes can be used in place of their base classes".to_string(),
        "interface_segregation" => "Split large interfaces into smaller, more specific ones".to_string(),
        "dependency_inversion" => "Depend on abstractions, not on concrete implementations".to_string(),
        _ => "Fix the violation".to_string(),
    }
}

/// Get the full name of a design pattern
fn get_pattern_name(pattern: &str) -> String {
    match pattern {
        "factory" => "Factory".to_string(),
        "singleton" => "Singleton".to_string(),
        "observer" => "Observer".to_string(),
        "strategy" => "Strategy".to_string(),
        "command" => "Command".to_string(),
        "repository" => "Repository".to_string(),
        "dependency_injection" => "Dependency Injection".to_string(),
        _ => pattern.to_string(),
    }
}

/// Get a fix suggestion for a design pattern violation
fn get_pattern_fix_suggestion(pattern: &str) -> String {
    match pattern {
        "factory" => "Use a factory method to create objects".to_string(),
        "singleton" => "Ensure that a class has only one instance and provide a global point of access to it".to_string(),
        "observer" => "Define a one-to-many dependency between objects so that when one object changes state, all its dependents are notified and updated automatically".to_string(),
        "strategy" => "Define a family of algorithms, encapsulate each one, and make them interchangeable".to_string(),
        "command" => "Encapsulate a request as an object, thereby letting you parameterize clients with different requests, queue or log requests, and support undoable operations".to_string(),
        "repository" => "Separate the logic that retrieves data from the underlying storage from the business logic that acts on the data".to_string(),
        "dependency_injection" => "Pass dependencies to objects instead of having objects create or find their dependencies".to_string(),
        _ => "Fix the violation".to_string(),
    }
}
