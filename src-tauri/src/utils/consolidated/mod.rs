// Consolidated utilities module
// This module contains consolidated utility implementations that replace redundant utilities

mod date_utils;
mod file_utils;
mod string_utils;
mod logger;
mod image_utils;

// Re-export date utilities
pub use date_utils::{
    parse_date, format_date, format_date_for_display, 
    serialize_date, deserialize_date, 
    is_date_before, is_date_after, is_date_between,
    get_current_date, add_days, subtract_days,
    date_diff_in_days, date_diff_in_hours, date_diff_in_minutes,
};

// Re-export file utilities
pub use file_utils::{
    read_file, write_file, append_file, delete_file,
    create_directory, delete_directory, copy_file, move_file,
    get_file_size, get_file_extension, get_file_name, get_file_path,
    is_file, is_directory, is_binary_file, is_text_file,
    list_files, list_directories, find_files, find_directories,
};

// Re-export string utilities
pub use string_utils::{
    capitalize, lowercase, uppercase, title_case,
    trim, trim_start, trim_end, truncate,
    is_empty, is_blank, is_numeric, is_alphanumeric,
    contains, starts_with, ends_with, replace_all,
    split, join, format_template, slugify,
};

// Re-export logger
pub use logger::{
    Logger, create_logger, init_logger, is_logger_initialized,
    log_info, log_error, log_warn, log_debug,
};

// Re-export image utilities
pub use image_utils::{
    resize_image, crop_image, rotate_image, flip_image,
    convert_image, get_image_dimensions, get_image_format,
    is_image_file, optimize_image, generate_thumbnail,
};
