use std::path::PathBuf;
use unified_analyzer::analyzers::modules::haskell_analyzer::HaskellAnalyzer;

fn main() {
    println!("Testing Haskell Analyzer...");
    
    let base_dir = PathBuf::from("C:\\Users\\Tim\\Desktop\\LMS");
    let analyzer = HaskellAnalyzer::new();
    
    match analyzer.analyze(&base_dir) {
        Ok(result) => {
            println!("Analysis successful!");
            println!("Result: {}", result);
        },
        Err(e) => {
            println!("Analysis failed: {}", e);
        }
    }
}
