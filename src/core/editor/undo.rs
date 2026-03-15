#![allow(non_camel_case_types)]
// Undo/redo stack — all mutations go through UndoStack::apply().

use crate::core::document::Document_core;
use crate::core::editor::commands::{Command_core, CommandError_core};
use crate::shared::limits::MAX_UNDO_DEPTH;

/// Undo/redo stack. All document mutations must go through apply().
#[derive(Debug)]
pub struct UndoStack_core {
    /// Command history (oldest first).
    history: Vec<Box<dyn Command_core>>,
    /// Current position in the history (points past the last applied command).
    cursor: usize,
}

impl UndoStack_core {
    /// Create a new empty undo stack.
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            cursor: 0,
        }
    }

    /// Apply a command to the document and push it to the undo stack.
    /// Clears any redo history beyond the current cursor.
    pub fn apply(
        &mut self,
        cmd: Box<dyn Command_core>,
        doc: &mut Document_core,
    ) -> Result<(), CommandError_core> {
        cmd.apply(doc)?;

        // Truncate any redo history.
        self.history.truncate(self.cursor);
        self.history.push(cmd);
        self.cursor += 1;

        // Enforce max depth.
        if self.history.len() > MAX_UNDO_DEPTH {
            let excess = self.history.len() - MAX_UNDO_DEPTH;
            self.history.drain(..excess);
            self.cursor -= excess;
        }

        Ok(())
    }

    /// Undo the last command. Returns true if an undo was performed.
    pub fn undo(&mut self, doc: &mut Document_core) -> Result<bool, CommandError_core> {
        if self.cursor == 0 {
            return Ok(false);
        }
        self.cursor -= 1;
        self.history[self.cursor].undo(doc)?;
        Ok(true)
    }

    /// Redo the last undone command. Returns true if a redo was performed.
    pub fn redo(&mut self, doc: &mut Document_core) -> Result<bool, CommandError_core> {
        if self.cursor >= self.history.len() {
            return Ok(false);
        }
        self.history[self.cursor].apply(doc)?;
        self.cursor += 1;
        Ok(true)
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        self.cursor < self.history.len()
    }

    /// Description of the command that would be undone.
    pub fn undo_description(&self) -> Option<&str> {
        if self.cursor > 0 {
            Some(self.history[self.cursor - 1].description())
        } else {
            None
        }
    }

    /// Number of commands in the history.
    pub fn depth(&self) -> usize {
        self.history.len()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.history.clear();
        self.cursor = 0;
    }
}

impl Default for UndoStack_core {
    fn default() -> Self {
        Self::new()
    }
}


