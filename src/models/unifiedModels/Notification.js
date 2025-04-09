/**
 * Unified Notification model for cross-platform notifications
 */
class Notification {
  /**
   * Create a new unified notification
   * @param {Object} data - Notification data
   */
  constructor(data = {}) {
    this.id = data.id || Math.random().toString(36).substring(2, 15);
    this.userId = data.userId;
    this.subject = data.subject || '';
    this.message = data.message || '';
    this.createdAt = data.createdAt || new Date().toISOString();
    this.read = data.read || false;
    this.notificationType = data.notificationType || 'general';
    this.sourceSystem = data.sourceSystem;
    this.canvasId = data.canvasId;
    this.discourseId = data.discourseId;
    this.url = data.url || '';
    this.metadata = data.metadata || {};
  }

  /**
   * Convert Canvas notification to unified model
   * @param {Object} canvasNotification - Canvas notification object
   * @returns {Notification} Unified notification
   */
  static fromCanvasNotification(canvasNotification) {
    return new Notification({
      sourceSystem: 'canvas',
      canvasId: canvasNotification.id,
      userId: canvasNotification.user_id,
      subject: canvasNotification.subject || canvasNotification.title,
      message: canvasNotification.message || canvasNotification.body,
      createdAt: canvasNotification.created_at,
      read: !!canvasNotification.read,
      notificationType: canvasNotification.notification_type || 'general',
      url: canvasNotification.html_url,
      metadata: canvasNotification
    });
  }

  /**
   * Convert Discourse notification to unified model
   * @param {Object} discourseNotification - Discourse notification object
   * @returns {Notification} Unified notification
   */
  static fromDiscourseNotification(discourseNotification) {
    return new Notification({
      sourceSystem: 'discourse',
      discourseId: discourseNotification.id,
      userId: discourseNotification.user_id,
      subject: discourseNotification.data?.topic?.title || 'Discourse Notification',
      message: discourseNotification.fancy_title || discourseNotification.excerpt || '',
      createdAt: discourseNotification.created_at,
      read: !!discourseNotification.read,
      notificationType: discourseNotification.notification_type,
      url: discourseNotification.url,
      metadata: discourseNotification
    });
  }

  /**
   * Convert to Canvas notification format
   * @returns {Object} Canvas format notification
   */
  toCanvasNotification() {
    return {
      id: this.canvasId,
      user_id: this.userId,
      subject: this.subject,
      message: this.message,
      created_at: this.createdAt,
      read: this.read,
      notification_type: this.notificationType,
      html_url: this.url
    };
  }

  /**
   * Convert to Discourse notification format
   * @returns {Object} Discourse format notification
   */
  toDiscourseNotification() {
    return {
      id: this.discourseId,
      user_id: this.userId,
      fancy_title: this.subject,
      excerpt: this.message,
      created_at: this.createdAt,
      read: this.read,
      notification_type: this.notificationType,
      url: this.url
    };
  }
}

module.exports = Notification;