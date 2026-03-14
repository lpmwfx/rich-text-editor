/// UI layer — GUI (Slint) and MCP server surfaces.
pub mod ui;
/// Adapter layer — data exchange hub, state owner, ViewModel.
pub mod adapter;
/// Core layer — business logic, document model, editor commands.
pub mod core;
/// PAL layer — platform abstraction, Skia/skparagraph rendering.
pub mod pal;
/// Render layer — convert Skia output to displayable image buffers.
pub mod render;
/// Gateway layer — IO adapter, file and media loading.
pub mod gateway;
/// Shared layer — cross-cutting constants, errors, types.
pub mod shared;
