/**
 * API routes for monitoring the synchronization process
 */

const express = require('express');
const router = express.Router();
const { SyncMonitor } = require('../services/monitoring/sync_monitor');
const { SyncState } = require('../services/integration/sync_state');
const { SyncTransaction } = require('../services/integration/sync_transaction');
const { SyncService } = require('../services/integration/sync_service');

// Initialize services
const syncState = new SyncState();
const syncTransaction = new SyncTransaction();
const syncMonitor = new SyncMonitor({ syncState, syncTransaction });
 
// Get synchronization statistics
router.get('/sync-stats', async (req, res) => {
  try {
    const stats = await syncMonitor.getStatistics();
    res.json(stats);
  } catch (error) {
    console.error('Error getting sync stats:', error);
    res.status(500).json({ error: 'Failed to retrieve synchronization statistics' });
  }
});

// Get pending items that need attention
router.get('/pending-items', async (req, res) => {
  try {
    const limit = parseInt(req.query.limit) || 100;
    const items = await syncMonitor.getPendingItems(limit);
    res.json(items);
  } catch (error) {
    console.error('Error getting pending items:', error);
    res.status(500).json({ error: 'Failed to retrieve pending items' });
  }
});

// Get entity synchronization history
router.get('/entity-history/:entityType/:entityId', async (req, res) => {
  try {
    const { entityType, entityId } = req.params;
    const history = await syncMonitor.getEntitySyncHistory(entityType, entityId);
    res.json(history);
  } catch (error) {
    console.error('Error getting entity history:', error);
    res.status(500).json({ error: 'Failed to retrieve entity history' });
  }
});

// Trigger manual resync for an entity
router.post('/resync', async (req, res) => {
  try {
    const { entityType, entityId, priority } = req.body;
    
    if (!entityType || !entityId) {
      return res.status(400).json({ error: 'Entity type and ID are required' });
    }
    
    const result = await syncMonitor.triggerResync(entityType, entityId, priority || 'high');
    
    if (result) {
      res.json({ success: true, message: 'Resync triggered successfully' });
    } else {
      res.status(500).json({ error: 'Failed to trigger resync' });
    }
  } catch (error) {
    console.error('Error triggering resync:', error);
    res.status(500).json({ error: 'Failed to trigger resync' });
  }
});

module.exports = router;