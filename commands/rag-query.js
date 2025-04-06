const { Command } = require('commander');
const UnifiedProjectAnalyzer = require('../unified-project-analyzer');

function registerRagQueryCommand(program) {
  program
    .command('rag-query')
    .description('Query the RAG system for information about Canvas and Discourse')
    .argument('<query>', 'The query to search for')
    .option('-s, --system <system>', 'Filter by system (canvas, discourse, integration)')
    .option('-c, --category <category>', 'Filter by category (models, architecture, apis, etc.)')
    .option('-t, --top <number>', 'Number of results to return', parseInt, 3)
    .option('--context', 'Generate context for LLM', false)
    .action(async (query, options) => {
      const baseDir = process.cwd();
      const analyzer = new UnifiedProjectAnalyzer(baseDir);
      
      // Build filter
      const filters = {};
      if (options.system) filters.system = options.system;
      if (options.category) filters.category = options.category;
      
      // Execute query
      const results = await analyzer.queryRag(query, {
        filters,
        topK: options.top,
        generateContext: options.context
      });
      
      // Display results
      console.log(`\nFound ${results.resultsCount} relevant documents for: "${query}"\n`);
      
      results.documents.forEach((doc, index) => {
        console.log(`${index + 1}. ${doc.id} (Score: ${doc.score.toFixed(2)})`);
        console.log(`   System: ${doc.metadata?.system || 'unknown'}, Category: ${doc.metadata?.category || 'unknown'}`);
        
        // Show snippet
        if (doc.content) {
          const snippet = doc.content.substring(0, 150).replace(/\n/g, ' ') + '...';
          console.log(`   ${snippet}`);
        }
        console.log();
      });
      
      if (options.context && results.context) {
        console.log("\n--- LLM Context ---\n");
        console.log(results.context);
      }
    });
}

module.exports = registerRagQueryCommand;