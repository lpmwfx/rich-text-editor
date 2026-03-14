/// Central editor state — the adapter's state owner.
/// Holds document, cursor, selection, undo stack.
/// All mutations go through the undo stack.

use std::path::{Path, PathBuf};

use crate::core::document::Document;
use crate::core::editor::commands::{Command, CommandError};
use crate::core::editor::undo::UndoStack;
use crate::gateway::file_ops;

/// The adapter's central state — owns the document and coordinates all layers.
#[derive(Debug)]
pub struct EditorState {
    /// The active document being edited.
    pub document: Document,
    /// Current cursor position as UTF-8 byte offset.
    pub cursor: usize,
    /// Active selection range (start, end) or None.
    pub selection: Option<(usize, usize)>,
    /// File path of the open document, if any.
    pub file_path: Option<std::path::PathBuf>,
    /// Undo/redo stack — all mutations go through here.
    pub undo_stack: UndoStack,
}

impl EditorState {
    /// Create a new editor state with an empty document.
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            cursor: 0,
            selection: None,
            file_path: None,
            undo_stack: UndoStack::new(),
        }
    }

    /// Create an editor state from a Markdown string.
    pub fn from_markdown(input: &str) -> Self {
        Self {
            document: Document::from_markdown(input),
            cursor: 0,
            selection: None,
            file_path: None,
            undo_stack: UndoStack::new(),
        }
    }

    /// Apply a command to the document via the undo stack.
    pub fn apply(&mut self, cmd: Box<dyn Command>) -> Result<(), CommandError> {
        self.selection = None;
        self.undo_stack.apply(cmd, &mut self.document)
    }

    /// Undo the last command.
    pub fn undo(&mut self) -> Result<bool, CommandError> {
        self.selection = None;
        self.undo_stack.undo(&mut self.document)
    }

    /// Redo the last undone command.
    pub fn redo(&mut self) -> Result<bool, CommandError> {
        self.selection = None;
        self.undo_stack.redo(&mut self.document)
    }

    /// Get the document as a Markdown string.
    pub fn to_markdown(&self) -> String {
        self.document.to_markdown()
    }

    /// Open a Markdown file — resets document, cursor, undo stack.
    pub fn open_file(&mut self, path: &Path) -> Result<(), EditorError> {
        let content = file_ops::read_file(path).map_err(EditorError::FileError)?;
        self.document = Document::from_markdown(&content);
        self.cursor = 0;
        self.selection = None;
        self.file_path = Some(path.to_path_buf());
        self.undo_stack.clear();
        Ok(())
    }

    /// Save the current document to its file path.
    pub fn save_file(&self) -> Result<(), EditorError> {
        let path = self.file_path.as_ref().ok_or(EditorError::NoFileOpen)?;
        let md = self.to_markdown();
        file_ops::write_file(path, &md).map_err(EditorError::FileError)?;
        Ok(())
    }

    /// Save the current document to a specific path.
    pub fn save_file_as(&mut self, path: &Path) -> Result<(), EditorError> {
        let md = self.to_markdown();
        file_ops::write_file(path, &md).map_err(EditorError::FileError)?;
        self.file_path = Some(path.to_path_buf());
        Ok(())
    }

    /// Get the selected text, if any.
    pub fn selected_text(&self) -> Option<String> {
        let (start, end) = self.selection?;
        let md = self.to_markdown();
        if end <= md.len() {
            Some(md[start..end].to_string())
        } else {
            None
        }
    }
}

/// Adapter-level errors combining command and file errors.
#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    /// A command failed.
    #[error("command error: {0}")]
    CommandError(#[from] CommandError),
    /// A file operation failed.
    #[error("file error: {0}")]
    FileError(file_ops::FileError),
    /// No file is currently open.
    #[error("no file open")]
    NoFileOpen,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::editor::commands::InsertTextCommand;

    #[test]
    fn new_state_is_empty() {
        let state = EditorState::new();
        assert!(state.document.blocks.is_empty());
        assert_eq!(state.cursor, 0);
        assert!(state.selection.is_none());
    }

    #[test]
    fn apply_and_undo() {
        let mut state = EditorState::from_markdown("Hello\n");
        state
            .apply(Box::new(InsertTextCommand {
                offset: 5,
                text: " world".into(),
            }))
            .unwrap();
        assert!(state.to_markdown().contains("Hello world"));

        state.undo().unwrap();
        assert!(!state.to_markdown().contains("world"));
    }

    #[test]
    fn selected_text() {
        let mut state = EditorState::from_markdown("Hello world\n");
        state.selection = Some((6, 11));
        assert_eq!(state.selected_text().unwrap(), "world");
    }

    #[test]
    fn open_and_save_file() {
        let dir = std::env::temp_dir().join("rte_test_adapter");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_open.md");
        std::fs::write(&path, "# Test\n\nBody\n").unwrap();

        let mut state = EditorState::new();
        state.open_file(&path).unwrap();
        assert_eq!(state.file_path.as_ref().unwrap(), &path);
        assert!(!state.document.blocks.is_empty());

        // Modify and save — use plain text that survives parse round-trip
        let md = state.to_markdown();
        let insert_offset = md.find("Body").unwrap();
        state
            .apply(Box::new(InsertTextCommand {
                offset: insert_offset,
                text: "Edited ".into(),
            }))
            .unwrap();
        state.save_file().unwrap();

        let saved = std::fs::read_to_string(&path).unwrap();
        assert!(saved.contains("Edited Body"));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_without_file_errors() {
        let state = EditorState::new();
        assert!(state.save_file().is_err());
    }
}
