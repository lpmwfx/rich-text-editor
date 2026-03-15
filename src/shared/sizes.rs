/// Default paragraph cache capacity (number of paragraphs).
pub const PARAGRAPH_CACHE_CAP: usize = 256;

/// Default media thumbnail width in pixels.
pub const THUMBNAIL_WIDTH: u32 = 320;

/// Default media thumbnail height in pixels.
pub const THUMBNAIL_HEIGHT: u32 = 240;

/// Caret width in pixels.
pub const CARET_WIDTH_PX: f32 = 2.0;
/// Fallback line height when no line metrics are available.
pub const FALLBACK_LINE_HEIGHT_PX: f32 = 20.0;
/// Fallback vertical position for cursor.
pub const FALLBACK_Y_PX: f32 = 16.0;
/// Mouse scroll sensitivity multiplier.
pub const SCROLL_FACTOR: f32 = 3.0;
/// Bytes per pixel (RGBA channels).
pub const ROW_BYTES_CHANNELS: usize = 4;
/// Left padding sides multiplier.
pub const LEFT_PADDING_SIDES: usize = 2;
/// Horizontal rule font size in points.
pub const HR_FONT_SIZE_PX: f32 = 1.0;
/// YAML frontmatter delimiter length in bytes.
pub const FRONTMATTER_DELIM_LEN: usize = 3;
/// YAML frontmatter delimiter string.
pub const FRONTMATTER_DELIM: &str = "---";
/// Heading level for H1.
pub const HEADING_LEVEL_H1: u8 = 1;
/// Heading level for H2.
pub const HEADING_LEVEL_H2: u8 = 2;
/// Heading level for H3.
pub const HEADING_LEVEL_H3: u8 = 3;
/// List marker prefix character offset.
pub const LIST_MARKER_OFFSET: f32 = 2.0;
