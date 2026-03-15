#![allow(non_camel_case_types)]
// Cursor rect computation — extracted from cursor.rs — PAL layer.

use crate::pal::cursor::{CursorInfo_pal, CursorPosition_pal};
use crate::pal::paragraph_cache::ParagraphCache_pal;
use crate::shared::sizes::{CARET_WIDTH_PX, FALLBACK_LINE_HEIGHT_PX, FALLBACK_Y_PX};

/// Get the visual cursor rect for a cursor position.
pub fn cursor_rect(cache: &ParagraphCache_pal, pos: &CursorPosition_pal) -> CursorInfo_pal {
    if let Some(entry) = cache.entry(pos.entry_index) {
        let offset = pos.offset_in_paragraph;

        if offset < entry.text_len {
            let boxes = entry.paragraph.get_rects_for_range(
                offset..offset + 1,
                skia_safe::textlayout::RectHeightStyle::Max,
                skia_safe::textlayout::RectWidthStyle::Tight,
            );
            if let Some(first) = boxes.first() {
                return CursorInfo_pal {
                    x: first.rect.left + cache.left_padding(),
                    y: entry.y_offset + first.rect.top,
                    width: CARET_WIDTH_PX,
                    height: first.rect.height(),
                };
            }
        }

        if offset > 0 {
            let boxes = entry.paragraph.get_rects_for_range(
                (offset - 1)..offset,
                skia_safe::textlayout::RectHeightStyle::Max,
                skia_safe::textlayout::RectWidthStyle::Tight,
            );
            if let Some(last) = boxes.last() {
                return CursorInfo_pal {
                    x: last.rect.right + cache.left_padding(),
                    y: entry.y_offset + last.rect.top,
                    width: CARET_WIDTH_PX,
                    height: last.rect.height(),
                };
            }
        }

        // Fallback: beginning of empty paragraph
        let line_height = entry.paragraph.height().max(FALLBACK_LINE_HEIGHT_PX);
        CursorInfo_pal {
            x: cache.left_padding(),
            y: entry.y_offset,
            width: CARET_WIDTH_PX,
            height: line_height,
        }
    } else {
        CursorInfo_pal {
            x: cache.left_padding(),
            y: FALLBACK_Y_PX,
            width: CARET_WIDTH_PX,
            height: FALLBACK_LINE_HEIGHT_PX,
        }
    }
}
