import ModelFactory from '../models/ModelFactory';
import canvasApi from '../api/canvasApi';
import discourseApi from '../api/discourseApi';
import { convertObjectKeys } from '../utils/namingConventions';

/**
 * Service for synchronizing models between Canvas and Discourse
 */
class ModelSyncService {
  /**
   * Synchronize a user between Canvas and Discourse
   * @param {string} userId - User ID in source system
   * @param {string} source - Source system (canvas or discourse)
   * @returns {Object} Unified user model after synchronization
   */
  async syncUser(userId, source = 'canvas') {
    try {
      let userData;
      let unifiedUser;
      
      // Fetch from source system
      if (source === 'canvas') {
        userData = await canvasApi.getUser(userId);
        unifiedUser = ModelFactory.create('user', userData, 'canvas');
        
        // Convert and send to Discourse
        const discourseData = ModelFactory.convertToSource(unifiedUser, 'discourse');
        await discourseApi.createOrUpdateUser(discourseData);
      } else {
        userData = await discourseApi.getUser(userId);
        unifiedUser = ModelFactory.create('user', userData, 'discourse');
        
        // Convert and send to Canvas
        const canvasData = ModelFactory.convertToSource(unifiedUser, 'canvas');
        await canvasApi.createOrUpdateUser(canvasData);
      }
      
      // Update last sync time
      unifiedUser.lastSync = new Date();
      
      return unifiedUser;
    } catch (error) {
      console.error('Error syncing user:', error);
      throw new Error(`Failed to sync user ${userId} from ${source}`);
    }
  }
  
  /**
   * Synchronize a course between Canvas and Discourse
   * @param {string} courseId - Course ID in source system
   * @param {string} source - Source system (canvas or discourse)
   * @returns {Object} Unified course model after synchronization
   */
  async syncCourse(courseId, source = 'canvas') {
    try {
      let courseData;
      let unifiedCourse;
      
      // Fetch from source system
      if (source === 'canvas') {
        courseData = await canvasApi.getCourse(courseId);
        unifiedCourse = ModelFactory.create('course', courseData, 'canvas');
        
        // Convert and send to Discourse
        const discourseData = ModelFactory.convertToSource(unifiedCourse, 'discourse');
        const categoryResult = await discourseApi.createOrUpdateCategory(discourseData);
        
        // Store mapping between Canvas course and Discourse category
        unifiedCourse.discourseId = categoryResult.id;
      } else {
        courseData = await discourseApi.getCategory(courseId);
        unifiedCourse = ModelFactory.create('course', courseData, 'discourse');
        
        // Convert and send to Canvas
        const canvasData = ModelFactory.convertToSource(unifiedCourse, 'canvas');
        const courseResult = await canvasApi.createOrUpdateCourse(canvasData);
        
        // Store mapping between Discourse category and Canvas course
        unifiedCourse.canvasId = courseResult.id;
      }
      
      // Update last sync time
      unifiedCourse.lastSync = new Date();
      
      return unifiedCourse;
    } catch (error) {
      console.error('Error syncing course:', error);
      throw new Error(`Failed to sync course ${courseId} from ${source}`);
    }
  }
  
  /**
   * Synchronize a discussion between Canvas and Discourse
   * @param {string} discussionId - Discussion ID in source system
   * @param {string} courseId - Course/Category ID
   * @param {string} source - Source system (canvas or discourse)
   * @returns {Object} Unified discussion model after synchronization
   */
  async syncDiscussion(discussionId, courseId, source = 'canvas') {
    try {
      let discussionData;
      let unifiedDiscussion;
      
      // Fetch from source system
      if (source === 'canvas') {
        discussionData = await canvasApi.getDiscussion(courseId, discussionId);
        unifiedDiscussion = ModelFactory.create('discussion', discussionData, 'canvas');
        
        // Find corresponding Discourse category
        const mapping = await this._getCategoryMapping(courseId, 'canvas');
        unifiedDiscussion.categoryId = mapping.discourseId;
        
        // Convert and send to Discourse
        const discourseData = ModelFactory.convertToSource(unifiedDiscussion, 'discourse');
        const topicResult = await discourseApi.createOrUpdateTopic(discourseData);
        
        // Store mapping between Canvas discussion and Discourse topic
        unifiedDiscussion.discourseId = topicResult.id;
      } else {
        discussionData = await discourseApi.getTopic(discussionId);
        unifiedDiscussion = ModelFactory.create('discussion', discussionData, 'discourse');
        
        // Find corresponding Canvas course
        const mapping = await this._getCategoryMapping(unifiedDiscussion.categoryId, 'discourse');
        unifiedDiscussion.courseId = mapping.canvasId;
        
        // Convert and send to Canvas
        const canvasData = ModelFactory.convertToSource(unifiedDiscussion, 'canvas');
        const discussionResult = await canvasApi.createOrUpdateDiscussion(
          unifiedDiscussion.courseId, 
          canvasData
        );
        
        // Store mapping between Discourse topic and Canvas discussion
        unifiedDiscussion.canvasId = discussionResult.id;
      }
      
      // Update last sync time
      unifiedDiscussion.lastSync = new Date();
      
      return unifiedDiscussion;
    } catch (error) {
      console.error('Error syncing discussion:', error);
      throw new Error(`Failed to sync discussion ${discussionId} from ${source}`);
    }
  }
  
  /**
   * Helper method to get course-category mapping
   * @private
   */
  async _getCategoryMapping(id, source) {
    // In a real implementation, this would fetch from a database
    // For now, we'll simulate this with a placeholder
    return { canvasId: id, discourseId: id };
  }
}

export default new ModelSyncService();