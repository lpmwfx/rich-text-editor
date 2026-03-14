/// Skia/skparagraph rendering pipeline.
pub mod paragraph;
/// Cursor and selection rectangle rendering.
pub mod cursor;
/// Selection highlighting via getRectsForRange().
pub mod selection;
/// Media placeholder layout and rendering.
pub mod media;
/// Media placeholder rendering via getRectsForPlaceholders().
pub mod media_rendering;

pub use paragraph::{build_paragraphs, StyledParagraph_pal};
pub use selection::SelectionRange_pal;
pub use media_rendering::MediaPlaceholder_pal;
