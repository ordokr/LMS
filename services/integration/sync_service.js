/**
 * Canvas-Discourse Synchronization Service
 * 
 * This service implements the event-driven synchronization architecture
 * for maintaining data consistency between Canvas LMS and Discourse forums.
 */

const amqp = require('amqplib');
const { logger } = require('../../shared/logger');
const { canvasApi } = require('../api/canvas_client');
const { discourseApi } = require('../api/discourse_client');
const { SyncState } = require('./sync_state');
const { SyncTransaction } = require('./sync_transaction');
const { priorityQueue } = require('./priority_queue');

class SyncService {
  constructor() {
    this.connection = null;
    this.channel = null;
    this.syncState = new SyncState();
    this.isProcessing = false;
  }

  /**
   * Initialize the synchronization service
   */
  async initialize() {
    try {
      logger.info('Initializing synchronization service');
      this.connection = await amqp.connect(process.env.RABBITMQ_URL || 'amqp://localhost');
      this.channel = await this.connection.createChannel();
      
      // Setup queues with different priorities
      await this.channel.assertQueue('sync_critical', { durable: true });
      await this.channel.assertQueue('sync_high', { durable: true });
      await this.channel.assertQueue('sync_background', { durable: true });
      
      // Dead letter queue for failed synchronizations
      await this.channel.assertQueue('sync_failed', { durable: true });
      
      logger.info('Synchronization service initialized successfully');
      return true;
    } catch (error) {
      logger.error(`Failed to initialize synchronization service: ${error.message}`);
      throw error;
    }
  }

  /**
   * Publish a synchronization event
   * 
   * @param {string} priority - 'critical', 'high', or 'background'
   * @param {string} entityType - Type of entity being synchronized
   * @param {string} operation - 'create', 'update', 'delete'
   * @param {string} sourceSystem - 'canvas' or 'discourse'
   * @param {object} data - Entity data to synchronize
   */
  async publishEvent(priority, entityType, operation, sourceSystem, data) {
    try {
      if (!this.channel) {
        throw new Error('Sync service not initialized');
      }
      
      const queueName = `sync_${priority}`;
      const event = {
        entityType,
        operation,
        sourceSystem,
        targetSystem: sourceSystem === 'canvas' ? 'discourse' : 'canvas',
        data,
        timestamp: new Date().toISOString(),
        transactionId: `tx-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`
      };
      
      await this.channel.sendToQueue(
        queueName,
        Buffer.from(JSON.stringify(event)),
        { persistent: true }
      );
      
      logger.info(`Published ${priority} sync event: ${event.transactionId}`);
      return event.transactionId;
    } catch (error) {
      logger.error(`Failed to publish sync event: ${error.message}`);
      throw error;
    }
  }

  /**
   * Start processing synchronization events
   */
  async startProcessing() {
    if (this.isProcessing) {
      logger.warn('Sync processing is already running');
      return;
    }

    this.isProcessing = true;
    logger.info('Starting sync event processing');

    try {
      // Process critical events (highest priority)
      await this.channel.consume('sync_critical', async (msg) => {
        if (msg) {
          await this.processEvent(msg, 'critical');
        }
      }, { noAck: false });

      // Process high priority events
      await this.channel.consume('sync_high', async (msg) => {
        if (msg) {
          await this.processEvent(msg, 'high');
        }
      }, { noAck: false });

      // Process background events (lowest priority)
      await this.channel.consume('sync_background', async (msg) => {
        if (msg) {
          await this.processEvent(msg, 'background');
        }
      }, { noAck: false });

    } catch (error) {
      this.isProcessing = false;
      logger.error(`Error in sync processing: ${error.message}`);
      throw error;
    }
  }

  /**
   * Process a synchronization event
   * 
   * @param {Object} msg - Message from the queue
   * @param {string} priority - Priority level
   */
  async processEvent(msg, priority) {
    let event;
    try {
      event = JSON.parse(msg.content.toString());
      logger.info(`Processing ${priority} sync event: ${event.transactionId}`);
      
      const transaction = new SyncTransaction(event);
      await transaction.begin();
      
      // Perform the actual synchronization
      if (event.targetSystem === 'discourse') {
        await this.syncToDiscourse(event.entityType, event.operation, event.data);
      } else {
        await this.syncToCanvas(event.entityType, event.operation, event.data);
      }
      
      await transaction.commit();
      await this.syncState.updateSyncStatus(event.entityType, event.transactionId, 'completed');
      
      // Acknowledge the message
      this.channel.ack(msg);
      logger.info(`Completed sync event: ${event.transactionId}`);
    } catch (error) {
      logger.error(`Failed to process sync event: ${error.message}`);
      
      if (event) {
        await this.syncState.updateSyncStatus(event.entityType, event.transactionId, 'failed', error.message);
        
        // Send to dead letter queue
        this.channel.sendToQueue(
          'sync_failed',
          Buffer.from(JSON.stringify({
            event,
            error: error.message,
            timestamp: new Date().toISOString()
          })),
          { persistent: true }
        );
      }
      
      // Acknowledge the message to remove from the original queue
      this.channel.ack(msg);
    }
  }

  /**
   * Synchronize data to Discourse
   */
  async syncToDiscourse(entityType, operation, data) {
    switch (entityType) {
      case 'user':
        return this.syncUserToDiscourse(operation, data);
      case 'course':
        return this.syncCourseToDiscourse(operation, data);
      case 'assignment':
        return this.syncAssignmentToDiscourse(operation, data);
      case 'submission':
        return this.syncSubmissionToDiscourse(operation, data);
      default:
        throw new Error(`Unsupported entity type for Discourse sync: ${entityType}`);
    }
  }

  /**
   * Synchronize data to Canvas
   */
  async syncToCanvas(entityType, operation, data) {
    switch (entityType) {
      case 'user':
        return this.syncUserToCanvas(operation, data);
      case 'topic':
        return this.syncTopicToCanvas(operation, data);
      case 'post':
        return this.syncPostToCanvas(operation, data);
      default:
        throw new Error(`Unsupported entity type for Canvas sync: ${entityType}`);
    }
  }

  /**
   * Sync a Canvas user to Discourse
   */
  async syncUserToDiscourse(operation, data) {
    logger.info(`Syncing user to Discourse: ${operation}`);
    
    switch (operation) {
      case 'create':
      case 'update':
        // Transform Canvas user to Discourse user format
        const discourseUser = {
          name: data.name,
          email: data.email,
          username: data.email.split('@')[0],
          external_id: `canvas-${data.id}`,
          // Add other relevant user fields
        };
        
        return await discourseApi.users.upsert(discourseUser);
        
      case 'delete':
        return await discourseApi.users.deactivate(`canvas-${data.id}`);
        
      default:
        throw new Error(`Unsupported operation for user sync: ${operation}`);
    }
  }

  /**
   * Sync a Canvas course to Discourse
   */
  async syncCourseToDiscourse(operation, data) {
    logger.info(`Syncing course to Discourse: ${operation}`);
    
    switch (operation) {
      case 'create':
      case 'update':
        // Transform Canvas course to Discourse category
        const courseCategory = {
          name: data.name,
          description: data.description,
          color: "0088CC",
          text_color: "FFFFFF",
          custom_fields: {
            canvas_course_id: data.id,
            canvas_course_code: data.course_code
          }
        };
        
        return await discourseApi.categories.upsert(courseCategory);
        
      case 'delete':
        return await discourseApi.categories.delete({ custom_fields: { canvas_course_id: data.id } });
        
      default:
        throw new Error(`Unsupported operation for course sync: ${operation}`);
    }
  }

  // Other sync methods would be implemented similarly...

  /**
   * Stop the synchronization service
   */
  async stop() {
    if (this.channel) {
      await this.channel.close();
    }
    if (this.connection) {
      await this.connection.close();
    }
    this.isProcessing = false;
    logger.info('Synchronization service stopped');
  }
}

module.exports = {
  syncService: new SyncService()
};
