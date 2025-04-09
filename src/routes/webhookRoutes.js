import express from 'express';
import webhookService from '../services/webhookService';
import { requireAuth } from '../middleware/authMiddleware';

const router = express.Router();

/**
 * Route for Canvas webhooks
 * @route POST /api/v1/webhooks/canvas
 */
router.post('/canvas', async (req, res) => {
  try {
    const result = await webhookService.handleCanvasWebhook(req.body);
    res.status(200).json({ status: 'success', result });
  } catch (error) {
    console.error('Error in Canvas webhook:', error);
    res.status(500).json({ status: 'error', message: error.message });
  }
});

/**
 * Route for Discourse webhooks
 * @route POST /api/v1/webhooks/discourse
 */
router.post('/discourse', async (req, res) => {
  try {
    const result = await webhookService.handleDiscourseWebhook(req.body);
    res.status(200).json({ status: 'success', result });
  } catch (error) {
    console.error('Error in Discourse webhook:', error);
    res.status(500).json({ status: 'error', message: error.message });
  }
});

export default router;