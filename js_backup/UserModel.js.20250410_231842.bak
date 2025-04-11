import BaseModel from './BaseModel';

/**
 * Unified User model that bridges Canvas and Discourse user models
 */
export default class UserModel extends BaseModel {
  constructor(data = {}, source = null) {
    super();
    
    // Define default properties
    this.id = null;             // Unified ID
    this.email = '';
    this.name = '';
    this.username = '';
    this.createdAt = null;
    this.updatedAt = null;
    this.lastLogin = null;
    
    // System-specific IDs
    this.canvasId = null;
    this.discourseId = null;
    
    // Canvas-specific properties
    this.enrollments = [];
    this.avatarUrl = '';
    this.bio = '';
    this.locale = 'en';
    this.timezone = 'UTC';
    
    // Discourse-specific properties
    this.trustLevel = 0;
    this.lastSeenAt = null;
    this.websiteUrl = '';
    this.badgeCount = 0;
    
    // Set data with appropriate transformations
    this.setData(data, source);
    
    // Process source-specific data
    if (source) {
      this._processSourceData(data, source);
    }
  }
  
  /**
   * Process source-specific data formats
   */
  _processSourceData(data, source) {
    if (source === 'canvas') {
      // Map Canvas-specific fields
      this.canvasId = data.id;
      this.enrollments = data.enrollments || [];
      this.avatarUrl = data.avatar_url || data.avatarUrl || '';
      
      // Handle timestamps
      if (data.created_at || data.createdAt) {
        this.createdAt = new Date(data.created_at || data.createdAt);
      }
      if (data.updated_at || data.updatedAt) {
        this.updatedAt = new Date(data.updated_at || data.updatedAt);
      }
      
      // Generate username if not present
      if (!this.username && this.email) {
        this.username = this.email.split('@')[0];
      }
    } else if (source === 'discourse') {
      // Map Discourse-specific fields
      this.discourseId = data.id;
      this.trustLevel = data.trust_level || data.trustLevel || 0;
      this.lastSeenAt = data.last_seen_at ? new Date(data.last_seen_at) : 
                        (data.lastSeenAt ? new Date(data.lastSeenAt) : null);
      this.websiteUrl = data.website || data.websiteUrl || '';
      this.badgeCount = data.badge_count || data.badgeCount || 0;
      
      // Handle timestamps
      if (data.created_at || data.createdAt) {
        this.createdAt = new Date(data.created_at || data.createdAt);
      }
    }
  }
  
  /**
   * Convert to Canvas user format
   */
  toCanvasUser() {
    return {
      id: this.canvasId || this.id,
      name: this.name,
      email: this.email,
      avatar_url: this.avatarUrl,
      bio: this.bio,
      locale: this.locale,
      timezone: this.timezone,
      created_at: this.createdAt ? this.createdAt.toISOString() : null,
      updated_at: this.updatedAt ? this.updatedAt.toISOString() : null,
      enrollments: this.enrollments
    };
  }
  
  /**
   * Convert to Discourse user format
   */
  toDiscourseUser() {
    return {
      id: this.discourseId || this.id,
      name: this.name,
      username: this.username || this.email.split('@')[0],
      email: this.email,
      avatar_template: this.avatarUrl || '',
      trust_level: this.trustLevel,
      website: this.websiteUrl,
      last_seen_at: this.lastSeenAt ? this.lastSeenAt.toISOString() : null,
      created_at: this.createdAt ? this.createdAt.toISOString() : null
    };
  }
  
  /**
   * Create a UserModel from Canvas data
   */
  static fromCanvas(canvasUser) {
    return new UserModel(canvasUser, 'canvas');
  }
  
  /**
   * Create a UserModel from Discourse data
   */
  static fromDiscourse(discourseUser) {
    return new UserModel(discourseUser, 'discourse');
  }
}