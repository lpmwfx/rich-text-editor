// Undo/redo stack — all mutations go through UndoStack::apply().

use crate::core::document::Document;
use crate::core::editor::commands::{Command, CommandError};
use crate::shared::limits::MAX_UNDO_DEPTH;

/// Undo/redo stack. All document mutations must go through apply().
#[derive(Debug)]
pub struct UndoStack {
    /// Command history (oldest first).
    history: Vec<Box<dyn Command>>,
    /// Current position in the history (points past the last applied command).
    cursor: usize,
}

impl UndoStack {
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
        cmd: Box<dyn Command>,
        doc: &mut Document,
    ) -> Result<(), CommandError> {
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
    pub fn undo(&mut self, doc: &mut Document) -> Result<bool, CommandError> {
        if self.cursor == 0 {
            return Ok(false);
        }
        self.cursor -= 1;
        self.history[self.cursor].undo(doc)?;
        Ok(true)
    }

    /// Redo the last undone command. Returns true if a redo was performed.
    pub fn redo(&mut self, doc: &mut Document) -> Result<bool, CommandError> {
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

impl Default for UndoStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::editor::commands::InsertTextCommand;
    use crate::core::editor::commands::ReplaceRangeCommand;

    #[test]
    fn apply_and_undo() {
        let mut doc = Document::from_markdown("Hello\n");
        let mut stack = UndoStack::new();

        let cmd = Box::new(InsertTextCommand {
            offset: 5,
            text: " world".into(),
        });
        stack.apply(cmd, &mut doc).unwrap();
        assert!(doc.to_markdown().contains("Hello world"));
        assert!(stack.can_undo());

        stack.undo(&mut doc).unwrap();
        assert!(doc.to_markdown().contains("Hello"));
        assert!(!doc.to_markdown().contains("world"));
    }

    #[test]
    fn undo_redo_cycle() {
        let mut doc = Document::from_markdown("A\n");
        let mut stack = UndoStack::new();

        stack
            .apply(
                Box::new(InsertTextCommand {
                    offset: 1,
                    text: "B".into(),
                }),
                &mut doc,
            )
            .unwrap();

        stack.undo(&mut doc).unwrap();
        assert!(!doc.to_markdown().contains("B"));
        assert!(stack.can_redo());

        stack.redo(&mut doc).unwrap();
        assert!(doc.to_markdown().contains("B"));
    }

    #[test]
    fn redo_cleared_after_new_command() {
        let mut doc = Document::from_markdown("Start\n");
        let mut stack = UndoStack::new();

        stack
            .apply(
                Box::new(InsertTextCommand {
                    offset: 5,
                    text: "1".into(),
                }),
                &mut doc,
            )
            .unwrap();

        stack.undo(&mut doc).unwrap();
        assert!(stack.can_redo());

        // New command should clear redo.
        stack
            .apply(
                Box::new(InsertTextCommand {
                    offset: 5,
                    text: "2".into(),
                }),
                &mut doc,
            )
            .unwrap();
        assert!(!stack.can_redo());
    }

    #[test]
    fn multiple_undo() {
        let mut doc = Document::from_markdown("Base\n");
        let mut stack = UndoStack::new();

        stack
            .apply(
                Box::new(InsertTextCommand {
                    offset: 4,
                    text: " one".into(),
                }),
                &mut doc,
            )
            .unwrap();
        stack
            .apply(
                Box::new(InsertTextCommand {
                    offset: 8,
                    text: " two".into(),
                }),
                &mut doc,
            )
            .unwrap();

        assert_eq!(stack.depth(), 2);

        stack.undo(&mut doc).unwrap();
        stack.undo(&mut doc).unwrap();
        assert!(!stack.can_undo());

        let md = doc.to_markdown();
        assert!(md.contains("Base"));
        assert!(!md.contains("one"));
        assert!(!md.contains("two"));
    }

    #[test]
    fn undo_nothing_returns_false() {
        let mut doc = Document::new();
        let mut stack = UndoStack::new();
        let result = stack.undo(&mut doc).unwrap();
        assert!(!result);
    }

    #[test]
    fn undo_description() {
        let mut doc = Document::from_markdown("X\n");
        let mut stack = UndoStack::new();

        assert!(stack.undo_description().is_none());

        stack
            .apply(
                Box::new(InsertTextCommand {
                    offset: 0,
                    text: "Y".into(),
                }),
                &mut doc,
            )
            .unwrap();
        assert_eq!(stack.undo_description(), Some("insert text"));
    }
}
