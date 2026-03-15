#![allow(non_camel_case_types)]
// Range-based editing commands — delete and replace.

use crate::core::document::Document_core;
use crate::core::editor::commands::{Command_core, CommandError_core};

/// Delete a byte range from the serialized Markdown.
#[derive(Debug)]
pub struct DeleteRangeCommand_core {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
    /// The deleted text (stored on apply for undo).
    deleted: std::sync::Mutex<String>,
}

impl DeleteRangeCommand_core {
    /// Create a new delete range command.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            deleted: std::sync::Mutex::new(String::new()),
        }
    }
}

impl Command_core for DeleteRangeCommand_core {
    fn apply(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        if self.start >= self.end {
            return Err(CommandError_core::InvalidRange {
                start: self.start,
                end: self.end,
            });
        }
        let mut md = doc.to_markdown();
        if self.end > md.len() {
            return Err(CommandError_core::OffsetOutOfBounds {
                offset: self.end,
                length: md.len(),
            });
        }
        let removed: String = md.drain(self.start..self.end).collect();
        *self.deleted.lock().unwrap_or_else(|p| p.into_inner()) = removed;
        *doc = Document_core::from_markdown(&md);
        Ok(())
    }

    fn undo(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        let mut md = doc.to_markdown();
        let deleted = self.deleted.lock().unwrap_or_else(|p| p.into_inner());
        md.insert_str(self.start, &deleted);
        *doc = Document_core::from_markdown(&md);
        Ok(())
    }

    fn description(&self) -> &str {
        "delete range"
    }
}

/// Replace a byte range in the serialized Markdown.
#[derive(Debug)]
pub struct ReplaceRangeCommand_core {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
    /// Replacement text.
    pub replacement: String,
    /// The original text (stored on apply for undo).
    original: std::sync::Mutex<String>,
}

impl ReplaceRangeCommand_core {
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

impl Command_core for ReplaceRangeCommand_core {
    fn apply(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        if self.start > self.end {
            return Err(CommandError_core::InvalidRange {
                start: self.start,
                end: self.end,
            });
        }
        let mut md = doc.to_markdown();
        if self.end > md.len() {
            return Err(CommandError_core::OffsetOutOfBounds {
                offset: self.end,
                length: md.len(),
            });
        }
        let removed: String = md.drain(self.start..self.end).collect();
        *self.original.lock().unwrap_or_else(|p| p.into_inner()) = removed;
        md.insert_str(self.start, &self.replacement);
        *doc = Document_core::from_markdown(&md);
        Ok(())
    }

    fn undo(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        let mut md = doc.to_markdown();
        let repl_end = self.start + self.replacement.len();
        if repl_end > md.len() {
            return Err(CommandError_core::OffsetOutOfBounds {
                offset: repl_end,
                length: md.len(),
            });
        }
        md.drain(self.start..repl_end);
        let original = self.original.lock().unwrap_or_else(|p| p.into_inner());
        md.insert_str(self.start, &original);
        *doc = Document_core::from_markdown(&md);
        Ok(())
    }

    fn description(&self) -> &str {
        "replace range"
    }
}
