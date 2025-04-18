#[cfg(test)]
mod tests {
    use super::super::enhanced_ruby_migration_analyzer::EnhancedRubyMigrationAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_migration_schema() {
        let temp_dir = tempdir().unwrap();
        
        // Create a migrations directory structure
        let migrations_dir = temp_dir.path().join("db").join("migrate");
        fs::create_dir_all(&migrations_dir).unwrap();
        
        // Create a migration file for users table
        let users_migration_path = migrations_dir.join("20220101000001_create_users.rb");
        let users_migration_content = r#"
class CreateUsers < ActiveRecord::Migration[6.1]
  def change
    create_table :users do |t|
      t.string :name, null: false
      t.string :email, null: false
      t.string :password_digest
      t.boolean :active, default: true
      t.timestamps
    end
    
    add_index :users, :email, unique: true
  end
end
        "#;
        fs::write(&users_migration_path, users_migration_content).unwrap();
        
        // Create a migration file for posts table
        let posts_migration_path = migrations_dir.join("20220101000002_create_posts.rb");
        let posts_migration_content = r#"
class CreatePosts < ActiveRecord::Migration[6.1]
  def change
    create_table :posts do |t|
      t.string :title, null: false
      t.text :content
      t.references :user, null: false, foreign_key: true
      t.timestamps
    end
    
    add_index :posts, [:user_id, :created_at]
  end
end
        "#;
        fs::write(&posts_migration_path, posts_migration_content).unwrap();
        
        // Create a migration file to add columns to users
        let add_columns_migration_path = migrations_dir.join("20220101000003_add_role_to_users.rb");
        let add_columns_migration_content = r#"
class AddRoleToUsers < ActiveRecord::Migration[6.1]
  def change
    add_column :users, :role, :string, default: 'user'
    add_column :users, :last_login_at, :datetime
  end
end
        "#;
        fs::write(&add_columns_migration_path, add_columns_migration_content).unwrap();
        
        let mut analyzer = EnhancedRubyMigrationAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        // Check that we found the migrations
        assert_eq!(analyzer.migrations.len(), 3);
        
        // Check that we have the correct tables
        assert_eq!(analyzer.tables.len(), 2);
        assert!(analyzer.tables.contains_key("users"));
        assert!(analyzer.tables.contains_key("posts"));
        
        // Check users table
        let users_table = analyzer.tables.get("users").unwrap();
        assert_eq!(users_table.name, "users");
        
        // Check users columns
        assert_eq!(users_table.columns.len(), 6); // name, email, password_digest, active, created_at, updated_at, role, last_login_at
        assert!(users_table.columns.iter().any(|col| col.name == "name" && col.column_type == "string"));
        assert!(users_table.columns.iter().any(|col| col.name == "email" && col.column_type == "string"));
        assert!(users_table.columns.iter().any(|col| col.name == "role" && col.column_type == "string"));
        
        // Check users indexes
        assert_eq!(users_table.indexes.len(), 1);
        assert!(users_table.indexes.iter().any(|idx| 
            idx.columns.contains(&"email".to_string()) && 
            idx.unique
        ));
        
        // Check posts table
        let posts_table = analyzer.tables.get("posts").unwrap();
        assert_eq!(posts_table.name, "posts");
        
        // Check posts columns
        assert_eq!(posts_table.columns.len(), 5); // title, content, user_id, created_at, updated_at
        assert!(posts_table.columns.iter().any(|col| col.name == "title" && col.column_type == "string"));
        assert!(posts_table.columns.iter().any(|col| col.name == "content" && col.column_type == "text"));
        assert!(posts_table.columns.iter().any(|col| col.name == "user_id"));
        
        // Check posts indexes
        assert_eq!(posts_table.indexes.len(), 1);
        assert!(posts_table.indexes.iter().any(|idx| 
            idx.columns.contains(&"user_id".to_string()) && 
            idx.columns.contains(&"created_at".to_string())
        ));
        
        // Check posts foreign keys
        assert_eq!(posts_table.foreign_keys.len(), 1);
        assert!(posts_table.foreign_keys.iter().any(|fk| 
            fk.from_column == "user_id" && 
            fk.to_table == "user"
        ));
        
        temp_dir.close().unwrap();
    }
}
