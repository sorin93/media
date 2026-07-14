use fast_image_resize::{images::Image, FilterType, PixelType, ResizeAlg, Resizer, ResizeOptions};
use mozjpeg::{ColorSpace, Compress};

use crate::error::{Error, Result};

pub fn generate_blurhash(
    src_pixels: &mut [u8],
    src_width: usize,
    src_height: usize,
    components_x: u32,
    components_y: u32,
) -> Result<String> {
    // Resize to a small size for performance (max 50px dimension)
    // We maintain aspect ratio to ensure the hash represents the image correctly.
    let max_size = 50.0;
    let scale = (max_size / src_width as f32).min(max_size / src_height as f32);
    let scaled_width = (src_width as f32 * scale).round() as u32;
    let scaled_height = (src_height as f32 * scale).round() as u32;

    // Use from_slice_u8 to avoid copying the entire pixel buffer (2MB+)
    let src_image = Image::from_slice_u8(
        src_width as u32,
        src_height as u32,
        src_pixels,
        PixelType::U8x3,
    ).map_err(|_| Error::Internal("Failed to create source image for blurhash".into()))?;

    // The destination image for resizing must have the same pixel type as the source.
    let mut scaled_image = Image::new(scaled_width, scaled_height, PixelType::U8x3);

    let mut resizer = Resizer::new();
    resizer.resize(&src_image, &mut scaled_image, &ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Bilinear)))
        .map_err(|e| Error::Internal(format!("Blurhash resize failed: {}", e)))?;

    // blurhash::encode expects RGBA pixels. We need to convert our resized RGB image.
    let rgb_pixels = scaled_image.buffer();
    let mut rgba_pixels = Vec::with_capacity(rgb_pixels.len() / 3 * 4);
    for chunk in rgb_pixels.chunks_exact(3) {
        rgba_pixels.push(chunk[0]); // R
        rgba_pixels.push(chunk[1]); // G
        rgba_pixels.push(chunk[2]); // B
        rgba_pixels.push(255);      // A
    }

    let hash = blurhash::encode(components_x, components_y, scaled_width, scaled_height, &rgba_pixels)
        .map_err(|e| Error::Internal(format!("Blurhash encoding failed: {:?}", e)))?;

    Ok(hash)
}

// Resizes, center-crops, and encodes an image into a new JPEG.
pub fn resize_and_encode_jpeg(
    src_pixels: &mut [u8],
    src_width: usize,
    src_height: usize,
    dst_width: u32,
    dst_height: u32,
    quality: u8,
) -> Result<Vec<u8>> {
    // Calculate scale to "cover" target dimensions, maintaining aspect ratio.
    let scale = (dst_width as f32 / src_width as f32)
        .max(dst_height as f32 / src_height as f32);

    let scaled_width = (src_width as f32 * scale).round() as u32;
    let scaled_height = (src_height as f32 * scale).round() as u32;

    // Step 1: Resize to cover dimensions
    let src_image = Image::from_slice_u8(
        src_width as u32,
        src_height as u32,
        src_pixels,
        PixelType::U8x3,
    ).map_err(|_| Error::Internal("Failed to create source image for resize".into()))?;

    let mut scaled_image = Image::new(
        scaled_width,
        scaled_height,
        PixelType::U8x3
    );

    let mut resizer = Resizer::new();
    resizer.resize(&src_image, &mut scaled_image, &ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Lanczos3)))
        .map_err(|e| Error::Internal(format!("Image resize failed: {}", e)))?;

    // Step 2: Center crop to exact target dimensions
    let crop_x = (scaled_width - dst_width) / 2;
    let crop_y = (scaled_height - dst_height) / 2;

    let cropped_pixels = crop_center(&scaled_image, crop_x, crop_y, dst_width, dst_height);

    // Step 3: Encode the cropped pixels as JPEG
    encode_jpeg(&cropped_pixels, dst_width as usize, dst_height as usize, quality)
}

// Manually crops the center of an image buffer.
pub fn crop_center(
    img: &Image,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let src_width = img.width() as usize;
    // Assuming PixelType::U8x3
    let bytes_per_pixel = 3;
    let src_stride = src_width * bytes_per_pixel;
    let dst_stride = width as usize * bytes_per_pixel;

    let mut cropped = Vec::with_capacity(height as usize * dst_stride);

    let buffer = img.buffer();
    for row in y..(y + height) {
        let src_offset = (row as usize * src_stride) + (x as usize * bytes_per_pixel);
        cropped.extend_from_slice(&buffer[src_offset..src_offset + dst_stride]);
    }

    cropped
}

// Encodes a raw RGB pixel buffer into a JPEG.
pub fn encode_jpeg(
    pixels: &[u8],
    width: usize,
    height: usize,
    quality: u8,
) -> Result<Vec<u8>> {
    let mut comp = Compress::new(ColorSpace::JCS_RGB);
    comp.set_size(width, height);
    comp.set_quality(quality as f32);

    let mut output = Vec::new();
    let mut compressor = comp.start_compress(&mut output)
        .map_err(|e| Error::Internal(format!("JPEG compression failed to start: {}", e)))?;

    compressor.write_scanlines(pixels)
        .map_err(|e| Error::Internal(format!("JPEG compression failed to write scanlines: {}", e)))?;

    compressor.finish()
        .map_err(|e| Error::Internal(format!("JPEG compression failed to finish: {}", e)))?;

    Ok(output)
}