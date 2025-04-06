const path = require('path');
const fs = require('fs-extra');

/**
 * Knowledge Graph Generator
 * Creates a graph representation of system entities and their relationships
 */
class KnowledgeGraphGenerator {
  constructor(metrics, options = {}) {
    this.metrics = metrics;
    this.options = Object.assign({
      outputDir: 'knowledge_base',
      graphFormat: 'json', // 'json', 'cytoscape', 'neo4j'
    }, options);
    
    // Initialize graph data structure
    this.graph = {
      nodes: [],
      edges: []
    };
    
    // Track node IDs to prevent duplicates
    this.nodeIds = new Set();
  }
  
  /**
   * Generate a knowledge graph from system data
   */
  async generateKnowledgeGraph(baseDir, sourceSystems) {
    console.log("Generating knowledge graph...");
    
    const outputDir = path.join(baseDir, this.options.outputDir);
    await fs.ensureDir(outputDir);
    
    // Clear existing graph
    this.graph = { nodes: [], edges: [] };
    this.nodeIds.clear();
    
    // Process models and their relationships
    await this.processModelsAndRelationships();
    
    // Process file dependencies
    await this.processFileDependencies();
    
    // Process architecture components
    await this.processArchitectureComponents();
    
    // Generate cross-system integration nodes
    await this.generateIntegrationNodes();
    
    // Write graph to file
    const graphPath = path.join(outputDir, 'knowledge_graph.json');
    await fs.writeFile(graphPath, JSON.stringify(this.graph, null, 2));
    
    // Generate visualization if needed
    if (this.options.generateVisualization) {
      await this.generateVisualization(outputDir);
    }
    
    console.log(`Knowledge graph generated with ${this.graph.nodes.length} nodes and ${this.graph.edges.length} edges`);
    
    return graphPath;
  }
  
  /**
   * Process models and their relationships
   */
  async processModelsAndRelationships() {
    console.log("Processing models and relationships...");
    
    // Process models from both systems
    for (const systemName of Object.keys(this.metrics.sourceSystems || {})) {
      const models = this.metrics.sourceSystems[systemName]?.models?.details || [];
      
      models.forEach(model => {
        this.addNode({
          id: `${systemName}.model.${model.name}`,
          label: model.name,
          type: 'model',
          system: systemName,
          attributes: {
            ...model,
            system: systemName
          }
        });
      });
    }
    
    // Process relationships
    const relationships = this.metrics.relationships || [];
    relationships.forEach(rel => {
      let fromSystem = 'unknown';
      let toSystem = 'unknown';
      
      // Determine which system each model belongs to
      for (const systemName of Object.keys(this.metrics.sourceSystems || {})) {
        const models = this.metrics.sourceSystems[systemName]?.models?.details || [];
        
        if (models.some(m => m.name === rel.from)) {
          fromSystem = systemName;
        }
        
        if (models.some(m => m.name === rel.to)) {
          toSystem = systemName;
        }
      }
      
      // Add edge between models
      this.addEdge({
        source: `${fromSystem}.model.${rel.from}`,
        target: `${toSystem}.model.${rel.to}`,
        label: rel.type,
        type: 'relationship',
        attributes: {
          relationshipType: rel.type,
          notes: rel.notes || ''
        }
      });
    });
  }
  
  /**
   * Process file dependencies
   */
  async processFileDependencies() {
    console.log("Processing file dependencies...");
    
    // We'll use controllers and their dependencies as an example
    for (const systemName of Object.keys(this.metrics.sourceSystems || {})) {
      const controllers = this.metrics.sourceSystems[systemName]?.controllers?.details || [];
      
      controllers.forEach(controller => {
        const controllerId = `${systemName}.controller.${controller.name}`;
        
        this.addNode({
          id: controllerId,
          label: controller.name,
          type: 'controller',
          system: systemName,
          attributes: {
            ...controller,
            system: systemName
          }
        });
        
        // Connect controllers to models they use
        const usedModels = controller.usedModels || [];
        usedModels.forEach(model => {
          // Try to find the model node
          const modelId = `${systemName}.model.${model}`;
          
          // Add edge only if the model node exists
          if (this.nodeIds.has(modelId)) {
            this.addEdge({
              source: controllerId,
              target: modelId,
              label: 'uses',
              type: 'dependency',
              attributes: {
                dependencyType: 'controller_model'
              }
            });
          }
        });
      });
    }
  }
  
  /**
   * Process architecture components
   */
  async processArchitectureComponents() {
    console.log("Processing architecture components...");
    
    // Define core architectural components for each system
    const architectureComponents = {
      canvas: [
        { id: 'auth', label: 'Authentication', type: 'component' },
        { id: 'courses', label: 'Courses', type: 'component' },
        { id: 'assignments', label: 'Assignments', type: 'component' },
        { id: 'gradebook', label: 'Gradebook', type: 'component' },
        { id: 'modules', label: 'Modules', type: 'component' },
        { id: 'files', label: 'Files', type: 'component' },
        { id: 'api', label: 'API', type: 'interface' }
      ],
      discourse: [
        { id: 'auth', label: 'Authentication', type: 'component' },
        { id: 'topics', label: 'Topics', type: 'component' },
        { id: 'posts', label: 'Posts', type: 'component' },
        { id: 'categories', label: 'Categories', type: 'component' },
        { id: 'users', label: 'Users', type: 'component' },
        { id: 'plugins', label: 'Plugins', type: 'component' },
        { id: 'api', label: 'API', type: 'interface' }
      ]
    };
    
    // Add architectural components as nodes
    for (const [systemName, components] of Object.entries(architectureComponents)) {
      components.forEach(component => {
        this.addNode({
          id: `${systemName}.arch.${component.id}`,
          label: component.label,
          type: 'architecture',
          system: systemName,
          attributes: {
            componentType: component.type,
            system: systemName
          }
        });
      });
      
      // Add relationships between components
      if (systemName === 'canvas') {
        // Example Canvas component relationships
        this.addEdge({
          source: `${systemName}.arch.auth`,
          target: `${systemName}.arch.courses`,
          label: 'authenticates',
          type: 'arch_relation'
        });
        
        this.addEdge({
          source: `${systemName}.arch.courses`,
          target: `${systemName}.arch.assignments`,
          label: 'contains',
          type: 'arch_relation'
        });
        
        this.addEdge({
          source: `${systemName}.arch.assignments`,
          target: `${systemName}.arch.gradebook`,
          label: 'feeds',
          type: 'arch_relation'
        });
        
        this.addEdge({
          source: `${systemName}.arch.api`,
          target: `${systemName}.arch.courses`,
          label: 'exposes',
          type: 'arch_relation'
        });
      } else if (systemName === 'discourse') {
        // Example Discourse component relationships
        this.addEdge({
          source: `${systemName}.arch.auth`,
          target: `${systemName}.arch.users`,
          label: 'authenticates',
          type: 'arch_relation'
        });
        
        this.addEdge({
          source: `${systemName}.arch.categories`,
          target: `${systemName}.arch.topics`,
          label: 'contains',
          type: 'arch_relation'
        });
        
        this.addEdge({
          source: `${systemName}.arch.topics`,
          target: `${systemName}.arch.posts`,
          label: 'contains',
          type: 'arch_relation'
        });
        
        this.addEdge({
          source: `${systemName}.arch.api`,
          target: `${systemName}.arch.topics`,
          label: 'exposes',
          type: 'arch_relation'
        });
      }
      
      // Connect architecture components to related models
      this.connectArchitectureToModels(systemName);
    }
  }
  
  /**
   * Connect architecture components to related models
   */
  connectArchitectureToModels(systemName) {
    const models = this.metrics.sourceSystems[systemName]?.models?.details || [];
    
    // Map models to architecture components based on naming patterns
    models.forEach(model => {
      const modelId = `${systemName}.model.${model.name}`;
      let connectedToArch = false;
      
      // Define mapping rules based on system
      const mappingRules = systemName === 'canvas' ? {
        'auth': /user|account|profile|session|authent/i,
        'courses': /course|section|enrollment/i,
        'assignments': /assign|submiss|quiz|rubric/i,
        'gradebook': /grade|score|rubric/i,
        'modules': /module|content|page|item/i,
        'files': /file|folder|attachment/i
      } : {
        'auth': /user|account|profile|session|authent/i,
        'topics': /topic|thread/i,
        'posts': /post|reply|message/i,
        'categories': /categor|tag|group/i,
        'users': /user|profile|account/i,
        'plugins': /plugin|extension|add_on/i
      };
      
      // Connect model to matching architecture component
      Object.entries(mappingRules).forEach(([component, pattern]) => {
        if (pattern.test(model.name)) {
          const componentId = `${systemName}.arch.${component}`;
          
          if (this.nodeIds.has(componentId)) {
            this.addEdge({
              source: modelId,
              target: componentId,
              label: 'implements',
              type: 'architecture_mapping',
              attributes: {
                confidence: pattern.test(model.name.toLowerCase()) ? 'high' : 'medium',
                mapping_type: 'semantic'
              }
            });
            connectedToArch = true;
          }
        }
      });
    });
  }
  
  /**
   * Add a node to the graph
   */
  addNode(node) {
    if (!this.nodeIds.has(node.id)) {
      this.graph.nodes.push(node);
      this.nodeIds.add(node.id);
    }
  }
  
  /**
   * Add an edge to the graph
   */
  addEdge(edge) {
    this.graph.edges.push(edge);
  }
  
  /**
   * Generate cross-system integration nodes
   */
  async generateIntegrationNodes() {
    console.log("Generating integration nodes...");
    
    // Example: Add integration nodes between Canvas and Discourse
    this.addNode({
      id: 'integration.canvas_discourse',
      label: 'Canvas-Discourse Integration',
      type: 'integration',
      attributes: {
        description: 'Integration between Canvas and Discourse systems'
      }
    });
    
    // Example: Add edges representing integration points
    this.addEdge({
      source: 'canvas.arch.api',
      target: 'integration.canvas_discourse',
      label: 'integrates_with',
      type: 'integration_point'
    });
    
    this.addEdge({
      source: 'discourse.arch.api',
      target: 'integration.canvas_discourse',
      label: 'integrates_with',
      type: 'integration_point'
    });
  }
  
  /**
   * Generate visualization of the knowledge graph
   */
  async generateVisualization(outputDir) {
    console.log("Generating visualization...");
    
    // Example: Generate a simple HTML visualization
    const htmlContent = `
      <!DOCTYPE html>
      <html>
      <head>
        <title>Knowledge Graph Visualization</title>
        <style>
          body { font-family: Arial, sans-serif; }
          #graph { width: 100%; height: 100vh; }
        </style>
      </head>
      <body>
        <h1>Knowledge Graph Visualization</h1>
        <div id="graph"></div>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/cytoscape/3.19.1/cytoscape.min.js"></script>
        <script>
          const graphData = ${JSON.stringify(this.graph, null, 2)};
          const cy = cytoscape({
            container: document.getElementById('graph'),
            elements: graphData,
            style: [
              { selector: 'node', style: { 'label': 'data(label)' } },
              { selector: 'edge', style: { 'label': 'data(label)', 'curve-style': 'bezier', 'target-arrow-shape': 'triangle' } }
            ],
            layout: { name: 'cose' }
          });
        </script>
      </body>
      </html>
    `;
    
    const htmlPath = path.join(outputDir, 'visualization.html');
    await fs.writeFile(htmlPath, htmlContent);
    
    console.log(`Visualization generated at ${htmlPath}`);
  }
}

module.exports = KnowledgeGraphGenerator;