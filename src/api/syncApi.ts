import { SyncState, SyncDirection, SyncResult } from '../types/sync';

const API_BASE_URL = '/api';

/**
 * Fetch the current synchronization state
 */
export async function fetchSyncState(): Promise<SyncState> {
  const response = await fetch(`${API_BASE_URL}/sync/state`);

  if (!response.ok) {
    throw new Error(`Failed to fetch sync state: ${response.statusText}`);
  }

  return await response.json();
}

/**
 * Start a full synchronization
 */
export async function startSync(direction: SyncDirection, strategy?: string): Promise<SyncState> {
  const response = await fetch(`${API_BASE_URL}/sync/start`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ direction, strategy }),
  });

  if (!response.ok) {
    throw new Error(`Failed to start sync: ${response.statusText}`);
  }

  return await response.json();
}

/**
 * Cancel the current synchronization
 */
export async function cancelSync(): Promise<SyncState> {
  const response = await fetch(`${API_BASE_URL}/sync/cancel`, {
    method: 'POST',
  });

  if (!response.ok) {
    throw new Error(`Failed to cancel sync: ${response.statusText}`);
  }

  return await response.json();
}

/**
 * Get available synchronization strategies
 */
export async function getAvailableStrategies(): Promise<string[]> {
  const response = await fetch(`${API_BASE_URL}/sync/strategies`);

  if (!response.ok) {
    throw new Error(`Failed to get available strategies: ${response.statusText}`);
  }

  return await response.json();
}

/**
 * Synchronize a specific entity
 */
export async function syncEntity(
  entityType: string,
  entityId: string,
  direction: SyncDirection,
  strategy?: string
): Promise<SyncResult> {
  const response = await fetch(`${API_BASE_URL}/sync/entity`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      entity_type: entityType,
      entity_id: entityId,
      direction,
      strategy,
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to sync entity: ${response.statusText}`);
  }

  return await response.json();
}
