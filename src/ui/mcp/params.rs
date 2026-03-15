#![allow(non_camel_case_types)]
// MCP parameter structs with schemars derive for JSON schema generation.

use rmcp::schemars;

/// Parameters for get_document tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetDocumentParams_ui {
    /// Output format: "markdown" (default) or "ast".
    pub format: Option<String>,
}

/// Parameters for open_document tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct OpenDocumentParams_ui {
    /// Absolute or project-relative path to .md file.
    pub path: String,
}

/// Parameters for insert_text tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InsertTextParams_ui {
    /// Text to insert.
    pub text: String,
    /// UTF-8 byte offset (None = cursor position).
    pub offset: Option<usize>,
}

/// Parameters for replace_range tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReplaceRangeParams_ui {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
    /// Replacement text.
    pub replacement: String,
}

/// Parameters for delete_range tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DeleteRangeParams_ui {
    /// Start offset (inclusive).
    pub start: usize,
    /// End offset (exclusive).
    pub end: usize,
}

/// Parameters for insert_block tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InsertBlockParams_ui {
    /// Block type: paragraph, heading, code_block, horizontal_rule, bullet_list.
    pub block_type: String,
    /// Text content (ignored for horizontal_rule).
    pub content: Option<String>,
    /// Heading level 1-3 (only for heading).
    pub level: Option<u8>,
    /// Code language (only for code_block).
    pub language: Option<String>,
    /// Block index to insert at (None = end).
    pub index: Option<usize>,
}

/// Parameters for set_cursor tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SetCursorParams_ui {
    /// UTF-8 byte offset.
    pub offset: usize,
}
