# LMS Integration Project Analyzer

A unified tool for analyzing the LMS integration project codebase.

## Project Structure

- `unified-project-analyzer.js`: Main analyzer orchestration
- `fileSystemUtils.js`: File discovery and reading
- `analysisUtils.js`: Code analysis for models, APIs, and UI components
- `astAnalyzer.js`: AST parsing and code complexity analysis
- `projectPredictor.js`: Project completion prediction

## Dependencies

- Node.js 14+
- @babel/parser
- @babel/traverse

## Usage

```bash
# Install dependencies
npm install @babel/parser @babel/traverse

# Run the analyzer on the current directory
node unified-project-analyzer.js

# Run the analyzer on a specific directory
node unified-project-analyzer.js /path/to/project