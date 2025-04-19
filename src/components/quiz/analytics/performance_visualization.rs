use leptos::*;
use crate::models::quiz::{Quiz, Question, Answer};
use std::rc::Rc;
use std::collections::HashMap;

/// Props for the PerformanceChart component
#[derive(Props, Clone)]
pub struct PerformanceChartProps {
    /// Chart title
    pub title: String,

    /// Chart type
    pub chart_type: ChartType,

    /// Chart data
    pub data: ChartData,

    /// Chart options
    #[prop(default = None)]
    pub options: Option<ChartOptions>,

    /// Chart height
    #[prop(default = 300)]
    pub height: u32,

    /// Chart width
    #[prop(default = 500)]
    pub width: u32,

    /// CSS class
    #[prop(default = "".to_string())]
    pub class: String,
}

/// Chart type
#[derive(Clone, PartialEq)]
pub enum ChartType {
    /// Bar chart
    Bar,

    /// Line chart
    Line,

    /// Pie chart
    Pie,

    /// Radar chart
    Radar,

    /// Scatter chart
    Scatter,
}

/// Chart data
#[derive(Clone)]
pub struct ChartData {
    /// Labels
    pub labels: Vec<String>,

    /// Datasets
    pub datasets: Vec<ChartDataset>,
}

/// Chart dataset
#[derive(Clone)]
pub struct ChartDataset {
    /// Dataset label
    pub label: String,

    /// Dataset data
    pub data: Vec<f32>,

    /// Dataset background color
    pub background_color: Option<Vec<String>>,

    /// Dataset border color
    pub border_color: Option<String>,

    /// Dataset fill
    pub fill: Option<bool>,
}

/// Chart options
#[derive(Clone)]
pub struct ChartOptions {
    /// Responsive
    pub responsive: bool,

    /// Maintain aspect ratio
    pub maintain_aspect_ratio: bool,

    /// Legend
    pub legend: Option<LegendOptions>,

    /// Scales
    pub scales: Option<ScalesOptions>,
}

/// Legend options
#[derive(Clone)]
pub struct LegendOptions {
    /// Display
    pub display: bool,

    /// Position
    pub position: String,
}

/// Scales options
#[derive(Clone)]
pub struct ScalesOptions {
    /// Y axes
    pub y_axes: Vec<AxisOptions>,

    /// X axes
    pub x_axes: Vec<AxisOptions>,
}

/// Axis options
#[derive(Clone)]
pub struct AxisOptions {
    /// Axis ID
    pub id: String,

    /// Axis type
    pub axis_type: String,

    /// Axis position
    pub position: String,

    /// Axis title
    pub title: Option<String>,

    /// Begin at zero
    pub begin_at_zero: bool,
}

/// A component that displays a performance chart
#[component]
pub fn PerformanceChart(props: PerformanceChartProps) -> impl IntoView {
    let PerformanceChartProps {
        title,
        chart_type,
        data,
        options,
        height,
        width,
        class,
    } = props;

    // Create a unique ID for the chart
    let chart_id = format!("chart-{}", web_sys::window()
        .unwrap()
        .crypto()
        .unwrap()
        .random_uuid()
        .unwrap());

    // Initialize the chart when the component mounts
    create_effect(move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(canvas) = document.get_element_by_id(&chart_id) {
            // Use Charming to create the chart
            let window = web_sys::window().unwrap();

            // Check if Charming is available
            if let Ok(charming) = js_sys::Reflect::get(
                &window,
                &wasm_bindgen::JsValue::from_str("Charming"),
            ) {
                // Prepare data for Charming
                let chart_type_str = match chart_type {
                    ChartType::Bar => "bar",
                    ChartType::Line => "line",
                    ChartType::Pie => "pie",
                    ChartType::Radar => "radar",
                    ChartType::Scatter => "scatter",
                };

                // Create dataset objects
                let datasets_array = js_sys::Array::new();
                for dataset in &data.datasets {
                    let dataset_obj = js_sys::Object::new();

                    // Set label
                    js_sys::Reflect::set(
                        &dataset_obj,
                        &wasm_bindgen::JsValue::from_str("label"),
                        &wasm_bindgen::JsValue::from_str(&dataset.label),
                    ).unwrap();

                    // Set data
                    let data_array = js_sys::Array::new();
                    for value in &dataset.data {
                        data_array.push(&wasm_bindgen::JsValue::from_f64(*value as f64));
                    }
                    js_sys::Reflect::set(
                        &dataset_obj,
                        &wasm_bindgen::JsValue::from_str("data"),
                        &data_array,
                    ).unwrap();

                    // Set colors
                    if let Some(colors) = &dataset.background_color {
                        let colors_array = js_sys::Array::new();
                        for color in colors {
                            colors_array.push(&wasm_bindgen::JsValue::from_str(color));
                        }
                        js_sys::Reflect::set(
                            &dataset_obj,
                            &wasm_bindgen::JsValue::from_str("backgroundColor"),
                            &colors_array,
                        ).unwrap();
                    }

                    if let Some(color) = &dataset.border_color {
                        js_sys::Reflect::set(
                            &dataset_obj,
                            &wasm_bindgen::JsValue::from_str("borderColor"),
                            &wasm_bindgen::JsValue::from_str(color),
                        ).unwrap();
                    }

                    if let Some(fill) = &dataset.fill {
                        js_sys::Reflect::set(
                            &dataset_obj,
                            &wasm_bindgen::JsValue::from_str("fill"),
                            &wasm_bindgen::JsValue::from_bool(*fill),
                        ).unwrap();
                    }

                    datasets_array.push(&dataset_obj);
                }

                // Create labels array
                let labels_array = js_sys::Array::new();
                for label in &data.labels {
                    labels_array.push(&wasm_bindgen::JsValue::from_str(label));
                }

                // Create config object
                let config = js_sys::Object::new();

                // Set type
                js_sys::Reflect::set(
                    &config,
                    &wasm_bindgen::JsValue::from_str("type"),
                    &wasm_bindgen::JsValue::from_str(chart_type_str),
                ).unwrap();

                // Set data
                let data_obj = js_sys::Object::new();
                js_sys::Reflect::set(
                    &data_obj,
                    &wasm_bindgen::JsValue::from_str("labels"),
                    &labels_array,
                ).unwrap();
                js_sys::Reflect::set(
                    &data_obj,
                    &wasm_bindgen::JsValue::from_str("datasets"),
                    &datasets_array,
                ).unwrap();

                js_sys::Reflect::set(
                    &config,
                    &wasm_bindgen::JsValue::from_str("data"),
                    &data_obj,
                ).unwrap();

                // Set options
                if let Some(opts) = &options {
                    let options_obj = js_sys::Object::new();

                    // Set responsive
                    js_sys::Reflect::set(
                        &options_obj,
                        &wasm_bindgen::JsValue::from_str("responsive"),
                        &wasm_bindgen::JsValue::from_bool(opts.responsive),
                    ).unwrap();

                    // Set maintain aspect ratio
                    js_sys::Reflect::set(
                        &options_obj,
                        &wasm_bindgen::JsValue::from_str("maintainAspectRatio"),
                        &wasm_bindgen::JsValue::from_bool(opts.maintain_aspect_ratio),
                    ).unwrap();

                    // Set legend
                    if let Some(legend) = &opts.legend {
                        let legend_obj = js_sys::Object::new();

                        js_sys::Reflect::set(
                            &legend_obj,
                            &wasm_bindgen::JsValue::from_str("display"),
                            &wasm_bindgen::JsValue::from_bool(legend.display),
                        ).unwrap();

                        js_sys::Reflect::set(
                            &legend_obj,
                            &wasm_bindgen::JsValue::from_str("position"),
                            &wasm_bindgen::JsValue::from_str(&legend.position),
                        ).unwrap();

                        js_sys::Reflect::set(
                            &options_obj,
                            &wasm_bindgen::JsValue::from_str("plugins"),
                            &js_sys::Object::new(),
                        ).unwrap();

                        let plugins_obj = js_sys::Reflect::get(
                            &options_obj,
                            &wasm_bindgen::JsValue::from_str("plugins"),
                        ).unwrap();

                        js_sys::Reflect::set(
                            &plugins_obj,
                            &wasm_bindgen::JsValue::from_str("legend"),
                            &legend_obj,
                        ).unwrap();
                    }

                    // Set scales
                    if let Some(scales) = &opts.scales {
                        let scales_obj = js_sys::Object::new();

                        // Set y axes
                        if !scales.y_axes.is_empty() {
                            let y_obj = js_sys::Object::new();

                            // Use the first y-axis for simplicity
                            let axis = &scales.y_axes[0];

                            if let Some(title) = &axis.title {
                                let title_obj = js_sys::Object::new();

                                js_sys::Reflect::set(
                                    &title_obj,
                                    &wasm_bindgen::JsValue::from_str("display"),
                                    &wasm_bindgen::JsValue::from_bool(true),
                                ).unwrap();

                                js_sys::Reflect::set(
                                    &title_obj,
                                    &wasm_bindgen::JsValue::from_str("text"),
                                    &wasm_bindgen::JsValue::from_str(title),
                                ).unwrap();

                                js_sys::Reflect::set(
                                    &y_obj,
                                    &wasm_bindgen::JsValue::from_str("title"),
                                    &title_obj,
                                ).unwrap();
                            }

                            let ticks_obj = js_sys::Object::new();

                            js_sys::Reflect::set(
                                &ticks_obj,
                                &wasm_bindgen::JsValue::from_str("beginAtZero"),
                                &wasm_bindgen::JsValue::from_bool(axis.begin_at_zero),
                            ).unwrap();

                            js_sys::Reflect::set(
                                &y_obj,
                                &wasm_bindgen::JsValue::from_str("ticks"),
                                &ticks_obj,
                            ).unwrap();

                            js_sys::Reflect::set(
                                &scales_obj,
                                &wasm_bindgen::JsValue::from_str("y"),
                                &y_obj,
                            ).unwrap();
                        }

                        // Set x axes
                        if !scales.x_axes.is_empty() {
                            let x_obj = js_sys::Object::new();

                            // Use the first x-axis for simplicity
                            let axis = &scales.x_axes[0];

                            if let Some(title) = &axis.title {
                                let title_obj = js_sys::Object::new();

                                js_sys::Reflect::set(
                                    &title_obj,
                                    &wasm_bindgen::JsValue::from_str("display"),
                                    &wasm_bindgen::JsValue::from_bool(true),
                                ).unwrap();

                                js_sys::Reflect::set(
                                    &title_obj,
                                    &wasm_bindgen::JsValue::from_str("text"),
                                    &wasm_bindgen::JsValue::from_str(title),
                                ).unwrap();

                                js_sys::Reflect::set(
                                    &x_obj,
                                    &wasm_bindgen::JsValue::from_str("title"),
                                    &title_obj,
                                ).unwrap();
                            }

                            js_sys::Reflect::set(
                                &scales_obj,
                                &wasm_bindgen::JsValue::from_str("x"),
                                &x_obj,
                            ).unwrap();
                        }

                        js_sys::Reflect::set(
                            &options_obj,
                            &wasm_bindgen::JsValue::from_str("scales"),
                            &scales_obj,
                        ).unwrap();
                    }

                    js_sys::Reflect::set(
                        &config,
                        &wasm_bindgen::JsValue::from_str("options"),
                        &options_obj,
                    ).unwrap();
                }

                // Create the chart using Charming
                let create_fn = js_sys::Reflect::get(
                    &charming,
                    &wasm_bindgen::JsValue::from_str("create"),
                ).unwrap();

                let args = js_sys::Array::new();
                args.push(&canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap());
                args.push(&config);

                let _ = js_sys::Reflect::apply(
                    &create_fn.dyn_into::<js_sys::Function>().unwrap(),
                    &charming,
                    &args,
                ).unwrap();
            } else {
                // Charming not available, show error message
                let ctx = canvas
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap()
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                ctx.set_font("14px Arial");
                ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("red"));
                ctx.fill_text("Charming library not loaded", 10.0, 30.0).unwrap();
            }
        }
    });

    view! {
        <div class=format!("performance-chart {}", class)>
            <h3 class="chart-title">{title}</h3>
            <canvas id=chart_id width=width.to_string() height=height.to_string()></canvas>
        </div>
    }
}

/// Props for the LearningInsightsDashboard component
#[derive(Props, Clone)]
pub struct LearningInsightsDashboardProps {
    /// User ID
    pub user_id: String,

    /// User name
    pub user_name: String,

    /// Performance data
    pub performance_data: UserPerformanceData,

    /// CSS class
    #[prop(default = "".to_string())]
    pub class: String,
}

/// User performance data
#[derive(Clone)]
pub struct UserPerformanceData {
    /// Quiz count
    pub quiz_count: usize,

    /// Completed count
    pub completed_count: usize,

    /// Average score
    pub average_score: f32,

    /// Total time spent (in seconds)
    pub total_time_spent: i32,

    /// Quiz performance
    pub quiz_performance: Vec<QuizPerformance>,

    /// Strengths
    pub strengths: Vec<String>,

    /// Weaknesses
    pub weaknesses: Vec<String>,

    /// Improvement suggestions
    pub improvement_suggestions: Vec<String>,

    /// Score over time
    pub score_over_time: Vec<(String, f32)>,

    /// Time spent distribution
    pub time_spent_distribution: HashMap<String, i32>,
}

/// Quiz performance
#[derive(Clone)]
pub struct QuizPerformance {
    /// Quiz ID
    pub quiz_id: String,

    /// Quiz title
    pub quiz_title: String,

    /// Score
    pub score: f32,

    /// Time spent
    pub time_spent: i32,

    /// Completed
    pub completed: bool,

    /// Date
    pub date: String,
}

/// A component that displays a learning insights dashboard
#[component]
pub fn LearningInsightsDashboard(props: LearningInsightsDashboardProps) -> impl IntoView {
    let LearningInsightsDashboardProps {
        user_id,
        user_name,
        performance_data,
        class,
    } = props;

    // Create score over time chart data
    let score_chart_data = ChartData {
        labels: performance_data.score_over_time.iter().map(|(date, _)| date.clone()).collect(),
        datasets: vec![
            ChartDataset {
                label: "Score".to_string(),
                data: performance_data.score_over_time.iter().map(|(_, score)| *score).collect(),
                background_color: Some(vec!["rgba(75, 192, 192, 0.2)".to_string()]),
                border_color: Some("rgba(75, 192, 192, 1)".to_string()),
                fill: Some(true),
            },
        ],
    };

    // Create score over time chart options
    let score_chart_options = ChartOptions {
        responsive: true,
        maintain_aspect_ratio: true,
        legend: Some(LegendOptions {
            display: true,
            position: "top".to_string(),
        }),
        scales: Some(ScalesOptions {
            y_axes: vec![
                AxisOptions {
                    id: "y-axis-0".to_string(),
                    axis_type: "linear".to_string(),
                    position: "left".to_string(),
                    title: Some("Score".to_string()),
                    begin_at_zero: true,
                },
            ],
            x_axes: vec![
                AxisOptions {
                    id: "x-axis-0".to_string(),
                    axis_type: "category".to_string(),
                    position: "bottom".to_string(),
                    title: Some("Date".to_string()),
                    begin_at_zero: true,
                },
            ],
        }),
    };

    // Create time spent distribution chart data
    let time_labels: Vec<String> = performance_data.time_spent_distribution.keys().cloned().collect();
    let time_data: Vec<f32> = performance_data.time_spent_distribution.values().map(|v| *v as f32).collect();

    let time_chart_data = ChartData {
        labels: time_labels,
        datasets: vec![
            ChartDataset {
                label: "Time Spent".to_string(),
                data: time_data,
                background_color: Some(vec![
                    "rgba(255, 99, 132, 0.2)".to_string(),
                    "rgba(54, 162, 235, 0.2)".to_string(),
                    "rgba(255, 206, 86, 0.2)".to_string(),
                    "rgba(75, 192, 192, 0.2)".to_string(),
                    "rgba(153, 102, 255, 0.2)".to_string(),
                ]),
                border_color: None,
                fill: None,
            },
        ],
    };

    view! {
        <div class=format!("learning-insights-dashboard {}", class)>
            <h2 class="dashboard-title">"Learning Insights for "{user_name}</h2>

            <div class="dashboard-summary">
                <div class="summary-item">
                    <div class="summary-value">{performance_data.quiz_count}</div>
                    <div class="summary-label">"Quizzes Attempted"</div>
                </div>

                <div class="summary-item">
                    <div class="summary-value">{performance_data.completed_count}</div>
                    <div class="summary-label">"Quizzes Completed"</div>
                </div>

                <div class="summary-item">
                    <div class="summary-value">{format!("{:.1}%", performance_data.average_score * 100.0)}</div>
                    <div class="summary-label">"Average Score"</div>
                </div>

                <div class="summary-item">
                    <div class="summary-value">{format_time(performance_data.total_time_spent)}</div>
                    <div class="summary-label">"Total Time Spent"</div>
                </div>
            </div>

            <div class="dashboard-charts">
                <div class="chart-container">
                    <PerformanceChart
                        title="Score Over Time".to_string()
                        chart_type=ChartType::Line
                        data=score_chart_data
                        options=Some(score_chart_options)
                        height=300
                        width=500
                        class="score-chart".to_string()
                    />
                </div>

                <div class="chart-container">
                    <PerformanceChart
                        title="Time Spent Distribution".to_string()
                        chart_type=ChartType::Pie
                        data=time_chart_data
                        options=None
                        height=300
                        width=500
                        class="time-chart".to_string()
                    />
                </div>
            </div>

            <div class="dashboard-insights">
                <div class="insights-section strengths">
                    <h3>"Strengths"</h3>
                    <ul class="insights-list">
                        {performance_data.strengths.iter().map(|strength| {
                            view! { <li>{strength}</li> }
                        }).collect_view()}
                    </ul>
                </div>

                <div class="insights-section weaknesses">
                    <h3>"Areas for Improvement"</h3>
                    <ul class="insights-list">
                        {performance_data.weaknesses.iter().map(|weakness| {
                            view! { <li>{weakness}</li> }
                        }).collect_view()}
                    </ul>
                </div>

                <div class="insights-section suggestions">
                    <h3>"Suggestions"</h3>
                    <ul class="insights-list">
                        {performance_data.improvement_suggestions.iter().map(|suggestion| {
                            view! { <li>{suggestion}</li> }
                        }).collect_view()}
                    </ul>
                </div>
            </div>

            <div class="dashboard-recent-quizzes">
                <h3>"Recent Quizzes"</h3>
                <table class="quizzes-table">
                    <thead>
                        <tr>
                            <th>"Quiz"</th>
                            <th>"Score"</th>
                            <th>"Time Spent"</th>
                            <th>"Date"</th>
                            <th>"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {performance_data.quiz_performance.iter().take(5).map(|quiz| {
                            view! {
                                <tr>
                                    <td>{quiz.quiz_title.clone()}</td>
                                    <td>{format!("{:.1}%", quiz.score * 100.0)}</td>
                                    <td>{format_time(quiz.time_spent)}</td>
                                    <td>{quiz.date.clone()}</td>
                                    <td>{if quiz.completed { "Completed" } else { "In Progress" }}</td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
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
