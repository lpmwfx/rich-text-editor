# `src/gateway/file_ops.rs`

## `pub fn read_file(path: &Path) -> Result<String, FileError>`
*Line 6 · fn*

Read a Markdown file from disk.

---

## `pub fn write_file(path: &Path, content: &str) -> Result<(), FileError>`
*Line 14 · fn*

Write a Markdown string to disk.

---

## `pub enum FileError`
*Line 23 · enum*

File IO errors.

---

