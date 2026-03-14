// Gateway layer — file IO operations.
// Only layer that touches disk. Core and Adapter never do IO directly.

/// File operations — open, save, read Markdown files.
pub mod file_ops;
