// Legacy utilities (to be deprecated)
pub mod date_utils;
pub mod errors;
pub mod error_handler;
pub mod file_system;
pub mod images;
pub mod index_project;
pub mod logger;
pub mod naming_conventions;

// Consolidated utilities
pub mod consolidated;

// Re-export commonly used utility functions
pub use date_utils::{parse_date_string, format_date};
pub use error_handler::{ErrorHandler, handle_error, handle_api_error, get_global_error_handler};

// Re-export consolidated utilities
pub use consolidated::{parse_date, format_date as format_date_iso, format_date_for_display, serialize_date, deserialize_date, is_date_before, is_date_after, is_date_between, get_current_date, add_days, subtract_days, date_diff_in_days, date_diff_in_hours, date_diff_in_minutes};
pub use consolidated::{read_file, write_file, append_file, delete_file, create_directory, delete_directory, copy_file, move_file, get_file_size, get_file_extension, get_file_name, get_file_path, is_file, is_directory, is_binary_file, is_text_file, list_files, list_directories, find_files, find_directories};
pub use consolidated::{capitalize, lowercase, uppercase, title_case, trim, trim_start, trim_end, truncate, is_empty, is_blank, is_numeric, is_alphanumeric, contains, starts_with, ends_with, replace_all, split, join, format_template, slugify};
pub use consolidated::{Logger, create_logger, init_logger, is_logger_initialized, log_info, log_error, log_warn, log_debug};
pub use consolidated::{resize_image, crop_image, rotate_image, flip_image, convert_image, get_image_dimensions, get_image_format, is_image_file, optimize_image, generate_thumbnail};