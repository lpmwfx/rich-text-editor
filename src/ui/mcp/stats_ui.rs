// Document statistics helpers for the MCP layer.

use crate::adapter::editor_state_adp::EditorState_adp;
use crate::shared::document_types_x::Block_x as Block;
use crate::shared::sizes::{HEADING_LEVEL_H1, HEADING_LEVEL_H2, HEADING_LEVEL_H3};

/// Count headings by level and media blocks into (h1, h2, h3, images, videos) buckets.
pub fn count_headings(blocks: &[Block]) -> (usize, usize, usize, usize, usize) {
    let (mut h1, mut h2, mut h3) = (0usize, 0usize, 0usize);
    let (mut images, mut videos) = (0usize, 0usize);
    for block in blocks {
        match block {
            Block::Heading { level, .. } => match *level {
                HEADING_LEVEL_H1 => h1 += 1,
                HEADING_LEVEL_H2 => h2 += 1,
                HEADING_LEVEL_H3 => h3 += 1,
                _ => {}
            },
            Block::Image { .. } => images += 1,
            Block::Video { .. } => videos += 1,
            _ => {}
        }
    }
    (h1, h2, h3, images, videos)
}

/// Compute and format document statistics from editor state.
pub fn compute_document_stats(editor_state: &EditorState_adp) -> String {
    let md = editor_state.to_markdown();
    let words = md.split_whitespace().count();
    let characters = md.len();
    let characters_no_spaces = md.chars().filter(|c| !c.is_whitespace()).count();
    let blocks = editor_state.document.blocks.len();
    let (h1, h2, h3, images, videos) = count_headings(&editor_state.document.blocks);
    format_document_stats(words, characters, characters_no_spaces, blocks, images, videos, h1, h2, h3)
}

/// Format document statistics as a JSON string.
pub fn format_document_stats(
    words: usize,
    characters: usize,
    characters_no_spaces: usize,
    blocks: usize,
    images: usize,
    videos: usize,
    h1: usize,
    h2: usize,
    h3: usize,
) -> String {
    format!(
        r#"{{"words": {}, "characters": {}, "characters_no_spaces": {}, "blocks": {}, "images": {}, "videos": {}, "headings": {{"h1": {}, "h2": {}, "h3": {}}}}}"#,
        words, characters, characters_no_spaces, blocks, images, videos, h1, h2, h3
    )
}
