// Block to skparagraph ParagraphBuilder mapping.
// Uses skia_safe::textlayout for professional text layout and rendering.

use crate::core::document::{Block, Document, Inline};
use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    Color, FontMgr,
};

// Catppuccin Mocha color palette
const COLOR_TEXT_ALPHA: u8 = 255;
const COLOR_TEXT_R: u8 = 205;
const COLOR_TEXT_G: u8 = 214;
const COLOR_TEXT_B: u8 = 244;

const COLOR_SUBTEXT_ALPHA: u8 = 255;
const COLOR_SUBTEXT_R: u8 = 166;
const COLOR_SUBTEXT_G: u8 = 173;
const COLOR_SUBTEXT_B: u8 = 200;

const COLOR_SURFACE1_ALPHA: u8 = 255;
const COLOR_SURFACE1_R: u8 = 88;
const COLOR_SURFACE1_G: u8 = 91;
const COLOR_SURFACE1_B: u8 = 112;

// Font sizing (points)
const FONT_SIZE_BASE: f32 = 14.0;
const FONT_SIZE_CODE: f32 = 12.0;
const FONT_SIZE_HEADING_1: f32 = 32.0;
const FONT_SIZE_HEADING_2: f32 = 24.0;
const FONT_SIZE_HEADING_3: f32 = 16.0;

// Media placeholder heights (pixels)
const MEDIA_HEIGHT_IMAGE: f32 = 200.0;
const MEDIA_HEIGHT_VIDEO: f32 = 300.0;

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
pub type RenderableParagraph = std::sync::Arc<std::sync::Mutex<skia_safe::textlayout::Paragraph>>;

/// Build Skia text layout paragraphs from document blocks.
/// Returns metadata that can be used to reconstruct and render paragraphs.
pub fn build_paragraphs(doc: &Document, available_width: f32) -> anyhow::Result<Vec<StyledParagraph_pal>> {
    let mut paragraphs = Vec::new();
    let fm = FontMgr::new();
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(fm.clone()));

    for block in &doc.blocks {
        match block {
            Block::Heading { level, content, .. } => {
                let font_size = match level {
                    1 => FONT_SIZE_HEADING_1,
                    2 => FONT_SIZE_HEADING_2,
                    _ => FONT_SIZE_HEADING_3,
                };
                let text = inline_to_string(content);
                let mut style = TextStyle::new();
                style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
                style.set_font_size(font_size);
                style.set_color(Color::from_argb(
                    COLOR_TEXT_ALPHA,
                    COLOR_TEXT_R,
                    COLOR_TEXT_G,
                    COLOR_TEXT_B,
                ));

                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);
                para_style.set_text_align(skia_safe::textlayout::TextAlign::Left);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text(&text);

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                let height = paragraph.height();

                paragraphs.push(StyledParagraph_pal {
                    block_type: format!("heading-{}", level),
                    text,
                    width: available_width,
                    height,
                    font_size,
                });
            }

            Block::Paragraph { content, .. } => {
                let font_size = FONT_SIZE_BASE;
                let text = inline_to_string(content);
                let mut style = TextStyle::new();
                style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
                style.set_font_size(font_size);
                style.set_color(Color::from_argb(
                    COLOR_TEXT_ALPHA,
                    COLOR_TEXT_R,
                    COLOR_TEXT_G,
                    COLOR_TEXT_B,
                ));

                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text(&text);

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                let height = paragraph.height();

                paragraphs.push(StyledParagraph_pal {
                    block_type: "paragraph".to_string(),
                    text,
                    width: available_width,
                    height,
                    font_size,
                });
            }

            Block::CodeBlock { code, .. } => {
                let font_size = FONT_SIZE_CODE;
                let mut style = TextStyle::new();
                style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
                style.set_font_size(font_size);
                style.set_color(Color::from_argb(
                    COLOR_SUBTEXT_ALPHA,
                    COLOR_SUBTEXT_R,
                    COLOR_SUBTEXT_G,
                    COLOR_SUBTEXT_B,
                ));

                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text(code);

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                let height = paragraph.height();

                paragraphs.push(StyledParagraph_pal {
                    block_type: "code-block".to_string(),
                    text: code.clone(),
                    width: available_width,
                    height,
                    font_size,
                });
            }

            Block::BulletList { items, .. } => {
                for item in items {
                    let font_size = FONT_SIZE_BASE;
                    let text = format!("• {}", inline_to_string(item));
                    let mut style = TextStyle::new();
                    style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
                    style.set_font_size(font_size);
                    style.set_color(Color::from_argb(
                    COLOR_TEXT_ALPHA,
                    COLOR_TEXT_R,
                    COLOR_TEXT_G,
                    COLOR_TEXT_B,
                ));

                    let mut para_style = ParagraphStyle::new();
                    para_style.set_text_style(&style);

                    let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                    builder.add_text(&text);

                    let mut paragraph = builder.build();
                    paragraph.layout(available_width);
                    let height = paragraph.height();

                    paragraphs.push(StyledParagraph_pal {
                        block_type: "list-item".to_string(),
                        text,
                        width: available_width,
                        height,
                        font_size,
                    });
                }
            }

            Block::HorizontalRule { .. } => {
                let font_size = FONT_SIZE_BASE;
                let text = "―――――――".to_string();
                let mut style = TextStyle::new();
                style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
                style.set_font_size(font_size);
                style.set_color(Color::from_argb(
                    COLOR_SURFACE1_ALPHA,
                    COLOR_SURFACE1_R,
                    COLOR_SURFACE1_G,
                    COLOR_SURFACE1_B,
                ));

                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);
                para_style.set_text_align(skia_safe::textlayout::TextAlign::Center);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text(&text);

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                let height = paragraph.height();

                paragraphs.push(StyledParagraph_pal {
                    block_type: "rule".to_string(),
                    text,
                    width: available_width,
                    height,
                    font_size,
                });
            }

            Block::Image { alt, path, .. } => {
                paragraphs.push(StyledParagraph_pal {
                    block_type: "image".to_string(),
                    text: format!("[image: {}]({})", alt, path),
                    width: available_width,
                    height: MEDIA_HEIGHT_IMAGE,
                    font_size: FONT_SIZE_BASE,
                });
            }

            Block::Video { path, .. } => {
                paragraphs.push(StyledParagraph_pal {
                    block_type: "video".to_string(),
                    text: format!("[video: {}]", path),
                    width: available_width,
                    height: MEDIA_HEIGHT_VIDEO,
                    font_size: FONT_SIZE_BASE,
                });
            }
        }
    }

    Ok(paragraphs)
}

/// Convert inline content to plain text string.
fn inline_to_string(inlines: &[Inline]) -> String {
    inlines
        .iter()
        .map(|inline| match inline {
            Inline::Text(s) => s.clone(),
            Inline::Bold(children) => inline_to_string(children),
            Inline::Italic(children) => inline_to_string(children),
            Inline::Code(s) => s.clone(),
            Inline::Link { text, .. } => text.clone(),
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Build renderable Skia Paragraphs for display.
///
/// Returns actual Skia Paragraph objects wrapped in Arc<Mutex<>> for thread-safe sharing.
/// These can be passed directly to the render module for display.
pub fn build_renderable_paragraphs(
    doc: &Document,
    available_width: f32,
) -> anyhow::Result<Vec<RenderableParagraph>> {
    let mut paragraphs = Vec::new();
    let fm = FontMgr::new();
    let mut font_collection = FontCollection::new();
    font_collection.set_asset_font_manager(Some(fm.clone()));

    for block in &doc.blocks {
        let para = match block {
            Block::Heading { level, content, .. } => {
                let font_size = match level {
                    1 => FONT_SIZE_HEADING_1,
                    2 => FONT_SIZE_HEADING_2,
                    _ => FONT_SIZE_HEADING_3,
                };
                let text = inline_to_string(content);
                let mut style = TextStyle::new();
                style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
                style.set_font_size(font_size);
                style.set_color(Color::from_argb(
                    COLOR_TEXT_ALPHA,
                    COLOR_TEXT_R,
                    COLOR_TEXT_G,
                    COLOR_TEXT_B,
                ));

                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);
                para_style.set_text_align(skia_safe::textlayout::TextAlign::Left);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text(&text);

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                paragraph
            }

            Block::Paragraph { content, .. } => {
                let font_size = FONT_SIZE_BASE;
                let text = inline_to_string(content);
                let mut style = TextStyle::new();
                style.set_font_families(&["Cascadia Code", "Consolas", "monospace"]);
                style.set_font_size(font_size);
                style.set_color(Color::from_argb(
                    COLOR_TEXT_ALPHA,
                    COLOR_TEXT_R,
                    COLOR_TEXT_G,
                    COLOR_TEXT_B,
                ));

                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text(&text);

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                paragraph
            }

            Block::CodeBlock { lang: _, code, .. } => {
                let font_size = FONT_SIZE_CODE;
                let mut style = TextStyle::new();
                style.set_font_families(&["Courier New", "monospace"]);
                style.set_font_size(font_size);
                style.set_color(Color::from_argb(
                    COLOR_SUBTEXT_ALPHA,
                    COLOR_SUBTEXT_R,
                    COLOR_SUBTEXT_G,
                    COLOR_SUBTEXT_B,
                ));

                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text(code);

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                paragraph
            }

            Block::HorizontalRule { .. } => {
                // Rule is rendered as empty paragraph for layout purposes
                let mut style = TextStyle::new();
                style.set_font_size(1.0);
                let mut para_style = ParagraphStyle::new();
                para_style.set_text_style(&style);

                let mut builder = ParagraphBuilder::new(&para_style, font_collection.clone());
                builder.add_text("");

                let mut paragraph = builder.build();
                paragraph.layout(available_width);
                paragraph
            }

            _ => continue,
        };

        paragraphs.push(std::sync::Arc::new(std::sync::Mutex::new(para)));
    }

    Ok(paragraphs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_paragraphs() {
        let doc = Document::from_markdown("# Hello\n\nWorld");
        let paragraphs = build_paragraphs(&doc, 800.0).expect("should build");
        assert!(!paragraphs.is_empty());
        assert_eq!(paragraphs[0].block_type, "heading-1");
    }

    #[test]
    fn test_inline_to_string() {
        let inlines = vec![
            Inline::Text("Hello ".to_string()),
            Inline::Bold(vec![Inline::Text("bold".to_string())]),
        ];
        let text = inline_to_string(&inlines);
        assert_eq!(text, "Hello bold");
    }
}
