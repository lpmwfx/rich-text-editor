# `src/adapter/editor_state_adp.rs`

## `pub struct EditorState`
*Line 14 · struct*

The adapter's central state — owns the document and coordinates all layers.

---

## `pub fn new() -> Self`
*Line 29 · fn*

Create a new editor state with an empty document.

---

## `pub fn from_markdown(input: &str) -> Self`
*Line 40 · fn*

Create an editor state from a Markdown string.

---

## `pub fn apply(&mut self, cmd: Box<dyn Command>) -> Result<(), CommandError>`
*Line 51 · fn*

Apply a command to the document via the undo stack.

---

## `pub fn undo(&mut self) -> Result<bool, CommandError>`
*Line 57 · fn*

Undo the last command.

---

## `pub fn redo(&mut self) -> Result<bool, CommandError>`
*Line 63 · fn*

Redo the last undone command.

---

## `pub fn to_markdown(&self) -> String`
*Line 69 · fn*

Get the document as a Markdown string.

---

## `pub fn open_file(&mut self, path: &Path) -> Result<(), EditorError>`
*Line 74 · fn*

Open a Markdown file — resets document, cursor, undo stack.

---

## `pub fn save_file(&self) -> Result<(), EditorError>`
*Line 85 · fn*

Save the current document to its file path.

---

## `pub fn save_file_as(&mut self, path: &Path) -> Result<(), EditorError>`
*Line 93 · fn*

Save the current document to a specific path.

---

## `pub fn selected_text(&self) -> Option<String>`
*Line 101 · fn*

Get the selected text, if any.

---

## `pub enum EditorError`
*Line 114 · enum*

Adapter-level errors combining command and file errors.

---

