use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

// Foreign function declarations
extern "C" {
    fn hs_parse_completion_rule(rule_text: *const c_char) -> *mut c_char;
    fn hs_free_string(ptr: *mut c_char);
    
    fn hs_parse_query(query_text: *const c_char) -> *mut c_char;
    fn hs_optimize_query(query_json: *const c_char) -> *mut c_char;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Requirement {
    CompleteAssignment { assignment_id: String },
    ScoreAbove { percentage: f64, assignment_id: String },
    And { requirements: Vec<Requirement> },
    Or { requirements: Vec<Requirement> },
    Not { requirement: Box<Requirement> },
    CompleteAllModules,
    MinimumPostCount { count: i32 },
}

pub struct CompletionRuleParser;

impl CompletionRuleParser {
    /// Parse a completion rule from a DSL string
    pub fn parse_rule(&self, rule_text: &str) -> Result<Requirement> {
        let c_rule_text = CString::new(rule_text)?;
        
        // Call Haskell parser
        let result_ptr = unsafe { hs_parse_completion_rule(c_rule_text.as_ptr()) };
        if result_ptr.is_null() {
            return Err(anyhow!("Failed to parse completion rule"));
        }
        
        // Convert result to Rust
        let result = unsafe {
            let result_str = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            // Free the string allocated by Haskell
            hs_free_string(result_ptr);
            result_str
        };
        
        // Parse JSON into Requirement
        serde_json::from_str(&result)
            .map_err(|e| anyhow!("Failed to parse JSON result: {}", e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub query_type: String,
    pub tables: Vec<String>,
    pub conditions: Vec<Condition>,
    pub projections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Equals { left: String, right: String },
    LessThan { left: String, right: String },
    GreaterThan { left: String, right: String },
}

pub struct QueryParser;

impl QueryParser {
    /// Parse a SQL-like query string into a Query object
    pub fn parse_query(&self, query_text: &str) -> Result<Query> {
        let c_query_text = CString::new(query_text)?;
        
        // Call Haskell parser
        let result_ptr = unsafe { hs_parse_query(c_query_text.as_ptr()) };
        if result_ptr.is_null() {
            return Err(anyhow!("Failed to parse query"));
        }
        
        // Convert result to Rust
        let result = unsafe {
            let result_str = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            // Free the string allocated by Haskell
            hs_free_string(result_ptr);
            result_str
        };
        
        // Parse JSON into Query
        serde_json::from_str(&result)
            .map_err(|e| anyhow!("Failed to parse JSON result: {}", e))
    }
    
    /// Optimize a query using Haskell query optimizer
    pub fn optimize_query(&self, query: &Query) -> Result<Query> {
        let query_json = serde_json::to_string(query)?;
        let c_query_json = CString::new(query_json)?;
        
        // Call Haskell optimizer
        let result_ptr = unsafe { hs_optimize_query(c_query_json.as_ptr()) };
        if result_ptr.is_null() {
            return Err(anyhow!("Failed to optimize query"));
        }
        
        // Convert result to Rust
        let result = unsafe {
            let result_str = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            // Free the string allocated by Haskell
            hs_free_string(result_ptr);
            result_str
        };
        
        // Parse JSON into Query
        serde_json::from_str(&result)
            .map_err(|e| anyhow!("Failed to parse JSON result: {}", e))
    }
}