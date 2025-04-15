// Test utilities
mod test_utils;

// Unit tests
mod unit {
    // Config tests
    mod config_test;
    
    // Generator tests
    mod generators {
        mod enhanced_central_hub_generator_test;
        mod error_test;
    }
}

// Integration tests
mod integration {
    mod unified_analyzer_test;
}
