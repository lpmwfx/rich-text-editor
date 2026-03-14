// Command trait and concrete editing commands.

use crate::core::document::{Block, Document, Inline};

/// A reversible editing command.
pub trait Command: std::fmt::Debug + Send {
    /// Apply the command to the document. Returns Ok on success.
    fn apply(&self, doc: &mut Document) -> Result<(), CommandError>;
    /// Reverse the command (undo). Returns Ok on success.
    fn undo(&self, doc: &mut Document) -> Result<(), CommandError>;
    /// Human-readable description for the undo stack display.
    fn description(&self) -> &str;
}

/// Errors from command execution.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
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
pub struct InsertTextCommand {
    /// UTF-8 byte offset in the Markdown output.
    pub offset: usize,
    /// Text to insert.
    pub text: String,
}

impl Command for InsertTextCommand {
    fn apply(&self, doc: &mut Document) -> Result<(), CommandError> {
        let mut md = doc.to_markdown();
        let len = md.len();
        if self.offset > len {
            return Err(CommandError::OffsetOutOfBounds {
                offset: self.offset,
                length: len,
            });
        }
        md.insert_str(self.offset, &self.text);
        *doc = Document::from_markdown(&md);
        Ok(())
    }

    fn undo(&self, doc: &mut Document) -> Result<(), CommandError> {
        let mut md = doc.to_markdown();
        let end = self.offset + self.text.len();
        if end > md.len() {
            return Err(CommandError::OffsetOutOfBounds {
                offset: end,
                length: md.len(),
            });
        }
        md.drain(self.offset..end);
        *doc = Document::from_markdown(&md);
        Ok(())
    }

    fn description(&self) -> &str {
        "insert text"
    }
}

/// Delete a byte range from the serialized Markdown.
#[derive(Debug)]
pub struct DeleteRangeCommand {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
    /// The deleted text (stored on apply for undo).
    deleted: std::sync::Mutex<String>,
}

impl DeleteRangeCommand {
    /// Create a new delete range command.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            deleted: std::sync::Mutex::new(String::new()),
        }
    }
}

impl Command for DeleteRangeCommand {
    fn apply(&self, doc: &mut Document) -> Result<(), CommandError> {
        if self.start >= self.end {
            return Err(CommandError::InvalidRange {
                start: self.start,
                end: self.end,
            });
        }
        let mut md = doc.to_markdown();
        if self.end > md.len() {
            return Err(CommandError::OffsetOutOfBounds {
                offset: self.end,
                length: md.len(),
            });
        }
        let removed: String = md.drain(self.start..self.end).collect();
        *self.deleted.lock().unwrap() = removed;
        *doc = Document::from_markdown(&md);
        Ok(())
    }

    fn undo(&self, doc: &mut Document) -> Result<(), CommandError> {
        let mut md = doc.to_markdown();
        let deleted = self.deleted.lock().unwrap();
        md.insert_str(self.start, &deleted);
        *doc = Document::from_markdown(&md);
        Ok(())
    }

    fn description(&self) -> &str {
        "delete range"
    }
}

/// Replace a byte range in the serialized Markdown.
#[derive(Debug)]
pub struct ReplaceRangeCommand {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
    /// Replacement text.
    pub replacement: String,
    /// The original text (stored on apply for undo).
    original: std::sync::Mutex<String>,
}

impl ReplaceRangeCommand {
    /// Create a new replace range command.
    pub fn new(start: usize, end: usize, replacement: String) -> Self {
        Self {
            start,
            end,
            replacement,
            original: std::sync::Mutex::new(String::new()),
        }
    }
}

impl Command for ReplaceRangeCommand {
    fn apply(&self, doc: &mut Document) -> Result<(), CommandError> {
        if self.start > self.end {
            return Err(CommandError::InvalidRange {
                start: self.start,
                end: self.end,
            });
        }
        let mut md = doc.to_markdown();
        if self.end > md.len() {
            return Err(CommandError::OffsetOutOfBounds {
                offset: self.end,
                length: md.len(),
            });
        }
        let removed: String = md.drain(self.start..self.end).collect();
        *self.original.lock().unwrap() = removed;
        md.insert_str(self.start, &self.replacement);
        *doc = Document::from_markdown(&md);
        Ok(())
    }

    fn undo(&self, doc: &mut Document) -> Result<(), CommandError> {
        let mut md = doc.to_markdown();
        let repl_end = self.start + self.replacement.len();
        if repl_end > md.len() {
            return Err(CommandError::OffsetOutOfBounds {
                offset: repl_end,
                length: md.len(),
            });
        }
        md.drain(self.start..repl_end);
        let original = self.original.lock().unwrap();
        md.insert_str(self.start, &original);
        *doc = Document::from_markdown(&md);
        Ok(())
    }

    fn description(&self) -> &str {
        "replace range"
    }
}

/// Insert a block at a specific index in the document.
#[derive(Debug)]
pub struct InsertBlockCommand {
    /// Block index where to insert (pushes existing blocks down).
    pub index: usize,
    /// The block to insert.
    pub block: Block,
}

impl Command for InsertBlockCommand {
    fn apply(&self, doc: &mut Document) -> Result<(), CommandError> {
        if self.index > doc.blocks.len() {
            return Err(CommandError::BlockOutOfBounds {
                index: self.index,
                count: doc.blocks.len(),
            });
        }
        doc.blocks.insert(self.index, self.block.clone());
        Ok(())
    }

    fn undo(&self, doc: &mut Document) -> Result<(), CommandError> {
        if self.index >= doc.blocks.len() {
            return Err(CommandError::BlockOutOfBounds {
                index: self.index,
                count: doc.blocks.len(),
            });
        }
        doc.blocks.remove(self.index);
        Ok(())
    }

    fn description(&self) -> &str {
        "insert block"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_text_and_undo() {
        let mut doc = Document::from_markdown("Hello world\n");
        let cmd = InsertTextCommand {
            offset: 5,
            text: " beautiful".into(),
        };
        cmd.apply(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello beautiful world"));

        cmd.undo(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello world"));
        assert!(!md.contains("beautiful"));
    }

    #[test]
    fn delete_range_and_undo() {
        let mut doc = Document::from_markdown("Hello beautiful world\n");
        let cmd = DeleteRangeCommand::new(5, 15);
        cmd.apply(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(!md.contains("beautiful"));

        cmd.undo(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello beautiful world"));
    }

    #[test]
    fn replace_range_and_undo() {
        let mut doc = Document::from_markdown("Hello world\n");
        let cmd = ReplaceRangeCommand::new(6, 11, "Rust".into());
        cmd.apply(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello Rust"));

        cmd.undo(&mut doc).unwrap();
        let md = doc.to_markdown();
        assert!(md.contains("Hello world"));
    }

    #[test]
    fn insert_block_and_undo() {
        let mut doc = Document::from_markdown("# Title\n");
        let block = Block::Paragraph {
            content: vec![Inline::Text("New paragraph".into())],
            range: None,
        };
        let cmd = InsertBlockCommand {
            index: 1,
            block,
        };
        cmd.apply(&mut doc).unwrap();
        assert_eq!(doc.blocks.len(), 2);

        cmd.undo(&mut doc).unwrap();
        assert_eq!(doc.blocks.len(), 1);
    }

    #[test]
    fn offset_out_of_bounds() {
        let mut doc = Document::from_markdown("Hi\n");
        let cmd = InsertTextCommand {
            offset: 9999,
            text: "x".into(),
        };
        assert!(cmd.apply(&mut doc).is_err());
    }
}
