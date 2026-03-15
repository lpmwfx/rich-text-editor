// Shared document data types — pure data, used by all layers.
#![allow(non_camel_case_types)]

/// UTF-8 byte range in the source Markdown string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange_x {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
}

/// YAML frontmatter key-value pairs.
#[derive(Debug, Clone, Default)]
pub struct Frontmatter_x {
    /// Raw YAML content (without --- delimiters).
    pub raw: String,
}

/// A block-level element in the document.
#[derive(Debug, Clone)]
pub enum Block_x {
    /// A paragraph containing inline elements.
    Paragraph {
        /// Inline content.
        content: Vec<Inline_x>,
        /// Source byte range.
        range: Option<ByteRange_x>,
    },
    /// A heading with level (1-3) and inline content.
    Heading {
        /// Heading level (1-3).
        level: u8,
        /// Inline content of the heading.
        content: Vec<Inline_x>,
        /// Source byte range.
        range: Option<ByteRange_x>,
    },
    /// A fenced code block with optional language tag.
    CodeBlock {
        /// Language identifier for syntax highlighting.
        lang: Option<String>,
        /// Raw code content.
        code: String,
        /// Source byte range.
        range: Option<ByteRange_x>,
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
        range: Option<ByteRange_x>,
    },
    /// An embedded video.
    Video {
        /// File path relative to project root.
        path: String,
        /// Optional poster image path.
        poster: Option<String>,
        /// Source byte range.
        range: Option<ByteRange_x>,
    },
    /// An unordered list.
    BulletList {
        /// List items, each containing inline elements.
        items: Vec<Vec<Inline_x>>,
        /// Source byte range.
        range: Option<ByteRange_x>,
    },
    /// A horizontal rule separator.
    HorizontalRule {
        /// Source byte range.
        range: Option<ByteRange_x>,
    },
}

/// An inline-level element within a block.
#[derive(Debug, Clone)]
pub enum Inline_x {
    /// Plain text.
    Text(String),
    /// Bold-formatted inline content.
    Bold(Vec<Inline_x>),
    /// Italic-formatted inline content.
    Italic(Vec<Inline_x>),
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

impl Inline_x {
    /// Extract plain text from this inline element recursively.
    pub fn plain_text(&self) -> String {
        let mut buf = String::new();
        self.write_plain_text(&mut buf);
        buf
    }

    /// Append plain text to a buffer without allocating intermediate strings.
    fn write_plain_text(&self, buf: &mut String) {
        match self {
            Inline_x::Text(s) | Inline_x::Code(s) => buf.push_str(s),
            Inline_x::Bold(children) | Inline_x::Italic(children) => {
                for child in children {
                    child.write_plain_text(buf);
                }
            }
            Inline_x::Link { text, .. } => buf.push_str(text),
        }
    }
}
