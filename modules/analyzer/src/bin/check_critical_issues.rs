use std::path::PathBuf;
use clap::Parser;
use lms_analyzer::{
    core::analyzer_config::AnalyzerConfig,
    runners::analysis_runner,
};

#[derive(Parser)]
#[command(author, version, about = "Check for critical issues in the codebase")]
struct Cli {
    /// Maximum number of critical technical debt issues allowed
    #[arg(long, default_value = "0")]
    max_critical_tech_debt: usize,
    
    /// Maximum number of high technical debt issues allowed
    #[arg(long, default_value = "5")]
    max_high_tech_debt: usize,
    
    /// Minimum overall progress percentage required
    #[arg(long, default_value = "50.0")]
    min_overall_progress: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();
    
    println!("Checking for critical issues in the codebase...");
    
    // Run analysis
    let mut config = AnalyzerConfig::load(None)?;
    config.analyze_tech_debt = true;
    
    let result = analysis_runner::run_analysis(config).await?;
    
    // Check for critical issues
    let mut critical_issues_found = false;
    let mut issues = Vec::new();
    
    // Check technical debt
    if result.tech_debt_metrics.critical_issues > cli.max_critical_tech_debt {
        issues.push(format!(
            "Found {} critical technical debt issues (maximum allowed: {})",
            result.tech_debt_metrics.critical_issues,
            cli.max_critical_tech_debt
        ));
        critical_issues_found = true;
    }
    
    if result.tech_debt_metrics.high_issues > cli.max_high_tech_debt {
        issues.push(format!(
            "Found {} high technical debt issues (maximum allowed: {})",
            result.tech_debt_metrics.high_issues,
            cli.max_high_tech_debt
        ));
        critical_issues_found = true;
    }
    
    // Check overall progress
    if result.overall_progress < cli.min_overall_progress {
        issues.push(format!(
            "Overall progress is {:.1}% (minimum required: {:.1}%)",
            result.overall_progress,
            cli.min_overall_progress
        ));
        critical_issues_found = true;
    }
    
    // Output results
    if critical_issues_found {
        println!("Critical issues found:");
        for issue in issues {
            println!("- {}", issue);
        }
        std::process::exit(1);
    } else {
        println!("No critical issues found.");
        std::process::exit(0);
    }
}
