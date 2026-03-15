#![allow(non_camel_case_types)]
/// Build a Block from MCP insert_block parameters.

use crate::shared::document_types_x::{Block_x as Block, Inline_x as Inline};
use crate::ui::mcp::block_kind_ui::BlockKind_ui;

/// Create a Block from a block kind, text content, optional level, and optional language.
pub fn build_block(
    block_kind: BlockKind_ui,
    text: String,
    level: Option<u8>,
    language: Option<String>,
) -> Block {
    match block_kind {
        BlockKind_ui::Paragraph => Block::Paragraph {
            content: vec![Inline::Text(text)],
            range: None,
        },
        BlockKind_ui::Heading => Block::Heading {
            level: level.unwrap_or(1),
            content: vec![Inline::Text(text)],
            range: None,
        },
        BlockKind_ui::CodeBlock => Block::CodeBlock {
            lang: language,
            code: text,
            range: None,
        },
        BlockKind_ui::HorizontalRule => Block::HorizontalRule { range: None },
        BlockKind_ui::BulletList => Block::BulletList {
            items: text
                .lines()
                .map(|l| vec![Inline::Text(l.to_string())])
                .collect(),
            range: None,
        },
    }
}
