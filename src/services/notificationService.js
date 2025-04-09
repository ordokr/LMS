const canvasApi = require('../api/canvasApi');
const discourseApi = require('../api/discourseApi');
const { Notification } = require('../models/unifiedModels');

/**
 * Service for handling notifications between Canvas and Discourse
 */
class NotificationService {
  /**
   * Get notifications for a user from both systems
   * @param {string} userId - User ID
   * @param {Object} options - Filtering options
   * @returns {Array} Combined notifications from both systems
   */
  async getUserNotifications(userId, options = {}) {
    try {
      // Get Canvas user ID
      const userMapping = await this._getUserMapping(userId);
      
      // Fetch notifications from both systems in parallel
      const [canvasNotifications, discourseNotifications] = await Promise.all([
        canvasApi.getUserNotifications(userMapping.canvasId),
        discourseApi.getUserNotifications(userMapping.discourseId)
      ]);
      
      // Convert to unified model
      const unifiedNotifications = [
        ...canvasNotifications.map(n => Notification.fromCanvasNotification(n)),
        ...discourseNotifications.map(n => Notification.fromDiscourseNotification(n))
      ];
      
      // Sort by date (newest first)
      unifiedNotifications.sort((a, b) => 
        new Date(b.createdAt) - new Date(a.createdAt)
      );
      
      // Apply filters
      return this._applyNotificationFilters(unifiedNotifications, options);
    } catch (error) {
      console.error('Error fetching notifications:', error);
      throw new Error(`Failed to fetch notifications for user ${userId}`);
    }
  }
  
  /**
   * Mark a notification as read
   * @param {string} notificationId - Notification ID
   * @param {string} source - Source system
   * @returns {Object} Updated notification
   */
  async markAsRead(notificationId, source) {
    try {
      let result;
      
      if (source === 'canvas') {
        result = await canvasApi.markNotificationAsRead(notificationId);
        return Notification.fromCanvasNotification(result);
      } else {
        result = await discourseApi.markNotificationAsRead(notificationId);
        return Notification.fromDiscourseNotification(result);
      }
    } catch (error) {
      console.error('Error marking notification as read:', error);
      throw new Error(`Failed to mark notification ${notificationId} as read`);
    }
  }
  
  /**
   * Creates a notification in both systems
   * @param {Object} notificationData - Notification data
   * @returns {Object} Created notification
   */
  async createNotification(notificationData) {
    try {
      // Create unified notification model
      const notification = new Notification(notificationData);
      
      // Get user mapping
      const userMapping = await this._getUserMapping(notification.userId);
      
      // Send to Canvas
      const canvasNotification = notification.toCanvasNotification();
      canvasNotification.user_id = userMapping.canvasId;
      const canvasResult = await canvasApi.createNotification(canvasNotification);
      
      // Send to Discourse
      const discourseNotification = notification.toDiscourseNotification();
      discourseNotification.user_id = userMapping.discourseId;
      const discourseResult = await discourseApi.createNotification(discourseNotification);
      
      // Update the notification with IDs from both systems
      notification.canvasId = canvasResult.id;
      notification.discourseId = discourseResult.id;
      
      return notification;
    } catch (error) {
      console.error('Error creating notification:', error);
      throw new Error('Failed to create notification');
    }
  }
  
  /**
   * Apply filters to notifications
   * @private
   */
  _applyNotificationFilters(notifications, options) {
    let filtered = [...notifications];
    
    // Filter by read status
    if (options.read !== undefined) {
      filtered = filtered.filter(n => n.read === options.read);
    }
    
    // Filter by type
    if (options.type) {
      filtered = filtered.filter(n => n.notificationType === options.type);
    }
    
    // Filter by date range
    if (options.since) {
      const sinceDate = new Date(options.since);
      filtered = filtered.filter(n => new Date(n.createdAt) >= sinceDate);
    }
    
    // Apply limit
    if (options.limit && options.limit > 0) {
      filtered = filtered.slice(0, options.limit);
    }
    
    return filtered;
  }
  
  /**
   * Get user mapping between Canvas and Discourse
   * @private
   */
  async _getUserMapping(userId) {
    // In a real implementation, this would fetch from a database
    // For now, we'll return a simple mapping
    return {
      internalId: userId,
      canvasId: userId,
      discourseId: userId
    };
  }
}

module.exports = new NotificationService();