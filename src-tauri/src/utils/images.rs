use image::{DynamicImage, GenericImageView, ImageFormat, imageops};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::io::Cursor;
use std::path::Path;
use uuid::Uuid;
use log::{debug, error};
use anyhow::{Result, Context};

// Process avatar uploads with optimizations
pub async fn process_avatar_image(
    file_path: &Path,
    output_dir: &Path,
) -> Result<String> {
    // Read file
    let mut file = File::open(file_path)
        .await
        .with_context(|| format!("Failed to open file at {:?}", file_path))?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .context("Failed to read file contents")?;
    
    // Process image in a worker thread to avoid blocking
    let output_dir = output_dir.to_path_buf();
    let result = tokio::task::spawn_blocking(move || {
        // Load image
        let img = image::load_from_memory(&buffer)
            .context("Failed to decode image")?;
        
        // Generate a unique filename
        let filename = format!("{}.webp", Uuid::new_v4());
        let output_path = output_dir.join(&filename);
        
        // Resize and optimize
        let optimized = optimize_avatar(img)?;
        
        // Save as WebP for better compression
        optimized.save_with_format(&output_path, ImageFormat::WebP)
            .context("Failed to save optimized image")?;
        
        Ok(filename)
    }).await.context("Image processing task failed")?;
    
    result
}

// Optimize avatar images for better performance
fn optimize_avatar(img: DynamicImage) -> Result<DynamicImage> {
    // Get dimensions
    let (width, height) = img.dimensions();
    
    // Determine target size (max 256x256, preserve aspect ratio)
    let max_dimension = 256;
    let scale = if width > height {
        max_dimension as f32 / width as f32
    } else {
        max_dimension as f32 / height as f32
    };
    
    let target_width = if scale < 1.0 { (width as f32 * scale) as u32 } else { width };
    let target_height = if scale < 1.0 { (height as f32 * scale) as u32 } else { height };
    
    // Resize with Lanczos3 for better quality
    let resized = imageops::resize(
        &img, 
        target_width, 
        target_height, 
        imageops::FilterType::Lanczos3
    );
    
    // Convert to RGB for consistent WebP encoding
    let rgb_img = DynamicImage::ImageRgba8(resized);
    
    Ok(rgb_img)
}

// Process image attachments for forum posts
pub async fn process_attachment_image(
    file_path: &Path,
    output_dir: &Path,
    max_width: u32,
) -> Result<(String, u32, u32)> {
    // Read file
    let mut file = File::open(file_path)
        .await
        .with_context(|| format!("Failed to open file at {:?}", file_path))?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .context("Failed to read file contents")?;
    
    // Process image in worker thread
    let output_dir = output_dir.to_path_buf();
    let result = tokio::task::spawn_blocking(move || {
        // Load image
        let img = image::load_from_memory(&buffer)
            .context("Failed to decode image")?;
        
        // Generate filename based on content hash for deduplication
        let image_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let mut hasher = DefaultHasher::new();
            buffer.hash(&mut hasher);
            hasher.finish()
        };
        
        let extension = match img {
            DynamicImage::ImageRgb8(_) | DynamicImage::ImageRgba8(_) => "webp",
            _ => "jpg", // Fall back to jpg for other formats
        };
        
        let filename = format!("{}.{}", image_hash, extension);
        let output_path = output_dir.join(&filename);
        
        // Check if file already exists (deduplication)
        if !output_path.exists() {
            // Get dimensions
            let (width, height) = img.dimensions();
            
            // Resize if width exceeds max
            let final_img = if width > max_width {
                let scale = max_width as f32 / width as f32;
                let new_height = (height as f32 * scale) as u32;
                
                imageops::resize(
                    &img, 
                    max_width, 
                    new_height, 
                    imageops::FilterType::Lanczos3
                )
            } else {
                img.to_rgba8()
            };
            
            // Save optimized image
            let format = if extension == "webp" { 
                ImageFormat::WebP 
            } else { 
                ImageFormat::Jpeg 
            };
            
            let final_img = DynamicImage::ImageRgba8(final_img);
            final_img.save_with_format(&output_path, format)
                .context("Failed to save optimized image")?;
        }
        
        // Get final dimensions
        let final_img = image::open(&output_path)
            .context("Failed to open saved image")?;
        let (width, height) = final_img.dimensions();
        
        Ok((filename, width, height))
    }).await.context("Image processing task failed")?;
    
    result
}

// Generate thumbnail for large images
pub async fn generate_thumbnail(
    source_path: &Path,
    output_dir: &Path,
    thumb_size: u32,
) -> Result<String> {
    // Read source file
    let img = image::open(source_path)
        .with_context(|| format!("Failed to open image at {:?}", source_path))?;
    
    // Generate thumbnail filename
    let source_filename = source_path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown");
    
    let thumb_filename = format!("thumb_{}", source_filename);
    let output_path = output_dir.join(&thumb_filename);
    
    // Resize to thumbnail
    let thumbnail = img.thumbnail(thumb_size, thumb_size);
    
    // Save as WebP for better compression
    thumbnail.save_with_format(&output_path, ImageFormat::WebP)
        .context("Failed to save thumbnail")?;
    
    Ok(thumb_filename)
}