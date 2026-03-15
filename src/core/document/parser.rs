// Markdown to AST parser using pulldown-cmark.

use pulldown_cmark::{Event, Options, Parser};

use crate::core::document::{Block, Document_core};
use crate::core::document::parser_blocks::parse_block;
use crate::core::document::parser_inline::is_video_extension;

/// Parse a Markdown string into a Document.
pub fn parse(input: &str) -> Document_core {
    let (frontmatter, body) = super::frontmatter::extract(input);
    let body_offset = input.len() - body.len();

    let opts = Options::ENABLE_STRIKETHROUGH;
    let parser = Parser::new_ext(body, opts);
    let events: Vec<(Event, std::ops::Range<usize>)> = parser.into_offset_iter().collect();

    let mut blocks = Vec::new();
    let mut idx = 0;

    while idx < events.len() {
        idx = parse_block(&events, idx, body_offset, &mut blocks);
    }

    let blocks = detect_videos(blocks);

    Document_core { frontmatter, blocks }
}

/// Promote image blocks to video blocks when the file extension is a known video format.
fn detect_videos(blocks: Vec<Block>) -> Vec<Block> {
    blocks
        .into_iter()
        .map(|b| match b {
            Block::Image {
                alt: _,
                path,
                caption: _,
                range,
            } if is_video_extension(&path) => Block::Video {
                path,
                poster: None,
                range,
            },
            other => other,
        })
        .collect()
}
