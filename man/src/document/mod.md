# `src/document/mod.rs`

## `pub mod parser;`
*Line 2 · mod*

Block and Inline AST types for the document model.

---

## `pub mod serializer;`
*Line 4 · mod*

AST to Markdown serializer.

---

## `pub mod frontmatter;`
*Line 6 · mod*

YAML frontmatter parsing and serialization.

---

## `pub struct Document`
*Line 10 · struct*

A complete document — sequence of blocks with optional frontmatter.

---

## `pub struct Frontmatter`
*Line 19 · struct*

YAML frontmatter key-value pairs.

---

## `pub enum Block`
*Line 26 · enum*

A block-level element in the document.

---

## `pub enum Inline`
*Line 67 · enum*

An inline-level element within a block.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=4" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
