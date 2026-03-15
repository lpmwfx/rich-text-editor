#![allow(non_camel_case_types)]
// Command trait and InsertTextCommand.

use crate::core::document::Document_core;

// Re-export child command types.
pub use super::range_commands::{DeleteRangeCommand_core as DeleteRangeCommand, ReplaceRangeCommand_core as ReplaceRangeCommand};
pub use super::block_commands::InsertBlockCommand_core as InsertBlockCommand;

/// A reversible editing command.
pub trait Command_core: std::fmt::Debug + Send {
    /// Apply the command to the document. Returns Ok on success.
    fn apply(&self, doc: &mut Document_core) -> Result<(), CommandError_core>;
    /// Reverse the command (undo). Returns Ok on success.
    fn undo(&self, doc: &mut Document_core) -> Result<(), CommandError_core>;
    /// Human-readable description for the undo stack display.
    fn description(&self) -> &str;
}

/// Errors from command execution.
#[derive(Debug, thiserror::Error)]
pub enum CommandError_core {
    /// The specified offset is outside the document bounds.
    #[error("offset {offset} is outside document (length {length})")]
    OffsetOutOfBounds {
        /// The invalid offset.
        offset: usize,
        /// The document length.
        length: usize,
    },
    /// The specified range is invalid.
    #[error("invalid range {start}..{end}")]
    InvalidRange {
        /// Range start.
        start: usize,
        /// Range end.
        end: usize,
    },
    /// The specified block index is out of bounds.
    #[error("block index {index} out of bounds (count: {count})")]
    BlockOutOfBounds {
        /// The invalid index.
        index: usize,
        /// The block count.
        count: usize,
    },
}

/// Insert text at a byte offset in the serialized Markdown.
#[derive(Debug)]
pub struct InsertTextCommand_core {
    /// UTF-8 byte offset in the Markdown output.
    pub offset: usize,
    /// Text to insert.
    pub text: String,
}

impl Command_core for InsertTextCommand_core {
    fn apply(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        let mut md = doc.to_markdown();
        let len = md.len();
        if self.offset > len {
            return Err(CommandError_core::OffsetOutOfBounds {
                offset: self.offset,
                length: len,
            });
        }
        md.insert_str(self.offset, &self.text);
        *doc = Document_core::from_markdown(&md);
        Ok(())
    }

    fn undo(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        let mut md = doc.to_markdown();
        let end = self.offset + self.text.len();
        if end > md.len() {
            return Err(CommandError_core::OffsetOutOfBounds {
                offset: end,
                length: md.len(),
            });
        }
        md.drain(self.offset..end);
        *doc = Document_core::from_markdown(&md);
        Ok(())
    }

    fn description(&self) -> &str {
        "insert text"
    }
}
