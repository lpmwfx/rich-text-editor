#![allow(non_camel_case_types)]
/// Markdown to AST parser using pulldown-cmark.
pub mod parser;
/// Block-level parse helpers.
pub mod parser_blocks;
/// Inline element parsing helpers.
pub mod parser_inline;
/// List and image parse helpers.
pub mod parser_list_image;
/// Parser tests.
#[cfg(test)]
mod parser_tests;
/// AST to Markdown string serializer.
pub mod serializer;
/// Block serialization helpers.
pub mod serializer_block;
/// Serializer tests.
#[cfg(test)]
mod serializer_tests;
/// YAML frontmatter parsing and serialization.
pub mod frontmatter;

// Re-export shared document types so existing `crate::core::document::Block` imports keep working.
pub use crate::shared::document_types_x::{
    Block_x as Block, ByteRange_x as ByteRange, Frontmatter_x as Frontmatter,
    Inline_x as Inline,
};

/// A complete document — sequence of blocks with optional frontmatter.
#[derive(Debug, Clone)]
pub struct Document_core {
    /// YAML frontmatter metadata.
    pub frontmatter: Option<Frontmatter>,
    /// Ordered sequence of content blocks.
    pub blocks: Vec<Block>,
}

impl Document_core {
    /// Create an empty document with no frontmatter.
    pub fn new() -> Self {
        Self {
            frontmatter: None,
            blocks: Vec::new(),
        }
    }

    /// Serialize the document to a Markdown string.
    pub fn to_markdown(&self) -> String {
        serializer::serialize(self)
    }

    /// Parse a Markdown string into a Document.
    pub fn from_markdown(input: &str) -> Self {
        parser::parse(input)
    }

    /// Total character count (excluding frontmatter delimiters).
    pub fn char_count(&self) -> usize {
        self.to_markdown().len()
    }

    /// Count words in the document.
    pub fn word_count(&self) -> usize {
        self.to_markdown().split_whitespace().count()
    }

    /// Count blocks by type.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
}

impl Default for Document_core {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_document() {
        let doc = Document_core::new();
        assert!(doc.blocks.is_empty());
        assert!(doc.frontmatter.is_none());
    }

    #[test]
    fn inline_plain_text() {
        let bold = Inline::Bold(vec![Inline::Text("hello".into())]);
        assert_eq!(bold.plain_text(), "hello");
    }

    #[test]
    fn byte_range_equality() {
        let a = ByteRange { start: 0, end: 10 };
        let b = ByteRange { start: 0, end: 10 };
        assert_eq!(a, b);
    }
}
