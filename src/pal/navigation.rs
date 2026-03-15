// Arrow key navigation via skparagraph line metrics — PAL layer.

use crate::pal::cursor::CursorPosition_pal;
use crate::pal::paragraph_cache::ParagraphCache_pal;
use skia_safe::Point;

/// Move cursor one character to the left.
pub fn move_left(cache: &ParagraphCache_pal, pos: &CursorPosition_pal) -> CursorPosition_pal {
    if pos.offset_in_paragraph > 0 {
        CursorPosition_pal {
            entry_index: pos.entry_index,
            offset_in_paragraph: pos.offset_in_paragraph - 1,
        }
    } else if pos.entry_index > 0 {
        // Move to end of previous paragraph
        let prev_idx = pos.entry_index - 1;
        let prev_len = cache
            .entry(prev_idx)
            .map(|e| e.text_len)
            .unwrap_or(0);
        CursorPosition_pal {
            entry_index: prev_idx,
            offset_in_paragraph: prev_len,
        }
    } else {
        *pos
    }
}

/// Move cursor one character to the right.
pub fn move_right(cache: &ParagraphCache_pal, pos: &CursorPosition_pal) -> CursorPosition_pal {
    let text_len = cache
        .entry(pos.entry_index)
        .map(|e| e.text_len)
        .unwrap_or(0);

    if pos.offset_in_paragraph < text_len {
        CursorPosition_pal {
            entry_index: pos.entry_index,
            offset_in_paragraph: pos.offset_in_paragraph + 1,
        }
    } else if pos.entry_index + 1 < cache.len() {
        // Move to start of next paragraph
        CursorPosition_pal {
            entry_index: pos.entry_index + 1,
            offset_in_paragraph: 0,
        }
    } else {
        *pos
    }
}

/// Move cursor one line up, preserving sticky_x.
pub fn move_up(
    cache: &ParagraphCache_pal,
    pos: &CursorPosition_pal,
    sticky_x: f32,
) -> CursorPosition_pal {
    if let Some(entry) = cache.entry(pos.entry_index) {
        let line_num = entry
            .paragraph
            .get_line_number_at(pos.offset_in_paragraph)
            .unwrap_or(0);

        if line_num > 0 {
            // Move to the previous line within the same paragraph
            let metrics = entry.paragraph.get_line_metrics();
            if let Some(prev_line) = metrics.get(line_num - 1) {
                let local_x = sticky_x - cache.left_padding();
                let target_y = prev_line.baseline as f32 - prev_line.ascent as f32 + 1.0;
                let glyph_pos = entry
                    .paragraph
                    .get_glyph_position_at_coordinate(Point::new(local_x, target_y));
                return CursorPosition_pal {
                    entry_index: pos.entry_index,
                    offset_in_paragraph: (glyph_pos.position as usize).min(entry.text_len),
                };
            }
        }

        // At first line of paragraph — move to last line of previous paragraph
        if pos.entry_index > 0 {
            let prev_idx = pos.entry_index - 1;
            if let Some(prev_entry) = cache.entry(prev_idx) {
                let metrics = prev_entry.paragraph.get_line_metrics();
                if let Some(last_line) = metrics.last() {
                    let local_x = sticky_x - cache.left_padding();
                    let target_y = last_line.baseline as f32 - last_line.ascent as f32 + 1.0;
                    let glyph_pos = prev_entry
                        .paragraph
                        .get_glyph_position_at_coordinate(Point::new(local_x, target_y));
                    return CursorPosition_pal {
                        entry_index: prev_idx,
                        offset_in_paragraph: (glyph_pos.position as usize)
                            .min(prev_entry.text_len),
                    };
                }
            }
        }
    }
    *pos
}

/// Move cursor one line down, preserving sticky_x.
pub fn move_down(
    cache: &ParagraphCache_pal,
    pos: &CursorPosition_pal,
    sticky_x: f32,
) -> CursorPosition_pal {
    if let Some(entry) = cache.entry(pos.entry_index) {
        let line_num = entry
            .paragraph
            .get_line_number_at(pos.offset_in_paragraph)
            .unwrap_or(0);
        let metrics = entry.paragraph.get_line_metrics();
        let line_count = metrics.len();

        if line_num + 1 < line_count {
            // Move to the next line within the same paragraph
            if let Some(next_line) = metrics.get(line_num + 1) {
                let local_x = sticky_x - cache.left_padding();
                let target_y = next_line.baseline as f32 - next_line.ascent as f32 + 1.0;
                let glyph_pos = entry
                    .paragraph
                    .get_glyph_position_at_coordinate(Point::new(local_x, target_y));
                return CursorPosition_pal {
                    entry_index: pos.entry_index,
                    offset_in_paragraph: (glyph_pos.position as usize).min(entry.text_len),
                };
            }
        }

        // At last line of paragraph — move to first line of next paragraph
        let next_idx = pos.entry_index + 1;
        if next_idx < cache.len() {
            if let Some(next_entry) = cache.entry(next_idx) {
                let metrics = next_entry.paragraph.get_line_metrics();
                if let Some(first_line) = metrics.first() {
                    let local_x = sticky_x - cache.left_padding();
                    let target_y = first_line.baseline as f32 - first_line.ascent as f32 + 1.0;
                    let glyph_pos = next_entry
                        .paragraph
                        .get_glyph_position_at_coordinate(Point::new(local_x, target_y));
                    return CursorPosition_pal {
                        entry_index: next_idx,
                        offset_in_paragraph: (glyph_pos.position as usize)
                            .min(next_entry.text_len),
                    };
                }
            }
        }
    }
    *pos
}

/// Move cursor to the start of the current line.
pub fn move_home(cache: &ParagraphCache_pal, pos: &CursorPosition_pal) -> CursorPosition_pal {
    if let Some(entry) = cache.entry(pos.entry_index) {
        let line_num = entry
            .paragraph
            .get_line_number_at(pos.offset_in_paragraph)
            .unwrap_or(0);
        let metrics = entry.paragraph.get_line_metrics();
        if let Some(line) = metrics.get(line_num) {
            return CursorPosition_pal {
                entry_index: pos.entry_index,
                offset_in_paragraph: line.start_index,
            };
        }
    }
    CursorPosition_pal {
        entry_index: pos.entry_index,
        offset_in_paragraph: 0,
    }
}

/// Move cursor to the end of the current line.
pub fn move_end(cache: &ParagraphCache_pal, pos: &CursorPosition_pal) -> CursorPosition_pal {
    if let Some(entry) = cache.entry(pos.entry_index) {
        let line_num = entry
            .paragraph
            .get_line_number_at(pos.offset_in_paragraph)
            .unwrap_or(0);
        let metrics = entry.paragraph.get_line_metrics();
        if let Some(line) = metrics.get(line_num) {
            let end = line.end_excluding_whitespaces.min(entry.text_len);
            return CursorPosition_pal {
                entry_index: pos.entry_index,
                offset_in_paragraph: end,
            };
        }
    }
    *pos
}

