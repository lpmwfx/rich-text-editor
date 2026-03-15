// Inline tokenization — recursively tokenize inline elements into flat tokens.

use crate::core::document::Inline;
use crate::core::tokenizer::{FlatRichToken_core, COLOR_BOLD, COLOR_CODE, COLOR_ITALIC, COLOR_LINK, FONT_SIZE_CODE};

/// Recursively tokenize inline elements into the flat list.
pub fn tokenize_inlines(
    inlines: &[Inline],
    tokens: &mut Vec<FlatRichToken_core>,
    line: i32,
    x: &mut f32,
    base_color: u32,
    base_bold: bool,
    base_italic: bool,
    font_size: f32,
) {
    for inline in inlines {
        match inline {
            Inline::Text(s) => {
                if !s.is_empty() {
                    tokens.push(FlatRichToken_core {
                        text: s.to_owned(),
                        color: base_color,
                        bold: base_bold,
                        italic: base_italic,
                        font_size,
                        line_idx: line,
                        x_offset: *x,
                    });
                    *x += s.len() as f32;
                }
            }
            Inline::Bold(children) => {
                tokenize_inlines(
                    children, tokens, line, x, COLOR_BOLD, true, base_italic, font_size,
                );
            }
            Inline::Italic(children) => {
                tokenize_inlines(
                    children, tokens, line, x, COLOR_ITALIC, base_bold, true, font_size,
                );
            }
            Inline::Code(s) => {
                tokens.push(FlatRichToken_core {
                    text: s.clone(),
                    color: COLOR_CODE,
                    bold: false,
                    italic: false,
                    font_size: FONT_SIZE_CODE,
                    line_idx: line,
                    x_offset: *x,
                });
                *x += s.len() as f32;
            }
            Inline::Link { text, .. } => {
                tokens.push(FlatRichToken_core {
                    text: text.to_owned(),
                    color: COLOR_LINK,
                    bold: base_bold,
                    italic: base_italic,
                    font_size,
                    line_idx: line,
                    x_offset: *x,
                });
                *x += text.len() as f32;
            }
        }
    }
}
