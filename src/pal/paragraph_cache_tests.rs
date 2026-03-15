// Tests for ParagraphCache_pal.

#[cfg(test)]
mod tests {
    use crate::core::document::Document_core;
    use crate::pal::paragraph_cache::ParagraphCache_pal;

    #[test]
    fn test_cache_from_sample_doc() {
        let doc = Document_core::from_markdown("# Hello\n\nWorld\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        assert!(cache.total_height() > 0.0);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_paragraph_at_y() {
        let doc = Document_core::from_markdown("# Hello\n\nWorld\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        let lookup_result = cache.paragraph_at_y(20.0);
        assert!(lookup_result.is_some());
        let (idx, _) = lookup_result.unwrap();
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_plain_text_content() {
        let doc = Document_core::from_markdown("# Hello\n\nWorld\n");
        let cache = ParagraphCache_pal::rebuild(&doc.blocks, &doc.to_markdown(), 800.0);
        assert_eq!(cache.entry(0).unwrap().plain_text, "Hello");
        assert_eq!(cache.entry(1).unwrap().plain_text, "World");
    }
}
