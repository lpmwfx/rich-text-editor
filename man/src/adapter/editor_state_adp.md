# `src/adapter/editor_state_adp.rs`

## `pub struct EditorState`
*Line 11 · struct*

The adapter's central state — owns the document and coordinates all layers.

---

## `pub fn new() -> Self`
*Line 26 · fn*

Create a new editor state with an empty document.

---

## `pub fn from_markdown(input: &str) -> Self`
*Line 37 · fn*

Create an editor state from a Markdown string.

---

## `pub fn apply(&mut self, cmd: Box<dyn Command>) -> Result<(), CommandError>`
*Line 48 · fn*

Apply a command to the document via the undo stack.

---

## `pub fn undo(&mut self) -> Result<bool, CommandError>`
*Line 54 · fn*

Undo the last command.

---

## `pub fn redo(&mut self) -> Result<bool, CommandError>`
*Line 60 · fn*

Redo the last undone command.

---

## `pub fn to_markdown(&self) -> String`
*Line 66 · fn*

Get the document as a Markdown string.

---

## `pub fn selected_text(&self) -> Option<String>`
*Line 71 · fn*

Get the selected text, if any.

---

