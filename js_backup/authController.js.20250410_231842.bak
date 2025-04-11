import { generateJwtToken } from '../auth/jwtService';
import { authenticateCanvasUser } from '../services/canvasAuthService';
import { generateDiscourseSSOPayload } from '../services/discourseSSOService';

/**
 * Handles user authentication and issues JWT tokens
 * @route POST /api/v1/auth/login
 */
export async function login(req, res) {
  try {
    // Authenticate user against Canvas or Discourse based on source
    const user = await authenticateUser(req.body.username, req.body.password, req.body.source);
    
    if (!user) {
      return res.status(401).json({ error: 'Authentication failed' });
    }
    
    // Generate JWT token
    const token = generateJwtToken(user);
    
    // Return token to client
    return res.json({ token });
  } catch (error) {
    console.error('Authentication error:', error);
    return res.status(500).json({ error: 'Authentication failed' });
  }
}

/**
 * Handles SSO authentication for Discourse
 * Creates a bridge between Canvas OAuth tokens and Discourse SSO
 * @route GET /api/v1/auth/discourse-sso
 */
export async function handleDiscourseSSO(req, res) {
  try {
    // Verify the user is authenticated in Canvas
    const canvasUser = await authenticateCanvasUser(req.headers.authorization);
    
    if (!canvasUser) {
      return res.status(401).json({ error: 'Canvas authentication required' });
    }
    
    // Generate Discourse SSO payload
    const ssoPayload = generateDiscourseSSOPayload(canvasUser, req.query.sso, req.query.sig);
    
    // Redirect to Discourse with SSO payload
    return res.redirect(`${process.env.DISCOURSE_URL}/session/sso_login?${ssoPayload}`);
  } catch (error) {
    console.error('SSO error:', error);
    return res.status(500).json({ error: 'SSO authentication failed' });
  }
}