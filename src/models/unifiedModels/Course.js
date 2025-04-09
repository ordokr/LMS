/**
 * Unified Course model
 * Maps between Canvas Course and Discourse Category
 */
class Course {
  constructor(data = {}) {
    // Core fields
    this.id = data.id || null;
    this.title = data.title || data.name || '';
    this.description = data.description || data.about || '';
    this.createdAt = data.createdAt || data.created_at || new Date();
    this.updatedAt = data.updatedAt || data.updated_at || new Date();
    
    // Canvas-specific fields
    this.canvasId = data.canvasId || data.canvas_id || null;
    this.courseCode = data.courseCode || data.course_code || '';
    this.startDate = data.startDate || data.start_at || null;
    this.endDate = data.endDate || data.end_at || null;
    this.enrollments = data.enrollments || [];
    this.term = data.term || {};
    
    // Discourse-specific fields
    this.discourseId = data.discourseId || data.discourse_id || null;
    this.slug = data.slug || this._generateSlug();
    this.color = data.color || data.category_color || '#0073A7'; // Default Canvas blue
    this.position = data.position || 0;
    this.parentId = data.parentId || data.parent_id || null;
    
    // Integration fields
    this.lastSync = data.lastSync || null;
    this.sourceSystem = data.sourceSystem || 'canvas';
  }
  
  _generateSlug() {
    return this.title
      ? this.title.toLowerCase().replace(/[^a-z0-9]+/g, '-')
      : '';
  }
  
  // Convert to Canvas course format
  toCanvasCourse() {
    return {
      id: this.canvasId || this.id,
      name: this.title,
      course_code: this.courseCode,
      start_at: this.startDate,
      end_at: this.endDate,
      description: this.description,
      created_at: this.createdAt,
      updated_at: this.updatedAt,
      enrollments: this.enrollments,
      term: this.term
    };
  }
  
  // Convert to Discourse category format
  toDiscourseCategory() {
    return {
      id: this.discourseId || this.id,
      name: this.title,
      description: this.description,
      slug: this.slug || this._generateSlug(),
      color: this.color,
      created_at: this.createdAt,
      updated_at: this.updatedAt,
      position: this.position,
      parent_category_id: this.parentId
    };
  }
  
  // Create from Canvas course
  static fromCanvasCourse(canvasCourse) {
    const course = new Course({
      ...canvasCourse,
      title: canvasCourse.name,
      canvasId: canvasCourse.id,
      createdAt: canvasCourse.created_at,
      updatedAt: canvasCourse.updated_at,
      startDate: canvasCourse.start_at,
      endDate: canvasCourse.end_at,
      courseCode: canvasCourse.course_code
    });
    
    course.sourceSystem = 'canvas';
    return course;
  }
  
  // Create from Discourse category
  static fromDiscourseCategory(category) {
    const course = new Course({
      title: category.name,
      description: category.description,
      discourseId: category.id,
      createdAt: category.created_at,
      updatedAt: category.updated_at,
      slug: category.slug,
      color: category.color,
      position: category.position,
      parentId: category.parent_category_id
    });
    
    course.sourceSystem = 'discourse';
    return course;
  }
}

export default Course;