# Consolidated Utilities

This directory contains the consolidated utility implementations for the LMS application. These utilities replace the redundant utility implementations that were scattered throughout the codebase.

## Overview

The consolidated utilities provide a consistent interface for common operations across the application. They are designed to be:

- **Consistent**: All utilities follow the same design patterns and conventions
- **Comprehensive**: All utilities include all methods needed for all use cases
- **Reusable**: All utilities are designed for easy reuse
- **Testable**: All utilities are designed for easy testing

## Utilities

The following utilities are included:

- **Date Utilities**: Functions for working with dates and times
- **File Utilities**: Functions for working with files and directories
- **String Utilities**: Functions for working with strings
- **Logger**: Functions for logging
- **Image Utilities**: Functions for working with images

## Usage

### Date Utilities

```rust
use crate::utils::consolidated::{
    parse_date, format_date, format_date_for_display,
    is_date_before, is_date_after, is_date_between,
    get_current_date, add_days, subtract_days,
    date_diff_in_days, date_diff_in_hours, date_diff_in_minutes,
};

// Parse a date
let date = parse_date("2023-07-17T12:34:56Z").unwrap();

// Format a date
let date_str = format_date(&date);
let display_str = format_date_for_display(&date, Some("%Y-%m-%d"));

// Compare dates
let is_before = is_date_before(&date1, &date2);
let is_after = is_date_after(&date1, &date2);
let is_between = is_date_between(&date, &start, &end, Some(true));

// Date arithmetic
let now = get_current_date();
let tomorrow = add_days(&now, 1);
let yesterday = subtract_days(&now, 1);
let days_diff = date_diff_in_days(&date1, &date2);
```

### File Utilities

```rust
use crate::utils::consolidated::{
    read_file, write_file, append_file, delete_file,
    create_directory, delete_directory, copy_file, move_file,
    get_file_size, get_file_extension, get_file_name, get_file_path,
    is_file, is_directory, is_binary_file, is_text_file,
    list_files, list_directories, find_files, find_directories,
};
use std::path::Path;

// Read and write files
let content = read_file(Path::new("file.txt")).unwrap();
write_file(Path::new("file.txt"), "Hello, world!").unwrap();
append_file(Path::new("file.txt"), "More content").unwrap();
delete_file(Path::new("file.txt")).unwrap();

// Work with directories
create_directory(Path::new("dir")).unwrap();
delete_directory(Path::new("dir"), true).unwrap();

// Copy and move files
copy_file(Path::new("src.txt"), Path::new("dst.txt")).unwrap();
move_file(Path::new("src.txt"), Path::new("dst.txt")).unwrap();

// Get file information
let size = get_file_size(Path::new("file.txt")).unwrap();
let extension = get_file_extension(Path::new("file.txt")).unwrap();
let name = get_file_name(Path::new("file.txt")).unwrap();
let path = get_file_path(Path::new("dir/file.txt")).unwrap();

// Check file types
let is_file = is_file(Path::new("file.txt"));
let is_dir = is_directory(Path::new("dir"));
let is_binary = is_binary_file(Path::new("image.png")).unwrap();
let is_text = is_text_file(Path::new("file.txt")).unwrap();

// List and find files
let files = list_files(Path::new("dir"), true).unwrap();
let dirs = list_directories(Path::new("dir"), true).unwrap();
let txt_files = find_files(Path::new("dir"), r"\.txt$", true).unwrap();
let test_dirs = find_directories(Path::new("dir"), r"^test", true).unwrap();
```

### String Utilities

```rust
use crate::utils::consolidated::{
    capitalize, lowercase, uppercase, title_case,
    trim, trim_start, trim_end, truncate,
    is_empty, is_blank, is_numeric, is_alphanumeric,
    contains, starts_with, ends_with, replace_all,
    split, join, format_template, slugify,
};
use std::collections::HashMap;

// Case conversion
let capitalized = capitalize("hello");
let lower = lowercase("Hello");
let upper = uppercase("Hello");
let title = title_case("hello world");

// Trimming and truncation
let trimmed = trim("  hello  ");
let trimmed_start = trim_start("  hello  ");
let trimmed_end = trim_end("  hello  ");
let truncated = truncate("Hello, world!", 5, Some("..."));

// String checks
let is_empty = is_empty("");
let is_blank = is_blank("   ");
let is_numeric = is_numeric("123");
let is_alphanumeric = is_alphanumeric("abc123");

// String operations
let contains = contains("Hello, world!", "world");
let starts_with = starts_with("Hello, world!", "Hello");
let ends_with = ends_with("Hello, world!", "world!");
let replaced = replace_all("Hello, world!", "world", "universe");

// String manipulation
let parts = split("Hello, world!", ", ");
let joined = join(&vec!["Hello".to_string(), "world!".to_string()], ", ");

// Template formatting
let mut variables = HashMap::new();
variables.insert("name".to_string(), "John".to_string());
let result = format_template("Hello, {name}!", &variables).unwrap();

// URL-friendly slugs
let slug = slugify("Hello, world!");
```

### Logger

```rust
use crate::utils::consolidated::{
    Logger, create_logger, init_logger, is_logger_initialized,
    log_info, log_error, log_warn, log_debug,
};

// Initialize the logger
init_logger().unwrap();

// Create a logger
let logger = create_logger("my_module");

// Log messages
logger.info("This is an info message", None::<String>);
logger.error("This is an error message", None::<String>);
logger.warn("This is a warning message", None::<String>);
logger.debug("This is a debug message", None::<String>);

// Log with context
let context = serde_json::json!({
    "key": "value",
    "number": 123
});
logger.info("Message with context", Some(context));

// Static logging functions
log_info("my_module", "This is an info message", None::<String>);
log_error("my_module", "This is an error message", None::<String>);
log_warn("my_module", "This is a warning message", None::<String>);
log_debug("my_module", "This is a debug message", None::<String>);
```

### Image Utilities

```rust
use crate::utils::consolidated::{
    resize_image, crop_image, rotate_image, flip_image,
    convert_image, get_image_dimensions, get_image_format,
    is_image_file, optimize_image, generate_thumbnail,
};
use image::{DynamicImage, ImageFormat};
use std::path::Path;

// Load an image
let img = image::open(Path::new("image.png")).unwrap();

// Resize an image
let resized = resize_image(&img, 100, 100, true).unwrap();

// Crop an image
let cropped = crop_image(&img, 10, 10, 100, 100).unwrap();

// Rotate and flip an image
let rotated = rotate_image(&img, 90.0).unwrap();
let flipped = flip_image(&img, true, false).unwrap();

// Convert an image
let png_data = convert_image(&img, ImageFormat::Png).unwrap();

// Get image information
let (width, height) = get_image_dimensions(&img);
let format = get_image_format(Path::new("image.png")).unwrap();
let is_image = is_image_file(Path::new("image.png"));

// Optimize an image
let optimized = optimize_image(&img, ImageFormat::Jpeg, 80).unwrap();

// Generate a thumbnail
let thumbnail = generate_thumbnail(&img, 100, 100).unwrap();
```

## Best Practices

1. **Use the consolidated utilities**: Always use the consolidated utilities instead of implementing your own
2. **Add new utilities to the consolidated modules**: If you need a new utility, add it to the appropriate consolidated module
3. **Follow the naming conventions**: Use consistent naming for utility functions
4. **Add comprehensive tests**: All utility functions should have tests
5. **Add proper documentation**: All utility functions should be documented
6. **Handle errors properly**: All utility functions should return `Result` when they can fail
7. **Use proper logging**: All utility functions should log important events
8. **Use proper validation**: All utility functions should validate input
9. **Use proper error messages**: All error messages should be clear and helpful
10. **Use proper error types**: All errors should use the unified error system
