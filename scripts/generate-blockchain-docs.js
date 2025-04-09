const fs = require('fs');
const path = require('path');

// Define the blockchain components and their descriptions
const blockchainComponents = [
  {
    name: 'HybridChain',
    description: 'Core blockchain implementation with CRDT-based consensus',
    methods: [
      { name: 'create_entity', params: ['entity_type', 'data'], description: 'Creates a new entity in the blockchain' },
      { name: 'update_entity', params: ['entity_id', 'data'], description: 'Updates an existing entity' },
      { name: 'get_entity', params: ['entity_id'], description: 'Retrieves an entity by ID' },
      { name: 'verify_entity', params: ['entity_id'], description: 'Verifies an entity exists in the blockchain' },
      { name: 'create_block', params: [], description: 'Creates a new block with the current state' }
    ]
  },
  {
    name: 'AdaptiveBatcher',
    description: 'Intelligent batching system for transaction processing',
    methods: [
      { name: 'add_change', params: ['change', 'priority'], description: 'Adds a change to the batch queue' },
      { name: 'process_batch', params: [], description: 'Processes pending changes in a batch' },
      { name: 'start_batch_loop', params: [], description: 'Starts the background batch processing loop' }
    ]
  },
  {
    name: 'AdaptiveSyncManager',
    description: 'Manages synchronization of blockchain events',
    methods: [
      { name: 'sync_event', params: ['event'], description: 'Synchronizes an event to the blockchain' },
      { name: 'force_sync', params: ['event'], description: 'Forces immediate synchronization of an event' },
      { name: 'determine_sync_priority', params: ['event'], description: 'Determines the sync priority for an event' }
    ]
  }
];

// Generate the API documentation in Markdown
function generateBlockchainApiDocs() {
  let content = '# Blockchain API Documentation\n\n';
  content += 'This document describes the API for the LMS blockchain implementation.\n\n';
  
  blockchainComponents.forEach(component => {
    content += `## ${component.name}\n\n`;
    content += `${component.description}\n\n`;
    content += '### Methods\n\n';
    
    component.methods.forEach(method => {
      content += `#### \`${method.name}(${method.params.join(', ')})\`\n\n`;
      content += `${method.description}\n\n`;
    });
    
    content += '---\n\n';
  });
  
  // Write the documentation file
  const docsDir = path.join(__dirname, '..', 'docs');
  if (!fs.existsSync(docsDir)) {
    fs.mkdirSync(docsDir, { recursive: true });
  }
  
  const filePath = path.join(docsDir, 'blockchain_api.md');
  fs.writeFileSync(filePath, content);
  console.log(`Blockchain API documentation generated at: ${filePath}`);
}

// Execute the function
generateBlockchainApiDocs();