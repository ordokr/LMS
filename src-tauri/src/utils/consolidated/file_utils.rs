use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use log::{error, warn};
use regex::Regex;
use crate::errors::error::{Error, Result};

/// Read a file as a string
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Result<String>` - The file content or an error
pub fn read_file(path: &Path) -> Result<String> {
    let mut content = String::new();
    let mut file = File::open(path)
        .map_err(|e| Error::internal(format!("Failed to open file {}: {}", path.display(), e)))?;
    
    file.read_to_string(&mut content)
        .map_err(|e| Error::internal(format!("Failed to read file {}: {}", path.display(), e)))?;
    
    Ok(content)
}

/// Write a string to a file
/// 
/// # Arguments
/// * `path` - The path to the file
/// * `content` - The content to write
/// 
/// # Returns
/// * `Result<()>` - Success or an error
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Error::internal(format!("Failed to create directory {}: {}", parent.display(), e)))?;
    }
    
    let mut file = File::create(path)
        .map_err(|e| Error::internal(format!("Failed to create file {}: {}", path.display(), e)))?;
    
    file.write_all(content.as_bytes())
        .map_err(|e| Error::internal(format!("Failed to write to file {}: {}", path.display(), e)))?;
    
    Ok(())
}

/// Append a string to a file
/// 
/// # Arguments
/// * `path` - The path to the file
/// * `content` - The content to append
/// 
/// # Returns
/// * `Result<()>` - Success or an error
pub fn append_file(path: &Path, content: &str) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Error::internal(format!("Failed to create directory {}: {}", parent.display(), e)))?;
    }
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| Error::internal(format!("Failed to open file {}: {}", path.display(), e)))?;
    
    file.write_all(content.as_bytes())
        .map_err(|e| Error::internal(format!("Failed to append to file {}: {}", path.display(), e)))?;
    
    Ok(())
}

/// Delete a file
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Result<()>` - Success or an error
pub fn delete_file(path: &Path) -> Result<()> {
    fs::remove_file(path)
        .map_err(|e| Error::internal(format!("Failed to delete file {}: {}", path.display(), e)))?;
    
    Ok(())
}

/// Create a directory
/// 
/// # Arguments
/// * `path` - The path to the directory
/// 
/// # Returns
/// * `Result<()>` - Success or an error
pub fn create_directory(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .map_err(|e| Error::internal(format!("Failed to create directory {}: {}", path.display(), e)))?;
    
    Ok(())
}

/// Delete a directory
/// 
/// # Arguments
/// * `path` - The path to the directory
/// * `recursive` - Whether to delete recursively
/// 
/// # Returns
/// * `Result<()>` - Success or an error
pub fn delete_directory(path: &Path, recursive: bool) -> Result<()> {
    if recursive {
        fs::remove_dir_all(path)
            .map_err(|e| Error::internal(format!("Failed to delete directory {}: {}", path.display(), e)))?;
    } else {
        fs::remove_dir(path)
            .map_err(|e| Error::internal(format!("Failed to delete directory {}: {}", path.display(), e)))?;
    }
    
    Ok(())
}

/// Copy a file
/// 
/// # Arguments
/// * `src` - The source path
/// * `dst` - The destination path
/// 
/// # Returns
/// * `Result<()>` - Success or an error
pub fn copy_file(src: &Path, dst: &Path) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Error::internal(format!("Failed to create directory {}: {}", parent.display(), e)))?;
    }
    
    fs::copy(src, dst)
        .map_err(|e| Error::internal(format!("Failed to copy file from {} to {}: {}", src.display(), dst.display(), e)))?;
    
    Ok(())
}

/// Move a file
/// 
/// # Arguments
/// * `src` - The source path
/// * `dst` - The destination path
/// 
/// # Returns
/// * `Result<()>` - Success or an error
pub fn move_file(src: &Path, dst: &Path) -> Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Error::internal(format!("Failed to create directory {}: {}", parent.display(), e)))?;
    }
    
    fs::rename(src, dst)
        .map_err(|e| Error::internal(format!("Failed to move file from {} to {}: {}", src.display(), dst.display(), e)))?;
    
    Ok(())
}

/// Get the size of a file
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Result<u64>` - The file size in bytes or an error
pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)
        .map_err(|e| Error::internal(format!("Failed to get metadata for {}: {}", path.display(), e)))?;
    
    Ok(metadata.len())
}

/// Get the extension of a file
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Option<String>` - The file extension or None
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

/// Get the name of a file
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Option<String>` - The file name or None
pub fn get_file_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
}

/// Get the path of a file
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Option<String>` - The file path or None
pub fn get_file_path(path: &Path) -> Option<String> {
    path.parent()
        .and_then(|parent| parent.to_str())
        .map(|parent| parent.to_string())
}

/// Check if a path is a file
/// 
/// # Arguments
/// * `path` - The path to check
/// 
/// # Returns
/// * `bool` - True if the path is a file
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// Check if a path is a directory
/// 
/// # Arguments
/// * `path` - The path to check
/// 
/// # Returns
/// * `bool` - True if the path is a directory
pub fn is_directory(path: &Path) -> bool {
    path.is_dir()
}

/// Check if a file is a binary file
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Result<bool>` - True if the file is binary or an error
pub fn is_binary_file(path: &Path) -> Result<bool> {
    // Define binary signatures
    let binary_signatures: Vec<Vec<u8>> = vec![
        vec![0xFF, 0xD8],                // JPEG
        vec![0x89, 0x50, 0x4E, 0x47],    // PNG
        vec![0x47, 0x49, 0x46],          // GIF
        vec![0x50, 0x4B, 0x03, 0x04],    // ZIP/JAR/DOCX
        vec![0x25, 0x50, 0x44, 0x46],    // PDF
    ];
    
    let mut buffer = [0u8; 8];
    let mut file = File::open(path)
        .map_err(|e| Error::internal(format!("Failed to open file {}: {}", path.display(), e)))?;
    
    let bytes_read = file.read(&mut buffer)
        .map_err(|e| Error::internal(format!("Failed to read file {}: {}", path.display(), e)))?;
    
    // Check signatures
    for signature in binary_signatures {
        if signature.len() <= bytes_read {
            let mut matches = true;
            for (i, &byte) in signature.iter().enumerate() {
                if buffer[i] != byte {
                    matches = false;
                    break;
                }
            }
            if matches {
                return Ok(true);
            }
        }
    }
    
    // Check for null bytes (common in binary files)
    for &byte in &buffer[..bytes_read] {
        if byte == 0 {
            return Ok(true);
        }
    }
    
    Ok(false)
}

/// Check if a file is a text file
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `Result<bool>` - True if the file is text or an error
pub fn is_text_file(path: &Path) -> Result<bool> {
    let is_binary = is_binary_file(path)?;
    Ok(!is_binary)
}

/// List files in a directory
/// 
/// # Arguments
/// * `path` - The path to the directory
/// * `recursive` - Whether to list recursively
/// 
/// # Returns
/// * `Result<Vec<PathBuf>>` - The list of files or an error
pub fn list_files(path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if !path.is_dir() {
        return Err(Error::internal(format!("Path is not a directory: {}", path.display())));
    }
    
    for entry in fs::read_dir(path)
        .map_err(|e| Error::internal(format!("Failed to read directory {}: {}", path.display(), e)))? {
        let entry = entry
            .map_err(|e| Error::internal(format!("Failed to read directory entry: {}", e)))?;
        
        let entry_path = entry.path();
        
        if entry_path.is_file() {
            files.push(entry_path);
        } else if recursive && entry_path.is_dir() {
            let mut sub_files = list_files(&entry_path, true)?;
            files.append(&mut sub_files);
        }
    }
    
    Ok(files)
}

/// List directories in a directory
/// 
/// # Arguments
/// * `path` - The path to the directory
/// * `recursive` - Whether to list recursively
/// 
/// # Returns
/// * `Result<Vec<PathBuf>>` - The list of directories or an error
pub fn list_directories(path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut directories = Vec::new();
    
    if !path.is_dir() {
        return Err(Error::internal(format!("Path is not a directory: {}", path.display())));
    }
    
    for entry in fs::read_dir(path)
        .map_err(|e| Error::internal(format!("Failed to read directory {}: {}", path.display(), e)))? {
        let entry = entry
            .map_err(|e| Error::internal(format!("Failed to read directory entry: {}", e)))?;
        
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            directories.push(entry_path.clone());
            
            if recursive {
                let mut sub_directories = list_directories(&entry_path, true)?;
                directories.append(&mut sub_directories);
            }
        }
    }
    
    Ok(directories)
}

/// Find files matching a pattern
/// 
/// # Arguments
/// * `path` - The path to the directory
/// * `pattern` - The regex pattern to match
/// * `recursive` - Whether to search recursively
/// 
/// # Returns
/// * `Result<Vec<PathBuf>>` - The list of matching files or an error
pub fn find_files(path: &Path, pattern: &str, recursive: bool) -> Result<Vec<PathBuf>> {
    let regex = Regex::new(pattern)
        .map_err(|e| Error::internal(format!("Invalid regex pattern: {}", e)))?;
    
    let files = list_files(path, recursive)?;
    
    let matching_files = files.into_iter()
        .filter(|file| {
            if let Some(file_name) = file.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    return regex.is_match(file_name_str);
                }
            }
            false
        })
        .collect();
    
    Ok(matching_files)
}

/// Find directories matching a pattern
/// 
/// # Arguments
/// * `path` - The path to the directory
/// * `pattern` - The regex pattern to match
/// * `recursive` - Whether to search recursively
/// 
/// # Returns
/// * `Result<Vec<PathBuf>>` - The list of matching directories or an error
pub fn find_directories(path: &Path, pattern: &str, recursive: bool) -> Result<Vec<PathBuf>> {
    let regex = Regex::new(pattern)
        .map_err(|e| Error::internal(format!("Invalid regex pattern: {}", e)))?;
    
    let directories = list_directories(path, recursive)?;
    
    let matching_directories = directories.into_iter()
        .filter(|dir| {
            if let Some(dir_name) = dir.file_name() {
                if let Some(dir_name_str) = dir_name.to_str() {
                    return regex.is_match(dir_name_str);
                }
            }
            false
        })
        .collect();
    
    Ok(matching_directories)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_read_write_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        let content = "Hello, world!";
        write_file(&file_path, content).unwrap();
        
        let read_content = read_file(&file_path).unwrap();
        assert_eq!(read_content, content);
    }
    
    #[test]
    fn test_append_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        let content1 = "Hello, ";
        let content2 = "world!";
        
        write_file(&file_path, content1).unwrap();
        append_file(&file_path, content2).unwrap();
        
        let read_content = read_file(&file_path).unwrap();
        assert_eq!(read_content, "Hello, world!");
    }
    
    #[test]
    fn test_delete_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        write_file(&file_path, "Hello, world!").unwrap();
        assert!(file_path.exists());
        
        delete_file(&file_path).unwrap();
        assert!(!file_path.exists());
    }
    
    #[test]
    fn test_create_delete_directory() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        
        create_directory(&dir_path).unwrap();
        assert!(dir_path.exists());
        assert!(is_directory(&dir_path));
        
        delete_directory(&dir_path, false).unwrap();
        assert!(!dir_path.exists());
    }
    
    #[test]
    fn test_copy_move_file() {
        let temp_dir = tempdir().unwrap();
        let file_path1 = temp_dir.path().join("test1.txt");
        let file_path2 = temp_dir.path().join("test2.txt");
        let file_path3 = temp_dir.path().join("test3.txt");
        
        write_file(&file_path1, "Hello, world!").unwrap();
        
        copy_file(&file_path1, &file_path2).unwrap();
        assert!(file_path1.exists());
        assert!(file_path2.exists());
        
        let content2 = read_file(&file_path2).unwrap();
        assert_eq!(content2, "Hello, world!");
        
        move_file(&file_path2, &file_path3).unwrap();
        assert!(file_path1.exists());
        assert!(!file_path2.exists());
        assert!(file_path3.exists());
        
        let content3 = read_file(&file_path3).unwrap();
        assert_eq!(content3, "Hello, world!");
    }
    
    #[test]
    fn test_file_info() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        write_file(&file_path, "Hello, world!").unwrap();
        
        assert_eq!(get_file_size(&file_path).unwrap(), 13);
        assert_eq!(get_file_extension(&file_path).unwrap(), "txt");
        assert_eq!(get_file_name(&file_path).unwrap(), "test.txt");
        assert!(is_file(&file_path));
        assert!(!is_directory(&file_path));
        assert!(!is_binary_file(&file_path).unwrap());
        assert!(is_text_file(&file_path).unwrap());
    }
    
    #[test]
    fn test_list_find() {
        let temp_dir = tempdir().unwrap();
        let dir_path1 = temp_dir.path().join("dir1");
        let dir_path2 = temp_dir.path().join("dir2");
        
        create_directory(&dir_path1).unwrap();
        create_directory(&dir_path2).unwrap();
        
        let file_path1 = dir_path1.join("test1.txt");
        let file_path2 = dir_path1.join("test2.log");
        let file_path3 = dir_path2.join("test3.txt");
        
        write_file(&file_path1, "Hello, world!").unwrap();
        write_file(&file_path2, "Hello, world!").unwrap();
        write_file(&file_path3, "Hello, world!").unwrap();
        
        let files = list_files(temp_dir.path(), false).unwrap();
        assert_eq!(files.len(), 0); // No files in the root directory
        
        let files = list_files(temp_dir.path(), true).unwrap();
        assert_eq!(files.len(), 3); // All files recursively
        
        let directories = list_directories(temp_dir.path(), false).unwrap();
        assert_eq!(directories.len(), 2); // dir1 and dir2
        
        let txt_files = find_files(temp_dir.path(), r"\.txt$", true).unwrap();
        assert_eq!(txt_files.len(), 2); // test1.txt and test3.txt
        
        let log_files = find_files(temp_dir.path(), r"\.log$", true).unwrap();
        assert_eq!(log_files.len(), 1); // test2.log
        
        let test_dirs = find_directories(temp_dir.path(), r"^dir\d$", false).unwrap();
        assert_eq!(test_dirs.len(), 2); // dir1 and dir2
    }
}
