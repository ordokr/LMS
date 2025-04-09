// Define message types for worker communication
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

// In-memory database for faster operations
class MemoryDatabase {
  private store: Map<string, Map<string, any>> = new Map();
  private indices: Map<string, Map<string, Set<string>>> = new Map();
  
  constructor() {
    // Initialize common collections
    this.createCollection('topics');
    this.createCollection('posts');
    this.createCollection('users');
    this.createCollection('categories');
    
    // Create indices for common queries
    this.createIndex('topics', 'category_id');
    this.createIndex('posts', 'topic_id');
    this.createIndex('posts', 'user_id');
  }
  
  createCollection(name: string): void {
    if (!this.store.has(name)) {
      this.store.set(name, new Map());
      this.indices.set(name, new Map());
    }
  }
  
  createIndex(collection: string, field: string): void {
    if (!this.store.has(collection)) {
      this.createCollection(collection);
    }
    
    const indices = this.indices.get(collection)!;
    if (!indices.has(field)) {
      indices.set(field, new Map());
      
      // Build index for existing items
      const items = this.store.get(collection)!;
      for (const [id, item] of items.entries()) {
        this.updateIndex(collection, field, id, null, item[field]);
      }
    }
  }
  
  private updateIndex(collection: string, field: string, id: string, oldValue: any, newValue: any): void {
    const indices = this.indices.get(collection);
    if (!indices) return;
    
    const fieldIndex = indices.get(field);
    if (!fieldIndex) return;
    
    // Remove from old value index
    if (oldValue !== null && oldValue !== undefined) {
      const oldValueKey = String(oldValue);
      const oldValueSet = fieldIndex.get(oldValueKey);
      if (oldValueSet) {
        oldValueSet.delete(id);
        if (oldValueSet.size === 0) {
          fieldIndex.delete(oldValueKey);
        }
      }
    }
    
    // Add to new value index
    if (newValue !== null && newValue !== undefined) {
      const newValueKey = String(newValue);
      let newValueSet = fieldIndex.get(newValueKey);
      if (!newValueSet) {
        newValueSet = new Set();
        fieldIndex.set(newValueKey, newValueSet);
      }
      newValueSet.add(id);
    }
  }
  
  insert(collection: string, id: string, data: any): void {
    const items = this.store.get(collection);
    if (!items) return;
    
    // Update indices before inserting
    const indices = this.indices.get(collection);
    if (indices) {
      for (const [field, _] of indices.entries()) {
        if (field in data) {
          this.updateIndex(collection, field, id, null, data[field]);
        }
      }
    }
    
    items.set(id, data);
  }
  
  update(collection: string, id: string, data: any): boolean {
    const items = this.store.get(collection);
    if (!items) return false;
    
    const existingItem = items.get(id);
    if (!existingItem) return false;
    
    // Update indices
    const indices = this.indices.get(collection);
    if (indices) {
      for (const [field, _] of indices.entries()) {
        if (field in data && data[field] !== existingItem[field]) {
          this.updateIndex(collection, field, id, existingItem[field], data[field]);
        }
      }
    }
    
    // Update data
    items.set(id, { ...existingItem, ...data });
    return true;
  }
  
  delete(collection: string, id: string): boolean {
    const items = this.store.get(collection);
    if (!items) return false;
    
    const existingItem = items.get(id);
    if (!existingItem) return false;
    
    // Update indices
    const indices = this.indices.get(collection);
    if (indices) {
      for (const [field, _] of indices.entries()) {
        if (field in existingItem) {
          this.updateIndex(collection, field, id, existingItem[field], null);
        }
      }
    }
    
    items.delete(id);
    return true;
  }
  
  get(collection: string, id: string): any {
    const items = this.store.get(collection);
    if (!items) return null;
    
    return items.get(id) || null;
  }
  
  query(collection: string, filter: Record<string, any>, sort?: string, order?: 'asc' | 'desc'): any[] {
    const items = this.store.get(collection);
    if (!items) return [];
    
    // Find best index to use
    const indices = this.indices.get(collection);
    let results: any[] = [];
    
    if (indices) {
      // Check if we can use an index
      const filterFields = Object.keys(filter);
      const indexedField = filterFields.find(field => indices.has(field));
      
      if (indexedField && filter[indexedField] !== undefined) {
        // Use index for this field
        const fieldIndex = indices.get(indexedField)!;
        const valueKey = String(filter[indexedField]);
        const matchingIds = fieldIndex.get(valueKey);
        
        if (matchingIds) {
          // Get all items matching the index
          results = Array.from(matchingIds).map(id => {
            const item = items.get(id)!;
            return { id, ...item };
          });
          
          // Apply remaining filters
          if (filterFields.length > 1) {
            results = results.filter(item => {
              return filterFields.every(field => {
                if (field === indexedField) return true;
                return item[field] === filter[field];
              });
            });
          }
        }
      } else {
        // No usable index, scan all items
        results = Array.from(items.entries()).map(([id, item]) => ({ id, ...item }));
        
        // Apply filters
        results = results.filter(item => {
          return Object.entries(filter).every(([field, value]) => {
            return item[field] === value;
          });
        });
      }
    } else {
      // No indices at all, scan all items
      results = Array.from(items.entries()).map(([id, item]) => ({ id, ...item }));
      
      // Apply filters
      results = results.filter(item => {
        return Object.entries(filter).every(([field, value]) => {
          return item[field] === value;
        });
      });
    }
    
    // Apply sorting if specified
    if (sort) {
      const sortDir = order === 'desc' ? -1 : 1;
      results.sort((a, b) => {
        if (a[sort] < b[sort]) return -1 * sortDir;
        if (a[sort] > b[sort]) return 1 * sortDir;
        return 0;
      });
    }
    
    return results;
  }
  
  // Bulk operations for better performance
  bulkInsert(collection: string, items: Array<[string, any]>): void {
    for (const [id, data] of items) {
      this.insert(collection, id, data);
    }
  }
  
  // Import data from JSON
  importJSON(data: Record<string, Array<[string, any]>>): void {
    for (const [collection, items] of Object.entries(data)) {
      this.createCollection(collection);
      this.bulkInsert(collection, items);
    }
  }
  
  // Export data to JSON
  exportJSON(): Record<string, Array<[string, any]>> {
    const result: Record<string, Array<[string, any]>> = {};
    
    for (const [collection, items] of this.store.entries()) {
      result[collection] = Array.from(items.entries());
    }
    
    return result;
  }
}

// Worker logic
const db = new MemoryDatabase();
const pendingRequests = new Map<string, (response: WorkerResponse) => void>();

// Handle messages
self.addEventListener('message', (event) => {
  const message: WorkerMessage = event.data;
  
  try {
    let result: any;
    
    switch (message.action) {
      case 'query':
        const { collection, filter, sort, order } = message.payload;
        result = db.query(collection, filter, sort, order);
        break;
        
      case 'get':
        result = db.get(message.payload.collection, message.payload.id);
        break;
        
      case 'insert':
        db.insert(message.payload.collection, message.payload.id, message.payload.data);
        result = true;
        break;
        
      case 'update':
        result = db.update(message.payload.collection, message.payload.id, message.payload.data);
        break;
        
      case 'delete':
        result = db.delete(message.payload.collection, message.payload.id);
        break;
        
      case 'import':
        db.importJSON(message.payload.data);
        result = true;
        break;
        
      case 'export':
        result = db.exportJSON();
        break;
        
      default:
        throw new Error(`Unknown action: ${message.action}`);
    }
    
    const response: WorkerResponse = {
      id: message.id,
      success: true,
      result
    };
    
    self.postMessage(response);
  } catch (error) {
    const response: WorkerResponse = {
      id: message.id,
      success: false,
      error: error instanceof Error ? error.message : String(error)
    };
    
    self.postMessage(response);
  }
});

// Acknowledge that worker is ready
self.postMessage({ id: 'init', success: true, result: 'Worker initialized' });