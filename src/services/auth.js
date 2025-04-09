const { createLogger } = require('../utils/logger');
const jwt = require('jsonwebtoken');
const SECRET_KEY = process.env.JWT_SECRET || 'your-secret-key';

/**
 * Service for handling authentication between Canvas and Discourse
 */
class UserAuthService {
  /**
   * Create a new authentication service
   * @param {Object} canvasClient - Canvas API client
   * @param {Object} discourseClient - Discourse API client
   */
  constructor(canvasClient, discourseClient) {
    this.canvasClient = canvasClient;
    this.discourseClient = discourseClient;
    this.logger = createLogger('auth-service');
  }

  /**
   * Create an SSO payload for Discourse authentication
   * @param {Object} canvasUser - Canvas user data
   * @returns {Object} - SSO payload and signature
   */
  createSSOPayload(canvasUser) {
    // In a real implementation, this would encode and sign the payload
    return {
      payload: Buffer.from(JSON.stringify({
        external_id: canvasUser.id,
        email: canvasUser.email,
        username: canvasUser.email.split('@')[0],
        name: canvasUser.name
      })).toString('base64'),
      signature: 'mock-signature'
    };
  }

  /**
   * Authenticate a Canvas user with Discourse
   * @param {Object} canvasUser - Canvas user data
   * @returns {Promise<Object>} - Authentication result
   */
  async authenticateUser(canvasUser) {
    try {
      this.logger.info(`Authenticating user ${canvasUser.email}`);
      const ssoData = this.createSSOPayload(canvasUser);
      
      // In a real implementation, this would use the actual Discourse SSO API
      const result = await this.discourseClient.authenticateSSO(ssoData);
      
      return {
        success: true,
        canvasUserId: canvasUser.id,
        discourseUserId: result.data.id,
        ssoToken: 'sample-token-' + Math.random().toString(36).substring(2)
      };
    } catch (error) {
      this.logger.error(`Authentication failed: ${error.message}`);
      return {
        success: false,
        error: error.message
      };
    }
  }
}

/**
 * Generate a JWT token with the provided payload.
 * @param {Object} payload - The payload to encode.
 * @returns {string} JWT token.
 */
function generateJwt(payload) {
  return jwt.sign(payload, SECRET_KEY, { expiresIn: '1h' });
}

/**
 * Verify a given JWT token.
 * @param {string} token - The token to verify.
 * @returns {Object} The decoded payload.
 * @throws {Error} If token is invalid.
 */
function verifyJwt(token) {
  try {
    return jwt.verify(token, SECRET_KEY);
  } catch (error) {
    throw new Error('Invalid token');
  }
}

module.exports = { UserAuthService, generateJwt, verifyJwt };