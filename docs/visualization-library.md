# Visualization Library Documentation

## Overview

The Quiz Module uses **Charming** as its visualization library for rendering charts and graphs in the analytics components. Charming was chosen over alternatives like Chart.js for its better performance, smaller footprint, and better integration with Rust/WebAssembly applications.

## Integration

Charming is integrated into the application through WebAssembly bindings. The `PerformanceChart` component in `src/components/quiz/analytics/performance_visualization.rs` provides a Leptos wrapper around Charming's functionality.

## Supported Chart Types

The following chart types are supported:

- Bar charts
- Line charts
- Pie charts
- Radar charts
- Scatter plots

## Usage

To use a chart in your component:

```rust
use crate::components::quiz::PerformanceChart;
use crate::components::quiz::analytics::performance_visualization::{
    ChartType, ChartData, ChartDataset, ChartOptions
};

// Create chart data
let data = ChartData {
    labels: vec!["Jan", "Feb", "Mar", "Apr", "May"].into_iter().map(String::from).collect(),
    datasets: vec![
        ChartDataset {
            label: "Dataset 1".to_string(),
            data: vec![10.0, 20.0, 30.0, 25.0, 40.0],
            background_color: Some(vec!["rgba(75, 192, 192, 0.2)".to_string()]),
            border_color: Some("rgba(75, 192, 192, 1)".to_string()),
            fill: Some(true),
        }
    ],
};

// Create chart options
let options = ChartOptions {
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
                title: Some("Value".to_string()),
                begin_at_zero: true,
            },
        ],
        x_axes: vec![
            AxisOptions {
                id: "x-axis-0".to_string(),
                axis_type: "category".to_string(),
                position: "bottom".to_string(),
                title: Some("Month".to_string()),
                begin_at_zero: true,
            },
        ],
    }),
};

// Render the chart
view! {
    <PerformanceChart
        title="Monthly Data".to_string()
        chart_type=ChartType::Line
        data=data
        options=Some(options)
        height=300
        width=500
        class="my-chart".to_string()
    />
}
```

## Dependencies

To use Charming in your application, you need to:

1. Include the Charming library in your HTML:

```html
<script src="https://cdn.jsdelivr.net/npm/charming/dist/charming.min.js"></script>
```

2. Add the appropriate dependencies to your `Cargo.toml`:

```toml
[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["HtmlCanvasElement", "CanvasRenderingContext2d"] }
```

## Customization

Charming supports extensive customization through its options API. The `ChartOptions` struct in our wrapper provides access to the most commonly used options. For more advanced customization, you can extend the options structure as needed.

## Performance Considerations

- Charming is optimized for performance, but large datasets can still impact rendering speed
- Consider using aggregated data for large datasets
- For time-series data with many points, consider using decimation or aggregation techniques

## Accessibility

The charts include the following accessibility features:

- Proper ARIA attributes
- Keyboard navigation support
- Color contrast compliance
- Text alternatives for screen readers

## Troubleshooting

If charts are not rendering correctly:

1. Check that the Charming library is properly loaded
2. Verify that the canvas element has the correct dimensions
3. Check the browser console for any JavaScript errors
4. Ensure that the data format matches what Charming expects
