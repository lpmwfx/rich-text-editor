#![allow(non_camel_case_types)]
// ParagraphCache — caches built Skia paragraphs with y-offsets for layout.

use crate::shared::document_types_x::{Block_x as Block, ByteRange_x as ByteRange};
use skia_safe::{
    textlayout::{FontCollection, Paragraph},
    Color, FontMgr,
};

use crate::pal::paragraph_builder::{build_single_paragraph, find_block_range};

/// Default text color (Catppuccin Mocha text).
pub const COLOR_TEXT: Color = Color::from_argb(255, 205, 214, 244);
/// Default secondary text color.
pub const COLOR_SUBTEXT: Color = Color::from_argb(255, 166, 173, 200);
/// Default surface accent color.
pub const COLOR_SURFACE1: Color = Color::from_argb(255, 88, 91, 112);

/// Base body text font size in points.
pub const FONT_SIZE_BASE: f32 = 14.0;
/// Font size for code blocks.
pub const FONT_SIZE_CODE: f32 = 12.0;
/// Font size for level-1 headings.
pub const FONT_SIZE_H1: f32 = 32.0;
/// Font size for level-2 headings.
pub const FONT_SIZE_H2: f32 = 24.0;
/// Font size for level-3 headings.
pub const FONT_SIZE_H3: f32 = 16.0;

/// Spacing between paragraphs in pixels.
const PARAGRAPH_SPACING: f32 = 8.0;
/// Left padding for all content.
const LEFT_PADDING: f32 = 16.0;
/// Top padding for the document.
const TOP_PADDING: f32 = 16.0;

/// A single cached paragraph entry with layout metadata.
pub struct ParagraphEntry_pal {
    /// The built Skia paragraph (already laid out).
    pub paragraph: Paragraph,
    /// Y offset from the top of the document.
    pub y_offset: f32,
    /// Index of the source block in Document.blocks.
    pub block_index: usize,
    /// Plain text length (what skparagraph sees).
    pub text_len: usize,
    /// Byte range in the serialized Markdown.
    pub markdown_range: ByteRange,
    /// The plain text content of this paragraph.
    pub plain_text: String,
}

/// Cache of all paragraphs in the document, with layout positions.
pub struct ParagraphCache_pal {
    /// Cached paragraph entries.
    entries: Vec<ParagraphEntry_pal>,
    /// Layout width used for building.
    width: f32,
}

impl ParagraphCache_pal {
    /// Rebuild the cache from document blocks and their serialized markdown.
    pub fn rebuild(blocks: &[Block], markdown: &str, width: f32) -> Self {
        let fm = FontMgr::new();
        let mut font_collection = FontCollection::new();
        font_collection.set_asset_font_manager(Some(fm.clone()));

        let mut entries = Vec::new();
        let mut y_offset = TOP_PADDING;

        let mut md_offset: usize = 0;

        for (block_idx, block) in blocks.iter().enumerate() {
            let (paragraph, plain_text, _font_size) =
                build_single_paragraph(block, width - LEFT_PADDING * crate::shared::sizes::LEFT_PADDING_SIDES as f32, &font_collection);

            let text_len = plain_text.len();
            let md_range = find_block_range(&markdown, &mut md_offset, block);

            entries.push(ParagraphEntry_pal {
                paragraph,
                y_offset,
                block_index: block_idx,
                text_len,
                markdown_range: md_range,
                plain_text,
            });

            let para_height = entries.last().map(|e| e.paragraph.height()).unwrap_or(0.0);
            y_offset += para_height + PARAGRAPH_SPACING;
        }

        ParagraphCache_pal { entries, width }
    }

    /// Total document height in pixels.
    pub fn total_height(&self) -> f32 {
        self.entries
            .last()
            .map(|e| e.y_offset + e.paragraph.height() + PARAGRAPH_SPACING)
            .unwrap_or(TOP_PADDING)
    }

    /// Find the paragraph entry at a given y coordinate.
    pub fn paragraph_at_y(&self, y: f32) -> Option<(usize, &ParagraphEntry_pal)> {
        for (i, entry) in self.entries.iter().enumerate() {
            let bottom = entry.y_offset + entry.paragraph.height();
            if y >= entry.y_offset && y < bottom {
                return Some((i, entry));
            }
        }
        if !self.entries.is_empty() {
            let last = self.entries.len() - 1;
            return Some((last, &self.entries[last]));
        }
        None
    }

    /// Get a paragraph entry by index.
    pub fn entry(&self, index: usize) -> Option<&ParagraphEntry_pal> {
        self.entries.get(index)
    }

    /// Get a mutable paragraph entry by index.
    pub fn entry_mut(&mut self, index: usize) -> Option<&mut ParagraphEntry_pal> {
        self.entries.get_mut(index)
    }

    /// Number of entries in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Iterator over entries.
    pub fn entries(&self) -> &[ParagraphEntry_pal] {
        &self.entries
    }

    /// Left padding constant.
    pub fn left_padding(&self) -> f32 {
        LEFT_PADDING
    }

    /// Layout width.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Convert a plain-text offset within a paragraph entry to a markdown byte offset.
    pub fn plain_to_markdown_offset(&self, entry_index: usize, plain_offset: usize) -> usize {
        if let Some(entry) = self.entries.get(entry_index) {
            entry.markdown_range.start + plain_offset
        } else {
            0
        }
    }

    /// Convert a markdown byte offset to (entry_index, plain_text_offset).
    pub fn markdown_to_plain_offset(&self, md_offset: usize) -> Option<(usize, usize)> {
        for (i, entry) in self.entries.iter().enumerate() {
            if md_offset >= entry.markdown_range.start && md_offset <= entry.markdown_range.end {
                let local_offset = md_offset - entry.markdown_range.start;
                let plain_offset = local_offset.min(entry.text_len);
                return Some((i, plain_offset));
            }
        }
        if let Some(last) = self.entries.last() {
            return Some((self.entries.len() - 1, last.text_len));
        }
        None
    }
}
