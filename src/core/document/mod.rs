/// Markdown to AST parser using pulldown-cmark.
pub mod parser;
/// AST to Markdown string serializer.
pub mod serializer;
/// YAML frontmatter parsing and serialization.
pub mod frontmatter;

/// UTF-8 byte range in the source Markdown string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
}

/// A complete document — sequence of blocks with optional frontmatter.
#[derive(Debug, Clone)]
pub struct Document {
    /// YAML frontmatter metadata.
    pub frontmatter: Option<Frontmatter>,
    /// Ordered sequence of content blocks.
    pub blocks: Vec<Block>,
}

impl Document {
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

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// YAML frontmatter key-value pairs.
#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    /// Raw YAML content (without --- delimiters).
    pub raw: String,
}

/// A block-level element in the document.
#[derive(Debug, Clone)]
pub enum Block {
    /// A paragraph containing inline elements.
    Paragraph {
        /// Inline content.
        content: Vec<Inline>,
        /// Source byte range.
        range: Option<ByteRange>,
    },
    /// A heading with level (1-3) and inline content.
    Heading {
        /// Heading level (1-3).
        level: u8,
        /// Inline content of the heading.
        content: Vec<Inline>,
        /// Source byte range.
        range: Option<ByteRange>,
    },
    /// A fenced code block with optional language tag.
    CodeBlock {
        /// Language identifier for syntax highlighting.
        lang: Option<String>,
        /// Raw code content.
        code: String,
        /// Source byte range.
        range: Option<ByteRange>,
    },
    /// An embedded image.
    Image {
        /// Alt text for accessibility.
        alt: String,
        /// File path relative to project root.
        path: String,
        /// Optional caption below the image.
        caption: Option<String>,
        /// Source byte range.
        range: Option<ByteRange>,
    },
    /// An embedded video.
    Video {
        /// File path relative to project root.
        path: String,
        /// Optional poster image path.
        poster: Option<String>,
        /// Source byte range.
        range: Option<ByteRange>,
    },
    /// An unordered list.
    BulletList {
        /// List items, each containing inline elements.
        items: Vec<Vec<Inline>>,
        /// Source byte range.
        range: Option<ByteRange>,
    },
    /// A horizontal rule separator.
    HorizontalRule {
        /// Source byte range.
        range: Option<ByteRange>,
    },
}

/// An inline-level element within a block.
#[derive(Debug, Clone)]
pub enum Inline {
    /// Plain text.
    Text(String),
    /// Bold-formatted inline content.
    Bold(Vec<Inline>),
    /// Italic-formatted inline content.
    Italic(Vec<Inline>),
    /// Inline code span.
    Code(String),
    /// A hyperlink.
    Link {
        /// Display text.
        text: String,
        /// URL target.
        url: String,
    },
}

impl Inline {
    /// Extract plain text from this inline element recursively.
    pub fn plain_text(&self) -> String {
        match self {
            Inline::Text(s) => s.clone(),
            Inline::Bold(children) | Inline::Italic(children) => {
                children.iter().map(|c| c.plain_text()).collect()
            }
            Inline::Code(s) => s.clone(),
            Inline::Link { text, .. } => text.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_document() {
        let doc = Document::new();
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
