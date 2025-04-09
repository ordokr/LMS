# Authentication Technical Implementation

Generated on: 2025-04-09

## Overview

This document details the technical implementation of authentication for the Canvas-Discourse integration.

## Implementation Files

- `services\integration\auth\jwt-provider.js` (javascript, last modified: 2025-04-05)
- `plugins\discourse\canvas_sso.rb` (ruby, last modified: 2025-04-05)

## Classes

### JwtAuthProvider

```undefined
class JwtAuthProvider {
  /**
   * Initialize the JWT authentication provider
   * @param {Object}
```

### CanvasSso

Extends: `::Auth::AuthProvider`

Methods:

- `name()`

```undefined
class CanvasSso < ::Auth::AuthProvider
  def name
    'canvas'
  end
```

### CanvasSsoAuthenticator

Extends: `::Auth::Authenticator`

Methods:

- `name()`

```undefined
class CanvasSsoAuthenticator < ::Auth::Authenticator
  def name
    'canvas_sso'
  end
```


## Functions

### enabled?

```javascript
def enabled?
    SiteSetting.canvas_sso_enabled
  end
```

### authenticator

```javascript
def authenticator
    CanvasSsoAuthenticator
  end
```

### after_authenticate

```javascript
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
```

### store_user_mapping

```javascript
def store_user_mapping(canvas_user_id, discourse_user_id)
    # Use the integration database to store mapping
    ActiveRecord::Base.connection_pool.with_connection do |connection|
      result = connection.execute(
        "SELECT id FROM user_mappings WHERE canvas_user_id = #{canvas_user_id}"
      )
      
      if result.count == 0
        connection.execute(
          "INSERT INTO user_mappings (canvas_user_id, discourse_user_id, status, last_sync_at) 
           VALUES (#{canvas_...
```

### name

```javascript
def name
    'canvas_sso'
  end
```

### register_middleware

```javascript
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
```

