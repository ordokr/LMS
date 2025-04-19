import { ErrorRecord, ErrorFilter } from '../types/error';

const API_BASE_URL = '/api';

/**
 * Fetch all errors
 */
export async function fetchAllErrors(): Promise<ErrorRecord[]> {
  const response = await fetch(`${API_BASE_URL}/errors`);
  
  if (!response.ok) {
    throw new Error(`Failed to fetch errors: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Fetch unresolved errors
 */
export async function fetchUnresolvedErrors(): Promise<ErrorRecord[]> {
  const response = await fetch(`${API_BASE_URL}/errors/unresolved`);
  
  if (!response.ok) {
    throw new Error(`Failed to fetch unresolved errors: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Fetch errors by filter
 */
export async function fetchErrorsByFilter(filter: ErrorFilter): Promise<ErrorRecord[]> {
  const response = await fetch(`${API_BASE_URL}/errors/filter`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(filter),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to fetch errors by filter: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Fetch error by ID
 */
export async function fetchErrorById(errorId: string): Promise<ErrorRecord | null> {
  const response = await fetch(`${API_BASE_URL}/errors/${errorId}`);
  
  if (!response.ok) {
    throw new Error(`Failed to fetch error: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Resolve an error
 */
export async function resolveError(errorId: string, resolution: string): Promise<boolean> {
  const response = await fetch(`${API_BASE_URL}/errors/${errorId}/resolve`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ resolution }),
  });
  
  if (!response.ok) {
    throw new Error(`Failed to resolve error: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Retry an error
 */
export async function retryError(errorId: string): Promise<boolean> {
  const response = await fetch(`${API_BASE_URL}/errors/${errorId}/retry`, {
    method: 'POST',
  });
  
  if (!response.ok) {
    throw new Error(`Failed to retry error: ${response.statusText}`);
  }
  
  return await response.json();
}

/**
 * Clear resolved errors
 */
export async function clearResolvedErrors(): Promise<number> {
  const response = await fetch(`${API_BASE_URL}/errors/clear-resolved`, {
    method: 'DELETE',
  });
  
  if (!response.ok) {
    throw new Error(`Failed to clear resolved errors: ${response.statusText}`);
  }
  
  return await response.json();
}
