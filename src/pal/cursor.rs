// Cursor and selection rectangle computation.
// TODO: Integrate with skparagraph APIs for production version.

/// Cursor position information.
#[derive(Debug, Clone, Copy)]
pub struct CursorInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Get cursor visual rect for rendering.
pub fn get_cursor_rect(offset: usize) -> CursorInfo {
    CursorInfo {
        x: offset as f32 * 8.4,
        y: 0.0,
        width: 2.0,
        height: 28.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_info() {
        let cursor = get_cursor_rect(5);
        assert_eq!(cursor.width, 2.0);
    }
}
