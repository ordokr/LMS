export enum ErrorSeverity {
  Critical = 'Critical',
  Error = 'Error',
  Warning = 'Warning',
  Info = 'Info'
}

export enum ErrorCategory {
  ApiConnection = 'ApiConnection',
  Authentication = 'Authentication',
  Authorization = 'Authorization',
  Validation = 'Validation',
  Synchronization = 'Synchronization',
  Database = 'Database',
  Configuration = 'Configuration',
  System = 'System',
  Unknown = 'Unknown'
}

export interface ErrorRecord {
  id: string;
  message: string;
  severity: string;
  category: string;
  source: string;
  entity_type?: string;
  entity_id?: string;
  details?: string;
  timestamp: string;
  resolved: boolean;
  resolved_at?: string;
  resolution?: string;
  retry_count: number;
  max_retries: number;
  retriable: boolean;
  next_retry?: string;
}

export interface ErrorFilter {
  severity?: string;
  category?: string;
  source?: string;
  entity_type?: string;
  entity_id?: string;
  resolved?: boolean;
  retriable?: boolean;
}
