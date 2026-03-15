// Render Skia Paragraphs to in-memory SharedPixelBuffer for Slint display.

use crate::pal::cursor::CursorInfo_pal;
use crate::pal::paragraph_cache::ParagraphCache_pal;
use crate::pal::selection::SelectionRect_pal;
use crate::shared::colors::{BG_ARGB, CARET_ARGB, SEL_ARGB};
use crate::shared::sizes::ROW_BYTES_CHANNELS;
use skia_safe::{surfaces, Color, ColorType, ImageInfo, Paint, Point, Rect};

/// Paint all editor content (selections, paragraphs, cursor) onto a canvas.
fn paint_frame(
    canvas: &skia_safe::Canvas,
    cache: &ParagraphCache_pal,
    height: f32,
    scroll_y: f32,
    cursor: Option<&CursorInfo_pal>,
    selection_rects: &[SelectionRect_pal],
) {
    canvas.clear(Color::from_argb(BG_ARGB.0, BG_ARGB.1, BG_ARGB.2, BG_ARGB.3));

    if !selection_rects.is_empty() {
        let mut sel_paint = Paint::default();
        sel_paint.set_color(Color::from_argb(SEL_ARGB.0, SEL_ARGB.1, SEL_ARGB.2, SEL_ARGB.3));
        sel_paint.set_anti_alias(true);
        for sel in selection_rects {
            canvas.draw_rect(Rect::from_xywh(sel.x, sel.y - scroll_y, sel.width, sel.height), &sel_paint);
        }
    }

    let left_pad = cache.left_padding();
    for entry in cache.entries() {
        let paint_y = entry.y_offset - scroll_y;
        if paint_y + entry.paragraph.height() < 0.0 { continue; }
        if paint_y > height { break; }
        entry.paragraph.paint(canvas, Point::new(left_pad, paint_y));
    }

    if let Some(cur) = cursor {
        let mut caret_paint = Paint::default();
        caret_paint.set_color(Color::from_argb(CARET_ARGB.0, CARET_ARGB.1, CARET_ARGB.2, CARET_ARGB.3));
        caret_paint.set_anti_alias(true);
        canvas.draw_rect(Rect::from_xywh(cur.x, cur.y - scroll_y, cur.width, cur.height), &caret_paint);
    }
}

/// Render the paragraph cache to a Slint Image via SharedPixelBuffer.
pub fn render_to_image(
    cache: &ParagraphCache_pal,
    width: u32,
    height: u32,
    scroll_y: f32,
    cursor: Option<&CursorInfo_pal>,
    selection_rects: &[SelectionRect_pal],
) -> slint::Image {
    let mut buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
    render_into_buffer(&mut buffer, cache, height, scroll_y, cursor, selection_rects);
    slint::Image::from_rgba8_premultiplied(buffer)
}

/// Render editor content into a pre-allocated pixel buffer.
fn render_into_buffer(
    buffer: &mut slint::SharedPixelBuffer<slint::Rgba8Pixel>,
    cache: &ParagraphCache_pal,
    height: u32,
    scroll_y: f32,
    cursor: Option<&CursorInfo_pal>,
    selection_rects: &[SelectionRect_pal],
) {
    let (width, height_px) = (buffer.width(), height);
    let pixels = buffer.make_mut_bytes();

    let image_info = ImageInfo::new(
        (width as i32, height_px as i32),
        ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );
    let row_bytes = width as usize * ROW_BYTES_CHANNELS;

    let Some(mut surface) = surfaces::wrap_pixels(&image_info, pixels, Some(row_bytes), None) else {
        return;
    };

    paint_frame(surface.canvas(), cache, height_px as f32, scroll_y, cursor, selection_rects);
}
