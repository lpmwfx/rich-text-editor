/// Editor state — cursor, selection, undo stack.
pub mod commands;
/// Undo/redo stack with command pattern.
pub mod undo;

/// Central editor state holding the document, cursor, and undo history.
#[derive(Debug)]
pub struct EditorState {
    /// The active document being edited.
    pub document: crate::document::Document,
    /// Current cursor position as UTF-8 byte offset.
    pub cursor: usize,
    /// Active selection range (start, end) or None.
    pub selection: Option<(usize, usize)>,
    /// File path of the open document, if any.
    pub file_path: Option<std::path::PathBuf>,
}
