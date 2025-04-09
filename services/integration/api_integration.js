/**
 * Canvas-Discourse API Integration
 * 
 * This module provides unified API operations that bridge Canvas and Discourse systems.
 * It handles the transformation of data models between systems and ensures consistency.
 */

const { logger } = require('../../shared/logger');
const { canvasApi } = require('../api/canvas_client');
const { discourseApi } = require('../api/discourse_client');
const { syncService } = require('./sync_service');
const { ModelMapper } = require('./model_mapper');

class ApiIntegration {
  constructor() {
    this.modelMapper = new ModelMapper();
  }

  /**
   * Initialize the API integration
   */
  async initialize() {
    logger.info('Initializing API integration service');
    return true;
  }

  /**
   * Create a user in both systems with proper linkage
   * 
   * @param {Object} userData - User data (Canvas format)
   * @returns {Object} - Created user with IDs from both systems
   */
  async createUser(userData) {
    try {
      logger.info(`Creating integrated user: ${userData.name}`);
      
      // Step 1: Create user in Canvas
      const canvasUser = await canvasApi.users.create(userData);
      
      // Step 2: Transform to Discourse format
      const discourseUserData = this.modelMapper.canvasToDiscourseUser(canvasUser);
      
      // Step 3: Create user in Discourse
      const discourseUser = await discourseApi.users.create(discourseUserData);
      
      // Step 4: Record the mapping between the two users
      await this.modelMapper.saveMapping('user', canvasUser.id, discourseUser.id);
      
      // Step 5: Return the integrated user object
      return {
        canvas: canvasUser,
        discourse: discourseUser,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to create integrated user: ${error.message}`);
      throw error;
    }
  }

  /**
   * Update a user in both systems
   * 
   * @param {string} canvasUserId - Canvas user ID
   * @param {Object} userData - Updated user data
   * @returns {Object} - Updated user with data from both systems
   */
  async updateUser(canvasUserId, userData) {
    try {
      logger.info(`Updating integrated user: ${canvasUserId}`);
      
      // Step 1: Get the mapping to find Discourse user ID
      const mapping = await this.modelMapper.getMapping('user', canvasUserId, 'canvas');
      
      if (!mapping) {
        throw new Error(`No mapping found for Canvas user ID: ${canvasUserId}`);
      }
      
      // Step 2: Update user in Canvas
      const canvasUser = await canvasApi.users.update(canvasUserId, userData);
      
      // Step 3: Transform to Discourse format
      const discourseUserData = this.modelMapper.canvasToDiscourseUser(canvasUser);
      
      // Step 4: Update user in Discourse
      const discourseUser = await discourseApi.users.update(mapping.targetId, discourseUserData);
      
      // Step 5: Return the integrated user object
      return {
        canvas: canvasUser,
        discourse: discourseUser,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to update integrated user: ${error.message}`);
      throw error;
    }
  }

  /**
   * Create a course in Canvas and corresponding category in Discourse
   * 
   * @param {Object} courseData - Course data (Canvas format)
   * @returns {Object} - Created course with IDs from both systems
   */
  async createCourse(courseData) {
    try {
      logger.info(`Creating integrated course: ${courseData.name}`);
      
      // Step 1: Create course in Canvas
      const canvasCourse = await canvasApi.courses.create(courseData);
      
      // Step 2: Transform to Discourse format (category)
      const discourseCategoryData = this.modelMapper.canvasToDiscourseCategory(canvasCourse);
      
      // Step 3: Create category in Discourse
      const discourseCategory = await discourseApi.categories.create(discourseCategoryData);
      
      // Step 4: Record the mapping between the two entities
      await this.modelMapper.saveMapping('course', canvasCourse.id, discourseCategory.id);
      
      // Step 5: Trigger synchronization for related entities (e.g., enrollments)
      await syncService.publishEvent(
        'high', 
        'course_creation',
        'create', 
        'canvas', 
        { courseId: canvasCourse.id }
      );
      
      // Step 6: Return the integrated course object
      return {
        canvas: canvasCourse,
        discourse: discourseCategory,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to create integrated course: ${error.message}`);
      throw error;
    }
  }

  /**
   * Create an assignment in Canvas and corresponding topic in Discourse
   * 
   * @param {string} courseId - Canvas course ID
   * @param {Object} assignmentData - Assignment data
   * @returns {Object} - Created assignment with IDs from both systems
   */
  async createAssignment(courseId, assignmentData) {
    try {
      logger.info(`Creating integrated assignment in course ${courseId}: ${assignmentData.name}`);
      
      // Step 1: Get the course mapping
      const courseMapping = await this.modelMapper.getMapping('course', courseId, 'canvas');
      
      if (!courseMapping) {
        throw new Error(`No mapping found for Canvas course ID: ${courseId}`);
      }
      
      // Step 2: Create assignment in Canvas
      const canvasAssignment = await canvasApi.assignments.create(courseId, assignmentData);
      
      // Step 3: Transform to Discourse format (topic)
      const discourseTopicData = this.modelMapper.canvasToDiscourseTopic(
        canvasAssignment, 
        courseMapping.targetId
      );
      
      // Step 4: Create topic in Discourse
      const discourseTopic = await discourseApi.topics.create(discourseTopicData);
      
      // Step 5: Record the mapping between the two entities
      await this.modelMapper.saveMapping('assignment', canvasAssignment.id, discourseTopic.id);
      
      // Step 6: Return the integrated assignment object
      return {
        canvas: canvasAssignment,
        discourse: discourseTopic,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to create integrated assignment: ${error.message}`);
      throw error;
    }
  }

  /**
   * Submit an assignment in Canvas and create a post in Discourse
   * 
   * @param {string} courseId - Canvas course ID
   * @param {string} assignmentId - Canvas assignment ID
   * @param {string} userId - Canvas user ID
   * @param {Object} submissionData - Submission data
   * @returns {Object} - Created submission with IDs from both systems
   */
  async submitAssignment(courseId, assignmentId, userId, submissionData) {
    try {
      logger.info(`Creating integrated submission for assignment ${assignmentId} by user ${userId}`);
      
      // Step 1: Get the necessary mappings
      const assignmentMapping = await this.modelMapper.getMapping('assignment', assignmentId, 'canvas');
      const userMapping = await this.modelMapper.getMapping('user', userId, 'canvas');
      
      if (!assignmentMapping) {
        throw new Error(`No mapping found for Canvas assignment ID: ${assignmentId}`);
      }
      
      if (!userMapping) {
        throw new Error(`No mapping found for Canvas user ID: ${userId}`);
      }
      
      // Step 2: Create submission in Canvas
      const canvasSubmission = await canvasApi.submissions.create(
        courseId,
        assignmentId,
        userId,
        submissionData
      );
      
      // Step 3: Transform to Discourse format (post)
      const discoursePostData = this.modelMapper.canvasToDiscoursePost(
        canvasSubmission,
        assignmentMapping.targetId,
        userMapping.targetId
      );
      
      // Step 4: Create post in Discourse
      const discoursePost = await discourseApi.posts.create(discoursePostData);
      
      // Step 5: Record the mapping between the two entities
      await this.modelMapper.saveMapping('submission', canvasSubmission.id, discoursePost.id);
      
      // Step 6: Publish event for critical synchronization (grades)
      await syncService.publishEvent(
        'critical', 
        'submission', 
        'create', 
        'canvas', 
        { submissionId: canvasSubmission.id }
      );
      
      // Step 7: Return the integrated submission object
      return {
        canvas: canvasSubmission,
        discourse: discoursePost,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to create integrated submission: ${error.message}`);
      throw error;
    }
  }

  /**
   * Grade a submission in Canvas and update the corresponding post in Discourse
   * 
   * @param {string} courseId - Canvas course ID
   * @param {string} assignmentId - Canvas assignment ID
   * @param {string} userId - Canvas user ID
   * @param {Object} gradeData - Grade data
   * @returns {Object} - Updated submission with data from both systems
   */
  async gradeSubmission(courseId, assignmentId, userId, gradeData) {
    try {
      logger.info(`Grading submission for assignment ${assignmentId} by user ${userId}`);
      
      // Step 1: Get submission ID from Canvas
      const canvasSubmission = await canvasApi.submissions.get(courseId, assignmentId, userId);
      
      // Step 2: Get the mapping to find Discourse post ID
      const submissionMapping = await this.modelMapper.getMapping('submission', canvasSubmission.id, 'canvas');
      
      if (!submissionMapping) {
        throw new Error(`No mapping found for Canvas submission ID: ${canvasSubmission.id}`);
      }
      
      // Step 3: Update grade in Canvas
      const updatedCanvasSubmission = await canvasApi.submissions.grade(
        courseId,
        assignmentId,
        userId,
        gradeData
      );
      
      // Step 4: Transform to Discourse format
      const discoursePostData = this.modelMapper.canvasGradeToDiscoursePost(
        updatedCanvasSubmission,
        submissionMapping.targetId
      );
      
      // Step 5: Update post in Discourse
      const discoursePost = await discourseApi.posts.update(
        submissionMapping.targetId,
        discoursePostData
      );
      
      // Step 6: Publish critical sync event for grade
      await syncService.publishEvent(
        'critical',
        'grade',
        'update',
        'canvas',
        { submissionId: canvasSubmission.id }
      );
      
      // Step 7: Return the integrated submission object with grade
      return {
        canvas: updatedCanvasSubmission,
        discourse: discoursePost,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to update integrated grade: ${error.message}`);
      throw error;
    }
  }

  /**
   * Create a discussion in Canvas and corresponding topic in Discourse
   * 
   * @param {string} courseId - Canvas course ID
   * @param {Object} discussionData - Discussion data
   * @returns {Object} - Created discussion with IDs from both systems
   */
  async createDiscussion(courseId, discussionData) {
    try {
      logger.info(`Creating integrated discussion in course ${courseId}: ${discussionData.title}`);
      
      // Step 1: Get the course mapping
      const courseMapping = await this.modelMapper.getMapping('course', courseId, 'canvas');
      
      if (!courseMapping) {
        throw new Error(`No mapping found for Canvas course ID: ${courseId}`);
      }
      
      // Step 2: Create discussion in Canvas
      const canvasDiscussion = await canvasApi.discussions.create(courseId, discussionData);
      
      // Step 3: Transform to Discourse format
      const discourseTopicData = this.modelMapper.canvasToDiscourseDiscussion(
        canvasDiscussion,
        courseMapping.targetId
      );
      
      // Step 4: Create topic in Discourse
      const discourseTopic = await discourseApi.topics.create(discourseTopicData);
      
      // Step 5: Record the mapping between the two entities
      await this.modelMapper.saveMapping('discussion', canvasDiscussion.id, discourseTopic.id);
      
      // Step 6: Return the integrated discussion object
      return {
        canvas: canvasDiscussion,
        discourse: discourseTopic,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to create integrated discussion: ${error.message}`);
      throw error;
    }
  }

  /**
   * Get an entity with data from both systems
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceId - ID in the source system
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @returns {Object} - Entity data from both systems
   */
  async getIntegratedEntity(entityType, sourceId, sourceSystem = 'canvas') {
    try {
      // Step 1: Get the mapping
      const mapping = await this.modelMapper.getMapping(entityType, sourceId, sourceSystem);
      
      if (!mapping) {
        throw new Error(`No mapping found for ${entityType} with ID ${sourceId} in ${sourceSystem}`);
      }
      
      // Step 2: Get data from both systems
      let sourceData, targetData;
      
      if (sourceSystem === 'canvas') {
        sourceData = await this.getCanvasEntity(entityType, sourceId);
        targetData = await this.getDiscourseEntity(entityType, mapping.targetId);
      } else {
        sourceData = await this.getDiscourseEntity(entityType, sourceId);
        targetData = await this.getCanvasEntity(entityType, mapping.targetId);
      }
      
      // Step 3: Return the integrated entity
      return {
        canvas: sourceSystem === 'canvas' ? sourceData : targetData,
        discourse: sourceSystem === 'canvas' ? targetData : sourceData,
        integrated: true
      };
    } catch (error) {
      logger.error(`Failed to get integrated entity: ${error.message}`);
      throw error;
    }
  }

  /**
   * Get entity data from Canvas
   * 
   * @param {string} entityType - Type of entity
   * @param {string} id - Entity ID
   * @returns {Object} - Entity data from Canvas
   */
  async getCanvasEntity(entityType, id) {
    switch (entityType) {
      case 'user':
        return await canvasApi.users.get(id);
      case 'course':
        return await canvasApi.courses.get(id);
      case 'assignment':
        return await canvasApi.assignments.get(id);
      case 'submission':
        return await canvasApi.submissions.get(id);
      case 'discussion':
        return await canvasApi.discussions.get(id);
      default:
        throw new Error(`Unsupported Canvas entity type: ${entityType}`);
    }
  }

  /**
   * Get entity data from Discourse
   * 
   * @param {string} entityType - Type of entity
   * @param {string} id - Entity ID
   * @returns {Object} - Entity data from Discourse
   */
  async getDiscourseEntity(entityType, id) {
    switch (entityType) {
      case 'user':
        return await discourseApi.users.get(id);
      case 'course': // maps to category
        return await discourseApi.categories.get(id);
      case 'assignment': // maps to topic
      case 'discussion': // maps to topic
        return await discourseApi.topics.get(id);
      case 'submission': // maps to post
        return await discourseApi.posts.get(id);
      default:
        throw new Error(`Unsupported Discourse entity type: ${entityType}`);
    }
  }
}

module.exports = {
  apiIntegration: new ApiIntegration()
};
