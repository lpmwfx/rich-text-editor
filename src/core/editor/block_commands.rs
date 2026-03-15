#![allow(non_camel_case_types)]
// Block-level editing commands.

use crate::core::document::{Block, Document_core};
use crate::core::editor::commands::{Command_core, CommandError_core};

/// Insert a block at a specific index in the document.
#[derive(Debug)]
pub struct InsertBlockCommand_core {
    /// Block index where to insert (pushes existing blocks down).
    pub index: usize,
    /// The block to insert.
    pub block: Block,
}

impl Command_core for InsertBlockCommand_core {
    fn apply(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        if self.index > doc.blocks.len() {
            return Err(CommandError_core::BlockOutOfBounds {
                index: self.index,
                count: doc.blocks.len(),
            });
        }
        doc.blocks.insert(self.index, self.block.clone());
        Ok(())
    }

    fn undo(&self, doc: &mut Document_core) -> Result<(), CommandError_core> {
        if self.index >= doc.blocks.len() {
            return Err(CommandError_core::BlockOutOfBounds {
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
