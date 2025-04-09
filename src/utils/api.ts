// Implement request batching and debouncing
type BatchKey = string;
type RequestId = string;
type RequestHandler<T> = () => Promise<T>;

interface BatchedRequest<T> {
  id: RequestId;
  handler: RequestHandler<T>;
  resolve: (value: T) => void;
  reject: (reason: any) => void;
}

class RequestBatcher {
  private batches: Map<BatchKey, BatchedRequest<any>[]> = new Map();
  private timeouts: Map<BatchKey, number> = new Map();
  private maxBatchSize: number = 50;
  private batchDelay: number = 20; // ms
  
  constructor(maxBatchSize: number = 50, batchDelay: number = 20) {
    this.maxBatchSize = maxBatchSize;
    this.batchDelay = batchDelay;
  }
  
  add<T>(batchKey: BatchKey, handler: RequestHandler<T>): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      const id = Math.random().toString(36).substring(2);
      
      if (!this.batches.has(batchKey)) {
        this.batches.set(batchKey, []);
      }
      
      const batch = this.batches.get(batchKey)!;
      batch.push({ id, handler, resolve, reject });
      
      // If we've reached max batch size, process immediately
      if (batch.length >= this.maxBatchSize) {
        this.processBatch(batchKey);
      } else if (!this.timeouts.has(batchKey)) {
        // Otherwise schedule processing
        const timeoutId = window.setTimeout(() => {
          this.processBatch(batchKey);
        }, this.batchDelay);
        
        this.timeouts.set(batchKey, timeoutId);
      }
    });
  }
  
  private processBatch(batchKey: BatchKey): void {
    const batch = this.batches.get(batchKey) || [];
    this.batches.delete(batchKey);
    
    if (this.timeouts.has(batchKey)) {
      window.clearTimeout(this.timeouts.get(batchKey));
      this.timeouts.delete(batchKey);
    }
    
    batch.forEach(request => {
      // Execute each request independently but in parallel
      request.handler()
        .then(request.resolve)
        .catch(request.reject);
    });
  }
  
  // Clear all pending batches
  clear(): void {
    this.timeouts.forEach(timeoutId => {
      window.clearTimeout(timeoutId);
    });
    
    this.timeouts.clear();
    this.batches.clear();
  }
}

// Export singleton instance
export const requestBatcher = new RequestBatcher();

// Debounce utility for user input
export function debounce<T extends (...args: any[]) => void>(
  fn: T,
  delay: number = 300
): (...args: Parameters<T>) => void {
  let timeoutId: number | undefined;
  
  return function(this: any, ...args: Parameters<T>) {
    if (timeoutId !== undefined) {
      window.clearTimeout(timeoutId);
    }
    
    timeoutId = window.setTimeout(() => {
      fn.apply(this, args);
      timeoutId = undefined;
    }, delay);
  };
}