/// Central editor state — the adapter's state owner.
/// Holds document, cursor, selection, and coordinates all layers.
#[derive(Debug)]
pub struct EditorState {
    /// The active document being edited.
    pub document: crate::core::document::Document,
    /// Current cursor position as UTF-8 byte offset.
    pub cursor: usize,
    /// Active selection range (start, end) or None.
    pub selection: Option<(usize, usize)>,
    /// File path of the open document, if any.
    pub file_path: Option<std::path::PathBuf>,
}
