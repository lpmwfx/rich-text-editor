# `src/core/editor/undo.rs`

## `pub struct UndoStack`
*Line 9 · struct*

Undo/redo stack. All document mutations must go through apply().

---

## `pub fn new() -> Self`
*Line 18 · fn*

Create a new empty undo stack.

---

## `pub fn apply( &mut self, cmd: Box<dyn Command>, doc: &mut Document, ) -> Result<(), CommandError>`
*Line 27 · fn*

Apply a command to the document and push it to the undo stack.
Clears any redo history beyond the current cursor.

---

## `pub fn undo(&mut self, doc: &mut Document) -> Result<bool, CommandError>`
*Line 50 · fn*

Undo the last command. Returns true if an undo was performed.

---

## `pub fn redo(&mut self, doc: &mut Document) -> Result<bool, CommandError>`
*Line 60 · fn*

Redo the last undone command. Returns true if a redo was performed.

---

## `pub fn can_undo(&self) -> bool`
*Line 70 · fn*

Check if undo is available.

---

## `pub fn can_redo(&self) -> bool`
*Line 75 · fn*

Check if redo is available.

---

## `pub fn undo_description(&self) -> Option<&str>`
*Line 80 · fn*

Description of the command that would be undone.

---

## `pub fn depth(&self) -> usize`
*Line 89 · fn*

Number of commands in the history.

---

## `pub fn clear(&mut self)`
*Line 94 · fn*

Clear all history.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=4" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
