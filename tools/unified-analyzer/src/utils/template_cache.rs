use std::fs;

/// Gets a template from disk or uses the embedded template as fallback.
///
/// # Arguments
/// * `path` - The path to the template file
/// * `embedded` - An optional embedded template to use as fallback
///
/// # Returns
/// * `Ok(String)` - The template content
/// * `Err(String)` - An error message if the template could not be loaded
pub fn get_template(path: &str, embedded: Option<&str>) -> Result<String, String> {
    // Load from disk
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => {
            // Try to use the embedded template if file not found
            if let Some(embedded_content) = embedded {
                Ok(embedded_content.to_string())
            } else {
                Err(format!("Failed to load template file: {}. Error: {}", path, e))
            }
        }
    }
}
