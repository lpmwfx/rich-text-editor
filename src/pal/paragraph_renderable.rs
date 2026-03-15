// Renderable Skia Paragraphs for display — thread-safe wrapped Paragraph objects.

use crate::shared::document_types_x::Block_x as Block;
use skia_safe::{
    textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle},
    Color, FontMgr,
};

use crate::pal::paragraph::{
    inline_to_string, RenderableParagraph_pal, COLOR_SUBTEXT_ALPHA, COLOR_SUBTEXT_B, COLOR_SUBTEXT_G,
    COLOR_SUBTEXT_R, COLOR_TEXT_ALPHA, COLOR_TEXT_B, COLOR_TEXT_G, COLOR_TEXT_R, FONT_SIZE_BASE,
    FONT_SIZE_CODE, FONT_SIZE_HEADING_1, FONT_SIZE_HEADING_2, FONT_SIZE_HEADING_3,
};

/// Map heading level to font size.
fn heading_font_size(level: u8) -> f32 {
    use crate::shared::sizes::{HEADING_LEVEL_H1, HEADING_LEVEL_H2};
    match level {
        HEADING_LEVEL_H1 => FONT_SIZE_HEADING_1,
        HEADING_LEVEL_H2 => FONT_SIZE_HEADING_2,
        _ => FONT_SIZE_HEADING_3,
    }
}

/// Build and layout a paragraph with the given text style.
fn build_styled_paragraph(
    text: &str,
    font_size: f32,
    color: Color,
    font_families: &[&str],
    available_width: f32,
    font_collection: &FontCollection,
) -> Paragraph {
    let mut style = TextStyle::new();
    style.set_font_families(font_families);
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

/// Build a single renderable Skia Paragraph from one block, wrapped for thread-safe sharing.
///
/// Returns `None` for block types that have no text representation (e.g. Image, Video).
fn build_block_renderable(
    block: &Block,
    available_width: f32,
    font_collection: &FontCollection,
    text_color: Color,
    subtext_color: Color,
    mono_fonts: &[&str],
) -> Option<RenderableParagraph_pal> {
    let para = match block {
        Block::Heading { level, content, .. } => {
            let text = inline_to_string(content);
            build_styled_paragraph(&text, heading_font_size(*level), text_color, mono_fonts, available_width, font_collection)
        }
        Block::Paragraph { content, .. } => {
            let text = inline_to_string(content);
            build_styled_paragraph(&text, FONT_SIZE_BASE, text_color, mono_fonts, available_width, font_collection)
        }
        Block::CodeBlock { code, .. } => {
            build_styled_paragraph(code, FONT_SIZE_CODE, subtext_color, &["Courier New", "monospace"], available_width, font_collection)
        }
        Block::HorizontalRule { .. } => {
            build_styled_paragraph("", crate::shared::sizes::HR_FONT_SIZE_PX, text_color, mono_fonts, available_width, font_collection)
        }
        _ => return None,
    };
    // Arc+Mutex: paragraph is shared between build phase and render thread.
    Some(std::sync::Arc::new(std::sync::Mutex::new(para)))
}

/// Build renderable Skia Paragraphs for display.
///
/// Returns actual Skia Paragraph objects wrapped in Arc<Mutex<>> for thread-safe sharing.
/// These can be passed directly to the render module for display.
pub fn build_renderable_paragraphs(
    blocks: &[Block],
    available_width: f32,
) -> anyhow::Result<Vec<RenderableParagraph_pal>> {
    let fm = FontMgr::new();
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(fm));

    let text_color = Color::from_argb(COLOR_TEXT_ALPHA, COLOR_TEXT_R, COLOR_TEXT_G, COLOR_TEXT_B);
    let subtext_color = Color::from_argb(COLOR_SUBTEXT_ALPHA, COLOR_SUBTEXT_R, COLOR_SUBTEXT_G, COLOR_SUBTEXT_B);
    let mono_fonts: &[&str] = &["Cascadia Code", "Consolas", "monospace"];

    let paragraphs = blocks.iter()
        .filter_map(|block| build_block_renderable(block, available_width, &font_collection, text_color, subtext_color, mono_fonts))
        .collect();

    Ok(paragraphs)
}
