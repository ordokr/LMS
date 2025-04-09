/**
 * Model Mapper
 * 
 * Maps data models between Canvas and Discourse systems,
 * providing bidirectional transformation and persistent mapping records.
 */

const { logger } = require('../../shared/logger');
const { db } = require('../../shared/db');

class ModelMapper {
  /**
   * Create a new model mapper instance
   */
  constructor() {
    this.mappingCache = {};
  }

  /**
   * Save a mapping between Canvas and Discourse entities
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceId - ID in the source system (usually Canvas)
   * @param {string} targetId - ID in the target system (usually Discourse)
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @returns {Object} - Created mapping
   */
  async saveMapping(entityType, sourceId, targetId, sourceSystem = 'canvas') {
    try {
      const targetSystem = sourceSystem === 'canvas' ? 'discourse' : 'canvas';
      
      // Check if mapping already exists
      const existingMapping = await db.entityMappings.findOne({
        where: {
          entityType,
          sourceId,
          sourceSystem
        }
      });
      
      if (existingMapping) {
        // Update existing mapping
        await db.entityMappings.update(
          { targetId },
          {
            where: {
              entityType,
              sourceId,
              sourceSystem
            }
          }
        );
        
        // Update cache
        const cacheKey = `${entityType}:${sourceId}:${sourceSystem}`;
        this.mappingCache[cacheKey] = {
          entityType,
          sourceId,
          sourceSystem,
          targetSystem,
          targetId
        };
        
        return this.mappingCache[cacheKey];
      } else {
        // Create new mapping
        const mapping = await db.entityMappings.create({
          entityType,
          sourceId,
          sourceSystem,
          targetSystem,
          targetId
        });
        
        // Update cache
        const cacheKey = `${entityType}:${sourceId}:${sourceSystem}`;
        this.mappingCache[cacheKey] = mapping;
        
        return mapping;
      }
    } catch (error) {
      logger.error(`Failed to save entity mapping: ${error.message}`);
      throw error;
    }
  }

  /**
   * Get a mapping between Canvas and Discourse entities
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceId - ID in the source system
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @returns {Object} - Retrieved mapping
   */
  async getMapping(entityType, sourceId, sourceSystem = 'canvas') {
    try {
      // Check cache first
      const cacheKey = `${entityType}:${sourceId}:${sourceSystem}`;
      
      if (this.mappingCache[cacheKey]) {
        return this.mappingCache[cacheKey];
      }
      
      // Query database
      const mapping = await db.entityMappings.findOne({
        where: {
          entityType,
          sourceId,
          sourceSystem
        }
      });
      
      // Update cache if found
      if (mapping) {
        this.mappingCache[cacheKey] = mapping;
      }
      
      return mapping;
    } catch (error) {
      logger.error(`Failed to get entity mapping: ${error.message}`);
      throw error;
    }
  }

  /**
   * Delete a mapping between Canvas and Discourse entities
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceId - ID in the source system
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @returns {boolean} - Success indicator
   */
  async deleteMapping(entityType, sourceId, sourceSystem = 'canvas') {
    try {
      // Delete from database
      await db.entityMappings.destroy({
        where: {
          entityType,
          sourceId,
          sourceSystem
        }
      });
      
      // Remove from cache
      const cacheKey = `${entityType}:${sourceId}:${sourceSystem}`;
      delete this.mappingCache[cacheKey];
      
      return true;
    } catch (error) {
      logger.error(`Failed to delete entity mapping: ${error.message}`);
      throw error;
    }
  }

  /**
   * Get all mappings for a specific entity type
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @returns {Array} - List of mappings
   */
  async getAllMappings(entityType) {
    try {
      return await db.entityMappings.findAll({
        where: { entityType }
      });
    } catch (error) {
      logger.error(`Failed to get all mappings: ${error.message}`);
      throw error;
    }
  }

  /**
   * Transform a Canvas user to Discourse user format
   * 
   * @param {Object} canvasUser - Canvas user object
   * @returns {Object} - Discourse user object
   */
  canvasToDiscourseUser(canvasUser) {
    const username = canvasUser.email 
      ? canvasUser.email.split('@')[0].replace(/[^a-zA-Z0-9_]/g, '_')
      : `canvas_user_${canvasUser.id}`;
    
    return {
      name: canvasUser.name,
      username: username,
      email: canvasUser.email,
      password: `canvas_${Date.now()}`, // Will be reset by SSO
      active: canvasUser.workflow_state !== 'deleted',
      approved: true,
      custom_fields: {
        canvas_user_id: canvasUser.id.toString()
      }
    };
  }

  /**
   * Transform a Discourse user to Canvas user format
   * 
   * @param {Object} discourseUser - Discourse user object
   * @returns {Object} - Canvas user object
   */
  discourseToCanvasUser(discourseUser) {
    return {
      name: discourseUser.name,
      short_name: discourseUser.username,
      sortable_name: discourseUser.name,
      email: discourseUser.email,
      status: discourseUser.active ? 'active' : 'deleted'
    };
  }

  /**
   * Transform a Canvas course to Discourse category format
   * 
   * @param {Object} canvasCourse - Canvas course object
   * @returns {Object} - Discourse category object
   */
  canvasToDiscourseCategory(canvasCourse) {
    return {
      name: canvasCourse.name,
      slug: this.slugify(canvasCourse.course_code || canvasCourse.name),
      color: "0088CC", // Default color
      text_color: "FFFFFF",
      description: canvasCourse.public_description || canvasCourse.syllabus_body || '',
      custom_fields: {
        canvas_course_id: canvasCourse.id.toString(),
        canvas_course_code: canvasCourse.course_code
      }
    };
  }

  /**
   * Transform a Discourse category to Canvas course format
   * 
   * @param {Object} discourseCategory - Discourse category object
   * @returns {Object} - Canvas course object
   */
  discourseToCanvasCourse(discourseCategory) {
    const courseCode = discourseCategory.custom_fields?.canvas_course_code || discourseCategory.slug;
    
    return {
      name: discourseCategory.name,
      course_code: courseCode,
      public_description: discourseCategory.description,
      default_view: 'modules'
    };
  }

  /**
   * Transform Canvas assignment to Discourse topic format
   * 
   * @param {Object} canvasAssignment - Canvas assignment object
   * @param {string} categoryId - Discourse category ID
   * @returns {Object} - Discourse topic object
   */
  canvasToDiscourseTopic(canvasAssignment, categoryId) {
    return {
      title: canvasAssignment.name,
      raw: this.formatAssignmentBody(canvasAssignment),
      category: categoryId,
      tags: ['canvas-assignment'],
      custom_fields: {
        canvas_assignment_id: canvasAssignment.id.toString(),
        canvas_points_possible: canvasAssignment.points_possible,
        canvas_due_at: canvasAssignment.due_at
      }
    };
  }

  /**
   * Format assignment description body for Discourse
   * 
   * @param {Object} assignment - Canvas assignment
   * @returns {string} - Formatted content
   */
  formatAssignmentBody(assignment) {
    const parts = [];
    
    parts.push(`# ${assignment.name}`);
    parts.push('');
    
    if (assignment.description) {
      parts.push(assignment.description);
      parts.push('');
    }
    
    parts.push('## Assignment Details');
    parts.push('');
    parts.push(`**Points:** ${assignment.points_possible}`);
    
    if (assignment.due_at) {
      parts.push(`**Due Date:** ${new Date(assignment.due_at).toLocaleString()}`);
    }
    
    if (assignment.submission_types) {
      parts.push(`**Submission Type:** ${assignment.submission_types.join(', ')}`);
    }
    
    parts.push('');
    parts.push(`[View in Canvas](${assignment.html_url})`);
    
    return parts.join('\n');
  }

  /**
   * Transform a Canvas submission to Discourse post format
   * 
   * @param {Object} submission - Canvas submission object
   * @param {string} topicId - Discourse topic ID (for the assignment)
   * @param {string} userId - Discourse user ID
   * @returns {Object} - Discourse post object
   */
  canvasToDiscoursePost(submission, topicId, userId) {
    const content = [];
    
    content.push('**Assignment Submission**');
    content.push('');
    
    if (submission.body) {
      content.push(submission.body);
      content.push('');
    }
    
    if (submission.submission_type === 'online_url' && submission.url) {
      content.push(`Submitted URL: ${submission.url}`);
      content.push('');
    }
    
    if (submission.attachments && submission.attachments.length > 0) {
      content.push('**Attachments:**');
      submission.attachments.forEach(attachment => {
        content.push(`- [${attachment.display_name}](${attachment.url})`);
      });
      content.push('');
    }
    
    content.push(`Submitted at: ${new Date(submission.submitted_at).toLocaleString()}`);
    
    return {
      topic_id: topicId,
      user_id: userId, 
      raw: content.join('\n'),
      custom_fields: {
        canvas_submission_id: submission.id.toString(),
        canvas_submission_type: submission.submission_type,
        canvas_submitted_at: submission.submitted_at
      }
    };
  }

  /**
   * Transform Canvas grade data to Discourse post update
   * 
   * @param {Object} submission - Canvas submission with grade
   * @param {string} postId - Discourse post ID
   * @returns {Object} - Discourse post update object
   */
  canvasGradeToDiscoursePost(submission, postId) {
    const content = [];
    
    // Original post content is preserved by Discourse API
    // We're adding a reply from the instructor
    content.push('**Grade Information**');
    content.push('');
    
    if (submission.grade) {
      content.push(`**Score:** ${submission.grade}/${submission.assignment.points_possible}`);
    }
    
    if (submission.score) {
      content.push(`**Points:** ${submission.score}`);
    }
    
    if (submission.graded_at) {
      content.push(`**Graded:** ${new Date(submission.graded_at).toLocaleString()}`);
    }
    
    if (submission.grader_id) {
      content.push(`**Grader ID:** ${submission.grader_id}`);
    }
    
    if (submission.comment) {
      content.push('');
      content.push('**Instructor Comment:**');
      content.push(submission.comment);
    }
    
    return {
      id: postId,
      raw: content.join('\n'),
      custom_fields: {
        canvas_grade: submission.grade,
        canvas_score: submission.score,
        canvas_graded_at: submission.graded_at
      }
    };
  }

  /**
   * Transform Canvas discussion to Discourse topic
   * 
   * @param {Object} discussion - Canvas discussion object
   * @param {string} categoryId - Discourse category ID
   * @returns {Object} - Discourse topic object
   */
  canvasToDiscourseDiscussion(discussion, categoryId) {
    return {
      title: discussion.title,
      raw: discussion.message || discussion.title,
      category: categoryId,
      tags: ['canvas-discussion'],
      custom_fields: {
        canvas_discussion_id: discussion.id.toString(),
        canvas_discussion_type: discussion.discussion_type
      }
    };
  }

  /**
   * Slugify a string for use in URLs
   * 
   * @param {string} text - Text to slugify
   * @returns {string} - Slugified text
   */
  slugify(text) {
    return text
      .toString()
      .toLowerCase()
      .replace(/\s+/g, '-')
      .replace(/[^\w\-]+/g, '')
      .replace(/\-\-+/g, '-')
      .replace(/^-+/, '')
      .replace(/-+$/, '');
  }
}

module.exports = {
  ModelMapper
};
