#[cfg(test)]
mod tests {
    use super::super::enhanced_ruby_controller_analyzer::EnhancedRubyControllerAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_controller_actions() {
        let temp_dir = tempdir().unwrap();
        
        // Create a routes.rb file
        let routes_path = temp_dir.path().join("routes.rb");
        let routes_content = r#"
Rails.application.routes.draw do
  resources :users
  
  get '/dashboard', to: 'dashboard:index'
  post '/login', to: 'sessions:create'
end
        "#;
        fs::write(&routes_path, routes_content).unwrap();
        
        // Create a controller file
        let controller_path = temp_dir.path().join("users_controller.rb");
        let controller_content = r#"
class UsersController < ApplicationController
  before_action :authenticate_user!, except: [:new, :create]
  before_action :set_user, only: [:show, :edit, :update, :destroy]
  
  layout 'application'
  
  respond_to :html, :json
  
  def index
    @users = User.all
    respond_with(@users)
  end
  
  def show
    respond_with(@user)
  end
  
  def new
    @user = User.new
    respond_with(@user)
  end
  
  def edit
  end
  
  def create
    @user = User.new(user_params)
    
    if @user.save
      redirect_to @user, notice: 'User was successfully created.'
    else
      render :new
    end
  end
  
  def update
    if @user.update(user_params)
      redirect_to @user, notice: 'User was successfully updated.'
    else
      render :edit
    end
  end
  
  def destroy
    @user.destroy
    redirect_to users_url, notice: 'User was successfully destroyed.'
  end
  
  private
  
  def set_user
    @user = User.find(params[:id])
  end
  
  def user_params
    params.require(:user).permit(:name, :email, :password)
  end
end
        "#;
        fs::write(&controller_path, controller_content).unwrap();
        
        let mut analyzer = EnhancedRubyControllerAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        assert_eq!(analyzer.controllers.len(), 1);
        
        let users_controller = analyzer.controllers.get("UsersController").unwrap();
        
        // Check controller properties
        assert_eq!(users_controller.name, "UsersController");
        assert_eq!(users_controller.parent_class, "ApplicationController");
        assert_eq!(users_controller.layout, Some("application".to_string()));
        assert_eq!(users_controller.respond_to_formats.len(), 2);
        assert!(users_controller.respond_to_formats.contains(&"html".to_string()));
        assert!(users_controller.respond_to_formats.contains(&"json".to_string()));
        
        // Check actions
        assert_eq!(users_controller.actions.len(), 7);
        assert!(users_controller.actions.iter().any(|action| action.name == "index"));
        assert!(users_controller.actions.iter().any(|action| action.name == "show"));
        assert!(users_controller.actions.iter().any(|action| action.name == "new"));
        assert!(users_controller.actions.iter().any(|action| action.name == "edit"));
        assert!(users_controller.actions.iter().any(|action| action.name == "create"));
        assert!(users_controller.actions.iter().any(|action| action.name == "update"));
        assert!(users_controller.actions.iter().any(|action| action.name == "destroy"));
        
        // Check filters
        assert_eq!(users_controller.filters.len(), 2);
        assert!(users_controller.filters.iter().any(|filter| 
            filter.filter_type == "before_action" && 
            filter.methods.contains(&"authenticate_user!".to_string()) &&
            filter.options.get("except") == Some(&"new, :create".to_string())
        ));
        assert!(users_controller.filters.iter().any(|filter| 
            filter.filter_type == "before_action" && 
            filter.methods.contains(&"set_user".to_string()) &&
            filter.options.get("only") == Some(&"show, :edit, :update, :destroy".to_string())
        ));
        
        // Check helpers
        assert_eq!(users_controller.helpers.len(), 2);
        assert!(users_controller.helpers.iter().any(|helper| helper.name == "set_user"));
        assert!(users_controller.helpers.iter().any(|helper| helper.name == "user_params"));
        
        temp_dir.close().unwrap();
    }
}
