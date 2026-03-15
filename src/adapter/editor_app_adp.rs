#![allow(non_camel_case_types)]
// EditorApp — shared editor state accessible from Slint callbacks.

use crate::adapter::editor_state_adp::EditorState_adp;
use crate::core::editor::commands::{DeleteRangeCommand, InsertTextCommand_core};
use crate::pal::cursor::{self, CursorPosition_pal};
use crate::pal::paragraph_cache::ParagraphCache_pal;
use crate::pal::selection::{self, SelectionRange_pal};
use crate::pal::render;

/// Editor canvas dimensions.
pub const EDITOR_WIDTH: u32 = 868; // 900 - some padding
/// Editor canvas height.
pub const EDITOR_HEIGHT: u32 = 528; // 600 - toolbar - statusbar

/// Shared editor state accessible from Slint callbacks.
pub struct EditorApp_adp {
    pub editor_state: EditorState_adp,
    pub cache: ParagraphCache_pal,
    pub cursor_pos: CursorPosition_pal,
    pub selection: Option<SelectionRange_pal>,
    pub scroll_y: f32,
    pub sticky_x: f32,
}

impl EditorApp_adp {
    /// Create a new editor app from markdown source.
    pub fn new(markdown: &str) -> Self {
        let editor_state = EditorState_adp::from_markdown(markdown);
        let md = editor_state.document.to_markdown();
        let cache = ParagraphCache_pal::rebuild(&editor_state.document.blocks, &md, EDITOR_WIDTH as f32);
        Self {
            editor_state,
            cache,
            cursor_pos: CursorPosition_pal {
                entry_index: 0,
                offset_in_paragraph: 0,
            },
            selection: None,
            scroll_y: 0.0,
            sticky_x: 0.0,
        }
    }

    /// Rebuild the paragraph cache after document changes.
    pub fn rebuild_cache(&mut self) {
        let md = self.editor_state.document.to_markdown();
        self.cache = ParagraphCache_pal::rebuild(&self.editor_state.document.blocks, &md, EDITOR_WIDTH as f32);
        // Clamp cursor to valid range after rebuild
        if self.cursor_pos.entry_index >= self.cache.len() {
            self.cursor_pos.entry_index = self.cache.len().saturating_sub(1);
        }
        if let Some(entry) = self.cache.entry(self.cursor_pos.entry_index) {
            if self.cursor_pos.offset_in_paragraph > entry.text_len {
                self.cursor_pos.offset_in_paragraph = entry.text_len;
            }
        }
    }

    /// Render the current state to a Slint Image.
    pub fn render_frame(&self) -> slint::Image {
        let cursor_info = cursor::cursor_rect(&self.cache, &self.cursor_pos);
        let sel_rects = self
            .selection
            .as_ref()
            .map(|sel| selection::selection_rects(&self.cache, sel))
            .unwrap_or_default();

        render::render_to_image(
            &self.cache,
            EDITOR_WIDTH,
            EDITOR_HEIGHT,
            self.scroll_y,
            Some(&cursor_info),
            &sel_rects,
        )
    }

    /// Get (line, col) for status bar display.
    pub fn cursor_line_col(&self) -> (i32, i32) {
        let (line, col) = cursor::line_col(&self.cache, &self.cursor_pos);
        (line as i32, col as i32)
    }

    /// Convert cursor position to a markdown byte offset.
    pub fn cursor_to_md_offset(&self) -> usize {
        self.cache
            .plain_to_markdown_offset(self.cursor_pos.entry_index, self.cursor_pos.offset_in_paragraph)
    }

    /// After a markdown mutation, try to place cursor at the given markdown offset.
    pub fn set_cursor_from_md_offset(&mut self, md_offset: usize) {
        if let Some((entry_idx, plain_offset)) = self.cache.markdown_to_plain_offset(md_offset) {
            self.cursor_pos = CursorPosition_pal {
                entry_index: entry_idx,
                offset_in_paragraph: plain_offset,
            };
        }
    }

    /// Update sticky_x from current cursor position.
    pub fn update_sticky_x(&mut self) {
        let rect = cursor::cursor_rect(&self.cache, &self.cursor_pos);
        self.sticky_x = rect.x;
    }

    /// Ensure the cursor is visible by adjusting scroll_y.
    pub fn auto_scroll_to_cursor(&mut self) {
        let rect = cursor::cursor_rect(&self.cache, &self.cursor_pos);
        let cursor_top = rect.y - self.scroll_y;
        let cursor_bottom = cursor_top + rect.height;
        let viewport = EDITOR_HEIGHT as f32;

        if cursor_top < 0.0 {
            self.scroll_y = rect.y;
        } else if cursor_bottom > viewport {
            self.scroll_y = rect.y + rect.height - viewport;
        }

        self.clamp_scroll();
    }

    /// Clamp scroll_y to valid range.
    pub fn clamp_scroll(&mut self) {
        let max_scroll = (self.cache.total_height() - EDITOR_HEIGHT as f32).max(0.0);
        self.scroll_y = self.scroll_y.clamp(0.0, max_scroll);
    }

    /// Insert text at cursor, handling selection replacement.
    pub fn insert_text(&mut self, text: &str) {
        if let Some(sel) = self.selection.take() {
            self.delete_selection(&sel);
        }

        let md_offset = self.cursor_to_md_offset();
        let cmd = InsertTextCommand_core {
            offset: md_offset,
            text: text.to_string(),
        };
        if self.editor_state.apply(Box::new(cmd)).is_ok() {
            self.rebuild_cache();
            self.set_cursor_from_md_offset(md_offset + text.len());
            self.update_sticky_x();
            self.auto_scroll_to_cursor();
        }
    }

    /// Delete the character before the cursor (backspace).
    pub fn backspace(&mut self) {
        if let Some(sel) = self.selection.take() {
            self.delete_selection(&sel);
            return;
        }

        let md_offset = self.cursor_to_md_offset();
        if md_offset == 0 {
            return;
        }

        let md = self.editor_state.to_markdown();
        let prev_offset = md[..md_offset]
            .char_indices()
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(0);

        let cmd = DeleteRangeCommand::new(prev_offset, md_offset);
        if self.editor_state.apply(Box::new(cmd)).is_ok() {
            self.rebuild_cache();
            self.set_cursor_from_md_offset(prev_offset);
            self.update_sticky_x();
            self.auto_scroll_to_cursor();
        }
    }

    /// Delete the character after the cursor.
    pub fn delete_forward(&mut self) {
        if let Some(sel) = self.selection.take() {
            self.delete_selection(&sel);
            return;
        }

        let md_offset = self.cursor_to_md_offset();
        let md = self.editor_state.to_markdown();
        if md_offset >= md.len() {
            return;
        }

        let next_offset = md[md_offset..]
            .char_indices()
            .nth(1)
            .map(|(i, _)| md_offset + i)
            .unwrap_or(md.len());

        let cmd = DeleteRangeCommand::new(md_offset, next_offset);
        if self.editor_state.apply(Box::new(cmd)).is_ok() {
            self.rebuild_cache();
            self.set_cursor_from_md_offset(md_offset);
            self.update_sticky_x();
            self.auto_scroll_to_cursor();
        }
    }

    /// Delete the selected text range.
    pub fn delete_selection(&mut self, sel: &SelectionRange_pal) {
        let (start, end) = sel.normalized();
        let start_md = self
            .cache
            .plain_to_markdown_offset(start.entry_index, start.offset_in_paragraph);
        let end_md = self
            .cache
            .plain_to_markdown_offset(end.entry_index, end.offset_in_paragraph);

        if start_md < end_md {
            let cmd = DeleteRangeCommand::new(start_md, end_md);
            if self.editor_state.apply(Box::new(cmd)).is_ok() {
                self.rebuild_cache();
                self.set_cursor_from_md_offset(start_md);
                self.update_sticky_x();
                self.auto_scroll_to_cursor();
            }
        }
    }

    /// Extend selection to a new cursor position.
    pub fn extend_selection(&mut self, new_pos: CursorPosition_pal) {
        let anchor = if let Some(sel) = &self.selection {
            sel.anchor
        } else {
            self.cursor_pos
        };
        self.selection = Some(SelectionRange_pal::new(anchor, new_pos));
        self.cursor_pos = new_pos;
    }

    /// Handle a click at (x, y) — positions cursor and clears selection.
    pub fn handle_click(&mut self, x: f32, y: f32) {
        let adjusted_y = y + self.scroll_y;
        self.cursor_pos = cursor::hit_test(&self.cache, x, adjusted_y);
        self.selection = None;
        self.update_sticky_x();
    }
}
