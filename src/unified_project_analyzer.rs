mod ast_analyzer;
mod file_system_utils;
mod analysis_utils;
mod report_generator;

use ast_analyzer::AstAnalyzer;
use file_system_utils::FileSystemUtils;
use analysis_utils::AnalysisUtils;
use report_generator::ReportGenerator;

struct UnifiedProjectAnalyzer {
    ast_analyzer: AstAnalyzer,
    fs_utils: FileSystemUtils,
    analysis_utils: AnalysisUtils,
    report_generator: ReportGenerator,
}

impl UnifiedProjectAnalyzer {
    fn new() -> Self {
        UnifiedProjectAnalyzer {
            ast_analyzer: AstAnalyzer::new(),
            fs_utils: FileSystemUtils::new(),
            analysis_utils: AnalysisUtils::new(),
            report_generator: ReportGenerator::new(),
        }
    }

    fn perform_analysis(&self) {
        println!("Starting unified project analysis...");
        self.ast_analyzer.analyze();
        self.fs_utils.perform_file_operations();
        self.analysis_utils.generate_insights();
        self.report_generator.create_report();
    }
}

fn main() {
    let analyzer = UnifiedProjectAnalyzer::new();
    analyzer.perform_analysis();
}
