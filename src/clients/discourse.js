const { createLogger } = require('../utils/logger');

/**
 * Client for interacting with the Discourse API
 */
class DiscourseClient {
  /**
   * Create a new Discourse API client
   * @param {Object} options - Configuration options
   * @param {string} options.baseUrl - Base URL for the Discourse API
   * @param {string} options.apiKey - API key
   * @param {string} options.apiUsername - Username for the API
   */
  constructor(options = {}) {
    this.baseUrl = options.baseUrl || process.env.DISCOURSE_API_URL || 'http://localhost:4000';
    this.apiKey = options.apiKey || process.env.DISCOURSE_API_KEY;
    this.apiUsername = options.apiUsername || process.env.DISCOURSE_API_USERNAME || 'system';
    this.logger = createLogger('discourse-client');
  }

  /**
   * Make an API request to Discourse
   * @param {string} method - HTTP method (GET, POST, etc.)
   * @param {string} endpoint - API endpoint path
   * @param {Object} [requestData=null] - Optional data to send
   * @returns {Promise<Object>} - API response
   */
  async request(method, endpoint, requestData = null) {
    try {
      this.logger.info(`Making ${method} request to ${endpoint}`);
      // In a real implementation, this would use fetch() or another HTTP client
      
      // Mock implementation for testing
      return {
        success: true,
        data: { 
          id: "discourse123",
          result: "success",
          // Additional mock data depending on endpoint
          ...(endpoint.includes('topics') ? { 
            topic_id: 12345,
            slug: "sample-topic" 
          } : {}),
          ...(endpoint.includes('users') ? { 
            id: 67890,
            username: "sample_user"
          } : {})
        }
      };
    } catch (error) {
      this.logger.error(`Discourse API error: ${error.message}`);
      throw error;
    }
  }

  /**
   * Create a new topic
   * @param {Object} topicData - Topic data
   * @param {string} topicData.title - Topic title
   * @param {string} topicData.raw - Topic content
   * @param {number} topicData.category - Category ID
   * @returns {Promise<Object>} - Created topic
   */
  async createTopic(topicData) {
    return this.request('POST', 'topics', topicData);
  }

  /**
   * Get a topic by ID
   * @param {number} topicId - Topic ID
   * @returns {Promise<Object>} - Topic information
   */
  async getTopic(topicId) {
    return this.request('GET', `topics/${topicId}`);
  }

  /**
   * Create a new user
   * @param {Object} userData - User data
   * @returns {Promise<Object>} - Created user
   */
  async createUser(userData) {
    return this.request('POST', 'users', userData);
  }

  /**
   * Authenticate a user via SSO
   * @param {Object} ssoData - SSO payload and signature
   * @returns {Promise<Object>} - Authentication result
   */
  async authenticateSSO(ssoData) {
    return this.request('POST', 'admin/users/sync_sso', ssoData);
  }
}

module.exports = { DiscourseClient };