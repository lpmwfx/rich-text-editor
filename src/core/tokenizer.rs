#![allow(non_camel_case_types)]
// Document AST to flat token list for Slint rendering.
// Each token has: text, color, bold, italic, font_size, line_idx, x_offset.
// Monospace: x_offset = character count from line start.

use crate::core::document::{Block, Document_core};

use crate::core::tokenizer_inline::tokenize_inlines;

/// Colors for different token types (Catppuccin Mocha palette).
const COLOR_TEXT: u32 = 0xFFcdd6f4;
const COLOR_HEADING: u32 = 0xFF89b4fa;
/// ARGB color used for bold inline tokens.
pub const COLOR_BOLD: u32 = 0xFFf5e0dc;
/// ARGB color used for italic inline tokens.
pub const COLOR_ITALIC: u32 = 0xFFf5c2e7;
/// ARGB color used for inline code and code block tokens.
pub const COLOR_CODE: u32 = 0xFFa6e3a1;
/// ARGB color used for hyperlink tokens.
pub const COLOR_LINK: u32 = 0xFF89dceb;
const COLOR_RULE: u32 = 0xFF585b70;
const COLOR_LIST_MARKER: u32 = 0xFFfab387;

/// Font sizes for different block types.
const FONT_SIZE_NORMAL: f32 = 16.0;
const FONT_SIZE_H1: f32 = 32.0;
const FONT_SIZE_H2: f32 = 24.0;
const FONT_SIZE_H3: f32 = 20.0;
/// Font size in pixels for code tokens.
pub const FONT_SIZE_CODE: f32 = 14.0;

/// A flat token ready for Slint rendering.
#[derive(Debug, Clone)]
pub struct FlatRichToken_core {
    /// Display text.
    pub text: String,
    /// ARGB color.
    pub color: u32,
    /// Bold weight.
    pub bold: bool,
    /// Italic style.
    pub italic: bool,
    /// Font size in pixels.
    pub font_size: f32,
    /// Line index (zero-based).
    pub line_idx: i32,
    /// Character offset from line start.
    pub x_offset: f32,
}

/// Map heading level to font size.
fn heading_font_size(level: u8) -> f32 {
    use crate::shared::sizes::{HEADING_LEVEL_H1, HEADING_LEVEL_H2};
    match level {
        HEADING_LEVEL_H1 => FONT_SIZE_H1,
        HEADING_LEVEL_H2 => FONT_SIZE_H2,
        _ => FONT_SIZE_H3,
    }
}

/// Convert a Document AST into a flat list of tokens for Slint.
pub fn tokenize(doc: &Document_core) -> (Vec<FlatRichToken_core>, i32) {
    let mut tokens = Vec::new();
    let mut line = 0i32;

    for block in &doc.blocks {
        match block {
            Block::Heading {
                level, content, ..
            } => {
                let font_size = heading_font_size(*level);
                let mut x = 0.0f32;
                tokenize_inlines(
                    content, &mut tokens, line, &mut x,
                    COLOR_HEADING, true, false, font_size,
                );
                line += 1;
            }

            Block::Paragraph { content, .. } => {
                let mut x = 0.0f32;
                tokenize_inlines(
                    content, &mut tokens, line, &mut x,
                    COLOR_TEXT, false, false, FONT_SIZE_NORMAL,
                );
                line += 1;
            }

            Block::CodeBlock { code, .. } => {
                for code_line in code.lines() {
                    tokens.push(FlatRichToken_core {
                        text: code_line.to_string(),
                        color: COLOR_CODE,
                        bold: false,
                        italic: false,
                        font_size: FONT_SIZE_CODE,
                        line_idx: line,
                        x_offset: 0.0,
                    });
                    line += 1;
                }
            }

            Block::BulletList { items, .. } => {
                for item in items {
                    tokens.push(FlatRichToken_core {
                        text: "- ".to_string(),
                        color: COLOR_LIST_MARKER,
                        bold: false,
                        italic: false,
                        font_size: FONT_SIZE_NORMAL,
                        line_idx: line,
                        x_offset: 0.0,
                    });
                    let mut x = crate::shared::sizes::LIST_MARKER_OFFSET;
                    tokenize_inlines(
                        item, &mut tokens, line, &mut x,
                        COLOR_TEXT, false, false, FONT_SIZE_NORMAL,
                    );
                    line += 1;
                }
            }

            Block::HorizontalRule { .. } => {
                tokens.push(FlatRichToken_core {
                    text: "---".to_string(),
                    color: COLOR_RULE,
                    bold: false,
                    italic: false,
                    font_size: FONT_SIZE_NORMAL,
                    line_idx: line,
                    x_offset: 0.0,
                });
                line += 1;
            }

            Block::Image { alt, path, .. } => {
                tokens.push(FlatRichToken_core {
                    text: format!("[img: {}]({})", alt, path),
                    color: COLOR_LINK,
                    bold: false,
                    italic: true,
                    font_size: FONT_SIZE_NORMAL,
                    line_idx: line,
                    x_offset: 0.0,
                });
                line += 1;
            }

            Block::Video { path, .. } => {
                tokens.push(FlatRichToken_core {
                    text: format!("[video: {}]", path),
                    color: COLOR_LINK,
                    bold: false,
                    italic: true,
                    font_size: FONT_SIZE_NORMAL,
                    line_idx: line,
                    x_offset: 0.0,
                });
                line += 1;
            }
        }

        // Blank line between blocks
        line += 1;
    }

    let total_lines = line;
    (tokens, total_lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::document::Document_core;

    #[test]
    fn tokenize_heading() {
        let doc = Document_core::from_markdown("# Hello");
        let (tokens, lines) = tokenize(&doc);
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].text, "Hello");
        assert!(tokens[0].bold);
        assert_eq!(tokens[0].font_size, FONT_SIZE_H1);
        assert!(lines > 0);
    }

    #[test]
    fn tokenize_bold_italic() {
        let doc = Document_core::from_markdown("**bold** and *italic*");
        let (tokens, _) = tokenize(&doc);
        assert!(tokens.len() >= 3);
        assert!(tokens[0].bold);
        assert!(tokens[2].italic);
    }

    #[test]
    fn tokenize_code_block() {
        let doc = Document_core::from_markdown("```\nline1\nline2\n```");
        let (tokens, _) = tokenize(&doc);
        assert!(tokens.iter().any(|t| t.text == "line1"));
        assert!(tokens.iter().any(|t| t.text == "line2"));
    }

    #[test]
    fn tokenize_bullet_list() {
        let doc = Document_core::from_markdown("- one\n- two");
        let (tokens, _) = tokenize(&doc);
        assert!(tokens.iter().any(|t| t.text == "- "));
        assert!(tokens.iter().any(|t| t.text == "one"));
    }
}
