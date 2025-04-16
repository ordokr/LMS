rust
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::analyzers::modules::ruby_rails_analyzer::{RubyRailsAnalyzer, Route, DatabaseSchema, Column, Callback, Hook};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_model_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();

        let model_path = temp_dir.path().join("user.rb");
        fs::write(model_path, "class User < ApplicationRecord\n  has_many :posts\n  validates :name\nend").unwrap();

        let analyzer = RubyRailsAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: RubyRailsAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.models.contains_key("User"));
        let user_model = analyzer.models.get("User").unwrap();
        assert_eq!(user_model.associations, vec!["posts"]);
        assert_eq!(user_model.validations, vec![":name"]);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_controller_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();

        let controller_path = temp_dir.path().join("posts_controller.rb");
        fs::write(controller_path, "class PostsController < ApplicationController\n  def index\n  end\n  def show\n  end\nend").unwrap();

        let analyzer = RubyRailsAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: RubyRailsAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.controllers.contains_key("PostsController"));
        let posts_controller = analyzer.controllers.get("PostsController").unwrap();
        assert_eq!(posts_controller.actions, vec!["index", "show"]);

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_route_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("config")).unwrap();

        let routes_path = temp_dir.path().join("config").join("routes.rb");
        fs::write(routes_path, "Rails.application.routes.draw do\n  get '/posts', to: 'posts#index'\n  post '/users', to: \"users#create\"\nend").unwrap();

        let analyzer = RubyRailsAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: RubyRailsAnalyzer = serde_json::from_str(&result).unwrap();

        assert_eq!(analyzer.routes.len(), 2);
        assert_eq!(analyzer.routes[0].verb, "get");
        assert_eq!(analyzer.routes[0].path, "/posts");
        assert_eq!(analyzer.routes[0].controller, "posts");
        assert_eq!(analyzer.routes[0].action, "index");

        assert_eq!(analyzer.routes[1].verb, "post");
        assert_eq!(analyzer.routes[1].path, "/users");
        assert_eq!(analyzer.routes[1].controller, "users");
        assert_eq!(analyzer.routes[1].action, "create");

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_database_schema_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();
        fs::create_dir(temp_dir.path().join("db")).unwrap();

        let schema_path = temp_dir.path().join("db").join("schema.rb");
        fs::write(schema_path, "ActiveRecord::Schema.define(version: 2023_01_01_000000) do\n  create_table \"users\" do |t|\n    t.string \"name\"\n    t.integer \"age\", default: 0\n    t.datetime \"created_at\", null: false\n    t.datetime \"updated_at\", null: false\n  end\nend").unwrap();

        let analyzer = RubyRailsAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: RubyRailsAnalyzer = serde_json::from_str(&result).unwrap();

        assert!(analyzer.database_schemas.contains_key("users"));
        let user_schema = analyzer.database_schemas.get("users").unwrap();
        assert_eq!(user_schema.columns.len(), 4);

        assert_eq!(user_schema.columns[0].name, "name");
        assert_eq!(user_schema.columns[0].r#type, "string");

        assert_eq!(user_schema.columns[1].name, "age");
        assert_eq!(user_schema.columns[1].r#type, "integer");
        assert_eq!(user_schema.columns[1].options.get("default"), Some(&"0".to_string()));

        assert_eq!(user_schema.columns[2].name, "created_at");
        assert_eq!(user_schema.columns[2].r#type, "datetime");
        assert_eq!(user_schema.columns[2].options.get("null"), Some(&"false".to_string()));

        assert_eq!(user_schema.columns[3].name, "updated_at");
        assert_eq!(user_schema.columns[3].r#type, "datetime");
        assert_eq!(user_schema.columns[3].options.get("null"), Some(&"false".to_string()));

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_callbacks_and_hooks_extraction() {
        let temp_dir = tempdir().unwrap();
        let root_path = temp_dir.path().to_str().unwrap();

        let model_path = temp_dir.path().join("post.rb");
        fs::write(model_path, "class Post < ApplicationRecord\n  after_create :log_creation, :send_notification\n  before_destroy :log_deletion\nend").unwrap();
        let initializer_path = temp_dir.path().join("initializer.rb");
        fs::write(initializer_path, "add_to_class(:name, &ActiveSupport::Dependencies.set_name_accessor)\ninitializer :foo do |bar|\n  define_method :my_method do\n  end\nend").unwrap();

        let analyzer = RubyRailsAnalyzer::default();
        let result = analyzer.analyze(root_path).unwrap();
        let analyzer: RubyRailsAnalyzer = serde_json::from_str(&result).unwrap();

        assert_eq!(analyzer.callbacks.len(), 3);
        assert_eq!(analyzer.callbacks[0].model, "Post");
        assert_eq!(analyzer.callbacks[0].r#type, "after_create");
        assert_eq!(analyzer.callbacks[0].method, "log_creation");

        assert_eq!(analyzer.callbacks[1].model, "Post");
        assert_eq!(analyzer.callbacks[1].r#type, "after_create");
        assert_eq!(analyzer.callbacks[1].method, "send_notification");

        assert_eq!(analyzer.callbacks[2].model, "Post");
        assert_eq!(analyzer.callbacks[2].r#type, "before_destroy");
        assert_eq!(analyzer.callbacks[2].method, "log_deletion");

        assert_eq!(analyzer.hooks.len(), 2);
        assert_eq!(analyzer.hooks[0].r#type, "name");
        assert_eq!(analyzer.hooks[0].method, "ActiveSupport::Dependencies.set_name_accessor");

        assert_eq!(analyzer.hooks[1].name, "foo");
        assert_eq!(analyzer.hooks[1].r#type, "initializer");
        assert_eq!(analyzer.hooks[1].target, "bar");
        assert_eq!(analyzer.hooks[1].method, "my_method");

        temp_dir.close().unwrap();
    }
}