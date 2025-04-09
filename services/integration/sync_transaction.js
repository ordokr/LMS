/**
 * Sync Transaction Manager
 * 
 * Manages the lifecycle of synchronization transactions, including
 * tracking, commit/rollback, and persistence.
 */

const { logger } = require('../../shared/logger');
const { db } = require('../../shared/db');

class SyncTransaction {
  /**
   * Create a new sync transaction
   * 
   * @param {Object} event - The synchronization event
   */
  constructor(event) {
    this.event = event;
    this.transactionId = event.transactionId;
    this.entityType = event.entityType;
    this.operation = event.operation;
    this.sourceSystem = event.sourceSystem;
    this.targetSystem = event.targetSystem;
    this.startTime = new Date();
    this.status = 'pending';
    this.steps = [];
  }

  /**
   * Begin the transaction
   */
  async begin() {
    logger.info(`Beginning sync transaction: ${this.transactionId}`);
    
    // Record transaction start in database
    await db.syncTransactions.create({
      transactionId: this.transactionId,
      entityType: this.entityType,
      operation: this.operation,
      sourceSystem: this.sourceSystem,
      targetSystem: this.targetSystem,
      startTime: this.startTime,
      status: this.status,
      eventData: JSON.stringify(this.event)
    });
    
    return this;
  }

  /**
   * Record a step in the transaction
   * 
   * @param {string} description - Description of the step
   * @param {Object} data - Additional data for the step
   */
  async recordStep(description, data = {}) {
    const step = {
      timestamp: new Date(),
      description,
      data
    };
    
    this.steps.push(step);
    
    // Update transaction in database
    await db.syncTransactionSteps.create({
      transactionId: this.transactionId,
      timestamp: step.timestamp,
      description: step.description,
      stepData: JSON.stringify(step.data)
    });
    
    return this;
  }

  /**
   * Commit the transaction
   */
  async commit() {
    logger.info(`Committing sync transaction: ${this.transactionId}`);
    this.status = 'completed';
    
    // Update transaction in database
    await db.syncTransactions.update(
      { 
        status: this.status,
        endTime: new Date(),
        duration: new Date() - this.startTime
      },
      { where: { transactionId: this.transactionId } }
    );
    
    return true;
  }

  /**
   * Roll back the transaction
   * 
   * @param {Error} error - Error that caused the rollback
   */
  async rollback(error) {
    logger.error(`Rolling back sync transaction: ${this.transactionId}. Error: ${error.message}`);
    this.status = 'failed';
    
    // Update transaction in database
    await db.syncTransactions.update(
      { 
        status: this.status,
        endTime: new Date(),
        duration: new Date() - this.startTime,
        errorMessage: error.message
      },
      { where: { transactionId: this.transactionId } }
    );
    
    // Implement any necessary cleanup or compensating actions
    // This would be specific to each entity type and operation
    
    return true;
  }

  /**
   * Get transaction by ID
   * 
   * @param {string} transactionId - The transaction ID to retrieve
   * @returns {Object} - The transaction data
   */
  static async getById(transactionId) {
    const transaction = await db.syncTransactions.findOne({
      where: { transactionId },
      include: [{
        model: db.syncTransactionSteps,
        as: 'steps'
      }]
    });
    
    return transaction;
  }

  /**
   * List recent transactions
   * 
   * @param {Object} filters - Optional filters for the query
   * @param {number} limit - Max number of transactions to return
   * @returns {Array} - List of transactions
   */
  static async listRecent(filters = {}, limit = 100) {
    const where = {};
    
    if (filters.status) {
      where.status = filters.status;
    }
    
    if (filters.entityType) {
      where.entityType = filters.entityType;
    }
    
    if (filters.sourceSystem) {
      where.sourceSystem = filters.sourceSystem;
    }
    
    const transactions = await db.syncTransactions.findAll({
      where,
      order: [['startTime', 'DESC']],
      limit,
      include: [{
        model: db.syncTransactionSteps,
        as: 'steps'
      }]
    });
    
    return transactions;
  }
}

module.exports = {
  SyncTransaction
};
