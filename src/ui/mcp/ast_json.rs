// AST to JSON serialization for the MCP get_document tool.

use crate::shared::document_types_x::{Block_x as Block, ByteRange_x as ByteRange, Inline_x as Inline};

/// Serialize the document AST to a JSON string with byte ranges.
pub fn serialize_ast_json(blocks: &[Block]) -> String {
    let mut blocks_json = Vec::new();

    for block in blocks {
        let bj = match block {
            Block::Paragraph { content, range } => format!(
                r#"{{"type": "Paragraph", "byte_range": {}, "inlines": {}}}"#,
                range_json(range),
                inlines_json(content)
            ),
            Block::Heading { level, content, range } => format!(
                r#"{{"type": "Heading", "level": {}, "byte_range": {}, "content": "{}"}}"#,
                level,
                range_json(range),
                content.iter().map(|i| i.plain_text()).collect::<String>().replace('"', "\\\"")
            ),
            Block::CodeBlock { lang, code: _, range } => format!(
                r#"{{"type": "CodeBlock", "lang": {}, "byte_range": {}}}"#,
                lang.as_ref().map(|l| format!("\"{}\"", l)).unwrap_or_else(|| "null".into()),
                range_json(range)
            ),
            Block::Image { alt, path, range, .. } => format!(
                r#"{{"type": "Image", "alt": "{}", "path": "{}", "byte_range": {}}}"#,
                alt.replace('"', "\\\""),
                path.replace('"', "\\\""),
                range_json(range)
            ),
            Block::Video { path, range, .. } => format!(
                r#"{{"type": "Video", "path": "{}", "byte_range": {}}}"#,
                path.replace('"', "\\\""),
                range_json(range)
            ),
            Block::BulletList { items, range } => format!(
                r#"{{"type": "BulletList", "items": {}, "byte_range": {}}}"#,
                items.len(),
                range_json(range)
            ),
            Block::HorizontalRule { range } => format!(
                r#"{{"type": "HorizontalRule", "byte_range": {}}}"#,
                range_json(range)
            ),
        };
        blocks_json.push(bj);
    }

    format!(r#"{{"blocks": [{}]}}"#, blocks_json.join(", "))
}

/// Format a ByteRange as JSON.
fn range_json(range: &Option<ByteRange>) -> String {
    match range {
        Some(r) => format!("[{}, {}]", r.start, r.end),
        None => "null".into(),
    }
}

/// Format inline elements as a JSON array.
fn inlines_json(inlines: &[Inline]) -> String {
    let items: Vec<String> = inlines
        .iter()
        .map(|i| match i {
            Inline::Text(s) => format!(r#"{{"type": "Text", "text": "{}"}}"#, s.replace('"', "\\\"")),
            Inline::Bold(_) => r#"{"type": "Bold"}"#.into(),
            Inline::Italic(_) => r#"{"type": "Italic"}"#.into(),
            Inline::Code(s) => format!(r#"{{"type": "Code", "text": "{}"}}"#, s.replace('"', "\\\"")),
            Inline::Link { text, url } => format!(
                r#"{{"type": "Link", "text": "{}", "url": "{}"}}"#,
                text.replace('"', "\\\""),
                url.replace('"', "\\\"")
            ),
        })
        .collect();
    format!("[{}]", items.join(", "))
}
