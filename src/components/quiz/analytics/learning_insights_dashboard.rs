use leptos::*;
use crate::models::quiz::{Quiz, Question, Answer};
use crate::components::quiz::analytics::performance_visualization::{
    LearningInsightsDashboard, UserPerformanceData, QuizPerformance,
    PerformanceChart, ChartType, ChartData, ChartDataset, ChartOptions
};
use std::rc::Rc;
use std::collections::HashMap;

/// Props for the LearningInsightsPage component
#[derive(Props, Clone)]
pub struct LearningInsightsPageProps {
    /// User ID
    pub user_id: String,

    /// User name
    #[prop(default = "User".to_string())]
    pub user_name: String,

    /// CSS class
    #[prop(default = "".to_string())]
    pub class: String,
}

/// A page that displays learning insights for a user
#[component]
pub fn LearningInsightsPage(props: LearningInsightsPageProps) -> impl IntoView {
    let LearningInsightsPageProps {
        user_id,
        user_name,
        class,
    } = props;

    // State for loading and error
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (performance_data, set_performance_data) = create_signal(None::<UserPerformanceData>);

    // Load performance data
    create_effect(move |_| {
        set_loading.set(true);
        set_error.set(None);

        // In a real implementation, this would fetch data from the server
        // For now, we'll use mock data
        let mock_data = UserPerformanceData {
            quiz_count: 12,
            completed_count: 10,
            average_score: 0.78,
            total_time_spent: 7200,
            quiz_performance: vec![
                QuizPerformance {
                    quiz_id: "1".to_string(),
                    quiz_title: "Introduction to Mathematics".to_string(),
                    score: 0.85,
                    time_spent: 1200,
                    completed: true,
                    date: "2023-05-15".to_string(),
                },
                QuizPerformance {
                    quiz_id: "2".to_string(),
                    quiz_title: "Basic Physics".to_string(),
                    score: 0.72,
                    time_spent: 900,
                    completed: true,
                    date: "2023-05-20".to_string(),
                },
                QuizPerformance {
                    quiz_id: "3".to_string(),
                    quiz_title: "Chemistry Fundamentals".to_string(),
                    score: 0.65,
                    time_spent: 1500,
                    completed: true,
                    date: "2023-05-25".to_string(),
                },
                QuizPerformance {
                    quiz_id: "4".to_string(),
                    quiz_title: "Biology Basics".to_string(),
                    score: 0.90,
                    time_spent: 1100,
                    completed: true,
                    date: "2023-06-01".to_string(),
                },
                QuizPerformance {
                    quiz_id: "5".to_string(),
                    quiz_title: "Advanced Mathematics".to_string(),
                    score: 0.68,
                    time_spent: 1800,
                    completed: true,
                    date: "2023-06-10".to_string(),
                },
                QuizPerformance {
                    quiz_id: "6".to_string(),
                    quiz_title: "Quantum Physics".to_string(),
                    score: 0.45,
                    time_spent: 2000,
                    completed: false,
                    date: "2023-06-15".to_string(),
                },
            ],
            strengths: vec![
                "Mathematics".to_string(),
                "Biology".to_string(),
                "Problem Solving".to_string(),
            ],
            weaknesses: vec![
                "Quantum Physics".to_string(),
                "Organic Chemistry".to_string(),
                "Thermodynamics".to_string(),
            ],
            improvement_suggestions: vec![
                "Focus on improving your understanding of Quantum Physics concepts.".to_string(),
                "Spend more time on Organic Chemistry problems.".to_string(),
                "Review Thermodynamics principles regularly.".to_string(),
                "Try to complete all quizzes you start.".to_string(),
            ],
            score_over_time: vec![
                ("2023-05-15".to_string(), 0.85),
                ("2023-05-20".to_string(), 0.72),
                ("2023-05-25".to_string(), 0.65),
                ("2023-06-01".to_string(), 0.90),
                ("2023-06-10".to_string(), 0.68),
                ("2023-06-15".to_string(), 0.45),
            ],
            time_spent_distribution: {
                let mut map = HashMap::new();
                map.insert("Morning".to_string(), 2500);
                map.insert("Afternoon".to_string(), 3200);
                map.insert("Evening".to_string(), 1500);
                map
            },
        };

        // Simulate network delay
        set_timeout(
            move || {
                set_performance_data.set(Some(mock_data));
                set_loading.set(false);
            },
            std::time::Duration::from_millis(1000),
        );
    });

    view! {
        <div class=format!("learning-insights-page {}", class)>
            {move || {
                if loading.get() {
                    view! {
                        <div class="loading-container">
                            <div class="loading-spinner"></div>
                            <p>"Loading learning insights..."</p>
                        </div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class="error-container">
                            <p class="error-message">{err}</p>
                            <button class="retry-button" on:click=move |_| {
                                // Reload data
                                create_effect(|_| {});
                            }>
                                "Retry"
                            </button>
                        </div>
                    }.into_view()
                } else if let Some(data) = performance_data.get() {
                    view! {
                        <LearningInsightsDashboard
                            user_id=user_id.clone()
                            user_name=user_name.clone()
                            performance_data=data
                        />
                    }.into_view()
                } else {
                    view! {
                        <div class="no-data-container">
                            <p>"No learning insights available."</p>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

/// Props for the QuizPerformanceAnalysisPage component
#[derive(Props, Clone)]
pub struct QuizPerformanceAnalysisPageProps {
    /// Quiz ID
    pub quiz_id: String,

    /// Quiz title
    #[prop(default = "Quiz".to_string())]
    pub quiz_title: String,

    /// CSS class
    #[prop(default = "".to_string())]
    pub class: String,
}

/// A page that displays performance analysis for a quiz
#[component]
pub fn QuizPerformanceAnalysisPage(props: QuizPerformanceAnalysisPageProps) -> impl IntoView {
    let QuizPerformanceAnalysisPageProps {
        quiz_id,
        quiz_title,
        class,
    } = props;

    // State for loading and error
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (quiz_data, set_quiz_data) = create_signal(None::<QuizAnalyticsData>);

    // Load quiz data
    create_effect(move |_| {
        set_loading.set(true);
        set_error.set(None);

        // In a real implementation, this would fetch data from the server
        // For now, we'll use mock data
        let mock_data = QuizAnalyticsData {
            quiz_id: quiz_id.clone(),
            quiz_title: quiz_title.clone(),
            attempt_count: 150,
            average_score: 0.72,
            median_score: 0.75,
            highest_score: 1.0,
            lowest_score: 0.25,
            completion_rate: 0.85,
            average_time_spent: 1200,
            score_distribution: {
                let mut map = HashMap::new();
                map.insert("0-19".to_string(), 5);
                map.insert("20-39".to_string(), 15);
                map.insert("40-59".to_string(), 30);
                map.insert("60-79".to_string(), 60);
                map.insert("80-100".to_string(), 40);
                map
            },
            question_performance: vec![
                QuestionAnalyticsData {
                    question_id: "1".to_string(),
                    question_text: "What is the capital of France?".to_string(),
                    correct_rate: 0.95,
                    difficulty_level: "Very Easy".to_string(),
                    answer_distribution: {
                        let mut map = HashMap::new();
                        map.insert("Paris".to_string(), 142);
                        map.insert("London".to_string(), 3);
                        map.insert("Berlin".to_string(), 2);
                        map.insert("Madrid".to_string(), 3);
                        map
                    },
                },
                QuestionAnalyticsData {
                    question_id: "2".to_string(),
                    question_text: "What is the chemical symbol for gold?".to_string(),
                    correct_rate: 0.85,
                    difficulty_level: "Easy".to_string(),
                    answer_distribution: {
                        let mut map = HashMap::new();
                        map.insert("Au".to_string(), 127);
                        map.insert("Ag".to_string(), 10);
                        map.insert("Fe".to_string(), 8);
                        map.insert("Cu".to_string(), 5);
                        map
                    },
                },
                QuestionAnalyticsData {
                    question_id: "3".to_string(),
                    question_text: "What is the formula for calculating the area of a circle?".to_string(),
                    correct_rate: 0.65,
                    difficulty_level: "Medium".to_string(),
                    answer_distribution: {
                        let mut map = HashMap::new();
                        map.insert("πr²".to_string(), 97);
                        map.insert("2πr".to_string(), 35);
                        map.insert("πd".to_string(), 10);
                        map.insert("r²".to_string(), 8);
                        map
                    },
                },
                QuestionAnalyticsData {
                    question_id: "4".to_string(),
                    question_text: "What is the Schrödinger equation used for?".to_string(),
                    correct_rate: 0.35,
                    difficulty_level: "Hard".to_string(),
                    answer_distribution: {
                        let mut map = HashMap::new();
                        map.insert("Quantum mechanics".to_string(), 52);
                        map.insert("Relativity".to_string(), 45);
                        map.insert("Thermodynamics".to_string(), 30);
                        map.insert("Classical mechanics".to_string(), 23);
                        map
                    },
                },
                QuestionAnalyticsData {
                    question_id: "5".to_string(),
                    question_text: "What is the half-life of Carbon-14?".to_string(),
                    correct_rate: 0.15,
                    difficulty_level: "Very Hard".to_string(),
                    answer_distribution: {
                        let mut map = HashMap::new();
                        map.insert("5,730 years".to_string(), 22);
                        map.insert("1,000 years".to_string(), 38);
                        map.insert("10,000 years".to_string(), 45);
                        map.insert("100,000 years".to_string(), 45);
                        map
                    },
                },
            ],
        };

        // Simulate network delay
        set_timeout(
            move || {
                set_quiz_data.set(Some(mock_data));
                set_loading.set(false);
            },
            std::time::Duration::from_millis(1000),
        );
    });

    view! {
        <div class=format!("quiz-performance-analysis-page {}", class)>
            {move || {
                if loading.get() {
                    view! {
                        <div class="loading-container">
                            <div class="loading-spinner"></div>
                            <p>"Loading quiz performance analysis..."</p>
                        </div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class="error-container">
                            <p class="error-message">{err}</p>
                            <button class="retry-button" on:click=move |_| {
                                // Reload data
                                create_effect(|_| {});
                            }>
                                "Retry"
                            </button>
                        </div>
                    }.into_view()
                } else if let Some(data) = quiz_data.get() {
                    view! {
                        <QuizPerformanceAnalysis
                            quiz_data=data
                        />
                    }.into_view()
                } else {
                    view! {
                        <div class="no-data-container">
                            <p>"No quiz performance analysis available."</p>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

/// Quiz analytics data
#[derive(Clone)]
pub struct QuizAnalyticsData {
    /// Quiz ID
    pub quiz_id: String,

    /// Quiz title
    pub quiz_title: String,

    /// Number of attempts
    pub attempt_count: usize,

    /// Average score
    pub average_score: f32,

    /// Median score
    pub median_score: f32,

    /// Highest score
    pub highest_score: f32,

    /// Lowest score
    pub lowest_score: f32,

    /// Completion rate
    pub completion_rate: f32,

    /// Average time spent
    pub average_time_spent: i32,

    /// Score distribution
    pub score_distribution: HashMap<String, usize>,

    /// Question performance
    pub question_performance: Vec<QuestionAnalyticsData>,
}

/// Question analytics data
#[derive(Clone)]
pub struct QuestionAnalyticsData {
    /// Question ID
    pub question_id: String,

    /// Question text
    pub question_text: String,

    /// Correct answer rate
    pub correct_rate: f32,

    /// Difficulty level
    pub difficulty_level: String,

    /// Answer distribution
    pub answer_distribution: HashMap<String, usize>,
}

/// Props for the QuizPerformanceAnalysis component
#[derive(Props, Clone)]
pub struct QuizPerformanceAnalysisProps {
    /// Quiz data
    pub quiz_data: QuizAnalyticsData,

    /// CSS class
    #[prop(default = "".to_string())]
    pub class: String,
}

/// A component that displays performance analysis for a quiz
#[component]
pub fn QuizPerformanceAnalysis(props: QuizPerformanceAnalysisProps) -> impl IntoView {
    let QuizPerformanceAnalysisProps {
        quiz_data,
        class,
    } = props;

    // Create score distribution chart data
    let score_labels: Vec<String> = quiz_data.score_distribution.keys().cloned().collect();
    let score_data: Vec<f32> = quiz_data.score_distribution.values().map(|v| *v as f32).collect();

    // Create question difficulty chart data
    let difficulty_labels = vec![
        "Very Easy".to_string(),
        "Easy".to_string(),
        "Medium".to_string(),
        "Hard".to_string(),
        "Very Hard".to_string(),
    ];

    let mut difficulty_counts = vec![0, 0, 0, 0, 0];

    for question in &quiz_data.question_performance {
        match question.difficulty_level.as_str() {
            "Very Easy" => difficulty_counts[0] += 1,
            "Easy" => difficulty_counts[1] += 1,
            "Medium" => difficulty_counts[2] += 1,
            "Hard" => difficulty_counts[3] += 1,
            "Very Hard" => difficulty_counts[4] += 1,
            _ => {},
        }
    }

    let difficulty_data: Vec<f32> = difficulty_counts.iter().map(|v| *v as f32).collect();

    view! {
        <div class=format!("quiz-performance-analysis {}", class)>
            <h2 class="analysis-title">"Performance Analysis: "{quiz_data.quiz_title}</h2>

            <div class="analysis-summary">
                <div class="summary-item">
                    <div class="summary-value">{quiz_data.attempt_count}</div>
                    <div class="summary-label">"Attempts"</div>
                </div>

                <div class="summary-item">
                    <div class="summary-value">{format!("{:.1}%", quiz_data.average_score * 100.0)}</div>
                    <div class="summary-label">"Average Score"</div>
                </div>

                <div class="summary-item">
                    <div class="summary-value">{format!("{:.1}%", quiz_data.median_score * 100.0)}</div>
                    <div class="summary-label">"Median Score"</div>
                </div>

                <div class="summary-item">
                    <div class="summary-value">{format!("{:.1}%", quiz_data.completion_rate * 100.0)}</div>
                    <div class="summary-label">"Completion Rate"</div>
                </div>

                <div class="summary-item">
                    <div class="summary-value">{format_time(quiz_data.average_time_spent)}</div>
                    <div class="summary-label">"Average Time"</div>
                </div>
            </div>

            <div class="analysis-charts">
                <div class="chart-container">
                    <h3>"Score Distribution"</h3>
                    <div class="score-distribution-chart">
                        <PerformanceChart
                            title="Score Distribution".to_string()
                            chart_type=ChartType::Bar
                            data=create_score_distribution_data(&quiz_data)
                            height=300
                            width=500
                            class="score-chart".to_string()
                        />
                    </div>
                </div>

                <div class="chart-container">
                    <h3>"Question Difficulty"</h3>
                    <div class="difficulty-chart">
                        <PerformanceChart
                            title="Question Difficulty".to_string()
                            chart_type=ChartType::Pie
                            data=create_difficulty_data(&quiz_data)
                            height=300
                            width=500
                            class="difficulty-chart".to_string()
                        />
                    </div>
                </div>
            </div>

            <div class="question-analysis">
                <h3>"Question Analysis"</h3>
                <table class="question-table">
                    <thead>
                        <tr>
                            <th>"Question"</th>
                            <th>"Correct Rate"</th>
                            <th>"Difficulty"</th>
                            <th>"Most Common Answer"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {quiz_data.question_performance.iter().map(|question| {
                            // Find most common answer
                            let most_common = question.answer_distribution.iter()
                                .max_by_key(|(_, count)| *count)
                                .map(|(answer, count)| (answer.clone(), *count))
                                .unwrap_or(("None".to_string(), 0));

                            view! {
                                <tr>
                                    <td class="question-text">{question.question_text.clone()}</td>
                                    <td class=format!("correct-rate {}", get_performance_class(question.correct_rate))>
                                        {format!("{:.1}%", question.correct_rate * 100.0)}
                                    </td>
                                    <td class=format!("difficulty {}", question.difficulty_level.to_lowercase().replace(" ", "-"))>
                                        {question.difficulty_level.clone()}
                                    </td>
                                    <td class="common-answer">
                                        {most_common.0}
                                        <span class="answer-count">{"("}{most_common.1}{" responses)"}</span>
                                    </td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
            </div>

            <div class="analysis-recommendations">
                <h3>"Recommendations"</h3>
                <ul class="recommendations-list">
                    {generate_recommendations(&quiz_data).iter().map(|rec| {
                        view! { <li>{rec}</li> }
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}

/// Format time in seconds to a human-readable string
fn format_time(seconds: i32) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}

/// Get CSS class for performance level
fn get_performance_class(rate: f32) -> String {
    match rate {
        r if r >= 0.9 => "excellent",
        r if r >= 0.7 => "good",
        r if r >= 0.5 => "average",
        r if r >= 0.3 => "poor",
        _ => "very-poor",
    }.to_string()
}

/// Generate recommendations based on quiz data
fn generate_recommendations(quiz_data: &QuizAnalyticsData) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Check average score
    if quiz_data.average_score < 0.6 {
        recommendations.push("The average score is below 60%. Consider reviewing the quiz content to ensure it matches the expected knowledge level.".to_string());
    }

    // Check completion rate
    if quiz_data.completion_rate < 0.7 {
        recommendations.push("The completion rate is below 70%. Consider checking if the quiz is too long or too difficult.".to_string());
    }

    // Check very hard questions
    let very_hard_count = quiz_data.question_performance.iter()
        .filter(|q| q.difficulty_level == "Very Hard")
        .count();

    if very_hard_count > quiz_data.question_performance.len() / 3 {
        recommendations.push("More than a third of the questions are very difficult. Consider adding more medium difficulty questions for better balance.".to_string());
    }

    // Check very easy questions
    let very_easy_count = quiz_data.question_performance.iter()
        .filter(|q| q.difficulty_level == "Very Easy")
        .count();

    if very_easy_count > quiz_data.question_performance.len() / 3 {
        recommendations.push("More than a third of the questions are very easy. Consider adding more challenging questions for better engagement.".to_string());
    }

    // Add general recommendations
    recommendations.push("Regularly review question performance and adjust difficulty as needed.".to_string());
    recommendations.push("Consider adding explanations for questions with low correct rates.".to_string());

    recommendations
}

/// Create score distribution data for the chart
fn create_score_distribution_data(quiz_data: &QuizAnalyticsData) -> ChartData {
    // Create score ranges
    let ranges = vec!["0-20%", "21-40%", "41-60%", "61-80%", "81-100%"];

    // Count scores in each range
    let mut counts = vec![0, 0, 0, 0, 0];

    for attempt in &quiz_data.attempts {
        let score = attempt.score;
        if score < 0.2 {
            counts[0] += 1;
        } else if score < 0.4 {
            counts[1] += 1;
        } else if score < 0.6 {
            counts[2] += 1;
        } else if score < 0.8 {
            counts[3] += 1;
        } else {
            counts[4] += 1;
        }
    }

    // Convert to float for the chart
    let data = counts.iter().map(|&c| c as f64).collect();

    // Create chart data
    ChartData {
        labels: ranges.iter().map(|&s| s.to_string()).collect(),
        datasets: vec![
            ChartDataset {
                label: "Number of Attempts".to_string(),
                data,
                background_color: Some(vec![
                    "rgba(255, 99, 132, 0.2)".to_string(),
                    "rgba(255, 159, 64, 0.2)".to_string(),
                    "rgba(255, 205, 86, 0.2)".to_string(),
                    "rgba(75, 192, 192, 0.2)".to_string(),
                    "rgba(54, 162, 235, 0.2)".to_string(),
                ]),
                border_color: Some("rgba(75, 192, 192, 1)".to_string()),
                fill: Some(true),
            }
        ],
    }
}

/// Create difficulty data for the pie chart
fn create_difficulty_data(quiz_data: &QuizAnalyticsData) -> ChartData {
    // Count questions by difficulty
    let mut easy_count = 0;
    let mut medium_count = 0;
    let mut hard_count = 0;

    for question in &quiz_data.question_performance {
        match question.difficulty_level.as_str() {
            "Easy" | "Very Easy" => easy_count += 1,
            "Medium" => medium_count += 1,
            "Hard" | "Very Hard" => hard_count += 1,
            _ => {}
        }
    }

    // Create chart data
    ChartData {
        labels: vec!["Easy".to_string(), "Medium".to_string(), "Hard".to_string()],
        datasets: vec![
            ChartDataset {
                label: "Question Difficulty".to_string(),
                data: vec![easy_count as f64, medium_count as f64, hard_count as f64],
                background_color: Some(vec![
                    "rgba(75, 192, 192, 0.8)".to_string(),
                    "rgba(255, 205, 86, 0.8)".to_string(),
                    "rgba(255, 99, 132, 0.8)".to_string(),
                ]),
                border_color: Some("#fff".to_string()),
                fill: Some(true),
            }
        ],
    }
}
