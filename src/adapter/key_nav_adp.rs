// Navigation key handling — arrow keys, home/end, backspace, delete, enter.

use crate::pal::cursor::CursorPosition_pal;
use crate::pal::navigation;
use crate::pal::paragraph_cache::ParagraphCache_pal;
use crate::adapter::editor_app_adp::EditorApp_adp;

/// Resolve an arrow/home/end key to a new cursor position.
fn resolve_nav_position(
    key: &str,
    cache: &ParagraphCache_pal,
    cursor_pos: &CursorPosition_pal,
    sticky_x: f32,
) -> Option<(CursorPosition_pal, bool)> {
    match key {
        "\u{f702}" => Some((navigation::move_left(cache, cursor_pos), true)),
        "\u{f703}" => Some((navigation::move_right(cache, cursor_pos), true)),
        "\u{f700}" => Some((navigation::move_up(cache, cursor_pos, sticky_x), false)),
        "\u{f701}" => Some((navigation::move_down(cache, cursor_pos, sticky_x), false)),
        "\u{f729}" => Some((navigation::move_home(cache, cursor_pos), true)),
        "\u{f72b}" => Some((navigation::move_end(cache, cursor_pos), true)),
        _ => None,
    }
}

impl EditorApp_adp {
    /// Handle arrow keys and navigation. Returns `Some(true)` if consumed, `None` if not handled.
    pub fn handle_nav_key(&mut self, key: &str, shift: bool) -> Option<bool> {
        if let Some((pos, update_sticky)) = resolve_nav_position(key, &self.cache, &self.cursor_pos, self.sticky_x) {
            self.apply_cursor_move(pos, shift, update_sticky);
            return Some(true);
        }

        match key {
            "\u{8}" | "\u{f728}" => {
                self.backspace();
                Some(true)
            }
            "\u{7f}" => {
                self.delete_forward();
                Some(true)
            }
            "\n" | "\r" => {
                self.insert_text("\n");
                Some(true)
            }
            _ => None,
        }
    }
}
