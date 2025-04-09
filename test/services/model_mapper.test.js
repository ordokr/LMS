/**
 * Unit tests for the Model Mapper
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
jest.mock('../../shared/db', () => {
  const mockFindOne = jest.fn();
  const mockFindAll = jest.fn();
  const mockCreate = jest.fn();
  const mockUpdate = jest.fn();
  const mockDestroy = jest.fn();
  
  return {
    db: {
      entityMappings: {
        findOne: mockFindOne,
        findAll: mockFindAll,
        create: mockCreate,
        update: mockUpdate,
        destroy: mockDestroy
      }
    }
  };
});

// Import the module under test
const { ModelMapper } = require('../../services/integration/model_mapper');
const { db } = require('../../shared/db');

describe('ModelMapper', () => {
  let modelMapper;

  beforeEach(() => {
    modelMapper = new ModelMapper();
    jest.clearAllMocks();
  });

  describe('saveMapping', () => {
    it('should create a new mapping when none exists', async () => {
      // Setup
      const mapping = {
        entityType: 'user',
        sourceId: '123',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        targetId: '456'
      };
      db.entityMappings.findOne.mockResolvedValueOnce(null);
      db.entityMappings.create.mockResolvedValueOnce(mapping);

      // Execute
      const result = await modelMapper.saveMapping('user', '123', '456', 'canvas');

      // Verify
      expect(db.entityMappings.findOne).toHaveBeenCalledWith({
        where: {
          entityType: 'user',
          sourceId: '123',
          sourceSystem: 'canvas'
        }
      });
      expect(db.entityMappings.create).toHaveBeenCalledWith({
        entityType: 'user',
        sourceId: '123',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        targetId: '456'
      });
      expect(result).toEqual(mapping);
      
      // Check cache
      const cacheKey = 'user:123:canvas';
      expect(modelMapper.mappingCache[cacheKey]).toEqual(mapping);
    });

    it('should update an existing mapping', async () => {
      // Setup
      const existingMapping = {
        entityType: 'user',
        sourceId: '123',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        targetId: '456'
      };
      db.entityMappings.findOne.mockResolvedValueOnce(existingMapping);

      // Execute
      const result = await modelMapper.saveMapping('user', '123', '789', 'canvas');

      // Verify
      expect(db.entityMappings.update).toHaveBeenCalledWith(
        { targetId: '789' },
        {
          where: {
            entityType: 'user',
            sourceId: '123',
            sourceSystem: 'canvas'
          }
        }
      );
      
      // Check cache update
      const cacheKey = 'user:123:canvas';
      expect(modelMapper.mappingCache[cacheKey]).toEqual({
        entityType: 'user',
        sourceId: '123',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        targetId: '789'
      });
    });

    it('should handle errors when saving mapping', async () => {
      // Setup
      db.entityMappings.findOne.mockRejectedValueOnce(new Error('Database error'));

      // Execute & Verify
      await expect(modelMapper.saveMapping('user', '123', '456')).rejects.toThrow('Database error');
    });
  });

  describe('getMapping', () => {
    it('should return mapping from cache if available', async () => {
      // Setup
      const mapping = {
        entityType: 'user',
        sourceId: '123',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        targetId: '456'
      };
      const cacheKey = 'user:123:canvas';
      modelMapper.mappingCache[cacheKey] = mapping;

      // Execute
      const result = await modelMapper.getMapping('user', '123', 'canvas');

      // Verify
      expect(result).toBe(mapping);
      expect(db.entityMappings.findOne).not.toHaveBeenCalled();
    });

    it('should query database if mapping not in cache', async () => {
      // Setup
      const mapping = {
        entityType: 'user',
        sourceId: '123',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        targetId: '456'
      };
      db.entityMappings.findOne.mockResolvedValueOnce(mapping);

      // Execute
      const result = await modelMapper.getMapping('user', '123', 'canvas');

      // Verify
      expect(db.entityMappings.findOne).toHaveBeenCalledWith({
        where: {
          entityType: 'user',
          sourceId: '123',
          sourceSystem: 'canvas'
        }
      });
      expect(result).toBe(mapping);
      
      // Check cache update
      const cacheKey = 'user:123:canvas';
      expect(modelMapper.mappingCache[cacheKey]).toBe(mapping);
    });

    it('should return null when mapping not found', async () => {
      // Setup
      db.entityMappings.findOne.mockResolvedValueOnce(null);

      // Execute
      const result = await modelMapper.getMapping('user', '123', 'canvas');

      // Verify
      expect(result).toBeNull();
    });
  });

  describe('deleteMapping', () => {
    it('should delete mapping and remove from cache', async () => {
      // Setup
      const cacheKey = 'user:123:canvas';
      modelMapper.mappingCache[cacheKey] = { some: 'data' };
      db.entityMappings.destroy.mockResolvedValueOnce(1);

      // Execute
      const result = await modelMapper.deleteMapping('user', '123', 'canvas');

      // Verify
      expect(db.entityMappings.destroy).toHaveBeenCalledWith({
        where: {
          entityType: 'user',
          sourceId: '123',
          sourceSystem: 'canvas'
        }
      });
      expect(result).toBe(true);
      expect(modelMapper.mappingCache[cacheKey]).toBeUndefined();
    });
  });

  describe('getAllMappings', () => {
    it('should return all mappings for an entity type', async () => {
      // Setup
      const mappings = [
        { entityType: 'course', sourceId: '1', targetId: '101' },
        { entityType: 'course', sourceId: '2', targetId: '102' }
      ];
      db.entityMappings.findAll.mockResolvedValueOnce(mappings);

      // Execute
      const result = await modelMapper.getAllMappings('course');

      // Verify
      expect(db.entityMappings.findAll).toHaveBeenCalledWith({
        where: { entityType: 'course' }
      });
      expect(result).toBe(mappings);
    });
  });

  describe('data transformations', () => {
    describe('canvasToDiscourseUser', () => {
      it('should correctly transform a Canvas user to Discourse format', () => {
        // Setup
        const canvasUser = {
          id: '123',
          name: 'John Doe',
          email: 'john.doe@example.com'
        };

        // Execute
        const result = modelMapper.canvasToDiscourseUser(canvasUser);

        // Verify
        expect(result).toEqual({
          name: 'John Doe',
          username: 'john_doe',
          email: 'john.doe@example.com',
          password: expect.stringContaining('canvas_'),
          active: true,
          approved: true,
          custom_fields: {
            canvas_user_id: '123'
          }
        });
      });

      it('should handle Canvas user without email', () => {
        // Setup
        const canvasUser = {
          id: '123',
          name: 'John Doe'
        };

        // Execute
        const result = modelMapper.canvasToDiscourseUser(canvasUser);

        // Verify
        expect(result.username).toBe('canvas_user_123');
      });
    });

    describe('canvasToDiscourseCategory', () => {
      it('should transform Canvas course to Discourse category', () => {
        // Setup
        const canvasCourse = {
          id: '456',
          name: 'Introduction to Computer Science',
          course_code: 'CS101',
          public_description: 'Learn the basics of computer science'
        };

        // Execute
        const result = modelMapper.canvasToDiscourseCategory(canvasCourse);

        // Verify
        expect(result).toEqual({
          name: 'Introduction to Computer Science',
          slug: 'cs101',
          color: '0088CC',
          text_color: 'FFFFFF',
          description: 'Learn the basics of computer science',
          custom_fields: {
            canvas_course_id: '456',
            canvas_course_code: 'CS101'
          }
        });
      });

      it('should handle courses without course_code', () => {
        // Setup
        const canvasCourse = {
          id: '456',
          name: 'Introduction to Computer Science'
        };

        // Execute
        const result = modelMapper.canvasToDiscourseCategory(canvasCourse);

        // Verify
        expect(result.slug).toBe('introduction-to-computer-science');
      });
    });

    describe('canvasToDiscourseTopic', () => {
      it('should transform Canvas assignment to Discourse topic', () => {
        // Setup
        const canvasAssignment = {
          id: '789',
          name: 'Final Project',
          description: 'Build a web application',
          points_possible: 100,
          due_at: '2025-05-15T23:59:59Z'
        };

        // Execute
        const result = modelMapper.canvasToDiscourseTopic(canvasAssignment, '101');

        // Verify
        expect(result).toEqual({
          title: 'Final Project',
          raw: expect.stringContaining('Final Project'),
          category: '101',
          tags: ['canvas-assignment'],
          custom_fields: {
            canvas_assignment_id: '789',
            canvas_points_possible: 100,
            canvas_due_at: '2025-05-15T23:59:59Z'
          }
        });
        
        // Check formatting of the raw content
        expect(result.raw).toContain('# Final Project');
        expect(result.raw).toContain('Build a web application');
        expect(result.raw).toContain('**Points:** 100');
      });
    });

    describe('slugify', () => {
      it('should convert text to URL-friendly slug', () => {
        expect(modelMapper.slugify('Hello World!')).toBe('hello-world');
        expect(modelMapper.slugify('CS 101: Intro to Programming')).toBe('cs-101-intro-to-programming');
        expect(modelMapper.slugify('   spaces   ')).toBe('spaces');
        expect(modelMapper.slugify('special@#$characters')).toBe('specialcharacters');
      });
    });
  });
});
