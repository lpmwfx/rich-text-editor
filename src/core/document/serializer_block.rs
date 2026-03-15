// Block-level serialization helpers.

use crate::core::document::{Block, Inline};

/// Serialize a single block to the output string.
pub fn serialize_block(block: &Block, out: &mut String) {
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
pub fn serialize_inlines(inlines: &[Inline], out: &mut String) {
    for inline in inlines {
        serialize_inline(inline, out);
    }
}

/// Serialize a single inline element.
pub fn serialize_inline(inline: &Inline, out: &mut String) {
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
