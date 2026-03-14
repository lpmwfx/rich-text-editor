# `src/core/editor/commands.rs`

## `pub trait Command: std::fmt::Debug + Send`
*Line 6 · trait*

A reversible editing command.

---

## `pub enum CommandError`
*Line 17 · enum*

Errors from command execution.

---

## `pub struct InsertTextCommand`
*Line 46 · struct*

Insert text at a byte offset in the serialized Markdown.

---

## `pub struct DeleteRangeCommand`
*Line 89 · struct*

Delete a byte range from the serialized Markdown.

---

## `pub fn new(start: usize, end: usize) -> Self`
*Line 100 · fn*

Create a new delete range command.

---

## `pub struct ReplaceRangeCommand`
*Line 145 · struct*

Replace a byte range in the serialized Markdown.

---

## `pub fn new(start: usize, end: usize, replacement: String) -> Self`
*Line 158 · fn*

Create a new replace range command.

---

## `pub struct InsertBlockCommand`
*Line 213 · struct*

Insert a block at a specific index in the document.

---

