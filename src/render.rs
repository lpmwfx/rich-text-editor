// Render Skia Paragraphs to image files for Slint display.

use skia_safe::{Color, EncodedImageFormat, Point, Surface};
use std::sync::{Arc, Mutex};
use skia_safe::textlayout::Paragraph;
use std::fs;
use std::path::Path;

/// Render paragraphs to PNG file and return file path.
///
/// Takes built Skia Paragraphs, paints them on a raster surface,
/// exports as PNG to a temporary file, and returns the path for Slint to display.
pub fn render_paragraphs_to_file(
    paragraphs: &[Arc<Mutex<Paragraph>>],
    width: f32,
    height: f32,
    output_path: &Path,
) -> anyhow::Result<()> {
    // Create raster surface (CPU rendering)
    let mut surface = Surface::new_raster_n32_premul((width as i32, height as i32))
        .ok_or_else(|| anyhow::anyhow!("Failed to create Skia surface"))?;

    let canvas = surface.canvas();

    // Clear background (Catppuccin Mocha: #1e1e2e)
    canvas.clear(Color::from_argb(255, 30, 30, 46));

    // Paint each paragraph
    let mut y_offset = 16.0;
    for para_mutex in paragraphs {
        if let Ok(para) = para_mutex.lock() {
            // Get paragraph dimensions
            let para_height = para.height();

            // Paint the paragraph at current y offset
            para.paint(canvas, Point::new(16.0, y_offset));

            y_offset += para_height + 8.0; // 8px spacing between paragraphs
        }
    }

    // Export as PNG file
    let image = surface.image_snapshot();
    let data = image
        .encode_to_data(EncodedImageFormat::PNG)
        .ok_or_else(|| anyhow::anyhow!("Failed to encode image"))?;

    fs::write(output_path, data.as_bytes())?;
    Ok(())
}
