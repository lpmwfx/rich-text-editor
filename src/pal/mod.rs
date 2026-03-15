/// Skia/skparagraph rendering pipeline.
pub mod paragraph;
/// Renderable Skia Paragraphs for display.
pub mod paragraph_renderable;
/// Paragraph building tests.
#[cfg(test)]
mod paragraph_tests;
/// Paragraph building helpers (single paragraph, text paragraph, inline_to_string).
pub mod paragraph_builder;
/// ParagraphCache — cached paragraphs with layout metadata.
pub mod paragraph_cache;
/// ParagraphCache tests.
#[cfg(test)]
mod paragraph_cache_tests;
/// Cursor positioning via skparagraph APIs.
pub mod cursor;
/// Cursor rect computation.
pub mod cursor_rect_pal;
/// Selection highlighting via get_rects_for_range().
pub mod selection;
/// Selection rect computation.
pub mod selection_rects_pal;
/// Arrow key navigation via line metrics.
pub mod navigation;
/// Navigation tests.
#[cfg(test)]
mod navigation_tests;
/// Media placeholder layout and rendering.
pub mod media;
/// Media placeholder rendering via getRectsForPlaceholders().
pub mod media_rendering;
/// Render Skia output to displayable image buffers.
pub mod render;

pub use paragraph::{build_paragraphs, RenderableParagraph_pal, StyledParagraph_pal};
pub use paragraph_renderable::build_renderable_paragraphs;
pub use paragraph_cache::ParagraphCache_pal;
pub use cursor::{CursorPosition_pal, CursorInfo_pal};
pub use selection::{SelectionRange_pal, SelectionRect_pal};
pub use media_rendering::MediaPlaceholder_pal;
