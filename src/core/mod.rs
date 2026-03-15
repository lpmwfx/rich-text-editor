/// Block/Inline AST document model, parser, and serializer.
pub mod document;
/// Editor commands (Insert, Delete, Format) and undo stack.
pub mod editor;
/// Document AST to flat token list for Slint rendering.
pub mod tokenizer;
/// Inline tokenization helpers.
pub mod tokenizer_inline;
