#![feature(test)]
extern crate test;

use test::{Bencher, black_box};
use std::sync::Arc;
use rayon::prelude::*;
use crate::haskell_runtime::HaskellRuntime;

// Import types from your project (adjust the module path as needed)
use lms::models::{CompletionRule, StudentData, Submission};

// Helper for test data
fn load_test_completion_rules() -> Vec<CompletionRule> {
    // In practice, load from test fixtures.
    // For demo, creating synthetic test data:
    let mut rules = Vec::new();
    
    // Sample rule: Complete 80% of assignments
    rules.push(CompletionRule::MinimumCompletionPercentage {
        percentage: 80.0,
        category: "assignment".into(),
    });
    
    // Sample rule: Score at least 70% on final exam
    rules.push(CompletionRule::MinimumScore {
        score: 70.0,
        assessment_id: "final-exam-101".into(),
    });
    
    // Complex rule with AND/OR conditions
    rules.push(CompletionRule::And(vec![
        CompletionRule::MinimumScore {
            score: 60.0,
            assessment_id: "midterm-1".into(),
        },
        CompletionRule::Or(vec![
            CompletionRule::MinimumScore {
                score: 80.0,
                assessment_id: "project".into(),
            },
            CompletionRule::And(vec![
                CompletionRule::MinimumScore {
                    score: 70.0, 
                    assessment_id: "midterm-2".into(),
                },
                CompletionRule::MinimumCompletionPercentage {
                    percentage: 90.0,
                    category: "quiz".into(),
                },
            ]),
        ]),
    ]));
    
    rules
}

fn load_test_student_data() -> Vec<StudentData> {
    // Generate synthetic student data.
    let mut students = Vec::with_capacity(100);
    
    for i in 0..100 {
        students.push(StudentData {
            id: format!("student-{}", i),
            submissions: generate_submissions_for_student(i),
            forum_posts: (i as usize % 20) + 5,
            last_login: chrono::Utc::now() - chrono::Duration::days(i as i64 % 7),
        });
    }
    
    students
}

fn generate_submissions_for_student(student_num: u32) -> Vec<Submission> {
    // Generate submissions based on student number for variety.
    let mut submissions = Vec::new();
    
    // Generate assignment submissions with varying scores.
    for i in 1..10 {
        let score = 65.0 + (student_num % 30) as f64 + (i % 5) as f64;
        submissions.push(Submission {
            id: format!("sub-{}-{}", student_num, i),
            assignment_id: format!("assignment-{}", i),
            score: score.min(100.0),
            submitted_at: chrono::Utc::now() - chrono::Duration::days(i as i64),
            graded: true,
        });
    }
    
    // Generate exam submissions.
    submissions.push(Submission {
        id: format!("final-{}", student_num),
        assignment_id: "final-exam-101".into(),
        score: 72.0 + (student_num % 25) as f64,
        submitted_at: chrono::Utc::now() - chrono::Duration::days(14),
        graded: true,
    });
    
    submissions.push(Submission {
        id: format!("midterm1-{}", student_num),
        assignment_id: "midterm-1".into(),
        score: 65.0 + (student_num % 30) as f64,
        submitted_at: chrono::Utc::now() - chrono::Duration::days(30),
        graded: true,
    });
    
    submissions.push(Submission {
        id: format!("midterm2-{}", student_num),
        assignment_id: "midterm-2".into(),
        score: 68.0 + (student_num % 27) as f64,
        submitted_at: chrono::Utc::now() - chrono::Duration::days(20),
        graded: true,
    });
    
    submissions.push(Submission {
        id: format!("project-{}", student_num),
        assignment_id: "project".into(),
        score: 75.0 + (student_num % 20) as f64,
        submitted_at: chrono::Utc::now() - chrono::Duration::days(10),
        graded: student_num % 5 != 0, // Some are ungraded.
    });
    
    // Generate quiz submissions.
    for i in 1..8 {
        if student_num % (i + 1) != 0 { // Skip some quizzes for some students.
            let score = 70.0 + (student_num % 25) as f64 + (i % 6) as f64;
            submissions.push(Submission {
                id: format!("quiz-{}-{}", student_num, i),
                assignment_id: format!("quiz-{}", i),
                score: score.min(100.0),
                submitted_at: chrono::Utc::now() - chrono::Duration::days(i as i64 * 5),
                graded: true,
            });
        }
    }
    
    submissions
}

#[bench]
fn simulate_course_completion_checks(b: &mut Bencher) {
    // Setup: Load realistic course completion rules.
    let rules = Arc::new(load_test_completion_rules());
    let student_data = load_test_student_data();
    
    // Initialize Haskell runtime with controlled memory.
    let runtime = HaskellRuntime::new_with_limits(
        256 * 1024 * 1024,  // 256MB heap.
        4 * 1024 * 1024     // 4MB stack.
    );
    
    b.iter(|| {
        // Simulate 100 students completing a course simultaneously.
        let results = (0..100).into_par_iter().map(|i| {
            let student = &student_data[i % student_data.len()];
            let rules_ref = rules.clone();
            black_box(runtime.validate_completion(rules_ref, student))
        }).collect::<Vec<_>>();
        
        // Prevent dead-code elimination.
        black_box(results)
    });
    
    // Report memory high-water mark.
    println!("Peak memory usage: {} MB", runtime.peak_memory_mb());
}