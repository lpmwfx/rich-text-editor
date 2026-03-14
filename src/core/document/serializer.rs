// AST to Markdown string serializer.

use super::{Block, Document, Inline};

/// Serialize a Document to a Markdown string.
pub fn serialize(doc: &Document) -> String {
    let mut out = String::new();

    if let Some(ref fm) = doc.frontmatter {
        out.push_str("---\n");
        out.push_str(&fm.raw);
        if !fm.raw.ends_with('\n') {
            out.push('\n');
        }
        out.push_str("---\n\n");
    }

    for (i, block) in doc.blocks.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        serialize_block(block, &mut out);
        out.push('\n');
    }

    out
}

/// Serialize a single block to the output string.
fn serialize_block(block: &Block, out: &mut String) {
    match block {
        Block::Paragraph { content, .. } => {
            serialize_inlines(content, out);
        }
        Block::Heading {
            level, content, ..
        } => {
            for _ in 0..*level {
                out.push('#');
            }
            out.push(' ');
            serialize_inlines(content, out);
        }
        Block::CodeBlock { lang, code, .. } => {
            out.push_str("```");
            if let Some(l) = lang {
                out.push_str(l);
            }
            out.push('\n');
            out.push_str(code);
            if !code.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("```");
        }
        Block::Image {
            alt, path, caption, ..
        } => {
            out.push_str("![");
            out.push_str(alt);
            out.push_str("](");
            out.push_str(path);
            if let Some(cap) = caption {
                out.push_str(" \"");
                out.push_str(cap);
                out.push('"');
            }
            out.push(')');
        }
        Block::Video { path, .. } => {
            out.push_str("![video](");
            out.push_str(path);
            out.push(')');
        }
        Block::BulletList { items, .. } => {
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push('\n');
                }
                out.push_str("- ");
                serialize_inlines(item, out);
            }
        }
        Block::HorizontalRule { .. } => {
            out.push_str("---");
        }
    }
}

/// Serialize a sequence of inline elements.
fn serialize_inlines(inlines: &[Inline], out: &mut String) {
    for inline in inlines {
        serialize_inline(inline, out);
    }
}

/// Serialize a single inline element.
fn serialize_inline(inline: &Inline, out: &mut String) {
    match inline {
        Inline::Text(s) => out.push_str(s),
        Inline::Bold(children) => {
            out.push_str("**");
            serialize_inlines(children, out);
            out.push_str("**");
        }
        Inline::Italic(children) => {
            out.push('*');
            serialize_inlines(children, out);
            out.push('*');
        }
        Inline::Code(s) => {
            out.push('`');
            out.push_str(s);
            out.push('`');
        }
        Inline::Link { text, url } => {
            out.push('[');
            out.push_str(text);
            out.push_str("](");
            out.push_str(url);
            out.push(')');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::document::Frontmatter;

    #[test]
    fn serialize_paragraph() {
        let doc = Document {
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
        let doc = Document {
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
        let doc = Document {
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
        let doc = Document {
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
        let doc = Document {
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
        let doc = Document {
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
        let doc = Document {
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
