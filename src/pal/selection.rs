#![allow(non_camel_case_types)]
// Selection highlighting via skparagraph get_rects_for_range() — PAL layer.

use crate::pal::cursor::CursorPosition_pal;
use crate::pal::paragraph_cache::ParagraphCache_pal;

pub use crate::pal::selection_rects_pal::selection_rects;

/// Selection range as two cursor positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange_pal {
    /// Anchor (where selection started).
    pub anchor: CursorPosition_pal,
    /// Head (where selection ends / cursor is).
    pub head: CursorPosition_pal,
}

/// Selection rectangle for rendering (absolute pixel coordinates).
#[derive(Debug, Clone, Copy)]
pub struct SelectionRect_pal {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl SelectionRange_pal {
    /// Create a new selection range.
    pub fn new(anchor: CursorPosition_pal, head: CursorPosition_pal) -> Self {
        Self { anchor, head }
    }

    /// Check if the selection is empty (anchor == head).
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// Get the normalized (start, end) with start <= end.
    pub fn normalized(&self) -> (CursorPosition_pal, CursorPosition_pal) {
        if self.anchor.entry_index < self.head.entry_index
            || (self.anchor.entry_index == self.head.entry_index
                && self.anchor.offset_in_paragraph <= self.head.offset_in_paragraph)
        {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }
}

/// Get the word boundary range at a cursor position (for double-click select).
pub fn word_range_at(
    cache: &ParagraphCache_pal,
    pos: &CursorPosition_pal,
) -> Option<SelectionRange_pal> {
    let entry = cache.entry(pos.entry_index)?;
    let text = &entry.plain_text;
    let offset = pos.offset_in_paragraph.min(text.len());

    // Find word boundaries by scanning for non-alphanumeric characters
    let bytes = text.as_bytes();

    let start = (0..offset)
        .rev()
        .find(|&i| !bytes[i].is_ascii_alphanumeric() && bytes[i] != b'_')
        .map(|i| i + 1)
        .unwrap_or(0);

    let end = (offset..text.len())
        .find(|&i| !bytes[i].is_ascii_alphanumeric() && bytes[i] != b'_')
        .unwrap_or(text.len());

    Some(SelectionRange_pal::new(
        CursorPosition_pal {
            entry_index: pos.entry_index,
            offset_in_paragraph: start,
        },
        CursorPosition_pal {
            entry_index: pos.entry_index,
            offset_in_paragraph: end,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::document::Document_core;

    #[test]
    fn test_empty_selection() {
        let pos = CursorPosition_pal {
            entry_index: 0,
            offset_in_paragraph: 5,
        };
        let sel = SelectionRange_pal::new(pos, pos);
        assert!(sel.is_empty());
    }

    #[test]
    fn test_selection_rects_basic() {
        let doc = Document_core::from_markdown("Hello World\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        let sel = SelectionRange_pal::new(
            CursorPosition_pal {
                entry_index: 0,
                offset_in_paragraph: 0,
            },
            CursorPosition_pal {
                entry_index: 0,
                offset_in_paragraph: 5,
            },
        );
        let rects = selection_rects(&cache, &sel);
        assert!(!rects.is_empty());
        assert!(rects[0].width > 0.0);
    }

    #[test]
    fn test_word_range() {
        let doc = Document_core::from_markdown("Hello World\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        let pos = CursorPosition_pal {
            entry_index: 0,
            offset_in_paragraph: 2, // inside "Hello"
        };
        let range = word_range_at(&cache, &pos).unwrap();
        assert_eq!(range.anchor.offset_in_paragraph, 0);
        assert_eq!(range.head.offset_in_paragraph, 5);
    }
}
