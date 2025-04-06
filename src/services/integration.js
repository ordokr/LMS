const { createLogger } = require('../utils/logger');

/**
 * Service for integrating Canvas and Discourse
 */
class IntegrationService {
  /**
   * Create a new integration service
   * @param {Object} canvasClient - Canvas API client
   * @param {Object} discourseClient - Discourse API client
   */
  constructor(canvasClient, discourseClient) {
    this.canvasClient = canvasClient;
    this.discourseClient = discourseClient;
    this.logger = createLogger('integration-service');
  }

  /**
   * Synchronize a Canvas announcement to a Discourse forum topic
   * @param {Object} announcement - Announcement data from Canvas
   * @returns {Promise<Object>} - Synchronization result
   */
  async syncAnnouncementToForum(announcement) {
    try {
      this.logger.info(`Syncing announcement "${announcement.title}" to Discourse`);
      
      // Create the topic in Discourse
      const topicResult = await this.discourseClient.createTopic({
        title: announcement.title,
        raw: announcement.message,
        category: await this.getDiscourseCategory(announcement.courseId)
      });
      
      this.logger.info(`Created Discourse topic ${topicResult.data.topic_id}`);
      
      return {
        success: true,
        canvasAnnouncementId: announcement.id,
        discourseTopicId: topicResult.data.topic_id,
        discourseTopic: topicResult.data
      };
    } catch (error) {
      this.logger.error(`Failed to sync announcement: ${error.message}`);
      return {
        success: false,
        error: error.message
      };
    }
  }

  /**
   * Get the appropriate Discourse category for a Canvas course
   * @param {string} _courseId - Canvas course ID
   * @returns {Promise<number>} - Discourse category ID
   */
  async getDiscourseCategory(_courseId) {
    // In a real implementation, this would look up the mapping
    // For now, just return a mock ID
    return 5; // Mock category ID
  }

  /**
   * Authenticate a Canvas user with Discourse via SSO
   * @param {Object} canvasUser - Canvas user object
   * @returns {Promise<Object>} - Authentication result
   */
  async authenticateUserWithDiscourse(canvasUser) {
    try {
      this.logger.info(`Authenticating user ${canvasUser.name} with Discourse`);
      
      // In a real implementation, this would create an SSO payload
      const ssoResult = await this.discourseClient.authenticateSSO({
        email: canvasUser.email,
        external_id: canvasUser.id,
        username: canvasUser.email.split('@')[0],
        name: canvasUser.name
      });
      
      return {
        success: true,
        canvasUserId: canvasUser.id,
        discourseUserId: ssoResult.data.id,
        ssoToken: 'sample-token-' + Math.random().toString(36).substring(2)
      };
    } catch (error) {
      this.logger.error(`Failed to authenticate user: ${error.message}`);
      return {
        success: false,
        error: error.message
      };
    }
  }
}

module.exports = { IntegrationService };