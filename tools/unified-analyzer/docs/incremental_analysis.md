# Implementing Incremental Analysis for Analyzers

This document provides a guide on how to implement incremental analysis for analyzers in the unified analyzer system. Incremental analysis significantly improves performance for large codebases by only analyzing files that have changed since the last analysis.

## Overview

Incremental analysis works by:

1. Caching analysis results for each file
2. Checking if a file has changed since the last analysis
3. Only re-analyzing files that have changed
4. Combining cached results with new results

## Implementation Steps

To implement incremental analysis for an analyzer, follow these steps:

### 1. Define Your Analysis Result Structure

Create a serializable structure that represents the result of your analysis:

```rust
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct YourAnalysisResult {
    // Add your analysis result fields here
    pub some_data: Vec<String>,
    pub some_metrics: HashMap<String, f32>,
    // ... other fields
}
```

### 2. Create Your Incremental Analyzer

Create a structure for your analyzer that includes the necessary fields for incremental analysis:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct YourIncrementalAnalyzer {
    pub base_dir: PathBuf,
    pub use_incremental: bool,
    pub cache_path: Option<PathBuf>,
    pub exclude_dirs: Vec<String>,
    pub include_extensions: Vec<String>,
    // ... other configuration fields
}
```

### 3. Implement the IncrementalAnalyzer Trait

Implement the `IncrementalAnalyzer` trait for your analyzer:

```rust
impl IncrementalAnalyzer<YourAnalysisResult> for YourIncrementalAnalyzer {
    fn base_dir(&self) -> &Path {
        &self.base_dir
    }
    
    fn cache_path(&self) -> Option<&Path> {
        self.cache_path.as_deref()
    }
    
    fn use_incremental(&self) -> bool {
        self.use_incremental
    }
    
    fn config_hash(&self) -> String {
        use crate::utils::incremental_analyzer::calculate_hash;
        
        // Create a simple configuration object for hashing
        let config = (
            &self.exclude_dirs,
            &self.include_extensions,
            // ... other configuration fields that affect analysis
        );
        
        calculate_hash(&config)
    }
    
    fn should_exclude_file(&self, file_path: &Path) -> bool {
        // Implement file exclusion logic
    }
    
    fn analyze_file(&self, file_path: &Path) -> Result<YourAnalysisResult> {
        // Implement file analysis logic
    }
}
```

### 4. Implement the Main Analysis Method

Implement the main analysis method that uses incremental analysis:

```rust
pub fn analyze(&self) -> Result<YourAnalysisResult> {
    // Collect all files to analyze
    let files_to_analyze = crate::utils::incremental_analyzer::collect_files_for_analysis(
        &self.base_dir,
        &self.include_extensions,
        &self.exclude_dirs,
        |path| self.should_exclude_file(path)
    );
    
    // Analyze files incrementally
    let file_results = self.analyze_files_incrementally(&files_to_analyze)?;
    
    // Combine results
    let mut combined_result = YourAnalysisResult::default();
    
    for result in file_results {
        // Combine the results from each file
    }
    
    // Post-process the combined results if needed
    self.post_process_results(&mut combined_result);
    
    Ok(combined_result)
}
```

### 5. Add Report Generation Methods

Add methods to generate reports from your analysis results:

```rust
pub fn generate_report(&self, result: &YourAnalysisResult) -> Result<String> {
    // Generate a markdown report
}

pub fn export_to_json(&self, result: &YourAnalysisResult) -> Result<String> {
    // Export to JSON
}
```

### 6. Add Tests

Add tests for your incremental analyzer:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_specific_file_type() -> Result<()> {
        // Test file analysis
    }
    
    #[test]
    fn test_incremental_analysis() -> Result<()> {
        // Test incremental analysis
    }
}
```

## Template

A template for implementing incremental analysis is available at:

```
src/utils/incremental_analyzer_template.rs
```

You can use this template as a starting point for implementing incremental analysis in your analyzer.

## Example Implementations

For reference, the following analyzers have been implemented with incremental analysis:

1. **EnhancedTechDebtAnalyzer**: Analyzes technical debt in the codebase
2. **IncrementalApiAnalyzer**: Analyzes API endpoints and clients

## Benefits of Incremental Analysis

Implementing incremental analysis provides several benefits:

1. **Faster Subsequent Runs**: After the initial analysis, subsequent runs will be much faster as only changed files are analyzed.
2. **Reduced Resource Usage**: Less CPU and memory are used during analysis since fewer files need to be processed.
3. **Scalability for Large Codebases**: The analyzer can handle very large codebases efficiently, as the analysis time scales with the number of changed files rather than the total number of files.
4. **Efficient CI/CD Integration**: When integrated into CI/CD pipelines, the analyzer will only process files that have changed in the current commit, making it practical to run on every commit.

## Best Practices

1. **Enable Incremental Analysis by Default**: Set `use_incremental` to `true` by default to ensure optimal performance.
2. **Use a Meaningful Cache Path**: Set the cache path to a location that makes sense for your analyzer, typically in the base directory with a name that reflects the analyzer.
3. **Implement Proper File Exclusion**: Make sure to exclude files that don't need to be analyzed to avoid unnecessary processing.
4. **Handle Cache Invalidation**: Implement proper cache invalidation by including all relevant configuration in the `config_hash` method.
5. **Add Comprehensive Tests**: Add tests that verify both the analysis logic and the incremental analysis functionality.
