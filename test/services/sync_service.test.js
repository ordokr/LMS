/**
 * Unit tests for the Synchronization Service
 */

const { describe, it, beforeEach, afterEach, expect, jest } = require('@jest/globals');
const amqp = require('amqplib');

// Mock dependencies
jest.mock('amqplib', () => ({
  connect: jest.fn()
}));
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
      get: jest.fn(),
      deactivate: jest.fn()
    }
  }
}));
jest.mock('../api/discourse_client', () => ({
  discourseApi: {
    users: {
      upsert: jest.fn(),
      deactivate: jest.fn(),
      get: jest.fn()
    },
    categories: {
      upsert: jest.fn(),
      delete: jest.fn()
    }
  }
}));
jest.mock('./sync_state', () => ({
  SyncState: jest.fn().mockImplementation(() => ({
    updateSyncStatus: jest.fn().mockResolvedValue({})
  }))
}));
jest.mock('./sync_transaction', () => ({
  SyncTransaction: jest.fn().mockImplementation(() => ({
    begin: jest.fn().mockResolvedValue({}),
    commit: jest.fn().mockResolvedValue({}),
    rollback: jest.fn().mockResolvedValue({})
  }))
}));

// Import the module under test
const { syncService } = require('../integration/sync_service');

describe('SyncService', () => {
  let mockChannel;
  let mockConnection;

  beforeEach(() => {
    // Reset service state
    syncService.channel = null;
    syncService.connection = null;
    syncService.isProcessing = false;

    // Setup mocks
    mockChannel = {
      assertQueue: jest.fn().mockResolvedValue({}),
      sendToQueue: jest.fn().mockResolvedValue({}),
      consume: jest.fn().mockResolvedValue({}),
      ack: jest.fn()
    };
    mockConnection = {
      createChannel: jest.fn().mockResolvedValue(mockChannel),
      close: jest.fn().mockResolvedValue({})
    };
    amqp.connect.mockResolvedValue(mockConnection);
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  describe('initialize', () => {
    it('should successfully initialize the service and create queues', async () => {
      await syncService.initialize();

      expect(amqp.connect).toHaveBeenCalledWith('amqp://localhost');
      expect(mockConnection.createChannel).toHaveBeenCalled();
      expect(mockChannel.assertQueue).toHaveBeenCalledWith('sync_critical', { durable: true });
      expect(mockChannel.assertQueue).toHaveBeenCalledWith('sync_high', { durable: true });
      expect(mockChannel.assertQueue).toHaveBeenCalledWith('sync_background', { durable: true });
      expect(mockChannel.assertQueue).toHaveBeenCalledWith('sync_failed', { durable: true });
      expect(syncService.connection).toBe(mockConnection);
      expect(syncService.channel).toBe(mockChannel);
    });

    it('should throw an error if the connection fails', async () => {
      amqp.connect.mockRejectedValueOnce(new Error('Connection error'));

      await expect(syncService.initialize()).rejects.toThrow('Connection error');
    });
  });

  describe('publishEvent', () => {
    beforeEach(async () => {
      await syncService.initialize();
    });

    it('should successfully publish an event to the queue', async () => {
      const result = await syncService.publishEvent(
        'critical',
        'user',
        'create',
        'canvas',
        { id: '123', name: 'Test User' }
      );

      expect(mockChannel.sendToQueue).toHaveBeenCalledWith(
        'sync_critical',
        expect.any(Buffer),
        { persistent: true }
      );
      expect(result).toMatch(/^tx-/); // Should return a transaction ID
    });

    it('should throw an error if service is not initialized', async () => {
      syncService.channel = null;

      await expect(syncService.publishEvent(
        'critical',
        'user',
        'create',
        'canvas',
        { id: '123' }
      )).rejects.toThrow('Sync service not initialized');
    });
  });

  describe('startProcessing', () => {
    beforeEach(async () => {
      await syncService.initialize();
    });

    it('should start consuming messages from all queues', async () => {
      await syncService.startProcessing();

      expect(mockChannel.consume).toHaveBeenCalledTimes(3);
      expect(mockChannel.consume).toHaveBeenCalledWith(
        'sync_critical',
        expect.any(Function),
        { noAck: false }
      );
      expect(mockChannel.consume).toHaveBeenCalledWith(
        'sync_high',
        expect.any(Function),
        { noAck: false }
      );
      expect(mockChannel.consume).toHaveBeenCalledWith(
        'sync_background',
        expect.any(Function),
        { noAck: false }
      );
      expect(syncService.isProcessing).toBe(true);
    });

    it('should not start processing if already running', async () => {
      syncService.isProcessing = true;
      await syncService.startProcessing();

      expect(mockChannel.consume).not.toHaveBeenCalled();
    });
  });

  describe('processEvent', () => {
    beforeEach(async () => {
      await syncService.initialize();
    });

    it('should process an event and acknowledge it', async () => {
      const mockEvent = {
        transactionId: 'tx-123',
        entityType: 'user',
        operation: 'create',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        data: { id: '123', name: 'Test User', email: 'test@example.com' }
      };
      
      const mockMessage = {
        content: Buffer.from(JSON.stringify(mockEvent))
      };

      // Mock implementations for testing user sync to Discourse
      const { discourseApi } = require('../api/discourse_client');
      discourseApi.users.upsert.mockResolvedValueOnce({ id: '456', username: 'test' });

      await syncService.processEvent(mockMessage, 'critical');

      expect(discourseApi.users.upsert).toHaveBeenCalled();
      expect(mockChannel.ack).toHaveBeenCalledWith(mockMessage);
    });

    it('should handle errors and send to failed queue', async () => {
      const mockEvent = {
        transactionId: 'tx-123',
        entityType: 'user',
        operation: 'create',
        sourceSystem: 'canvas',
        targetSystem: 'discourse',
        data: { id: '123', name: 'Test User', email: 'test@example.com' }
      };
      
      const mockMessage = {
        content: Buffer.from(JSON.stringify(mockEvent))
      };

      // Mock implementation to throw an error
      const { discourseApi } = require('../api/discourse_client');
      discourseApi.users.upsert.mockRejectedValueOnce(new Error('API error'));

      await syncService.processEvent(mockMessage, 'critical');

      expect(mockChannel.sendToQueue).toHaveBeenCalledWith(
        'sync_failed',
        expect.any(Buffer),
        { persistent: true }
      );
      expect(mockChannel.ack).toHaveBeenCalledWith(mockMessage);
    });
  });

  describe('syncToDiscourse', () => {
    it('should call the appropriate sync method based on entity type', async () => {
      // Create spies on the internal sync methods
      const syncUserSpy = jest.spyOn(syncService, 'syncUserToDiscourse').mockResolvedValueOnce({});
      const syncCourseSpy = jest.spyOn(syncService, 'syncCourseToDiscourse').mockResolvedValueOnce({});
      
      await syncService.syncToDiscourse('user', 'create', { id: '123' });
      expect(syncUserSpy).toHaveBeenCalledWith('create', { id: '123' });
      
      await syncService.syncToDiscourse('course', 'update', { id: '456' });
      expect(syncCourseSpy).toHaveBeenCalledWith('update', { id: '456' });
    });

    it('should throw an error for unsupported entity types', async () => {
      await expect(syncService.syncToDiscourse('unknown', 'create', {}))
        .rejects.toThrow('Unsupported entity type for Discourse sync: unknown');
    });
  });

  describe('stop', () => {
    it('should close connections and stop processing', async () => {
      syncService.channel = mockChannel;
      syncService.connection = mockConnection;
      syncService.isProcessing = true;

      await syncService.stop();

      expect(mockChannel.close).toHaveBeenCalled();
      expect(mockConnection.close).toHaveBeenCalled();
      expect(syncService.isProcessing).toBe(false);
    });
  });
});
