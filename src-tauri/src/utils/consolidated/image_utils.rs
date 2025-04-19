use image::{DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat};
use std::io::Cursor;
use std::path::Path;
use crate::errors::error::{Error, Result};

/// Resize an image
/// 
/// # Arguments
/// * `img` - The image to resize
/// * `width` - The new width
/// * `height` - The new height
/// * `preserve_aspect_ratio` - Whether to preserve the aspect ratio
/// 
/// # Returns
/// * `Result<DynamicImage>` - The resized image or an error
pub fn resize_image(img: &DynamicImage, width: u32, height: u32, preserve_aspect_ratio: bool) -> Result<DynamicImage> {
    if preserve_aspect_ratio {
        Ok(img.resize(width, height, image::imageops::FilterType::Lanczos3))
    } else {
        Ok(img.resize_exact(width, height, image::imageops::FilterType::Lanczos3))
    }
}

/// Crop an image
/// 
/// # Arguments
/// * `img` - The image to crop
/// * `x` - The x coordinate of the top-left corner
/// * `y` - The y coordinate of the top-left corner
/// * `width` - The width of the crop
/// * `height` - The height of the crop
/// 
/// # Returns
/// * `Result<DynamicImage>` - The cropped image or an error
pub fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> Result<DynamicImage> {
    // Check if the crop is within the image bounds
    let (img_width, img_height) = img.dimensions();
    if x + width > img_width || y + height > img_height {
        return Err(Error::validation("Crop dimensions exceed image bounds"));
    }
    
    Ok(img.crop_imm(x, y, width, height))
}

/// Rotate an image
/// 
/// # Arguments
/// * `img` - The image to rotate
/// * `degrees` - The rotation angle in degrees
/// 
/// # Returns
/// * `Result<DynamicImage>` - The rotated image or an error
pub fn rotate_image(img: &DynamicImage, degrees: f32) -> Result<DynamicImage> {
    // Normalize degrees to 0-360
    let degrees = degrees % 360.0;
    
    // Convert degrees to radians
    let radians = degrees.to_radians();
    
    Ok(img.rotate(radians))
}

/// Flip an image
/// 
/// # Arguments
/// * `img` - The image to flip
/// * `horizontal` - Whether to flip horizontally
/// * `vertical` - Whether to flip vertically
/// 
/// # Returns
/// * `Result<DynamicImage>` - The flipped image or an error
pub fn flip_image(img: &DynamicImage, horizontal: bool, vertical: bool) -> Result<DynamicImage> {
    let mut result = img.clone();
    
    if horizontal {
        result = result.fliph();
    }
    
    if vertical {
        result = result.flipv();
    }
    
    Ok(result)
}

/// Convert an image to a different format
/// 
/// # Arguments
/// * `img` - The image to convert
/// * `format` - The target format
/// 
/// # Returns
/// * `Result<Vec<u8>>` - The converted image data or an error
pub fn convert_image(img: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    let output_format = match format {
        ImageFormat::Png => ImageOutputFormat::Png,
        ImageFormat::Jpeg => ImageOutputFormat::Jpeg(90), // Quality 90
        ImageFormat::Gif => ImageOutputFormat::Gif,
        ImageFormat::WebP => ImageOutputFormat::WebP,
        _ => return Err(Error::validation("Unsupported image format")),
    };
    
    img.write_to(&mut cursor, output_format)
        .map_err(|e| Error::internal(format!("Failed to convert image: {}", e)))?;
    
    Ok(buffer)
}

/// Get the dimensions of an image
/// 
/// # Arguments
/// * `img` - The image
/// 
/// # Returns
/// * `(u32, u32)` - The width and height
pub fn get_image_dimensions(img: &DynamicImage) -> (u32, u32) {
    img.dimensions()
}

/// Get the format of an image file
/// 
/// # Arguments
/// * `path` - The path to the image file
/// 
/// # Returns
/// * `Result<ImageFormat>` - The image format or an error
pub fn get_image_format(path: &Path) -> Result<ImageFormat> {
    let format = image::ImageFormat::from_path(path)
        .map_err(|e| Error::internal(format!("Failed to determine image format: {}", e)))?;
    
    Ok(format)
}

/// Check if a file is an image
/// 
/// # Arguments
/// * `path` - The path to the file
/// 
/// # Returns
/// * `bool` - True if the file is an image
pub fn is_image_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            let ext = ext_str.to_lowercase();
            return matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tiff" | "tif");
        }
    }
    
    false
}

/// Optimize an image for web
/// 
/// # Arguments
/// * `img` - The image to optimize
/// * `format` - The target format
/// * `quality` - The quality (0-100, only for JPEG and WebP)
/// 
/// # Returns
/// * `Result<Vec<u8>>` - The optimized image data or an error
pub fn optimize_image(img: &DynamicImage, format: ImageFormat, quality: u8) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    // Clamp quality to 0-100
    let quality = quality.clamp(0, 100);
    
    let output_format = match format {
        ImageFormat::Png => ImageOutputFormat::Png,
        ImageFormat::Jpeg => ImageOutputFormat::Jpeg(quality),
        ImageFormat::WebP => ImageOutputFormat::WebP,
        _ => return Err(Error::validation("Unsupported image format for optimization")),
    };
    
    img.write_to(&mut cursor, output_format)
        .map_err(|e| Error::internal(format!("Failed to optimize image: {}", e)))?;
    
    Ok(buffer)
}

/// Generate a thumbnail
/// 
/// # Arguments
/// * `img` - The image to generate a thumbnail from
/// * `max_width` - The maximum width
/// * `max_height` - The maximum height
/// 
/// # Returns
/// * `Result<DynamicImage>` - The thumbnail or an error
pub fn generate_thumbnail(img: &DynamicImage, max_width: u32, max_height: u32) -> Result<DynamicImage> {
    let (width, height) = img.dimensions();
    
    // Calculate the scaling factor
    let width_scale = max_width as f32 / width as f32;
    let height_scale = max_height as f32 / height as f32;
    let scale = width_scale.min(height_scale);
    
    // Calculate the new dimensions
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;
    
    // Resize the image
    Ok(img.thumbnail(new_width, new_height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};
    use tempfile::tempdir;
    
    // Helper function to create a test image
    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let mut img = RgbaImage::new(width, height);
        
        // Fill with a gradient
        for y in 0..height {
            for x in 0..width {
                let r = (x * 255 / width) as u8;
                let g = (y * 255 / height) as u8;
                let b = ((x + y) * 255 / (width + height)) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
        
        DynamicImage::ImageRgba8(img)
    }
    
    #[test]
    fn test_resize_image() {
        let img = create_test_image(100, 100);
        
        // Test resize with preserved aspect ratio
        let resized = resize_image(&img, 50, 50, true).unwrap();
        assert_eq!(resized.dimensions(), (50, 50));
        
        // Test resize without preserved aspect ratio
        let resized = resize_image(&img, 50, 25, false).unwrap();
        assert_eq!(resized.dimensions(), (50, 25));
    }
    
    #[test]
    fn test_crop_image() {
        let img = create_test_image(100, 100);
        
        // Test valid crop
        let cropped = crop_image(&img, 25, 25, 50, 50).unwrap();
        assert_eq!(cropped.dimensions(), (50, 50));
        
        // Test invalid crop
        let result = crop_image(&img, 75, 75, 50, 50);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_rotate_image() {
        let img = create_test_image(100, 50);
        
        // Test 90 degree rotation
        let rotated = rotate_image(&img, 90.0).unwrap();
        let (width, height) = rotated.dimensions();
        assert!(width > 0 && height > 0);
        
        // Test 180 degree rotation
        let rotated = rotate_image(&img, 180.0).unwrap();
        assert_eq!(rotated.dimensions(), (100, 50));
    }
    
    #[test]
    fn test_flip_image() {
        let img = create_test_image(100, 100);
        
        // Test horizontal flip
        let flipped = flip_image(&img, true, false).unwrap();
        assert_eq!(flipped.dimensions(), (100, 100));
        
        // Test vertical flip
        let flipped = flip_image(&img, false, true).unwrap();
        assert_eq!(flipped.dimensions(), (100, 100));
        
        // Test both flips
        let flipped = flip_image(&img, true, true).unwrap();
        assert_eq!(flipped.dimensions(), (100, 100));
    }
    
    #[test]
    fn test_get_image_dimensions() {
        let img = create_test_image(100, 50);
        let (width, height) = get_image_dimensions(&img);
        assert_eq!(width, 100);
        assert_eq!(height, 50);
    }
    
    #[test]
    fn test_is_image_file() {
        assert!(is_image_file(Path::new("test.jpg")));
        assert!(is_image_file(Path::new("test.jpeg")));
        assert!(is_image_file(Path::new("test.png")));
        assert!(is_image_file(Path::new("test.gif")));
        assert!(is_image_file(Path::new("test.webp")));
        assert!(is_image_file(Path::new("test.bmp")));
        assert!(is_image_file(Path::new("test.tiff")));
        assert!(is_image_file(Path::new("test.tif")));
        
        assert!(!is_image_file(Path::new("test.txt")));
        assert!(!is_image_file(Path::new("test.pdf")));
        assert!(!is_image_file(Path::new("test")));
    }
    
    #[test]
    fn test_generate_thumbnail() {
        let img = create_test_image(100, 100);
        
        // Test square thumbnail
        let thumbnail = generate_thumbnail(&img, 50, 50).unwrap();
        assert_eq!(thumbnail.dimensions(), (50, 50));
        
        // Test rectangular thumbnail (landscape)
        let thumbnail = generate_thumbnail(&img, 50, 25).unwrap();
        assert_eq!(thumbnail.dimensions(), (25, 25));
        
        // Test rectangular thumbnail (portrait)
        let thumbnail = generate_thumbnail(&img, 25, 50).unwrap();
        assert_eq!(thumbnail.dimensions(), (25, 25));
    }
}
