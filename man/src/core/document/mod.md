# `src/core/document/mod.rs`

## `pub mod parser;`
*Line 2 · mod*

Markdown to AST parser using pulldown-cmark.

---

## `pub mod serializer;`
*Line 4 · mod*

AST to Markdown string serializer.

---

## `pub mod frontmatter;`
*Line 6 · mod*

YAML frontmatter parsing and serialization.

---

## `pub struct ByteRange`
*Line 10 · struct*

UTF-8 byte range in the source Markdown string.

---

## `pub struct Document`
*Line 19 · struct*

A complete document — sequence of blocks with optional frontmatter.

---

## `pub fn new() -> Self`
*Line 28 · fn*

Create an empty document with no frontmatter.

---

## `pub fn to_markdown(&self) -> String`
*Line 36 · fn*

Serialize the document to a Markdown string.

---

## `pub fn from_markdown(input: &str) -> Self`
*Line 41 · fn*

Parse a Markdown string into a Document.

---

## `pub fn char_count(&self) -> usize`
*Line 46 · fn*

Total character count (excluding frontmatter delimiters).

---

## `pub fn word_count(&self) -> usize`
*Line 51 · fn*

Count words in the document.

---

## `pub fn block_count(&self) -> usize`
*Line 56 · fn*

Count blocks by type.

---

## `pub struct Frontmatter`
*Line 69 · struct*

YAML frontmatter key-value pairs.

---

## `pub enum Block`
*Line 76 · enum*

A block-level element in the document.

---

## `pub enum Inline`
*Line 138 · enum*

An inline-level element within a block.

---

## `pub fn plain_text(&self) -> String`
*Line 158 · fn*

Extract plain text from this inline element recursively.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=4" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
