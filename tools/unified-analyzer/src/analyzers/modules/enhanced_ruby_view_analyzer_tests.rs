#[cfg(test)]
mod tests {
    use super::super::enhanced_ruby_view_analyzer::EnhancedRubyViewAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_view_components() {
        let temp_dir = tempdir().unwrap();
        
        // Create a views directory structure
        let views_dir = temp_dir.path().join("app").join("views").join("users");
        fs::create_dir_all(&views_dir).unwrap();
        
        // Create a view file
        let view_path = views_dir.join("show.html.erb");
        let view_content = r#"
<% content_for :title, "User Profile" %>

<div class="user-profile">
  <h1><%= @user.name %></h1>
  
  <div class="user-details">
    <p><strong>Email:</strong> <%= @user.email %></p>
    <p><strong>Joined:</strong> <%= format_date(@user.created_at) %></p>
  </div>
  
  <%= render partial: "shared/avatar", locals: { user: @user, size: "large" } %>
  
  <div class="user-actions">
    <%= link_to "Edit Profile", edit_user_path(@user), class: "btn btn-primary" %>
    <%= link_to "Back to Users", users_path, class: "btn btn-secondary" %>
  </div>
  
  <% if current_user.admin? %>
    <div class="admin-actions">
      <%= form_for @user, url: admin_user_path(@user), method: :patch do |f| %>
        <div class="form-group">
          <%= f.label :role %>
          <%= f.select :role, User.roles.keys, { include_blank: "Select Role" }, class: "form-control" %>
        </div>
        
        <div class="form-group">
          <%= f.check_box :active, class: "form-check-input" %>
          <%= f.label :active, class: "form-check-label" %>
        </div>
        
        <div class="form-actions">
          <%= f.submit "Update Role", class: "btn btn-warning" %>
        </div>
      <% end %>
    </div>
  <% end %>
</div>
        "#;
        fs::write(&view_path, view_content).unwrap();
        
        // Create a partial file
        let partials_dir = temp_dir.path().join("app").join("views").join("shared");
        fs::create_dir_all(&partials_dir).unwrap();
        
        let partial_path = partials_dir.join("_avatar.html.erb");
        let partial_content = r#"
<div class="avatar <%= size %>">
  <% if user.avatar.present? %>
    <%= image_tag user.avatar.url, alt: user.name %>
  <% else %>
    <%= image_tag "default_avatar.png", alt: user.name %>
  <% end %>
</div>
        "#;
        fs::write(&partial_path, partial_content).unwrap();
        
        let mut analyzer = EnhancedRubyViewAnalyzer::new();
        analyzer.analyze_directory(&temp_dir.path().join("app").join("views")).unwrap();
        
        // Check that we found the view
        assert_eq!(analyzer.views.len(), 2);
        
        // Get the view
        let view = analyzer.views.values().find(|v| v.name == "show").unwrap();
        
        // Check view properties
        assert_eq!(view.controller, Some("users".to_string()));
        assert_eq!(view.action, Some("show".to_string()));
        
        // Check instance variables
        assert!(view.instance_variables.contains(&"user".to_string()));
        
        // Check partials
        assert_eq!(view.partials.len(), 1);
        let partial = &view.partials[0];
        assert_eq!(partial.name, "shared/avatar");
        assert!(partial.locals.contains(&"user".to_string()));
        assert!(partial.locals.contains(&"size".to_string()));
        
        // Check links
        assert_eq!(view.links.len(), 2);
        assert!(view.links.iter().any(|link| 
            link.text.as_deref() == Some("Edit Profile") && 
            link.url.contains("edit_user_path")
        ));
        assert!(view.links.iter().any(|link| 
            link.text.as_deref() == Some("Back to Users") && 
            link.url.contains("users_path")
        ));
        
        // Check forms
        assert_eq!(view.forms.len(), 1);
        let form = &view.forms[0];
        assert_eq!(form.model, Some("user".to_string()));
        assert_eq!(form.method, Some("patch".to_string()));
        
        // Check form fields
        assert_eq!(form.fields.len(), 2);
        assert!(form.fields.iter().any(|field| 
            field.name == "role" && 
            field.field_type == "select"
        ));
        assert!(form.fields.iter().any(|field| 
            field.name == "active" && 
            field.field_type == "check_box"
        ));
        
        // Check helpers
        assert!(view.helpers.contains(&"format_date".to_string()));
        assert!(view.helpers.contains(&"current_user".to_string()));
        
        temp_dir.close().unwrap();
    }
}
