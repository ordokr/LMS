/**
 * Unified Assignment model
 * Maps between Canvas Assignment and Discourse CustomField
 */
class Assignment {
  constructor(data = {}) {
    // Core fields
    this.id = data.id || null;
    this.title = data.title || data.name || '';
    this.description = data.description || '';
    this.createdAt = data.createdAt || data.created_at || new Date();
    this.updatedAt = data.updatedAt || data.updated_at || new Date();
    
    // Canvas-specific fields
    this.canvasId = data.canvasId || data.canvas_id || null;
    this.courseId = data.courseId || data.course_id || null;
    this.dueAt = data.dueAt || data.due_at || null;
    this.lockAt = data.lockAt || data.lock_at || null;
    this.unlockAt = data.unlockAt || data.unlock_at || null;
    this.pointsPossible = data.pointsPossible || data.points_possible || 0;
    this.submissionTypes = data.submissionTypes || data.submission_types || [];
    this.gradingType = data.gradingType || data.grading_type || 'points';
    
    // Discourse-specific fields
    this.discourseId = data.discourseId || data.discourse_id || null;
    this.topicId = data.topicId || data.topic_id || null;
    this.categoryId = data.categoryId || data.category_id || null;
    
    // Integration fields
    this.lastSync = data.lastSync || null;
    this.sourceSystem = data.sourceSystem || 'canvas';
  }
  
  // Convert to Canvas assignment format
  toCanvasAssignment() {
    return {
      id: this.canvasId || this.id,
      name: this.title,
      description: this.description,
      created_at: this.createdAt,
      updated_at: this.updatedAt,
      course_id: this.courseId,
      due_at: this.dueAt,
      lock_at: this.lockAt,
      unlock_at: this.unlockAt,
      points_possible: this.pointsPossible,
      submission_types: this.submissionTypes,
      grading_type: this.gradingType
    };
  }
  
  // Convert to Discourse custom fields (stored on topics)
  toDiscourseCustomFields() {
    return {
      assignment_id: this.id.toString(),
      due_at: this.dueAt ? this.dueAt.toISOString() : null,
      points_possible: this.pointsPossible.toString(),
      grading_type: this.gradingType,
      lock_at: this.lockAt ? this.lockAt.toISOString() : null,
      unlock_at: this.unlockAt ? this.unlockAt.toISOString() : null
    };
  }
  
  // Create from Canvas assignment
  static fromCanvasAssignment(canvasAssignment) {
    const assignment = new Assignment({
      ...canvasAssignment,
      title: canvasAssignment.name,
      canvasId: canvasAssignment.id,
      createdAt: canvasAssignment.created_at,
      updatedAt: canvasAssignment.updated_at,
      courseId: canvasAssignment.course_id,
      dueAt: canvasAssignment.due_at,
      lockAt: canvasAssignment.lock_at,
      unlockAt: canvasAssignment.unlock_at,
      pointsPossible: canvasAssignment.points_possible,
      submissionTypes: canvasAssignment.submission_types,
      gradingType: canvasAssignment.grading_type
    });
    
    assignment.sourceSystem = 'canvas';
    return assignment;
  }
  
  // Create from Discourse topic with custom fields
  static fromDiscourseTopic(topic) {
    if (!topic.custom_fields) {
      throw new Error('Topic does not contain assignment custom fields');
    }
    
    const cf = topic.custom_fields;
    
    const assignment = new Assignment({
      id: cf.assignment_id || null,
      title: topic.title,
      description: topic.first_post?.raw || '',
      discourseId: `topic-${topic.id}`,
      topicId: topic.id,
      categoryId: topic.category_id,
      createdAt: topic.created_at,
      updatedAt: topic.updated_at,
      dueAt: cf.due_at ? new Date(cf.due_at) : null,
      pointsPossible: parseFloat(cf.points_possible || '0'),
      gradingType: cf.grading_type || 'points',
      lockAt: cf.lock_at ? new Date(cf.lock_at) : null,
      unlockAt: cf.unlock_at ? new Date(cf.unlock_at) : null
    });
    
    assignment.sourceSystem = 'discourse';
    return assignment;
  }
}

export default Assignment;