/// Rich text editor library — document model, editor state, renderer, and MCP server.
pub mod document;
/// Editor state management — cursor, selection, undo/redo.
pub mod editor;
/// Skia/skparagraph rendering pipeline.
pub mod renderer;
/// Media asset management — lazy loading and thumbnail cache.
pub mod media;
/// MCP server — exposes editor as tool/resource server.
pub mod mcp;
/// Named constants and configuration values.
pub mod state;
