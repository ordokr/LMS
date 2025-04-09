/**
 * Synchronization Monitoring Service
 * 
 * This module provides monitoring capabilities for the Canvas-Discourse synchronization,
 * allowing administrators to view sync status, error rates, and sync history.
 */

class SyncMonitor {
  /**
   * Create a new synchronization monitoring service
   * 
   * @param {Object} options - Configuration options
   * @param {SyncState} options.syncState - Reference to the sync state manager
   * @param {SyncTransaction} options.syncTransaction - Reference to transaction manager
   */
  constructor({ syncState, syncTransaction }) {
    this.syncState = syncState;
    this.syncTransaction = syncTransaction;
    this.cachedStats = null;
    this.cacheTime = null;
    this.CACHE_TTL = 60000; // 1 minute cache
  }

  /**
   * Get overall synchronization statistics
   * 
   * @returns {Promise<Object>} Statistics about synchronization
   */
  async getStatistics() {
    // Use cached stats if available and recent
    if (this.cachedStats && (Date.now() - this.cacheTime < this.CACHE_TTL)) {
      return this.cachedStats;
    }

    const stats = {
      entities: {
        users: await this._getEntityStats('user'),
        courses: await this._getEntityStats('course'),
        assignments: await this._getEntityStats('assignment'),
        discussions: await this._getEntityStats('discussion'),
        posts: await this._getEntityStats('post')
      },
      transactions: await this._getTransactionStats(),
      overall: {
        lastSync: await this._getLastSyncTime(),
        errorRate: await this._calculateErrorRate()
      }
    };

    // Cache the results
    this.cachedStats = stats;
    this.cacheTime = Date.now();

    return stats;
  }

  /**
   * Get pending synchronization items that need attention
   * 
   * @param {number} limit - Maximum number of items to return
   * @returns {Promise<Array>} List of sync items needing attention
   */
  async getPendingItems(limit = 100) {
    return await this.syncState.getFailedSyncs(limit);
  }

  /**
   * Get synchronization history for a specific entity
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} entityId - ID of the entity
   * @returns {Promise<Array>} Synchronization history for the entity
   */
  async getEntitySyncHistory(entityType, entityId) {
    const transactions = await this.syncTransaction.getTransactionsForEntity(entityType, entityId);
    const syncState = await this.syncState.getSyncStatus(entityType, entityId);
    
    return {
      currentStatus: syncState,
      syncHistory: transactions.map(t => ({
        id: t.id,
        timestamp: t.timestamp,
        operation: t.operation,
        status: t.status,
        duration: t.endTime ? (t.endTime - t.startTime) : null,
        error: t.error
      }))
    };
  }

  /**
   * Trigger a manual resync for a failing entity
   * 
   * @param {string} entityType - Type of entity to resync
   * @param {string} entityId - ID of the entity to resync
   * @param {string} priority - Priority for the resync operation
   * @returns {Promise<boolean>} True if resync was successfully triggered
   */
  async triggerResync(entityType, entityId, priority = 'high') {
    // Reset the sync state
    await this.syncState.resetSyncState(entityType, entityId);
    
    // Create a new sync transaction
    const transaction = await this.syncTransaction.beginTransaction({
      entityType,
      entityId,
      operation: 'MANUAL_RESYNC',
      initiatedBy: 'admin-dashboard'
    });
    
    // Publish the event to the sync service
    // Note: In a real implementation, this would call the actual sync service
    console.log(`Publishing manual resync event for ${entityType}:${entityId}`);
    
    await this.syncTransaction.recordStep(transaction.id, 'QUEUED', 'Manual resync requested');
    
    return true;
  }

  // Private helper methods
  
  /**
   * Get statistics for a specific entity type
   * 
   * @private
   * @param {string} entityType - Type of entity
   * @returns {Promise<Object>} Statistics for the entity type
   */
  async _getEntityStats(entityType) {
    const counts = await this.syncState.getSyncCountsByStatus(entityType);
    const totalCount = Object.values(counts).reduce((sum, count) => sum + count, 0);
    
    return {
      total: totalCount,
      synced: counts.SYNCED || 0,
      pending: counts.PENDING || 0,
      failed: counts.FAILED || 0,
      inProgress: counts.IN_PROGRESS || 0,
      syncPercentage: totalCount > 0 ? Math.round((counts.SYNCED || 0) / totalCount * 100) : 0
    };
  }

  /**
   * Get statistics about transactions
   * 
   * @private
   * @returns {Promise<Object>} Transaction statistics
   */
  async _getTransactionStats() {
    const last24Hours = new Date(Date.now() - 24 * 60 * 60 * 1000);
    const transactions = await this.syncTransaction.getTransactions({ after: last24Hours });
    
    const stats = {
      last24Hours: transactions.length,
      successful: transactions.filter(t => t.status === 'COMPLETED').length,
      failed: transactions.filter(t => t.status === 'FAILED').length,
      inProgress: transactions.filter(t => t.status === 'IN_PROGRESS').length
    };
    
    stats.successRate = stats.last24Hours > 0 ? 
      Math.round((stats.successful / stats.last24Hours) * 100) : 0;
    
    return stats;
  }

  /**
   * Get the timestamp of the last successful sync
   * 
   * @private
   * @returns {Promise<string>} ISO timestamp of last sync
   */
  async _getLastSyncTime() {
    const lastTransaction = await this.syncTransaction.getLatestSuccessfulTransaction();
    return lastTransaction ? lastTransaction.endTime : null;
  }

  /**
   * Calculate the current error rate
   * 
   * @private
   * @returns {Promise<number>} Error rate as a percentage
   */
  async _calculateErrorRate() {
    const last100 = await this.syncTransaction.getTransactions({ limit: 100 });
    if (last100.length === 0) return 0;
    
    const failedCount = last100.filter(t => t.status === 'FAILED').length;
    return Math.round((failedCount / last100.length) * 100);
  }
}

module.exports = { SyncMonitor };