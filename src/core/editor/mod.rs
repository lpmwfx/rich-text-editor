/// Command trait and InsertTextCommand.
pub mod commands;
/// Range-based commands (delete, replace).
pub mod range_commands;
/// Block-level commands (insert block).
pub mod block_commands;
/// Command tests.
#[cfg(test)]
mod commands_tests;
/// Undo/redo stack with command pattern.
pub mod undo;
/// Undo stack tests.
#[cfg(test)]
mod undo_tests;
