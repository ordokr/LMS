import axios from 'axios';
import { config } from '../config';

/**
 * Authenticate a user through Canvas OAuth
 * @param {string} token - Canvas OAuth token
 * @returns {Object|null} User object if authentication successful
 */
export async function authenticateCanvasUser(token) {
  try {
    const response = await axios.get(`${config.CANVAS_API_URL}/users/self`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    
    if (response.status === 200) {
      return mapCanvasUserToInternal(response.data);
    }
    return null;
  } catch (error) {
    console.error('Canvas authentication error:', error);
    return null;
  }
}

/**
 * Map Canvas user data to internal user model
 */
function mapCanvasUserToInternal(canvasUser) {
  return {
    id: canvasUser.id,
    email: canvasUser.email,
    name: canvasUser.name,
    roles: determineUserRoles(canvasUser),
    canvas_id: canvasUser.id,
    source: 'canvas'
  };
}