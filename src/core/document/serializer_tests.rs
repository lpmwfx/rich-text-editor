// Tests for the Markdown serializer.

#[cfg(test)]
mod tests {
    use crate::core::document::serializer::serialize;
    use crate::core::document::{Block, Document_core, Frontmatter, Inline};

    #[test]
    fn serialize_paragraph() {
        let doc = Document_core {
            frontmatter: None,
            blocks: vec![Block::Paragraph {
                content: vec![Inline::Text("Hello world".into())],
                range: None,
            }],
        };
        let md = serialize(&doc);
        assert_eq!(md.trim(), "Hello world");
    }

    #[test]
    fn serialize_heading() {
        let doc = Document_core {
            frontmatter: None,
            blocks: vec![Block::Heading {
                level: 2,
                content: vec![Inline::Text("Title".into())],
                range: None,
            }],
        };
        let md = serialize(&doc);
        assert_eq!(md.trim(), "## Title");
    }

    #[test]
    fn serialize_bold_italic() {
        let doc = Document_core {
            frontmatter: None,
            blocks: vec![Block::Paragraph {
                content: vec![
                    Inline::Bold(vec![Inline::Text("bold".into())]),
                    Inline::Text(" and ".into()),
                    Inline::Italic(vec![Inline::Text("italic".into())]),
                ],
                range: None,
            }],
        };
        let md = serialize(&doc);
        assert_eq!(md.trim(), "**bold** and *italic*");
    }

    #[test]
    fn serialize_code_block() {
        let doc = Document_core {
            frontmatter: None,
            blocks: vec![Block::CodeBlock {
                lang: Some("rust".into()),
                code: "fn main() {}\n".into(),
                range: None,
            }],
        };
        let md = serialize(&doc);
        assert!(md.contains("```rust"));
        assert!(md.contains("fn main()"));
    }

    #[test]
    fn serialize_frontmatter() {
        let doc = Document_core {
            frontmatter: Some(Frontmatter {
                raw: "title: Test\n".into(),
            }),
            blocks: vec![Block::Paragraph {
                content: vec![Inline::Text("Body".into())],
                range: None,
            }],
        };
        let md = serialize(&doc);
        assert!(md.starts_with("---\ntitle: Test\n---\n"));
    }

    #[test]
    fn serialize_bullet_list() {
        let doc = Document_core {
            frontmatter: None,
            blocks: vec![Block::BulletList {
                items: vec![
                    vec![Inline::Text("one".into())],
                    vec![Inline::Text("two".into())],
                ],
                range: None,
            }],
        };
        let md = serialize(&doc);
        assert!(md.contains("- one"));
        assert!(md.contains("- two"));
    }

    #[test]
    fn serialize_link() {
        let doc = Document_core {
            frontmatter: None,
            blocks: vec![Block::Paragraph {
                content: vec![Inline::Link {
                    text: "click".into(),
                    url: "https://example.com".into(),
                }],
                range: None,
            }],
        };
        let md = serialize(&doc);
        assert_eq!(md.trim(), "[click](https://example.com)");
    }
}
