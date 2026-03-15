#![allow(non_camel_case_types)]
/// Document output format discriminator for MCP get_document.

/// Wire name for AST JSON format.
const AST: &str = "ast";

/// Supported document output formats.
pub enum FormatKind_ui {
    /// Render as Markdown text.
    Markdown,
    /// Render as AST JSON.
    Ast,
}

impl FormatKind_ui {
    /// Parse a format string into a FormatKind.
    pub fn parse(s: &str) -> Self {
        match s {
            AST => Self::Ast,
            _ => Self::Markdown,
        }
    }
}
