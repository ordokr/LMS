/**
 * Unified Discussion model
 * Maps between Canvas Discussion and Discourse Topic
 */
class Discussion {
  constructor(data = {}) {
    // Core fields
    this.id = data.id || null;
    this.title = data.title || '';
    this.message = data.message || data.raw || data.body || '';
    this.createdAt = data.createdAt || data.created_at || new Date();
    this.updatedAt = data.updatedAt || data.updated_at || new Date();
    this.creatorId = data.creatorId || data.user_id || data.creator_id || null;
    
    // Canvas-specific fields
    this.canvasId = data.canvasId || data.canvas_id || null;
    this.courseId = data.courseId || data.course_id || null;
    this.pinned = data.pinned || false;
    this.locked = data.locked || false;
    this.allowRating = data.allowRating || data.allow_rating || false;
    this.onlyGradersCanRate = data.onlyGradersCanRate || data.only_graders_can_rate || false;
    
    // Discourse-specific fields
    this.discourseId = data.discourseId || data.discourse_id || null;
    this.categoryId = data.categoryId || data.category_id || null;
    this.slug = data.slug || this._generateSlug();
    this.views = data.views || 0;
    this.postsCount = data.postsCount || data.posts_count || 0;
    this.closed = data.closed || false;
    this.archived = data.archived || false;
    this.tags = data.tags || [];
    
    // Integration fields
    this.lastSync = data.lastSync || null;
    this.sourceSystem = data.sourceSystem || 'canvas';
  }
  
  _generateSlug() {
    return this.title
      ? this.title.toLowerCase().replace(/[^a-z0-9]+/g, '-')
      : '';
  }
  
  // Convert to Canvas discussion format
  toCanvasDiscussion() {
    return {
      id: this.canvasId || this.id,
      title: this.title,
      message: this.message,
      created_at: this.createdAt,
      updated_at: this.updatedAt,
      user_id: this.creatorId,
      course_id: this.courseId,
      pinned: this.pinned,
      locked: this.locked,
      allow_rating: this.allowRating,
      only_graders_can_rate: this.onlyGradersCanRate
    };
  }
  
  // Convert to Discourse topic format
  toDiscourseTopic() {
    return {
      id: this.discourseId || this.id,
      title: this.title,
      raw: this.message, // Initial post content
      created_at: this.createdAt,
      updated_at: this.updatedAt,
      user_id: this.creatorId,
      category_id: this.categoryId,
      slug: this.slug || this._generateSlug(),
      views: this.views,
      posts_count: this.postsCount,
      closed: this.closed || this.locked,
      archived: this.archived,
      tags: this.tags
    };
  }
  
  // Create from Canvas discussion
  static fromCanvasDiscussion(canvasDiscussion) {
    const discussion = new Discussion({
      ...canvasDiscussion,
      canvasId: canvasDiscussion.id,
      createdAt: canvasDiscussion.created_at,
      updatedAt: canvasDiscussion.updated_at,
      creatorId: canvasDiscussion.user_id,
      courseId: canvasDiscussion.course_id,
      allowRating: canvasDiscussion.allow_rating,
      onlyGradersCanRate: canvasDiscussion.only_graders_can_rate
    });
    
    discussion.sourceSystem = 'canvas';
    return discussion;
  }
  
  // Create from Discourse topic
  static fromDiscourseTopic(topic, post) {
    const discussion = new Discussion({
      title: topic.title,
      message: post?.raw || '', // First post content
      discourseId: topic.id,
      createdAt: topic.created_at,
      updatedAt: topic.updated_at,
      creatorId: topic.user_id,
      categoryId: topic.category_id,
      slug: topic.slug,
      views: topic.views,
      postsCount: topic.posts_count,
      closed: topic.closed,
      archived: topic.archived,
      tags: topic.tags || []
    });
    
    discussion.sourceSystem = 'discourse';
    return discussion;
  }
}

export default Discussion;