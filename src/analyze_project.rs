use std::collections::HashMap;
use std::path::Path;

struct Metrics {
    models: HashMap<String, usize>,
    api_endpoints: HashMap<String, usize>,
    ui_components: HashMap<String, usize>,
    tests: HashMap<String, usize>,
    overall_phase: String,
}

impl Metrics {
    fn new() -> Self {
        Metrics {
            models: HashMap::from([
                ("total".to_string(), 28),
                ("implemented".to_string(), 0),
            ]),
            api_endpoints: HashMap::from([
                ("total".to_string(), 42),
                ("implemented".to_string(), 0),
            ]),
            ui_components: HashMap::from([
                ("total".to_string(), 35),
                ("implemented".to_string(), 0),
            ]),
            tests: HashMap::from([
                ("coverage".to_string(), 0),
                ("passing".to_string(), 0),
                ("total".to_string(), 0),
            ]),
            overall_phase: "planning".to_string(),
        }
    }

    fn display(&self) {
        println!("\u{1F50D} Starting project analysis...");
        println!("Metrics: {:?}", self);
    }
}

fn main() {
    let base_dir = Path::new(".");
    println!("Base directory: {:?}", base_dir);

    let metrics = Metrics::new();
    metrics.display();
}
