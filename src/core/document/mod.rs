/// Block and Inline AST types for the document model.
pub mod parser;
/// AST to Markdown serializer.
pub mod serializer;
/// YAML frontmatter parsing and serialization.
pub mod frontmatter;

/// A complete document — sequence of blocks with optional frontmatter.
#[derive(Debug, Clone)]
pub struct Document {
    /// YAML frontmatter metadata.
    pub frontmatter: Option<Frontmatter>,
    /// Ordered sequence of content blocks.
    pub blocks: Vec<Block>,
}

/// YAML frontmatter key-value pairs.
#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    /// Raw YAML content.
    pub raw: String,
}

/// A block-level element in the document.
#[derive(Debug, Clone)]
pub enum Block {
    /// A paragraph containing inline elements.
    Paragraph(Vec<Inline>),
    /// A heading with level (1-3) and inline content.
    Heading {
        /// Heading level (1-3).
        level: u8,
        /// Inline content of the heading.
        content: Vec<Inline>,
    },
    /// A fenced code block with optional language tag.
    CodeBlock {
        /// Language identifier for syntax highlighting.
        lang: Option<String>,
        /// Raw code content.
        code: String,
    },
    /// An embedded image.
    Image {
        /// Alt text for accessibility.
        alt: String,
        /// File path relative to project root.
        path: String,
        /// Optional caption below the image.
        caption: Option<String>,
    },
    /// An embedded video.
    Video {
        /// File path relative to project root.
        path: String,
        /// Optional poster image path.
        poster: Option<String>,
    },
    /// An unordered list.
    BulletList(Vec<Vec<Inline>>),
    /// A horizontal rule separator.
    HorizontalRule,
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
