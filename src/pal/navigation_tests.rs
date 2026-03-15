// Tests for arrow key navigation.

#[cfg(test)]
mod tests {
    use crate::core::document::Document_core;
    use crate::pal::cursor::CursorPosition_pal;
    use crate::pal::navigation::{move_left, move_right};
    use crate::pal::paragraph_cache::ParagraphCache_pal;

    #[test]
    fn test_move_left_right() {
        let doc = Document_core::from_markdown("Hello\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        let pos = CursorPosition_pal {
            entry_index: 0,
            offset_in_paragraph: 2,
        };
        let left = move_left(&cache, &pos);
        assert_eq!(left.offset_in_paragraph, 1);
        let right = move_right(&cache, &pos);
        assert_eq!(right.offset_in_paragraph, 3);
    }

    #[test]
    fn test_move_left_at_start() {
        let doc = Document_core::from_markdown("Hello\n\nWorld\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        let pos = CursorPosition_pal {
            entry_index: 1,
            offset_in_paragraph: 0,
        };
        let left = move_left(&cache, &pos);
        assert_eq!(left.entry_index, 0);
        assert_eq!(left.offset_in_paragraph, 5); // end of "Hello"
    }
}
