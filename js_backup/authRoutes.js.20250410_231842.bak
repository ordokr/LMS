import express from 'express';
import { login, handleDiscourseSSO } from '../controllers/authController';
import { requireAuth } from '../middleware/authMiddleware';

const router = express.Router();

// Authentication routes
router.post('/login', login);
router.get('/discourse-sso', requireAuth, handleDiscourseSSO);

export default router;