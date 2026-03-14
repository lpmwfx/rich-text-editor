// Selection highlighting via skparagraph getRectsForRange() — PAL layer.

use skia_safe::textlayout::Paragraph;

/// Selection range (start and end offsets in the document).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange_pal {
    pub start: usize,
    pub end: usize,
}

/// Selection rectangle for rendering (visual bounding box).
#[derive(Debug, Clone, Copy)]
pub struct SelectionRect_pal {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}


/// Selection highlight color constants (ARGB)
const SELECTION_ALPHA: u8 = 100;
const SELECTION_R: u8 = 100;
const SELECTION_G: u8 = 150;
const SELECTION_B: u8 = 255;

impl SelectionRange_pal {
    /// Create a new selection range.
    pub fn new(start: usize, end: usize) -> Self {
        let (start, end) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        SelectionRange_pal { start, end }
    }

    /// Check if range is empty (start == end).
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the length of the selected text.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

/// Get selection rectangles for a text range using skparagraph.
///
/// Uses paragraph.getRectsForRange() to compute the bounding boxes for
/// the selected text, handling multi-line selections correctly.
///
/// **Note**: Placeholder using approximate offset-based calculation. Real implementation
/// pending full skparagraph API integration.
pub fn get_selection_rects(
    _paragraph: &Paragraph,
    range: SelectionRange_pal,
) -> Vec<SelectionRect_pal> {
    if range.is_empty() {
        return vec![];
    }

    const CHAR_WIDTH: f32 = 8.4;
    const MIN_WIDTH: f32 = 2.0;
    const HEIGHT: f32 = 20.0;

    let start_x = range.start as f32 * CHAR_WIDTH;
    let end_x = range.end as f32 * CHAR_WIDTH;
    let width = (end_x - start_x).max(MIN_WIDTH);

    vec![SelectionRect_pal {
        x: start_x,
        y: 0.0,
        width,
        height: HEIGHT,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_range_empty() {
        let range = SelectionRange::new(5, 5);
        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
    }

    #[test]
    fn test_selection_range_order() {
        let range1 = SelectionRange::new(5, 10);
        let range2 = SelectionRange::new(10, 5);
        assert_eq!(range1.start, range2.start);
        assert_eq!(range1.end, range2.end);
    }

    #[test]
    fn test_selection_range_len() {
        let range = SelectionRange::new(5, 15);
        assert_eq!(range.len(), 10);
    }
}
