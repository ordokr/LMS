use regex::Regex;
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    static ref NONALNUM: Regex = Regex::new(r"[^a-zA-Z0-9\s-]").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
    static ref EDGEDASHES: Regex = Regex::new(r"^-|-$").unwrap();
    static ref MULTIPLEDASHES: Regex = Regex::new(r"-+").unwrap();
}

/// Convert a string to a URL-friendly slug
pub fn slugify(text: &str) -> String {
    let slug = NONALNUM.replace_all(text.to_lowercase().as_str(), "");
    let slug = WHITESPACE.replace_all(slug.as_ref(), "-");
    let slug = EDGEDASHES.replace_all(slug.as_ref(), "");
    let slug = MULTIPLEDASHES.replace_all(slug.as_ref(), "-");
    
    let result = slug.to_string();
    
    // If the slug is empty, generate a random one
    if result.is_empty() {
        format!("topic-{}", Uuid::new_v4().to_string().split('-').next().unwrap())
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("Hello  -  World"), "hello-world");
        assert_eq!(slugify("HELLO WORLD"), "hello-world");
        assert_eq!(slugify("hello-world"), "hello-world");
        assert_eq!(slugify("--hello-world--"), "hello-world");
        assert_eq!(slugify("  spaces  "), "spaces");
        assert_eq!(slugify(""), "topic-");
        assert_eq!(slugify("!!!"), "topic-");
    }
}