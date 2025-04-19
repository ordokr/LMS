export enum SyncDirection {
  CanvasToDiscourse = 'canvasToDiscourse',
  DiscourseToCanvas = 'discourseToCanvas',
  Bidirectional = 'bidirectional'
}

export enum SyncStatus {
  Synced = 'Synced',
  PendingToCanvas = 'PendingToCanvas',
  PendingToDiscourse = 'PendingToDiscourse',
  Conflict = 'Conflict',
  Error = 'Error',
  LocalOnly = 'LocalOnly'
}

export interface SyncState {
  is_syncing: boolean;
  last_sync: string | null;
  current_sync_started: string | null;
  current_sync_progress: number; // 0.0 to 1.0
  current_sync_stage: string;
  current_sync_entity_type: string | null;
  current_sync_entity_id: string | null;
  current_sync_direction: SyncDirection | null;
  current_sync_results: SyncResult[];
  error_count: number;
  success_count: number;
}

export interface SyncResult {
  id: string;
  entity_type: string;
  entity_id: string;
  canvas_updates: number;
  discourse_updates: number;
  errors: string[];
  status: SyncStatus;
  started_at: string;
  completed_at: string;
}
