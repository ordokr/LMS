import { v4 as uuidv4 } from 'uuid';

interface WorkerMessage {
  id: string;
  action: string;
  payload: any;
}

interface WorkerResponse {
  id: string;
  success: boolean;
  result?: any;
  error?: string;
}

// Client for the database worker
export class DatabaseClient {
  private worker: Worker;
  private pendingRequests: Map<string, { 
    resolve: (value: any) => void, 
    reject: (reason: any) => void 
  }> = new Map();
  private isReady: boolean = false;
  private readyPromise: Promise<void>;
  private readyResolve!: () => void;
  
  constructor() {
    this.worker = new Worker(new URL('./database_worker.ts', import.meta.url));
    
    this.readyPromise = new Promise<void>((resolve) => {
      this.readyResolve = resolve;
    });
    
    this.worker.addEventListener('message', this.handleWorkerMessage.bind(this));
  }
  
  private handleWorkerMessage(event: MessageEvent<WorkerResponse>): void {
    const response = event.data;
    
    if (response.id === 'init') {
      this.isReady = true;
      this.readyResolve();
      return;
    }
    
    const request = this.pendingRequests.get(response.id);
    if (request) {
      this.pendingRequests.delete(response.id);
      
      if (response.success) {
        request.resolve(response.result);
      } else {
        request.reject(new Error(response.error));
      }
    }
  }
  
  private sendMessage<T>(action: string, payload: any): Promise<T> {
    return new Promise<T>(async (resolve, reject) => {
      // Wait until worker is ready
      if (!this.isReady) {
        await this.readyPromise;
      }
      
      const id = uuidv4();
      
      this.pendingRequests.set(id, { resolve, reject });
      
      const message: WorkerMessage = {
        id,
        action,
        payload
      };
      
      this.worker.postMessage(message);
      
      // Set timeout to prevent hanging promises
      setTimeout(() => {
        if (this.pendingRequests.has(id)) {
          this.pendingRequests.delete(id);
          reject(new Error(`Request timed out: ${action}`));
        }
      }, 5000);
    });
  }
  
  // API methods
  async query<T>(collection: string, filter: Record<string, any> = {}, sort?: string, order?: 'asc' | 'desc'): Promise<T[]> {
    return this.sendMessage<T[]>('query', { collection, filter, sort, order });
  }
  
  async get<T>(collection: string, id: string): Promise<T | null> {
    return this.sendMessage<T | null>('get', { collection, id });
  }
  
  async insert<T>(collection: string, id: string, data: T): Promise<boolean> {
    return this.sendMessage<boolean>('insert', { collection, id, data });
  }
  
  async update<T>(collection: string, id: string, data: Partial<T>): Promise<boolean> {
    return this.sendMessage<boolean>('update', { collection, id, data });
  }
  
  async delete(collection: string, id: string): Promise<boolean> {
    return this.sendMessage<boolean>('delete', { collection, id });
  }
  
  async import(data: Record<string, Array<[string, any]>>): Promise<boolean> {
    return this.sendMessage<boolean>('import', { data });
  }
  
  async export(): Promise<Record<string, Array<[string, any]>>> {
    return this.sendMessage<Record<string, Array<[string, any]>>>('export', {});
  }
  
  // Batch operations for efficiency
  async batchInsert<T>(collection: string, items: Array<[string, T]>): Promise<boolean> {
    const promises = items.map(([id, data]) => this.insert(collection, id, data));
    await Promise.all(promises);
    return true;
  }
  
  async batchGet<T>(collection: string, ids: string[]): Promise<(T | null)[]> {
    const promises = ids.map(id => this.get<T>(collection, id));
    return Promise.all(promises);
  }
  
  // Terminate worker when done
  terminate(): void {
    this.worker.terminate();
  }
}

// Singleton instance
let databaseClient: DatabaseClient | null = null;

export function getDatabase(): DatabaseClient {
  if (!databaseClient) {
    databaseClient = new DatabaseClient();
  }
  return databaseClient;
}