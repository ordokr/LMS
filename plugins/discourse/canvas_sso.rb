# Canvas SSO Plugin for Discourse
# Allows Canvas users to seamlessly log in to Discourse

require 'jwt'

class CanvasSso < ::Auth::AuthProvider
  def name
    'canvas'
  end
  
  def enabled?
    SiteSetting.canvas_sso_enabled
  end
  
  def authenticator
    CanvasSsoAuthenticator
  end
  
  def after_authenticate(auth_token)
    result = Auth::Result.new
    
    begin
      payload = auth_token[:credentials][:jwt]
      
      # Validate user information
      if payload[:email].blank?
        result.failed = true
        result.failed_reason = "Email is missing from JWT payload"
        return result
      end
      
      # Find or create user
      user = User.find_by(email: payload[:email])
      
      if !user
        # Create new user
        user = User.new(
          email: payload[:email],
          username: generate_username(payload[:name] || payload[:email]),
          name: payload[:name],
          approved: true
        )
        
        # Validate and save
        if user.valid?
          user.save!
          
          # Store external ID in user custom field
          user.custom_fields['canvas_user_id'] = payload[:external_id]
          user.custom_fields['canvas_roles'] = payload[:roles]
          user.save_custom_fields
        else
          result.failed = true
          result.failed_reason = "Failed to create user: #{user.errors.full_messages.join(', ')}"
          return result
        end
      else
        # Update existing user info
        user.name = payload[:name] if payload[:name].present?
        user.custom_fields['canvas_user_id'] = payload[:external_id]
        user.custom_fields['canvas_roles'] = payload[:roles]
        user.save_custom_fields
      end
      
      # Handle admin status
      if payload[:admin] && !user.admin && SiteSetting.canvas_sync_admin_status
        user.admin = true
        user.save
      end
      
      # Complete authentication
      result.user = user
      result.email = payload[:email]
      result.email_valid = true
      
      # Store mapping in database
      store_user_mapping(payload[:user_id], user.id)
      
    rescue JWT::DecodeError => e
      result.failed = true
      result.failed_reason = "Invalid JWT token: #{e.message}"
    rescue => e
      result.failed = true
      result.failed_reason = "Authentication error: #{e.message}"
      Rails.logger.error("Canvas SSO error: #{e.class} - #{e.message}\n#{e.backtrace.join("\n")}")
    end
    
    result
  end
  
  def generate_username(name)
    base = name.downcase.gsub(/[^a-z0-9]/, '_').gsub(/_+/, '_')
    # Ensure uniqueness
    return base if !User.find_by(username: base)
    
    # Add random suffix if username is taken
    "#{base}_#{SecureRandom.hex(3)}"
  end
  
  def store_user_mapping(canvas_user_id, discourse_user_id)
    # Use the integration database to store mapping
    ActiveRecord::Base.connection_pool.with_connection do |connection|
      result = connection.execute(
        "SELECT id FROM user_mappings WHERE canvas_user_id = #{canvas_user_id}"
      )
      
      if result.count == 0
        connection.execute(
          "INSERT INTO user_mappings (canvas_user_id, discourse_user_id, status, last_sync_at) 
           VALUES (#{canvas_user_id}, #{discourse_user_id}, 'active', NOW())"
        )
      else
        connection.execute(
          "UPDATE user_mappings 
           SET discourse_user_id = #{discourse_user_id}, 
               status = 'active', 
               last_sync_at = NOW() 
           WHERE canvas_user_id = #{canvas_user_id}"
        )
      end
    end
  end
end

class CanvasSsoAuthenticator < ::Auth::Authenticator
  def name
    'canvas_sso'
  end
  
  def register_middleware(omniauth)
    omniauth.provider :jwt,
      name: :canvas,
      uid_claim: 'user_id',
      required_claims: ['email', 'external_id'],
      secret: SiteSetting.canvas_jwt_secret,
      algorithm: 'HS256',
      iss: SiteSetting.canvas_jwt_issuer,
      aud: SiteSetting.canvas_jwt_audience,
      callback_path: '/auth/canvas/callback'
  end
end

auth_provider title: 'Canvas SSO',
              enabled_setting: 'canvas_sso_enabled',
              authenticator: CanvasSsoAuthenticator.new,
              message: 'Logging in via Canvas...'

# Add site settings
SiteSetting.defaults.merge!(
  canvas_sso_enabled: false,
  canvas_jwt_secret: '',
  canvas_jwt_issuer: 'canvas',
  canvas_jwt_audience: 'discourse',
  canvas_sync_admin_status: false
)