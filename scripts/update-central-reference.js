const fs = require('fs').promises;
const path = require('path');
const { UnifiedProjectAnalyzer } = require('../src/analysis/unified-project-analyzer');

async function updateCentralReferenceHub() {
  console.log('Updating Central Reference Hub...');
  
  // Get project status using the analyzer
  const analyzer = new UnifiedProjectAnalyzer();
  const implementationStatus = await analyzer.generateImplementationSummary();
  const apiDocumentation = await analyzer.generateApiDocumentation();
  
  // Get the current date
  const currentDate = new Date().toISOString().split('T')[0];
  
  // Fetch integration points from analyzer
  const canvasToDiscourseIntegrations = await analyzer.getIntegrationPoints('canvas', 'discourse');
  const discourseToCanvasIntegrations = await analyzer.getIntegrationPoints('discourse', 'canvas');
  
  // Format integration points into table rows
  const formatIntegrationPoints = (integrations) => {
    return integrations.map(integration => {
      return `| ${integration.sourceName} | ${integration.targetName} | ${integration.status} | ${integration.details} |`;
    }).join('\n');
  };

  const canvasToDiscourseRows = formatIntegrationPoints(canvasToDiscourseIntegrations);
  const discourseToCanvasRows = formatIntegrationPoints(discourseToCanvasIntegrations);
  
  // Generate the markdown content for the hub
  const centralReferenceContent = `# Canvas-Discourse Integration: Central Reference Hub

*Last updated: ${currentDate}*

## Overview

This document serves as the central reference for the Canvas-Discourse integration project. It provides a comprehensive guide to all components, APIs, and implementation details.

## Integration Points

### Canvas to Discourse

| Canvas Feature | Discourse Integration | Status | Implementation Details |
|---------------|---------------------|--------|------------------------|
${canvasToDiscourseRows}

### Discourse to Canvas

| Discourse Feature | Canvas Integration | Status | Implementation Details |
|------------------|-------------------|--------|------------------------|
${discourseToCanvasRows}

## API Reference

### Canvas APIs Utilized
${apiDocumentation.canvas.map(api => `- ${api.name}: ${api.description}`).join('\n')}

### Discourse APIs Utilized
${apiDocumentation.discourse.map(api => `- ${api.name}: ${api.description}`).join('\n')}

## Implementation Guidelines

When implementing new integration points, follow these guidelines:

1. Each integration point should have corresponding tests in the \`tests/integration\` directory
2. All API calls should utilize the respective client libraries in \`src/clients\`
3. Integration services should be stateless when possible
4. Error handling should include appropriate logging and retry mechanisms

## Implementation Status

- Models: ${implementationStatus.models.percentage}% complete
- API Endpoints: ${implementationStatus.api.percentage}% complete
- UI Components: ${implementationStatus.ui.percentage}% complete
- Test Coverage: ${implementationStatus.tests.coveragePercentage}%

## Recent Changes

${implementationStatus.recentChanges.map(change => `- ${change}`).join('\n')}

## Known Issues

${implementationStatus.knownIssues.map(issue => `- ${issue}`).join('\n')}
`;

  // Write the updated content to the Central Reference Hub file
  const hubPath = path.join(__dirname, '../docs/central_reference_hub.md');
  await fs.writeFile(hubPath, centralReferenceContent);
  
  console.log(`Central Reference Hub updated: ${hubPath}`);
}

updateCentralReferenceHub().catch(err => {
  console.error('Failed to update Central Reference Hub:', err);
  process.exit(1);
});