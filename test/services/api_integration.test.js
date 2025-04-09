/**
 * Unit tests for the API Integration component
 */

const { describe, it, beforeEach, afterEach, expect, jest } = require('@jest/globals');

// Mock dependencies
jest.mock('../../shared/logger', () => ({
  logger: {
    info: jest.fn(),
    error: jest.fn(),
    warn: jest.fn()
  }
}));

jest.mock('../api/canvas_client', () => ({
  canvasApi: {
    users: {
      create: jest.fn(),
      update: jest.fn(),
      get: jest.fn()
    },
    courses: {
      create: jest.fn(),
      update: jest.fn(),
      get: jest.fn()
    },
    assignments: {
      create: jest.fn(),
      update: jest.fn(),
      get: jest.fn()
    },
    submissions: {
      create: jest.fn(),
      grade: jest.fn(),
      get: jest.fn()
    },
    discussions: {
      create: jest.fn(),
      get: jest.fn()
    }
  }
}));

jest.mock('../api/discourse_client', () => ({
  discourseApi: {
    users: {
      create: jest.fn(),
      update: jest.fn(),
      get: jest.fn()
    },
    categories: {
      create: jest.fn(),
      update: jest.fn(),
      get: jest.fn()
    },
    topics: {
      create: jest.fn(),
      update: jest.fn(),
      get: jest.fn()
    },
    posts: {
      create: jest.fn(),
      update: jest.fn(),
      get: jest.fn()
    }
  }
}));

jest.mock('../../services/integration/sync_service', () => ({
  syncService: {
    publishEvent: jest.fn().mockResolvedValue('tx-123')
  }
}));

jest.mock('../../services/integration/model_mapper', () => {
  const mockModelMapper = {
    saveMapping: jest.fn(),
    getMapping: jest.fn(),
    canvasToDiscourseUser: jest.fn(),
    canvasToDiscourseCategory: jest.fn(),
    canvasToDiscourseTopic: jest.fn(),
    canvasToDiscoursePost: jest.fn(),
    canvasGradeToDiscoursePost: jest.fn(),
    canvasToDiscourseDiscussion: jest.fn()
  };
  
  return {
    ModelMapper: jest.fn().mockImplementation(() => mockModelMapper)
  };
});

// Import module under test
const { apiIntegration } = require('../../services/integration/api_integration');
const { canvasApi } = require('../api/canvas_client');
const { discourseApi } = require('../api/discourse_client');
const { syncService } = require('../../services/integration/sync_service');

describe('API Integration', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('createUser', () => {
    it('should create a user in both systems with proper linkage', async () => {
      // Setup
      const canvasUser = { id: '123', name: 'Jane Doe', email: 'jane@example.com' };
      const discourseUser = { id: '456', username: 'jane', email: 'jane@example.com' };
      
      canvasApi.users.create.mockResolvedValueOnce(canvasUser);
      discourseApi.users.create.mockResolvedValueOnce(discourseUser);
      apiIntegration.modelMapper.canvasToDiscourseUser.mockReturnValueOnce({
        name: 'Jane Doe',
        username: 'jane',
        email: 'jane@example.com'
      });
      apiIntegration.modelMapper.saveMapping.mockResolvedValueOnce({
        entityType: 'user',
        sourceId: '123',
        targetId: '456'
      });

      // Execute
      const result = await apiIntegration.createUser({
        name: 'Jane Doe',
        email: 'jane@example.com'
      });

      // Verify
      expect(canvasApi.users.create).toHaveBeenCalled();
      expect(apiIntegration.modelMapper.canvasToDiscourseUser).toHaveBeenCalledWith(canvasUser);
      expect(discourseApi.users.create).toHaveBeenCalled();
      expect(apiIntegration.modelMapper.saveMapping).toHaveBeenCalledWith('user', '123', '456');
      
      expect(result).toEqual({
        canvas: canvasUser,
        discourse: discourseUser,
        integrated: true
      });
    });
    
    it('should handle errors during user creation', async () => {
      // Setup
      canvasApi.users.create.mockRejectedValueOnce(new Error('Canvas API error'));
      
      // Execute & Verify
      await expect(apiIntegration.createUser({ name: 'Test' }))
        .rejects.toThrow('Canvas API error');
    });
  });

  describe('createCourse', () => {
    it('should create a course in Canvas and category in Discourse', async () => {
      // Setup
      const canvasCourse = { 
        id: '101', 
        name: 'Biology 101', 
        course_code: 'BIO101' 
      };
      const discourseCategory = { 
        id: '201', 
        name: 'Biology 101' 
      };
      
      canvasApi.courses.create.mockResolvedValueOnce(canvasCourse);
      discourseApi.categories.create.mockResolvedValueOnce(discourseCategory);
      apiIntegration.modelMapper.canvasToDiscourseCategory.mockReturnValueOnce({
        name: 'Biology 101',
        slug: 'bio101'
      });
      apiIntegration.modelMapper.saveMapping.mockResolvedValueOnce({
        entityType: 'course',
        sourceId: '101',
        targetId: '201'
      });

      // Execute
      const result = await apiIntegration.createCourse({
        name: 'Biology 101',
        course_code: 'BIO101'
      });

      // Verify
      expect(canvasApi.courses.create).toHaveBeenCalled();
      expect(apiIntegration.modelMapper.canvasToDiscourseCategory).toHaveBeenCalledWith(canvasCourse);
      expect(discourseApi.categories.create).toHaveBeenCalled();
      expect(apiIntegration.modelMapper.saveMapping).toHaveBeenCalledWith('course', '101', '201');
      expect(syncService.publishEvent).toHaveBeenCalledWith(
        'high', 
        'course_creation',
        'create', 
        'canvas', 
        { courseId: '101' }
      );
      
      expect(result).toEqual({
        canvas: canvasCourse,
        discourse: discourseCategory,
        integrated: true
      });
    });
  });

  describe('createAssignment', () => {
    it('should create an assignment in Canvas and topic in Discourse', async () => {
      // Setup
      const courseMapping = {
        entityType: 'course',
        sourceId: '101',
        targetId: '201'
      };
      const canvasAssignment = { 
        id: '301', 
        name: 'Midterm Exam',
        description: 'Comprehensive exam',
        points_possible: 100
      };
      const discourseTopic = { 
        id: '401', 
        title: 'Midterm Exam' 
      };
      
      apiIntegration.modelMapper.getMapping.mockResolvedValueOnce(courseMapping);
      canvasApi.assignments.create.mockResolvedValueOnce(canvasAssignment);
      discourseApi.topics.create.mockResolvedValueOnce(discourseTopic);
      apiIntegration.modelMapper.canvasToDiscourseTopic.mockReturnValueOnce({
        title: 'Midterm Exam',
        raw: 'Comprehensive exam',
        category: '201'
      });
      apiIntegration.modelMapper.saveMapping.mockResolvedValueOnce({
        entityType: 'assignment',
        sourceId: '301',
        targetId: '401'
      });

      // Execute
      const result = await apiIntegration.createAssignment('101', {
        name: 'Midterm Exam',
        description: 'Comprehensive exam',
        points_possible: 100
      });

      // Verify
      expect(apiIntegration.modelMapper.getMapping).toHaveBeenCalledWith('course', '101', 'canvas');
      expect(canvasApi.assignments.create).toHaveBeenCalledWith('101', expect.any(Object));
      expect(apiIntegration.modelMapper.canvasToDiscourseTopic)
        .toHaveBeenCalledWith(canvasAssignment, '201');
      expect(discourseApi.topics.create).toHaveBeenCalled();
      expect(apiIntegration.modelMapper.saveMapping)
        .toHaveBeenCalledWith('assignment', '301', '401');
      
      expect(result).toEqual({
        canvas: canvasAssignment,
        discourse: discourseTopic,
        integrated: true
      });
    });
    
    it('should throw an error if course mapping not found', async () => {
      // Setup
      apiIntegration.modelMapper.getMapping.mockResolvedValueOnce(null);
      
      // Execute & Verify
      await expect(apiIntegration.createAssignment('101', {}))
        .rejects.toThrow('No mapping found for Canvas course ID: 101');
    });
  });

  describe('submitAssignment', () => {
    it('should create submission in Canvas and post in Discourse', async () => {
      // Setup
      const assignmentMapping = {
        entityType: 'assignment',
        sourceId: '301',
        targetId: '401'
      };
      const userMapping = {
        entityType: 'user',
        sourceId: '123',
        targetId: '456'
      };
      const canvasSubmission = { 
        id: '501', 
        body: 'My submission',
        submitted_at: '2025-04-08T14:32:00Z'
      };
      const discoursePost = { 
        id: '601', 
        raw: 'My submission' 
      };
      
      apiIntegration.modelMapper.getMapping
        .mockResolvedValueOnce(assignmentMapping)
        .mockResolvedValueOnce(userMapping);
        
      canvasApi.submissions.create.mockResolvedValueOnce(canvasSubmission);
      discourseApi.posts.create.mockResolvedValueOnce(discoursePost);
      
      apiIntegration.modelMapper.canvasToDiscoursePost.mockReturnValueOnce({
        topic_id: '401',
        user_id: '456',
        raw: 'My submission'
      });
      
      apiIntegration.modelMapper.saveMapping.mockResolvedValueOnce({
        entityType: 'submission',
        sourceId: '501',
        targetId: '601'
      });

      // Execute
      const result = await apiIntegration.submitAssignment('101', '301', '123', {
        body: 'My submission'
      });

      // Verify
      expect(apiIntegration.modelMapper.getMapping).toHaveBeenCalledWith('assignment', '301', 'canvas');
      expect(apiIntegration.modelMapper.getMapping).toHaveBeenCalledWith('user', '123', 'canvas');
      expect(canvasApi.submissions.create).toHaveBeenCalledWith(
        '101', '301', '123', { body: 'My submission' }
      );
      expect(apiIntegration.modelMapper.canvasToDiscoursePost)
        .toHaveBeenCalledWith(canvasSubmission, '401', '456');
      expect(discourseApi.posts.create).toHaveBeenCalled();
      expect(apiIntegration.modelMapper.saveMapping)
        .toHaveBeenCalledWith('submission', '501', '601');
      expect(syncService.publishEvent).toHaveBeenCalledWith(
        'critical', 
        'submission', 
        'create', 
        'canvas', 
        { submissionId: '501' }
      );
      
      expect(result).toEqual({
        canvas: canvasSubmission,
        discourse: discoursePost,
        integrated: true
      });
    });
  });

  describe('getIntegratedEntity', () => {
    it('should get entity data from both systems', async () => {
      // Setup
      const mapping = {
        entityType: 'user',
        sourceId: '123',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        targetId: '456'
      };
      const canvasUser = { id: '123', name: 'Jane Doe' };
      const discourseUser = { id: '456', username: 'jane' };
      
      apiIntegration.modelMapper.getMapping.mockResolvedValueOnce(mapping);
      canvasApi.users.get.mockResolvedValueOnce(canvasUser);
      discourseApi.users.get.mockResolvedValueOnce(discourseUser);

      // Execute
      const result = await apiIntegration.getIntegratedEntity('user', '123');

      // Verify
      expect(apiIntegration.modelMapper.getMapping).toHaveBeenCalledWith('user', '123', 'canvas');
      expect(canvasApi.users.get).toHaveBeenCalledWith('123');
      expect(discourseApi.users.get).toHaveBeenCalledWith('456');
      
      expect(result).toEqual({
        canvas: canvasUser,
        discourse: discourseUser,
        integrated: true
      });
    });
    
    it('should work with discourse as source system', async () => {
      // Setup
      const mapping = {
        entityType: 'user',
        sourceId: '456',
        sourceSystem: 'discourse',
        targetSystem: 'canvas',
        targetId: '123'
      };
      const canvasUser = { id: '123', name: 'Jane Doe' };
      const discourseUser = { id: '456', username: 'jane' };
      
      apiIntegration.modelMapper.getMapping.mockResolvedValueOnce(mapping);
      discourseApi.users.get.mockResolvedValueOnce(discourseUser);
      canvasApi.users.get.mockResolvedValueOnce(canvasUser);

      // Execute
      const result = await apiIntegration.getIntegratedEntity('user', '456', 'discourse');

      // Verify
      expect(result).toEqual({
        canvas: canvasUser,
        discourse: discourseUser,
        integrated: true
      });
    });
    
    it('should throw error if mapping not found', async () => {
      // Setup
      apiIntegration.modelMapper.getMapping.mockResolvedValueOnce(null);
      
      // Execute & Verify
      await expect(apiIntegration.getIntegratedEntity('user', '123'))
        .rejects.toThrow('No mapping found for user with ID 123 in canvas');
    });
  });
});
