const fs = require('fs').promises;
const path = require('path');
const { metrics } = require('../src/monitoring/metrics');
const { FeedbackCollector } = require('../src/feedback/collector');
const { UnifiedProjectAnalyzer } = require('../src/analysis/unified-project-analyzer');

async function generateStakeholderDashboard() {
  console.log('Generating stakeholder dashboard...');
  
  try {
    // Get project implementation status
    const analyzer = new UnifiedProjectAnalyzer();
    const implementationStatus = await analyzer.generateImplementationSummary();
    
    // Get performance metrics
    let performanceMetrics = {};
    try {
      performanceMetrics = metrics.getAllMetrics();
    } catch (error) {
      console.warn('Could not retrieve performance metrics:', error.message);
      performanceMetrics = {};
    }
    
    // Get feedback summary
    let feedbackSummary = {
      totalItems: 0,
      categoryCounts: {},
      averageRatings: {},
      recentFeedback: []
    };
    
    try {
      const feedbackCollector = new FeedbackCollector();
      // Wait for initialization to complete
      await new Promise(resolve => setTimeout(resolve, 1000));
      feedbackSummary = await feedbackCollector.generateFeedbackSummary();
    } catch (error) {
      console.warn('Could not generate feedback summary:', error.message);
    }
    
    // Create directories if they don't exist
    const docsDir = path.join(__dirname, '../docs');
    await fs.mkdir(docsDir, { recursive: true });
    
    // Generate dashboard markdown
    const dashboardContent = `# Canvas-Discourse Integration Dashboard

*Generated on: ${new Date().toISOString().split('T')[0]}*

## Implementation Status

- **Models**: ${implementationStatus.models.percentage}% complete
- **API Endpoints**: ${implementationStatus.api.percentage}% complete
- **UI Components**: ${implementationStatus.ui.percentage}% complete
- **Test Coverage**: ${implementationStatus.tests.coveragePercentage}%

## Performance Metrics

${Object.entries(performanceMetrics)
  .filter(([key]) => key && (key.startsWith('api.') || key.startsWith('integration.')))
  .map(([key, value]) => {
    if (value && value.type === 'timer') {
      return `- **${key}**: Avg ${value.avg?.toFixed(2)}ms (${value.count} calls)`;
    }
    return `- **${key}**: ${value?.value}`;
  })
  .join('\n') || '- No metrics available yet'}

## Recent Feedback

Total feedback items: ${feedbackSummary.totalItems}

${Object.entries(feedbackSummary.categoryCounts)
  .map(([category, count]) => {
    const rating = feedbackSummary.averageRatings[category] 
      ? `(Avg rating: ${feedbackSummary.averageRatings[category]})` 
      : '';
    return `- **${category}**: ${count} items ${rating}`;
  })
  .join('\n') || '- No feedback categories available'}

${feedbackSummary.totalItems > 0 ? '### Most Recent Feedback\n\n' + 
  feedbackSummary.recentFeedback
    .map(item => `- **${item.category}**: "${item.content.substring(0, 100)}${item.content.length > 100 ? '...' : ''}" (${new Date(item.timestamp).toISOString().split('T')[0]})`)
    .join('\n') : '- No feedback available yet'}

## Next Steps

1. Continue integration testing for remaining components
2. Address feedback related to performance issues
3. Complete implementation of Assignment synchronization
4. Expand API monitoring coverage

`;

    // Write dashboard to file
    const outputPath = path.join(__dirname, '../docs/stakeholder_dashboard.md');
    await fs.writeFile(outputPath, dashboardContent);
    
    console.log(`Dashboard generated and saved to ${outputPath}`);
  } catch (error) {
    console.error('Failed to generate dashboard:', error);
    throw error;
  }
}

// Execute the function
generateStakeholderDashboard().catch(err => {
  console.error('Failed to generate dashboard:', err);
  process.exit(1);
});