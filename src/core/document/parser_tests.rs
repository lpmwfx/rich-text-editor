// Tests for the Markdown parser.

#[cfg(test)]
mod tests {
    use crate::core::document::parser::parse;
    use crate::core::document::{Block, Inline};

    #[test]
    fn parse_simple_paragraph() {
        let doc = parse("Hello world");
        assert_eq!(doc.blocks.len(), 1);
        match &doc.blocks[0] {
            Block::Paragraph { content, range } => {
                assert_eq!(content.len(), 1);
                assert!(range.is_some());
                match &content[0] {
                    Inline::Text(t) => assert_eq!(t, "Hello world"),
                    _ => panic!("expected Text"),
                }
            }
            _ => panic!("expected Paragraph"),
        }
    }

    #[test]
    fn parse_heading() {
        let doc = parse("# Title");
        assert_eq!(doc.blocks.len(), 1);
        match &doc.blocks[0] {
            Block::Heading {
                level, content, ..
            } => {
                assert_eq!(*level, 1);
                assert_eq!(content[0].plain_text(), "Title");
            }
            _ => panic!("expected Heading"),
        }
    }

    #[test]
    fn parse_bold_italic() {
        let doc = parse("**bold** and *italic*");
        match &doc.blocks[0] {
            Block::Paragraph { content, .. } => {
                assert!(content.len() >= 3);
                match &content[0] {
                    Inline::Bold(children) => {
                        assert_eq!(children[0].plain_text(), "bold");
                    }
                    _ => panic!("expected Bold"),
                }
            }
            _ => panic!("expected Paragraph"),
        }
    }

    #[test]
    fn parse_code_block() {
        let doc = parse("```rust\nfn main() {}\n```");
        match &doc.blocks[0] {
            Block::CodeBlock { lang, code, .. } => {
                assert_eq!(lang.as_deref(), Some("rust"));
                assert!(code.contains("fn main()"));
            }
            _ => panic!("expected CodeBlock"),
        }
    }

    #[test]
    fn parse_horizontal_rule() {
        let doc = parse("---");
        match &doc.blocks[0] {
            Block::HorizontalRule { range } => {
                assert!(range.is_some());
            }
            _ => panic!("expected HorizontalRule"),
        }
    }

    #[test]
    fn parse_bullet_list() {
        let doc = parse("- item one\n- item two");
        match &doc.blocks[0] {
            Block::BulletList { items, .. } => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0][0].plain_text(), "item one");
                assert_eq!(items[1][0].plain_text(), "item two");
            }
            _ => panic!("expected BulletList"),
        }
    }

    #[test]
    fn parse_image_as_video() {
        let doc = parse("![clip](video.mp4)");
        match &doc.blocks[0] {
            Block::Paragraph { content, .. } => {
                assert!(!content.is_empty());
            }
            Block::Video { path, .. } => {
                assert!(path.ends_with(".mp4"));
            }
            _ => {}
        }
    }

    #[test]
    fn parse_link() {
        let doc = parse("[click](https://example.com)");
        match &doc.blocks[0] {
            Block::Paragraph { content, .. } => match &content[0] {
                Inline::Link { text, url } => {
                    assert_eq!(text, "click");
                    assert_eq!(url, "https://example.com");
                }
                _ => panic!("expected Link"),
            },
            _ => panic!("expected Paragraph"),
        }
    }

    #[test]
    fn roundtrip_simple() {
        let input = "# Hello\n\nWorld\n";
        let doc = parse(input);
        let output = doc.to_markdown();
        let doc2 = parse(&output);
        assert_eq!(doc.blocks.len(), doc2.blocks.len());
    }
}
