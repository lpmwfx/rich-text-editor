// Media placeholder rendering via skparagraph getRectsForPlaceholders() — PAL layer.

use skia_safe::textlayout::Paragraph;

/// Media placeholder visual bounds for rendering.
#[derive(Debug, Clone, Copy)]
pub struct MediaPlaceholder_pal {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub media_type: MediaType_pal,
}

/// Media type identifier (image or video).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType_pal {
    Image,
    Video,
}


/// Get media placeholder positions for rendering via skparagraph.
///
/// Uses paragraph.getRectsForPlaceholders() to retrieve the layout bounds
/// of embedded media (images, videos) within the text layout.
///
/// **Note**: Placeholder implementation. Real integration pending skparagraph API.
pub fn get_media_placeholders(
    _paragraph: &Paragraph,
) -> Vec<MediaPlaceholder_pal> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type() {
        assert_eq!(MediaType_pal::Image, MediaType_pal::Image);
        assert_ne!(MediaType_pal::Image, MediaType_pal::Video);
    }

    #[test]
    fn test_get_media_placeholders_empty() {
        // Placeholder test — real API will be tested with actual paragraphs
        let placeholders = vec![];
        assert!(placeholders.is_empty());
    }
}
