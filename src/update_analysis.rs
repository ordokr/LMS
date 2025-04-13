use std::collections::HashMap;

struct Metrics {
    implemented: Vec<String>,
    total: usize,
}

impl Metrics {
    fn new(implemented: Vec<String>, total: usize) -> Self {
        Metrics { implemented, total }
    }

    fn percentage(&self) -> usize {
        ((self.implemented.len() as f64 / self.total as f64) * 100.0).round() as usize
    }

    fn count(&self) -> usize {
        self.implemented.len()
    }
}

fn main() {
    let models = Metrics::new(
        vec!["User".to_string(), "Course".to_string(), "Category".to_string(), "Assignment".to_string()],
        28,
    );

    let apis = Metrics::new(
        vec!["login".to_string(), "logout".to_string(), "refreshToken".to_string()],
        42,
    );

    println!("Models implemented: {} / {} ({}%)", models.count(), models.total, models.percentage());
    println!("APIs implemented: {} / {} ({}%)", apis.count(), apis.total, apis.percentage());
}
