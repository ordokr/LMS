/**
 * JWT Authentication Provider for Canvas-Discourse Integration
 * Handles generation and verification of JWT tokens for SSO between systems
 */
class JwtAuthProvider {
  /**
   * Initialize the JWT authentication provider
   * @param {Object} options Configuration options
   * @param {string} options.secret JWT secret key
   * @param {string} options.issuer JWT issuer (typically 'canvas')
   * @param {string} options.audience JWT audience (typically 'discourse')
   * @param {number} options.expiresIn Token expiration in seconds (default: 1 hour)
   */
  constructor(options = {}) {
    this.secret = options.secret || process.env.JWT_SECRET;
    this.issuer = options.issuer || 'canvas';
    this.audience = options.audience || 'discourse';
    this.expiresIn = options.expiresIn || 3600; // 1 hour
    
    if (!this.secret) {
      throw new Error('JWT secret is required');
    }
  }
  
  /**
   * Generate a JWT token for Canvas user to authenticate with Discourse
   * @param {Object} user Canvas user object
   * @returns {string} JWT token
   */
  async generateToken(user) {
    if (!user || !user.id) {
      throw new Error('Valid user object is required');
    }
    
    const payload = {
      user_id: user.id,
      email: user.email,
      name: user.display_name || user.name,
      external_id: `canvas_${user.id}`,
      admin: !!user.admin,
      roles: Array.isArray(user.roles) ? user.roles.join(',') : ''
    };
    
    return jwt.sign(payload, this.secret, {
      expiresIn: this.expiresIn,
      audience: this.audience,
      issuer: this.issuer
    });
  }
  
  /**
   * Verify a JWT token from Discourse
   * @param {string} token JWT token
   * @returns {Object} Decoded token payload
   */
  async verifyToken(token) {
    try {
      return jwt.verify(token, this.secret, {
        audience: this.issuer, // Reversed for verification direction
        issuer: this.audience
      });
    } catch (error) {
      throw new Error(`Invalid JWT token: ${error.message}`);
    }
  }
  
  /**
   * Generate SSO redirect URL for Canvas user to Discourse
   * @param {Object} user Canvas user object
   * @param {string} returnUrl URL to return to after authentication
   * @returns {string} Discourse SSO URL with token
   */
  async generateSsoUrl(user, returnUrl) {
    const token = await this.generateToken(user);
    
    const discourseUrl = process.env.DISCOURSE_URL || 'http://discourse.example.com';
    const ssoEndpoint = '/canvas-sso';
    
    const url = new URL(ssoEndpoint, discourseUrl);
    url.searchParams.append('token', token);
    
    if (returnUrl) {
      url.searchParams.append('return_url', returnUrl);
    }
    
    return url.toString();
  }
}

module.exports = JwtAuthProvider;