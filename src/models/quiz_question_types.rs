use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Drag and Drop Question Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragDropContent {
    pub background_image_url: Option<String>,
    pub drag_items: Vec<DragItem>,
    pub drop_zones: Vec<DropZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragItem {
    pub id: String,
    pub text: String,
    pub rich_text: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropZone {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label: Option<String>,
}

// Hotspot Question Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotspotContent {
    pub image_url: String,
    pub hotspots: Vec<Hotspot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label: Option<String>,
}

// Drawing Question Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingContent {
    pub background_image_url: Option<String>,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub tools: Vec<DrawingTool>,
    pub reference_drawing: Option<String>, // SVG or base64 encoded image
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DrawingTool {
    Pen,
    Brush,
    Eraser,
    Line,
    Rectangle,
    Circle,
    Text,
}

// Code Execution Question Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionContent {
    pub language: String,
    pub initial_code: String,
    pub test_cases: Vec<CodeTestCase>,
    pub allowed_imports: Option<Vec<String>>,
    pub time_limit_ms: Option<u32>,
    pub memory_limit_kb: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTestCase {
    pub id: String,
    pub input: String,
    pub expected_output: String,
    pub is_hidden: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeExecutionAnswer {
    pub code: String,
    pub language: String,
    pub execution_results: Vec<CodeExecutionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeExecutionResult {
    pub test_case_id: String,
    pub output: String,
    pub passed: bool,
    pub execution_time_ms: u32,
    pub memory_used_kb: Option<u32>,
    pub error: Option<String>,
}

// Math Equation Question Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathEquationContent {
    pub equation_type: MathEquationType,
    pub variables: Option<HashMap<String, Vec<f64>>>,
    pub precision: Option<u32>,
    pub display_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MathEquationType {
    Algebraic,
    Calculus,
    Geometric,
    Statistical,
    Custom,
}

// Timeline Question Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineContent {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub events: Vec<TimelineEvent>,
    pub allow_custom_events: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEvent {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub image_url: Option<String>,
}

// Diagram Labeling Question Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramLabelingContent {
    pub diagram_image_url: String,
    pub labels: Vec<DiagramLabel>,
    pub label_positions: Vec<DiagramLabelPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramLabel {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramLabelPosition {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
