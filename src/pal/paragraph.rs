#![allow(non_camel_case_types)]
// Block to skparagraph ParagraphBuilder mapping.
// Uses skia_safe::textlayout for professional text layout and rendering.

use crate::pal::paragraph_builder::build_single_paragraph;
use crate::shared::document_types_x::{Block_x as Block, Inline_x as Inline};
use skia_safe::{
    textlayout::FontCollection,
    FontMgr,
};

// Catppuccin Mocha color palette — pub(crate) for child modules.
/// Alpha channel for default text color.
pub const COLOR_TEXT_ALPHA: u8 = 255;
/// Red channel for default text color.
pub const COLOR_TEXT_R: u8 = 205;
/// Green channel for default text color.
pub const COLOR_TEXT_G: u8 = 214;
/// Blue channel for default text color.
pub const COLOR_TEXT_B: u8 = 244;

/// Alpha channel for secondary text color.
pub const COLOR_SUBTEXT_ALPHA: u8 = 255;
/// Red channel for secondary text color.
pub const COLOR_SUBTEXT_R: u8 = 166;
/// Green channel for secondary text color.
pub const COLOR_SUBTEXT_G: u8 = 173;
/// Blue channel for secondary text color.
pub const COLOR_SUBTEXT_B: u8 = 200;

/// Alpha channel for surface accent color.
pub const COLOR_SURFACE1_ALPHA: u8 = 255;
/// Red channel for surface accent color.
pub const COLOR_SURFACE1_R: u8 = 88;
/// Green channel for surface accent color.
pub const COLOR_SURFACE1_G: u8 = 91;
/// Blue channel for surface accent color.
pub const COLOR_SURFACE1_B: u8 = 112;

// Font sizing (points)
/// Base font size for normal paragraphs.
pub const FONT_SIZE_BASE: f32 = 14.0;
/// Font size for code blocks.
pub const FONT_SIZE_CODE: f32 = 12.0;
/// Font size for level-1 headings.
pub const FONT_SIZE_HEADING_1: f32 = 32.0;
/// Font size for level-2 headings.
pub const FONT_SIZE_HEADING_2: f32 = 24.0;
/// Font size for level-3 headings.
pub const FONT_SIZE_HEADING_3: f32 = 16.0;

/// Metadata and handle for a rendered Skia paragraph.
#[derive(Debug, Clone)]
pub struct StyledParagraph_pal {
    pub block_type: String,
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
}

/// Renderable paragraph wrapping Skia Paragraph for display.
pub type RenderableParagraph_pal = std::sync::Arc<std::sync::Mutex<skia_safe::textlayout::Paragraph>>;

/// Convert inline content to plain text string.
pub fn inline_to_string(inlines: &[Inline]) -> String {
    let mut buf = String::new();
    write_inline_text(inlines, &mut buf);
    buf
}

/// Append inline text to a buffer without intermediate allocations.
fn write_inline_text(inlines: &[Inline], buf: &mut String) {
    for inline in inlines {
        match inline {
            Inline::Text(s) | Inline::Code(s) => buf.push_str(s),
            Inline::Bold(children) | Inline::Italic(children) => write_inline_text(children, buf),
            Inline::Link { text, .. } => buf.push_str(text),
        }
    }
}

/// Derive block type label from a Block variant.
fn block_type_label(block: &Block) -> String {
    match block {
        Block::Heading { level, .. } => format!("heading-{}", level),
        Block::Paragraph { .. } => "paragraph".to_string(),
        Block::CodeBlock { .. } => "code-block".to_string(),
        Block::BulletList { .. } => "list-item".to_string(),
        Block::HorizontalRule { .. } => "rule".to_string(),
        Block::Image { .. } => "image".to_string(),
        Block::Video { .. } => "video".to_string(),
    }
}

/// Build Skia text layout paragraphs from document blocks.
/// Returns metadata that can be used to reconstruct and render paragraphs.
pub fn build_paragraphs(blocks: &[Block], available_width: f32) -> anyhow::Result<Vec<StyledParagraph_pal>> {
    let fm = FontMgr::new();
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(fm));

    let mut paragraphs = Vec::new();
    for block in blocks {
        let label = block_type_label(block);
        let (para, text, font_size) = build_single_paragraph(block, available_width, &font_collection);
        paragraphs.push(StyledParagraph_pal {
            block_type: label,
            text,
            width: available_width,
            height: para.height(),
            font_size,
        });
    }

    Ok(paragraphs)
}
