#[cfg(test)]
mod tests {
    use super::super::enhanced_ruby_model_analyzer::EnhancedRubyModelAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_attributes() {
        let temp_dir = tempdir().unwrap();
        let model_path = temp_dir.path().join("user.rb");
        
        let model_content = r#"
class User < ApplicationRecord
  attr_accessor :name, :email
  attr_reader :created_at
  attr_writer :password
  
  has_many :posts, dependent: :destroy
  belongs_to :account
  
  validates :name, presence: true
  validates :email, presence: true, uniqueness: true
  
  before_save :normalize_email
  after_create :send_welcome_email
  
  scope :active, -> { where(active: true) }
  scope :recent, -> { order(created_at: :desc).limit(10) }
  
  def self.find_by_email(email)
    where(email: email).first
  end
  
  def full_name
    "#{first_name} #{last_name}"
  end
  
  private
  
  def normalize_email
    self.email = email.downcase.strip if email.present?
  end
  
  def send_welcome_email
    # Send email logic
  end
end
        "#;
        
        fs::write(&model_path, model_content).unwrap();
        
        let mut analyzer = EnhancedRubyModelAnalyzer::new();
        analyzer.analyze_model_file(&model_path).unwrap();
        
        assert_eq!(analyzer.models.len(), 1);
        
        let user_model = analyzer.models.get("User").unwrap();
        
        // Check attributes
        assert_eq!(user_model.attributes.len(), 4);
        assert!(user_model.attributes.iter().any(|attr| attr.name == "name" && attr.attr_type == "accessor"));
        assert!(user_model.attributes.iter().any(|attr| attr.name == "email" && attr.attr_type == "accessor"));
        assert!(user_model.attributes.iter().any(|attr| attr.name == "created_at" && attr.attr_type == "reader"));
        assert!(user_model.attributes.iter().any(|attr| attr.name == "password" && attr.attr_type == "writer"));
        
        // Check associations
        assert_eq!(user_model.associations.len(), 2);
        assert!(user_model.associations.iter().any(|assoc| 
            assoc.name == "posts" && 
            assoc.association_type == "has_many" && 
            assoc.dependent.as_deref() == Some("destroy")
        ));
        assert!(user_model.associations.iter().any(|assoc| 
            assoc.name == "account" && 
            assoc.association_type == "belongs_to"
        ));
        
        // Check validations
        assert_eq!(user_model.validations.len(), 2);
        assert!(user_model.validations.iter().any(|val| 
            val.validation_type == "validates" && 
            val.fields.contains(&"name".to_string()) &&
            val.options.get("presence") == Some(&"true".to_string())
        ));
        assert!(user_model.validations.iter().any(|val| 
            val.validation_type == "validates" && 
            val.fields.contains(&"email".to_string()) &&
            val.options.get("presence") == Some(&"true".to_string()) &&
            val.options.get("uniqueness") == Some(&"true".to_string())
        ));
        
        // Check callbacks
        assert_eq!(user_model.callbacks.len(), 2);
        assert!(user_model.callbacks.iter().any(|cb| 
            cb.callback_type == "before_save" && 
            cb.method_name == "normalize_email"
        ));
        assert!(user_model.callbacks.iter().any(|cb| 
            cb.callback_type == "after_create" && 
            cb.method_name == "send_welcome_email"
        ));
        
        // Check scopes
        assert_eq!(user_model.scopes.len(), 2);
        assert!(user_model.scopes.iter().any(|scope| 
            scope.name == "active" && 
            scope.query.contains("where(active: true)")
        ));
        assert!(user_model.scopes.iter().any(|scope| 
            scope.name == "recent" && 
            scope.query.contains("order(created_at: :desc)")
        ));
        
        // Check methods
        assert!(user_model.methods.iter().any(|method| 
            method.name == "find_by_email" && 
            method.is_class_method &&
            method.parameters.contains(&"email".to_string())
        ));
        assert!(user_model.methods.iter().any(|method| 
            method.name == "full_name" && 
            !method.is_class_method
        ));
        assert!(user_model.methods.iter().any(|method| 
            method.name == "normalize_email" && 
            !method.is_class_method
        ));
        assert!(user_model.methods.iter().any(|method| 
            method.name == "send_welcome_email" && 
            !method.is_class_method
        ));
        
        temp_dir.close().unwrap();
    }
}
