#![allow(non_camel_case_types)]
// Cursor positioning via skparagraph APIs — PAL layer.

use crate::pal::paragraph_cache::ParagraphCache_pal;
use skia_safe::Point;

pub use crate::pal::cursor_rect_pal::cursor_rect;

/// Cursor position in the document (paragraph index + offset within paragraph).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition_pal {
    /// Index of the paragraph entry in the cache.
    pub entry_index: usize,
    /// Character offset within the paragraph's plain text.
    pub offset_in_paragraph: usize,
}

/// Cursor visual info for rendering (absolute pixel coordinates).
#[derive(Debug, Clone, Copy)]
pub struct CursorInfo_pal {
    /// X position in pixels.
    pub x: f32,
    /// Y position in pixels.
    pub y: f32,
    /// Width of the caret (typically 2px).
    pub width: f32,
    /// Height of the caret.
    pub height: f32,
}

/// Map a click coordinate to a cursor position using skparagraph hit testing.
pub fn hit_test(cache: &ParagraphCache_pal, x: f32, y: f32) -> CursorPosition_pal {
    if let Some((entry_idx, entry)) = cache.paragraph_at_y(y) {
        let local_x = x - cache.left_padding();
        let local_y = y - entry.y_offset;

        let pos = entry
            .paragraph
            .get_glyph_position_at_coordinate(Point::new(local_x, local_y));

        let offset = (pos.position as usize).min(entry.text_len);

        CursorPosition_pal {
            entry_index: entry_idx,
            offset_in_paragraph: offset,
        }
    } else {
        CursorPosition_pal {
            entry_index: 0,
            offset_in_paragraph: 0,
        }
    }
}

/// Convert a CursorPosition_pal to (line, column) for display.
pub fn line_col(cache: &ParagraphCache_pal, pos: &CursorPosition_pal) -> (usize, usize) {
    if let Some(entry) = cache.entry(pos.entry_index) {
        let line = entry
            .paragraph
            .get_line_number_at(pos.offset_in_paragraph)
            .unwrap_or(0);
        (pos.entry_index + line, pos.offset_in_paragraph)
    } else {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::document::Document_core;
    use crate::shared::sizes::CARET_WIDTH_PX;

    #[test]
    fn test_hit_test_basic() {
        let doc = Document_core::from_markdown("# Hello\n\nWorld\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        let pos = hit_test(&cache, 50.0, 20.0);
        assert_eq!(pos.entry_index, 0);
    }

    #[test]
    fn test_cursor_rect_basic() {
        let doc = Document_core::from_markdown("Hello\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        let pos = CursorPosition_pal {
            entry_index: 0,
            offset_in_paragraph: 0,
        };
        let rect = cursor_rect(&cache, &pos);
        assert_eq!(rect.width, CARET_WIDTH_PX);
        assert!(rect.height > 0.0);
    }
}
