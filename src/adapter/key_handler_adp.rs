// Keyboard input handling for the editor.

use crate::pal::cursor::CursorPosition_pal;
use crate::pal::selection::SelectionRange_pal;

use crate::adapter::editor_app_adp::EditorApp_adp;

impl EditorApp_adp {
    /// Handle a key press. Returns true if the key was consumed.
    pub fn handle_key(&mut self, key: &str, shift: bool, ctrl: bool, _alt: bool) -> bool {
        if ctrl {
            if let Some(consumed) = self.handle_ctrl_key(key) {
                return consumed;
            }
        }

        if let Some(consumed) = self.handle_nav_key(key, shift) {
            return consumed;
        }

        if key.len() == 1 || (key.len() > 1 && !key.starts_with('\u{f7}') && !key.starts_with('\u{f0}')) {
            let ch = key.chars().next().unwrap_or('\0');
            if ch.is_control() {
                return false;
            }
            self.insert_text(key);
            return true;
        }

        false
    }

    /// Handle Ctrl+key shortcuts.
    fn handle_ctrl_key(&mut self, key: &str) -> Option<bool> {
        match key {
            "z" => {
                if self.editor_state.undo().is_ok() {
                    self.rebuild_cache();
                    self.auto_scroll_to_cursor();
                }
                Some(true)
            }
            "y" => {
                if self.editor_state.redo().is_ok() {
                    self.rebuild_cache();
                    self.auto_scroll_to_cursor();
                }
                Some(true)
            }
            "a" => {
                if self.cache.len() > 0 {
                    let last_idx = self.cache.len() - 1;
                    let last_len = self.cache.entry(last_idx).map(|e| e.text_len).unwrap_or(0);
                    self.selection = Some(SelectionRange_pal::new(
                        CursorPosition_pal { entry_index: 0, offset_in_paragraph: 0 },
                        CursorPosition_pal { entry_index: last_idx, offset_in_paragraph: last_len },
                    ));
                }
                Some(true)
            }
            _ => None,
        }
    }

    /// Apply a cursor move: extend selection if shift held, otherwise clear and move.
    pub fn apply_cursor_move(&mut self, new_pos: CursorPosition_pal, shift: bool, update_sticky: bool) {
        if shift {
            self.extend_selection(new_pos);
        } else {
            self.selection = None;
            self.cursor_pos = new_pos;
        }
        if update_sticky {
            self.update_sticky_x();
        }
        self.auto_scroll_to_cursor();
    }
}
