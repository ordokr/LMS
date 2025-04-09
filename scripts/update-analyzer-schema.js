const fs = require('fs');
const path = require('path');

// Path to your analyzer schema file
const analyzerSchemaPath = path.join(__dirname, '..', 'modules', 'analyzer', 'schema.json');

// Read existing schema
let analyzerSchema;
try {
  const schemaData = fs.readFileSync(analyzerSchemaPath, 'utf8');
  analyzerSchema = JSON.parse(schemaData);
} catch (error) {
  console.error('Error reading analyzer schema:', error);
  analyzerSchema = { components: [], dataModels: [] };
}

// Add blockchain components to the analyzer schema
const blockchainComponents = [
  {
    name: 'blockchain_core',
    type: 'module',
    description: 'Core blockchain implementation',
    files: ['src-tauri/src/blockchain/core.rs'],
    dependencies: ['blockchain_storage', 'blockchain_error']
  },
  {
    name: 'blockchain_storage',
    type: 'module',
    description: 'Blockchain storage layer',
    files: ['src-tauri/src/blockchain/storage.rs'],
    dependencies: ['blockchain_error']
  },
  {
    name: 'blockchain_batching',
    type: 'module',
    description: 'Transaction batching system',
    files: ['src-tauri/src/blockchain/batching.rs'],
    dependencies: ['blockchain_core', 'blockchain_anchoring']
  },
  {
    name: 'blockchain_sync',
    type: 'module',
    description: 'Synchronization management',
    files: ['src-tauri/src/blockchain/sync.rs'],
    dependencies: ['blockchain_batching']
  },
  {
    name: 'blockchain_metrics',
    type: 'module',
    description: 'Performance monitoring',
    files: ['src-tauri/src/blockchain/metrics.rs'],
    dependencies: []
  },
  {
    name: 'blockchain_governor',
    type: 'module',
    description: 'Resource management',
    files: ['src-tauri/src/blockchain/governor.rs'],
    dependencies: []
  }
];

// Add data models
const blockchainDataModels = [
  {
    name: 'BlockchainEntity',
    module: 'blockchain_core',
    fields: [
      { name: 'id', type: 'string', description: 'Unique entity identifier' },
      { name: 'entity_type', type: 'string', description: 'Type of entity' },
      { name: 'data', type: 'map<string,string>', description: 'Entity data' },
      { name: 'created_at', type: 'i64', description: 'Creation timestamp' },
      { name: 'updated_at', type: 'i64', description: 'Update timestamp' },
      { name: 'version', type: 'u64', description: 'Entity version' }
    ]
  },
  {
    name: 'UserEvent',
    module: 'blockchain_sync',
    fields: [
      { name: 'event_type', type: 'enum', description: 'Type of user event' },
      { name: 'data', type: 'variant', description: 'Event-specific data' },
      { name: 'timestamp', type: 'i64', description: 'Event timestamp' }
    ]
  }
];

// Add the blockchain components to the schema
analyzerSchema.components = [
  ...analyzerSchema.components.filter(c => !c.name.startsWith('blockchain_')),
  ...blockchainComponents
];

// Add the blockchain data models to the schema
analyzerSchema.dataModels = [
  ...analyzerSchema.dataModels.filter(m => !m.module?.startsWith('blockchain_')),
  ...blockchainDataModels
];

// Save updated schema
fs.writeFileSync(analyzerSchemaPath, JSON.stringify(analyzerSchema, null, 2));
console.log('Analyzer schema updated with blockchain components');