pub async fn generate_analysis_report() -> Result<String, Box<dyn std::error::Error>> {
    // ... existing code ...
    
    // Run blockchain analyzer with performance metrics
    let blockchain_analyzer = BlockchainAnalyzer::new(Arc::clone(&state.chain)).await?;
    let blockchain_results = blockchain_analyzer.analyze().await?;
    
    // Add blockchain section with performance focus to report
    report.push_str("\n## Blockchain Integration Status\n\n");
    report.push_str("| Metric | Value |\n");
    report.push_str("|--------|-------|\n");
    
    for (metric, value) in blockchain_results.metrics() {
        report.push_str(&format!("| {} | {} |\n", metric, value));
    }
    
    // Add performance-focused section
    report.push_str("\n### Blockchain Performance Metrics\n\n");
    report.push_str("| Optimization | Current Value | Target |\n");
    report.push_str("|--------------|--------------|--------|\n");
    report.push_str(&format!("| Batch Efficiency | {}% | >90% |\n", 
        blockchain_results.get_metric("batch_efficiency")));
    report.push_str(&format!("| Cache Hit Rate | {}% | >85% |\n", 
        blockchain_results.get_metric("cache_hit_rate")));
    report.push_str(&format!("| Consensus Time | {}ms | <30ms |\n", 
        blockchain_results.get_metric("avg_consensus_ms")));
    
    // Add a performance optimization chart (ASCII representation)
    report.push_str("\n### Performance Optimization Status\n\n");
    report.push_str("```\n");
    report.push_str("Optimization Score: [");
    
    let score = blockchain_results.get_metric("optimization_score")
        .parse::<f64>()
        .unwrap_or(0.0);
    let score_int = (score * 10.0) as usize / 10;
    
    for i in 0..10 {
        if i < score_int {
            report.push_str("█");
        } else {
            report.push_str("░");
        }
    }
    
    report.push_str("] ");
    report.push_str(&blockchain_results.get_metric("optimization_score"));
    report.push_str("/10.0\n```\n");
    
    // Add blockchain next steps
    report.push_str("\n### Performance Optimization Next Steps\n\n");
    for i in 0..blockchain_results.next_steps_count() {
        report.push_str(&format!("{}. {}\n", i + 1, blockchain_results.get_next_step(i)));
    }
    
    // ... rest of the report generation ...
    
    Ok(report)
}