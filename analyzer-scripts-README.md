# LMS Project Analyzer Scripts

This directory contains scripts for running the LMS Project Analyzers and generating reports.

## Available Scripts

### Main Analysis Scripts

- **unified-analyze.bat**: The main script for running the analyzers. Supports various options for customizing the analysis.
- **test-analyzers.bat**: Runs all analyzers and generates all reports. Useful for testing the analyzers.
- **generate-comprehensive-report.bat**: Generates a comprehensive analysis report that includes all aspects of the project.
- **generate-enhanced-dashboard.bat**: Generates an enhanced visual dashboard of the project status.
- **generate-trend-report.bat**: Generates a trend analysis report that tracks changes in the codebase over time.
- **generate-statistical-trend-report.bat**: Generates a statistical trend analysis report with forecasts and confidence intervals.
- **generate-llm-insights.bat**: Generates AI-powered insights using LM Studio with Qwen 2.5-coder-32b-instruct model.

### Automation Scripts

- **schedule-analysis.ps1**: Sets up a scheduled task to run the analyzers daily. Requires PowerShell.
- **setup-git-hooks.ps1**: Sets up Git hooks to run the analyzers automatically on commit and after merge. Requires PowerShell.
- **trigger-analysis.bat**: Manually triggers the scheduled analysis task.

## Usage

### Running a Full Analysis

To run a full analysis with all options:

```batch
unified-analyze.bat --full --tech-debt --code-quality --models --dashboard
```

### Running a Quick Analysis

To run a quick analysis:

```batch
unified-analyze.bat --quick
```

### Updating the Central Reference Hub

To update only the central reference hub:

```batch
unified-analyze.bat --update-hub
```

### Generating a Summary Report

To generate a summary report:

```batch
unified-analyze.bat --summary
```

### Setting Up Scheduled Analysis

To set up a scheduled task to run the analyzers daily:

```powershell
.\schedule-analysis.ps1
```

### Setting Up Git Hooks

To set up Git hooks to run the analyzers automatically:

```powershell
.\setup-git-hooks.ps1
```

## Generated Reports

The analyzers generate the following reports:

- **docs/central_reference_hub.md**: The main entry point for project documentation.
- **docs/SUMMARY_REPORT.md**: Summary of the project's status.
- **docs/technical_debt_report.md**: Report on technical debt in the project.
- **docs/code_quality_summary.md**: Summary of code quality in the project.
- **docs/model_summary.md**: Summary of data models in the project.
- **docs/dependency_summary.md**: Summary of project dependencies.
- **docs/trend_summary.md**: Summary of project trends over time.
- **docs/statistical_trend_summary.md**: Summary of statistical trend analysis with forecasts.
- **docs/comprehensive_report.md**: Comprehensive analysis report.
- **docs/dashboard.html**: Visual dashboard of project metrics.
- **docs/enhanced_dashboard.html**: Enhanced visual dashboard with interactive charts.
- **docs/trends/trend_report.md**: Detailed trend analysis report.
- **docs/trends/statistical_trend_report.md**: Statistical trend analysis with forecasts and confidence intervals.
- **docs/insights/llm_insights_report.md**: AI-powered insights generated using LM Studio with Qwen 2.5.

## Iterative Development Process

The analyzers are designed to support an iterative development process:

1. **Analyze**: Run the analyzers to gather insights about the codebase.
2. **Report**: Generate documentation and reports.
3. **Read**: Review the generated documentation and reports.
4. **Build**: Implement changes based on the insights.
5. **Repeat**: Run the analyzers again to track progress.

This iterative process ensures that the project's documentation is always up-to-date and that development is guided by data-driven insights.
