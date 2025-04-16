rust
use std::fs;
use std::path::PathBuf;
use crate::analyzers::modules::ember_analyzer::{EmberAnalyzer};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_model_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("models")).unwrap();

        let model_path = temp_dir.path().join("models").join("user.js");
        fs::write(&model_path, "@attr 'name'\n hasMany 'posts'").unwrap();

        let mut analyzer = EmberAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: EmberAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.models.contains_key("user"));
        let user_model = analyzer.models.get("user").unwrap();
        assert_eq!(user_model.attributes, vec!["name"]);
        assert_eq!(user_model.relationships, vec!["posts"]);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_controller_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("controllers")).unwrap();

        let controller_path = temp_dir.path().join("controllers").join("posts.js");
        fs::write(&controller_path, "action: 'index'\naction: 'show'").unwrap();

        let mut analyzer = EmberAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: EmberAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.controllers.contains_key("posts"));
        let posts_controller = analyzer.controllers.get("posts").unwrap();
        assert_eq!(posts_controller.actions, vec!["index", "show"]);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_component_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("components")).unwrap();

        let component_path = temp_dir.path().join("components").join("my-component.js");
        fs::write(&component_path, "attribute: 'title'\naction: 'toggle'").unwrap();

        let mut analyzer = EmberAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: EmberAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.components.contains_key("my-component"));
        let my_component = analyzer.components.get("my-component").unwrap();
        assert_eq!(my_component.properties, vec!["title"]);
        assert_eq!(my_component.actions, vec!["toggle"]);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_route_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("routes")).unwrap();

        let route_path = temp_dir.path().join("routes").join("posts.js");
        fs::write(&route_path, "path: '/posts'\nmodel: 'post'").unwrap();

        let mut analyzer = EmberAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: EmberAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.routes.contains_key("posts"));
        let posts_route = analyzer.routes.get("posts").unwrap();
        assert_eq!(posts_route.path, "/posts");
        assert_eq!(posts_route.model, Some("post".to_string()));

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_service_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("services")).unwrap();

        let service_path = temp_dir.path().join("services").join("user.js");
        fs::write(&service_path, "method: 'login'\nmethod: 'logout'").unwrap();

        let mut analyzer = EmberAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: EmberAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.services.contains_key("user"));
        let user_service = analyzer.services.get("user").unwrap();
        assert_eq!(user_service.methods, vec!["login", "logout"]);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_helper_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("helpers")).unwrap();

        let helper_path = temp_dir.path().join("helpers").join("format-date.js");
        fs::write(&helper_path, "function: 'format'").unwrap();

        let mut analyzer = EmberAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: EmberAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.helpers.contains_key("format-date"));
        let format_date_helper = analyzer.helpers.get("format-date").unwrap();
        assert_eq!(format_date_helper.functions, vec!["format"]);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_template_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();

        let template_path = temp_dir.path().join("post.hbs");
        fs::write(&template_path, "{{title}}\n{{body}}\n{{user.name}}").unwrap();

        let mut analyzer = EmberAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: EmberAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.templates.contains_key("post"));
        let post_template = analyzer.templates.get("post").unwrap();
        assert_eq!(post_template.bindings, vec!["title", "body", "user.name"]);

        temp_dir.close().unwrap();
    }
}