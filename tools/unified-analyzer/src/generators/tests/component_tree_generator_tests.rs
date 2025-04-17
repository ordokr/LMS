#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::collections::HashMap;
    use std::fs;
    
    use crate::generators::ComponentTreeGenerator;
    use crate::output_schema::{UnifiedAnalysisOutput, ComponentInfo};

    #[test]
    fn test_component_tree_generator_initialization() {
        let generator = ComponentTreeGenerator::new();
        assert!(generator.generate(&create_test_output(), &PathBuf::from("./test_output")).is_err());
    }

    #[test]
    fn test_determine_group() {
        let generator = ComponentTreeGenerator::new();
        
        // Create test components
        let mut ui_component = ComponentInfo::default();
        ui_component.name = "Button".to_string();
        ui_component.file_path = "src/components/Button.js".to_string();
        ui_component.framework = "React".to_string();
        
        let mut model_component = ComponentInfo::default();
        model_component.name = "User".to_string();
        model_component.file_path = "src/models/User.js".to_string();
        model_component.framework = "React".to_string();
        
        let mut service_component = ComponentInfo::default();
        service_component.name = "AuthService".to_string();
        service_component.file_path = "src/services/AuthService.js".to_string();
        service_component.framework = "React".to_string();
        
        let mut util_component = ComponentInfo::default();
        util_component.name = "DateUtil".to_string();
        util_component.file_path = "src/utils/DateUtil.js".to_string();
        util_component.framework = "React".to_string();
        
        let mut other_component = ComponentInfo::default();
        other_component.name = "Unknown".to_string();
        other_component.file_path = "src/unknown/Unknown.js".to_string();
        other_component.framework = "React".to_string();
        
        // Test group determination
        assert_eq!(generator.determine_group(&ui_component), 1);
        assert_eq!(generator.determine_group(&model_component), 2);
        assert_eq!(generator.determine_group(&service_component), 3);
        assert_eq!(generator.determine_group(&util_component), 4);
        assert_eq!(generator.determine_group(&other_component), 5);
    }

    #[test]
    fn test_generate_markdown() {
        let generator = ComponentTreeGenerator::new();
        let output = create_test_output();
        
        let result = generator.generate_markdown(&output);
        assert!(result.is_ok());
        
        let markdown = result.unwrap();
        assert!(markdown.contains("# Component Tree"));
        assert!(markdown.contains("## Components"));
        assert!(markdown.contains("## Component Relationships"));
    }

    #[test]
    fn test_template_loading() {
        // Create a temporary template file
        let temp_dir = tempfile::tempdir().unwrap();
        let template_path = temp_dir.path().join("component_tree_template.html");
        
        fs::write(&template_path, r#"<!DOCTYPE html>
<html>
<head>
    <title>Component Tree</title>
</head>
<body>
    <!-- GRAPH_DATA_PLACEHOLDER -->
</body>
</html>"#).unwrap();
        
        // Test loading the template
        let template = fs::read_to_string(&template_path).unwrap();
        assert!(template.contains("<!-- GRAPH_DATA_PLACEHOLDER -->"));
    }

    // Helper function to create a test output
    fn create_test_output() -> UnifiedAnalysisOutput {
        let mut output = UnifiedAnalysisOutput::default();
        
        // Add some components
        let mut components = HashMap::new();
        
        let mut button_component = ComponentInfo::default();
        button_component.name = "Button".to_string();
        button_component.file_path = "src/components/Button.js".to_string();
        button_component.framework = "React".to_string();
        button_component.dependencies = vec!["Icon".to_string()];
        
        let mut icon_component = ComponentInfo::default();
        icon_component.name = "Icon".to_string();
        icon_component.file_path = "src/components/Icon.js".to_string();
        icon_component.framework = "React".to_string();
        icon_component.dependencies = vec![];
        
        let mut app_component = ComponentInfo::default();
        app_component.name = "App".to_string();
        app_component.file_path = "src/App.js".to_string();
        app_component.framework = "React".to_string();
        app_component.dependencies = vec!["Button".to_string(), "Icon".to_string()];
        
        components.insert("Button".to_string(), button_component);
        components.insert("Icon".to_string(), icon_component);
        components.insert("App".to_string(), app_component);
        
        output.components = components;
        
        output
    }
}
