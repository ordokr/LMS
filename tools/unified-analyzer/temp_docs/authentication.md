# Authentication System

This document outlines the authentication system used in the Canvas-Discourse integration.

## JWT Authentication

The system uses JSON Web Tokens (JWT) to handle authentication between Canvas and Discourse.

### Token Flow

1. User authenticates with Canvas OAuth
2. System generates a JWT token containing user information
3. JWT token is used for all API requests
4. For Discourse access, the JWT is verified and a Discourse SSO payload is generated

### Configuration

The authentication system requires the following environment variables:

- `JWT_SECRET`: Secret key for signing JWT tokens
- `JWT_EXPIRATION`: Token expiration time (default: 24h)
- `DISCOURSE_SSO_SECRET`: Shared secret for Discourse SSO
- `DISCOURSE_URL`: Base URL for Discourse instance
- `CANVAS_API_URL`: Base URL for Canvas API

### API Endpoints

- `POST /api/v1/auth/login`: Authenticates a user and returns a JWT token
- `GET /api/v1/auth/discourse-sso`: Handles Discourse SSO authentication

## Security Considerations

- JWT tokens are signed and verified using HMAC SHA256
- Tokens expire after 24 hours by default
- Sensitive user information is not stored in tokens
- SSO signatures are verified to prevent tampering