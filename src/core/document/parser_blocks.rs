#![allow(non_camel_case_types)]
// Block-level parsing dispatcher.

use pulldown_cmark::{Event, Tag};

use crate::core::document::{Block, ByteRange};
use crate::core::document::parser_list_image::{
    parse_paragraph, parse_code_block, parse_list, parse_image,
};

/// Parse a single block-level element from the event stream.
/// Returns the next index to process.
pub fn parse_block(
    events: &[(Event, std::ops::Range<usize>)],
    start: usize,
    body_offset: usize,
    blocks: &mut Vec<Block>,
) -> usize {
    let (ref event, _) = events[start];

    match event {
        Event::Start(Tag::Paragraph) | Event::Start(Tag::Heading { .. }) => {
            let heading_level = match event {
                Event::Start(Tag::Heading { level, .. }) => Some(*level as u8),
                _ => None,
            };
            parse_paragraph(events, start, body_offset, blocks, heading_level)
        }
        Event::Start(Tag::CodeBlock(kind)) => {
            let lang = match &kind {
                pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                    let l = lang.to_string();
                    if l.is_empty() { None } else { Some(l) }
                }
                pulldown_cmark::CodeBlockKind::Indented => None,
            };
            parse_code_block(events, start, body_offset, blocks, lang)
        }
        Event::Start(Tag::List(None)) => parse_list(events, start, body_offset, blocks),
        Event::Start(Tag::Image { dest_url, title, .. }) => {
            parse_image(events, start, body_offset, blocks, dest_url.to_string(), title.to_string())
        }
        Event::Rule => {
            let block_start = events[start].1.start + body_offset;
            let block_end = events[start].1.end + body_offset;
            blocks.push(Block::HorizontalRule {
                range: Some(ByteRange { start: block_start, end: block_end }),
            });
            start + 1
        }
        _ => start + 1,
    }
}
