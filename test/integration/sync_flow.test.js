/**
 * Integration tests for the synchronization flow between Canvas and Discourse
 * 
 * This file tests the complete synchronization flow across multiple components,
 * ensuring they work together as expected.
 */

const { expect } = require('chai');
const sinon = require('sinon');
const amqplib = require('amqplib');
const { SyncService } = require('../../services/integration/sync_service');
const { SyncTransaction } = require('../../services/integration/sync_transaction');
const { SyncState } = require('../../services/integration/sync_state');
const { ModelMapper } = require('../../services/integration/model_mapper');
const { ApiIntegration } = require('../../services/integration/api_integration');

describe('Synchronization Integration Flow', function() {
  // Increase timeout for integration tests
  this.timeout(10000);
  
  let syncService;
  let modelMapper;
  let apiIntegration;
  let mockCanvasAPI;
  let mockDiscourseAPI;
  let amqpConnection;
  let amqpChannel;
  
  // Test data
  const testUser = {
    canvas_id: '12345',
    name: 'Test User',
    email: 'testuser@example.com',
    roles: ['student']
  };
  
  const testCourse = {
    canvas_id: '67890',
    name: 'Test Course',
    code: 'TST101',
    start_date: '2025-01-15',
    end_date: '2025-05-15'
  };
  
  before(async function() {
    // Create mocks for external APIs
    mockCanvasAPI = {
      getUser: sinon.stub().resolves(testUser),
      getCourse: sinon.stub().resolves(testCourse),
      updateUser: sinon.stub().resolves({ ...testUser, name: 'Updated Test User' }),
      updateCourse: sinon.stub().resolves({ ...testCourse, name: 'Updated Test Course' })
    };
    
    mockDiscourseAPI = {
      getUser: sinon.stub().resolves(null), // Initially doesn't exist in Discourse
      createUser: sinon.stub().resolves({
        discourse_id: 'd12345',
        username: 'testuser',
        name: 'Test User',
        email: 'testuser@example.com'
      }),
      getCategory: sinon.stub().resolves(null), // Initially doesn't exist in Discourse
      createCategory: sinon.stub().resolves({
        discourse_id: 'd67890',
        name: 'Test Course',
        slug: 'tst101'
      }),
      updateUser: sinon.stub().resolves({
        discourse_id: 'd12345',
        username: 'testuser',
        name: 'Updated Test User',
        email: 'testuser@example.com'
      })
    };
    
    // Initialize RabbitMQ connection
    try {
      amqpConnection = await amqplib.connect(process.env.RABBITMQ_URL || 'amqp://localhost');
      amqpChannel = await amqpConnection.createChannel();
      
      // Ensure queues exist and are empty
      await amqpChannel.assertQueue('sync_critical', { durable: true });
      await amqpChannel.assertQueue('sync_high', { durable: true });
      await amqpChannel.assertQueue('sync_background', { durable: true });
      
      await amqpChannel.purgeQueue('sync_critical');
      await amqpChannel.purgeQueue('sync_high');
      await amqpChannel.purgeQueue('sync_background');
    } catch (error) {
      console.warn('Could not connect to RabbitMQ for integration tests. Using mock mode.');
      // Create mock channel for tests to work without RabbitMQ
      amqpChannel = {
        assertQueue: sinon.stub().resolves(),
        sendToQueue: sinon.stub().returns(true),
        consume: sinon.stub().callsFake((queue, callback) => {
          // Store callback for later manual triggering
          amqpChannel.mockCallback = callback;
          return { consumerTag: 'mock-consumer' };
        }),
        cancel: sinon.stub().resolves(),
        close: sinon.stub().resolves()
      };
      amqpConnection = {
        createChannel: sinon.stub().resolves(amqpChannel),
        close: sinon.stub().resolves()
      };
    }
    
    // Initialize components with dependencies and mocks
    modelMapper = new ModelMapper();
    apiIntegration = new ApiIntegration({
      canvasAPI: mockCanvasAPI,
      discourseAPI: mockDiscourseAPI,
      modelMapper
    });
    
    syncService = new SyncService({
      amqpConnection,
      apiIntegration,
      syncTransaction: new SyncTransaction(),
      syncState: new SyncState()
    });
    
    // Start sync service
    await syncService.start();
  });
  
  after(async function() {
    // Clean up resources
    await syncService.stop();
    if (amqpConnection && amqpConnection.close) {
      await amqpConnection.close();
    }
  });
  
  describe('User Synchronization Flow', function() {
    it('should synchronize a new user from Canvas to Discourse', async function() {
      // Trigger user sync
      const result = await syncService.publishSyncEvent({
        entityType: 'user',
        entityId: '12345',
        operation: 'SYNC',
        priority: 'high'
      });
      
      expect(result).to.be.true;
      
      // If we're using mock RabbitMQ, manually trigger message processing
      if (amqpChannel.mockCallback) {
        const message = {
          content: Buffer.from(JSON.stringify({
            entityType: 'user',
            entityId: '12345',
            operation: 'SYNC',
            priority: 'high'
          })),
          fields: { routingKey: 'sync_high' }
        };
        await amqpChannel.mockCallback(message);
      } else {
        // Wait for real message processing (may need adjustment based on your implementation)
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
      
      // Verify Canvas API was called to get user data
      expect(mockCanvasAPI.getUser.calledWith('12345')).to.be.true;
      
      // Verify Discourse API was checked for existing user
      expect(mockDiscourseAPI.getUser.calledOnce).to.be.true;
      
      // Verify user was created in Discourse
      expect(mockDiscourseAPI.createUser.calledOnce).to.be.true;
      
      // Verify model mapping was created
      const mapping = await modelMapper.getMapping('user', '12345');
      expect(mapping).to.exist;
      expect(mapping.discourseId).to.equal('d12345');
    });
    
    it('should update a user when changed in Canvas', async function() {
      // Mock Canvas API to return updated user
      mockCanvasAPI.getUser.resolves({
        ...testUser,
        name: 'Updated Test User'
      });
      
      // Mock Discourse API to return existing user
      mockDiscourseAPI.getUser.resolves({
        discourse_id: 'd12345',
        username: 'testuser',
        name: 'Test User',
        email: 'testuser@example.com'
      });
      
      // Trigger user sync
      const result = await syncService.publishSyncEvent({
        entityType: 'user',
        entityId: '12345',
        operation: 'SYNC',
        priority: 'high'
      });
      
      expect(result).to.be.true;
      
      // If we're using mock RabbitMQ, manually trigger message processing
      if (amqpChannel.mockCallback) {
        const message = {
          content: Buffer.from(JSON.stringify({
            entityType: 'user',
            entityId: '12345',
            operation: 'SYNC',
            priority: 'high'
          })),
          fields: { routingKey: 'sync_high' }
        };
        await amqpChannel.mockCallback(message);
      } else {
        // Wait for real message processing
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
      
      // Verify user was updated in Discourse
      expect(mockDiscourseAPI.updateUser.calledOnce).to.be.true;
      expect(mockDiscourseAPI.updateUser.firstCall.args[0]).to.have.property('name', 'Updated Test User');
    });
  });
  
  describe('Course Synchronization Flow', function() {
    it('should synchronize a new course from Canvas to Discourse', async function() {
      // Trigger course sync
      const result = await syncService.publishSyncEvent({
        entityType: 'course',
        entityId: '67890',
        operation: 'SYNC',
        priority: 'high'
      });
      
      expect(result).to.be.true;
      
      // If we're using mock RabbitMQ, manually trigger message processing
      if (amqpChannel.mockCallback) {
        const message = {
          content: Buffer.from(JSON.stringify({
            entityType: 'course',
            entityId: '67890',
            operation: 'SYNC',
            priority: 'high'
          })),
          fields: { routingKey: 'sync_high' }
        };
        await amqpChannel.mockCallback(message);
      } else {
        // Wait for real message processing
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
      
      // Verify Canvas API was called to get course data
      expect(mockCanvasAPI.getCourse.calledWith('67890')).to.be.true;
      
      // Verify Discourse API was checked for existing category
      expect(mockDiscourseAPI.getCategory.calledOnce).to.be.true;
      
      // Verify course was created in Discourse as a category
      expect(mockDiscourseAPI.createCategory.calledOnce).to.be.true;
      
      // Verify model mapping was created
      const mapping = await modelMapper.getMapping('course', '67890');
      expect(mapping).to.exist;
      expect(mapping.discourseId).to.equal('d67890');
    });
    
    it('should update a course when changed in Canvas', async function() {
      // Mock Canvas API to return updated course
      mockCanvasAPI.getCourse.resolves({
        ...testCourse,
        name: 'Updated Test Course'
      });
      
      // Mock Discourse API to return existing category
      mockDiscourseAPI.getCategory.resolves({
        discourse_id: 'd67890',
        name: 'Test Course',
        slug: 'tst101'
      });
      
      // Mock update function
      mockDiscourseAPI.updateCategory = sinon.stub().resolves({
        discourse_id: 'd67890',
        name: 'Updated Test Course',
        slug: 'tst101'
      });
      
      // Trigger course sync
      const result = await syncService.publishSyncEvent({
        entityType: 'course',
        entityId: '67890',
        operation: 'SYNC',
        priority: 'high'
      });
      
      expect(result).to.be.true;
      
      // If we're using mock RabbitMQ, manually trigger message processing
      if (amqpChannel.mockCallback) {
        const message = {
          content: Buffer.from(JSON.stringify({
            entityType: 'course',
            entityId: '67890',
            operation: 'SYNC',
            priority: 'high'
          })),
          fields: { routingKey: 'sync_high' }
        };
        await amqpChannel.mockCallback(message);
      } else {
        // Wait for real message processing
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
      
      // Verify category was updated in Discourse
      expect(mockDiscourseAPI.updateCategory.calledOnce).to.be.true;
      expect(mockDiscourseAPI.updateCategory.firstCall.args[0]).to.have.property('name', 'Updated Test Course');
    });
  });
  
  describe('Assignment Synchronization Flow', function() {
    // Setup test data
    const testAssignment = {
      canvas_id: '54321',
      course_id: '67890',
      name: 'Test Assignment',
      description: 'This is a test assignment',
      due_date: '2025-03-15T23:59:59Z',
      points_possible: 100
    };
    
    const testDiscussionTopic = {
      discourse_id: 'd54321',
      category_id: 'd67890',
      title: 'Test Assignment',
      raw: 'This is a test assignment\n\nDue: March 15, 2025\nPoints: 100',
      created_at: new Date().toISOString()
    };
    
    before(function() {
      // Add assignment-specific mock methods
      mockCanvasAPI.getAssignment = sinon.stub().resolves(testAssignment);
      mockDiscourseAPI.createTopic = sinon.stub().resolves(testDiscussionTopic);
      mockDiscourseAPI.getTopic = sinon.stub().resolves(null); // Initially doesn't exist
    });
    
    it('should synchronize a new assignment from Canvas to Discourse', async function() {
      // Trigger assignment sync
      const result = await syncService.publishSyncEvent({
        entityType: 'assignment',
        entityId: '54321',
        courseId: '67890',
        operation: 'SYNC',
        priority: 'critical'
      });
      
      expect(result).to.be.true;
      
      // If we're using mock RabbitMQ, manually trigger message processing
      if (amqpChannel.mockCallback) {
        const message = {
          content: Buffer.from(JSON.stringify({
            entityType: 'assignment',
            entityId: '54321',
            courseId: '67890',
            operation: 'SYNC',
            priority: 'critical'
          })),
          fields: { routingKey: 'sync_critical' }
        };
        await amqpChannel.mockCallback(message);
      } else {
        // Wait for real message processing
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
      
      // Verify Canvas API was called to get assignment data
      expect(mockCanvasAPI.getAssignment.calledWith('54321')).to.be.true;
      
      // Verify course mapping was checked
      expect(modelMapper.getMapping.calledWith('course', '67890')).to.be.true;
      
      // Verify topic was created in Discourse
      expect(mockDiscourseAPI.createTopic.calledOnce).to.be.true;
      expect(mockDiscourseAPI.createTopic.firstCall.args[0]).to.have.property('title', 'Test Assignment');
      
      // Verify model mapping was created
      const mapping = await modelMapper.getMapping('assignment', '54321');
      expect(mapping).to.exist;
      expect(mapping.discourseId).to.equal('d54321');
    });
  });
  
  describe('Error Handling and Recovery', function() {
    it('should handle API failures during synchronization', async function() {
      // Setup failure scenario
      mockDiscourseAPI.createUser.rejects(new Error('API temporarily unavailable'));
      
      // Reset user fetch to simulate new user
      mockDiscourseAPI.getUser.resolves(null);
      
      // Trigger user sync
      const result = await syncService.publishSyncEvent({
        entityType: 'user',
        entityId: '99999', // Use different ID to avoid conflicts with previous tests
        operation: 'SYNC',
        priority: 'high'
      });
      
      expect(result).to.be.true;
      
      // If we're using mock RabbitMQ, manually trigger message processing
      if (amqpChannel.mockCallback) {
        const message = {
          content: Buffer.from(JSON.stringify({
            entityType: 'user',
            entityId: '99999',
            operation: 'SYNC',
            priority: 'high'
          })),
          fields: { routingKey: 'sync_high' }
        };
        await amqpChannel.mockCallback(message);
      } else {
        // Wait for real message processing
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
      
      // Verify sync state has recorded the failure
      const syncStatus = await syncService.syncState.getSyncStatus('user', '99999');
      expect(syncStatus).to.exist;
      expect(syncStatus.status).to.equal('FAILED');
      expect(syncStatus.lastError).to.contain('API temporarily unavailable');
      
      // Now test recovery
      mockDiscourseAPI.createUser.resolves({
        discourse_id: 'd99999',
        username: 'recovereduser',
        name: 'Recovered User',
        email: 'recovered@example.com'
      });
      
      // Trigger recovery
      const recoveryResult = await syncService.processPendingRetries();
      expect(recoveryResult.processed).to.be.greaterThan(0);
      
      // Verify recovery worked
      const updatedSyncStatus = await syncService.syncState.getSyncStatus('user', '99999');
      expect(updatedSyncStatus.status).to.equal('SYNCED');
      
      // Verify mapping was created after recovery
      const mapping = await modelMapper.getMapping('user', '99999');
      expect(mapping).to.exist;
      expect(mapping.discourseId).to.equal('d99999');
    });
  });
  
  describe('Bidirectional Synchronization', function() {
    // Setting up test data for Discourse-to-Canvas sync
    const discoursePost = {
      discourse_id: 'd76543',
      topic_id: 'd54321', // Associated with assignment
      user_id: 'd12345',  // Associated with user
      raw: 'Here is my submission for the assignment.',
      created_at: new Date().toISOString()
    };
    
    before(function() {
      // Add bi-directional sync mock methods
      mockDiscourseAPI.getPost = sinon.stub().resolves(discoursePost);
      mockCanvasAPI.createSubmission = sinon.stub().resolves({
        canvas_id: '76543',
        assignment_id: '54321',
        user_id: '12345',
        submitted_at: new Date().toISOString(),
        body: 'Here is my submission for the assignment.'
      });
    });
    
    it('should synchronize Discourse posts to Canvas submissions', async function() {
      // Setup mappings to allow bidirectional mapping
      await modelMapper.createMapping('user', '12345', 'd12345');
      await modelMapper.createMapping('assignment', '54321', 'd54321');
      
      // Trigger sync from Discourse to Canvas
      const result = await syncService.publishSyncEvent({
        entityType: 'post',
        entityId: 'd76543',
        operation: 'SYNC_FROM_DISCOURSE',
        priority: 'critical'
      });
      
      expect(result).to.be.true;
      
      // If we're using mock RabbitMQ, manually trigger message processing
      if (amqpChannel.mockCallback) {
        const message = {
          content: Buffer.from(JSON.stringify({
            entityType: 'post',
            entityId: 'd76543',
            operation: 'SYNC_FROM_DISCOURSE',
            priority: 'critical'
          })),
          fields: { routingKey: 'sync_critical' }
        };
        await amqpChannel.mockCallback(message);
      } else {
        // Wait for real message processing
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
      
      // Verify Discourse API was called to get post data
      expect(mockDiscourseAPI.getPost.calledWith('d76543')).to.be.true;
      
      // Verify mappings were used to find Canvas equivalents
      expect(modelMapper.getCanvasId.calledWith('assignment', 'd54321')).to.be.true;
      expect(modelMapper.getCanvasId.calledWith('user', 'd12345')).to.be.true;
      
      // Verify submission was created in Canvas
      expect(mockCanvasAPI.createSubmission.calledOnce).to.be.true;
      
      // Verify mapping was created for post->submission
      const mapping = await modelMapper.getMapping('post', 'd76543');
      expect(mapping).to.exist;
      expect(mapping.canvasId).to.equal('76543');
    });
  });
});