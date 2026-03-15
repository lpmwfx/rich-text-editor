#![allow(non_camel_case_types)]
// Selection rect computation — extracted from selection.rs — PAL layer.

use crate::pal::paragraph_cache::ParagraphCache_pal;
use crate::pal::selection::{SelectionRange_pal, SelectionRect_pal};

/// Get selection rectangles for a selection range using skparagraph APIs.
pub fn selection_rects(
    cache: &ParagraphCache_pal,
    sel: &SelectionRange_pal,
) -> Vec<SelectionRect_pal> {
    if sel.is_empty() {
        return vec![];
    }

    let (start, end) = sel.normalized();
    let mut rects = Vec::new();
    let left_pad = cache.left_padding();

    for entry_idx in start.entry_index..=end.entry_index {
        let entry = match cache.entry(entry_idx) {
            Some(e) => e,
            None => continue,
        };

        // Determine the range within this paragraph
        let range_start = if entry_idx == start.entry_index {
            start.offset_in_paragraph
        } else {
            0
        };
        let range_end = if entry_idx == end.entry_index {
            end.offset_in_paragraph
        } else {
            entry.text_len
        };

        if range_start >= range_end {
            continue;
        }

        // Use skparagraph to get the actual rectangles
        let boxes = entry.paragraph.get_rects_for_range(
            range_start..range_end,
            skia_safe::textlayout::RectHeightStyle::Max,
            skia_safe::textlayout::RectWidthStyle::Tight,
        );

        for text_box in &boxes {
            rects.push(SelectionRect_pal {
                x: text_box.rect.left + left_pad,
                y: entry.y_offset + text_box.rect.top,
                width: text_box.rect.width(),
                height: text_box.rect.height(),
            });
        }
    }

    rects
}
