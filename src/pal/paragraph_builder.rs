// Paragraph building helpers — build single paragraphs from document blocks.

use crate::shared::document_types_x::{Block_x as Block, ByteRange_x as ByteRange, Inline_x as Inline};
use skia_safe::{
    textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle},
    Color,
};

// Re-use constants from paragraph_cache.
pub use super::paragraph_cache::{
    COLOR_SUBTEXT, COLOR_SURFACE1, COLOR_TEXT, FONT_SIZE_BASE, FONT_SIZE_CODE, FONT_SIZE_H1,
    FONT_SIZE_H2, FONT_SIZE_H3,
};

/// Map heading level to font size.
fn heading_font_size(level: u8) -> f32 {
    use crate::shared::sizes::{HEADING_LEVEL_H1, HEADING_LEVEL_H2};
    match level {
        HEADING_LEVEL_H1 => FONT_SIZE_H1,
        HEADING_LEVEL_H2 => FONT_SIZE_H2,
        _ => FONT_SIZE_H3,
    }
}

/// Build a single Skia Paragraph from a document Block.
pub fn build_single_paragraph(
    block: &Block,
    available_width: f32,
    font_collection: &FontCollection,
) -> (Paragraph, String, f32) {
    match block {
        Block::Heading { level, content, .. } => {
            let font_size = heading_font_size(*level);
            let text = inline_to_string(content);
            let para = build_text_paragraph(&text, font_size, COLOR_TEXT, available_width, font_collection);
            (para, text, font_size)
        }
        Block::Paragraph { content, .. } => {
            let text = inline_to_string(content);
            let para = build_text_paragraph(&text, FONT_SIZE_BASE, COLOR_TEXT, available_width, font_collection);
            (para, text, FONT_SIZE_BASE)
        }
        Block::CodeBlock { code, .. } => {
            let para = build_text_paragraph(code, FONT_SIZE_CODE, COLOR_SUBTEXT, available_width, font_collection);
            (para, code.clone(), FONT_SIZE_CODE)
        }
        Block::BulletList { items, .. } => {
            let text: String = items
                .iter()
                .map(|item| format!("• {}", inline_to_string(item)))
                .collect::<Vec<_>>()
                .join("\n");
            let para = build_text_paragraph(&text, FONT_SIZE_BASE, COLOR_TEXT, available_width, font_collection);
            (para, text, FONT_SIZE_BASE)
        }
        Block::HorizontalRule { .. } => {
            let text = "―――――――".to_string();
            let mut style = TextStyle::new();
            style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
            style.set_font_size(FONT_SIZE_BASE);
            style.set_color(COLOR_SURFACE1);

            let mut para_style = ParagraphStyle::new();
            para_style.set_text_style(&style);
            para_style.set_text_align(skia_safe::textlayout::TextAlign::Center);

            let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
            builder.add_text(&text);
            let mut paragraph = builder.build();
            paragraph.layout(available_width);
            (paragraph, text, FONT_SIZE_BASE)
        }
        Block::Image { alt, path, .. } => {
            let text = format!("[image: {}]({})", alt, path);
            let para = build_text_paragraph(&text, FONT_SIZE_BASE, COLOR_SUBTEXT, available_width, font_collection);
            (para, text, FONT_SIZE_BASE)
        }
        Block::Video { path, .. } => {
            let text = format!("[video: {}]", path);
            let para = build_text_paragraph(&text, FONT_SIZE_BASE, COLOR_SUBTEXT, available_width, font_collection);
            (para, text, FONT_SIZE_BASE)
        }
    }
}

/// Build a simple text paragraph with the given style.
pub fn build_text_paragraph(
    text: &str,
    font_size: f32,
    color: Color,
    available_width: f32,
    font_collection: &FontCollection,
) -> Paragraph {
    let mut style = TextStyle::new();
    style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
    style.set_font_size(font_size);
    style.set_color(color);

    let mut para_style = ParagraphStyle::new();
    para_style.set_text_style(&style);

    let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
    builder.add_text(text);
    let mut paragraph = builder.build();
    paragraph.layout(available_width);
    paragraph
}

/// Convert inline AST nodes to plain text.
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

/// Find the approximate byte range of a block in the serialized markdown.
pub fn find_block_range(markdown: &str, search_from: &mut usize, _block: &Block) -> ByteRange {
    let start = *search_from;
    let remaining = &markdown[start..];

    let block_end = remaining
        .find("\n\n")
        .map(|pos| start + pos + crate::shared::sizes::LEFT_PADDING_SIDES)
        .unwrap_or(markdown.len());

    *search_from = block_end;

    ByteRange {
        start,
        end: block_end,
    }
}
