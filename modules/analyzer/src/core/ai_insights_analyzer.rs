use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;
use crate::core::tech_debt_analyzer::TechDebtAnalyzer;
use crate::core::code_quality_analyzer::CodeQualityAnalyzer;
use crate::core::model_analyzer::ModelAnalyzer;
use crate::core::dependency_analyzer::DependencyAnalyzer;
use crate::core::trend_analyzer::TrendAnalyzer;

/// AI insights analyzer
pub struct AiInsightsAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
}

/// AI insight
#[derive(Debug, Clone)]
pub struct AiInsight {
    /// Insight category
    pub category: InsightCategory,
    
    /// Insight title
    pub title: String,
    
    /// Insight description
    pub description: String,
    
    /// Insight priority
    pub priority: InsightPriority,
    
    /// Recommended actions
    pub recommendations: Vec<String>,
    
    /// Related files
    pub related_files: Vec<String>,
}

/// Insight category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsightCategory {
    /// Technical debt
    TechnicalDebt,
    
    /// Code quality
    CodeQuality,
    
    /// Architecture
    Architecture,
    
    /// Performance
    Performance,
    
    /// Security
    Security,
    
    /// Testing
    Testing,
    
    /// Documentation
    Documentation,
    
    /// Dependencies
    Dependencies,
    
    /// Project progress
    ProjectProgress,
}

/// Insight priority
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InsightPriority {
    /// Critical priority
    Critical,
    
    /// High priority
    High,
    
    /// Medium priority
    Medium,
    
    /// Low priority
    Low,
}

/// AI insights
#[derive(Debug, Clone)]
pub struct AiInsights {
    /// Insights
    pub insights: Vec<AiInsight>,
    
    /// Insights by category
    pub insights_by_category: HashMap<InsightCategory, Vec<AiInsight>>,
    
    /// Insights by priority
    pub insights_by_priority: HashMap<InsightPriority, Vec<AiInsight>>,
    
    /// Top insights
    pub top_insights: Vec<AiInsight>,
}

impl AiInsightsAnalyzer {
    /// Create a new AI insights analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
        }
    }
    
    /// Generate AI insights
    pub fn generate_insights(&self, result: &AnalysisResult) -> Result<AiInsights, String> {
        println!("Generating AI insights...");
        
        let mut insights = Vec::new();
        
        // Generate technical debt insights
        self.generate_tech_debt_insights(result, &mut insights)?;
        
        // Generate code quality insights
        self.generate_code_quality_insights(result, &mut insights)?;
        
        // Generate architecture insights
        self.generate_architecture_insights(result, &mut insights)?;
        
        // Generate performance insights
        self.generate_performance_insights(result, &mut insights)?;
        
        // Generate security insights
        self.generate_security_insights(result, &mut insights)?;
        
        // Generate testing insights
        self.generate_testing_insights(result, &mut insights)?;
        
        // Generate documentation insights
        self.generate_documentation_insights(result, &mut insights)?;
        
        // Generate dependency insights
        self.generate_dependency_insights(result, &mut insights)?;
        
        // Generate project progress insights
        self.generate_project_progress_insights(result, &mut insights)?;
        
        // Group insights by category
        let mut insights_by_category = HashMap::new();
        
        for insight in &insights {
            insights_by_category.entry(insight.category.clone())
                .or_insert_with(Vec::new)
                .push(insight.clone());
        }
        
        // Group insights by priority
        let mut insights_by_priority = HashMap::new();
        
        for insight in &insights {
            insights_by_priority.entry(insight.priority.clone())
                .or_insert_with(Vec::new)
                .push(insight.clone());
        }
        
        // Get top insights
        let mut top_insights = Vec::new();
        
        // Add all critical insights
        if let Some(critical_insights) = insights_by_priority.get(&InsightPriority::Critical) {
            top_insights.extend(critical_insights.clone());
        }
        
        // Add high priority insights if we have less than 5 top insights
        if top_insights.len() < 5 {
            if let Some(high_insights) = insights_by_priority.get(&InsightPriority::High) {
                top_insights.extend(high_insights.iter().take(5 - top_insights.len()).cloned());
            }
        }
        
        // Add medium priority insights if we still have less than 5 top insights
        if top_insights.len() < 5 {
            if let Some(medium_insights) = insights_by_priority.get(&InsightPriority::Medium) {
                top_insights.extend(medium_insights.iter().take(5 - top_insights.len()).cloned());
            }
        }
        
        // Add low priority insights if we still have less than 5 top insights
        if top_insights.len() < 5 {
            if let Some(low_insights) = insights_by_priority.get(&InsightPriority::Low) {
                top_insights.extend(low_insights.iter().take(5 - top_insights.len()).cloned());
            }
        }
        
        Ok(AiInsights {
            insights,
            insights_by_category,
            insights_by_priority,
            top_insights,
        })
    }
    
    /// Generate technical debt insights
    fn generate_tech_debt_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // Check for critical technical debt issues
        if result.tech_debt_metrics.critical_issues > 0 {
            insights.push(AiInsight {
                category: InsightCategory::TechnicalDebt,
                title: "Critical Technical Debt Issues".to_string(),
                description: format!("Found {} critical technical debt issues that should be addressed immediately.", result.tech_debt_metrics.critical_issues),
                priority: InsightPriority::Critical,
                recommendations: vec![
                    "Review and fix all critical technical debt issues as soon as possible.".to_string(),
                    "Consider setting up a dedicated technical debt reduction sprint.".to_string(),
                ],
                related_files: result.tech_debt_metrics.items.iter()
                    .filter(|item| item.severity == crate::core::analysis_result::TechDebtSeverity::Critical)
                    .map(|item| item.file.clone())
                    .collect(),
            });
        }
        
        // Check for high technical debt issues
        if result.tech_debt_metrics.high_issues > 5 {
            insights.push(AiInsight {
                category: InsightCategory::TechnicalDebt,
                title: "High Technical Debt Issues".to_string(),
                description: format!("Found {} high-priority technical debt issues that should be addressed soon.", result.tech_debt_metrics.high_issues),
                priority: InsightPriority::High,
                recommendations: vec![
                    "Review and fix high-priority technical debt issues in the next few sprints.".to_string(),
                    "Consider allocating a percentage of development time to technical debt reduction.".to_string(),
                ],
                related_files: result.tech_debt_metrics.items.iter()
                    .filter(|item| item.severity == crate::core::analysis_result::TechDebtSeverity::High)
                    .map(|item| item.file.clone())
                    .collect(),
            });
        }
        
        // Check for overall technical debt
        if result.tech_debt_metrics.total_issues > 20 {
            insights.push(AiInsight {
                category: InsightCategory::TechnicalDebt,
                title: "High Overall Technical Debt".to_string(),
                description: format!("The project has a high level of technical debt ({} issues in total).", result.tech_debt_metrics.total_issues),
                priority: InsightPriority::Medium,
                recommendations: vec![
                    "Establish a technical debt budget for each sprint.".to_string(),
                    "Implement a 'boy scout rule': leave the code cleaner than you found it.".to_string(),
                    "Consider using static analysis tools to prevent new technical debt.".to_string(),
                ],
                related_files: Vec::new(),
            });
        }
        
        Ok(())
    }
    
    /// Generate code quality insights
    fn generate_code_quality_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // Check for code complexity
        if result.code_metrics.avg_complexity > 10.0 {
            insights.push(AiInsight {
                category: InsightCategory::CodeQuality,
                title: "High Code Complexity".to_string(),
                description: format!("The average code complexity is {:.1}, which is higher than the recommended threshold of 10.", result.code_metrics.avg_complexity),
                priority: InsightPriority::Medium,
                recommendations: vec![
                    "Refactor complex functions into smaller, more manageable pieces.".to_string(),
                    "Consider using functional programming techniques to reduce complexity.".to_string(),
                    "Add more unit tests for complex functions.".to_string(),
                ],
                related_files: Vec::new(),
            });
        }
        
        // Check for large modules
        if result.code_metrics.function_count > 0 && result.code_metrics.module_count > 0 {
            let avg_functions_per_module = result.code_metrics.function_count as f32 / result.code_metrics.module_count as f32;
            
            if avg_functions_per_module > 20.0 {
                insights.push(AiInsight {
                    category: InsightCategory::CodeQuality,
                    title: "Large Modules".to_string(),
                    description: format!("The average number of functions per module is {:.1}, which is higher than the recommended threshold of 20.", avg_functions_per_module),
                    priority: InsightPriority::Medium,
                    recommendations: vec![
                        "Split large modules into smaller, more focused modules.".to_string(),
                        "Group related functions into separate modules.".to_string(),
                        "Consider using a more modular architecture.".to_string(),
                    ],
                    related_files: Vec::new(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Generate architecture insights
    fn generate_architecture_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // Check for implementation progress
        if result.models.implementation_percentage < 50.0 {
            insights.push(AiInsight {
                category: InsightCategory::Architecture,
                title: "Low Model Implementation".to_string(),
                description: format!("Only {:.1}% of models are implemented, which may indicate architectural issues.", result.models.implementation_percentage),
                priority: InsightPriority::High,
                recommendations: vec![
                    "Review the data model design to ensure it meets the project requirements.".to_string(),
                    "Prioritize implementing core models before building features that depend on them.".to_string(),
                    "Consider using code generation tools to speed up model implementation.".to_string(),
                ],
                related_files: Vec::new(),
            });
        }
        
        // Check for API implementation
        if result.api_endpoints.implementation_percentage < 50.0 {
            insights.push(AiInsight {
                category: InsightCategory::Architecture,
                title: "Low API Implementation".to_string(),
                description: format!("Only {:.1}% of API endpoints are implemented, which may indicate architectural issues.", result.api_endpoints.implementation_percentage),
                priority: InsightPriority::High,
                recommendations: vec![
                    "Review the API design to ensure it meets the project requirements.".to_string(),
                    "Prioritize implementing core API endpoints before building features that depend on them.".to_string(),
                    "Consider using API-first development approach.".to_string(),
                ],
                related_files: Vec::new(),
            });
        }
        
        Ok(())
    }
    
    /// Generate performance insights
    fn generate_performance_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // This is a placeholder for performance insights
        // In a real implementation, we would analyze performance metrics
        
        Ok(())
    }
    
    /// Generate security insights
    fn generate_security_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // Check for security-related technical debt
        let security_issues = result.tech_debt_metrics.items.iter()
            .filter(|item| item.category.to_lowercase().contains("security"))
            .collect::<Vec<_>>();
        
        if !security_issues.is_empty() {
            insights.push(AiInsight {
                category: InsightCategory::Security,
                title: "Security Issues".to_string(),
                description: format!("Found {} security-related issues in the codebase.", security_issues.len()),
                priority: InsightPriority::Critical,
                recommendations: vec![
                    "Address all security-related issues immediately.".to_string(),
                    "Conduct a security audit of the codebase.".to_string(),
                    "Implement security best practices and guidelines.".to_string(),
                ],
                related_files: security_issues.iter().map(|item| item.file.clone()).collect(),
            });
        }
        
        Ok(())
    }
    
    /// Generate testing insights
    fn generate_testing_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // This is a placeholder for testing insights
        // In a real implementation, we would analyze test coverage and other testing metrics
        
        Ok(())
    }
    
    /// Generate documentation insights
    fn generate_documentation_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // Check for documentation-related technical debt
        let doc_issues = result.tech_debt_metrics.items.iter()
            .filter(|item| item.category.to_lowercase().contains("doc") || item.description.to_lowercase().contains("document"))
            .collect::<Vec<_>>();
        
        if !doc_issues.is_empty() {
            insights.push(AiInsight {
                category: InsightCategory::Documentation,
                title: "Documentation Issues".to_string(),
                description: format!("Found {} documentation-related issues in the codebase.", doc_issues.len()),
                priority: InsightPriority::Medium,
                recommendations: vec![
                    "Address documentation issues to improve code maintainability.".to_string(),
                    "Consider using documentation generation tools.".to_string(),
                    "Establish documentation standards and guidelines.".to_string(),
                ],
                related_files: doc_issues.iter().map(|item| item.file.clone()).collect(),
            });
        }
        
        Ok(())
    }
    
    /// Generate dependency insights
    fn generate_dependency_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // This is a placeholder for dependency insights
        // In a real implementation, we would analyze dependencies
        
        Ok(())
    }
    
    /// Generate project progress insights
    fn generate_project_progress_insights(&self, result: &AnalysisResult, insights: &mut Vec<AiInsight>) -> Result<(), String> {
        // Check for overall progress
        if result.overall_progress < 50.0 {
            insights.push(AiInsight {
                category: InsightCategory::ProjectProgress,
                title: "Low Overall Progress".to_string(),
                description: format!("The overall project progress is {:.1}%, which is below the expected threshold of 50%.", result.overall_progress),
                priority: InsightPriority::High,
                recommendations: vec![
                    "Review project priorities and focus on high-impact features.".to_string(),
                    "Consider reducing project scope or extending the timeline.".to_string(),
                    "Allocate more resources to critical features.".to_string(),
                ],
                related_files: Vec::new(),
            });
        }
        
        // Check for feature area progress
        let low_progress_areas = result.feature_areas.iter()
            .filter(|(_, metrics)| metrics.implementation_percentage < 30.0)
            .collect::<Vec<_>>();
        
        if !low_progress_areas.is_empty() {
            let area_names = low_progress_areas.iter()
                .map(|(area, _)| area.clone())
                .collect::<Vec<_>>()
                .join(", ");
            
            insights.push(AiInsight {
                category: InsightCategory::ProjectProgress,
                title: "Low Progress in Feature Areas".to_string(),
                description: format!("The following feature areas have low implementation progress (<30%): {}.", area_names),
                priority: InsightPriority::Medium,
                recommendations: vec![
                    "Prioritize development in low-progress feature areas.".to_string(),
                    "Review requirements for these areas to ensure they are clear and achievable.".to_string(),
                    "Consider allocating more resources to these areas.".to_string(),
                ],
                related_files: Vec::new(),
            });
        }
        
        Ok(())
    }
    
    /// Generate an AI insights report
    pub fn generate_report(&self, insights: &AiInsights) -> Result<String, String> {
        let mut report = String::new();
        
        // Header
        report.push_str("# AI Insights Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("**Total Insights: {}**\n\n", insights.insights.len()));
        
        // Insights by priority
        report.push_str("| Priority | Count |\n");
        report.push_str("|----------|-------|\n");
        
        let critical_count = insights.insights_by_priority.get(&InsightPriority::Critical).map_or(0, |v| v.len());
        let high_count = insights.insights_by_priority.get(&InsightPriority::High).map_or(0, |v| v.len());
        let medium_count = insights.insights_by_priority.get(&InsightPriority::Medium).map_or(0, |v| v.len());
        let low_count = insights.insights_by_priority.get(&InsightPriority::Low).map_or(0, |v| v.len());
        
        report.push_str(&format!("| Critical | {} |\n", critical_count));
        report.push_str(&format!("| High | {} |\n", high_count));
        report.push_str(&format!("| Medium | {} |\n", medium_count));
        report.push_str(&format!("| Low | {} |\n\n", low_count));
        
        // Insights by category
        report.push_str("| Category | Count |\n");
        report.push_str("|----------|-------|\n");
        
        for (category, category_insights) in &insights.insights_by_category {
            let category_name = match category {
                InsightCategory::TechnicalDebt => "Technical Debt",
                InsightCategory::CodeQuality => "Code Quality",
                InsightCategory::Architecture => "Architecture",
                InsightCategory::Performance => "Performance",
                InsightCategory::Security => "Security",
                InsightCategory::Testing => "Testing",
                InsightCategory::Documentation => "Documentation",
                InsightCategory::Dependencies => "Dependencies",
                InsightCategory::ProjectProgress => "Project Progress",
            };
            
            report.push_str(&format!("| {} | {} |\n", category_name, category_insights.len()));
        }
        
        report.push_str("\n");
        
        // Top Insights
        report.push_str("## Top Insights\n\n");
        
        for insight in &insights.top_insights {
            let priority_str = match insight.priority {
                InsightPriority::Critical => "⚠️ Critical",
                InsightPriority::High => "🔴 High",
                InsightPriority::Medium => "🟠 Medium",
                InsightPriority::Low => "🟢 Low",
            };
            
            let category_str = match insight.category {
                InsightCategory::TechnicalDebt => "Technical Debt",
                InsightCategory::CodeQuality => "Code Quality",
                InsightCategory::Architecture => "Architecture",
                InsightCategory::Performance => "Performance",
                InsightCategory::Security => "Security",
                InsightCategory::Testing => "Testing",
                InsightCategory::Documentation => "Documentation",
                InsightCategory::Dependencies => "Dependencies",
                InsightCategory::ProjectProgress => "Project Progress",
            };
            
            report.push_str(&format!("### {} - {}\n\n", priority_str, insight.title));
            report.push_str(&format!("**Category:** {}\n\n", category_str));
            report.push_str(&format!("{}\n\n", insight.description));
            
            report.push_str("**Recommendations:**\n\n");
            for recommendation in &insight.recommendations {
                report.push_str(&format!("- {}\n", recommendation));
            }
            report.push_str("\n");
            
            if !insight.related_files.is_empty() {
                report.push_str("**Related Files:**\n\n");
                for file in &insight.related_files {
                    report.push_str(&format!("- {}\n", file));
                }
                report.push_str("\n");
            }
        }
        
        // Insights by Category
        report.push_str("## Insights by Category\n\n");
        
        for (category, category_insights) in &insights.insights_by_category {
            let category_name = match category {
                InsightCategory::TechnicalDebt => "Technical Debt",
                InsightCategory::CodeQuality => "Code Quality",
                InsightCategory::Architecture => "Architecture",
                InsightCategory::Performance => "Performance",
                InsightCategory::Security => "Security",
                InsightCategory::Testing => "Testing",
                InsightCategory::Documentation => "Documentation",
                InsightCategory::Dependencies => "Dependencies",
                InsightCategory::ProjectProgress => "Project Progress",
            };
            
            report.push_str(&format!("### {}\n\n", category_name));
            
            for insight in category_insights {
                let priority_str = match insight.priority {
                    InsightPriority::Critical => "⚠️ Critical",
                    InsightPriority::High => "🔴 High",
                    InsightPriority::Medium => "🟠 Medium",
                    InsightPriority::Low => "🟢 Low",
                };
                
                report.push_str(&format!("#### {} - {}\n\n", priority_str, insight.title));
                report.push_str(&format!("{}\n\n", insight.description));
                
                report.push_str("**Recommendations:**\n\n");
                for recommendation in &insight.recommendations {
                    report.push_str(&format!("- {}\n", recommendation));
                }
                report.push_str("\n");
                
                if !insight.related_files.is_empty() {
                    report.push_str("**Related Files:**\n\n");
                    for file in &insight.related_files {
                        report.push_str(&format!("- {}\n", file));
                    }
                    report.push_str("\n");
                }
            }
        }
        
        Ok(report)
    }
}
