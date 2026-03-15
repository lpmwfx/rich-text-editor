#![allow(non_camel_case_types)]
// List and image block-level parsing helpers extracted from parser_blocks.rs.

use pulldown_cmark::{Event, Tag, TagEnd};

use crate::core::document::{Block, ByteRange};
use crate::core::document::parser_inline::parse_inline;

/// Parse a paragraph or heading block (inline content until end tag).
/// If `heading_level` is Some, produces a Heading block; otherwise a Paragraph.
pub fn parse_paragraph(
    events: &[(Event, std::ops::Range<usize>)],
    start: usize,
    body_offset: usize,
    blocks: &mut Vec<Block>,
    heading_level: Option<u8>,
) -> usize {
    let mut inlines = Vec::new();
    let mut idx = start + 1;
    let block_start = events[start].1.start + body_offset;

    while idx < events.len() {
        let is_end = match (&events[idx].0, heading_level) {
            (Event::End(TagEnd::Paragraph), None) => true,
            (Event::End(TagEnd::Heading(..)), Some(_)) => true,
            _ => false,
        };

        if is_end {
            let block_end = events[idx].1.end + body_offset;
            let range = Some(ByteRange { start: block_start, end: block_end });
            let block = match heading_level {
                Some(level) => Block::Heading { level, content: inlines, range },
                None => Block::Paragraph { content: inlines, range },
            };
            blocks.push(block);
            return idx + 1;
        }

        let (inline, next) = parse_inline(events, idx);
        inlines.push(inline);
        idx = next;
    }

    if heading_level.is_none() {
        blocks.push(Block::Paragraph {
            content: inlines,
            range: Some(ByteRange { start: block_start, end: block_start }),
        });
    }
    idx
}

/// Parse a fenced or indented code block from the event stream.
pub fn parse_code_block(
    events: &[(Event, std::ops::Range<usize>)],
    start: usize,
    body_offset: usize,
    blocks: &mut Vec<Block>,
    lang: Option<String>,
) -> usize {
    let mut code = String::new();
    let mut idx = start + 1;
    let block_start = events[start].1.start + body_offset;

    while idx < events.len() {
        match &events[idx].0 {
            Event::End(TagEnd::CodeBlock) => {
                let block_end = events[idx].1.end + body_offset;
                blocks.push(Block::CodeBlock {
                    lang,
                    code,
                    range: Some(ByteRange {
                        start: block_start,
                        end: block_end,
                    }),
                });
                return idx + 1;
            }
            Event::Text(t) => {
                code.push_str(t);
                idx += 1;
            }
            _ => {
                idx += 1;
            }
        }
    }
    idx
}

/// Parse a bullet list from the event stream.
/// Returns the next index to process.
pub fn parse_list(
    events: &[(Event, std::ops::Range<usize>)],
    start: usize,
    body_offset: usize,
    blocks: &mut Vec<Block>,
) -> usize {
    let mut items = Vec::new();
    let mut idx = start + 1;
    let block_start = events[start].1.start + body_offset;

    while idx < events.len() {
        match &events[idx].0 {
            Event::End(TagEnd::List(false)) => {
                let block_end = events[idx].1.end + body_offset;
                blocks.push(Block::BulletList {
                    items,
                    range: Some(ByteRange {
                        start: block_start,
                        end: block_end,
                    }),
                });
                return idx + 1;
            }
            Event::Start(Tag::Item) => {
                let mut item_inlines = Vec::new();
                idx += 1;
                while idx < events.len() {
                    match &events[idx].0 {
                        Event::End(TagEnd::Item) => {
                            idx += 1;
                            break;
                        }
                        Event::Start(Tag::Paragraph) => {
                            idx += 1;
                        }
                        Event::End(TagEnd::Paragraph) => {
                            idx += 1;
                        }
                        _ => {
                            let (inline, next) = parse_inline(events, idx);
                            item_inlines.push(inline);
                            idx = next;
                        }
                    }
                }
                items.push(item_inlines);
            }
            _ => {
                idx += 1;
            }
        }
    }
    idx
}

/// Parse an image block from the event stream.
/// Returns the next index to process.
pub fn parse_image(
    events: &[(Event, std::ops::Range<usize>)],
    start: usize,
    body_offset: usize,
    blocks: &mut Vec<Block>,
    path: String,
    title: String,
) -> usize {
    let mut alt = String::new();
    let mut idx = start + 1;
    let block_start = events[start].1.start + body_offset;

    while idx < events.len() {
        match &events[idx].0 {
            Event::End(TagEnd::Image) => {
                let block_end = events[idx].1.end + body_offset;
                let caption = if title.is_empty() {
                    None
                } else {
                    Some(title)
                };
                blocks.push(Block::Image {
                    alt,
                    path,
                    caption,
                    range: Some(ByteRange {
                        start: block_start,
                        end: block_end,
                    }),
                });
                return idx + 1;
            }
            Event::Text(t) => {
                alt.push_str(t);
                idx += 1;
            }
            _ => {
                idx += 1;
            }
        }
    }
    idx
}
