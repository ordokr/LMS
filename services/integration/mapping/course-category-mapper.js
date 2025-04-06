/**
 * Course-Category Mapper for Canvas-Discourse Integration
 * Handles mapping between Canvas courses and Discourse categories
 */
const db = require('../../database');

class CourseCategoryMapper {
  /**
   * Create a new course-category mapper
   * @param {Object} options Configuration options
   * @param {Object} options.canvasClient Canvas API client
   * @param {Object} options.discourseClient Discourse API client
   */
  constructor(options = {}) {
    this.canvasClient = options.canvasClient;
    this.discourseClient = options.discourseClient;
    
    if (!this.canvasClient) {
      throw new Error('Canvas API client is required');
    }
    
    if (!this.discourseClient) {
      throw new Error('Discourse API client is required');
    }
  }
  
  /**
   * Get Discourse category for Canvas course
   * @param {number} courseId Canvas course ID
   * @returns {Promise<Object|null>} Discourse category or null if not mapped
   */
  async getDiscourseCategory(courseId) {
    try {
      const result = await db.query(
        'SELECT discourse_category_id FROM course_category_mappings WHERE canvas_course_id = $1',
        [courseId]
      );
      
      if (!result.rows.length) {
        return null;
      }
      
      const categoryId = result.rows[0].discourse_category_id;
      return this.discourseClient.getCategory(categoryId);
    } catch (error) {
      console.error(`Error getting Discourse category for course ${courseId}:`, error);
      throw new Error(`Failed to get Discourse category: ${error.message}`);
    }
  }
  
  /**
   * Create Discourse category for Canvas course
   * @param {number} courseId Canvas course ID
   * @returns {Promise<Object>} Created Discourse category
   */
  async createDiscourseCategory(courseId) {
    try {
      // Get course details from Canvas
      const course = await this.canvasClient.getCourse(courseId);
      
      if (!course) {
        throw new Error(`Course ${courseId} not found in Canvas`);
      }
      
      // Check if mapping already exists
      const existing = await this.getDiscourseCategory(courseId);
      if (existing) {
        return existing;
      }
      
      // Generate category data
      const categoryData = {
        name: course.name,
        color: this.generateColorFromCourse(courseId),
        text_color: "FFFFFF",
        description: `Discussion forum for ${course.name} (Canvas Course ID: ${courseId})`,
        permissions: {
          everyone: 0, // no access
          staff: 3     // full access (see, reply, create)
        }
      };
      
      // Create category in Discourse
      const category = await this.discourseClient.createCategory(categoryData);
      
      // Store mapping
      await db.query(
        'INSERT INTO course_category_mappings (canvas_course_id, discourse_category_id) VALUES ($1, $2)',
        [courseId, category.id]
      );
      
      return category;
    } catch (error) {
      console.error(`Error creating Discourse category for course ${courseId}:`, error);
      throw new Error(`Failed to create Discourse category: ${error.message}`);
    }
  }
  
  /**
   * Update Discourse category for Canvas course
   * @param {number} courseId Canvas course ID
   * @returns {Promise<Object>} Updated Discourse category
   */
  async updateDiscourseCategory(courseId) {
    try {
      // Get course details from Canvas
      const course = await this.canvasClient.getCourse(courseId);
      
      if (!course) {
        throw new Error(`Course ${courseId} not found in Canvas`);
      }
      
      // Get existing category
      const result = await db.query(
        'SELECT discourse_category_id FROM course_category_mappings WHERE canvas_course_id = $1',
        [courseId]
      );
      
      if (!result.rows.length) {
        return this.createDiscourseCategory(courseId);
      }
      
      const categoryId = result.rows[0].discourse_category_id;
      
      // Update category in Discourse
      const categoryData = {
        name: course.name,
        description: `Discussion forum for ${course.name} (Canvas Course ID: ${courseId})`
      };
      
      const category = await this.discourseClient.updateCategory(categoryId, categoryData);
      
      return category;
    } catch (error) {
      console.error(`Error updating Discourse category for course ${courseId}:`, error);
      throw new Error(`Failed to update Discourse category: ${error.message}`);
    }
  }
  
  /**
   * Delete mapping between Canvas course and Discourse category
   * @param {number} courseId Canvas course ID
   * @param {boolean} deleteCategory Whether to delete the Discourse category
   * @returns {Promise<boolean>} Success
   */
  async deleteMapping(courseId, deleteCategory = false) {
    try {
      // Get existing category
      const result = await db.query(
        'SELECT discourse_category_id FROM course_category_mappings WHERE canvas_course_id = $1',
        [courseId]
      );
      
      if (!result.rows.length) {
        return true; // Nothing to delete
      }
      
      const categoryId = result.rows[0].discourse_category_id;
      
      // Delete category in Discourse if requested
      if (deleteCategory) {
        await this.discourseClient.deleteCategory(categoryId);
      }
      
      // Delete mapping
      await db.query(
        'DELETE FROM course_category_mappings WHERE canvas_course_id = $1',
        [courseId]
      );
      
      return true;
    } catch (error) {
      console.error(`Error deleting mapping for course ${courseId}:`, error);
      throw new Error(`Failed to delete mapping: ${error.message}`);
    }
  }
  
  /**
   * Generate a color for Discourse category based on course ID
   * @param {number} courseId Canvas course ID
   * @returns {string} Hex color code
   */
  generateColorFromCourse(courseId) {
    // Generate a deterministic but seemingly random color based on course ID
    const hue = (courseId * 137) % 360;
    const saturation = 65;
    const lightness = 45;
    
    // Convert HSL to hex
    const h = hue / 360;
    const s = saturation / 100;
    const l = lightness / 100;
    
    const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
    const p = 2 * l - q;
    
    const r = Math.round(this.hueToRgb(p, q, h + 1/3) * 255);
    const g = Math.round(this.hueToRgb(p, q, h) * 255);
    const b = Math.round(this.hueToRgb(p, q, h - 1/3) * 255);
    
    return `${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}`;
  }
  
  /**
   * Helper function for HSL to RGB conversion
   */
  hueToRgb(p, q, t) {
    if (t < 0) t += 1;
    if (t > 1) t -= 1;
    if (t < 1/6) return p + (q - p) * 6 * t;
    if (t < 1/2) return q;
    if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
    return p;
  }
}

module.exports = CourseCategoryMapper;