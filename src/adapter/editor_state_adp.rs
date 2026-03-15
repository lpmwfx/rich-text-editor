#![allow(non_camel_case_types)]
/// Central editor state — the adapter's state owner.
/// Holds document, cursor, selection, undo stack.
/// All mutations go through the undo stack.

use std::path::Path;

use crate::core::document::Document_core;
use crate::core::editor::commands::{Command_core, CommandError_core, InsertBlockCommand, InsertTextCommand_core, DeleteRangeCommand, ReplaceRangeCommand};
use crate::core::editor::undo::UndoStack_core;
use crate::gateway::file_ops;
use crate::shared::document_types_x::Block_x as Block;

/// The adapter's central state — owns the document and coordinates all layers.
#[derive(Debug)]
pub struct EditorState_adp {
    /// The active document being edited.
    pub document: Document_core,
    /// Current cursor position as UTF-8 byte offset.
    pub cursor: usize,
    /// Active selection range (start, end) or None.
    pub selection: Option<(usize, usize)>,
    /// File path of the open document, if any.
    pub file_path: Option<std::path::PathBuf>,
    /// Undo/redo stack — all mutations go through here.
    pub undo_stack: UndoStack_core,
}

impl EditorState_adp {
    /// Create a new editor state with an empty document.
    pub fn new() -> Self {
        Self {
            document: Document_core::new(),
            cursor: 0,
            selection: None,
            file_path: None,
            undo_stack: UndoStack_core::new(),
        }
    }

    /// Create an editor state from a Markdown string.
    pub fn from_markdown(input: &str) -> Self {
        Self {
            document: Document_core::from_markdown(input),
            cursor: 0,
            selection: None,
            file_path: None,
            undo_stack: UndoStack_core::new(),
        }
    }

    /// Apply a command to the document via the undo stack.
    pub fn apply(&mut self, cmd: Box<dyn Command_core>) -> Result<(), CommandError_core> {
        self.selection = None;
        self.undo_stack.apply(cmd, &mut self.document)
    }

    /// Undo the last command.
    pub fn undo(&mut self) -> Result<bool, CommandError_core> {
        self.selection = None;
        self.undo_stack.undo(&mut self.document)
    }

    /// Redo the last undone command.
    pub fn redo(&mut self) -> Result<bool, CommandError_core> {
        self.selection = None;
        self.undo_stack.redo(&mut self.document)
    }

    /// Get the document as a Markdown string.
    pub fn to_markdown(&self) -> String {
        self.document.to_markdown()
    }

    /// Open a Markdown file — resets document, cursor, undo stack.
    pub fn open_file(&mut self, path: &Path) -> Result<(), EditorError_adp> {
        let content = file_ops::read_file(path).map_err(EditorError_adp::FileError)?;
        self.document = Document_core::from_markdown(&content);
        self.cursor = 0;
        self.selection = None;
        self.file_path = Some(path.to_path_buf());
        self.undo_stack.clear();
        Ok(())
    }

    /// Save the current document to its file path.
    pub fn save_file(&self) -> Result<(), EditorError_adp> {
        let path = self.file_path.as_ref().ok_or(EditorError_adp::NoFileOpen)?;
        let md = self.to_markdown();
        file_ops::write_file(path, &md).map_err(EditorError_adp::FileError)?;
        Ok(())
    }

    /// Save the current document to a specific path.
    pub fn save_file_as(&mut self, path: &Path) -> Result<(), EditorError_adp> {
        let md = self.to_markdown();
        file_ops::write_file(path, &md).map_err(EditorError_adp::FileError)?;
        self.file_path = Some(path.to_path_buf());
        Ok(())
    }

    /// Insert text at a byte offset in the markdown.
    pub fn insert_text_at(&mut self, offset: usize, text: String) -> Result<(), CommandError_core> {
        self.apply(Box::new(InsertTextCommand_core { offset, text }))
    }

    /// Delete a byte range from the markdown.
    pub fn delete_range(&mut self, start: usize, end: usize) -> Result<(), CommandError_core> {
        self.apply(Box::new(DeleteRangeCommand::new(start, end)))
    }

    /// Replace a byte range in the markdown.
    pub fn replace_range(&mut self, start: usize, end: usize, replacement: String) -> Result<(), CommandError_core> {
        self.apply(Box::new(ReplaceRangeCommand::new(start, end, replacement)))
    }

    /// Insert a block at a specific index.
    pub fn insert_block_at(&mut self, index: usize, block: Block) -> Result<(), CommandError_core> {
        self.apply(Box::new(InsertBlockCommand { index, block }))
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
pub enum EditorError_adp {
    /// A command failed.
    #[error("command error: {0}")]
    CommandError(#[from] CommandError_core),
    /// A file operation failed.
    #[error("file error: {0}")]
    FileError(file_ops::FileError_gtw),
    /// No file is currently open.
    #[error("no file open")]
    NoFileOpen,
}

impl Default for EditorState_adp {
    fn default() -> Self {
        Self::new()
    }
}
