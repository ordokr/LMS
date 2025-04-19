use regex::Regex;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use crate::errors::error::{Error, Result};

/// Capitalize the first letter of a string
/// 
/// # Arguments
/// * `s` - The string to capitalize
/// 
/// # Returns
/// * `String` - The capitalized string
pub fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert a string to lowercase
/// 
/// # Arguments
/// * `s` - The string to convert
/// 
/// # Returns
/// * `String` - The lowercase string
pub fn lowercase(s: &str) -> String {
    s.to_lowercase()
}

/// Convert a string to uppercase
/// 
/// # Arguments
/// * `s` - The string to convert
/// 
/// # Returns
/// * `String` - The uppercase string
pub fn uppercase(s: &str) -> String {
    s.to_uppercase()
}

/// Convert a string to title case
/// 
/// # Arguments
/// * `s` - The string to convert
/// 
/// # Returns
/// * `String` - The title case string
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(capitalize)
        .collect::<Vec<String>>()
        .join(" ")
}

/// Trim whitespace from both ends of a string
/// 
/// # Arguments
/// * `s` - The string to trim
/// 
/// # Returns
/// * `String` - The trimmed string
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

/// Trim whitespace from the start of a string
/// 
/// # Arguments
/// * `s` - The string to trim
/// 
/// # Returns
/// * `String` - The trimmed string
pub fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
}

/// Trim whitespace from the end of a string
/// 
/// # Arguments
/// * `s` - The string to trim
/// 
/// # Returns
/// * `String` - The trimmed string
pub fn trim_end(s: &str) -> String {
    s.trim_end().to_string()
}

/// Truncate a string to a maximum length
/// 
/// # Arguments
/// * `s` - The string to truncate
/// * `max_length` - The maximum length
/// * `suffix` - The suffix to add if truncated (optional)
/// 
/// # Returns
/// * `String` - The truncated string
pub fn truncate(s: &str, max_length: usize, suffix: Option<&str>) -> String {
    if s.graphemes(true).count() <= max_length {
        return s.to_string();
    }
    
    let suffix = suffix.unwrap_or("...");
    let truncated_len = max_length.saturating_sub(suffix.len());
    
    let truncated: String = s.graphemes(true)
        .take(truncated_len)
        .collect();
    
    truncated + suffix
}

/// Check if a string is empty
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// * `bool` - True if the string is empty
pub fn is_empty(s: &str) -> bool {
    s.is_empty()
}

/// Check if a string is blank (empty or only whitespace)
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// * `bool` - True if the string is blank
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// Check if a string is numeric
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// * `bool` - True if the string is numeric
pub fn is_numeric(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    s.chars().all(|c| c.is_numeric())
}

/// Check if a string is alphanumeric
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// * `bool` - True if the string is alphanumeric
pub fn is_alphanumeric(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    s.chars().all(|c| c.is_alphanumeric())
}

/// Check if a string contains a substring
/// 
/// # Arguments
/// * `s` - The string to check
/// * `substring` - The substring to look for
/// 
/// # Returns
/// * `bool` - True if the string contains the substring
pub fn contains(s: &str, substring: &str) -> bool {
    s.contains(substring)
}

/// Check if a string starts with a prefix
/// 
/// # Arguments
/// * `s` - The string to check
/// * `prefix` - The prefix to look for
/// 
/// # Returns
/// * `bool` - True if the string starts with the prefix
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// Check if a string ends with a suffix
/// 
/// # Arguments
/// * `s` - The string to check
/// * `suffix` - The suffix to look for
/// 
/// # Returns
/// * `bool` - True if the string ends with the suffix
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Replace all occurrences of a substring
/// 
/// # Arguments
/// * `s` - The string to modify
/// * `from` - The substring to replace
/// * `to` - The replacement
/// 
/// # Returns
/// * `String` - The modified string
pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// Split a string by a delimiter
/// 
/// # Arguments
/// * `s` - The string to split
/// * `delimiter` - The delimiter
/// 
/// # Returns
/// * `Vec<String>` - The split string
pub fn split(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter)
        .map(|s| s.to_string())
        .collect()
}

/// Join strings with a delimiter
/// 
/// # Arguments
/// * `strings` - The strings to join
/// * `delimiter` - The delimiter
/// 
/// # Returns
/// * `String` - The joined string
pub fn join(strings: &[String], delimiter: &str) -> String {
    strings.join(delimiter)
}

/// Format a template string with variables
/// 
/// # Arguments
/// * `template` - The template string with {variable} placeholders
/// * `variables` - The variables to substitute
/// 
/// # Returns
/// * `Result<String>` - The formatted string or an error
pub fn format_template(template: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut result = template.to_string();
    
    for (key, value) in variables {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    
    // Check if there are any remaining placeholders
    let re = Regex::new(r"\{([^}]+)\}")
        .map_err(|e| Error::internal(format!("Invalid regex pattern: {}", e)))?;
    
    if re.is_match(&result) {
        let missing_vars: Vec<String> = re.captures_iter(&result)
            .map(|cap| cap[1].to_string())
            .collect();
        
        return Err(Error::validation(format!("Missing variables in template: {}", missing_vars.join(", "))));
    }
    
    Ok(result)
}

/// Convert a string to a URL-friendly slug
/// 
/// # Arguments
/// * `s` - The string to convert
/// 
/// # Returns
/// * `String` - The slug
pub fn slugify(s: &str) -> String {
    // Convert to lowercase
    let s = s.to_lowercase();
    
    // Replace non-alphanumeric characters with hyphens
    let re = Regex::new(r"[^a-z0-9]+").unwrap();
    let s = re.replace_all(&s, "-").to_string();
    
    // Remove leading and trailing hyphens
    let s = s.trim_matches('-').to_string();
    
    // Collapse multiple hyphens
    let re = Regex::new(r"-{2,}").unwrap();
    let s = re.replace_all(&s, "-").to_string();
    
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize("Hello"), "Hello");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("h"), "H");
        assert_eq!(capitalize("123"), "123");
    }
    
    #[test]
    fn test_case_conversion() {
        assert_eq!(lowercase("Hello World"), "hello world");
        assert_eq!(uppercase("Hello World"), "HELLO WORLD");
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("HELLO WORLD"), "Hello World");
        assert_eq!(title_case("hello-world"), "Hello-world");
    }
    
    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
        assert_eq!(trim_start("  hello  "), "hello  ");
        assert_eq!(trim_end("  hello  "), "  hello");
    }
    
    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello, world!", 5, None), "Hello...");
        assert_eq!(truncate("Hello, world!", 5, Some("...")), "Hello...");
        assert_eq!(truncate("Hello, world!", 5, Some("…")), "Hello…");
        assert_eq!(truncate("Hello", 10, None), "Hello");
    }
    
    #[test]
    fn test_is_checks() {
        assert!(is_empty(""));
        assert!(!is_empty("hello"));
        
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(!is_blank("hello"));
        
        assert!(is_numeric("123"));
        assert!(!is_numeric("123a"));
        assert!(!is_numeric(""));
        
        assert!(is_alphanumeric("abc123"));
        assert!(!is_alphanumeric("abc 123"));
        assert!(!is_alphanumeric(""));
    }
    
    #[test]
    fn test_string_operations() {
        assert!(contains("Hello, world!", "world"));
        assert!(!contains("Hello, world!", "universe"));
        
        assert!(starts_with("Hello, world!", "Hello"));
        assert!(!starts_with("Hello, world!", "hello"));
        
        assert!(ends_with("Hello, world!", "world!"));
        assert!(!ends_with("Hello, world!", "World!"));
        
        assert_eq!(replace_all("Hello, world!", "world", "universe"), "Hello, universe!");
        
        let parts = split("Hello, world!", ", ");
        assert_eq!(parts, vec!["Hello", "world!"]);
        
        assert_eq!(join(&vec!["Hello".to_string(), "world!".to_string()], ", "), "Hello, world!");
    }
    
    #[test]
    fn test_format_template() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "John".to_string());
        variables.insert("age".to_string(), "30".to_string());
        
        let template = "Hello, {name}! You are {age} years old.";
        let result = format_template(template, &variables).unwrap();
        assert_eq!(result, "Hello, John! You are 30 years old.");
        
        let template = "Hello, {name}! You are {age} years old and live in {city}.";
        let result = format_template(template, &variables);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello, world!"), "hello-world");
        assert_eq!(slugify("Hello   World"), "hello-world");
        assert_eq!(slugify("Hello-World"), "hello-world");
        assert_eq!(slugify("Hello_World"), "hello-world");
        assert_eq!(slugify("hello-world-"), "hello-world");
        assert_eq!(slugify("-hello-world"), "hello-world");
        assert_eq!(slugify("hello--world"), "hello-world");
        assert_eq!(slugify("héllö wörld"), "h-ll-w-rld");
    }
}
