/**
 * Sync State Manager
 * 
 * Tracks the synchronization state between Canvas and Discourse systems.
 * Maintains records of which entities have been synced, when, and their current status.
 */

const { logger } = require('../../shared/logger');
const { db } = require('../../shared/db');

class SyncState {
  /**
   * Get the sync status for an entity
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceId - ID in the source system
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @returns {Object} - Sync status information
   */
  async getSyncStatus(entityType, sourceId, sourceSystem) {
    try {
      const status = await db.syncState.findOne({
        where: {
          entityType,
          sourceId,
          sourceSystem
        }
      });
      
      return status || { synced: false };
    } catch (error) {
      logger.error(`Error retrieving sync status: ${error.message}`);
      throw error;
    }
  }

  /**
   * Update sync status for an entity
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceId - ID in the source system
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @param {string} targetId - ID in the target system
   * @param {string} status - Sync status ('pending', 'completed', 'failed')
   * @param {string} error - Error message if status is 'failed'
   * @returns {Object} - Updated sync status
   */
  async updateSyncStatus(entityType, sourceId, sourceSystem, targetId = null, status = 'completed', error = null) {
    try {
      const targetSystem = sourceSystem === 'canvas' ? 'discourse' : 'canvas';
      
      // Check if a record already exists
      const existing = await db.syncState.findOne({
        where: {
          entityType,
          sourceId,
          sourceSystem
        }
      });
      
      if (existing) {
        // Update existing record
        await db.syncState.update({
          targetId: targetId || existing.targetId,
          lastSyncTime: new Date(),
          status,
          errorMessage: error
        }, {
          where: {
            entityType,
            sourceId,
            sourceSystem
          }
        });
      } else {
        // Create new record
        await db.syncState.create({
          entityType,
          sourceId,
          sourceSystem,
          targetSystem,
          targetId,
          lastSyncTime: new Date(),
          status,
          errorMessage: error
        });
      }
      
      logger.info(`Updated sync status for ${entityType} ${sourceId}: ${status}`);
      
      return this.getSyncStatus(entityType, sourceId, sourceSystem);
    } catch (error) {
      logger.error(`Error updating sync status: ${error.message}`);
      throw error;
    }
  }

  /**
   * Mark entity as requiring resync
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceId - ID in the source system
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @returns {boolean} - Success indicator
   */
  async markForResync(entityType, sourceId, sourceSystem) {
    try {
      // Update or create the sync state record
      await this.updateSyncStatus(entityType, sourceId, sourceSystem, null, 'pending', null);
      
      logger.info(`Marked ${entityType} ${sourceId} for resynchronization`);
      return true;
    } catch (error) {
      logger.error(`Error marking for resync: ${error.message}`);
      throw error;
    }
  }

  /**
   * Get entities that need synchronization
   * 
   * @param {string} entityType - Type of entity (user, course, etc.)
   * @param {string} sourceSystem - Source system ('canvas' or 'discourse')
   * @param {number} limit - Maximum number of records to return
   * @returns {Array} - List of entities needing sync
   */
  async getPendingSyncs(entityType, sourceSystem, limit = 100) {
    try {
      const pending = await db.syncState.findAll({
        where: {
          entityType,
          sourceSystem,
          status: 'pending'
        },
        order: [['lastSyncTime', 'ASC']],
        limit
      });
      
      return pending;
    } catch (error) {
      logger.error(`Error retrieving pending syncs: ${error.message}`);
      throw error;
    }
  }

  /**
   * Get sync statistics
   * 
   * @returns {Object} - Sync statistics by entity type and status
   */
  async getStats() {
    try {
      // Get counts by entity type and status
      const stats = await db.sequelize.query(`
        SELECT 
          "entityType", 
          "sourceSystem", 
          "status", 
          COUNT(*) as count 
        FROM 
          sync_states 
        GROUP BY 
          "entityType", "sourceSystem", "status"
      `, { type: db.sequelize.QueryTypes.SELECT });
      
      // Reshape the data for easier consumption
      const result = {
        byEntityType: {},
        byStatus: {
          pending: 0,
          completed: 0,
          failed: 0
        },
        total: 0
      };
      
      stats.forEach(stat => {
        // Initialize entity type if not exists
        if (!result.byEntityType[stat.entityType]) {
          result.byEntityType[stat.entityType] = {
            total: 0,
            pending: 0,
            completed: 0,
            failed: 0
          };
        }
        
        // Update counts
        result.byEntityType[stat.entityType][stat.status] = stat.count;
        result.byEntityType[stat.entityType].total += stat.count;
        result.byStatus[stat.status] += stat.count;
        result.total += stat.count;
      });
      
      return result;
    } catch (error) {
      logger.error(`Error retrieving sync stats: ${error.message}`);
      throw error;
    }
  }

  /**
   * Reset sync state for testing or recovery purposes
   * 
   * @param {string} entityType - Optional entity type to reset
   * @returns {number} - Number of records reset
   */
  async reset(entityType = null) {
    try {
      const where = entityType ? { entityType } : {};
      
      const count = await db.syncState.destroy({
        where
      });
      
      logger.warn(`Reset sync state for ${entityType || 'all entities'}: ${count} records deleted`);
      return count;
    } catch (error) {
      logger.error(`Error resetting sync state: ${error.message}`);
      throw error;
    }
  }
}

module.exports = {
  SyncState
};
