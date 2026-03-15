// Tests for paragraph building.

#[cfg(test)]
mod tests {
    use crate::core::document::{Document_core, Inline};
    use crate::pal::paragraph::{build_paragraphs, inline_to_string};

    #[test]
    fn test_build_paragraphs() {
        let doc = Document_core::from_markdown("# Hello\n\nWorld");
        let paragraphs = build_paragraphs(&doc.blocks, 800.0).expect("should build");
        assert!(!paragraphs.is_empty());
        assert_eq!(paragraphs[0].block_type, "heading-1");
    }

    #[test]
    fn test_inline_to_string() {
        let inlines = vec![
            Inline::Text("Hello ".to_string()),
            Inline::Bold(vec![Inline::Text("bold".to_string())]),
        ];
        let text = inline_to_string(&inlines);
        assert_eq!(text, "Hello bold");
    }
}
