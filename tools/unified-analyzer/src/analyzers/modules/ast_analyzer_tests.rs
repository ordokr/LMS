#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::analyzers::modules::ast_analyzer::AstAnalyzer;

    #[test]
    fn test_analyze_file() {
        let analyzer = AstAnalyzer::new();
        let content = r#"
            struct User {
                name: String,
                age: u32,
            }

            impl User {
                fn new(name: String, age: u32) -> Self {
                    Self { name, age }
                }

                fn get_name(&self) -> &str {
                    &self.name
                }

                fn get_age(&self) -> u32 {
                    self.age
                }
            }

            fn main() {
                let user = User::new("John".to_string(), 30);
                println!("Name: {}, Age: {}", user.get_name(), user.get_age());
            }
        "#;

        let metrics = analyzer.analyze_file(Path::new("test.rs"), content);

        // Check that the metrics are calculated correctly
        assert!(metrics.functions >= 4); // main + 3 methods
        assert!(metrics.structs >= 1);   // User struct
        assert!(metrics.impls >= 1);     // User impl
        assert!(metrics.complexity > 0.0); // Some complexity value
    }

    #[test]
    fn test_count_functions() {
        let analyzer = AstAnalyzer::new();
        let content = r#"
            fn function1() {}
            fn function2(param: i32) -> i32 { param * 2 }
            fn function3<T>(param: T) -> T { param }
        "#;

        let count = analyzer.count_functions(content);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_structs() {
        let analyzer = AstAnalyzer::new();
        let content = r#"
            struct Struct1 {}
            struct Struct2<T> { field: T }
            struct Struct3(i32, String);
        "#;

        let count = analyzer.count_structs(content);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_impls() {
        let analyzer = AstAnalyzer::new();
        let content = r#"
            impl Struct1 {}
            impl<T> Struct2<T> {}
            impl Trait for Struct3 {}
        "#;

        let count = analyzer.count_impls(content);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_estimate_complexity() {
        let analyzer = AstAnalyzer::new();
        let simple_content = "fn simple() {}";
        let complex_content = r#"
            fn complex(param: i32) -> i32 {
                if param > 10 {
                    for i in 0..param {
                        if i % 2 == 0 {
                            println!("Even: {}", i);
                        } else {
                            println!("Odd: {}", i);
                        }
                    }
                    param * 2
                } else {
                    match param {
                        0 => 0,
                        1 => 1,
                        _ => param - 1
                    }
                }
            }
        "#;

        let simple_complexity = analyzer.estimate_complexity(simple_content);
        let complex_complexity = analyzer.estimate_complexity(complex_content);

        assert!(simple_complexity < complex_complexity);
    }
}
