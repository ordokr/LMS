# Quiz Module Frontend Enhancements

This document describes the frontend enhancements implemented for the Quiz Module, including advanced question types, rich text editor for questions, and media upload and management.

## 1. Advanced Question Types

The Quiz Module now includes support for more interactive and engaging question types beyond traditional multiple choice and text-based questions.

### Drag and Drop Questions

Drag and Drop questions allow users to match items by dragging them to corresponding targets, providing a more interactive learning experience.

#### Key Features

- **Intuitive Drag and Drop Interface**: Smooth drag and drop functionality with visual feedback
- **Flexible Item Mapping**: Items can be mapped to any target
- **Visual Feedback**: Clear visual indication of mapped items
- **Reset Functionality**: Option to reset all mappings and start over
- **Accessibility Support**: Keyboard navigation and screen reader support

#### Usage

```rust
// In your component
let (question, _) = create_signal(Question {
    // ... question properties
    content: QuestionContent {
        text: "Match the countries with their capitals".to_string(),
        drag_drop_content: Some(DragDropContent {
            items: vec![
                DragDropItem { id: "item1".to_string(), text: "France".to_string(), image_url: None },
                DragDropItem { id: "item2".to_string(), text: "Germany".to_string(), image_url: None },
                // ... more items
            ],
            targets: vec![
                "Paris".to_string(),
                "Berlin".to_string(),
                // ... more targets
            ],
        }),
        // ... other content properties
    },
    answer_type: AnswerType::DragDrop,
    // ... other question properties
});

// In your view
view! {
    <DragDropQuestion
        question=question
        on_answer=on_answer
        is_submitted=is_submitted
        is_correct=is_correct
    />
}
```

### Hotspot Questions

Hotspot questions allow users to identify specific areas on an image, making them ideal for anatomy, geography, and other visual subjects.

#### Key Features

- **Multiple Hotspot Shapes**: Support for rectangles, circles, and polygons
- **Multiple Selection**: Users can select multiple hotspots
- **Visual Feedback**: Clear visual indication of selected hotspots
- **Correct/Incorrect Feedback**: Visual feedback for correct and incorrect selections
- **Responsive Design**: Hotspots scale properly with image size

#### Usage

```rust
// In your component
let (question, _) = create_signal(Question {
    // ... question properties
    content: QuestionContent {
        text: "Identify the countries in Europe".to_string(),
        hotspot_content: Some(HotspotContent {
            image_url: "/images/europe_map.jpg".to_string(),
            hotspots: vec![
                Hotspot {
                    id: "france".to_string(),
                    shape: HotspotShape::Polygon { 
                        points: vec![
                            Point { x: 100.0, y: 100.0 },
                            Point { x: 150.0, y: 120.0 },
                            // ... more points
                        ] 
                    },
                    label: Some("France".to_string()),
                },
                // ... more hotspots
            ],
        }),
        // ... other content properties
    },
    answer_type: AnswerType::Hotspot,
    // ... other question properties
});

// In your view
view! {
    <HotspotQuestion
        question=question
        on_answer=on_answer
        is_submitted=is_submitted
        is_correct=is_correct
    />
}
```

## 2. Rich Text Editor for Questions

The Quiz Module now includes a rich text editor for creating and editing questions, allowing for formatted text, lists, and other rich content.

### Key Features

- **Formatting Options**: Bold, italic, underline
- **Heading Styles**: Multiple heading levels
- **Lists**: Bullet and numbered lists
- **Clean Interface**: Intuitive toolbar with clear icons
- **HTML Output**: Generates clean HTML for storage and display
- **Accessibility Support**: Keyboard shortcuts and screen reader support

### Usage

```rust
// In your component
let (content, set_content) = create_signal("<p>Initial content</p>".to_string());

let on_content_change = move |new_content: String| {
    set_content.set(new_content);
};

// In your view
view! {
    <RichTextEditor
        initial_content=content
        on_change=on_content_change
        placeholder="Enter question text here..."
    />
}
```

## 3. Media Upload and Management

The Quiz Module now includes comprehensive media upload and management capabilities, allowing users to include images, audio, and video in their questions.

### Key Features

- **Multiple Media Types**: Support for images, audio, and video
- **File Size Validation**: Configurable maximum file size
- **Upload Progress**: Visual feedback during upload
- **Media Preview**: Preview uploaded media before saving
- **Media Management**: Remove or replace uploaded media
- **Responsive Design**: Media displays properly on all screen sizes

### Usage

```rust
// In your component
let (media_file, set_media_file) = create_signal(None::<MediaFile>);

let on_media_upload = move |file: MediaFile| {
    set_media_file.set(Some(file));
};

let on_media_remove = move |_| {
    set_media_file.set(None);
};

// In your view
view! {
    <div>
        <MediaUploader
            on_upload=on_media_upload
            media_type=Some(MediaType::Image)
            max_size_mb=Some(5.0)
        />
        
        <MediaPreview
            media=media_file
            on_remove=Some(on_media_remove)
        />
    </div>
}
```

## Integration with Question Editor

These enhancements are integrated into the question editor, allowing for a seamless experience when creating and editing questions.

### Example: Creating a Hotspot Question

```rust
// In your question editor component
let (question_text, set_question_text) = create_signal("".to_string());
let (image_file, set_image_file) = create_signal(None::<MediaFile>);
let (hotspots, set_hotspots) = create_signal(Vec::<Hotspot>::new());

// ... other state and handlers

// In your view
view! {
    <div class="question-editor">
        <h2>"Create Hotspot Question"</h2>
        
        <div class="form-group">
            <label for="question-text">"Question Text"</label>
            <RichTextEditor
                initial_content=question_text
                on_change=move |text| set_question_text.set(text)
                placeholder="Enter question text here..."
            />
        </div>
        
        <div class="form-group">
            <label>"Question Image"</label>
            <MediaUploader
                on_upload=move |file| set_image_file.set(Some(file))
                media_type=Some(MediaType::Image)
                max_size_mb=Some(5.0)
            />
            
            <MediaPreview
                media=image_file
                on_remove=Some(move |_| set_image_file.set(None))
            />
        </div>
        
        // ... hotspot editor UI
        
        <div class="form-actions">
            <button type="button" class="save-btn" on:click=save_question>
                "Save Question"
            </button>
        </div>
    </div>
}
```

## Styling and Theming

All new components follow the quiz module's theming system, ensuring a consistent look and feel throughout the application. The components use the CSS variables defined in `quiz-theme.css`, allowing them to adapt to theme changes.

### CSS Variables Used

```css
/* Colors */
--quiz-primary
--quiz-primary-hover
--quiz-primary-light
--quiz-text
--quiz-text-light
--quiz-text-lighter
--quiz-bg
--quiz-card-bg
--quiz-border
--quiz-danger
--quiz-success

/* Spacing */
--quiz-spacing-xs
--quiz-spacing-sm
--quiz-spacing
--quiz-spacing-md
--quiz-spacing-lg

/* Border Radius */
--quiz-radius-sm
--quiz-radius
--quiz-radius-md

/* Typography */
--quiz-font-body
--quiz-font-heading
--quiz-font-sm
--quiz-font-base
--quiz-font-medium

/* Transitions */
--quiz-transition
```

## Accessibility Considerations

All new components are designed with accessibility in mind:

- **Keyboard Navigation**: All interactive elements are keyboard accessible
- **Screen Reader Support**: Appropriate ARIA attributes and semantic HTML
- **Color Contrast**: Sufficient contrast for text and interactive elements
- **Focus Indicators**: Clear visual indication of focused elements
- **Reduced Motion**: Respects user's reduced motion preferences

## Browser Compatibility

The components are tested and compatible with:

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

## Future Enhancements

Potential future enhancements for these components include:

- **More Question Types**: Sequence ordering, fill-in-the-blanks, crossword puzzles
- **Advanced Media Editing**: Crop, resize, and annotate images
- **Collaborative Editing**: Real-time collaborative question editing
- **AI-Assisted Creation**: AI suggestions for questions and answers
- **Accessibility Improvements**: Enhanced screen reader support and keyboard navigation
