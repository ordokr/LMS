import { ApiConfig, CanvasConfig, DiscourseConfig } from '../types/config';

const API_BASE_URL = '/api';

/**
 * Fetch the current API configuration
 */
export async function fetchApiConfig(): Promise<ApiConfig> {
  const response = await fetch(`${API_BASE_URL}/config`);
  
  if (!response.ok) {
    throw new Error(`Failed to fetch API config: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Update the API configuration
 */
export async function updateApiConfig(config: ApiConfig): Promise<ApiConfig> {
  const response = await fetch(`${API_BASE_URL}/config`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(config),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to update API config: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Test the Canvas API connection
 */
export async function testCanvasConnection(config: CanvasConfig): Promise<{ success: boolean; message: string }> {
  const response = await fetch(`${API_BASE_URL}/config/test-canvas`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(config),
  });
  
  return await response.json();
}

/**
 * Test the Discourse API connection
 */
export async function testDiscourseConnection(config: DiscourseConfig): Promise<{ success: boolean; message: string }> {
  const response = await fetch(`${API_BASE_URL}/config/test-discourse`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(config),
  });
  
  return await response.json();
}
