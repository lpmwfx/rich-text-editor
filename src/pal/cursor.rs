// Cursor and selection rectangle computation via skparagraph APIs.

use skia_safe::{textlayout::Paragraph, Point};

/// Cursor position information (visual rendering bounds) — PAL layer.
#[derive(Debug, Clone, Copy)]
pub struct CursorInfo_pal {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Cursor offset information (document position) — PAL layer.
#[derive(Debug, Clone, Copy)]
pub struct CursorOffset_pal {
    pub offset: usize,
    pub line: usize,
}


/// Cursor width and height constants (pixels)
const CURSOR_WIDTH: f32 = 2.0;
const CURSOR_HEIGHT: f32 = 20.0;
/// Character width in pixels (monospace font approximation)
const CHAR_WIDTH: f32 = 8.4;

/// Map click coordinates to document offset using skparagraph API.
///
/// Uses paragraph.getGlyphPositionAtCoordinate() to find the character position
/// at the given pixel coordinates within the paragraph.
pub fn coordinate_to_offset(paragraph: &Paragraph, click_x: f32, click_y: f32) -> CursorOffset_pal {
    // Get glyph position at the click coordinates
    // Note: getGlyphPositionAtCoordinate takes a Point and returns PositionWithAffinity
    let click_point = Point::new(click_x, click_y);
    let position = paragraph.get_glyph_position_at_coordinate(click_point);

    // Extract the character offset from the position
    // The offset represents the character index in the text
    let offset = position.position as usize;

    // For now, line number is derived from offset (would need proper line tracking in real impl)
    let line = (click_y / CURSOR_HEIGHT).floor() as usize;

    CursorOffset_pal { offset, line }
}

/// Get cursor visual rect for rendering at a given offset.
pub fn get_cursor_rect(offset: usize) -> CursorInfo_pal {
    CursorInfo_pal {
        x: offset as f32 * CHAR_WIDTH,
        y: 0.0,
        width: CURSOR_WIDTH,
        height: CURSOR_HEIGHT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_info() {
        let cursor = get_cursor_rect(5);
        assert_eq!(cursor.width, CURSOR_WIDTH);
    }
}
