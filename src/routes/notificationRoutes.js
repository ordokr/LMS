import express from 'express';
import notificationService from '../services/notificationService';
import { requireAuth } from '../middleware/authMiddleware';

const router = express.Router();

/**
 * Get notifications for the authenticated user
 * @route GET /api/v1/notifications
 */
router.get('/', requireAuth, async (req, res) => {
  try {
    const options = {
      read: req.query.read === 'true',
      type: req.query.type,
      since: req.query.since,
      limit: parseInt(req.query.limit) || 20
    };
    
    const notifications = await notificationService.getUserNotifications(req.user.id, options);
    res.json(notifications);
  } catch (error) {
    console.error('Error fetching notifications:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * Mark a notification as read
 * @route POST /api/v1/notifications/:id/read
 */
router.post('/:id/read', requireAuth, async (req, res) => {
  try {
    const { id } = req.params;
    const { source } = req.body;
    
    // Critical: Validate source parameter
    if (!source) {
      // This is the key line that must be present and working
      return res.status(400).json({ error: 'Source parameter is required' });
    }
    
    const notification = await notificationService.markAsRead(id, source);
    res.json(notification);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

export default router;